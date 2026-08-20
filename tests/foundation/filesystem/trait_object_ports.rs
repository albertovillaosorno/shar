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
//   - Trait object ports test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Trait object ports test module.
// - Description:
//   - Implements the declared test module responsibility for filesystem.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Trait object ports test module.

use std::io;
use std::path::{Path, PathBuf};

use schoenwald_filesystem::application::CollectRegularFiles;
use schoenwald_filesystem::ports::TreeReader;

struct EmptyTree;

impl TreeReader for EmptyTree {
    fn regular_files(&self, _root: &Path) -> io::Result<Vec<PathBuf>> {
        Ok(Vec::new())
    }
}

#[test]
fn collection_accepts_trait_object_reader() -> Result<(), String> {
    let reader: &dyn TreeReader = &EmptyTree;
    let files = CollectRegularFiles::execute(reader, Path::new("root"))
        .map_err(|error| error.to_string())?;

    if !files.is_empty() {
        return Err(format!("empty adapter returned files: {files:?}"));
    }
    Ok(())
}
