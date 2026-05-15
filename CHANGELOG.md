# Changelog

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
