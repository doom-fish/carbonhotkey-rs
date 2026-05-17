//! Keyboard event-handler installation and Carbon event-loop helpers.

use core::ffi::c_void;
use core::ptr::NonNull;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use crate::bridge_ffi;
use crate::error::HotkeyError;

const KEYBOARD_EVENT_CLASS: u32 = 0x6b65_7962;
const HOTKEY_PRESSED_KIND: u32 = 5;
const HOTKEY_RELEASED_KIND: u32 = 6;
const EVENT_LOOP_SLICE: Duration = Duration::from_millis(50);
const EVENT_LOOP_TIMED_OUT_ERR: i32 = -9875;
const EVENT_LOOP_QUIT_ERR: i32 = -9876;

static SHOULD_QUIT_EVENT_LOOP: AtomicBool = AtomicBool::new(false);

/// Hotkey event kind delivered through Carbon's keyboard event class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HotKeyEventKind {
    Pressed,
    Released,
}

impl HotKeyEventKind {
    #[must_use]
    pub const fn from_raw(raw: u32) -> Option<Self> {
        match raw {
            HOTKEY_PRESSED_KIND => Some(Self::Pressed),
            HOTKEY_RELEASED_KIND => Some(Self::Released),
            _ => None,
        }
    }

    #[must_use]
    pub const fn raw(self) -> u32 {
        match self {
            Self::Pressed => HOTKEY_PRESSED_KIND,
            Self::Released => HOTKEY_RELEASED_KIND,
        }
    }
}

/// Decoded `kEventClassKeyboard` hotkey event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HotKeyEvent {
    event_class: u32,
    event_kind: HotKeyEventKind,
    signature: u32,
    identifier: u32,
}

impl HotKeyEvent {
    #[must_use]
    pub const fn event_class(self) -> u32 {
        self.event_class
    }

    #[must_use]
    pub const fn event_kind(self) -> HotKeyEventKind {
        self.event_kind
    }

    #[must_use]
    pub const fn raw_event_kind(self) -> u32 {
        self.event_kind.raw()
    }

    #[must_use]
    pub const fn signature(self) -> u32 {
        self.signature
    }

    #[must_use]
    pub const fn id(self) -> u32 {
        self.identifier
    }

    #[must_use]
    pub const fn is_keyboard_hotkey(self) -> bool {
        self.event_class == KEYBOARD_EVENT_CLASS
    }
}

type KeyboardCallback = Box<dyn Fn(HotKeyEvent) + Send + Sync + 'static>;

struct CallbackState {
    callback: KeyboardCallback,
}

/// Installed keyboard-event handler.
pub struct EventHandler {
    handle: Option<NonNull<c_void>>,
    callback_state: Option<NonNull<CallbackState>>,
}

unsafe extern "C" fn keyboard_callback_trampoline(
    event_class: u32,
    event_kind: u32,
    signature: u32,
    hotkey_id: u32,
    user_data: *mut c_void,
) {
    let Some(event_kind) = HotKeyEventKind::from_raw(event_kind) else {
        return;
    };
    let Some(user_data) = NonNull::new(user_data.cast::<CallbackState>()) else {
        return;
    };

    let callback_state = unsafe { user_data.as_ref() };
    (callback_state.callback)(HotKeyEvent {
        event_class,
        event_kind,
        signature,
        identifier: hotkey_id,
    });
}

impl Drop for EventHandler {
    fn drop(&mut self) {
        if let Some(handle) = self.handle.take() {
            unsafe {
                let _ = bridge_ffi::carbonhotkey_event_handler_remove(handle.as_ptr());
                bridge_ffi::carbonhotkey_event_handler_release(handle.as_ptr());
            }
        }
        if let Some(callback_state) = self.callback_state.take() {
            unsafe { drop(Box::from_raw(callback_state.as_ptr())) };
        }
    }
}

impl EventHandler {
    /// Install a keyboard-event handler for `kEventHotKeyPressed` and
    /// `kEventHotKeyReleased`.
    ///
    /// # Errors
    ///
    /// Returns [`HotkeyError::HandlerInstallFailed`] if Carbon refuses to
    /// install the handler.
    pub fn install<F>(callback: F) -> Result<Self, HotkeyError>
    where
        F: Fn(HotKeyEvent) + Send + Sync + 'static,
    {
        let callback_state = Box::new(CallbackState {
            callback: Box::new(callback),
        });
        let callback_state = unsafe { NonNull::new_unchecked(Box::into_raw(callback_state)) };

        let mut handle = core::ptr::null_mut();
        let status = unsafe {
            bridge_ffi::carbonhotkey_event_handler_install(
                keyboard_callback_trampoline,
                callback_state.as_ptr().cast(),
                &mut handle,
            )
        };

        if status != 0 {
            unsafe { drop(Box::from_raw(callback_state.as_ptr())) };
            return Err(HotkeyError::HandlerInstallFailed(status));
        }

        Ok(Self {
            handle: NonNull::new(handle),
            callback_state: Some(callback_state),
        })
    }

    /// Remove the installed handler immediately.
    ///
    /// # Errors
    ///
    /// Returns [`HotkeyError::HandlerRemoveFailed`] if Carbon refuses to
    /// remove the handler.
    pub fn remove(mut self) -> Result<(), HotkeyError> {
        let status = self.handle.take().map_or(0, |handle| {
            let status = unsafe { bridge_ffi::carbonhotkey_event_handler_remove(handle.as_ptr()) };
            unsafe { bridge_ffi::carbonhotkey_event_handler_release(handle.as_ptr()) };
            status
        });

        if let Some(callback_state) = self.callback_state.take() {
            unsafe { drop(Box::from_raw(callback_state.as_ptr())) };
        }

        core::mem::forget(self);

        if status != 0 {
            return Err(HotkeyError::HandlerRemoveFailed(status));
        }
        Ok(())
    }
}

/// Install a keyboard-event handler for Carbon hotkey callbacks.
///
/// # Errors
///
/// Returns [`HotkeyError::HandlerInstallFailed`] if Carbon refuses to
/// install the handler.
pub fn install_keyboard_handler<F>(callback: F) -> Result<EventHandler, HotkeyError>
where
    F: Fn(HotKeyEvent) + Send + Sync + 'static,
{
    EventHandler::install(callback)
}

/// Run Carbon's current event loop for a bounded amount of time.
///
/// # Errors
///
/// Returns [`HotkeyError::EventLoopFailed`] if Carbon returns a non-zero
/// `OSStatus`.
pub fn run_current_event_loop(duration: Duration) -> Result<(), HotkeyError> {
    let status =
        unsafe { bridge_ffi::carbonhotkey_event_loop_run_for_seconds(duration.as_secs_f64()) };
    if matches!(status, 0 | EVENT_LOOP_TIMED_OUT_ERR | EVENT_LOOP_QUIT_ERR) {
        return Ok(());
    }
    Err(HotkeyError::EventLoopFailed(status))
}

/// Run the current event loop until [`quit_event_loop`] is called.
///
/// # Panics
///
/// Panics if `RunCurrentEventLoop` returns a non-zero `OSStatus`.
pub fn run_event_loop() {
    SHOULD_QUIT_EVENT_LOOP.store(false, Ordering::SeqCst);
    while !SHOULD_QUIT_EVENT_LOOP.load(Ordering::SeqCst) {
        run_current_event_loop(EVENT_LOOP_SLICE).expect("RunCurrentEventLoop failed");
    }
}

/// Request that [`run_event_loop`] return on its next polling slice.
pub fn quit_event_loop() {
    SHOULD_QUIT_EVENT_LOOP.store(true, Ordering::SeqCst);
}
