import Carbon

public typealias CarbonHotKeyKeyboardCallback = @convention(c) (
    UInt32,
    UInt32,
    UInt32,
    UInt32,
    UnsafeMutableRawPointer?
) -> Void

private func carbonhotkeyKeyboardEventHandler(
    _ handlerCallRef: EventHandlerCallRef?,
    _ event: EventRef?,
    _ userData: UnsafeMutableRawPointer?
) -> OSStatus {
    _ = handlerCallRef

    guard let userData, let event else {
        return OSStatus(paramErr)
    }

    let handler = Unmanaged<CarbonHotKeyKeyboardHandler>.fromOpaque(userData).takeUnretainedValue()
    return handler.handleEvent(event)
}

final class CarbonHotKeyKeyboardHandler {
    private var eventHandlerRef: EventHandlerRef?
    private let callback: CarbonHotKeyKeyboardCallback
    private let callbackUserData: UnsafeMutableRawPointer?

    init(callback: @escaping CarbonHotKeyKeyboardCallback, callbackUserData: UnsafeMutableRawPointer?) {
        self.callback = callback
        self.callbackUserData = callbackUserData
    }

    func install() -> OSStatus {
        var eventHandlerRef: EventHandlerRef?
        let eventTypes = [
            EventTypeSpec(eventClass: OSType(kEventClassKeyboard), eventKind: UInt32(kEventHotKeyPressed)),
            EventTypeSpec(eventClass: OSType(kEventClassKeyboard), eventKind: UInt32(kEventHotKeyReleased))
        ]

        let status = eventTypes.withUnsafeBufferPointer { buffer in
            InstallEventHandler(
                GetApplicationEventTarget(),
                carbonhotkeyKeyboardEventHandler,
                buffer.count,
                buffer.baseAddress,
                Unmanaged.passUnretained(self).toOpaque(),
                &eventHandlerRef
            )
        }

        if status == noErr {
            self.eventHandlerRef = eventHandlerRef
        }

        return status
    }

    func remove() -> OSStatus {
        guard let eventHandlerRef else {
            return noErr
        }
        self.eventHandlerRef = nil
        return RemoveEventHandler(eventHandlerRef)
    }

    func handleEvent(_ event: EventRef) -> OSStatus {
        let eventClass = UInt32(GetEventClass(event))
        let eventKind = UInt32(GetEventKind(event))
        var hotKeyID = EventHotKeyID(signature: 0, id: 0)
        var actualSize = 0

        let status = withUnsafeMutablePointer(to: &hotKeyID) { hotKeyIDPointer in
            GetEventParameter(
                event,
                EventParamName(kEventParamDirectObject),
                EventParamType(typeEventHotKeyID),
                nil,
                MemoryLayout<EventHotKeyID>.size,
                &actualSize,
                hotKeyIDPointer
            )
        }

        guard status == noErr else {
            return status
        }

        callback(eventClass, eventKind, UInt32(hotKeyID.signature), hotKeyID.id, callbackUserData)
        return noErr
    }

    deinit {
        _ = remove()
    }
}

@_cdecl("carbonhotkey_event_handler_install")
public func carbonhotkeyEventHandlerInstall(
    _ callback: CarbonHotKeyKeyboardCallback?,
    _ userData: UnsafeMutableRawPointer?,
    _ outHandle: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> Int32 {
    guard let callback else {
        outHandle?.pointee = nil
        return OSStatus(paramErr)
    }

    let handler = CarbonHotKeyKeyboardHandler(callback: callback, callbackUserData: userData)
    let status = handler.install()
    guard status == noErr else {
        outHandle?.pointee = nil
        return status
    }

    outHandle?.pointee = Unmanaged.passRetained(handler).toOpaque()
    return noErr
}

@_cdecl("carbonhotkey_event_handler_remove")
public func carbonhotkeyEventHandlerRemove(_ handle: UnsafeMutableRawPointer?) -> Int32 {
    guard let handle else {
        return noErr
    }
    let handler = Unmanaged<CarbonHotKeyKeyboardHandler>.fromOpaque(handle).takeUnretainedValue()
    return handler.remove()
}

@_cdecl("carbonhotkey_event_handler_retain")
public func carbonhotkeyEventHandlerRetain(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let handle else {
        return nil
    }
    let handler = Unmanaged<CarbonHotKeyKeyboardHandler>.fromOpaque(handle).takeUnretainedValue()
    return Unmanaged.passRetained(handler).toOpaque()
}

@_cdecl("carbonhotkey_event_handler_release")
public func carbonhotkeyEventHandlerRelease(_ handle: UnsafeMutableRawPointer?) {
    guard let handle else {
        return
    }
    Unmanaged<CarbonHotKeyKeyboardHandler>.fromOpaque(handle).release()
}

@_cdecl("carbonhotkey_event_loop_run_for_seconds")
public func carbonhotkeyEventLoopRunForSeconds(_ seconds: Double) -> Int32 {
    RunCurrentEventLoop(EventTimeout(seconds))
}
