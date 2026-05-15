//! Global keyboard hotkey registration via Carbon's `RegisterEventHotKey`.

use core::ffi::c_void;
use core::ptr;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};

use crate::error::HotkeyError;
use crate::ffi;

bitflags::bitflags! {
    /// Modifier mask. Carbon-style — these constants match Apple's
    /// `cmdKey`, `shiftKey`, `optionKey`, `controlKey`, etc.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
    pub struct Modifier: u32 {
        const CMD     = ffi::cmdKey;
        const SHIFT   = ffi::shiftKey;
        const OPTION  = ffi::optionKey;
        const CONTROL = ffi::controlKey;
        const CAPS    = ffi::alphaLock;
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
    raw: ffi::EventHotKeyRef,
    id: u32,
}

unsafe impl Send for Hotkey {}
unsafe impl Sync for Hotkey {}

impl Drop for Hotkey {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe { ffi::UnregisterEventHotKey(self.raw) };
            self.raw = ptr::null_mut();
        }
        callback_table().lock().unwrap().remove(&self.id);
    }
}

type Callback = Box<dyn Fn(HotkeyEdge) + Send + Sync + 'static>;

fn callback_table() -> &'static Mutex<HashMap<u32, Arc<Callback>>> {
    static T: OnceLock<Mutex<HashMap<u32, Arc<Callback>>>> = OnceLock::new();
    T.get_or_init(|| Mutex::new(HashMap::new()))
}

fn next_id() -> u32 {
    use std::sync::atomic::{AtomicU32, Ordering};
    static NEXT: AtomicU32 = AtomicU32::new(1);
    NEXT.fetch_add(1, Ordering::SeqCst)
}

unsafe extern "C" fn handler(
    _call_ref: ffi::EventHandlerCallRef,
    event: ffi::EventRef,
    _user: *mut c_void,
) -> ffi::OSStatus {
    let kind = unsafe { ffi::GetEventKind(event) };
    let edge = if kind == ffi::kEventHotKeyPressed {
        HotkeyEdge::Pressed
    } else if kind == ffi::kEventHotKeyReleased {
        HotkeyEdge::Released
    } else {
        return 0;
    };
    let mut hkid = ffi::EventHotKeyID {
        signature: 0,
        id: 0,
    };
    let mut actual_size: ffi::ByteCount = 0;
    let status = unsafe {
        ffi::GetEventParameter(
            event,
            ffi::kEventParamDirectObject,
            ffi::typeEventHotKeyID,
            ptr::null_mut(),
            core::mem::size_of::<ffi::EventHotKeyID>() as ffi::ByteCount,
            &mut actual_size,
            ptr::from_mut(&mut hkid).cast(),
        )
    };
    if status != 0 {
        return status;
    }
    let cb = {
        let table = callback_table().lock().unwrap();
        table.get(&hkid.id).cloned()
    };
    if let Some(cb) = cb {
        cb(edge);
    }
    0
}

fn install_handler_once() -> Result<(), HotkeyError> {
    static INSTALLED: OnceLock<Result<(), i32>> = OnceLock::new();
    let result = INSTALLED.get_or_init(|| {
        let types = [
            ffi::EventTypeSpec {
                eventClass: ffi::kEventClassKeyboard,
                eventKind: ffi::kEventHotKeyPressed,
            },
            ffi::EventTypeSpec {
                eventClass: ffi::kEventClassKeyboard,
                eventKind: ffi::kEventHotKeyReleased,
            },
        ];
        let mut handler_ref: ffi::EventHandlerRef = ptr::null_mut();
        let status = unsafe {
            ffi::InstallEventHandler(
                ffi::GetApplicationEventTarget(),
                handler,
                types.len() as ffi::ItemCount,
                types.as_ptr(),
                ptr::null_mut(),
                &mut handler_ref,
            )
        };
        if status == 0 {
            Ok(())
        } else {
            Err(status)
        }
    });
    match result {
        Ok(()) => Ok(()),
        Err(s) => Err(HotkeyError::HandlerInstallFailed(*s)),
    }
}

/// Register a global hotkey. The callback fires whenever the user presses
/// the combination, even if your app isn't focused.
///
/// `keycode` uses the same virtual-keycode space as `CGEvent` (see the
/// `cgevents` crate's `Keycode` module — `Keycode::A`, `Keycode::F1`, etc.).
///
/// The returned [`Hotkey`] guard unregisters the hotkey when dropped.
///
/// # Errors
///
/// * [`HotkeyError::HandlerInstallFailed`] — installing the global event
///   handler failed (only happens once per process).
/// * [`HotkeyError::AlreadyRegistered`] — same hotkey is already
///   registered by this process or by another with `kEventHotKeyExclusive`.
/// * [`HotkeyError::RegisterFailed`] — generic Carbon failure.
///
/// # Panics
///
/// Panics if the internal callback table mutex is poisoned (only possible
/// if a previous callback panicked while holding it).
///
/// # Examples
///
/// ```rust,no_run
/// use carbonhotkey::{register, Modifier, HotkeyEdge};
///
/// let _hk = register(0x00 /* A */, Modifier::CMD | Modifier::SHIFT, |edge| {
///     match edge {
///         HotkeyEdge::Pressed  => println!("pressed!"),
///         HotkeyEdge::Released => println!("released!"),
///     }
/// })?;
///
/// // Run your app's event loop here so the callback fires.
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn register<F>(keycode: u16, modifiers: Modifier, callback: F) -> Result<Hotkey, HotkeyError>
where
    F: Fn(HotkeyEdge) + Send + Sync + 'static,
{
    install_handler_once()?;

    let id = next_id();
    callback_table()
        .lock()
        .unwrap()
        .insert(id, Arc::new(Box::new(callback)));

    let hkid = ffi::EventHotKeyID {
        signature: 0x646f_6f6d, // 'doom' - doom-fish vendor signature
        id,
    };
    let mut raw: ffi::EventHotKeyRef = ptr::null_mut();
    let status = unsafe {
        ffi::RegisterEventHotKey(
            u32::from(keycode),
            modifiers.bits(),
            hkid,
            ffi::GetApplicationEventTarget(),
            0,
            &mut raw,
        )
    };
    if status != 0 {
        callback_table().lock().unwrap().remove(&id);
        return Err(if status == -9878 {
            HotkeyError::AlreadyRegistered
        } else {
            HotkeyError::RegisterFailed(status)
        });
    }
    Ok(Hotkey { raw, id })
}

/// Run Carbon's application event loop forever (or until
/// [`quit_event_loop`] is called from another thread).
///
/// Hotkey callbacks fire from this loop's thread.
pub fn run_event_loop() {
    unsafe { ffi::RunApplicationEventLoop() };
}

/// Stop a running event loop. Call from another thread.
pub fn quit_event_loop() {
    unsafe { ffi::QuitApplicationEventLoop() };
}
