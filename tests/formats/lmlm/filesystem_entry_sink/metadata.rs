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
//   - Metadata test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Metadata test module.
// - Description:
//   - Implements the declared test module responsibility for lmlm.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Metadata test module.

use std::io;
use std::path::Path;

use super::materialize_entries;
use crate::FileEntry;

#[test]
fn metadata_errors_include_the_output_root() {
    let entries = [FileEntry {
        path: "file.bin".to_owned(),
        offset: 0,
        size: 1,
    }];
    let result = materialize_entries(b"x", &entries, Path::new("bad\0root"));

    assert!(
        matches!(
            result,
            Err(error)
                if error.kind() == io::ErrorKind::InvalidInput
                    && error
                        .to_string()
                        .contains("bad")
                    && error
                        .to_string()
                        .contains("root")
                    && !error
                        .to_string()
                        .chars()
                        .any(char::is_control)
        ),
        "metadata errors must identify the inspected output root"
    );
}
