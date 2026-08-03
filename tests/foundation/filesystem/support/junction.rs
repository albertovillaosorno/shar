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
//   - Junction test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Junction test module.
// - Description:
//   - Implements the declared test module responsibility for filesystem.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Junction test module.

use std::path::Path;

/// Creates one Windows junction for a bounded integration-test fixture.
///
/// # Errors
///
/// Returns a diagnostic when the platform command cannot create the junction.
pub fn create_junction(link: &Path, target: &Path) -> Result<(), String> {
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
