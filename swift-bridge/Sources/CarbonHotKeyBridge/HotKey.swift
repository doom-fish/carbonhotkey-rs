import Carbon

final class CarbonHotKeyRegistration {
    private var hotKeyRef: EventHotKeyRef?
    let identifier: UInt32

    init(hotKeyRef: EventHotKeyRef, identifier: UInt32) {
        self.hotKeyRef = hotKeyRef
        self.identifier = identifier
    }

    func unregister() -> OSStatus {
        guard let hotKeyRef else {
            return noErr
        }
        self.hotKeyRef = nil
        return UnregisterEventHotKey(hotKeyRef)
    }

    deinit {
        _ = unregister()
    }
}

@_cdecl("carbonhotkey_hotkey_register")
public func carbonhotkeyHotKeyRegister(
    _ keyCode: UInt32,
    _ modifiers: UInt32,
    _ signature: UInt32,
    _ identifier: UInt32,
    _ options: UInt32,
    _ outHandle: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> Int32 {
    var hotKeyRef: EventHotKeyRef?
    let hotKeyID = EventHotKeyID(signature: OSType(signature), id: identifier)
    let status = RegisterEventHotKey(
        keyCode,
        modifiers,
        hotKeyID,
        GetApplicationEventTarget(),
        OptionBits(options),
        &hotKeyRef
    )

    guard status == noErr, let hotKeyRef else {
        outHandle?.pointee = nil
        return status
    }

    let registration = CarbonHotKeyRegistration(hotKeyRef: hotKeyRef, identifier: identifier)
    outHandle?.pointee = Unmanaged.passRetained(registration).toOpaque()
    return noErr
}

@_cdecl("carbonhotkey_hotkey_unregister")
public func carbonhotkeyHotKeyUnregister(_ handle: UnsafeMutableRawPointer?) -> Int32 {
    guard let handle else {
        return noErr
    }
    let registration = Unmanaged<CarbonHotKeyRegistration>.fromOpaque(handle).takeUnretainedValue()
    return registration.unregister()
}

@_cdecl("carbonhotkey_hotkey_id")
public func carbonhotkeyHotKeyId(_ handle: UnsafeMutableRawPointer?) -> UInt32 {
    guard let handle else {
        return 0
    }
    return Unmanaged<CarbonHotKeyRegistration>.fromOpaque(handle).takeUnretainedValue().identifier
}

@_cdecl("carbonhotkey_hotkey_retain")
public func carbonhotkeyHotKeyRetain(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let handle else {
        return nil
    }
    let registration = Unmanaged<CarbonHotKeyRegistration>.fromOpaque(handle).takeUnretainedValue()
    return Unmanaged.passRetained(registration).toOpaque()
}

@_cdecl("carbonhotkey_hotkey_release")
public func carbonhotkeyHotKeyRelease(_ handle: UnsafeMutableRawPointer?) {
    guard let handle else {
        return
    }
    Unmanaged<CarbonHotKeyRegistration>.fromOpaque(handle).release()
}
