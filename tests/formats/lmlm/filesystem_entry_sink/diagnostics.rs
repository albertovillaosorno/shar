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
//   - Diagnostics test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Diagnostics test module.
// - Description:
//   - Implements the declared test module responsibility for lmlm.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Diagnostics test module.

use std::path::Path;

use super::materialize_entries;
use crate::FileEntry;

#[test]
fn materialization_errors_escape_untrusted_paths() {
    let entries = [FileEntry {
        path: "unsafe\nname.bin".to_owned(),
        offset: 0,
        size: 1,
    }];
    let result = materialize_entries(b"x", &entries, Path::new("output"));

    assert!(
        matches!(
            result,
            Err(error)
                if !error
                    .to_string()
                    .chars()
                    .any(char::is_control)
        ),
        "materialization errors must escape untrusted path text"
    );
}
