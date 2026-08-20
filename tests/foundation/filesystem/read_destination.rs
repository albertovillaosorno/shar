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
//   - Read destination test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Read destination test module.
// - Description:
//   - Implements the declared test module responsibility for filesystem.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Read destination test module.

use std::io;
use std::path::Path;

use schoenwald_filesystem::application::ReadFile;
use schoenwald_filesystem::ports::FileReader;

struct PermissiveReader;

impl FileReader for PermissiveReader {
    fn read_bytes(&self, _path: &Path) -> io::Result<Vec<u8>> {
        Ok(Vec::new())
    }
}

#[test]
fn directory_syntax_read_destination_is_rejected() -> Result<(), String> {
    let result = ReadFile::bytes(&PermissiveReader, Path::new("report/"));

    if result.is_ok() {
        return Err("directory syntax unexpectedly returned bytes".to_owned());
    }
    Ok(())
}
