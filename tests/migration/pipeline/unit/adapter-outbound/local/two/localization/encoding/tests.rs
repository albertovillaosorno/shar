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

use super::decode_text_source;

#[test]
fn decodes_windows_1252_symbol_byte() -> Result<(), String> {
    match decode_text_source(b"Logitech\xae Force", "test") {
        Ok(text) if text == "Logitech\u{ae} Force" => Ok(()),
        Ok(text) => Err(format!("unexpected decoded text: {text}")),
        Err(error) => {
            Err(format!("era Windows-1252 byte must decode: {error:?}"))
        },
    }
}

#[test]
fn rejects_undefined_windows_1252_byte() -> Result<(), String> {
    if decode_text_source(b"bad\x81byte", "test").is_err() {
        Ok(())
    } else {
        Err("undefined Windows-1252 byte unexpectedly decoded".to_owned())
    }
}

#[test]
fn rejects_unpaired_utf16_surrogate() -> Result<(), String> {
    if decode_text_source(&[0xff, 0xfe, 0x00, 0xd8], "test").is_err() {
        Ok(())
    } else {
        Err("invalid UTF-16 unexpectedly succeeded".to_owned())
    }
}
