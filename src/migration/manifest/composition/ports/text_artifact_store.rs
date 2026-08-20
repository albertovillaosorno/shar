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
//   - Text artifact store outbound port.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Text artifact store outbound port.
// - Description:
//   - Implements the declared outbound port responsibility for manifest.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Text artifact store outbound port.

use std::io;
use std::path::Path;

/// Reads and publishes complete UTF-8 text artifacts.
pub trait TextArtifactStore {
    /// Reads an optional complete artifact.
    ///
    /// # Errors
    ///
    /// Returns an I/O error when an existing artifact cannot be read.
    fn read_optional(&self, path: &Path) -> io::Result<Option<String>>;

    /// Publishes one complete artifact.
    ///
    /// # Errors
    ///
    /// Returns an I/O error when parent creation or writing fails.
    fn write(&self, path: &Path, text: &str) -> io::Result<()>;
}
