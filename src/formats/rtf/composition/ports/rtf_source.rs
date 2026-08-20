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
//   - Rtf source outbound port.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Rtf source outbound port.
// - Description:
//   - Implements the declared outbound port responsibility for rtf.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Rtf source outbound port.

use std::io;
use std::path::Path;

/// Complete source evidence needed by the conversion use case.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RtfSnapshot {
    /// Raw RTF bytes.
    pub bytes: Vec<u8>,
    /// Optional Unix timestamp from weak filesystem provenance.
    pub modified_unix_seconds: Option<i64>,
}

/// Loads RTF bytes and optional provenance evidence.
pub trait RtfSource {
    /// Loads one complete source snapshot.
    ///
    /// # Errors
    ///
    /// Returns an I/O error when the document bytes cannot be read.
    fn load(&self, path: &Path) -> io::Result<RtfSnapshot>;
}
