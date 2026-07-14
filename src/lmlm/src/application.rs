// File:
//   - application.rs
// Path:
//   - src/lmlm/src/application.rs
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
//   - LMLM application use cases.
// - Must-Not:
//   - Implement parsing internals, filesystem IO, or CLI presentation.
// - Allows:
//   - Coordinate domain behavior through explicit ports.
// - Split-When:
//   - Split when a use-case family becomes independently deployable.
// - Merge-When:
//   - Another facade owns the same application commands.
// - Summary:
//   - Application facade for LMLM extraction.
// - Description:
//   - Exposes validated extraction without selecting concrete adapters.
// - Usage:
//   - Called by driving adapters and library clients.
// - Defaults:
//   - No source or sink is selected implicitly.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! LMLM application use cases.
//!
//! Commands coordinate pure parsing through replaceable source and sink ports.
mod extract_archive;

pub use extract_archive::{ExtractArchive, ExtractArchiveError};
