// File:
//   - text_artifact_store.rs
// Path:
//   - src/game-manifest/src/ports/text_artifact_store.rs
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
//   - Outbound read and publication contract for complete text artifacts.
// - Must-Not:
//   - Traverse trees, classify records, or infer artifact paths.
// - Allows:
//   - Read optional complete text and publish complete caller-supplied text.
// - Split-When:
//   - Split when reading and publication need independent providers.
// - Merge-When:
//   - Another port owns the same complete-text artifact boundary.
// - Summary:
//   - Port for manifest text artifacts.
// - Description:
//   - Keeps file storage outside application and domain layers.
// - Usage:
//   - Implemented by driven adapters and invoked by application commands.
// - Defaults:
//   - Missing artifacts are represented as `None`.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Outbound port for complete manifest text artifacts.
//!
//! Reads and writes remain replaceable behind one explicit storage contract.
use std::io;
use std::path::Path;

/// Reads and publishes complete UTF-8 text artifacts.
pub trait TextArtifactStore {
    /// Reads an optional complete artifact.
    ///
    /// # Errors
    ///
    /// Returns an I/O error when an existing artifact cannot be read.
    fn read_optional(
        &self,
        path: &Path,
    ) -> io::Result<Option<String>>;

    /// Publishes one complete artifact.
    ///
    /// # Errors
    ///
    /// Returns an I/O error when parent creation or writing fails.
    fn write(
        &self,
        path: &Path,
        text: &str,
    ) -> io::Result<()>;
}
