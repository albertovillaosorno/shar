// File:
//   - domain.rs
// Path:
//   - src/lmlm/src/domain/domain.rs
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
//   - Pure LMLM parsing, validation, and payload boundaries.
// - Must-Not:
//   - Read archive files, write extracted payloads, or parse CLI arguments.
// - Allows:
//   - Binary decoding, validation, entry records, and bounded payload access.
// - Split-When:
//   - Split when one parser subdomain becomes independently versioned.
// - Merge-When:
//   - Another facade owns the same LMLM format rules.
// - Summary:
//   - Domain facade for validated LMLM packages.
// - Description:
//   - Exposes pure package interpretation without filesystem policy.
// - Usage:
//   - Used by application commands, ports, adapters, and library clients.
// - Defaults:
//   - Unsupported packages fail closed.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Pure LMLM package parsing and validation domain.
//!
//! Binary interpretation remains deterministic and free of filesystem side
//! effects so adapters can be replaced independently.
mod binary;
mod container;
mod entry;
mod error;
mod layout;
mod name;
mod package;
mod parser;
mod payload;
mod table;
mod validation;

pub use entry::FileEntry;
pub use error::LmlmError;
pub(crate) use name::{portable_identity, portable_path_is_safe};
pub use parser::parse;
pub use payload::entry_bytes;

#[cfg(test)]
mod tests;
