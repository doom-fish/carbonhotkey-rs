//! Install a keyboard hotkey event handler for a short sample window.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;

use carbonhotkey::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let seen = Arc::new(AtomicUsize::new(0));
    let seen_in_callback = Arc::clone(&seen);

    let handler = install_keyboard_handler(move |event| {
        seen_in_callback.fetch_add(1, Ordering::SeqCst);
        println!(
            "event kind={:?} id={} signature=0x{:08X}",
            event.event_kind(),
            event.id(),
            event.signature()
        );
    })?;

    run_current_event_loop(Duration::from_millis(10))?;
    handler.remove()?;
    println!(
        "observed {} hotkey events during the sample window",
        seen.load(Ordering::SeqCst)
    );
    Ok(())
}
