//! Register Cmd+Shift+T as a global hotkey for a short sample window.
//!
//! Run: `cargo run --example 01_register_hotkey`

use std::time::Duration;

use carbonhotkey::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let hotkey = register_key(
        KeyCode::ANSI_T,
        Modifier::CMD | Modifier::SHIFT,
        |edge| match edge {
            HotkeyEdge::Pressed => println!("Cmd+Shift+T pressed"),
            HotkeyEdge::Released => println!("Cmd+Shift+T released"),
        },
    )?;

    println!("Registered Cmd+Shift+T as hotkey id {}.", hotkey.id());
    run_current_event_loop(Duration::from_millis(10))?;
    hotkey.unregister()?;
    Ok(())
}
