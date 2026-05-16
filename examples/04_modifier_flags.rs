//! Inspect Carbon modifier-flag helpers.

use carbonhotkey::{Modifier, ModifierFlags};

fn main() {
    let flags = Modifier::CMD | Modifier::SHIFT | ModifierFlags::RIGHT_OPTION;
    println!("flags=0x{:04X}", flags.bits());
    println!("supported=0x{:04X}", ModifierFlags::supported_mask().bits());
    println!(
        "right-side=0x{:04X}",
        ModifierFlags::right_side_mask().bits()
    );
    assert!(flags.contains_right_side());
}
