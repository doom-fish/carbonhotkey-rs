use carbonhotkey::{
    register_key, register_key_with_options, HotKeyOptions, HotkeyEdge, HotkeyError, KeyCode,
    Modifier,
};

const fn noop(_: HotkeyEdge) {}

fn reserve_available_hotkey(options: HotKeyOptions) -> (carbonhotkey::Hotkey, KeyCode, Modifier) {
    let candidates = [
        (
            KeyCode::F20,
            Modifier::CMD | Modifier::SHIFT | Modifier::OPTION | Modifier::CONTROL,
        ),
        (
            KeyCode::F19,
            Modifier::CMD | Modifier::SHIFT | Modifier::OPTION | Modifier::CONTROL,
        ),
        (
            KeyCode::F18,
            Modifier::CMD | Modifier::SHIFT | Modifier::OPTION | Modifier::CONTROL,
        ),
    ];

    for (key, modifiers) in candidates {
        match register_key_with_options(key, modifiers, options, noop) {
            Ok(hotkey) => return (hotkey, key, modifiers),
            Err(HotkeyError::AlreadyRegistered) => {}
            Err(error) => panic!("unexpected hotkey registration failure: {error}"),
        }
    }

    panic!("could not reserve a hotkey candidate");
}

#[test]
fn register_and_unregister_hotkey() {
    let (hotkey, _, _) = reserve_available_hotkey(HotKeyOptions::NO_OPTIONS);
    assert!(hotkey.id() > 0);
    hotkey.unregister().expect("unregister hotkey");
}

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
fn exclusive_registration_conflicts_when_reused() {
    let (hotkey, key, modifiers) = reserve_available_hotkey(HotKeyOptions::EXCLUSIVE);
    let result = register_key_with_options(key, modifiers, HotKeyOptions::EXCLUSIVE, noop);
    assert!(matches!(result, Err(HotkeyError::AlreadyRegistered)));
    drop(hotkey);
}

#[test]
fn register_key_matches_typed_key_code() {
    let candidates = [
        (
            KeyCode::F17,
            Modifier::CMD | Modifier::SHIFT | Modifier::OPTION | Modifier::CONTROL,
        ),
        (
            KeyCode::F16,
            Modifier::CMD | Modifier::SHIFT | Modifier::OPTION | Modifier::CONTROL,
        ),
        (
            KeyCode::HELP,
            Modifier::CMD | Modifier::SHIFT | Modifier::OPTION | Modifier::CONTROL,
        ),
    ];

    for (key, modifiers) in candidates {
        match register_key(key, modifiers, noop) {
            Ok(hotkey) => {
                assert!(hotkey.id() > 0);
                return;
            }
            Err(HotkeyError::AlreadyRegistered) => {}
            Err(error) => panic!("unexpected typed registration failure: {error}"),
        }
    }

    panic!("could not reserve a typed hotkey candidate");
}
