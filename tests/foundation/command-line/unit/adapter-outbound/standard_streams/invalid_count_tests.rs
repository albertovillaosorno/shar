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
//   - Invalid count tests unit tests.
// - Must-Not:
//   - Own production behavior or broaden the tested API surface.
// - Allows:
//   - Private test fixtures and assertions for the owning source module.
// - Split-When:
//   - Split when an independent fixture family gains separate ownership.
// - Merge-When:
//   - Merge when another test module owns the identical evidence.
// - Summary:
//   - Invalid count tests unit tests.
// - Description:
//   - Preserves unit-test access through a test-only path module.
// - Usage:
//   - Included only by the owning source module under cfg(test).
// - Defaults:
//   - Test setup and assertions fail explicitly.
//

//! Invalid count tests unit tests.

use std::io::{self, Write};

/// Increments one bounded fixture byte count.
const fn increment(count: usize) -> usize {
    count.saturating_add(1)
}

/// Writes the shared five-byte fixture through the adapter helper.
fn write_alpha(writer: &mut impl Write) -> io::Result<()> {
    super::write_complete(writer, "alpha")
}

/// Fails when an empty-write provider is invoked unexpectedly.
fn reject_text(_text: &str) -> io::Result<()> {
    Err(io::Error::new(
        io::ErrorKind::PermissionDenied,
        "provider must not be acquired",
    ))
}

#[test]
fn empty_text_skips_provider_acquisition() {
    let result = super::write_if_non_empty("", reject_text);

    assert!(result.is_ok());
}

struct OverreportingWriter;

impl Write for OverreportingWriter {
    fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        Ok(increment(buffer.len()))
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[test]
fn invalid_writer_count_reports_actual_and_allowed_bytes() {
    let mut writer = OverreportingWriter;

    let result = write_alpha(&mut writer);

    assert!(result.is_err());
    let Some(error) = result.err() else {
        return;
    };
    assert_eq!(error.kind(), io::ErrorKind::InvalidData);
    assert_eq!(
        error.to_string(),
        concat!(
            "failed to write standard stream after 0 of 5 bytes: ",
            "writer reported 6 bytes for a 5-byte buffer"
        )
    );
}
