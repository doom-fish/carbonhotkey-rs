//! Register Cmd+Shift+T as a global hotkey. The callback prints when
//! pressed, even if focus is on a completely different app.
//!
//! Run: `cargo run --example 01_register_hotkey`
//!
//! Press Ctrl-C to exit.

use carbonhotkey::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _hk = register(0x11 /* T */, Modifier::CMD | Modifier::SHIFT, |edge| {
        match edge {
            HotkeyEdge::Pressed => println!("Cmd+Shift+T pressed"),
            HotkeyEdge::Released => println!("Cmd+Shift+T released"),
        }
    })?;
    println!("Listening for Cmd+Shift+T globally. Press Ctrl-C to exit.");
    run_event_loop();
    Ok(())
}
