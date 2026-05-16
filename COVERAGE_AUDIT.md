# carbonhotkey-rs coverage audit (vs MacOSX26.2.sdk)

Focused scope only: every `kVK_*` constant in `Events.h`, plus the `RegisterEventHotKey` / `EventTypeSpec` hotkey slice from `CarbonEvents.h` (including `CarbonEventsCore.h` entries that `CarbonEvents.h` re-exports). This is **not** a full HIToolbox audit.

SDK_PUBLIC_SYMBOLS: 139
VERIFIED: 139
GAPS: 0
EXEMPT: 0
COVERAGE_PCT: 100.00%

## 🟢 VERIFIED
| Symbol | Kind | Header | Wrapped by |
| --- | --- | --- | --- |
| `EventTypeSpec` | struct | `CarbonEventsCore.h` | `ffi::EventTypeSpec`; safe handler install path uses it internally |
| `EventHotKeyID` | struct | `CarbonEvents.h` | `ffi::EventHotKeyID`; safe `HotKeyEvent::{signature,id}` mirrors the payload |
| `EventHotKeyRef` | opaque type | `CarbonEvents.h` | `ffi::EventHotKeyRef`; safe `Hotkey` wraps the handle |
| `eventHotKeyExistsErr` | enum constant | `CarbonEventsCore.h` | `ffi::eventHotKeyExistsErr`; safe `HotkeyError::AlreadyRegistered` |
| `kEventClassKeyboard` | enum constant | `CarbonEvents.h` | `ffi::kEventClassKeyboard`; safe `HotKeyEvent::event_class` |
| `kEventHotKeyPressed` | enum constant | `CarbonEvents.h` | `ffi::kEventHotKeyPressed`; safe `HotKeyEventKind::Pressed` |
| `kEventHotKeyReleased` | enum constant | `CarbonEvents.h` | `ffi::kEventHotKeyReleased`; safe `HotKeyEventKind::Released` |
| `kEventHotKeyExclusive` | enum constant | `CarbonEvents.h` | `ffi::kEventHotKeyExclusive`; safe `HotKeyOptions::EXCLUSIVE` |
| `kEventHotKeyNoOptions` | enum constant | `CarbonEvents.h` | `ffi::kEventHotKeyNoOptions`; safe `HotKeyOptions::NO_OPTIONS` |
| `kEventParamDirectObject` | enum constant | `CarbonEventsCore.h` | `ffi::kEventParamDirectObject` |
| `typeEventHotKeyID` | enum constant | `CarbonEvents.h` | `ffi::typeEventHotKeyID` |
| `GetApplicationEventTarget` | function | `CarbonEvents.h` | `ffi::GetApplicationEventTarget`; safe registration/handler helpers always target it |
| `InstallEventHandler` | function | `CarbonEventsCore.h` | `ffi::InstallEventHandler`; safe `EventHandler::install` / `install_keyboard_handler` |
| `RemoveEventHandler` | function | `CarbonEventsCore.h` | `ffi::RemoveEventHandler`; safe `EventHandler::remove` |
| `RegisterEventHotKey` | function | `CarbonEvents.h` | `ffi::RegisterEventHotKey`; safe `register*` helpers |
| `UnregisterEventHotKey` | function | `CarbonEvents.h` | `ffi::UnregisterEventHotKey`; safe `Hotkey::unregister` |
| `GetEventClass` | function | `CarbonEventsCore.h` | `ffi::GetEventClass`; safe `HotKeyEvent::event_class` |
| `GetEventKind` | function | `CarbonEventsCore.h` | `ffi::GetEventKind`; safe `HotKeyEvent::event_kind` |
| `GetEventParameter` | function | `CarbonEventsCore.h` | `ffi::GetEventParameter`; safe `HotKeyEvent::{signature,id}` |
| `kVK_ANSI_A` | enum constant | `Events.h` | `KeyCode::ANSI_A` |
| `kVK_ANSI_S` | enum constant | `Events.h` | `KeyCode::ANSI_S` |
| `kVK_ANSI_D` | enum constant | `Events.h` | `KeyCode::ANSI_D` |
| `kVK_ANSI_F` | enum constant | `Events.h` | `KeyCode::ANSI_F` |
| `kVK_ANSI_H` | enum constant | `Events.h` | `KeyCode::ANSI_H` |
| `kVK_ANSI_G` | enum constant | `Events.h` | `KeyCode::ANSI_G` |
| `kVK_ANSI_Z` | enum constant | `Events.h` | `KeyCode::ANSI_Z` |
| `kVK_ANSI_X` | enum constant | `Events.h` | `KeyCode::ANSI_X` |
| `kVK_ANSI_C` | enum constant | `Events.h` | `KeyCode::ANSI_C` |
| `kVK_ANSI_V` | enum constant | `Events.h` | `KeyCode::ANSI_V` |
| `kVK_ANSI_B` | enum constant | `Events.h` | `KeyCode::ANSI_B` |
| `kVK_ANSI_Q` | enum constant | `Events.h` | `KeyCode::ANSI_Q` |
| `kVK_ANSI_W` | enum constant | `Events.h` | `KeyCode::ANSI_W` |
| `kVK_ANSI_E` | enum constant | `Events.h` | `KeyCode::ANSI_E` |
| `kVK_ANSI_R` | enum constant | `Events.h` | `KeyCode::ANSI_R` |
| `kVK_ANSI_Y` | enum constant | `Events.h` | `KeyCode::ANSI_Y` |
| `kVK_ANSI_T` | enum constant | `Events.h` | `KeyCode::ANSI_T` |
| `kVK_ANSI_1` | enum constant | `Events.h` | `KeyCode::ANSI_1` |
| `kVK_ANSI_2` | enum constant | `Events.h` | `KeyCode::ANSI_2` |
| `kVK_ANSI_3` | enum constant | `Events.h` | `KeyCode::ANSI_3` |
| `kVK_ANSI_4` | enum constant | `Events.h` | `KeyCode::ANSI_4` |
| `kVK_ANSI_6` | enum constant | `Events.h` | `KeyCode::ANSI_6` |
| `kVK_ANSI_5` | enum constant | `Events.h` | `KeyCode::ANSI_5` |
| `kVK_ANSI_Equal` | enum constant | `Events.h` | `KeyCode::ANSI_EQUAL` |
| `kVK_ANSI_9` | enum constant | `Events.h` | `KeyCode::ANSI_9` |
| `kVK_ANSI_7` | enum constant | `Events.h` | `KeyCode::ANSI_7` |
| `kVK_ANSI_Minus` | enum constant | `Events.h` | `KeyCode::ANSI_MINUS` |
| `kVK_ANSI_8` | enum constant | `Events.h` | `KeyCode::ANSI_8` |
| `kVK_ANSI_0` | enum constant | `Events.h` | `KeyCode::ANSI_0` |
| `kVK_ANSI_RightBracket` | enum constant | `Events.h` | `KeyCode::ANSI_RIGHT_BRACKET` |
| `kVK_ANSI_O` | enum constant | `Events.h` | `KeyCode::ANSI_O` |
| `kVK_ANSI_U` | enum constant | `Events.h` | `KeyCode::ANSI_U` |
| `kVK_ANSI_LeftBracket` | enum constant | `Events.h` | `KeyCode::ANSI_LEFT_BRACKET` |
| `kVK_ANSI_I` | enum constant | `Events.h` | `KeyCode::ANSI_I` |
| `kVK_ANSI_P` | enum constant | `Events.h` | `KeyCode::ANSI_P` |
| `kVK_ANSI_L` | enum constant | `Events.h` | `KeyCode::ANSI_L` |
| `kVK_ANSI_J` | enum constant | `Events.h` | `KeyCode::ANSI_J` |
| `kVK_ANSI_Quote` | enum constant | `Events.h` | `KeyCode::ANSI_QUOTE` |
| `kVK_ANSI_K` | enum constant | `Events.h` | `KeyCode::ANSI_K` |
| `kVK_ANSI_Semicolon` | enum constant | `Events.h` | `KeyCode::ANSI_SEMICOLON` |
| `kVK_ANSI_Backslash` | enum constant | `Events.h` | `KeyCode::ANSI_BACKSLASH` |
| `kVK_ANSI_Comma` | enum constant | `Events.h` | `KeyCode::ANSI_COMMA` |
| `kVK_ANSI_Slash` | enum constant | `Events.h` | `KeyCode::ANSI_SLASH` |
| `kVK_ANSI_N` | enum constant | `Events.h` | `KeyCode::ANSI_N` |
| `kVK_ANSI_M` | enum constant | `Events.h` | `KeyCode::ANSI_M` |
| `kVK_ANSI_Period` | enum constant | `Events.h` | `KeyCode::ANSI_PERIOD` |
| `kVK_ANSI_Grave` | enum constant | `Events.h` | `KeyCode::ANSI_GRAVE` |
| `kVK_ANSI_KeypadDecimal` | enum constant | `Events.h` | `KeyCode::ANSI_KEYPAD_DECIMAL` |
| `kVK_ANSI_KeypadMultiply` | enum constant | `Events.h` | `KeyCode::ANSI_KEYPAD_MULTIPLY` |
| `kVK_ANSI_KeypadPlus` | enum constant | `Events.h` | `KeyCode::ANSI_KEYPAD_PLUS` |
| `kVK_ANSI_KeypadClear` | enum constant | `Events.h` | `KeyCode::ANSI_KEYPAD_CLEAR` |
| `kVK_ANSI_KeypadDivide` | enum constant | `Events.h` | `KeyCode::ANSI_KEYPAD_DIVIDE` |
| `kVK_ANSI_KeypadEnter` | enum constant | `Events.h` | `KeyCode::ANSI_KEYPAD_ENTER` |
| `kVK_ANSI_KeypadMinus` | enum constant | `Events.h` | `KeyCode::ANSI_KEYPAD_MINUS` |
| `kVK_ANSI_KeypadEquals` | enum constant | `Events.h` | `KeyCode::ANSI_KEYPAD_EQUALS` |
| `kVK_ANSI_Keypad0` | enum constant | `Events.h` | `KeyCode::ANSI_KEYPAD0` |
| `kVK_ANSI_Keypad1` | enum constant | `Events.h` | `KeyCode::ANSI_KEYPAD1` |
| `kVK_ANSI_Keypad2` | enum constant | `Events.h` | `KeyCode::ANSI_KEYPAD2` |
| `kVK_ANSI_Keypad3` | enum constant | `Events.h` | `KeyCode::ANSI_KEYPAD3` |
| `kVK_ANSI_Keypad4` | enum constant | `Events.h` | `KeyCode::ANSI_KEYPAD4` |
| `kVK_ANSI_Keypad5` | enum constant | `Events.h` | `KeyCode::ANSI_KEYPAD5` |
| `kVK_ANSI_Keypad6` | enum constant | `Events.h` | `KeyCode::ANSI_KEYPAD6` |
| `kVK_ANSI_Keypad7` | enum constant | `Events.h` | `KeyCode::ANSI_KEYPAD7` |
| `kVK_ANSI_Keypad8` | enum constant | `Events.h` | `KeyCode::ANSI_KEYPAD8` |
| `kVK_ANSI_Keypad9` | enum constant | `Events.h` | `KeyCode::ANSI_KEYPAD9` |
| `kVK_Return` | enum constant | `Events.h` | `KeyCode::RETURN` |
| `kVK_Tab` | enum constant | `Events.h` | `KeyCode::TAB` |
| `kVK_Space` | enum constant | `Events.h` | `KeyCode::SPACE` |
| `kVK_Delete` | enum constant | `Events.h` | `KeyCode::DELETE` |
| `kVK_Escape` | enum constant | `Events.h` | `KeyCode::ESCAPE` |
| `kVK_Command` | enum constant | `Events.h` | `KeyCode::COMMAND` |
| `kVK_Shift` | enum constant | `Events.h` | `KeyCode::SHIFT` |
| `kVK_CapsLock` | enum constant | `Events.h` | `KeyCode::CAPS_LOCK` |
| `kVK_Option` | enum constant | `Events.h` | `KeyCode::OPTION` |
| `kVK_Control` | enum constant | `Events.h` | `KeyCode::CONTROL` |
| `kVK_RightCommand` | enum constant | `Events.h` | `KeyCode::RIGHT_COMMAND` |
| `kVK_RightShift` | enum constant | `Events.h` | `KeyCode::RIGHT_SHIFT` |
| `kVK_RightOption` | enum constant | `Events.h` | `KeyCode::RIGHT_OPTION` |
| `kVK_RightControl` | enum constant | `Events.h` | `KeyCode::RIGHT_CONTROL` |
| `kVK_Function` | enum constant | `Events.h` | `KeyCode::FUNCTION` |
| `kVK_F17` | enum constant | `Events.h` | `KeyCode::F17` |
| `kVK_VolumeUp` | enum constant | `Events.h` | `KeyCode::VOLUME_UP` |
| `kVK_VolumeDown` | enum constant | `Events.h` | `KeyCode::VOLUME_DOWN` |
| `kVK_Mute` | enum constant | `Events.h` | `KeyCode::MUTE` |
| `kVK_F18` | enum constant | `Events.h` | `KeyCode::F18` |
| `kVK_F19` | enum constant | `Events.h` | `KeyCode::F19` |
| `kVK_F20` | enum constant | `Events.h` | `KeyCode::F20` |
| `kVK_F5` | enum constant | `Events.h` | `KeyCode::F5` |
| `kVK_F6` | enum constant | `Events.h` | `KeyCode::F6` |
| `kVK_F7` | enum constant | `Events.h` | `KeyCode::F7` |
| `kVK_F3` | enum constant | `Events.h` | `KeyCode::F3` |
| `kVK_F8` | enum constant | `Events.h` | `KeyCode::F8` |
| `kVK_F9` | enum constant | `Events.h` | `KeyCode::F9` |
| `kVK_F11` | enum constant | `Events.h` | `KeyCode::F11` |
| `kVK_F13` | enum constant | `Events.h` | `KeyCode::F13` |
| `kVK_F16` | enum constant | `Events.h` | `KeyCode::F16` |
| `kVK_F14` | enum constant | `Events.h` | `KeyCode::F14` |
| `kVK_F10` | enum constant | `Events.h` | `KeyCode::F10` |
| `kVK_ContextualMenu` | enum constant | `Events.h` | `KeyCode::CONTEXTUAL_MENU` |
| `kVK_F12` | enum constant | `Events.h` | `KeyCode::F12` |
| `kVK_F15` | enum constant | `Events.h` | `KeyCode::F15` |
| `kVK_Help` | enum constant | `Events.h` | `KeyCode::HELP` |
| `kVK_Home` | enum constant | `Events.h` | `KeyCode::HOME` |
| `kVK_PageUp` | enum constant | `Events.h` | `KeyCode::PAGE_UP` |
| `kVK_ForwardDelete` | enum constant | `Events.h` | `KeyCode::FORWARD_DELETE` |
| `kVK_F4` | enum constant | `Events.h` | `KeyCode::F4` |
| `kVK_End` | enum constant | `Events.h` | `KeyCode::END` |
| `kVK_F2` | enum constant | `Events.h` | `KeyCode::F2` |
| `kVK_PageDown` | enum constant | `Events.h` | `KeyCode::PAGE_DOWN` |
| `kVK_F1` | enum constant | `Events.h` | `KeyCode::F1` |
| `kVK_LeftArrow` | enum constant | `Events.h` | `KeyCode::LEFT_ARROW` |
| `kVK_RightArrow` | enum constant | `Events.h` | `KeyCode::RIGHT_ARROW` |
| `kVK_DownArrow` | enum constant | `Events.h` | `KeyCode::DOWN_ARROW` |
| `kVK_UpArrow` | enum constant | `Events.h` | `KeyCode::UP_ARROW` |
| `kVK_ISO_Section` | enum constant | `Events.h` | `KeyCode::ISO_SECTION` |
| `kVK_JIS_Yen` | enum constant | `Events.h` | `KeyCode::JIS_YEN` |
| `kVK_JIS_Underscore` | enum constant | `Events.h` | `KeyCode::JIS_UNDERSCORE` |
| `kVK_JIS_KeypadComma` | enum constant | `Events.h` | `KeyCode::JIS_KEYPAD_COMMA` |
| `kVK_JIS_Eisu` | enum constant | `Events.h` | `KeyCode::JIS_EISU` |
| `kVK_JIS_Kana` | enum constant | `Events.h` | `KeyCode::JIS_KANA` |

## 🔴 GAPS
| Symbol | Kind | Header | Notes |
| --- | --- | --- | --- |
| — | — | — | None. |

## ⏭️ EXEMPT
| Symbol | Kind | Header | Reason | SDK attribute |
| --- | --- | --- | --- | --- |
| — | — | — | None in the focused slice. | — |

