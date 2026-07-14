// File:
//   - junction.rs
// Path:
//   - src/filesystem/tests/support/junction.rs
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
//   - Shared platform fixtures for filesystem integration tests.
// - Must-Not:
//   - Assert product behavior or expose machine-specific paths.
// - Allows:
//   - Create bounded local link fixtures with explicit cleanup ownership.
// - Split-When:
//   - Split when another platform fixture has independent lifecycle policy.
// - Merge-When:
//   - Another test support module owns the same fixture command.
// - Summary:
//   - Filesystem integration test support.
// - Description:
//   - Centralizes reliable platform setup without duplicating assertions.
// - Usage:
//   - Imported only by filesystem integration tests under matching cfg gates.
// - Defaults:
//   - Callers own fixture cleanup after each assertion.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Shared platform fixtures for filesystem integration tests.
//!
//! Assertion policy stays in each owning integration test.
use std::path::Path;

/// Creates one Windows junction for a bounded integration-test fixture.
///
/// # Errors
///
/// Returns a diagnostic when the platform command cannot create the junction.
pub fn create_junction(
    link: &Path,
    target: &Path,
) -> Result<(), String> {
    let link_status = std::process::Command::new("cmd")
        .arg("/C")
        .arg("mklink")
        .arg("/J")
        .arg(link)
        .arg(target)
        .status()
        .map_err(|error| error.to_string())?;
    if !link_status.success() {
        return Err("failed to create junction fixture".to_owned());
    }
    Ok(())
}
