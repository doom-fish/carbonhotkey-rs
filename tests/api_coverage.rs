//! API-surface coverage harness for `carbonhotkey`.
//!
//! Carbon's `RegisterEventHotKey` is a tiny subset of the
//! `CarbonEventsCore.h` header (which itself is huge — most of it is
//! Carbon's broader event system that we don't touch). We only assert
//! that every C symbol our `extern "C"` block declares is actually
//! present in the SDK header — the inverse direction (Apple symbols ⇒
//! ours) is intentionally narrow because Carbon ships hundreds of
//! event-system primitives and we wrap fewer than ten.

#![allow(clippy::cast_precision_loss, clippy::iter_on_single_items)]

use std::collections::BTreeSet;
use std::path::PathBuf;
use std::process::Command;

fn sdk_root() -> PathBuf {
    let out = Command::new("xcrun")
        .args(["--sdk", "macosx", "--show-sdk-path"])
        .output()
        .expect("xcrun");
    assert!(out.status.success());
    PathBuf::from(String::from_utf8(out.stdout).unwrap().trim().to_string())
}

fn read_carbon_events_header() -> String {
    let sdk = sdk_root();
    // Carbon's hotkey API + constants are spread across two headers in
    // HIToolbox: `CarbonEventsCore.h` (functions) and `CarbonEvents.h`
    // (event-class constants like `kEventClassKeyboard`). Read both.
    let mut combined = String::new();
    for name in ["CarbonEventsCore.h", "CarbonEvents.h", "MacApplication.h"] {
        let path = sdk.join(format!(
            "System/Library/Frameworks/Carbon.framework/Frameworks/HIToolbox.framework/Headers/{name}"
        ));
        if let Ok(b) = std::fs::read(&path) {
            combined.push_str(&String::from_utf8_lossy(&b));
            combined.push('\n');
        }
    }
    combined
}

fn extract_rust_externs() -> BTreeSet<String> {
    let p = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/ffi/mod.rs");
    let s = std::fs::read_to_string(&p).unwrap();
    let re = regex_lite::Regex::new(r"pub\s+fn\s+([A-Za-z0-9_]+)\s*\(").unwrap();
    re.captures_iter(&s).map(|c| c[1].to_string()).collect()
}

#[test]
fn ffi_symbols_exist_in_carbon_header() {
    let header = read_carbon_events_header();
    let ours = extract_rust_externs();
    let mut missing: BTreeSet<&String> = BTreeSet::new();
    for sym in &ours {
        // Each declared C function must appear textually in the header.
        // Carbon doesn't expose them via a single regex pattern (no
        // common macro prefix), so we just check identifier presence.
        if !header.contains(sym.as_str()) {
            missing.insert(sym);
        }
    }
    println!(
        "\n=== carbonhotkey FFI vs CarbonEventsCore.h ===\n  declared={}, missing_in_header={}",
        ours.len(),
        missing.len(),
    );
    if !missing.is_empty() {
        for m in &missing {
            println!("  - {m}");
        }
    }
    assert!(
        missing.is_empty(),
        "FFI symbols missing from Carbon header: {missing:?}"
    );
}

#[test]
fn known_constants_present() {
    let header = read_carbon_events_header();
    for sym in &[
        "kEventClassKeyboard",
        "kEventHotKeyPressed",
        "kEventHotKeyReleased",
        "kEventParamDirectObject",
        "typeEventHotKeyID",
    ] {
        assert!(
            header.contains(sym),
            "Carbon header missing constant {sym:?}"
        );
    }
}
