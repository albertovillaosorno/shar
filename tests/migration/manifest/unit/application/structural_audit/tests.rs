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
//   - Tests unit tests.
// - Must-Not:
//   - Own production behavior or broaden the tested API surface.
// - Allows:
//   - Private test fixtures and assertions for the owning source module.
// - Split-When:
//   - Split when an independent fixture family gains separate ownership.
// - Merge-When:
//   - Merge when another test module owns the identical evidence.
// - Summary:
//   - Tests unit tests.
// - Description:
//   - Preserves unit-test access through a test-only path module.
// - Usage:
//   - Included only by the owning source module under cfg(test).
// - Defaults:
//   - Test setup and assertions fail explicitly.
//

//! Tests unit tests.

use std::io;
use std::path::{Path, PathBuf};

use super::StructuralAudit;
use crate::domain::{NO_EXTENSION, extension_of};
use crate::ports::{GameTree, PathKind};

struct DuplicateTree;
struct OutsideTree;
struct ParentTraversalTree;

impl GameTree for DuplicateTree {
    fn kind(&self, _path: &Path) -> io::Result<PathKind> {
        Ok(PathKind::Directory)
    }

    fn files(&self, root: &Path) -> io::Result<Vec<PathBuf>> {
        let path = root.join("asset.mfk");
        Ok(vec![path.clone(), path])
    }
}

impl GameTree for OutsideTree {
    fn kind(&self, _path: &Path) -> io::Result<PathKind> {
        Ok(PathKind::Directory)
    }

    fn files(&self, _root: &Path) -> io::Result<Vec<PathBuf>> {
        Ok(vec![PathBuf::from("other/asset.mfk")])
    }
}

impl GameTree for ParentTraversalTree {
    fn kind(&self, _path: &Path) -> io::Result<PathKind> {
        Ok(PathKind::Directory)
    }

    fn files(&self, root: &Path) -> io::Result<Vec<PathBuf>> {
        Ok(vec![root.join("area/../asset.mfk")])
    }
}

#[test]
fn duplicate_file_evidence_counts_once() {
    let report = StructuralAudit::execute(&DuplicateTree, Path::new("game"));

    assert_eq!(
        report.ok().map(|value| value.total_dirty_extensions,),
        Some(1),
    );
}

#[test]
fn outside_root_evidence_is_rejected() {
    let result = StructuralAudit::execute(&OutsideTree, Path::new("game"));

    assert!(result.is_err());
}

#[test]
fn parent_traversal_evidence_is_rejected() {
    let result =
        StructuralAudit::execute(&ParentTraversalTree, Path::new("game"));

    assert!(result.is_err());
}

#[test]
fn trailing_dot_extension_is_missing() {
    assert_eq!(extension_of(Path::new("asset.")), NO_EXTENSION);
}
