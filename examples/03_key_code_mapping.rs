//! Inspect Carbon key-code mapping helpers.

use carbonhotkey::KeyCode;

fn main() {
    let key = KeyCode::ANSI_T;
    println!("{} => 0x{:02X}", key.name(), key.raw());
    println!(
        "known={} modifier={} documented={}",
        key.is_known(),
        key.is_modifier_key(),
        KeyCode::documented_count()
    );
    assert_eq!(KeyCode::from_name("kvk_escape"), Some(KeyCode::ESCAPE));
}
