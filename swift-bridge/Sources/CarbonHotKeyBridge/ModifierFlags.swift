import Carbon

private let carbonhotkeySupportedModifierMask: UInt32 =
    UInt32(cmdKey) | UInt32(shiftKey) | UInt32(alphaLock) | UInt32(optionKey) | UInt32(controlKey) |
    UInt32(rightShiftKey) | UInt32(rightOptionKey) | UInt32(rightControlKey)

private let carbonhotkeyRightSideModifierMask: UInt32 =
    UInt32(rightShiftKey) | UInt32(rightOptionKey) | UInt32(rightControlKey)

@_cdecl("carbonhotkey_modifierflags_supported_mask")
public func carbonhotkeyModifierFlagsSupportedMask() -> UInt32 {
    carbonhotkeySupportedModifierMask
}

@_cdecl("carbonhotkey_modifierflags_right_side_mask")
public func carbonhotkeyModifierFlagsRightSideMask() -> UInt32 {
    carbonhotkeyRightSideModifierMask
}

@_cdecl("carbonhotkey_modifierflags_has_right_side")
public func carbonhotkeyModifierFlagsHasRightSide(_ flags: UInt32) -> Int32 {
    flags & carbonhotkeyRightSideModifierMask == 0 ? 0 : 1
}
