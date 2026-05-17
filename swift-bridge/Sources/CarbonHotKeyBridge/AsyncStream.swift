import Carbon

/// Callback signature for async hotkey events.
/// Called with (hotkey_id, event_kind, event_class, signature) and context.
public typealias CarbonHotKeyAsyncEventCallback = @convention(c) (
    UInt32,
    UInt32,
    UInt32,
    UInt32,
    UnsafeMutableRawPointer?
) -> Void

/// Bridge holder for multiplexed hotkey event stream.
/// Installs a single application event handler and routes all hotkey fires
/// to the active stream via the C callback.
final class CarbonHotKeyAsyncStreamBridge: NSObject {
    private var eventHandlerRef: EventHandlerRef?
    private let callback: CarbonHotKeyAsyncEventCallback
    private let callbackUserData: UnsafeMutableRawPointer?

    init(
        callback: @escaping CarbonHotKeyAsyncEventCallback,
        callbackUserData: UnsafeMutableRawPointer?
    ) {
        self.callback = callback
        self.callbackUserData = callbackUserData
    }

    /// Install the application event handler for hotkey events.
    func install() -> OSStatus {
        var eventHandlerRef: EventHandlerRef?
        let eventTypes = [
            EventTypeSpec(eventClass: OSType(kEventClassKeyboard), eventKind: UInt32(kEventHotKeyPressed)),
            EventTypeSpec(eventClass: OSType(kEventClassKeyboard), eventKind: UInt32(kEventHotKeyReleased))
        ]

        let status = eventTypes.withUnsafeBufferPointer { buffer in
            InstallEventHandler(
                GetApplicationEventTarget(),
                carbonHotKeyAsyncEventHandler,
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

    /// Remove the installed event handler.
    func remove() -> OSStatus {
        guard let eventHandlerRef else {
            return noErr
        }
        self.eventHandlerRef = nil
        return RemoveEventHandler(eventHandlerRef)
    }

    /// Handle a Carbon hotkey event and invoke the callback.
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

        // Invoke the callback with hotkey_id, event_kind, event_class, signature
        callback(hotKeyID.id, eventKind, eventClass, UInt32(hotKeyID.signature), callbackUserData)
        return noErr
    }

    deinit {
        _ = remove()
    }
}

/// C callback trampoline for the event handler.
private func carbonHotKeyAsyncEventHandler(
    _ handlerCallRef: EventHandlerCallRef?,
    _ event: EventRef?,
    _ userData: UnsafeMutableRawPointer?
) -> OSStatus {
    _ = handlerCallRef

    guard let userData, let event else {
        return OSStatus(paramErr)
    }

    let bridge = Unmanaged<CarbonHotKeyAsyncStreamBridge>.fromOpaque(userData).takeUnretainedValue()
    return bridge.handleEvent(event)
}

/// Subscribe to hotkey events as an async stream.
/// Installs a single application event handler that routes all hotkey fires
/// to the active stream's callback.
///
/// Returns an opaque handle that must be passed to `carbonhotkey_hotkey_stream_unsubscribe`
/// to release the bridge and the event handler.
@_cdecl("carbonhotkey_hotkey_stream_subscribe")
public func carbonhotKeyHotKeyStreamSubscribe(
    _ callback: CarbonHotKeyAsyncEventCallback?,
    _ userData: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    guard let callback else {
        return nil
    }

    let bridge = CarbonHotKeyAsyncStreamBridge(callback: callback, callbackUserData: userData)
    let status = bridge.install()
    guard status == noErr else {
        return nil
    }

    return Unmanaged.passRetained(bridge).toOpaque()
}

/// Unsubscribe from hotkey events and release the stream bridge.
@_cdecl("carbonhotkey_hotkey_stream_unsubscribe")
public func carbonhotKeyHotKeyStreamUnsubscribe(_ handle: UnsafeMutableRawPointer?) {
    guard let handle else {
        return
    }
    Unmanaged<CarbonHotKeyAsyncStreamBridge>.fromOpaque(handle).release()
}
