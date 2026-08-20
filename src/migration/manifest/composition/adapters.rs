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
//   - Adapters composition module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Adapters composition module.
// - Description:
//   - Implements the declared composition module responsibility for manifest.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Adapters composition module.

#[path = "adapter-outbound/mod.rs"]
pub mod driven;
#[path = "adapter-inbound/mod.rs"]
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
    Ok(count_by_dir_ext_paths(root, &files))
}
