// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT
//
// Boundary-Contract:
// - Owns:
//   - Tests unit tests.
// - Must-Not:
//   - Own production behavior or broaden the tested API surface.
// - Allows:
//   - Private test fixtures and assertions for the owning source module.
// - Split-When:
//   - Split when an independent fixture family gains separate ownership.
// - Merge-When:
//   - Merge when another test module owns the identical evidence.
// - Summary:
//   - Tests unit tests.
// - Description:
//   - Preserves unit-test access through a test-only path module.
// - Usage:
//   - Included only by the owning source module under cfg(test).
// - Defaults:
//   - Test setup and assertions fail explicitly.
//

//! Tests unit tests.

use std::time::Duration;

use super::{eta_duration, format_duration, progress_line, shorten_item};

#[test]
fn duration_uses_stable_clock_text() {
    assert_eq!(format_duration(0), "00:00:00");
    assert_eq!(format_duration(61), "00:01:01");
    assert_eq!(format_duration(3_661), "01:01:01");
}

#[test]
fn eta_uses_observed_item_rate() {
    assert_eq!(
        eta_duration(Duration::from_secs(10), 50, 100,),
        Some(Duration::from_secs(10))
    );
    assert_eq!(eta_duration(Duration::from_secs(10), 0, 100,), None);
    assert_eq!(eta_duration(Duration::from_secs(10), 100, 100,), None);
}

#[test]
fn detailed_line_includes_percent_eta_and_current_item() {
    let line = progress_line(
        "p3d",
        50,
        Some(100),
        Duration::from_secs(10),
        "art/sample.p3d",
    );
    assert_eq!(
        line,
        concat!(
            "[p3d] 50.0% (50/100) elapsed 00:00:10 ",
            "eta 00:00:10 current=art/sample.p3d"
        )
    );
}

#[test]
fn unknown_total_does_not_fabricate_percent_or_eta() {
    assert_eq!(
        progress_line("rcf", 4, None, Duration::from_secs(2), "entry.bin",),
        "[rcf] 4 items elapsed 00:00:02 current=entry.bin"
    );
}

#[test]
fn current_item_is_unicode_safe_and_bounded() {
    let input = "é".repeat(120);
    let output = shorten_item(&input);
    assert!(output.ends_with("..."));
    assert_eq!(output.chars().count(), 99);
}

#[test]
fn current_item_escapes_terminal_control_characters() {
    assert_eq!(
        shorten_item("entry\nnext\u{1b}[31m"),
        "entry\\u{a}next\\u{1b}[31m"
    );
}

#[test]
fn current_item_does_not_split_control_escapes() {
    let mut input = "a".repeat(95);
    input.push(char::from(10));
    let mut expected = "a".repeat(95);
    expected.push_str("...");
    assert_eq!(shorten_item(&input), expected);
}
