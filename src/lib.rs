#![doc = include_str!("../README.md")]
//!
//! ---
//!
//! # API documentation
//!
//! Safe Rust bindings for Carbon's `RegisterEventHotKey` API on macOS —
//! global keyboard shortcuts that fire even when your app isn't focused.

#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod error;
pub mod ffi;
pub mod hotkey;

pub use error::HotkeyError;
pub use hotkey::{quit_event_loop, register, run_event_loop, Hotkey, HotkeyEdge, Modifier};

/// Common imports.
pub mod prelude {
    pub use crate::error::HotkeyError;
    pub use crate::hotkey::{
        quit_event_loop, register, run_event_loop, Hotkey, HotkeyEdge, Modifier,
    };
}
