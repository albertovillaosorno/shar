// File:
//   - adapters.rs
// Path:
//   - src/game-manifest/src/adapters.rs
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
//   - Game-manifest inbound and outbound adapter families.
// - Must-Not:
//   - Own domain classification or application orchestration.
// - Allows:
//   - Protocol translation and concrete external mechanisms.
// - Split-When:
//   - Split when one adapter family becomes independently versioned.
// - Merge-When:
//   - Another facade owns the same adapter families.
// - Summary:
//   - Adapter facade for manifest workflows.
// - Description:
//   - Separates driving CLI composition from driven filesystem mechanisms.
// - Usage:
//   - Imported by thin binaries, integration tests, and compatibility callers.
// - Defaults:
//   - Core layers select no concrete adapter.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Inbound and outbound adapters for game-manifest workflows.
//!
//! Driving adapters compose requests while driven adapters implement ports.
pub mod driven;
pub mod driving;

use std::io;
use std::path::Path;

pub use driven::{FilesystemGameTree, FilesystemTextStore};

use crate::domain::{DirExtCounts, count_by_dir_ext_paths};
use crate::ports::GameTree as _;

/// Compatibility helper that counts one local tree through the filesystem port.
///
/// # Errors
///
/// Returns a traversal error from the filesystem adapter.
pub fn count_by_dir_ext(root: &Path) -> io::Result<DirExtCounts> {
    let files = FilesystemGameTree.files(root)?;
    Ok(
        count_by_dir_ext_paths(
            root, &files,
        ),
    )
}
