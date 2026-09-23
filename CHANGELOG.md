# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.6.0] - Unreleased

### Security

- Hotkeys registered by other code in the process are no longer swallowed or misrouted. Every handler the crate installed returned `noErr` for every hotkey event and matched on the hotkey ID alone; since Carbon calls handlers newest first and stops at `noErr`, other code's hotkeys never reached their own handlers, and a `HotKeyEventStream` and `register()` callbacks shadowed each other. One shared dispatcher now claims only events whose signature and ID belong to a hotkey registered through this crate, delivers them to that hotkey's callback and to every stream, and returns `eventNotHandledErr` for all others.
- Carbon hotkey and handler objects are no longer torn down while the main thread dispatches into them. `Hotkey` and the stream's subscription handle were `Send` and `Sync`: dropping a `Hotkey` on another thread called the non-thread-safe `UnregisterEventHotKey` during dispatch, and dropping a stream released its Swift handler while the Carbon callback's `takeUnretainedValue` could still be retaining it (use-after-free). Unregistration now always runs on the main thread, streams no longer own a Carbon handler, and `EventHandler` callbacks keep their state in a `CallbackContext` that the Swift handler retains, so dropping an `EventHandler` inside its own callback is safe.

### Fixed

- The async example and the README snippet delivered no events: the stream was shadowed by the handler installed later, and nothing ran the Carbon event loop on the main thread. They now register first, consume the stream on another thread, run the event loop on the main thread, and stop after 30 seconds even when no event arrives.
- A failed installation of the shared handler is retried on the next registration instead of being cached forever.
- A `quit_event_loop` issued before `run_event_loop` started was lost, so the loop never returned. The quit now stays pending until a run consumes it.
- `HotKeyEventStream::new` with a failed installation returned an inert stream and leaked its sender; streams now subscribe to the shared dispatcher, and capacity zero is an error instead of a panic.
- Handlers return `eventNotHandledErr` for malformed events instead of an error status, which Carbon also treats as handled.

### Changed

- **BREAKING:** `register`, `register_key`, `register_with_options`, `register_key_with_options`, `install_keyboard_handler` and `EventHandler::install` return `HotkeyError::NotMainThread` off the main thread, because the Carbon functions behind them are not thread safe.
- **BREAKING:** dropping a `Hotkey` off the main thread schedules the unregistration on the main queue; `Hotkey::unregister` returns `HotkeyError::NotMainThread` in that case.
- **BREAKING:** `HotKeyEventStream::new` and `subscribe_hotkey_stream` return `Result<HotKeyEventStream, HotkeyError>`. A stream receives the events of every hotkey registered through this crate, and no others.
- **BREAKING:** `run_event_loop` returns `Result<(), HotkeyError>` instead of panicking when `RunCurrentEventLoop` fails.
- **BREAKING:** handlers from `install_keyboard_handler` observe hotkey events and never consume them.
- **BREAKING:** requires `apple-cf` 0.11 (`>=0.11, <0.12`) and `doom-fish-utils` 0.4.1 (`>=0.4.1, <0.5`); `rust-version` is 1.82.

### Added

- `HotkeyError::NotMainThread` and `HotkeyError::InvalidArgument`.

### Removed

- The separate Swift stream handler (`carbonhotkey_hotkey_stream_subscribe` / `_unsubscribe`); streams use the shared dispatcher.

## [0.5.2] - 2026-06-06

### Fixed

- Contained panics in the Carbon hotkey callbacks and ignored callbacks with a null context; removed the vestigial Swift bridge C header.

## [0.5.1] - 2026-05-20

- Widen `doom-fish-utils` dependency bound to `<0.4` so the 0.3.x SPSC-ring release resolves cleanly. No source changes.

## [0.5.0] - 2026-05-18

### Changed

- Added `apple-cf` (`>=0.9, <0.10`) and re-exported `OSStatus` / `OSType` from `apple_cf::raw`, removing the remaining crate-local duplicates.

## [0.4.2] - 2026-05-18

- Widen doom-fish-utils version bound to `<0.3` so 0.2.x resolves.

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
