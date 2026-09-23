//! Global keyboard hotkey registration via Carbon's `RegisterEventHotKey`.

use core::ffi::c_void;
use core::ptr::NonNull;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard, OnceLock, PoisonError};

#[cfg(feature = "async")]
use doom_fish_utils::stream::AsyncStreamSender;

#[cfg(feature = "async")]
use crate::async_api::StreamHotKeyEvent;
use crate::bridge_ffi;
use crate::error::HotkeyError;
use crate::event_handler::{HotKeyEventKind, KEYBOARD_EVENT_CLASS};
use crate::key_code::KeyCode;
use crate::modifier_flags::ModifierFlags;

const VENDOR_SIGNATURE: u32 = 0x646f_6f6d;
const EVENT_HOTKEY_EXISTS_ERR: i32 = -9878;
const DISPOSE_DEFERRED: i32 = 1;

type Callback = dyn Fn(HotkeyEdge) + Send + Sync + 'static;

bitflags::bitflags! {
    /// Additional Carbon hotkey-registration options.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
    pub struct HotKeyOptions: u32 {
        /// Mirror Carbon's `kEventHotKeyNoOptions` constant.
        const NO_OPTIONS = 0;
        /// Request per-process exclusive registration.
        const EXCLUSIVE = 1 << 0;
    }
}

/// Whether the hotkey fired on key down or key up.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HotkeyEdge {
    Pressed,
    Released,
}

/// One registered hotkey. Drops the registration when this value drops.
pub struct Hotkey {
    handle: Option<NonNull<c_void>>,
    id: u32,
}

unsafe impl Send for Hotkey {}
unsafe impl Sync for Hotkey {}

impl Drop for Hotkey {
    fn drop(&mut self) {
        let _ = self.release();
    }
}

impl Hotkey {
    /// The internal hotkey identifier used by Carbon.
    #[must_use]
    pub const fn id(&self) -> u32 {
        self.id
    }

    /// Unregister the hotkey. On the main thread this happens immediately;
    /// elsewhere it is scheduled on the main queue.
    ///
    /// # Errors
    ///
    /// Returns [`HotkeyError::UnregisterFailed`] if Carbon refuses to
    /// unregister the hotkey, and [`HotkeyError::NotMainThread`] when the
    /// unregistration was scheduled on the main queue instead.
    pub fn unregister(mut self) -> Result<(), HotkeyError> {
        match self.release() {
            0 => Ok(()),
            DISPOSE_DEFERRED => Err(HotkeyError::NotMainThread),
            status => Err(HotkeyError::UnregisterFailed(status)),
        }
    }

    fn release(&mut self) -> i32 {
        let callback = lock_dispatcher().callbacks.remove(&self.id);
        drop(callback);
        self.handle.take().map_or(0, |handle| unsafe {
            bridge_ffi::carbonhotkey_hotkey_dispose(handle.as_ptr())
        })
    }
}

#[derive(Default)]
struct Dispatcher {
    callbacks: HashMap<u32, Arc<Callback>>,
    #[cfg(feature = "async")]
    streams: Vec<(u64, AsyncStreamSender<StreamHotKeyEvent>)>,
    #[cfg(feature = "async")]
    next_stream: u64,
}

fn lock_dispatcher() -> MutexGuard<'static, Dispatcher> {
    static DISPATCHER: OnceLock<Mutex<Dispatcher>> = OnceLock::new();
    DISPATCHER
        .get_or_init(Mutex::default)
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
}

#[cfg(feature = "async")]
pub(crate) fn subscribe_stream(sender: AsyncStreamSender<StreamHotKeyEvent>) -> u64 {
    let mut dispatcher = lock_dispatcher();
    dispatcher.next_stream += 1;
    let token = dispatcher.next_stream;
    dispatcher.streams.push((token, sender));
    token
}

#[cfg(feature = "async")]
pub(crate) fn unsubscribe_stream(token: u64) {
    let sender = {
        let mut dispatcher = lock_dispatcher();
        dispatcher
            .streams
            .iter()
            .position(|(subscription, _)| *subscription == token)
            .map(|index| dispatcher.streams.swap_remove(index))
    };
    drop(sender);
}

fn route(event_class: u32, event_kind: u32, signature: u32, hotkey_id: u32) -> bool {
    if event_class != KEYBOARD_EVENT_CLASS || signature != VENDOR_SIGNATURE {
        return false;
    }
    let Some(kind) = HotKeyEventKind::from_raw(event_kind) else {
        return false;
    };
    let dispatcher = lock_dispatcher();
    let Some(callback) = dispatcher.callbacks.get(&hotkey_id).cloned() else {
        return false;
    };
    #[cfg(feature = "async")]
    let streams: Vec<_> = dispatcher
        .streams
        .iter()
        .map(|(_, sender)| sender.clone())
        .collect();
    drop(dispatcher);

    #[cfg(feature = "async")]
    for sender in streams {
        sender.push(StreamHotKeyEvent {
            hotkey_id,
            event_kind,
            event_class,
            signature,
        });
    }
    let edge = match kind {
        HotKeyEventKind::Pressed => HotkeyEdge::Pressed,
        HotKeyEventKind::Released => HotkeyEdge::Released,
    };
    doom_fish_utils::panic_safe::catch_user_panic("hotkey callback", || callback(edge));
    true
}

unsafe extern "C" fn dispatch_hotkey_event(
    event_class: u32,
    event_kind: u32,
    signature: u32,
    hotkey_id: u32,
    _context: *mut c_void,
) -> bool {
    doom_fish_utils::panic_safe::catch_user_panic_result("hotkey dispatcher", || {
        route(event_class, event_kind, signature, hotkey_id)
    })
    .unwrap_or(false)
}

fn install_once(installed: &Mutex<bool>, install: impl FnOnce() -> i32) -> Result<(), HotkeyError> {
    let mut installed = installed.lock().unwrap_or_else(PoisonError::into_inner);
    let status = if *installed { 0 } else { install() };
    *installed = status == 0;
    drop(installed);
    if status == 0 {
        Ok(())
    } else {
        Err(HotkeyError::HandlerInstallFailed(status))
    }
}

pub(crate) fn ensure_dispatcher_installed() -> Result<(), HotkeyError> {
    static INSTALLED: Mutex<bool> = Mutex::new(false);
    install_once(&INSTALLED, || unsafe {
        bridge_ffi::carbonhotkey_dispatcher_install(Some(dispatch_hotkey_event))
    })
}

fn next_id() -> u32 {
    use std::sync::atomic::{AtomicU32, Ordering};

    static NEXT_ID: AtomicU32 = AtomicU32::new(1);
    NEXT_ID.fetch_add(1, Ordering::SeqCst)
}

/// Register a global hotkey using a raw Carbon key code.
///
/// # Errors
///
/// * [`HotkeyError::HandlerInstallFailed`] if the shared keyboard event
///   handler could not be installed.
/// * [`HotkeyError::AlreadyRegistered`] if the same hotkey already exists.
/// * [`HotkeyError::RegisterFailed`] for other Carbon failures.
pub fn register<F>(
    keycode: u16,
    modifiers: ModifierFlags,
    callback: F,
) -> Result<Hotkey, HotkeyError>
where
    F: Fn(HotkeyEdge) + Send + Sync + 'static,
{
    register_with_options(keycode, modifiers, HotKeyOptions::NO_OPTIONS, callback)
}

/// Register a global hotkey using a typed [`KeyCode`].
///
/// # Errors
///
/// Mirrors [`register`].
pub fn register_key<F>(
    keycode: KeyCode,
    modifiers: ModifierFlags,
    callback: F,
) -> Result<Hotkey, HotkeyError>
where
    F: Fn(HotkeyEdge) + Send + Sync + 'static,
{
    register(keycode.raw(), modifiers, callback)
}

/// Register a global hotkey with explicit Carbon options.
///
/// # Errors
///
/// * [`HotkeyError::HandlerInstallFailed`] if the shared keyboard event
///   handler could not be installed.
/// * [`HotkeyError::AlreadyRegistered`] if the same hotkey already exists.
/// * [`HotkeyError::RegisterFailed`] for other Carbon failures.
pub fn register_with_options<F>(
    keycode: u16,
    modifiers: ModifierFlags,
    options: HotKeyOptions,
    callback: F,
) -> Result<Hotkey, HotkeyError>
where
    F: Fn(HotkeyEdge) + Send + Sync + 'static,
{
    if unsafe { bridge_ffi::pthread_main_np() } == 0 {
        return Err(HotkeyError::NotMainThread);
    }
    ensure_dispatcher_installed()?;

    let id = next_id();
    lock_dispatcher().callbacks.insert(id, Arc::new(callback));

    let mut handle = core::ptr::null_mut();
    let status = unsafe {
        bridge_ffi::carbonhotkey_hotkey_register(
            u32::from(keycode),
            modifiers.bits(),
            VENDOR_SIGNATURE,
            id,
            options.bits(),
            &raw mut handle,
        )
    };

    let Some(handle) = NonNull::new(handle).filter(|_| status == 0) else {
        let callback = lock_dispatcher().callbacks.remove(&id);
        drop(callback);
        return Err(match status {
            EVENT_HOTKEY_EXISTS_ERR => HotkeyError::AlreadyRegistered,
            0 => HotkeyError::RegisterFailed(-50),
            status => HotkeyError::RegisterFailed(status),
        });
    };
    Ok(Hotkey {
        handle: Some(handle),
        id,
    })
}

/// Register a global hotkey with explicit Carbon options and a typed [`KeyCode`].
///
/// # Errors
///
/// Mirrors [`register_with_options`].
pub fn register_key_with_options<F>(
    keycode: KeyCode,
    modifiers: ModifierFlags,
    options: HotKeyOptions,
    callback: F,
) -> Result<Hotkey, HotkeyError>
where
    F: Fn(HotkeyEdge) + Send + Sync + 'static,
{
    register_with_options(keycode.raw(), modifiers, options, callback)
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::{Arc, Mutex};

    use super::{install_once, lock_dispatcher, route, HotkeyEdge, VENDOR_SIGNATURE};
    use crate::error::HotkeyError;
    use crate::event_handler::KEYBOARD_EVENT_CLASS;

    const PRESSED: u32 = 5;
    const RELEASED: u32 = 6;
    const FOREIGN_SIGNATURE: u32 = u32::from_be_bytes(*b"othr");

    #[test]
    fn a_failed_install_is_retried() {
        let installed = Mutex::new(false);
        let attempts = AtomicUsize::new(0);
        let attempt = |status| {
            attempts.fetch_add(1, Ordering::SeqCst);
            status
        };

        assert_eq!(
            install_once(&installed, || attempt(-50)),
            Err(HotkeyError::HandlerInstallFailed(-50))
        );
        assert_eq!(install_once(&installed, || attempt(0)), Ok(()));
        assert_eq!(install_once(&installed, || attempt(-50)), Ok(()));
        assert_eq!(attempts.load(Ordering::SeqCst), 2);
    }

    #[test]
    fn only_registered_hotkeys_with_our_signature_are_claimed() {
        let id = 0x7fff_0100;
        let edges = Arc::new(Mutex::new(Vec::new()));
        let sink = Arc::clone(&edges);
        lock_dispatcher()
            .callbacks
            .insert(id, Arc::new(move |edge| sink.lock().unwrap().push(edge)));

        assert!(route(KEYBOARD_EVENT_CLASS, PRESSED, VENDOR_SIGNATURE, id));
        assert!(route(KEYBOARD_EVENT_CLASS, RELEASED, VENDOR_SIGNATURE, id));
        assert!(!route(KEYBOARD_EVENT_CLASS, PRESSED, FOREIGN_SIGNATURE, id));
        assert!(!route(
            KEYBOARD_EVENT_CLASS,
            PRESSED,
            VENDOR_SIGNATURE,
            0x7fff_01ff
        ));
        assert!(!route(0x6d6f_7573, PRESSED, VENDOR_SIGNATURE, id));
        assert!(!route(KEYBOARD_EVENT_CLASS, 99, VENDOR_SIGNATURE, id));
        assert_eq!(
            *edges.lock().unwrap(),
            vec![HotkeyEdge::Pressed, HotkeyEdge::Released]
        );

        let removed = lock_dispatcher().callbacks.remove(&id);
        drop(removed);
        assert!(!route(KEYBOARD_EVENT_CLASS, PRESSED, VENDOR_SIGNATURE, id));
        assert_eq!(Arc::strong_count(&edges), 1);
    }

    #[test]
    fn a_panicking_callback_still_claims_its_hotkey() {
        let id = 0x7fff_0200;
        lock_dispatcher()
            .callbacks
            .insert(id, Arc::new(|_| panic!("hotkey callback panic")));
        assert!(route(KEYBOARD_EVENT_CLASS, PRESSED, VENDOR_SIGNATURE, id));
        let removed = lock_dispatcher().callbacks.remove(&id);
        drop(removed);
    }

    #[cfg(feature = "async")]
    #[test]
    fn one_dispatch_reaches_the_callback_and_every_stream() {
        use crate::async_api::{HotKeyEventStream, StreamHotKeyEvent};

        fn events_for(stream: &HotKeyEventStream, id: u32) -> Vec<StreamHotKeyEvent> {
            std::iter::from_fn(|| stream.try_next())
                .filter(|event| event.hotkey_id() == id)
                .collect()
        }

        let id = 0x7fff_0300;
        let calls = Arc::new(AtomicUsize::new(0));
        let counter = Arc::clone(&calls);
        lock_dispatcher().callbacks.insert(
            id,
            Arc::new(move |_| {
                counter.fetch_add(1, Ordering::SeqCst);
            }),
        );
        let first = HotKeyEventStream::new(8).expect("first stream");
        let second = HotKeyEventStream::new(8).expect("second stream");

        assert!(route(KEYBOARD_EVENT_CLASS, PRESSED, VENDOR_SIGNATURE, id));
        assert_eq!(calls.load(Ordering::SeqCst), 1);
        for stream in [&first, &second] {
            let events = events_for(stream, id);
            assert_eq!(events.len(), 1);
            assert!(events[0].is_pressed());
            assert_eq!(events[0].signature(), VENDOR_SIGNATURE);
            assert_eq!(events[0].event_class(), KEYBOARD_EVENT_CLASS);
        }

        drop(second);
        assert!(route(KEYBOARD_EVENT_CLASS, RELEASED, VENDOR_SIGNATURE, id));
        let events = events_for(&first, id);
        assert_eq!(events.len(), 1);
        assert!(events[0].is_released());
        assert_eq!(calls.load(Ordering::SeqCst), 2);

        let removed = lock_dispatcher().callbacks.remove(&id);
        drop(removed);
    }
}
