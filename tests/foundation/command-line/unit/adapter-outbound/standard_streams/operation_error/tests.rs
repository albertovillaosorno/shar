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

use std::error::Error as _;
use std::io;

use super::{StreamOperation, contextualize};

#[test]
fn provider_error_escapes_source_control_characters() {
    let error = contextualize(
        StreamOperation::Flush { accepted_bytes: 4 },
        io::Error::other("flush\ninjected"),
    );

    let rendered = error.to_string();

    assert!(
        !rendered.chars().any(char::is_control),
        "diagnostic contains a control character: {rendered:?}"
    );
    assert!(rendered.contains(r"flush\ninjected"));
    assert!(error.source().is_some());
}

#[test]
fn one_byte_progress_uses_the_singular_unit() {
    let write = StreamOperation::Write {
        accepted_bytes: 0,
        total_bytes: 1,
    };
    let flush = StreamOperation::Flush { accepted_bytes: 1 };

    assert_eq!(write.to_string(), "write standard stream after 0 of 1 byte");
    assert_eq!(
        flush.to_string(),
        "flush standard stream after accepting 1 byte"
    );
}
