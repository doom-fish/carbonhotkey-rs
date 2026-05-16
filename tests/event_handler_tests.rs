use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;

use carbonhotkey::{
    install_keyboard_handler, quit_event_loop, run_current_event_loop, EventHandler,
    HotKeyEventKind,
};

#[test]
fn install_and_remove_handler() {
    let seen = Arc::new(AtomicUsize::new(0));
    let seen_in_callback = Arc::clone(&seen);

    let handler: EventHandler = install_keyboard_handler(move |event| {
        seen_in_callback.fetch_add(1, Ordering::SeqCst);
        assert!(matches!(
            event.event_kind(),
            HotKeyEventKind::Pressed | HotKeyEventKind::Released
        ));
    })
    .expect("install handler");

    run_current_event_loop(Duration::from_millis(5)).expect("run current event loop");
    handler.remove().expect("remove handler");
    assert_eq!(seen.load(Ordering::SeqCst), 0);
    quit_event_loop();
}
