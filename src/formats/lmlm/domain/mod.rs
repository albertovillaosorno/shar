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
//   - Domain domain module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Domain domain module.
// - Description:
//   - Implements the declared domain module responsibility for lmlm.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Domain domain module.

mod binary;
mod container;
pub(crate) mod diagnostic;
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
#[path = "../../../../tests/formats/lmlm/unit/domain/tests.rs"]
mod tests;
