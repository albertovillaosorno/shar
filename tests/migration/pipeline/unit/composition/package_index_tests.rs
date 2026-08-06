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
//   - Public package-index filesystem intake regressions.
// - Must-Not:
//   - Own package parsing or selection policy.
// - Allows:
//   - Missing temporary paths and diagnostic assertions.
// - Split-When:
//   - Split when another package-index transport gains independent behavior.
// - Merge-When:
//   - Merge when the composition adapter owns the same intake evidence.
// - Summary:
//   - Public package-index filesystem intake regressions.
// - Description:
//   - Prevents physical index paths and raw I/O text from entering errors.
// - Usage:
//   - Included only by the owning source module under cfg(test).
// - Defaults:
//   - Physical path leakage fails explicitly.
//

//! Public package-index filesystem intake regressions.

use std::path::PathBuf;

use super::read_contents;

#[test]
fn package_index_read_errors_hide_the_physical_path() -> Result<(), String> {
    let private_fragment = "private-workstation-package-index";
    let path = std::env::temp_dir()
        .join(private_fragment)
        .join(PathBuf::from("missing").join("index.jsonl"));
    let error = match read_contents(&path) {
        Ok(_contents) => {
            return Err("missing package index was accepted".to_owned());
        },
        Err(error) => error.to_string(),
    };
    if error.contains(private_fragment)
        || error != "failed to read package index (NotFound)"
    {
        return Err(format!("package-index diagnostic leaked: {error}"));
    }
    Ok(())
}
