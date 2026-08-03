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
//   - Directory creation test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Directory creation test module.
// - Description:
//   - Implements the declared test module responsibility for filesystem.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Directory creation test module.

use std::io;
use std::path::Path;

use schoenwald_filesystem::adapters::driving::local;

#[test]
fn empty_directory_path_is_rejected() -> Result<(), String> {
    let error = match local::create_dir_all(Path::new("")) {
        Ok(()) => {
            return Err(
                "an empty path reported directory creation success".to_owned()
            );
        },
        Err(error) => error,
    };

    if error.kind() != io::ErrorKind::InvalidInput {
        return Err(format!(
            "unexpected empty-path error kind: {:?}",
            error.kind()
        ));
    }
    Ok(())
}
