// File:
//   - adapters.rs
// Path: src/formats/lmlm/composition/adapters.rs
//
// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT
// Path-Rule:
//   - All paths in this header are repository-root relative.
//
// Boundary-Contract:
// - Owns:
//   - LMLM inbound and outbound adapter families.
// - Must-Not:
//   - Own package parsing rules or application orchestration.
// - Allows:
//   - Protocol translation and concrete external mechanisms.
// - Split-When:
//   - Split when one adapter family becomes independently versioned.
// - Merge-When:
//   - Another facade owns the same adapter families.
// - Summary:
//   - Adapter facade for LMLM extraction.
// - Description:
//   - Separates driving request translation from driven storage mechanisms.
// - Usage:
//   - Imported by composition roots and integration tests.
// - Defaults:
//   - No adapter is selected implicitly by the core layers.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Inbound and outbound adapters for LMLM extraction.
//!
//! Driving adapters compose requests while driven adapters implement source and
//! sink ports.
#[path = "../adapter-outbound/mod.rs"]
pub mod driven;
#[path = "../adapter-inbound/mod.rs"]
pub mod driving;

pub use driven::{FileArchiveSource, FilesystemEntrySink, materialize_entries};
