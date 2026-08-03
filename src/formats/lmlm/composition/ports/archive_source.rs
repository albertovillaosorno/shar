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
//   - Archive source outbound port.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Archive source outbound port.
// - Description:
//   - Implements the declared outbound port responsibility for lmlm.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Archive source outbound port.

use std::io;
use std::path::Path;

/// Loads archive bytes from a caller-selected source.
pub trait ArchiveSource {
    /// Reads one complete archive snapshot.
    ///
    /// # Errors
    ///
    /// Returns an I/O error when the source cannot be read completely.
    fn read_archive(&self, path: &Path) -> io::Result<Vec<u8>>;
}
