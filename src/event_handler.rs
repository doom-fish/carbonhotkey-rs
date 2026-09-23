//! Keyboard event-handler installation and Carbon event-loop helpers.

use core::ffi::c_void;
use core::ptr::NonNull;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use doom_fish_utils::callback_context::CallbackContext;

use crate::bridge_ffi;
use crate::error::HotkeyError;

pub(crate) const KEYBOARD_EVENT_CLASS: u32 = 0x6b65_7962;
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
type KeyboardContext = CallbackContext<KeyboardCallback>;

/// Installed keyboard-event handler.
pub struct EventHandler {
    handle: Option<NonNull<c_void>>,
    context: KeyboardContext,
}

unsafe extern "C" fn keyboard_callback_trampoline(
    event_class: u32,
    event_kind: u32,
    signature: u32,
    hotkey_id: u32,
    user_data: *mut c_void,
) -> bool {
    let Some(event_kind) = HotKeyEventKind::from_raw(event_kind) else {
        return false;
    };
    let event = HotKeyEvent {
        event_class,
        event_kind,
        signature,
        identifier: hotkey_id,
    };
    let _ = unsafe {
        KeyboardContext::with(user_data, "keyboard_callback_trampoline", |callback| {
            callback(event);
        })
    };
    false
}

impl Drop for EventHandler {
    fn drop(&mut self) {
        let _ = self.detach();
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
        if unsafe { bridge_ffi::pthread_main_np() } == 0 {
            return Err(HotkeyError::NotMainThread);
        }
        let context = KeyboardContext::new(Box::new(callback));
        let mut handle = core::ptr::null_mut();
        let status = unsafe {
            bridge_ffi::carbonhotkey_event_handler_install(
                Some(keyboard_callback_trampoline),
                context.as_ptr(),
                Some(KeyboardContext::RETAIN),
                Some(KeyboardContext::RELEASE),
                &raw mut handle,
            )
        };

        if status != 0 {
            return Err(HotkeyError::HandlerInstallFailed(status));
        }
        let Some(handle) = NonNull::new(handle) else {
            return Err(HotkeyError::HandlerInstallFailed(-50));
        };

        Ok(Self {
            handle: Some(handle),
            context,
        })
    }

    /// Remove the installed handler immediately.
    ///
    /// # Errors
    ///
    /// Returns [`HotkeyError::HandlerRemoveFailed`] if Carbon refuses to
    /// remove the handler.
    pub fn remove(mut self) -> Result<(), HotkeyError> {
        match self.detach() {
            0 => Ok(()),
            status => Err(HotkeyError::HandlerRemoveFailed(status)),
        }
    }

    fn detach(&mut self) -> i32 {
        self.context.deactivate();
        self.handle.take().map_or(0, |handle| unsafe {
            let status = bridge_ffi::carbonhotkey_event_handler_remove(handle.as_ptr());
            bridge_ffi::carbonhotkey_event_handler_release(handle.as_ptr());
            status
        })
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
/// # Errors
///
/// Returns [`HotkeyError::EventLoopFailed`] if `RunCurrentEventLoop` returns
/// a non-zero `OSStatus`.
pub fn run_event_loop() -> Result<(), HotkeyError> {
    while !SHOULD_QUIT_EVENT_LOOP.swap(false, Ordering::SeqCst) {
        run_current_event_loop(EVENT_LOOP_SLICE)?;
    }
    Ok(())
}

/// Request that [`run_event_loop`] return on its next polling slice.
pub fn quit_event_loop() {
    SHOULD_QUIT_EVENT_LOOP.store(true, Ordering::SeqCst);
}
