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
//   - Paths test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Paths test module.
// - Description:
//   - Implements the declared test module responsibility for lmlm.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Paths test module.

use std::io;

use super::{materialize_entries, remove_test_root, test_root};
use crate::FileEntry;

#[test]
fn rejects_unicode_path_modifiers_before_writing() -> Result<(), String> {
    let root = test_root("unsafe-unicode-path");
    remove_test_root(&root)?;
    let path = "report\u{202e}cod.exe";
    let entries = [FileEntry {
        path: path.to_owned(),
        offset: 0,
        size: 1,
    }];
    let result = materialize_entries(b"x", &entries, &root);
    let destination_exists = root.join(path).exists();
    remove_test_root(&root)?;
    match result {
        Err(error)
            if error.kind() == io::ErrorKind::InvalidInput
                && !destination_exists =>
        {
            Ok(())
        },
        other => Err(format!(
            "unsafe path must fail before writing, got {other:?}, \
                 destination_exists={destination_exists}"
        )),
    }
}
