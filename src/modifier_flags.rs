//! Carbon modifier flags from `Events.h`.

bitflags::bitflags! {
    /// Carbon modifier flags used by `RegisterEventHotKey`.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
    pub struct ModifierFlags: u32 {
        const CMD = 1 << 8;
        const SHIFT = 1 << 9;
        const CAPS = 1 << 10;
        const OPTION = 1 << 11;
        const CONTROL = 1 << 12;
        const RIGHT_SHIFT = 1 << 13;
        const RIGHT_OPTION = 1 << 14;
        const RIGHT_CONTROL = 1 << 15;
    }
}

impl ModifierFlags {
    /// All modifier bits Carbon exposes for global hotkeys.
    #[must_use]
    pub fn supported_mask() -> Self {
        unsafe {
            Self::from_bits_retain(crate::bridge_ffi::carbonhotkey_modifierflags_supported_mask())
        }
    }

    /// Right-side modifier bits from `Events.h`.
    #[must_use]
    pub fn right_side_mask() -> Self {
        unsafe {
            Self::from_bits_retain(crate::bridge_ffi::carbonhotkey_modifierflags_right_side_mask())
        }
    }

    /// Return whether any right-side modifier is present.
    #[must_use]
    pub fn contains_right_side(self) -> bool {
        unsafe { crate::bridge_ffi::carbonhotkey_modifierflags_has_right_side(self.bits()) != 0 }
    }
}
