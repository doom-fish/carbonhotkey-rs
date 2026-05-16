# Changelog

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
