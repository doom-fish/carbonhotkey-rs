//! Async hotkey event stream example.
//!
//! Demonstrates using the async Stream API to listen for hotkey events
//! without blocking.

#[cfg(feature = "async")]
fn main() -> Result<(), carbonhotkey::HotkeyError> {
    use carbonhotkey::async_api::HotKeyEventStream;
    use carbonhotkey::prelude::*;
    use std::thread;
    use std::time::Duration;

    let modifiers = Modifier::CMD | Modifier::OPTION | Modifier::CONTROL;
    let hotkey_a = register_key(KeyCode::ANSI_A, modifiers, |_| {})?;
    let hotkey_b = register_key(KeyCode::ANSI_B, modifiers, |_| {})?;
    let stream = HotKeyEventStream::new(16)?;
    let (id_a, id_b) = (hotkey_a.id(), hotkey_b.id());

    println!("Registered hotkeys:");
    println!("  - Control+Option+Command+A (id: {id_a})");
    println!("  - Control+Option+Command+B (id: {id_b})");
    println!("\nListening for hotkey events for 30 seconds...");
    println!("Try pressing them in the terminal or other apps.\n");

    thread::spawn(move || {
        pollster::block_on(async {
            while let Some(event) = stream.next().await {
                let name = match event.hotkey_id {
                    id if id == id_a => "Control+Option+Command+A",
                    id if id == id_b => "Control+Option+Command+B",
                    _ => "another hotkey",
                };
                let edge = if event.is_pressed() {
                    "pressed"
                } else {
                    "released"
                };
                println!("✓ {name} {edge}");
            }
        });
    });
    thread::spawn(|| {
        thread::sleep(Duration::from_secs(30));
        quit_event_loop();
    });

    run_event_loop()?;
    hotkey_a.unregister()?;
    hotkey_b.unregister()?;
    println!("\nTimeout reached. Exiting.");
    Ok(())
}

#[cfg(not(feature = "async"))]
fn main() {
    eprintln!("This example requires the 'async' feature.");
    eprintln!("Run with: cargo run --example 02_async_hotkey_stream --features async");
    std::process::exit(1);
}
