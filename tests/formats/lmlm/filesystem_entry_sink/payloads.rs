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
//   - Payloads test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Payloads test module.
// - Description:
//   - Implements the declared test module responsibility for lmlm.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Payloads test module.

use std::io;
use std::path::Path;

use super::{materialize_entries, remove_test_root, test_root};
use crate::FileEntry;

#[test]
fn invalid_payload_errors_include_the_declared_range() {
    let entries = [FileEntry {
        path: "invalid.bin".to_owned(),
        offset: 2,
        size: 3,
    }];
    let result =
        materialize_entries(b"a", &entries, Path::new("unused-output-root"));

    assert!(
        matches!(
            result,
            Err(error)
                if error
                    .to_string()
                    .contains("offset 2")
                    && error
                        .to_string()
                        .contains("size 3")
        ),
        "invalid payload errors must retain offset and size evidence"
    );
}

#[test]
fn rejects_invalid_later_payload_before_writing() -> Result<(), String> {
    let root = test_root("invalid-later-payload");
    remove_test_root(&root)?;
    let entries = [
        FileEntry {
            path: "first.bin".to_owned(),
            offset: 0,
            size: 1,
        },
        FileEntry {
            path: "invalid.bin".to_owned(),
            offset: 2,
            size: 1,
        },
    ];
    let result = materialize_entries(b"a", &entries, &root);
    let first_exists = root.join("first.bin").exists();
    remove_test_root(&root)?;
    match result {
        Err(error)
            if error.kind() == io::ErrorKind::InvalidData && !first_exists =>
        {
            Ok(())
        },
        other => Err(format!(
            "invalid later payload must fail before writes, got {other:?} \
                 and first_exists={first_exists}"
        )),
    }
}
