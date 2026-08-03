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

use std::collections::BTreeSet;
#[cfg(windows)]
use std::ffi::OsString;
#[cfg(windows)]
use std::os::windows::ffi::OsStringExt as _;
#[cfg(windows)]
use std::path::PathBuf;

use super::register_portable_destination;

#[cfg(windows)]
#[test]
fn collision_error_preserves_unpaired_utf16_destination_unit()
-> Result<(), String> {
    let destination = PathBuf::from(OsString::from_wide(&[
        u16::from(b'a'),
        0xd800,
        u16::from(b'b'),
    ]));
    let mut files = BTreeSet::new();
    let mut directories = BTreeSet::new();
    register_portable_destination(
        "same.bin",
        &destination,
        &mut files,
        &mut directories,
    )
    .map_err(|error| error.to_string())?;

    let result = register_portable_destination(
        "same.bin",
        &destination,
        &mut files,
        &mut directories,
    );
    let Err(error) = result else {
        return Err("duplicate destination unexpectedly passed".to_owned());
    };
    let rendered = error.to_string();
    if !rendered.contains(r"a\u{D800}b") {
        return Err(format!(
            "diagnostic lost the native path unit: {rendered:?}"
        ));
    }
    if rendered.contains(r"\u{fffd}") {
        return Err(format!("diagnostic used lossy replacement: {rendered:?}"));
    }
    Ok(())
}
