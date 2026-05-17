#![doc = include_str!("../README.md")]
//!
//! ---
//!
//! # API documentation
//!
//! Safe Rust bindings for Carbon's `RegisterEventHotKey` API on macOS —
//! global keyboard shortcuts that fire even when your app isn't focused.

#![cfg_attr(docsrs, feature(doc_cfg))]

mod bridge_ffi;

#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
pub mod async_api;

pub mod error;
pub mod event_handler;
#[cfg(feature = "raw-ffi")]
#[cfg_attr(docsrs, doc(cfg(feature = "raw-ffi")))]
pub mod ffi;
pub mod hotkey;
pub mod key_code;
pub mod modifier_flags;

pub use error::HotkeyError;
pub use event_handler::{
    install_keyboard_handler, quit_event_loop, run_current_event_loop, run_event_loop,
    EventHandler, HotKeyEvent, HotKeyEventKind,
};
pub use hotkey::{
    register, register_key, register_key_with_options, register_with_options, HotKeyOptions,
    Hotkey, HotkeyEdge,
};
pub use key_code::KeyCode;
pub use modifier_flags::ModifierFlags;
pub use modifier_flags::ModifierFlags as Modifier;

/// Common imports.
pub mod prelude {
    pub use crate::error::HotkeyError;
    pub use crate::event_handler::{
        install_keyboard_handler, quit_event_loop, run_current_event_loop, run_event_loop,
        EventHandler, HotKeyEvent, HotKeyEventKind,
    };
    pub use crate::hotkey::{
        register, register_key, register_key_with_options, register_with_options, HotKeyOptions,
        Hotkey, HotkeyEdge,
    };
    pub use crate::key_code::KeyCode;
    pub use crate::modifier_flags::ModifierFlags;
    pub use crate::modifier_flags::ModifierFlags as Modifier;
}
