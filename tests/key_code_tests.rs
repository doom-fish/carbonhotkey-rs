use carbonhotkey::KeyCode;

#[test]
fn key_code_count_matches_bridge() {
    assert_eq!(KeyCode::ALL.len(), KeyCode::documented_count());
}

#[test]
fn key_code_parsing_and_modifier_detection_work() {
    assert_eq!(KeyCode::from_name("kvk_ansi_t"), Some(KeyCode::ANSI_T));
    assert_eq!(KeyCode::from_name("escape"), Some(KeyCode::ESCAPE));
    assert!(KeyCode::COMMAND.is_modifier_key());
    assert!(KeyCode::ANSI_T.is_known());
    assert!(!KeyCode::ANSI_T.is_modifier_key());
}
