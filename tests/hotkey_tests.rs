use std::thread;

use carbonhotkey::{
    register, register_key, register_key_with_options, register_with_options, HotKeyOptions,
    HotkeyEdge, HotkeyError, KeyCode, Modifier,
};

const fn noop(_: HotkeyEdge) {}

#[test]
fn no_options_constant_matches_default() {
    assert!(HotKeyOptions::NO_OPTIONS.is_empty());
    assert_eq!(HotKeyOptions::NO_OPTIONS, HotKeyOptions::default());

    #[cfg(feature = "raw-ffi")]
    assert_eq!(
        carbonhotkey::ffi::kEventHotKeyNoOptions,
        HotKeyOptions::NO_OPTIONS.bits()
    );
}

#[test]
fn registering_off_the_main_thread_is_rejected() {
    let modifiers = Modifier::CMD | Modifier::SHIFT | Modifier::OPTION | Modifier::CONTROL;
    let outcomes = thread::spawn(move || {
        [
            register(KeyCode::F20.raw(), modifiers, noop).map(|_| ()),
            register_key(KeyCode::F20, modifiers, noop).map(|_| ()),
            register_with_options(
                KeyCode::F20.raw(),
                modifiers,
                HotKeyOptions::EXCLUSIVE,
                noop,
            )
            .map(|_| ()),
            register_key_with_options(KeyCode::F20, modifiers, HotKeyOptions::NO_OPTIONS, noop)
                .map(|_| ()),
        ]
    })
    .join()
    .expect("registration thread");
    for outcome in outcomes {
        assert_eq!(outcome, Err(HotkeyError::NotMainThread));
    }
}
