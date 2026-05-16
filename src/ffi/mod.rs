//! Raw Carbon FFI declarations for the hotkey/event-handler slice.
//!
//! This module is gated behind the crate's `raw-ffi` feature. The safe
//! API uses the Swift bridge instead.

#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    missing_docs
)]

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
pub const kEventHotKeyNoOptions: OptionBits = 0;
pub const kEventHotKeyExclusive: OptionBits = 1 << 0;
pub const eventHotKeyExistsErr: OSStatus = -9878;

pub const kEventParamDirectObject: EventParamName = 0x2d2d_2d2d; // '----'
pub const typeEventHotKeyID: EventParamType = 0x686b_6964; // 'hkid'

// Carbon modifier masks
pub const cmdKeyBit: UInt32 = 8;
pub const shiftKeyBit: UInt32 = 9;
pub const alphaLockBit: UInt32 = 10;
pub const optionKeyBit: UInt32 = 11;
pub const controlKeyBit: UInt32 = 12;
pub const rightShiftKeyBit: UInt32 = 13;
pub const rightOptionKeyBit: UInt32 = 14;
pub const rightControlKeyBit: UInt32 = 15;

pub const cmdKey: UInt32 = 1 << cmdKeyBit;
pub const shiftKey: UInt32 = 1 << shiftKeyBit;
pub const alphaLock: UInt32 = 1 << alphaLockBit;
pub const optionKey: UInt32 = 1 << optionKeyBit;
pub const controlKey: UInt32 = 1 << controlKeyBit;
pub const rightShiftKey: UInt32 = 1 << rightShiftKeyBit;
pub const rightOptionKey: UInt32 = 1 << rightOptionKeyBit;
pub const rightControlKey: UInt32 = 1 << rightControlKeyBit;

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
