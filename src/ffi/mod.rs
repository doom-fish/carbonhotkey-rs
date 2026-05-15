//! Raw FFI declarations for Carbon's `RegisterEventHotKey` API.
//!
//! Pure C — no Swift bridge. Linked against `Carbon` + `CoreFoundation`.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, missing_docs)]

use core::ffi::c_void;

pub type OSStatus = i32;
pub type OSType = u32;
pub type UInt32 = u32;
pub type ItemCount = u64;
pub type ByteCount = u64;
pub type OptionBits = u32;

pub type EventTargetRef = *mut c_void;
pub type EventHandlerRef = *mut c_void;
pub type EventHandlerUPP = *mut c_void;
pub type EventHandlerCallRef = *mut c_void;
pub type EventRef = *mut c_void;
pub type EventHotKeyRef = *mut c_void;
pub type EventParamName = OSType;
pub type EventParamType = OSType;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct EventHotKeyID {
    pub signature: OSType,
    pub id: UInt32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct EventTypeSpec {
    pub eventClass: OSType,
    pub eventKind: UInt32,
}

// Event class / kind constants
pub const kEventClassKeyboard: OSType = 0x6b65_7962; // 'keyb'
pub const kEventHotKeyPressed: UInt32 = 5;
pub const kEventHotKeyReleased: UInt32 = 6;

pub const kEventParamDirectObject: EventParamName = 0x2d2d_2d2d; // '----'
pub const typeEventHotKeyID: EventParamType = 0x686b_6964; // 'hkid'

// Carbon modifier masks
pub const cmdKey: UInt32 = 1 << 8;
pub const shiftKey: UInt32 = 1 << 9;
pub const alphaLock: UInt32 = 1 << 10;
pub const optionKey: UInt32 = 1 << 11;
pub const controlKey: UInt32 = 1 << 12;
pub const rightShiftKey: UInt32 = 1 << 13;
pub const rightOptionKey: UInt32 = 1 << 14;
pub const rightControlKey: UInt32 = 1 << 15;

pub type EventHandlerProcPtr = unsafe extern "C" fn(
    in_handler_call_ref: EventHandlerCallRef,
    in_event: EventRef,
    in_user_data: *mut c_void,
) -> OSStatus;

extern "C" {
    pub fn RegisterEventHotKey(
        inHotKeyCode: UInt32,
        inHotKeyModifiers: UInt32,
        inHotKeyID: EventHotKeyID,
        inTarget: EventTargetRef,
        inOptions: OptionBits,
        outRef: *mut EventHotKeyRef,
    ) -> OSStatus;

    pub fn UnregisterEventHotKey(inHotKey: EventHotKeyRef) -> OSStatus;

    pub fn GetApplicationEventTarget() -> EventTargetRef;

    pub fn InstallEventHandler(
        inTarget: EventTargetRef,
        inHandler: EventHandlerProcPtr,
        inNumTypes: ItemCount,
        inList: *const EventTypeSpec,
        inUserData: *mut c_void,
        outRef: *mut EventHandlerRef,
    ) -> OSStatus;

    pub fn RemoveEventHandler(inHandlerRef: EventHandlerRef) -> OSStatus;

    pub fn GetEventClass(inEvent: EventRef) -> OSType;
    pub fn GetEventKind(inEvent: EventRef) -> UInt32;
    pub fn GetEventParameter(
        inEvent: EventRef,
        inName: EventParamName,
        inDesiredType: EventParamType,
        outActualType: *mut EventParamType,
        inBufferSize: ByteCount,
        outActualSize: *mut ByteCount,
        outData: *mut c_void,
    ) -> OSStatus;

    pub fn RunCurrentEventLoop(inTimeout: f64) -> OSStatus;
    pub fn QuitApplicationEventLoop();
    pub fn RunApplicationEventLoop();
}
