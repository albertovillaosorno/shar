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
//   - Filesystem intake for the phase-three package index.
// - Must-Not:
//   - Own package parsing, validation, or selection policy.
// - Allows:
//   - Reading one caller-selected JSONL artifact and delegating parsing.
// - Split-When:
//   - Split when another storage transport gains independent behavior.
// - Merge-When:
//   - Merge when another composition module owns the same intake route.
// - Summary:
//   - Package-index filesystem intake.
// - Description:
//   - Preserves the public index read API while keeping domain parsing pure.
// - Usage:
//   - Compiled by the Pipeline composition root.
// - Defaults:
//   - Read failures retain escaped path and operating-system evidence.
//

//! Package-index filesystem intake.

use std::fs;
use std::path::Path;

use crate::domain::package::index::{
    PackageIntakeError, PhaseThreePackageIndex,
};

impl PhaseThreePackageIndex {
    /// Read the generated package index JSONL file from disk.
    ///
    /// # Errors
    ///
    /// Returns an error when the file cannot be read, a row cannot be parsed,
    /// or duplicate package ids are encountered.
    pub fn read(path: &Path) -> Result<Self, PackageIntakeError> {
        let contents = read_contents(path)?;
        Self::from_jsonl(&contents)
    }

    /// Read the package index for Unreal staging, validating but excluding
    /// fail-closed error evidence.
    pub(crate) fn read_for_unreal(
        path: &Path,
    ) -> Result<Self, PackageIntakeError> {
        let contents = read_contents(path)?;
        Self::from_jsonl_for_unreal(&contents)
    }
}

/// Read one package-index artifact with escaped path evidence on failure.
fn read_contents(path: &Path) -> Result<String, PackageIntakeError> {
    fs::read_to_string(path).map_err(|error| {
        PackageIntakeError::new(format!(
            "failed to read package index {}: {error}",
            path.display()
        ))
    })
}
