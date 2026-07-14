// File:
//   - lib.rs
// Path:
//   - src/lmlm/src/lib.rs
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
//   - The LMLM public hexagonal facade.
// - Must-Not:
//   - Hide dependency direction or select concrete adapters implicitly.
// - Allows:
//   - Expose layered APIs and deliberate compatibility re-exports.
// - Split-When:
//   - Split when a public bounded context becomes independently versioned.
// - Merge-When:
//   - Another facade owns the same crate-level contracts.
// - Summary:
//   - Public facade for validated LMLM parsing and extraction.
// - Description:
//   - Separates package rules, application commands, and filesystem adapters.
// - Usage:
//   - Imported by workspace crates and the thin executable.
// - Defaults:
//   - No source or destination is selected implicitly.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Hexagonal facade for validated LMLM parsing and extraction.
//!
//! Domain parsing stays pure, application commands depend on ports, and
//! concrete filesystem behavior remains in adapters.
pub mod adapters;
pub mod application;
pub(crate) mod diagnostic;
#[path = "domain/domain.rs"]
pub mod domain;
pub mod ports;

pub use adapters::materialize_entries;
pub use application::{ExtractArchive, ExtractArchiveError};
pub use domain::{FileEntry, LmlmError, entry_bytes, parse};
pub use ports::{ArchiveSource, EntrySink};
