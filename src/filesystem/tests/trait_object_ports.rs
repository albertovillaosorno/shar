// File:
//   - trait_object_ports.rs
// Path:
//   - src/filesystem/tests/trait_object_ports.rs
//
// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE
// Path-Rule:
//   - All paths in this header are repository-root relative.
//
// Boundary-Contract:
// - Owns:
//   - Regression coverage for dynamically dispatched filesystem ports.
// - Must-Not:
//   - Depend on the standard adapter or caller-specific composition.
// - Allows:
//   - Exercise application use cases through trait-object references.
// - Split-When:
//   - Split when another dispatch model gains independent policy.
// - Merge-When:
//   - Another test file owns the same port-dispatch contract.
// - Summary:
//   - Trait-object port regression tests.
// - Description:
//   - Protects runtime adapter substitution at the application boundary.
// - Usage:
//   - Runs through the filesystem crate test target.
// - Defaults:
//   - Static and dynamic port references are equivalent.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Regression coverage for dynamically dispatched filesystem ports.
//!
//! Hexagonal application use cases must accept runtime-selected adapters.
use std::io;
use std::path::{Path, PathBuf};

use schoenwald_filesystem::application::CollectRegularFiles;
use schoenwald_filesystem::ports::TreeReader;

struct EmptyTree;

impl TreeReader for EmptyTree {
    fn regular_files(
        &self,
        _root: &Path,
    ) -> io::Result<Vec<PathBuf>> {
        Ok(Vec::new())
    }
}

#[test]
fn collection_accepts_trait_object_reader() -> Result<(), String> {
    let reader: &dyn TreeReader = &EmptyTree;
    let files = CollectRegularFiles::execute(
        reader,
        Path::new("root"),
    )
    .map_err(|error| error.to_string())?;

    if !files.is_empty() {
        return Err(format!("empty adapter returned files: {files:?}"));
    }
    Ok(())
}
