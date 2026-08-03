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

use std::ffi::OsStr;
use std::path::Path;

use super::destination_path;
use crate::domain::RsdError;

#[test]
fn reserved_output_component_is_rejected() {
    let relative = Path::new("CON.rsd");
    let result =
        destination_path(Path::new("output"), OsStr::new("source"), relative);

    assert!(
        matches!(
            result,
            Err(RsdError::InvalidPath(path)) if path == relative
        ),
        "reserved host aliases must not become WAV destinations"
    );
}
