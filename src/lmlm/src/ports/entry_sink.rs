// File:
//   - entry_sink.rs
// Path:
//   - src/lmlm/src/ports/entry_sink.rs
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
//   - Outbound publication contract for validated LMLM entries.
// - Must-Not:
//   - Parse archives, infer output roots, or alter entry paths.
// - Allows:
//   - Materialize bounded payloads beneath a caller-supplied root.
// - Split-When:
//   - Split when transactional and nonfilesystem sinks need distinct contracts.
// - Merge-When:
//   - Another port owns the same validated-entry publication boundary.
// - Summary:
//   - Port for publishing validated LMLM entries.
// - Description:
//   - Keeps output mechanisms outside the application and domain layers.
// - Usage:
//   - Implemented by driven adapters and invoked by extraction commands.
// - Defaults:
//   - No destination is inferred.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Outbound port for publishing validated LMLM entry payloads.
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
