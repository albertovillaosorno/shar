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
//   - Rcf lib.rs.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Rcf lib.rs.
// - Description:
//   - Implements the declared lib.rs responsibility for rcf.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Rcf lib.rs.

#[path = "adapters.rs"]
pub mod adapters;
#[path = "application/mod.rs"]
pub mod application;
#[path = "../domain/mod.rs"]
pub mod domain;
#[path = "ports/mod.rs"]
pub mod ports;

pub use application::{
    ArchiveParser, ExtractionReport, Extractor, ListArchive,
};
pub use domain::{Archive, ArchiveEntry, ArchiveError, IoFailure};
