import Carbon
import Foundation

public typealias CarbonHotKeyKeyboardCallback = @convention(c) (
    UInt32,
    UInt32,
    UInt32,
    UInt32,
    UnsafeMutableRawPointer?
) -> Bool

public typealias CarbonHotKeyContextHook = @convention(c) (UnsafeMutableRawPointer?) -> Void

private func carbonhotkeyKeyboardEventHandler(
    _ handlerCallRef: EventHandlerCallRef?,
    _ event: EventRef?,
    _ userData: UnsafeMutableRawPointer?
) -> OSStatus {
    _ = handlerCallRef

    guard let userData, let event else {
        return OSStatus(eventNotHandledErr)
    }

    let handler = Unmanaged<CarbonHotKeyKeyboardHandler>.fromOpaque(userData).takeUnretainedValue()
    return withExtendedLifetime(handler) {
        handler.handleEvent(event)
    }
}

final class CarbonHotKeyKeyboardHandler {
    private var eventHandlerRef: EventHandlerRef?
    private let callback: CarbonHotKeyKeyboardCallback
    private let context: UnsafeMutableRawPointer?
    private let releaseContext: CarbonHotKeyContextHook?

    init(
        callback: @escaping CarbonHotKeyKeyboardCallback,
        context: UnsafeMutableRawPointer?,
        retainContext: CarbonHotKeyContextHook?,
        releaseContext: CarbonHotKeyContextHook?
    ) {
        self.callback = callback
        self.context = context
        self.releaseContext = releaseContext
        retainContext?(context)
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

        let status = withUnsafeMutablePointer(to: &hotKeyID) { hotKeyIDPointer in
            GetEventParameter(
                event,
                EventParamName(kEventParamDirectObject),
                EventParamType(typeEventHotKeyID),
                nil,
                MemoryLayout<EventHotKeyID>.size,
                nil,
                hotKeyIDPointer
            )
        }

        guard status == noErr else {
            return OSStatus(eventNotHandledErr)
        }

        let handled = callback(eventClass, eventKind, UInt32(hotKeyID.signature), hotKeyID.id, context)
        return handled ? noErr : OSStatus(eventNotHandledErr)
    }

    deinit {
        _ = remove()
        releaseContext?(context)
    }
}

private var hotKeyDispatcher: CarbonHotKeyKeyboardHandler?

@_cdecl("carbonhotkey_dispatcher_install")
public func carbonhotkeyDispatcherInstall(_ callback: CarbonHotKeyKeyboardCallback?) -> Int32 {
    guard let callback else {
        return OSStatus(paramErr)
    }
    guard Thread.isMainThread else {
        return OSStatus(paramErr)
    }
    guard hotKeyDispatcher == nil else {
        return noErr
    }
    let dispatcher = CarbonHotKeyKeyboardHandler(
        callback: callback,
        context: nil,
        retainContext: nil,
        releaseContext: nil)
    let status = dispatcher.install()
    if status == noErr {
        hotKeyDispatcher = dispatcher
    }
    return status
}

@_cdecl("carbonhotkey_event_handler_install")
public func carbonhotkeyEventHandlerInstall(
    _ callback: CarbonHotKeyKeyboardCallback?,
    _ context: UnsafeMutableRawPointer?,
    _ retainContext: CarbonHotKeyContextHook?,
    _ releaseContext: CarbonHotKeyContextHook?,
    _ outHandle: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> Int32 {
    outHandle?.pointee = nil
    guard let callback, let outHandle else {
        return OSStatus(paramErr)
    }
    guard Thread.isMainThread else {
        return OSStatus(paramErr)
    }

    let handler = CarbonHotKeyKeyboardHandler(
        callback: callback,
        context: context,
        retainContext: retainContext,
        releaseContext: releaseContext)
    let status = handler.install()
    guard status == noErr else {
        return status
    }

    outHandle.pointee = Unmanaged.passRetained(handler).toOpaque()
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
