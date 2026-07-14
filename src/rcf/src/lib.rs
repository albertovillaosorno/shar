// File:
//   - lib.rs
// Path:
//   - src/rcf/src/lib.rs
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
//   - The rcf public library facade for lib.
// - Must-Not:
//   - Violate repository architecture, path, provenance, or output rules.
// - Allows:
//   - Operations required to validate and execute public crate facade.
// - Split-When:
//   - Split when public crate facade contains two independently testable
//   - contracts.
// - Merge-When:
//   - Another rcf module owns the same library facade boundary with no distinct
//   - invariant.
// - Summary:
//   - RCF archive reader and extractor.
// - Description:
//   - Defines public crate facade data and behavior for rcf root.
// - Usage:
//   - Imported by workspace crates through the public library surface.
// - Defaults:
//   - No implicit output outside the repository is allowed.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! RCF archive reader and extractor.
//!
//! The supported archives use the `RADCORE CEMENT LIBRARY` container layout
//! found in the local game evidence owned by the operator. This crate keeps the
//! format parser dependency-free and separates pure archive interpretation from
//! filesystem extraction.
pub mod adapters;
pub mod application;
#[path = "domain/domain.rs"]
pub mod domain;
pub mod ports;

pub use application::{
    ArchiveParser, ExtractionReport, Extractor, ListArchive,
};
pub use domain::{Archive, ArchiveEntry, ArchiveError};
