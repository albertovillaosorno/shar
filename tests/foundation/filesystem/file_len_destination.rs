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
//   - File len destination test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - File len destination test module.
// - Description:
//   - Implements the declared test module responsibility for filesystem.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! File len destination test module.

use std::io;
use std::path::{Path, PathBuf};

use schoenwald_filesystem::PathKind;
use schoenwald_filesystem::application::InspectPath;
use schoenwald_filesystem::ports::PathInspector;

struct PermissiveInspector;

impl PathInspector for PermissiveInspector {
    fn path_kind(&self, _path: &Path) -> io::Result<PathKind> {
        Ok(PathKind::File)
    }

    fn file_len(&self, _path: &Path) -> io::Result<u64> {
        Ok(7)
    }

    fn canonicalize(&self, path: &Path) -> io::Result<PathBuf> {
        Ok(path.to_path_buf())
    }
}

#[test]
fn directory_syntax_file_length_is_rejected() -> Result<(), String> {
    let result = InspectPath::len(&PermissiveInspector, Path::new("report/"));

    if result.is_ok() {
        return Err(
            "directory syntax unexpectedly returned a length".to_owned()
        );
    }
    Ok(())
}
