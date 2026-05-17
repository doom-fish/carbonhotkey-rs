#![cfg(feature = "async")]

use carbonhotkey::async_api::{HotKeyEventStream, StreamHotKeyEvent};

#[test]
fn test_stream_creation() {
    let stream = HotKeyEventStream::new(32);
    assert_eq!(stream.buffered_count(), 0);
}

#[test]
fn test_stream_try_next_empty() {
    let stream = HotKeyEventStream::new(32);
    assert_eq!(stream.try_next(), None);
}

#[test]
fn test_stream_hotkey_event_pressed() {
    // The event is pressed
    let event = StreamHotKeyEvent {
        hotkey_id: 42,
        event_kind: 5,
        event_class: 0x6b65_7962,
        signature: 1234,
    };

    assert_eq!(event.hotkey_id(), 42);
    assert_eq!(event.raw_event_kind(), 5);
    assert!(event.is_pressed());
    assert!(!event.is_released());
    assert_eq!(event.event_class(), 0x6b65_7962);
    assert_eq!(event.signature(), 1234);
}

#[test]
fn test_stream_hotkey_event_released() {
    // The event is released
    let event = StreamHotKeyEvent {
        hotkey_id: 42,
        event_kind: 6,
        event_class: 0x6b65_7962,
        signature: 1234,
    };

    assert_eq!(event.hotkey_id(), 42);
    assert_eq!(event.raw_event_kind(), 6);
    assert!(!event.is_pressed());
    assert!(event.is_released());
}

#[test]
fn test_stream_event_debug() {
    let event = StreamHotKeyEvent {
        hotkey_id: 42,
        event_kind: 5,
        event_class: 0x6b65_7962,
        signature: 1234,
    };

    let debug_str = format!("{event:?}");
    assert!(debug_str.contains("StreamHotKeyEvent"));
}

#[test]
fn test_stream_event_copy() {
    // Verify that Copy trait works
    let event1 = StreamHotKeyEvent {
        hotkey_id: 42,
        event_kind: 5,
        event_class: 0x6b65_7962,
        signature: 1234,
    };

    let event2 = event1;
    assert_eq!(event1, event2);
}

