// Copyright:
//   - Copyright © 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT
//
// Boundary-Contract:
// - Owns:
//   - Pure reviewed LSPA-v5 archive parsing domain.
// - Must-Not:
//   - Read files, publish workspaces, or execute legacy payloads.
// - Allows:
//   - Validate archive bytes and expose safe entry descriptors.
// - Split-When:
//   - Another archive revision requires independent parsing policy.
// - Merge-When:
//   - Another domain module owns the identical LSPA-v5 contract.
// - Summary:
//   - LMLM archive parsing domain.
// - Description:
//   - Models and validates supported legacy container bytes without effects.
// - Usage:
//   - Used by composition inspection and conversion services.
// - Defaults:
//   - Malformed, ambiguous, or unsafe archive structure fails closed.
//

//! Complete reviewed LSPA-v5 archive parsing boundary used by this tool.
//!
//! This module describes archive structure only. Publication containment and
//! hostile-input policy live in [`super::security`].

mod binary;
mod container;
pub(crate) mod diagnostic;
mod entry;
mod error;
mod layout;
mod name;
mod parser;
mod payload;
mod table;
mod validation;

pub use entry::FileEntry;
pub use error::LmlmError;
pub use parser::parse;
pub use payload::entry_bytes;

#[cfg(test)]
#[path = "../../../../tests/tools/lmlm/archive.rs"]
mod tests;
