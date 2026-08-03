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
//   - Tree ordering test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Tree ordering test module.
// - Description:
//   - Implements the declared test module responsibility for filesystem.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Tree ordering test module.

use std::io;
use std::path::{Path, PathBuf};

use schoenwald_filesystem::application::CollectRegularFiles;
use schoenwald_filesystem::ports::TreeReader;

struct UnsortedTree;

impl TreeReader for UnsortedTree {
    fn regular_files(&self, _root: &Path) -> io::Result<Vec<PathBuf>> {
        Ok(vec![
            PathBuf::from("root/z.bin"),
            PathBuf::from("root/a.bin"),
        ])
    }
}

#[test]
fn application_sorts_adversarial_port_output() -> Result<(), String> {
    let actual = CollectRegularFiles::execute(&UnsortedTree, Path::new("root"))
        .map_err(|error| error.to_string())?;
    let expected =
        vec![PathBuf::from("root/a.bin"), PathBuf::from("root/z.bin")];

    if actual != expected {
        return Err(format!("tree snapshot was not sorted: {actual:?}"));
    }
    Ok(())
}
