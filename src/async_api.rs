//! Async event stream API for hotkey events.
//!
//! This module provides an executor-agnostic, async-friendly wrapper around
//! Carbon's hotkey event system using [`doom_fish_utils::stream::BoundedAsyncStream`].
//!
//! # Overview
//!
//! A [`HotKeyEventStream`] multiplexes the fires (both press and release) of
//! every hotkey registered through this crate into a single async stream.
//! Each fire delivers a [`StreamHotKeyEvent`] with the hotkey ID, event kind
//! (pressed/released), and other metadata. Carbon delivers hotkey events only
//! while the main thread runs its event loop, so the main thread must run
//! [`run_event_loop`](crate::run_event_loop) (or an app run loop) while the
//! stream is consumed elsewhere.
//!
//! # Examples
//!
//! ```no_run
//! use carbonhotkey::prelude::*;
//! use carbonhotkey::async_api::HotKeyEventStream;
//!
//! # fn example() -> Result<(), HotkeyError> {
//! let hotkey = register_key(KeyCode::ANSI_A, Modifier::CMD | Modifier::OPTION, |_| {})?;
//! let stream = HotKeyEventStream::new(64)?;
//! let id = hotkey.id();
//!
//! std::thread::spawn(move || {
//!     pollster::block_on(async {
//!         while let Some(event) = stream.next().await {
//!             if event.hotkey_id == id && event.is_pressed() {
//!                 println!("Hotkey pressed!");
//!             }
//!         }
//!     });
//! });
//!
//! run_event_loop()?;
//! # Ok(())
//! # }
//! ```

use doom_fish_utils::stream::BoundedAsyncStream;

use crate::error::HotkeyError;

/// A hotkey event item in the async stream.
///
/// This is a snapshot of the hotkey event including the hotkey ID and event kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StreamHotKeyEvent {
    pub hotkey_id: u32,
    pub event_kind: u32,
    pub event_class: u32,
    pub signature: u32,
}

impl StreamHotKeyEvent {
    /// The hotkey ID that fired.
    #[must_use]
    pub const fn hotkey_id(self) -> u32 {
        self.hotkey_id
    }

    /// The raw event kind (5 for pressed, 6 for released).
    #[must_use]
    pub const fn raw_event_kind(self) -> u32 {
        self.event_kind
    }

    /// Whether this event is a hotkey press.
    #[must_use]
    pub const fn is_pressed(self) -> bool {
        self.event_kind == 5
    }

    /// Whether this event is a hotkey release.
    #[must_use]
    pub const fn is_released(self) -> bool {
        self.event_kind == 6
    }

    /// The event class (always 0x6b657962 for keyboard).
    #[must_use]
    pub const fn event_class(self) -> u32 {
        self.event_class
    }

    /// The hotkey signature.
    #[must_use]
    pub const fn signature(self) -> u32 {
        self.signature
    }
}

/// An async stream of hotkey events.
///
/// When dropped, automatically unsubscribes from the Carbon event system
/// and closes the stream.
pub struct HotKeyEventStream {
    inner: BoundedAsyncStream<StreamHotKeyEvent>,
    subscription: u64,
}

impl Drop for HotKeyEventStream {
    fn drop(&mut self) {
        crate::hotkey::unsubscribe_stream(self.subscription);
    }
}

/// Subscribe to hotkey events as an async stream.
///
/// Installs a single application event handler that routes all hotkey fires
/// (both press and release events) to the stream.
///
/// # Arguments
///
/// * `capacity` - Maximum number of events to buffer. When full, the oldest
///   event is dropped to make room for new ones.
///
/// # Returns
///
/// A [`HotKeyEventStream`] that yields [`StreamHotKeyEvent`] items as hotkeys fire.
/// The stream closes when dropped.
///
/// # Example
///
/// ```no_run
/// use carbonhotkey::async_api::HotKeyEventStream;
///
/// # async fn example() -> Result<(), carbonhotkey::HotkeyError> {
/// let stream = HotKeyEventStream::new(64)?;
/// while let Some(event) = stream.next().await {
///     println!("Hotkey {} fired: {:?}", event.hotkey_id, event.raw_event_kind());
/// }
/// # Ok(())
/// # }
/// ```
#[allow(clippy::missing_errors_doc)]
pub fn subscribe_hotkey_stream(capacity: usize) -> Result<HotKeyEventStream, HotkeyError> {
    if capacity == 0 {
        return Err(HotkeyError::InvalidArgument(
            "hotkey stream capacity must be greater than zero".into(),
        ));
    }
    if unsafe { crate::bridge_ffi::pthread_main_np() } != 0 {
        crate::hotkey::ensure_dispatcher_installed()?;
    }
    let (inner, sender) = BoundedAsyncStream::new(capacity);
    let subscription = crate::hotkey::subscribe_stream(sender);
    Ok(HotKeyEventStream {
        inner,
        subscription,
    })
}

impl HotKeyEventStream {
    /// Create a new hotkey event stream with the given capacity.
    ///
    /// # Arguments
    ///
    /// * `capacity` - Maximum number of events to buffer.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use carbonhotkey::async_api::HotKeyEventStream;
    ///
    /// let stream = HotKeyEventStream::new(64)?;
    /// # Ok::<(), carbonhotkey::HotkeyError>(())
    /// ```
    #[allow(clippy::missing_errors_doc)]
    pub fn new(capacity: usize) -> Result<Self, HotkeyError> {
        subscribe_hotkey_stream(capacity)
    }

    /// Wait for the next hotkey event.
    ///
    /// Returns `None` when the stream is closed.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use carbonhotkey::async_api::HotKeyEventStream;
    ///
    /// # async fn example() -> Result<(), carbonhotkey::HotkeyError> {
    /// let stream = HotKeyEventStream::new(64)?;
    /// if let Some(event) = stream.next().await {
    ///     println!("Got event: {:?}", event);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    #[allow(clippy::missing_const_for_fn)]
    pub fn next(&self) -> doom_fish_utils::stream::NextItem<'_, StreamHotKeyEvent> {
        self.inner.next()
    }

    /// Try to get the next event without waiting.
    ///
    /// Returns `None` if no event is currently buffered.
    #[must_use]
    pub fn try_next(&self) -> Option<StreamHotKeyEvent> {
        self.inner.try_next()
    }

    /// Query the current number of buffered events.
    #[must_use]
    pub fn buffered_count(&self) -> usize {
        self.inner.buffered_count()
    }
}
