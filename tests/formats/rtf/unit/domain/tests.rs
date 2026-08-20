// Copyright:
//   - Copyright © 2026 Alberto Villa Osorno.
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

use super::rtf_to_markdown;

#[test]
fn unicode_escape_honors_zero_fallback_count() {
    let markdown = rtf_to_markdown(br"{\rtf1\ansi\uc0\u233x}");

    assert_eq!(markdown, "éx\n");
}

#[test]
fn empty_document_stays_empty() {
    assert_eq!(rtf_to_markdown(b""), "");
}

#[test]
fn unicode_control_without_parameter_preserves_text() {
    let markdown = rtf_to_markdown(br"{\rtf1\ansi\u?x}");

    assert_eq!(markdown, "?x\n");
}

#[test]
fn unicode_escape_decodes_surrogate_pairs() {
    let markdown = rtf_to_markdown(br"{\rtf1\ansi\uc1\u-10179?\u-8704?}");

    assert_eq!(markdown, "😀\n");
}

#[test]
fn unicode_escape_skips_control_word_fallback() {
    let markdown = rtf_to_markdown(br"{\rtf1\ansi\uc1\u233\emdash X}");

    assert_eq!(markdown, "éX\n");
}

#[test]
fn binary_control_skips_declared_bytes() {
    let markdown = rtf_to_markdown(br"{\rtf1\ansi A\bin3 xyzB}");

    assert_eq!(markdown, "AB\n");
}

#[test]
fn standard_character_controls_are_preserved() {
    let input = concat!(
        r"{\rtf1 A\emdash B\endash C",
        r"\bullet D\lquote E\rquote F",
        r"\ldblquote G\rdblquote H}",
    );
    let markdown = rtf_to_markdown(input.as_bytes());

    assert_eq!(markdown, "A—B–C•D‘E’F“G”H\n");
}

#[test]
fn optional_hyphen_does_not_become_visible_text() {
    let markdown = rtf_to_markdown(br"{\rtf1 co\-operate}");

    assert_eq!(markdown, "cooperate\n");
}

#[test]
fn nonbreaking_space_remains_nonbreaking() {
    let markdown = rtf_to_markdown(br"{\rtf1 A\~B}");

    assert_eq!(markdown, "A\u{00a0}B\n");
}

#[test]
fn truncated_hex_escape_does_not_leak_nibble() {
    let markdown = rtf_to_markdown(br"A\'4");

    assert_eq!(markdown, "A\n");
}

#[test]
fn page_break_separates_text_blocks() {
    let markdown = rtf_to_markdown(br"{\rtf1 first\page second}");

    assert_eq!(markdown, "first\nsecond\n");
}

#[test]
fn table_controls_separate_cells_and_rows() {
    let markdown = rtf_to_markdown(br"{\rtf1 one\cell two\row three}");

    assert_eq!(markdown, "one two\nthree\n");
}

#[test]
fn metadata_destinations_do_not_leak_content() {
    let input = concat!(
        r"{\rtf1 visible",
        r"{\filetbl file-data}",
        r"{\revtbl revision-data}",
        r"{\fldinst instruction-data}",
        r"{\datafield field-data}",
        r"end}",
    );
    let markdown = rtf_to_markdown(input.as_bytes());

    assert_eq!(markdown, "visibleend\n");
}

#[test]
fn raw_control_bytes_do_not_leak_into_text() {
    let input = [
        b'{', b'\\', b'r', b't', b'f', b'1', b' ', b'A', 0_u8, 1_u8, 0x7f_u8,
        b'B', b'}',
    ];
    let markdown = rtf_to_markdown(&input);

    assert_eq!(
        markdown,
        "AB
"
    );
}

#[test]
fn semantic_space_controls_preserve_width() {
    let markdown = rtf_to_markdown(br"{\rtf1 A\emspace B\enspace C\qmspace D}");

    assert_eq!(markdown, "A\u{2003}B\u{2002}C\u{2005}D\n");
}
