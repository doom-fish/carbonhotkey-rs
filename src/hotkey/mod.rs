//! Global keyboard hotkey registration via Carbon's `RegisterEventHotKey`.

use core::ffi::c_void;
use core::ptr::NonNull;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard, OnceLock};

use crate::bridge_ffi;
use crate::error::HotkeyError;
use crate::event_handler::{self, HotKeyEventKind};
use crate::key_code::KeyCode;
use crate::modifier_flags::ModifierFlags;

const VENDOR_SIGNATURE: u32 = 0x646f_6f6d;
const EVENT_HOTKEY_EXISTS_ERR: i32 = -9878;

type Callback = dyn Fn(HotkeyEdge) + Send + Sync + 'static;

bitflags::bitflags! {
    /// Additional Carbon hotkey-registration options.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
    pub struct HotKeyOptions: u32 {
        /// Mirror Carbon's `kEventHotKeyNoOptions` constant.
        const NO_OPTIONS = 0;
        /// Request per-process exclusive registration.
        const EXCLUSIVE = 1 << 0;
    }
}

/// Whether the hotkey fired on key down or key up.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HotkeyEdge {
    Pressed,
    Released,
}

/// One registered hotkey. Drops the registration when this value drops.
pub struct Hotkey {
    handle: Option<NonNull<c_void>>,
    id: u32,
}

unsafe impl Send for Hotkey {}
unsafe impl Sync for Hotkey {}

impl Drop for Hotkey {
    fn drop(&mut self) {
        if let Some(handle) = self.handle.take() {
            unsafe {
                let _ = bridge_ffi::carbonhotkey_hotkey_unregister(handle.as_ptr());
                bridge_ffi::carbonhotkey_hotkey_release(handle.as_ptr());
            }
        }
        lock_callback_table().remove(&self.id);
    }
}

impl Hotkey {
    /// The internal hotkey identifier used by Carbon.
    #[must_use]
    pub const fn id(&self) -> u32 {
        self.id
    }

    /// Unregister the hotkey immediately.
    ///
    /// # Errors
    ///
    /// Returns [`HotkeyError::UnregisterFailed`] if Carbon refuses to
    /// unregister the hotkey.
    pub fn unregister(mut self) -> Result<(), HotkeyError> {
        let status = self.handle.take().map_or(0, |handle| {
            let status = unsafe { bridge_ffi::carbonhotkey_hotkey_unregister(handle.as_ptr()) };
            unsafe { bridge_ffi::carbonhotkey_hotkey_release(handle.as_ptr()) };
            status
        });

        lock_callback_table().remove(&self.id);
        core::mem::forget(self);

        if status != 0 {
            return Err(HotkeyError::UnregisterFailed(status));
        }
        Ok(())
    }
}

fn callback_table() -> &'static Mutex<HashMap<u32, Arc<Callback>>> {
    static TABLE: OnceLock<Mutex<HashMap<u32, Arc<Callback>>>> = OnceLock::new();
    TABLE.get_or_init(|| Mutex::new(HashMap::new()))
}

fn lock_callback_table() -> MutexGuard<'static, HashMap<u32, Arc<Callback>>> {
    match callback_table().lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    }
}

fn next_id() -> u32 {
    use std::sync::atomic::{AtomicU32, Ordering};

    static NEXT_ID: AtomicU32 = AtomicU32::new(1);
    NEXT_ID.fetch_add(1, Ordering::SeqCst)
}

fn install_handler_once() -> Result<(), HotkeyError> {
    static HANDLER: OnceLock<Result<usize, HotkeyError>> = OnceLock::new();

    HANDLER
        .get_or_init(|| {
            event_handler::install_keyboard_handler(|event| {
                let edge = match event.event_kind() {
                    HotKeyEventKind::Pressed => HotkeyEdge::Pressed,
                    HotKeyEventKind::Released => HotkeyEdge::Released,
                };

                let callback = { lock_callback_table().get(&event.id()).cloned() };

                if let Some(callback) = callback {
                    callback(edge);
                }
            })
            .map(|handler| Box::into_raw(Box::new(handler)) as usize)
        })
        .clone()
        .map(|_| ())
}

/// Register a global hotkey using a raw Carbon key code.
///
/// # Errors
///
/// * [`HotkeyError::HandlerInstallFailed`] if the shared keyboard event
///   handler could not be installed.
/// * [`HotkeyError::AlreadyRegistered`] if the same hotkey already exists.
/// * [`HotkeyError::RegisterFailed`] for other Carbon failures.
pub fn register<F>(
    keycode: u16,
    modifiers: ModifierFlags,
    callback: F,
) -> Result<Hotkey, HotkeyError>
where
    F: Fn(HotkeyEdge) + Send + Sync + 'static,
{
    register_with_options(keycode, modifiers, HotKeyOptions::NO_OPTIONS, callback)
}

/// Register a global hotkey using a typed [`KeyCode`].
///
/// # Errors
///
/// Mirrors [`register`].
pub fn register_key<F>(
    keycode: KeyCode,
    modifiers: ModifierFlags,
    callback: F,
) -> Result<Hotkey, HotkeyError>
where
    F: Fn(HotkeyEdge) + Send + Sync + 'static,
{
    register(keycode.raw(), modifiers, callback)
}

/// Register a global hotkey with explicit Carbon options.
///
/// # Errors
///
/// * [`HotkeyError::HandlerInstallFailed`] if the shared keyboard event
///   handler could not be installed.
/// * [`HotkeyError::AlreadyRegistered`] if the same hotkey already exists.
/// * [`HotkeyError::RegisterFailed`] for other Carbon failures.
pub fn register_with_options<F>(
    keycode: u16,
    modifiers: ModifierFlags,
    options: HotKeyOptions,
    callback: F,
) -> Result<Hotkey, HotkeyError>
where
    F: Fn(HotkeyEdge) + Send + Sync + 'static,
{
    install_handler_once()?;

    let id = next_id();
    lock_callback_table().insert(id, Arc::new(callback));

    let mut handle = core::ptr::null_mut();
    let status = unsafe {
        bridge_ffi::carbonhotkey_hotkey_register(
            u32::from(keycode),
            modifiers.bits(),
            VENDOR_SIGNATURE,
            id,
            options.bits(),
            &mut handle,
        )
    };

    if status != 0 {
        lock_callback_table().remove(&id);
        return Err(if status == EVENT_HOTKEY_EXISTS_ERR {
            HotkeyError::AlreadyRegistered
        } else {
            HotkeyError::RegisterFailed(status)
        });
    }

    let handle = NonNull::new(handle).ok_or(HotkeyError::RegisterFailed(-50))?;
    Ok(Hotkey {
        handle: Some(handle),
        id: unsafe { bridge_ffi::carbonhotkey_hotkey_id(handle.as_ptr()) },
    })
}

/// Register a global hotkey with explicit Carbon options and a typed [`KeyCode`].
///
/// # Errors
///
/// Mirrors [`register_with_options`].
pub fn register_key_with_options<F>(
    keycode: KeyCode,
    modifiers: ModifierFlags,
    options: HotKeyOptions,
    callback: F,
) -> Result<Hotkey, HotkeyError>
where
    F: Fn(HotkeyEdge) + Send + Sync + 'static,
{
    register_with_options(keycode.raw(), modifiers, options, callback)
}
