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
//   - Implements the declared domain module responsibility for rcf.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Domain domain module.

mod escaped_path;
mod failure;
pub(crate) mod path_policy;
mod records;

pub use failure::{ArchiveError, IoFailure};
pub use records::{Archive, ArchiveEntry, ArchiveHeader, IndexRecord};
