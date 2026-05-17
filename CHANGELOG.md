# Changelog

## [0.4.1] - 2026-05-17

### Fixed

- `Hotkey::drop()` now calls `carbonhotkey_hotkey_unregister()` before `carbonhotkey_hotkey_release()` to properly clean up process-global handler slots, preventing handler leaks.
- `EventHandler::drop()` now calls `carbonhotkey_event_handler_remove()` before `carbonhotkey_event_handler_release()` to properly clean up process-global event handlers, preventing handler leaks.

## [0.4.0] - 2026-05-16

### Added

- **Tier-2 Async API:** New `async_api` module (gated by `async` feature) providing executor-agnostic async streams via `doom_fish_utils::stream::BoundedAsyncStream<T>`.
- `HotKeyEventStream` — async stream of multiplexed hotkey events (both press and release), backed by a single global event handler.
- `StreamHotKeyEvent` — event type carrying hotkey ID, event kind, and metadata.
- `subscribe_hotkey_stream()` constructor for creating event streams with configurable buffer capacity.
- Example `02_async_hotkey_stream.rs` demonstrating pollster integration and multi-hotkey listening.
- Test suite `async_stream_tests.rs` covering stream creation, event properties, and Drop semantics.

### Changed

- `Cargo.toml` now depends on `doom-fish-utils` (with `async` feature).
- Dev-dependencies now include `pollster = "0.3"` for example support.

## [0.3.1] - 2026-05-16

### Added

- `HotKeyOptions::NO_OPTIONS` and `ffi::kEventHotKeyNoOptions`, giving the safe and raw APIs a named mirror of Carbon's default hotkey-registration option.

### Changed

- `COVERAGE_AUDIT.md` now reports 100% coverage for the audited Carbon hotkey slice.

## [0.3.0] - 2026-05-16

### Added

- Swift bridge package (`swift-bridge/`) wrapping Carbon's hotkey and keyboard event-handler APIs.
- Safe `event_handler` module with `EventHandler`, `install_keyboard_handler`, `run_current_event_loop`, `run_event_loop`, and `quit_event_loop`.
- Safe `key_code` module covering every `kVK_*` constant from `Events.h`.
- Safe `modifier_flags` module covering Carbon modifier masks, including the right-side variants.
- `HotKeyOptions::EXCLUSIVE`, `register_key`, `register_with_options`, and `register_key_with_options`.
- Numbered examples for every logical area and per-area smoke tests.
- `COVERAGE.md` auditing the Carbon/HIToolbox hotkey slice.

### Changed

- The safe API now talks to Carbon through the Swift bridge instead of direct raw FFI calls.
- Raw Carbon FFI is now gated behind the `raw-ffi` feature (still enabled by default for backward compatibility).

## [0.1.0] - Initial release

### Added

- `register(keycode, modifiers, callback) -> Hotkey` — installs a global
  keyboard shortcut via `RegisterEventHotKey`. The returned `Hotkey`
  guard auto-unregisters on drop.
- `Modifier` bitflags: `CMD`, `SHIFT`, `OPTION`, `CONTROL`, `CAPS`.
- `HotkeyEdge::{Pressed, Released}` callback parameter.
- `run_event_loop` / `quit_event_loop` to drive the Carbon event loop.
- One-time `InstallEventHandler` registration handled internally.
- `HotkeyError` enum: `RegisterFailed`, `AlreadyRegistered`,
  `HandlerInstallFailed`, `UnregisterFailed`.

### Why?

Carbon's `RegisterEventHotKey` is the only public macOS API for global
hotkeys that doesn't require Accessibility permission (unlike
`CGEventTap`).
