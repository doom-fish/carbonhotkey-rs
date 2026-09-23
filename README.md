# carbonhotkey

Safe Rust bindings for Carbon's global hotkey APIs on macOS, built on a Swift bridge over HIToolbox/Events.

The safe surface is split into five logical areas:

- `hotkey` — `register`, `register_key`, `register_with_options`, `Hotkey`, `HotKeyOptions`
- `event_handler` — `install_keyboard_handler`, `EventHandler`, `run_current_event_loop`, `run_event_loop`, `quit_event_loop`
- `key_code` — `KeyCode` mapping for every `kVK_*` constant in `Events.h`
- `modifier_flags` — `ModifierFlags` / `Modifier` for Carbon modifier masks
- `async_api` (feature-gated behind `async`) — executor-agnostic async streams for hotkey events via `BoundedAsyncStream<T>`

Raw Carbon FFI remains available behind the `raw-ffi` feature. It stays enabled by default for backward compatibility; use `default-features = false` if you only want the safe Swift-backed API.

Requires macOS 10.13 or later.

## Why Carbon?

Carbon's `RegisterEventHotKey` is still the only public macOS API for global hotkeys that does **not** require Accessibility permission.

- ✅ Permission-free (unlike `CGEventTap`)
- ✅ Lightweight (no polling)
- ✅ Available from command-line tools and agents
- ❌ Limited to `modifier + single key`

## Threads and dispatch

Carbon's hotkey and event-handler functions are not thread safe, and Carbon delivers hotkey events only while the main thread runs its event loop.

- Call `register*`, `install_keyboard_handler` and `EventHandler::install` on the main thread; elsewhere they return `HotkeyError::NotMainThread`. Run `run_event_loop` or `run_current_event_loop` on the main thread, or let an application run loop run there, to receive events.
- `Hotkey` is `Send` and `Sync`. Dropping it on another thread stops its callback at once and schedules the Carbon unregistration on the main queue, which runs the next time the main thread services it; `Hotkey::unregister` returns `HotkeyError::NotMainThread` in that case. `EventHandler` stays on the main thread.
- All hotkeys registered through this crate share one Carbon event handler. It claims only events whose signature and ID belong to a hotkey registered here, delivers each to that hotkey's callback and to every `HotKeyEventStream`, and returns `eventNotHandledErr` for everything else, so handlers installed by other code keep receiving their hotkeys. Handlers from `install_keyboard_handler` observe events and never consume them.
- `quit_event_loop` makes the current or the next `run_event_loop` return, even when it is called before the loop starts.

## Quick start

```rust,no_run
use std::time::Duration;

use carbonhotkey::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let hotkey = register_key(
        KeyCode::ANSI_A,
        Modifier::CMD | Modifier::SHIFT,
        |edge| println!("hotkey edge: {edge:?}"),
    )?;

    run_current_event_loop(Duration::from_millis(250))?;
    hotkey.unregister()?;
    Ok(())
}
```

## Event handlers

If you want to observe the raw hotkey events yourself, install the keyboard event handler directly:

```rust,no_run
use std::time::Duration;

use carbonhotkey::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let handler = install_keyboard_handler(|event| {
        println!("kind={:?} id={} signature=0x{:08X}", event.event_kind(), event.id(), event.signature());
    })?;

    run_current_event_loop(Duration::from_millis(250))?;
    handler.remove()?;
    Ok(())
}
```

## Async Event Streams (requires `async` feature)

For async/await code, use `HotKeyEventStream` to receive hotkey events on another thread. Carbon only delivers them while the main thread runs its event loop, so register on the main thread, consume the stream elsewhere, and keep the main thread in `run_event_loop` (or an app run loop):

```rust,no_run
use std::thread;
use std::time::Duration;

use carbonhotkey::async_api::HotKeyEventStream;
use carbonhotkey::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let hotkey = register_key(KeyCode::ANSI_A, Modifier::CMD | Modifier::OPTION, |_| {})?;
    let stream = HotKeyEventStream::new(16)?;
    let id = hotkey.id();

    thread::spawn(move || {
        pollster::block_on(async {
            while let Some(event) = stream.next().await {
                if event.hotkey_id == id && event.is_pressed() {
                    println!("Cmd+Option+A pressed!");
                }
            }
        });
    });
    thread::spawn(|| {
        thread::sleep(Duration::from_secs(30));
        quit_event_loop();
    });

    run_event_loop()?;
    hotkey.unregister()?;
    Ok(())
}
```

The stream is **executor-agnostic** — works with tokio, async-std, smol, or any other async runtime. Use `pollster::block_on` if you need async in a sync context. Every stream receives the events of every hotkey registered through this crate; filter on `hotkey_id`.

## Key codes and modifiers

`KeyCode` maps every `kVK_*` constant from `Events.h`, and `ModifierFlags` mirrors Carbon's modifier masks. Key codes name physical key positions, not characters: the `ANSI_*` constants follow the US ANSI layout, so `KeyCode::ANSI_A` is the key labelled Q on a French AZERTY keyboard.

```rust
use carbonhotkey::{KeyCode, Modifier, ModifierFlags};

assert_eq!(KeyCode::from_name("kvk_escape"), Some(KeyCode::ESCAPE));
assert!(KeyCode::COMMAND.is_modifier_key());
assert!(ModifierFlags::supported_mask().contains(Modifier::CMD));
assert!((Modifier::SHIFT | ModifierFlags::RIGHT_OPTION).contains_right_side());
```

## Explicit hotkey options

Carbon exposes `kEventHotKeyNoOptions` for the default behavior and `kEventHotKeyExclusive` for per-process exclusivity. Use `register_with_options` or `register_key_with_options` when you want to pick the option explicitly:

```rust,no_run
use carbonhotkey::{register_key_with_options, HotKeyOptions, KeyCode, Modifier};

let hotkey = register_key_with_options(
    KeyCode::F20,
    Modifier::CMD | Modifier::SHIFT | Modifier::OPTION | Modifier::CONTROL,
    HotKeyOptions::EXCLUSIVE,
    |_| {},
)?;
# hotkey.unregister()?;
# Ok::<(), carbonhotkey::HotkeyError>(())
```

`register` and `register_key` are thin wrappers over `HotKeyOptions::NO_OPTIONS`.

## Raw FFI feature

```toml
[dependencies]
carbonhotkey = { version = "0.6", default-features = false }
```

Enable the legacy raw Carbon FFI surface when you need it:

```toml
[dependencies]
carbonhotkey = { version = "0.6", features = ["raw-ffi"] }
```

## Examples

- `01_register_hotkey` — register and unregister a hotkey through the Swift bridge
- `02_async_hotkey_stream` — async event stream (requires `async` feature)
- `03_key_code_mapping` — inspect the `KeyCode` mapping helpers
- `04_modifier_flags` — inspect Carbon modifier masks

## Coverage audit

`COVERAGE.md` tracks the audited Carbon/HIToolbox hotkey slice, including every `kVK_*` constant from `Events.h` and all exposed hotkey/event-handler symbols.

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
