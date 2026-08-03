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
//   - Tree containment test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Tree containment test module.
// - Description:
//   - Implements the declared test module responsibility for filesystem.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Tree containment test module.

use std::io;
use std::path::{Path, PathBuf};

use schoenwald_filesystem::application::CollectRegularFiles;
use schoenwald_filesystem::ports::TreeReader;

struct EscapingTree;

impl TreeReader for EscapingTree {
    fn regular_files(&self, _root: &Path) -> io::Result<Vec<PathBuf>> {
        Ok(vec![PathBuf::from("root/../escape.bin")])
    }
}

#[test]
fn application_rejects_lexically_escaping_port_path() -> Result<(), String> {
    let result = CollectRegularFiles::execute(&EscapingTree, Path::new("root"));
    let Err(error) = result else {
        return Err("escaping path passed containment validation".to_owned());
    };

    if error.kind() != io::ErrorKind::InvalidData {
        return Err(format!(
            "unexpected containment error kind: {:?}",
            error.kind()
        ));
    }
    Ok(())
}
