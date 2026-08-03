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
//   - Tree portable collision test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Tree portable collision test module.
// - Description:
//   - Implements the declared test module responsibility for filesystem.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Tree portable collision test module.

use std::io;
use std::path::{Path, PathBuf};

use schoenwald_filesystem::application::CollectRegularFiles;
use schoenwald_filesystem::ports::TreeReader;

struct CaseCollidingTree;

impl TreeReader for CaseCollidingTree {
    fn regular_files(&self, _root: &Path) -> io::Result<Vec<PathBuf>> {
        Ok(vec![
            PathBuf::from("root/File.bin"),
            PathBuf::from("root/file.bin"),
        ])
    }
}

#[test]
fn case_colliding_tree_entries_are_rejected() -> Result<(), String> {
    let result =
        CollectRegularFiles::execute(&CaseCollidingTree, Path::new("root"));
    let Err(error) = result else {
        return Err("case-colliding tree entries were accepted".to_owned());
    };
    if error.kind() != io::ErrorKind::InvalidData {
        return Err(format!(
            "unexpected collision error kind: {:?}",
            error.kind()
        ));
    }
    Ok(())
}
