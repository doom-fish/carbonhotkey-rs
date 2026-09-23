use std::cell::RefCell;
use std::ffi::c_void;
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

use carbonhotkey::prelude::*;

const KEYBOARD_EVENT_CLASS: u32 = 0x6b65_7962;
const HOTKEY_PRESSED: u32 = 5;
const PARAM_DIRECT_OBJECT: u32 = 0x2d2d_2d2d;
const TYPE_EVENT_HOTKEY_ID: u32 = 0x686b_6964;
const NO_ERR: i32 = 0;
const EVENT_NOT_HANDLED_ERR: i32 = -9874;
const CRATE_SIGNATURE: u32 = 0x646f_6f6d;
const FOREIGN_SIGNATURE: u32 = u32::from_be_bytes(*b"othr");
const GLOBAL_HOTKEY_OPT_IN: &str = "CARBONHOTKEY_TEST_GLOBAL_HOTKEYS";

#[repr(C)]
struct EventHotKeyId {
    signature: u32,
    id: u32,
}

#[repr(C)]
struct DispatchQueue {
    _opaque: [u8; 0],
}

extern "C" {
    fn CreateEvent(
        allocator: *const c_void,
        class_id: u32,
        kind: u32,
        when: f64,
        attributes: u32,
        out_event: *mut *mut c_void,
    ) -> i32;
    fn SetEventParameter(
        event: *mut c_void,
        name: u32,
        param_type: u32,
        size: usize,
        data: *const c_void,
    ) -> i32;
    fn SendEventToEventTarget(event: *mut c_void, target: *mut c_void) -> i32;
    fn ReleaseEvent(event: *mut c_void);
    fn GetApplicationEventTarget() -> *mut c_void;
    fn pthread_main_np() -> i32;
    static _dispatch_main_q: DispatchQueue;
    fn dispatch_async_f(
        queue: *const DispatchQueue,
        context: *mut c_void,
        work: extern "C" fn(*mut c_void),
    );
}

fn send_hotkey_event(signature: u32, id: u32) -> i32 {
    unsafe {
        let mut event = core::ptr::null_mut();
        assert_eq!(
            CreateEvent(
                core::ptr::null(),
                KEYBOARD_EVENT_CLASS,
                HOTKEY_PRESSED,
                0.0,
                0,
                &raw mut event
            ),
            NO_ERR
        );
        let hotkey = EventHotKeyId { signature, id };
        assert_eq!(
            SetEventParameter(
                event,
                PARAM_DIRECT_OBJECT,
                TYPE_EVENT_HOTKEY_ID,
                core::mem::size_of::<EventHotKeyId>(),
                (&raw const hotkey).cast(),
            ),
            NO_ERR
        );
        let status = SendEventToEventTarget(event, GetApplicationEventTarget());
        ReleaseEvent(event);
        status
    }
}

fn counting_handler(counter: &Arc<AtomicUsize>) -> EventHandler {
    let counter = Arc::clone(counter);
    install_keyboard_handler(move |_| {
        counter.fetch_add(1, Ordering::SeqCst);
    })
    .expect("keyboard handler on the main thread")
}

fn observers_never_consume_hotkey_events() {
    let older = Arc::new(AtomicUsize::new(0));
    let newer = Arc::new(AtomicUsize::new(0));
    let older_handler = counting_handler(&older);
    let newer_handler = counting_handler(&newer);

    assert_eq!(
        send_hotkey_event(FOREIGN_SIGNATURE, 1),
        EVENT_NOT_HANDLED_ERR
    );
    assert_eq!(newer.load(Ordering::SeqCst), 1);
    assert_eq!(older.load(Ordering::SeqCst), 1);

    newer_handler.remove().expect("remove the newer handler");
    drop(older_handler);
    send_hotkey_event(FOREIGN_SIGNATURE, 2);
    assert_eq!(newer.load(Ordering::SeqCst), 1);
    assert_eq!(older.load(Ordering::SeqCst), 1);
    assert_eq!(Arc::strong_count(&older), 1);
    assert_eq!(Arc::strong_count(&newer), 1);
}

#[cfg(feature = "async")]
fn the_dispatcher_passes_on_hotkeys_it_does_not_own() {
    let observed = Arc::new(AtomicUsize::new(0));
    let older_handler = counting_handler(&observed);
    let stream =
        carbonhotkey::async_api::HotKeyEventStream::new(4).expect("stream on the main thread");

    assert_eq!(
        send_hotkey_event(FOREIGN_SIGNATURE, 1),
        EVENT_NOT_HANDLED_ERR
    );
    assert_eq!(
        send_hotkey_event(CRATE_SIGNATURE, u32::MAX),
        EVENT_NOT_HANDLED_ERR
    );
    assert_eq!(observed.load(Ordering::SeqCst), 2);
    assert!(stream.try_next().is_none());
    drop(older_handler);
}

thread_local! {
    static SELF_REMOVING: RefCell<Option<EventHandler>> = const { RefCell::new(None) };
}

fn a_handler_can_drop_itself_inside_its_callback() {
    let calls = Arc::new(AtomicUsize::new(0));
    let checksum = Arc::new(AtomicU64::new(0));
    let payload = vec![7_u8; 4096];
    let handler = {
        let calls = Arc::clone(&calls);
        let checksum = Arc::clone(&checksum);
        install_keyboard_handler(move |_| {
            calls.fetch_add(1, Ordering::SeqCst);
            let this = SELF_REMOVING.with(|slot| slot.borrow_mut().take());
            drop(this);
            let sum = payload.iter().map(|byte| u64::from(*byte)).sum();
            checksum.store(sum, Ordering::SeqCst);
        })
        .expect("keyboard handler on the main thread")
    };
    SELF_REMOVING.with(|slot| *slot.borrow_mut() = Some(handler));

    send_hotkey_event(FOREIGN_SIGNATURE, 7);
    send_hotkey_event(FOREIGN_SIGNATURE, 8);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    assert_eq!(checksum.load(Ordering::SeqCst), 7 * 4096);
    assert_eq!(Arc::strong_count(&calls), 1);
}

fn off_main_registration_is_rejected() {
    let outcome = thread::spawn(|| {
        let hotkey = register_key(KeyCode::F20, Modifier::CMD, |_| {}).map(|_| ());
        let handler = install_keyboard_handler(|_| {}).map(|_| ());
        (hotkey, handler)
    })
    .join()
    .expect("registration thread");
    assert_eq!(outcome.0, Err(HotkeyError::NotMainThread));
    assert_eq!(outcome.1, Err(HotkeyError::NotMainThread));
}

fn a_quit_before_the_loop_starts_is_not_lost() {
    quit_event_loop();
    let started = Instant::now();
    run_event_loop().expect("run event loop");
    assert!(started.elapsed() < Duration::from_secs(1));
}

static MAIN_QUEUE_RAN: AtomicBool = AtomicBool::new(false);

extern "C" fn mark_main_queue_ran(_: *mut c_void) {
    MAIN_QUEUE_RAN.store(unsafe { pthread_main_np() } != 0, Ordering::SeqCst);
}

fn the_event_loop_services_main_queue_work() {
    thread::spawn(|| unsafe {
        dispatch_async_f(
            &raw const _dispatch_main_q,
            core::ptr::null_mut(),
            mark_main_queue_ran,
        );
    })
    .join()
    .expect("dispatch thread");
    let deadline = Instant::now() + Duration::from_secs(5);
    while !MAIN_QUEUE_RAN.load(Ordering::SeqCst) && Instant::now() < deadline {
        run_current_event_loop(Duration::from_millis(50)).expect("run current event loop");
    }
    assert!(MAIN_QUEUE_RAN.load(Ordering::SeqCst));
}

fn reserve_hotkey(options: HotKeyOptions) -> (Hotkey, KeyCode, Modifier) {
    let modifiers = Modifier::CMD | Modifier::SHIFT | Modifier::OPTION | Modifier::CONTROL;
    for key in [KeyCode::F20, KeyCode::F19, KeyCode::F18] {
        match register_key_with_options(key, modifiers, options, |_| {}) {
            Ok(hotkey) => return (hotkey, key, modifiers),
            Err(HotkeyError::AlreadyRegistered) => {}
            Err(error) => panic!("unexpected hotkey registration failure: {error}"),
        }
    }
    panic!("could not reserve a hotkey candidate");
}

fn global_hotkeys_register_dispatch_and_unregister() {
    let (hotkey, _, _) = reserve_hotkey(HotKeyOptions::NO_OPTIONS);
    assert!(hotkey.id() > 0);
    hotkey.unregister().expect("unregister hotkey");

    let (hotkey, key, modifiers) = reserve_hotkey(HotKeyOptions::EXCLUSIVE);
    let duplicate = register_key_with_options(key, modifiers, HotKeyOptions::EXCLUSIVE, |_| {});
    assert!(matches!(duplicate, Err(HotkeyError::AlreadyRegistered)));

    let unregistered = thread::spawn(move || hotkey.unregister())
        .join()
        .expect("unregister thread");
    assert_eq!(unregistered, Err(HotkeyError::NotMainThread));
    let deadline = Instant::now() + Duration::from_secs(5);
    let hotkey = loop {
        run_current_event_loop(Duration::from_millis(50)).expect("run current event loop");
        match register_key_with_options(key, modifiers, HotKeyOptions::EXCLUSIVE, |_| {}) {
            Ok(hotkey) => break hotkey,
            Err(HotkeyError::AlreadyRegistered) if Instant::now() < deadline => {}
            Err(error) => panic!("deferred unregistration never ran: {error}"),
        }
    };

    #[cfg(feature = "async")]
    {
        let stream = carbonhotkey::async_api::HotKeyEventStream::new(4).expect("stream");
        assert_eq!(send_hotkey_event(CRATE_SIGNATURE, hotkey.id()), NO_ERR);
        let event = stream.try_next().expect("stream event");
        assert_eq!(event.hotkey_id(), hotkey.id());
        assert!(event.is_pressed());
    }
    drop(hotkey);
}

fn main() {
    assert_ne!(unsafe { pthread_main_np() }, 0);
    let mut tests: Vec<(&str, fn())> = vec![
        (
            "observers_never_consume_hotkey_events",
            observers_never_consume_hotkey_events,
        ),
        (
            "a_handler_can_drop_itself_inside_its_callback",
            a_handler_can_drop_itself_inside_its_callback,
        ),
        (
            "off_main_registration_is_rejected",
            off_main_registration_is_rejected,
        ),
        (
            "a_quit_before_the_loop_starts_is_not_lost",
            a_quit_before_the_loop_starts_is_not_lost,
        ),
        (
            "the_event_loop_services_main_queue_work",
            the_event_loop_services_main_queue_work,
        ),
    ];
    #[cfg(feature = "async")]
    tests.push((
        "the_dispatcher_passes_on_hotkeys_it_does_not_own",
        the_dispatcher_passes_on_hotkeys_it_does_not_own,
    ));

    for (name, test) in &tests {
        test();
        println!("test {name} ... ok");
    }
    let mut ignored = 0;
    if std::env::var_os(GLOBAL_HOTKEY_OPT_IN).is_some() {
        global_hotkeys_register_dispatch_and_unregister();
        println!("test global_hotkeys_register_dispatch_and_unregister ... ok");
    } else {
        ignored += 1;
        println!(
            "test global_hotkeys_register_dispatch_and_unregister ... ignored, registers system-wide hotkeys; set {GLOBAL_HOTKEY_OPT_IN}=1 to run it"
        );
    }
    println!(
        "\ntest result: ok. {} passed; 0 failed; {ignored} ignored",
        tests.len()
    );
}
