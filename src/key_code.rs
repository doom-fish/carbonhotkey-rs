//! Carbon virtual key-code mapping (`kVK_*`) from `Events.h`.

use core::fmt;

/// Carbon virtual key code from `Events.h`.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KeyCode(u16);

impl KeyCode {
    /// Create a key code from the raw Carbon value.
    #[must_use]
    pub const fn from_raw(raw: u16) -> Self {
        Self(raw)
    }

    /// Return the raw Carbon value.
    #[must_use]
    pub const fn raw(self) -> u16 {
        self.0
    }

    /// `kVK_ANSI_A` from `Events.h`.
    pub const ANSI_A: Self = Self(0x00);

    /// `kVK_ANSI_S` from `Events.h`.
    pub const ANSI_S: Self = Self(0x01);

    /// `kVK_ANSI_D` from `Events.h`.
    pub const ANSI_D: Self = Self(0x02);

    /// `kVK_ANSI_F` from `Events.h`.
    pub const ANSI_F: Self = Self(0x03);

    /// `kVK_ANSI_H` from `Events.h`.
    pub const ANSI_H: Self = Self(0x04);

    /// `kVK_ANSI_G` from `Events.h`.
    pub const ANSI_G: Self = Self(0x05);

    /// `kVK_ANSI_Z` from `Events.h`.
    pub const ANSI_Z: Self = Self(0x06);

    /// `kVK_ANSI_X` from `Events.h`.
    pub const ANSI_X: Self = Self(0x07);

    /// `kVK_ANSI_C` from `Events.h`.
    pub const ANSI_C: Self = Self(0x08);

    /// `kVK_ANSI_V` from `Events.h`.
    pub const ANSI_V: Self = Self(0x09);

    /// `kVK_ANSI_B` from `Events.h`.
    pub const ANSI_B: Self = Self(0x0B);

    /// `kVK_ANSI_Q` from `Events.h`.
    pub const ANSI_Q: Self = Self(0x0C);

    /// `kVK_ANSI_W` from `Events.h`.
    pub const ANSI_W: Self = Self(0x0D);

    /// `kVK_ANSI_E` from `Events.h`.
    pub const ANSI_E: Self = Self(0x0E);

    /// `kVK_ANSI_R` from `Events.h`.
    pub const ANSI_R: Self = Self(0x0F);

    /// `kVK_ANSI_Y` from `Events.h`.
    pub const ANSI_Y: Self = Self(0x10);

    /// `kVK_ANSI_T` from `Events.h`.
    pub const ANSI_T: Self = Self(0x11);

    /// `kVK_ANSI_1` from `Events.h`.
    pub const ANSI_1: Self = Self(0x12);

    /// `kVK_ANSI_2` from `Events.h`.
    pub const ANSI_2: Self = Self(0x13);

    /// `kVK_ANSI_3` from `Events.h`.
    pub const ANSI_3: Self = Self(0x14);

    /// `kVK_ANSI_4` from `Events.h`.
    pub const ANSI_4: Self = Self(0x15);

    /// `kVK_ANSI_6` from `Events.h`.
    pub const ANSI_6: Self = Self(0x16);

    /// `kVK_ANSI_5` from `Events.h`.
    pub const ANSI_5: Self = Self(0x17);

    /// `kVK_ANSI_Equal` from `Events.h`.
    pub const ANSI_EQUAL: Self = Self(0x18);

    /// `kVK_ANSI_9` from `Events.h`.
    pub const ANSI_9: Self = Self(0x19);

    /// `kVK_ANSI_7` from `Events.h`.
    pub const ANSI_7: Self = Self(0x1A);

    /// `kVK_ANSI_Minus` from `Events.h`.
    pub const ANSI_MINUS: Self = Self(0x1B);

    /// `kVK_ANSI_8` from `Events.h`.
    pub const ANSI_8: Self = Self(0x1C);

    /// `kVK_ANSI_0` from `Events.h`.
    pub const ANSI_0: Self = Self(0x1D);

    /// `kVK_ANSI_RightBracket` from `Events.h`.
    pub const ANSI_RIGHT_BRACKET: Self = Self(0x1E);

    /// `kVK_ANSI_O` from `Events.h`.
    pub const ANSI_O: Self = Self(0x1F);

    /// `kVK_ANSI_U` from `Events.h`.
    pub const ANSI_U: Self = Self(0x20);

    /// `kVK_ANSI_LeftBracket` from `Events.h`.
    pub const ANSI_LEFT_BRACKET: Self = Self(0x21);

    /// `kVK_ANSI_I` from `Events.h`.
    pub const ANSI_I: Self = Self(0x22);

    /// `kVK_ANSI_P` from `Events.h`.
    pub const ANSI_P: Self = Self(0x23);

    /// `kVK_ANSI_L` from `Events.h`.
    pub const ANSI_L: Self = Self(0x25);

    /// `kVK_ANSI_J` from `Events.h`.
    pub const ANSI_J: Self = Self(0x26);

    /// `kVK_ANSI_Quote` from `Events.h`.
    pub const ANSI_QUOTE: Self = Self(0x27);

    /// `kVK_ANSI_K` from `Events.h`.
    pub const ANSI_K: Self = Self(0x28);

    /// `kVK_ANSI_Semicolon` from `Events.h`.
    pub const ANSI_SEMICOLON: Self = Self(0x29);

    /// `kVK_ANSI_Backslash` from `Events.h`.
    pub const ANSI_BACKSLASH: Self = Self(0x2A);

    /// `kVK_ANSI_Comma` from `Events.h`.
    pub const ANSI_COMMA: Self = Self(0x2B);

    /// `kVK_ANSI_Slash` from `Events.h`.
    pub const ANSI_SLASH: Self = Self(0x2C);

    /// `kVK_ANSI_N` from `Events.h`.
    pub const ANSI_N: Self = Self(0x2D);

    /// `kVK_ANSI_M` from `Events.h`.
    pub const ANSI_M: Self = Self(0x2E);

    /// `kVK_ANSI_Period` from `Events.h`.
    pub const ANSI_PERIOD: Self = Self(0x2F);

    /// `kVK_ANSI_Grave` from `Events.h`.
    pub const ANSI_GRAVE: Self = Self(0x32);

    /// `kVK_ANSI_KeypadDecimal` from `Events.h`.
    pub const ANSI_KEYPAD_DECIMAL: Self = Self(0x41);

    /// `kVK_ANSI_KeypadMultiply` from `Events.h`.
    pub const ANSI_KEYPAD_MULTIPLY: Self = Self(0x43);

    /// `kVK_ANSI_KeypadPlus` from `Events.h`.
    pub const ANSI_KEYPAD_PLUS: Self = Self(0x45);

    /// `kVK_ANSI_KeypadClear` from `Events.h`.
    pub const ANSI_KEYPAD_CLEAR: Self = Self(0x47);

    /// `kVK_ANSI_KeypadDivide` from `Events.h`.
    pub const ANSI_KEYPAD_DIVIDE: Self = Self(0x4B);

    /// `kVK_ANSI_KeypadEnter` from `Events.h`.
    pub const ANSI_KEYPAD_ENTER: Self = Self(0x4C);

    /// `kVK_ANSI_KeypadMinus` from `Events.h`.
    pub const ANSI_KEYPAD_MINUS: Self = Self(0x4E);

    /// `kVK_ANSI_KeypadEquals` from `Events.h`.
    pub const ANSI_KEYPAD_EQUALS: Self = Self(0x51);

    /// `kVK_ANSI_Keypad0` from `Events.h`.
    pub const ANSI_KEYPAD0: Self = Self(0x52);

    /// `kVK_ANSI_Keypad1` from `Events.h`.
    pub const ANSI_KEYPAD1: Self = Self(0x53);

    /// `kVK_ANSI_Keypad2` from `Events.h`.
    pub const ANSI_KEYPAD2: Self = Self(0x54);

    /// `kVK_ANSI_Keypad3` from `Events.h`.
    pub const ANSI_KEYPAD3: Self = Self(0x55);

    /// `kVK_ANSI_Keypad4` from `Events.h`.
    pub const ANSI_KEYPAD4: Self = Self(0x56);

    /// `kVK_ANSI_Keypad5` from `Events.h`.
    pub const ANSI_KEYPAD5: Self = Self(0x57);

    /// `kVK_ANSI_Keypad6` from `Events.h`.
    pub const ANSI_KEYPAD6: Self = Self(0x58);

    /// `kVK_ANSI_Keypad7` from `Events.h`.
    pub const ANSI_KEYPAD7: Self = Self(0x59);

    /// `kVK_ANSI_Keypad8` from `Events.h`.
    pub const ANSI_KEYPAD8: Self = Self(0x5B);

    /// `kVK_ANSI_Keypad9` from `Events.h`.
    pub const ANSI_KEYPAD9: Self = Self(0x5C);

    /// `kVK_Return` from `Events.h`.
    pub const RETURN: Self = Self(0x24);

    /// `kVK_Tab` from `Events.h`.
    pub const TAB: Self = Self(0x30);

    /// `kVK_Space` from `Events.h`.
    pub const SPACE: Self = Self(0x31);

    /// `kVK_Delete` from `Events.h`.
    pub const DELETE: Self = Self(0x33);

    /// `kVK_Escape` from `Events.h`.
    pub const ESCAPE: Self = Self(0x35);

    /// `kVK_Command` from `Events.h`.
    pub const COMMAND: Self = Self(0x37);

    /// `kVK_Shift` from `Events.h`.
    pub const SHIFT: Self = Self(0x38);

    /// `kVK_CapsLock` from `Events.h`.
    pub const CAPS_LOCK: Self = Self(0x39);

    /// `kVK_Option` from `Events.h`.
    pub const OPTION: Self = Self(0x3A);

    /// `kVK_Control` from `Events.h`.
    pub const CONTROL: Self = Self(0x3B);

    /// `kVK_RightCommand` from `Events.h`.
    pub const RIGHT_COMMAND: Self = Self(0x36);

    /// `kVK_RightShift` from `Events.h`.
    pub const RIGHT_SHIFT: Self = Self(0x3C);

    /// `kVK_RightOption` from `Events.h`.
    pub const RIGHT_OPTION: Self = Self(0x3D);

    /// `kVK_RightControl` from `Events.h`.
    pub const RIGHT_CONTROL: Self = Self(0x3E);

    /// `kVK_Function` from `Events.h`.
    pub const FUNCTION: Self = Self(0x3F);

    /// `kVK_F17` from `Events.h`.
    pub const F17: Self = Self(0x40);

    /// `kVK_VolumeUp` from `Events.h`.
    pub const VOLUME_UP: Self = Self(0x48);

    /// `kVK_VolumeDown` from `Events.h`.
    pub const VOLUME_DOWN: Self = Self(0x49);

    /// `kVK_Mute` from `Events.h`.
    pub const MUTE: Self = Self(0x4A);

    /// `kVK_F18` from `Events.h`.
    pub const F18: Self = Self(0x4F);

    /// `kVK_F19` from `Events.h`.
    pub const F19: Self = Self(0x50);

    /// `kVK_F20` from `Events.h`.
    pub const F20: Self = Self(0x5A);

    /// `kVK_F5` from `Events.h`.
    pub const F5: Self = Self(0x60);

    /// `kVK_F6` from `Events.h`.
    pub const F6: Self = Self(0x61);

    /// `kVK_F7` from `Events.h`.
    pub const F7: Self = Self(0x62);

    /// `kVK_F3` from `Events.h`.
    pub const F3: Self = Self(0x63);

    /// `kVK_F8` from `Events.h`.
    pub const F8: Self = Self(0x64);

    /// `kVK_F9` from `Events.h`.
    pub const F9: Self = Self(0x65);

    /// `kVK_F11` from `Events.h`.
    pub const F11: Self = Self(0x67);

    /// `kVK_F13` from `Events.h`.
    pub const F13: Self = Self(0x69);

    /// `kVK_F16` from `Events.h`.
    pub const F16: Self = Self(0x6A);

    /// `kVK_F14` from `Events.h`.
    pub const F14: Self = Self(0x6B);

    /// `kVK_F10` from `Events.h`.
    pub const F10: Self = Self(0x6D);

    /// `kVK_ContextualMenu` from `Events.h`.
    pub const CONTEXTUAL_MENU: Self = Self(0x6E);

    /// `kVK_F12` from `Events.h`.
    pub const F12: Self = Self(0x6F);

    /// `kVK_F15` from `Events.h`.
    pub const F15: Self = Self(0x71);

    /// `kVK_Help` from `Events.h`.
    pub const HELP: Self = Self(0x72);

    /// `kVK_Home` from `Events.h`.
    pub const HOME: Self = Self(0x73);

    /// `kVK_PageUp` from `Events.h`.
    pub const PAGE_UP: Self = Self(0x74);

    /// `kVK_ForwardDelete` from `Events.h`.
    pub const FORWARD_DELETE: Self = Self(0x75);

    /// `kVK_F4` from `Events.h`.
    pub const F4: Self = Self(0x76);

    /// `kVK_End` from `Events.h`.
    pub const END: Self = Self(0x77);

    /// `kVK_F2` from `Events.h`.
    pub const F2: Self = Self(0x78);

    /// `kVK_PageDown` from `Events.h`.
    pub const PAGE_DOWN: Self = Self(0x79);

    /// `kVK_F1` from `Events.h`.
    pub const F1: Self = Self(0x7A);

    /// `kVK_LeftArrow` from `Events.h`.
    pub const LEFT_ARROW: Self = Self(0x7B);

    /// `kVK_RightArrow` from `Events.h`.
    pub const RIGHT_ARROW: Self = Self(0x7C);

    /// `kVK_DownArrow` from `Events.h`.
    pub const DOWN_ARROW: Self = Self(0x7D);

    /// `kVK_UpArrow` from `Events.h`.
    pub const UP_ARROW: Self = Self(0x7E);

    /// `kVK_ISO_Section` from `Events.h`.
    pub const ISO_SECTION: Self = Self(0x0A);

    /// `kVK_JIS_Yen` from `Events.h`.
    pub const JIS_YEN: Self = Self(0x5D);

    /// `kVK_JIS_Underscore` from `Events.h`.
    pub const JIS_UNDERSCORE: Self = Self(0x5E);

    /// `kVK_JIS_KeypadComma` from `Events.h`.
    pub const JIS_KEYPAD_COMMA: Self = Self(0x5F);

    /// `kVK_JIS_Eisu` from `Events.h`.
    pub const JIS_EISU: Self = Self(0x66);

    /// `kVK_JIS_Kana` from `Events.h`.
    pub const JIS_KANA: Self = Self(0x68);

    /// Every `kVK_*` constant exposed by Apple in `Events.h`.
    pub const ALL: &'static [Self] = &[
        Self::ANSI_A,
        Self::ANSI_S,
        Self::ANSI_D,
        Self::ANSI_F,
        Self::ANSI_H,
        Self::ANSI_G,
        Self::ANSI_Z,
        Self::ANSI_X,
        Self::ANSI_C,
        Self::ANSI_V,
        Self::ANSI_B,
        Self::ANSI_Q,
        Self::ANSI_W,
        Self::ANSI_E,
        Self::ANSI_R,
        Self::ANSI_Y,
        Self::ANSI_T,
        Self::ANSI_1,
        Self::ANSI_2,
        Self::ANSI_3,
        Self::ANSI_4,
        Self::ANSI_6,
        Self::ANSI_5,
        Self::ANSI_EQUAL,
        Self::ANSI_9,
        Self::ANSI_7,
        Self::ANSI_MINUS,
        Self::ANSI_8,
        Self::ANSI_0,
        Self::ANSI_RIGHT_BRACKET,
        Self::ANSI_O,
        Self::ANSI_U,
        Self::ANSI_LEFT_BRACKET,
        Self::ANSI_I,
        Self::ANSI_P,
        Self::ANSI_L,
        Self::ANSI_J,
        Self::ANSI_QUOTE,
        Self::ANSI_K,
        Self::ANSI_SEMICOLON,
        Self::ANSI_BACKSLASH,
        Self::ANSI_COMMA,
        Self::ANSI_SLASH,
        Self::ANSI_N,
        Self::ANSI_M,
        Self::ANSI_PERIOD,
        Self::ANSI_GRAVE,
        Self::ANSI_KEYPAD_DECIMAL,
        Self::ANSI_KEYPAD_MULTIPLY,
        Self::ANSI_KEYPAD_PLUS,
        Self::ANSI_KEYPAD_CLEAR,
        Self::ANSI_KEYPAD_DIVIDE,
        Self::ANSI_KEYPAD_ENTER,
        Self::ANSI_KEYPAD_MINUS,
        Self::ANSI_KEYPAD_EQUALS,
        Self::ANSI_KEYPAD0,
        Self::ANSI_KEYPAD1,
        Self::ANSI_KEYPAD2,
        Self::ANSI_KEYPAD3,
        Self::ANSI_KEYPAD4,
        Self::ANSI_KEYPAD5,
        Self::ANSI_KEYPAD6,
        Self::ANSI_KEYPAD7,
        Self::ANSI_KEYPAD8,
        Self::ANSI_KEYPAD9,
        Self::RETURN,
        Self::TAB,
        Self::SPACE,
        Self::DELETE,
        Self::ESCAPE,
        Self::COMMAND,
        Self::SHIFT,
        Self::CAPS_LOCK,
        Self::OPTION,
        Self::CONTROL,
        Self::RIGHT_COMMAND,
        Self::RIGHT_SHIFT,
        Self::RIGHT_OPTION,
        Self::RIGHT_CONTROL,
        Self::FUNCTION,
        Self::F17,
        Self::VOLUME_UP,
        Self::VOLUME_DOWN,
        Self::MUTE,
        Self::F18,
        Self::F19,
        Self::F20,
        Self::F5,
        Self::F6,
        Self::F7,
        Self::F3,
        Self::F8,
        Self::F9,
        Self::F11,
        Self::F13,
        Self::F16,
        Self::F14,
        Self::F10,
        Self::CONTEXTUAL_MENU,
        Self::F12,
        Self::F15,
        Self::HELP,
        Self::HOME,
        Self::PAGE_UP,
        Self::FORWARD_DELETE,
        Self::F4,
        Self::END,
        Self::F2,
        Self::PAGE_DOWN,
        Self::F1,
        Self::LEFT_ARROW,
        Self::RIGHT_ARROW,
        Self::DOWN_ARROW,
        Self::UP_ARROW,
        Self::ISO_SECTION,
        Self::JIS_YEN,
        Self::JIS_UNDERSCORE,
        Self::JIS_KEYPAD_COMMA,
        Self::JIS_EISU,
        Self::JIS_KANA,
    ];

    /// Return Apple's canonical `kVK_*` suffix for this key code.
    #[allow(clippy::too_many_lines)]
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self.0 {
            0x00 => "ANSI_A",
            0x01 => "ANSI_S",
            0x02 => "ANSI_D",
            0x03 => "ANSI_F",
            0x04 => "ANSI_H",
            0x05 => "ANSI_G",
            0x06 => "ANSI_Z",
            0x07 => "ANSI_X",
            0x08 => "ANSI_C",
            0x09 => "ANSI_V",
            0x0B => "ANSI_B",
            0x0C => "ANSI_Q",
            0x0D => "ANSI_W",
            0x0E => "ANSI_E",
            0x0F => "ANSI_R",
            0x10 => "ANSI_Y",
            0x11 => "ANSI_T",
            0x12 => "ANSI_1",
            0x13 => "ANSI_2",
            0x14 => "ANSI_3",
            0x15 => "ANSI_4",
            0x16 => "ANSI_6",
            0x17 => "ANSI_5",
            0x18 => "ANSI_Equal",
            0x19 => "ANSI_9",
            0x1A => "ANSI_7",
            0x1B => "ANSI_Minus",
            0x1C => "ANSI_8",
            0x1D => "ANSI_0",
            0x1E => "ANSI_RightBracket",
            0x1F => "ANSI_O",
            0x20 => "ANSI_U",
            0x21 => "ANSI_LeftBracket",
            0x22 => "ANSI_I",
            0x23 => "ANSI_P",
            0x25 => "ANSI_L",
            0x26 => "ANSI_J",
            0x27 => "ANSI_Quote",
            0x28 => "ANSI_K",
            0x29 => "ANSI_Semicolon",
            0x2A => "ANSI_Backslash",
            0x2B => "ANSI_Comma",
            0x2C => "ANSI_Slash",
            0x2D => "ANSI_N",
            0x2E => "ANSI_M",
            0x2F => "ANSI_Period",
            0x32 => "ANSI_Grave",
            0x41 => "ANSI_KeypadDecimal",
            0x43 => "ANSI_KeypadMultiply",
            0x45 => "ANSI_KeypadPlus",
            0x47 => "ANSI_KeypadClear",
            0x4B => "ANSI_KeypadDivide",
            0x4C => "ANSI_KeypadEnter",
            0x4E => "ANSI_KeypadMinus",
            0x51 => "ANSI_KeypadEquals",
            0x52 => "ANSI_Keypad0",
            0x53 => "ANSI_Keypad1",
            0x54 => "ANSI_Keypad2",
            0x55 => "ANSI_Keypad3",
            0x56 => "ANSI_Keypad4",
            0x57 => "ANSI_Keypad5",
            0x58 => "ANSI_Keypad6",
            0x59 => "ANSI_Keypad7",
            0x5B => "ANSI_Keypad8",
            0x5C => "ANSI_Keypad9",
            0x24 => "Return",
            0x30 => "Tab",
            0x31 => "Space",
            0x33 => "Delete",
            0x35 => "Escape",
            0x37 => "Command",
            0x38 => "Shift",
            0x39 => "CapsLock",
            0x3A => "Option",
            0x3B => "Control",
            0x36 => "RightCommand",
            0x3C => "RightShift",
            0x3D => "RightOption",
            0x3E => "RightControl",
            0x3F => "Function",
            0x40 => "F17",
            0x48 => "VolumeUp",
            0x49 => "VolumeDown",
            0x4A => "Mute",
            0x4F => "F18",
            0x50 => "F19",
            0x5A => "F20",
            0x60 => "F5",
            0x61 => "F6",
            0x62 => "F7",
            0x63 => "F3",
            0x64 => "F8",
            0x65 => "F9",
            0x67 => "F11",
            0x69 => "F13",
            0x6A => "F16",
            0x6B => "F14",
            0x6D => "F10",
            0x6E => "ContextualMenu",
            0x6F => "F12",
            0x71 => "F15",
            0x72 => "Help",
            0x73 => "Home",
            0x74 => "PageUp",
            0x75 => "ForwardDelete",
            0x76 => "F4",
            0x77 => "End",
            0x78 => "F2",
            0x79 => "PageDown",
            0x7A => "F1",
            0x7B => "LeftArrow",
            0x7C => "RightArrow",
            0x7D => "DownArrow",
            0x7E => "UpArrow",
            0x0A => "ISO_Section",
            0x5D => "JIS_Yen",
            0x5E => "JIS_Underscore",
            0x5F => "JIS_KeypadComma",
            0x66 => "JIS_Eisu",
            0x68 => "JIS_Kana",
            _ => "Unknown",
        }
    }

    /// Return whether this key code is one of Apple's documented `kVK_*` values.
    #[must_use]
    pub fn is_known(self) -> bool {
        unsafe { crate::bridge_ffi::carbonhotkey_keycode_is_known(self.0) != 0 }
    }

    /// Return whether this key is itself a modifier key.
    #[must_use]
    pub fn is_modifier_key(self) -> bool {
        unsafe { crate::bridge_ffi::carbonhotkey_keycode_is_modifier_key(self.0) != 0 }
    }

    /// Return the number of documented `kVK_*` constants in the SDK.
    #[must_use]
    pub fn documented_count() -> usize {
        unsafe { crate::bridge_ffi::carbonhotkey_keycode_count() as usize }
    }

    /// Parse a `kVK_*` suffix, case-insensitively.
    #[allow(clippy::too_many_lines)]
    #[must_use]
    pub fn from_name(name: &str) -> Option<Self> {
        let mut normalized = name.trim().replace(['-', ' '], "_").to_ascii_uppercase();
        if let Some(stripped) = normalized.strip_prefix("K_VK_") {
            normalized = stripped.to_string();
        } else if let Some(stripped) = normalized.strip_prefix("KVK_") {
            normalized = stripped.to_string();
        }
        match normalized.as_str() {
            "ANSI_A" => Some(Self::ANSI_A),
            "ANSI_S" => Some(Self::ANSI_S),
            "ANSI_D" => Some(Self::ANSI_D),
            "ANSI_F" => Some(Self::ANSI_F),
            "ANSI_H" => Some(Self::ANSI_H),
            "ANSI_G" => Some(Self::ANSI_G),
            "ANSI_Z" => Some(Self::ANSI_Z),
            "ANSI_X" => Some(Self::ANSI_X),
            "ANSI_C" => Some(Self::ANSI_C),
            "ANSI_V" => Some(Self::ANSI_V),
            "ANSI_B" => Some(Self::ANSI_B),
            "ANSI_Q" => Some(Self::ANSI_Q),
            "ANSI_W" => Some(Self::ANSI_W),
            "ANSI_E" => Some(Self::ANSI_E),
            "ANSI_R" => Some(Self::ANSI_R),
            "ANSI_Y" => Some(Self::ANSI_Y),
            "ANSI_T" => Some(Self::ANSI_T),
            "ANSI_1" => Some(Self::ANSI_1),
            "ANSI_2" => Some(Self::ANSI_2),
            "ANSI_3" => Some(Self::ANSI_3),
            "ANSI_4" => Some(Self::ANSI_4),
            "ANSI_6" => Some(Self::ANSI_6),
            "ANSI_5" => Some(Self::ANSI_5),
            "ANSI_EQUAL" => Some(Self::ANSI_EQUAL),
            "ANSI_9" => Some(Self::ANSI_9),
            "ANSI_7" => Some(Self::ANSI_7),
            "ANSI_MINUS" => Some(Self::ANSI_MINUS),
            "ANSI_8" => Some(Self::ANSI_8),
            "ANSI_0" => Some(Self::ANSI_0),
            "ANSI_RIGHTBRACKET" | "ANSI_RIGHT_BRACKET" => Some(Self::ANSI_RIGHT_BRACKET),
            "ANSI_O" => Some(Self::ANSI_O),
            "ANSI_U" => Some(Self::ANSI_U),
            "ANSI_LEFTBRACKET" | "ANSI_LEFT_BRACKET" => Some(Self::ANSI_LEFT_BRACKET),
            "ANSI_I" => Some(Self::ANSI_I),
            "ANSI_P" => Some(Self::ANSI_P),
            "ANSI_L" => Some(Self::ANSI_L),
            "ANSI_J" => Some(Self::ANSI_J),
            "ANSI_QUOTE" => Some(Self::ANSI_QUOTE),
            "ANSI_K" => Some(Self::ANSI_K),
            "ANSI_SEMICOLON" => Some(Self::ANSI_SEMICOLON),
            "ANSI_BACKSLASH" => Some(Self::ANSI_BACKSLASH),
            "ANSI_COMMA" => Some(Self::ANSI_COMMA),
            "ANSI_SLASH" => Some(Self::ANSI_SLASH),
            "ANSI_N" => Some(Self::ANSI_N),
            "ANSI_M" => Some(Self::ANSI_M),
            "ANSI_PERIOD" => Some(Self::ANSI_PERIOD),
            "ANSI_GRAVE" => Some(Self::ANSI_GRAVE),
            "ANSI_KEYPADDECIMAL" | "ANSI_KEYPAD_DECIMAL" => Some(Self::ANSI_KEYPAD_DECIMAL),
            "ANSI_KEYPADMULTIPLY" | "ANSI_KEYPAD_MULTIPLY" => Some(Self::ANSI_KEYPAD_MULTIPLY),
            "ANSI_KEYPADPLUS" | "ANSI_KEYPAD_PLUS" => Some(Self::ANSI_KEYPAD_PLUS),
            "ANSI_KEYPADCLEAR" | "ANSI_KEYPAD_CLEAR" => Some(Self::ANSI_KEYPAD_CLEAR),
            "ANSI_KEYPADDIVIDE" | "ANSI_KEYPAD_DIVIDE" => Some(Self::ANSI_KEYPAD_DIVIDE),
            "ANSI_KEYPADENTER" | "ANSI_KEYPAD_ENTER" => Some(Self::ANSI_KEYPAD_ENTER),
            "ANSI_KEYPADMINUS" | "ANSI_KEYPAD_MINUS" => Some(Self::ANSI_KEYPAD_MINUS),
            "ANSI_KEYPADEQUALS" | "ANSI_KEYPAD_EQUALS" => Some(Self::ANSI_KEYPAD_EQUALS),
            "ANSI_KEYPAD0" => Some(Self::ANSI_KEYPAD0),
            "ANSI_KEYPAD1" => Some(Self::ANSI_KEYPAD1),
            "ANSI_KEYPAD2" => Some(Self::ANSI_KEYPAD2),
            "ANSI_KEYPAD3" => Some(Self::ANSI_KEYPAD3),
            "ANSI_KEYPAD4" => Some(Self::ANSI_KEYPAD4),
            "ANSI_KEYPAD5" => Some(Self::ANSI_KEYPAD5),
            "ANSI_KEYPAD6" => Some(Self::ANSI_KEYPAD6),
            "ANSI_KEYPAD7" => Some(Self::ANSI_KEYPAD7),
            "ANSI_KEYPAD8" => Some(Self::ANSI_KEYPAD8),
            "ANSI_KEYPAD9" => Some(Self::ANSI_KEYPAD9),
            "RETURN" => Some(Self::RETURN),
            "TAB" => Some(Self::TAB),
            "SPACE" => Some(Self::SPACE),
            "DELETE" => Some(Self::DELETE),
            "ESCAPE" => Some(Self::ESCAPE),
            "COMMAND" => Some(Self::COMMAND),
            "SHIFT" => Some(Self::SHIFT),
            "CAPSLOCK" | "CAPS_LOCK" => Some(Self::CAPS_LOCK),
            "OPTION" => Some(Self::OPTION),
            "CONTROL" => Some(Self::CONTROL),
            "RIGHTCOMMAND" | "RIGHT_COMMAND" => Some(Self::RIGHT_COMMAND),
            "RIGHTSHIFT" | "RIGHT_SHIFT" => Some(Self::RIGHT_SHIFT),
            "RIGHTOPTION" | "RIGHT_OPTION" => Some(Self::RIGHT_OPTION),
            "RIGHTCONTROL" | "RIGHT_CONTROL" => Some(Self::RIGHT_CONTROL),
            "FUNCTION" => Some(Self::FUNCTION),
            "F17" => Some(Self::F17),
            "VOLUMEUP" | "VOLUME_UP" => Some(Self::VOLUME_UP),
            "VOLUMEDOWN" | "VOLUME_DOWN" => Some(Self::VOLUME_DOWN),
            "MUTE" => Some(Self::MUTE),
            "F18" => Some(Self::F18),
            "F19" => Some(Self::F19),
            "F20" => Some(Self::F20),
            "F5" => Some(Self::F5),
            "F6" => Some(Self::F6),
            "F7" => Some(Self::F7),
            "F3" => Some(Self::F3),
            "F8" => Some(Self::F8),
            "F9" => Some(Self::F9),
            "F11" => Some(Self::F11),
            "F13" => Some(Self::F13),
            "F16" => Some(Self::F16),
            "F14" => Some(Self::F14),
            "F10" => Some(Self::F10),
            "CONTEXTUALMENU" | "CONTEXTUAL_MENU" => Some(Self::CONTEXTUAL_MENU),
            "F12" => Some(Self::F12),
            "F15" => Some(Self::F15),
            "HELP" => Some(Self::HELP),
            "HOME" => Some(Self::HOME),
            "PAGEUP" | "PAGE_UP" => Some(Self::PAGE_UP),
            "FORWARDDELETE" | "FORWARD_DELETE" => Some(Self::FORWARD_DELETE),
            "F4" => Some(Self::F4),
            "END" => Some(Self::END),
            "F2" => Some(Self::F2),
            "PAGEDOWN" | "PAGE_DOWN" => Some(Self::PAGE_DOWN),
            "F1" => Some(Self::F1),
            "LEFTARROW" | "LEFT_ARROW" => Some(Self::LEFT_ARROW),
            "RIGHTARROW" | "RIGHT_ARROW" => Some(Self::RIGHT_ARROW),
            "DOWNARROW" | "DOWN_ARROW" => Some(Self::DOWN_ARROW),
            "UPARROW" | "UP_ARROW" => Some(Self::UP_ARROW),
            "ISO_SECTION" => Some(Self::ISO_SECTION),
            "JIS_YEN" => Some(Self::JIS_YEN),
            "JIS_UNDERSCORE" => Some(Self::JIS_UNDERSCORE),
            "JIS_KEYPADCOMMA" | "JIS_KEYPAD_COMMA" => Some(Self::JIS_KEYPAD_COMMA),
            "JIS_EISU" => Some(Self::JIS_EISU),
            "JIS_KANA" => Some(Self::JIS_KANA),
            _ => None,
        }
    }
}

impl fmt::Debug for KeyCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("KeyCode")
            .field("name", &self.name())
            .field("raw", &format_args!("0x{:02X}", self.0))
            .finish()
    }
}

impl fmt::Display for KeyCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_known() {
            write!(f, "{}", self.name())
        } else {
            write!(f, "0x{:02X}", self.0)
        }
    }
}

impl From<u16> for KeyCode {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl From<KeyCode> for u16 {
    fn from(value: KeyCode) -> Self {
        value.0
    }
}
