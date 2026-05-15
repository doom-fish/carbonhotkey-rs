//! Errors returned by the `carbonhotkey` crate.

use core::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum HotkeyError {
    /// `RegisterEventHotKey` returned non-zero.
    RegisterFailed(i32),
    /// Hotkey already registered by another process with `kEventHotKeyExclusive`.
    AlreadyRegistered,
    /// `InstallEventHandler` returned non-zero.
    HandlerInstallFailed(i32),
    /// `UnregisterEventHotKey` returned non-zero.
    UnregisterFailed(i32),
}

impl fmt::Display for HotkeyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RegisterFailed(s) => write!(f, "RegisterEventHotKey failed: OSStatus {s}"),
            Self::AlreadyRegistered => write!(f, "hotkey already registered"),
            Self::HandlerInstallFailed(s) => {
                write!(f, "InstallEventHandler failed: OSStatus {s}")
            }
            Self::UnregisterFailed(s) => write!(f, "UnregisterEventHotKey failed: OSStatus {s}"),
        }
    }
}

impl std::error::Error for HotkeyError {}
