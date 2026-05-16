use carbonhotkey::{Modifier, ModifierFlags};

#[test]
fn modifier_masks_match_bridge() {
    let expected_supported = Modifier::CMD
        | Modifier::SHIFT
        | Modifier::CAPS
        | Modifier::OPTION
        | Modifier::CONTROL
        | ModifierFlags::RIGHT_SHIFT
        | ModifierFlags::RIGHT_OPTION
        | ModifierFlags::RIGHT_CONTROL;

    let expected_right =
        ModifierFlags::RIGHT_SHIFT | ModifierFlags::RIGHT_OPTION | ModifierFlags::RIGHT_CONTROL;

    assert_eq!(ModifierFlags::supported_mask(), expected_supported);
    assert_eq!(ModifierFlags::right_side_mask(), expected_right);
    assert!((Modifier::CMD | ModifierFlags::RIGHT_OPTION).contains_right_side());
    assert!(!(Modifier::CMD | Modifier::SHIFT).contains_right_side());
}
