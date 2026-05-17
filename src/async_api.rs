//! Async event stream API for hotkey events.
//!
//! This module provides an executor-agnostic, async-friendly wrapper around
//! Carbon's hotkey event system using [`doom_fish_utils::stream::BoundedAsyncStream`].
//!
//! # Overview
//!
//! A [`HotKeyEventStream`] multiplexes all hotkey fires (both press and release)
//! into a single async stream. Each fire delivers a [`StreamHotKeyEvent`] with
//! the hotkey ID, event kind (pressed/released), and other metadata.
//!
//! # Examples
//!
//! ```no_run
//! use carbonhotkey::prelude::*;
//! use carbonhotkey::async_api::HotKeyEventStream;
//!
//! # fn example() {
//! // Subscribe to hotkey events
//! let stream = HotKeyEventStream::new(64);
//!
//! // Register a hotkey
//! let hotkey = register_key(KeyCode::ANSI_A, Modifier::CMD, |_| {})
//!     .expect("register failed");
//!
//! // Spawn an async task to listen for events
//! std::thread::spawn(move || {
//!     pollster::block_on(async {
//!         while let Some(event) = stream.next().await {
//!             if event.hotkey_id == hotkey.id() && event.is_pressed() {
//!                 println!("Hotkey pressed!");
//!             }
//!         }
//!     });
//! });
//! # }
//! ```

use doom_fish_utils::stream::{AsyncStreamSender, BoundedAsyncStream};
use std::ffi::c_void;

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

/// RAII handle to unsubscribe from hotkey events.
struct SubscriptionHandle(*mut c_void);

impl Drop for SubscriptionHandle {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe { unsubscribe_hotkey_stream(self.0) };
        }
    }
}

unsafe impl Send for SubscriptionHandle {}
unsafe impl Sync for SubscriptionHandle {}

/// An async stream of hotkey events.
///
/// When dropped, automatically unsubscribes from the Carbon event system
/// and closes the stream.
pub struct HotKeyEventStream {
    inner: BoundedAsyncStream<StreamHotKeyEvent>,
    _handle: SubscriptionHandle,
}

/// C callback invoked by the Swift bridge on each hotkey event.
extern "C" fn hotkey_event_callback(
    hotkey_id: u32,
    event_kind: u32,
    event_class: u32,
    signature: u32,
    ctx: *mut c_void,
) {
    let sender = unsafe { &*ctx.cast::<AsyncStreamSender<StreamHotKeyEvent>>() };
    sender.push(StreamHotKeyEvent {
        hotkey_id,
        event_kind,
        event_class,
        signature,
    });
}

/// FFI bindings to the Swift async stream bridge.
mod ffi {
    use std::ffi::c_void;

    pub type HotKeyAsyncEventCallback = unsafe extern "C" fn(
        hotkey_id: u32,
        event_kind: u32,
        event_class: u32,
        signature: u32,
        ctx: *mut c_void,
    );

    extern "C" {
        pub fn carbonhotkey_hotkey_stream_subscribe(
            callback: HotKeyAsyncEventCallback,
            ctx: *mut c_void,
        ) -> *mut c_void;

        pub fn carbonhotkey_hotkey_stream_unsubscribe(handle: *mut c_void);
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
/// # async fn example() {
/// let stream = HotKeyEventStream::new(64);
/// while let Some(event) = stream.next().await {
///     println!("Hotkey {} fired: {:?}", event.hotkey_id, event.raw_event_kind());
/// }
/// # }
/// ```
#[must_use]
pub fn subscribe_hotkey_stream(capacity: usize) -> HotKeyEventStream {
    let (stream, sender) = BoundedAsyncStream::new(capacity);
    let sender_ptr = Box::into_raw(Box::new(sender));

    let handle = unsafe {
        ffi::carbonhotkey_hotkey_stream_subscribe(
            hotkey_event_callback as ffi::HotKeyAsyncEventCallback,
            sender_ptr.cast(),
        )
    };

    HotKeyEventStream {
        inner: stream,
        _handle: SubscriptionHandle(handle),
    }
}

unsafe fn unsubscribe_hotkey_stream(handle: *mut c_void) {
    unsafe { ffi::carbonhotkey_hotkey_stream_unsubscribe(handle) };
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
    /// let stream = HotKeyEventStream::new(64);
    /// ```
    #[must_use]
    pub fn new(capacity: usize) -> Self {
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
    /// # async fn example() {
    /// let stream = HotKeyEventStream::new(64);
    /// if let Some(event) = stream.next().await {
    ///     println!("Got event: {:?}", event);
    /// }
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
