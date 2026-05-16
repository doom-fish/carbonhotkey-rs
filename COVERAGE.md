# carbonhotkey coverage audit

Audited headers:

- `Carbon.framework/Frameworks/HIToolbox.framework/Headers/CarbonEventsCore.h`
- `Carbon.framework/Frameworks/HIToolbox.framework/Headers/CarbonEvents.h`
- `Carbon.framework/Frameworks/HIToolbox.framework/Headers/Events.h`

`carbonhotkey` targets the global-hotkey slice of Carbon/HIToolbox rather than the entire Carbon framework. The rows below cover every hotkey, keyboard-event-handler, key-code, and modifier symbol in scope for this crate. Wider Carbon surface area (menus, windows, controls, text, drag-and-drop, etc.) is out of scope and therefore not wrapped here.

## Status legend

- ✅ implemented
- ⏭️ skipped (with reason)

## HotKey

| Header | Apple symbol | Status | Rust surface | Notes |
| --- | --- | --- | --- | --- |
| CarbonEvents.h | `EventHotKeyID` | ✅ implemented | `ffi::EventHotKeyID`, `Hotkey` | bridged through `HotKey.swift` |
| CarbonEvents.h | `EventHotKeyRef` | ✅ implemented | `ffi::EventHotKeyRef`, `Hotkey` | owned by the Swift handle |
| CarbonEvents.h | `kEventHotKeyNoOptions` | ✅ implemented | `HotKeyOptions::NO_OPTIONS`, `ffi::kEventHotKeyNoOptions` | named mirror of Carbon's default registration mode |
| CarbonEvents.h | `kEventHotKeyExclusive` | ✅ implemented | `HotKeyOptions::EXCLUSIVE`, `ffi::kEventHotKeyExclusive` | exposed in safe and raw surfaces |
| CarbonEvents.h | `eventHotKeyExistsErr` | ✅ implemented | `HotkeyError::AlreadyRegistered`, `ffi::eventHotKeyExistsErr` | maps exclusive conflicts to a Rust enum |
| CarbonEvents.h | `RegisterEventHotKey` | ✅ implemented | `register`, `register_key`, `register_with_options`, `register_key_with_options` | invoked via the Swift bridge |
| CarbonEvents.h | `UnregisterEventHotKey` | ✅ implemented | `Hotkey::unregister`, `Drop` | released through the Swift bridge |

## EventHandler

| Header | Apple symbol | Status | Rust surface | Notes |
| --- | --- | --- | --- | --- |
| CarbonEventsCore.h | `EventTargetRef` | ✅ implemented | `ffi::EventTargetRef`, bridge internals | used with `GetApplicationEventTarget()` |
| CarbonEventsCore.h | `EventHandlerRef` | ✅ implemented | `ffi::EventHandlerRef`, `EventHandler` | owned by the Swift handle |
| CarbonEventsCore.h | `EventHandlerCallRef` | ✅ implemented | `ffi::EventHandlerCallRef`, bridge callback | forwarded through `EventHandler.swift` |
| CarbonEventsCore.h | `EventRef` | ✅ implemented | `ffi::EventRef`, bridge callback | decoded into `HotKeyEvent` |
| CarbonEventsCore.h | `EventTypeSpec` | ✅ implemented | `ffi::EventTypeSpec`, bridge install list | registers `Pressed` + `Released` |
| CarbonEvents.h | `kEventClassKeyboard` | ✅ implemented | `HotKeyEvent::is_keyboard_hotkey`, raw constants | keyboard hotkey event class |
| CarbonEvents.h | `kEventHotKeyPressed` | ✅ implemented | `HotKeyEventKind::Pressed`, `HotkeyEdge::Pressed` | hotkey press edge |
| CarbonEvents.h | `kEventHotKeyReleased` | ✅ implemented | `HotKeyEventKind::Released`, `HotkeyEdge::Released` | hotkey release edge |
| CarbonEvents.h | `kEventParamDirectObject` | ✅ implemented | raw constants, bridge internals | parameter used to fetch `EventHotKeyID` |
| CarbonEvents.h | `typeEventHotKeyID` | ✅ implemented | raw constants, bridge internals | expected parameter type |
| CarbonEvents.h | `GetApplicationEventTarget` | ✅ implemented | bridge internals, raw FFI | installs both the handler and hotkeys on the app target |
| CarbonEventsCore.h | `InstallEventHandler` | ✅ implemented | `EventHandler::install`, `install_keyboard_handler` | keyboard handler bridge |
| CarbonEventsCore.h | `RemoveEventHandler` | ✅ implemented | `EventHandler::remove`, `Drop` | handler cleanup |
| CarbonEventsCore.h | `GetEventClass` | ✅ implemented | bridge internals | decodes `HotKeyEvent::event_class()` |
| CarbonEventsCore.h | `GetEventKind` | ✅ implemented | bridge internals | decodes `HotKeyEventKind` |
| CarbonEventsCore.h | `GetEventParameter` | ✅ implemented | bridge internals | extracts `EventHotKeyID` from callbacks |
| CarbonEventsCore.h | `RunCurrentEventLoop` | ✅ implemented | `run_current_event_loop`, `run_event_loop` | polling event loop for headless-safe examples/tests |
| CarbonEvents.h | `RunApplicationEventLoop` | ⏭️ skipped | — | 32-bit-only in the headers; safe API polls `RunCurrentEventLoop` instead |
| CarbonEvents.h | `QuitApplicationEventLoop` | ⏭️ skipped | — | 32-bit-only in the headers; safe API uses a Rust-side quit flag |
| CarbonEvents.h | `InstallApplicationEventHandler` | ⏭️ skipped | — | convenience macro over `InstallEventHandler` once the bridge already targets `GetApplicationEventTarget()` |

## ModifierFlags

| Header | Apple symbol | Status | Rust surface | Notes |
| --- | --- | --- | --- | --- |
| Events.h | `EventModifiers` | ⏭️ skipped | — | legacy 16-bit type; the safe API exposes the `UInt32` masks used by `RegisterEventHotKey` |
| Events.h | `cmdKeyBit` | ✅ implemented | `ffi::cmdKeyBit`, `ModifierFlags::CMD` | command bit position |
| Events.h | `shiftKeyBit` | ✅ implemented | `ffi::shiftKeyBit`, `ModifierFlags::SHIFT` | shift bit position |
| Events.h | `alphaLockBit` | ✅ implemented | `ffi::alphaLockBit`, `ModifierFlags::CAPS` | caps-lock bit position |
| Events.h | `optionKeyBit` | ✅ implemented | `ffi::optionKeyBit`, `ModifierFlags::OPTION` | option bit position |
| Events.h | `controlKeyBit` | ✅ implemented | `ffi::controlKeyBit`, `ModifierFlags::CONTROL` | control bit position |
| Events.h | `rightShiftKeyBit` | ✅ implemented | `ffi::rightShiftKeyBit`, `ModifierFlags::RIGHT_SHIFT` | right shift bit position |
| Events.h | `rightOptionKeyBit` | ✅ implemented | `ffi::rightOptionKeyBit`, `ModifierFlags::RIGHT_OPTION` | right option bit position |
| Events.h | `rightControlKeyBit` | ✅ implemented | `ffi::rightControlKeyBit`, `ModifierFlags::RIGHT_CONTROL` | right control bit position |
| Events.h | `cmdKey` | ✅ implemented | `ffi::cmdKey`, `ModifierFlags::CMD` | command mask |
| Events.h | `shiftKey` | ✅ implemented | `ffi::shiftKey`, `ModifierFlags::SHIFT` | shift mask |
| Events.h | `alphaLock` | ✅ implemented | `ffi::alphaLock`, `ModifierFlags::CAPS` | caps-lock mask |
| Events.h | `optionKey` | ✅ implemented | `ffi::optionKey`, `ModifierFlags::OPTION` | option mask |
| Events.h | `controlKey` | ✅ implemented | `ffi::controlKey`, `ModifierFlags::CONTROL` | control mask |
| Events.h | `rightShiftKey` | ✅ implemented | `ffi::rightShiftKey`, `ModifierFlags::RIGHT_SHIFT` | right shift mask |
| Events.h | `rightOptionKey` | ✅ implemented | `ffi::rightOptionKey`, `ModifierFlags::RIGHT_OPTION` | right option mask |
| Events.h | `rightControlKey` | ✅ implemented | `ffi::rightControlKey`, `ModifierFlags::RIGHT_CONTROL` | right control mask |
| Events.h | `activeFlagBit` / `activeFlag` | ⏭️ skipped | — | activation state, not a hotkey modifier |
| Events.h | `btnStateBit` / `btnState` | ⏭️ skipped | — | mouse-button state, not a hotkey modifier |
| Events.h | `k*CharCode` constants | ⏭️ skipped | — | character-code tables, not physical key codes |
| Events.h | `k*Unicode` constants | ⏭️ skipped | — | glyph helpers, not physical key codes |

## KeyCode mapping

| Header | Apple symbol | Status | Rust surface | Notes |
| --- | --- | --- | --- | --- |
| Events.h | `kVK_ANSI_A` | ✅ implemented | `KeyCode::ANSI_A` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_ANSI_S` | ✅ implemented | `KeyCode::ANSI_S` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_ANSI_D` | ✅ implemented | `KeyCode::ANSI_D` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_ANSI_F` | ✅ implemented | `KeyCode::ANSI_F` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_ANSI_H` | ✅ implemented | `KeyCode::ANSI_H` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_ANSI_G` | ✅ implemented | `KeyCode::ANSI_G` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_ANSI_Z` | ✅ implemented | `KeyCode::ANSI_Z` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_ANSI_X` | ✅ implemented | `KeyCode::ANSI_X` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_ANSI_C` | ✅ implemented | `KeyCode::ANSI_C` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_ANSI_V` | ✅ implemented | `KeyCode::ANSI_V` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_ANSI_B` | ✅ implemented | `KeyCode::ANSI_B` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_ANSI_Q` | ✅ implemented | `KeyCode::ANSI_Q` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_ANSI_W` | ✅ implemented | `KeyCode::ANSI_W` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_ANSI_E` | ✅ implemented | `KeyCode::ANSI_E` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_ANSI_R` | ✅ implemented | `KeyCode::ANSI_R` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_ANSI_Y` | ✅ implemented | `KeyCode::ANSI_Y` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_ANSI_T` | ✅ implemented | `KeyCode::ANSI_T` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_ANSI_1` | ✅ implemented | `KeyCode::ANSI_1` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_ANSI_2` | ✅ implemented | `KeyCode::ANSI_2` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_ANSI_3` | ✅ implemented | `KeyCode::ANSI_3` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_ANSI_4` | ✅ implemented | `KeyCode::ANSI_4` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_ANSI_6` | ✅ implemented | `KeyCode::ANSI_6` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_ANSI_5` | ✅ implemented | `KeyCode::ANSI_5` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_ANSI_Equal` | ✅ implemented | `KeyCode::ANSI_EQUAL` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_ANSI_9` | ✅ implemented | `KeyCode::ANSI_9` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_ANSI_7` | ✅ implemented | `KeyCode::ANSI_7` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_ANSI_Minus` | ✅ implemented | `KeyCode::ANSI_MINUS` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_ANSI_8` | ✅ implemented | `KeyCode::ANSI_8` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_ANSI_0` | ✅ implemented | `KeyCode::ANSI_0` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_ANSI_RightBracket` | ✅ implemented | `KeyCode::ANSI_RIGHT_BRACKET` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_ANSI_O` | ✅ implemented | `KeyCode::ANSI_O` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_ANSI_U` | ✅ implemented | `KeyCode::ANSI_U` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_ANSI_LeftBracket` | ✅ implemented | `KeyCode::ANSI_LEFT_BRACKET` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_ANSI_I` | ✅ implemented | `KeyCode::ANSI_I` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_ANSI_P` | ✅ implemented | `KeyCode::ANSI_P` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_ANSI_L` | ✅ implemented | `KeyCode::ANSI_L` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_ANSI_J` | ✅ implemented | `KeyCode::ANSI_J` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_ANSI_Quote` | ✅ implemented | `KeyCode::ANSI_QUOTE` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_ANSI_K` | ✅ implemented | `KeyCode::ANSI_K` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_ANSI_Semicolon` | ✅ implemented | `KeyCode::ANSI_SEMICOLON` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_ANSI_Backslash` | ✅ implemented | `KeyCode::ANSI_BACKSLASH` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_ANSI_Comma` | ✅ implemented | `KeyCode::ANSI_COMMA` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_ANSI_Slash` | ✅ implemented | `KeyCode::ANSI_SLASH` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_ANSI_N` | ✅ implemented | `KeyCode::ANSI_N` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_ANSI_M` | ✅ implemented | `KeyCode::ANSI_M` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_ANSI_Period` | ✅ implemented | `KeyCode::ANSI_PERIOD` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_ANSI_Grave` | ✅ implemented | `KeyCode::ANSI_GRAVE` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_ANSI_KeypadDecimal` | ✅ implemented | `KeyCode::ANSI_KEYPAD_DECIMAL` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_ANSI_KeypadMultiply` | ✅ implemented | `KeyCode::ANSI_KEYPAD_MULTIPLY` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_ANSI_KeypadPlus` | ✅ implemented | `KeyCode::ANSI_KEYPAD_PLUS` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_ANSI_KeypadClear` | ✅ implemented | `KeyCode::ANSI_KEYPAD_CLEAR` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_ANSI_KeypadDivide` | ✅ implemented | `KeyCode::ANSI_KEYPAD_DIVIDE` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_ANSI_KeypadEnter` | ✅ implemented | `KeyCode::ANSI_KEYPAD_ENTER` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_ANSI_KeypadMinus` | ✅ implemented | `KeyCode::ANSI_KEYPAD_MINUS` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_ANSI_KeypadEquals` | ✅ implemented | `KeyCode::ANSI_KEYPAD_EQUALS` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_ANSI_Keypad0` | ✅ implemented | `KeyCode::ANSI_KEYPAD0` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_ANSI_Keypad1` | ✅ implemented | `KeyCode::ANSI_KEYPAD1` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_ANSI_Keypad2` | ✅ implemented | `KeyCode::ANSI_KEYPAD2` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_ANSI_Keypad3` | ✅ implemented | `KeyCode::ANSI_KEYPAD3` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_ANSI_Keypad4` | ✅ implemented | `KeyCode::ANSI_KEYPAD4` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_ANSI_Keypad5` | ✅ implemented | `KeyCode::ANSI_KEYPAD5` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_ANSI_Keypad6` | ✅ implemented | `KeyCode::ANSI_KEYPAD6` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_ANSI_Keypad7` | ✅ implemented | `KeyCode::ANSI_KEYPAD7` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_ANSI_Keypad8` | ✅ implemented | `KeyCode::ANSI_KEYPAD8` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_ANSI_Keypad9` | ✅ implemented | `KeyCode::ANSI_KEYPAD9` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_Return` | ✅ implemented | `KeyCode::RETURN` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_Tab` | ✅ implemented | `KeyCode::TAB` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_Space` | ✅ implemented | `KeyCode::SPACE` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_Delete` | ✅ implemented | `KeyCode::DELETE` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_Escape` | ✅ implemented | `KeyCode::ESCAPE` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_Command` | ✅ implemented | `KeyCode::COMMAND` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_Shift` | ✅ implemented | `KeyCode::SHIFT` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_CapsLock` | ✅ implemented | `KeyCode::CAPS_LOCK` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_Option` | ✅ implemented | `KeyCode::OPTION` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_Control` | ✅ implemented | `KeyCode::CONTROL` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_RightCommand` | ✅ implemented | `KeyCode::RIGHT_COMMAND` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_RightShift` | ✅ implemented | `KeyCode::RIGHT_SHIFT` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_RightOption` | ✅ implemented | `KeyCode::RIGHT_OPTION` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_RightControl` | ✅ implemented | `KeyCode::RIGHT_CONTROL` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_Function` | ✅ implemented | `KeyCode::FUNCTION` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_F17` | ✅ implemented | `KeyCode::F17` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_VolumeUp` | ✅ implemented | `KeyCode::VOLUME_UP` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_VolumeDown` | ✅ implemented | `KeyCode::VOLUME_DOWN` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_Mute` | ✅ implemented | `KeyCode::MUTE` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_F18` | ✅ implemented | `KeyCode::F18` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_F19` | ✅ implemented | `KeyCode::F19` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_F20` | ✅ implemented | `KeyCode::F20` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_F5` | ✅ implemented | `KeyCode::F5` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_F6` | ✅ implemented | `KeyCode::F6` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_F7` | ✅ implemented | `KeyCode::F7` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_F3` | ✅ implemented | `KeyCode::F3` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_F8` | ✅ implemented | `KeyCode::F8` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_F9` | ✅ implemented | `KeyCode::F9` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_F11` | ✅ implemented | `KeyCode::F11` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_F13` | ✅ implemented | `KeyCode::F13` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_F16` | ✅ implemented | `KeyCode::F16` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_F14` | ✅ implemented | `KeyCode::F14` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_F10` | ✅ implemented | `KeyCode::F10` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_ContextualMenu` | ✅ implemented | `KeyCode::CONTEXTUAL_MENU` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_F12` | ✅ implemented | `KeyCode::F12` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_F15` | ✅ implemented | `KeyCode::F15` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_Help` | ✅ implemented | `KeyCode::HELP` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_Home` | ✅ implemented | `KeyCode::HOME` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_PageUp` | ✅ implemented | `KeyCode::PAGE_UP` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_ForwardDelete` | ✅ implemented | `KeyCode::FORWARD_DELETE` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_F4` | ✅ implemented | `KeyCode::F4` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_End` | ✅ implemented | `KeyCode::END` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_F2` | ✅ implemented | `KeyCode::F2` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_PageDown` | ✅ implemented | `KeyCode::PAGE_DOWN` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_F1` | ✅ implemented | `KeyCode::F1` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_LeftArrow` | ✅ implemented | `KeyCode::LEFT_ARROW` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_RightArrow` | ✅ implemented | `KeyCode::RIGHT_ARROW` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_DownArrow` | ✅ implemented | `KeyCode::DOWN_ARROW` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_UpArrow` | ✅ implemented | `KeyCode::UP_ARROW` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_ISO_Section` | ✅ implemented | `KeyCode::ISO_SECTION` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_JIS_Yen` | ✅ implemented | `KeyCode::JIS_YEN` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_JIS_Underscore` | ✅ implemented | `KeyCode::JIS_UNDERSCORE` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_JIS_KeypadComma` | ✅ implemented | `KeyCode::JIS_KEYPAD_COMMA` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_JIS_Eisu` | ✅ implemented | `KeyCode::JIS_EISU` | mirrored in Rust and validated against the Swift bridge |
| Events.h | `kVK_JIS_Kana` | ✅ implemented | `KeyCode::JIS_KANA` | mirrored in Rust and validated against the Swift bridge |
