//! Async hotkey event stream example.
//!
//! Demonstrates using the async Stream API to listen for hotkey events
//! without blocking.

use carbonhotkey::prelude::*;

#[cfg(feature = "async")]
fn main() {
    use carbonhotkey::async_api::HotKeyEventStream;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::time::Duration;

    // Create the event stream
    let stream = HotKeyEventStream::new(16);

    // Register a couple of hotkeys
    let hotkey1 = register_key(KeyCode::ANSI_A, Modifier::CMD, |_| {})
        .expect("Failed to register hotkey 1");
    let hotkey2 = register_key(KeyCode::ANSI_B, Modifier::CMD | Modifier::OPTION, |_| {})
        .expect("Failed to register hotkey 2");

    println!("Registered hotkeys:");
    println!("  - Cmd+A (id: {})", hotkey1.id());
    println!("  - Cmd+Option+B (id: {})", hotkey2.id());
    println!("\nListening for hotkey events for 30 seconds...");
    println!("Try pressing Cmd+A and Cmd+Option+B in the terminal or other apps.\n");

    let should_stop = Arc::new(AtomicBool::new(false));
    let should_stop_clone = Arc::clone(&should_stop);

    // Spawn a thread to stop after 30 seconds
    std::thread::spawn(move || {
        std::thread::sleep(Duration::from_secs(30));
        should_stop_clone.store(true, Ordering::SeqCst);
    });

    // Use pollster to run async code in a sync context
    pollster::block_on(async {
        while let Some(event) = stream.next().await {
            match event.hotkey_id {
                id if id == hotkey1.id() => {
                    if event.is_pressed() {
                        println!("✓ Cmd+A pressed!");
                    } else {
                        println!("✓ Cmd+A released");
                    }
                }
                id if id == hotkey2.id() => {
                    if event.is_pressed() {
                        println!("✓ Cmd+Option+B pressed!");
                    } else {
                        println!("✓ Cmd+Option+B released");
                    }
                }
                id => {
                    println!("  Event for unknown hotkey id={}, kind={}", id, event.event_kind);
                }
            }

            if should_stop.load(Ordering::SeqCst) {
                println!("\nTimeout reached, closing stream.");
                break;
            }
        }
    });

    println!("Stream closed. Exiting.");
}

#[cfg(not(feature = "async"))]
fn main() {
    eprintln!("This example requires the 'async' feature.");
    eprintln!("Run with: cargo run --example 02_async_hotkey_stream --features async");
    std::process::exit(1);
}
