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

use super::parse_custom_text_bytes;

#[test]
fn strips_utf8_bom_from_first_line() -> Result<(), String> {
    let entries =
        parse_custom_text_bytes("\u{feff}FIRST=one".as_bytes(), "test")
            .map_err(|error| error.to_string())?;
    if entries.first().is_some_and(|entry| entry.key == "FIRST") {
        Ok(())
    } else {
        Err(format!("unexpected BOM-prefixed entries: {entries:?}"))
    }
}

#[test]
fn preserves_bom_after_first_line() -> Result<(), String> {
    let entries = parse_custom_text_bytes(
        "FIRST=one\n\u{feff}SECOND=two".as_bytes(),
        "test",
    )
    .map_err(|error| error.to_string())?;
    let second = entries
        .get(1)
        .ok_or_else(|| "missing second custom-text entry".to_owned())?;
    if second.key == "\u{feff}SECOND" {
        Ok(())
    } else {
        Err(format!("second-line BOM was rewritten as {:?}", second.key,))
    }
}

#[test]
fn rejects_malformed_record() -> Result<(), String> {
    if parse_custom_text_bytes(b"BROKEN_LINE", "test").is_err() {
        Ok(())
    } else {
        Err("malformed custom-text record was accepted".to_owned())
    }
}

#[test]
fn rejects_empty_key() -> Result<(), String> {
    if parse_custom_text_bytes(b" =value", "test").is_err() {
        Ok(())
    } else {
        Err("empty custom-text key was accepted".to_owned())
    }
}

#[test]
fn rejects_duplicate_key() -> Result<(), String> {
    if parse_custom_text_bytes(b"KEY=first\nKEY=second", "test").is_err() {
        Ok(())
    } else {
        Err("duplicate custom-text key was accepted".to_owned())
    }
}

#[test]
fn parses_utf16_custom_text() -> Result<(), String> {
    let bytes = [0xff, 0xfe, b'A', 0, b'=', 0, b'B', 0];
    let entries = parse_custom_text_bytes(&bytes, "test")
        .map_err(|error| error.to_string())?;
    if entries.len() == 1
        && entries.first().is_some_and(|entry| entry.key == "A")
    {
        Ok(())
    } else {
        Err(format!("unexpected custom-text entries: {entries:?}"))
    }
}
