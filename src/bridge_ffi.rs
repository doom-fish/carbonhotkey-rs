//! Private FFI bindings to the Swift bridge.

#![allow(dead_code, missing_docs)]

use core::ffi::c_void;

pub use apple_cf::raw::{OSStatus, OSType};

pub type KeyboardEventCallback = unsafe extern "C" fn(
    event_class: OSType,
    event_kind: u32,
    signature: OSType,
    hotkey_id: u32,
    user_data: *mut c_void,
);

extern "C" {
    pub fn carbonhotkey_hotkey_register(
        key_code: u32,
        modifiers: u32,
        signature: OSType,
        identifier: u32,
        options: u32,
        out_handle: *mut *mut c_void,
    ) -> OSStatus;

    pub fn carbonhotkey_hotkey_unregister(handle: *mut c_void) -> OSStatus;
    pub fn carbonhotkey_hotkey_id(handle: *mut c_void) -> u32;
    pub fn carbonhotkey_hotkey_retain(handle: *mut c_void) -> *mut c_void;
    pub fn carbonhotkey_hotkey_release(handle: *mut c_void);

    pub fn carbonhotkey_event_handler_install(
        callback: KeyboardEventCallback,
        user_data: *mut c_void,
        out_handle: *mut *mut c_void,
    ) -> OSStatus;

    pub fn carbonhotkey_event_handler_remove(handle: *mut c_void) -> OSStatus;
    pub fn carbonhotkey_event_handler_retain(handle: *mut c_void) -> *mut c_void;
    pub fn carbonhotkey_event_handler_release(handle: *mut c_void);
    pub fn carbonhotkey_event_loop_run_for_seconds(seconds: f64) -> OSStatus;

    pub fn carbonhotkey_keycode_count() -> u32;
    pub fn carbonhotkey_keycode_is_known(key_code: u16) -> i32;
    pub fn carbonhotkey_keycode_is_modifier_key(key_code: u16) -> i32;

    pub fn carbonhotkey_modifierflags_supported_mask() -> u32;
    pub fn carbonhotkey_modifierflags_right_side_mask() -> u32;
    pub fn carbonhotkey_modifierflags_has_right_side(flags: u32) -> i32;
}
