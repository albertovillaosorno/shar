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
//   - Entry sink outbound port.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Entry sink outbound port.
// - Description:
//   - Implements the declared outbound port responsibility for lmlm.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Entry sink outbound port.

use std::io;
use std::path::Path;

use crate::domain::FileEntry;

/// Publishes validated archive entries through a replaceable mechanism.
pub trait EntrySink {
    /// Materializes all validated entries beneath one explicit output root.
    ///
    /// # Errors
    ///
    /// Returns an I/O error when preflight or publication fails.
    fn materialize(
        &self,
        archive: &[u8],
        entries: &[FileEntry],
        output_root: &Path,
    ) -> io::Result<usize>;
}
