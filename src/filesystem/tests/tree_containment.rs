// File:
//   - tree_containment.rs
// Path:
//   - src/filesystem/tests/tree_containment.rs
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
//   - Regression coverage for tree-entry containment.
// - Must-Not:
//   - Depend on local storage or trust malformed adapter output.
// - Allows:
//   - Assert that snapshots contain strict descendants of their root.
// - Split-When:
//   - Split when physical containment gains a separate provider contract.
// - Merge-When:
//   - Another test file owns the same lexical containment behavior.
// - Summary:
//   - Tree containment regression tests.
// - Description:
//   - Prevents adapters from returning lexically escaping paths.
// - Usage:
//   - Runs through the filesystem crate test target.
// - Defaults:
//   - Every result is a normalized lexical descendant.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Regression coverage for tree-entry containment.
//!
//! Provider output must remain a strict normalized descendant of the request.
use std::io;
use std::path::{Path, PathBuf};

use schoenwald_filesystem::application::CollectRegularFiles;
use schoenwald_filesystem::ports::TreeReader;

struct EscapingTree;

impl TreeReader for EscapingTree {
    fn regular_files(
        &self,
        _root: &Path,
    ) -> io::Result<Vec<PathBuf>> {
        Ok(vec![PathBuf::from("root/../escape.bin")])
    }
}

#[test]
fn application_rejects_lexically_escaping_port_path() -> Result<(), String> {
    let result = CollectRegularFiles::execute(
        &EscapingTree,
        Path::new("root"),
    );
    let Err(error) = result else {
        return Err("escaping path passed containment validation".to_owned());
    };

    if error.kind() != io::ErrorKind::InvalidData {
        return Err(
            format!(
                "unexpected containment error kind: {:?}",
                error.kind()
            ),
        );
    }
    Ok(())
}
