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
//   - Tree uniqueness test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Tree uniqueness test module.
// - Description:
//   - Implements the declared test module responsibility for filesystem.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Tree uniqueness test module.

use std::io;
use std::path::{Path, PathBuf};

use schoenwald_filesystem::application::CollectRegularFiles;
use schoenwald_filesystem::ports::TreeReader;

struct DuplicateTree;

impl TreeReader for DuplicateTree {
    fn regular_files(&self, _root: &Path) -> io::Result<Vec<PathBuf>> {
        Ok(vec![
            PathBuf::from("root/file.bin"),
            PathBuf::from("root/file.bin"),
        ])
    }
}

#[test]
fn application_removes_duplicate_port_rows() -> Result<(), String> {
    let actual =
        CollectRegularFiles::execute(&DuplicateTree, Path::new("root"))
            .map_err(|error| error.to_string())?;
    let expected = vec![PathBuf::from("root/file.bin")];

    if actual != expected {
        return Err(format!(
            "duplicate paths escaped the use case: {actual:?}"
        ));
    }
    Ok(())
}
