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

use super::{ByteCursor, read_utf16z};

#[test]
fn failed_byte_read_preserves_bounded_position() -> Result<(), String> {
    let bytes = [10, 20];
    let mut cursor =
        ByteCursor::new(&bytes, 1, 1).map_err(|error| error.to_string())?;
    if cursor.read_u8().is_ok() {
        return Err("byte beyond declared chunk was returned".to_owned());
    }
    if cursor.position != 1 {
        return Err(format!(
            "failed byte read advanced cursor to {}",
            cursor.position
        ));
    }
    Ok(())
}

#[test]
fn rejects_invalid_pstring_utf8() -> Result<(), String> {
    let mut cursor =
        ByteCursor::new(&[1, 0xff], 0, 2).map_err(|error| error.to_string())?;
    if cursor.read_pstring().is_err() {
        Ok(())
    } else {
        Err("invalid PString UTF-8 was accepted".to_owned())
    }
}

#[test]
fn rejects_nonzero_pstring_padding() -> Result<(), String> {
    let mut cursor = ByteCursor::new(&[3, b'A', 0, b'B'], 0, 4)
        .map_err(|error| error.to_string())?;
    if cursor.read_pstring().is_err() {
        Ok(())
    } else {
        Err("invalid PString padding was accepted".to_owned())
    }
}

#[test]
fn rejects_unterminated_utf16_string() -> Result<(), String> {
    if read_utf16z(&[b'A', 0], 0).is_err() {
        Ok(())
    } else {
        Err("unterminated UTF-16 string was accepted".to_owned())
    }
}

#[test]
fn rejects_unpaired_utf16_surrogate() -> Result<(), String> {
    if read_utf16z(&[0, 0xd8, 0, 0], 0).is_err() {
        Ok(())
    } else {
        Err("unpaired UTF-16 surrogate was accepted".to_owned())
    }
}

#[test]
fn rejects_odd_buffer_and_offset() -> Result<(), String> {
    if read_utf16z(&[0, 0, 0xff], 0).is_ok() {
        return Err("odd UTF-16 buffer was accepted".to_owned());
    }
    if read_utf16z(&[0xff, b'A', 0, 0], 1).is_ok() {
        return Err("odd UTF-16 offset was accepted".to_owned());
    }
    Ok(())
}
