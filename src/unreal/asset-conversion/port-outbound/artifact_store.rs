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
//   - Artifact store outbound port.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Artifact store outbound port.
// - Description:
//   - Implements the declared responsibility for asset conversion.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Artifact store outbound port.

/// Storage boundary for deterministic conversion-plan artifacts.
pub trait PlanStore {
    /// Read one UTF-8 conversion artifact.
    ///
    /// # Errors
    ///
    /// Returns an error when the adapter cannot read or decode the artifact.
    fn read_text(&self, path: &str) -> Result<String, String>;

    /// Write one UTF-8 conversion artifact.
    ///
    /// # Errors
    ///
    /// Returns an error when the adapter cannot persist the artifact.
    fn write_text(&self, path: &str, text: &str) -> Result<(), String>;

    /// Return true when the conversion artifact exists.
    fn exists(&self, path: &str) -> bool;
}
