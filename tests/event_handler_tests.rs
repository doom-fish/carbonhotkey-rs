use std::thread;
use std::time::{Duration, Instant};

use carbonhotkey::{
    install_keyboard_handler, quit_event_loop, run_current_event_loop, run_event_loop,
    EventHandler, HotkeyError,
};

#[test]
fn installing_a_handler_off_the_main_thread_is_rejected() {
    let outcome = thread::spawn(|| {
        let installed = install_keyboard_handler(|_| {}).map(|_| ());
        let direct = EventHandler::install(|_| {}).map(|_| ());
        (installed, direct)
    })
    .join()
    .expect("handler thread");
    assert_eq!(outcome.0, Err(HotkeyError::NotMainThread));
    assert_eq!(outcome.1, Err(HotkeyError::NotMainThread));
}

#[test]
fn event_loop_slices_and_early_quits_return() {
    run_current_event_loop(Duration::from_millis(5)).expect("run current event loop");

    quit_event_loop();
    let started = Instant::now();
    run_event_loop().expect("run event loop");
    assert!(started.elapsed() < Duration::from_secs(1));
}
