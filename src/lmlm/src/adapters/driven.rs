// File:
//   - driven.rs
// Path:
//   - src/lmlm/src/adapters/driven.rs
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
//   - Replaceable outbound adapters for LMLM source and publication ports.
// - Must-Not:
//   - Parse CLI requests or change package semantics.
// - Allows:
//   - Filesystem archive reads and validated payload materialization.
// - Split-When:
//   - Split when another storage provider gains an independent adapter family.
// - Merge-When:
//   - Another facade owns the same outbound implementations.
// - Summary:
//   - Driven adapter facade for LMLM extraction.
// - Description:
//   - Exposes concrete archive source and entry sink implementations.
// - Usage:
//   - Constructed by driving adapters and integration tests.
// - Defaults:
//   - No input or output path is selected implicitly.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Driven adapters implementing LMLM outbound ports.
//!
//! Filesystem mechanisms remain outside the domain and application layers.
mod file_archive_source;
mod filesystem_entry_sink;

pub use file_archive_source::FileArchiveSource;
pub use filesystem_entry_sink::{FilesystemEntrySink, materialize_entries};
