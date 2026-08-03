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
//   - Lmlm lib.rs.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Lmlm lib.rs.
// - Description:
//   - Implements the declared lib.rs responsibility for lmlm.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Lmlm lib.rs.

#[path = "adapters.rs"]
pub mod adapters;
#[path = "application/mod.rs"]
pub mod application;
#[path = "../domain/mod.rs"]
pub mod domain;
#[path = "ports/mod.rs"]
pub mod ports;

pub use adapters::materialize_entries;
pub use application::{ExtractArchive, ExtractArchiveError};
pub use domain::{FileEntry, LmlmError, entry_bytes, parse};
pub use ports::{ArchiveSource, EntrySink};
