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
//   - Binary artifact test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Binary artifact test module.
// - Description:
//   - Implements the declared test module responsibility for fbx.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Binary artifact test module.

use std::fs;
use std::path::Path;

/// Read two generated artifacts and remove both temporary files.
#[must_use]
pub(super) fn read_binary_pair(
    first_path: &Path,
    second_path: &Path,
    label: &str,
) -> Option<(Vec<u8>, Vec<u8>)> {
    let first_result = fs::read(first_path);
    assert!(
        first_result.is_ok(),
        "first {label} should be readable: {first_result:?}"
    );
    let first = first_result.ok()?;
    let second_result = fs::read(second_path);
    assert!(
        second_result.is_ok(),
        "second {label} should be readable: {second_result:?}"
    );
    let second = second_result.ok()?;
    assert!(
        fs::remove_file(first_path).is_ok(),
        "first {label} temporary file should be removable"
    );
    assert!(
        fs::remove_file(second_path).is_ok(),
        "second {label} temporary file should be removable"
    );
    Some((first, second))
}
