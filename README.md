# carbonhotkey

Safe Rust bindings for Carbon's `RegisterEventHotKey` API on macOS — register **global keyboard shortcuts** that fire even when your app isn't focused.

> **Status:** experimental. v0.1 ships hotkey registration with press / release callbacks. v0.2 will add per-process exclusivity flags (`kEventHotKeyExclusive`), modifier-only chords, and key-up debouncing.

Pure C — **zero Swift bridge** (like `cgevents`, `imageio`, `videotoolbox`).

## Why Carbon (in 2026)?

Carbon's `RegisterEventHotKey` is the only public API for **global hotkeys** on macOS that doesn't require Accessibility permission. It's:
- ✅ Permission-free (unlike `CGEventTap`)
- ✅ Lightweight (no run-loop polling)
- ✅ Still actively supported by Apple in macOS 26
- ❌ Limited to "modifier + single key" combinations (no chords like Cmd-K-Cmd-K)

For more flexible matching (chord sequences, regex on text, conditional drops), pair with [`cgevents`](https://github.com/doom-fish/cgevents-rs)'s `EventTap`.

## Quick start

```rust,no_run
use carbonhotkey::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Register Cmd+Shift+A as a global hotkey.
    // Keep the returned Hotkey alive — Drop unregisters automatically.
    let _hk = register(0x00 /* A */, Modifier::CMD | Modifier::SHIFT, |edge| {
        match edge {
            HotkeyEdge::Pressed  => println!("Cmd+Shift+A pressed!"),
            HotkeyEdge::Released => println!("Cmd+Shift+A released"),
        }
    })?;

    // Run the Carbon event loop so callbacks fire. Blocks.
    run_event_loop();
    Ok(())
}
```

## Keycodes

Use the same virtual-keycode space as [`cgevents`](https://github.com/doom-fish/cgevents-rs)'s `Keycode` module:
```rust,no_run
use carbonhotkey::*;
let _ = register(0x00, Modifier::CMD, |_| {});  // A
let _ = register(0x7A, Modifier::empty(), |_| {});  // F1
let _ = register(0x35, Modifier::CMD | Modifier::OPTION, |_| {});  // ESC
```

## Pipeline composition

```text
carbonhotkey (global trigger) ──► your action
        │
        └──► cgevents::EventTap (full keyboard interception, needs Accessibility)
        └──► screencapturekit (start recording)
        └──► foundation-models (run a model on selected text)
```

## Roadmap

- [x] `register(keycode, modifiers, callback) -> Hotkey`
- [x] `Modifier` bitflags (CMD/SHIFT/OPTION/CONTROL/CAPS)
- [x] `HotkeyEdge::{Pressed, Released}` callbacks
- [x] `run_event_loop` / `quit_event_loop`
- [x] Auto-unregister via `Drop`
- [ ] `kEventHotKeyExclusive` option (per-process exclusivity)
- [ ] Hotkey chord sequences (Cmd-K Cmd-K)
- [ ] Lookup by combination (`unregister_all_with_keycode(...)`)
- [ ] Async callback variant

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
