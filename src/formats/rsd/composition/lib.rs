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
//   - Rsd lib.rs.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Rsd lib.rs.
// - Description:
//   - Implements the declared lib.rs responsibility for rsd.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! RSD audio export library facade.

/// Inbound and outbound runtime adapters.
#[path = "adapters.rs"]
pub mod adapters;
/// Audio export application use cases.
#[path = "../application/mod.rs"]
pub mod application;
/// RSD and WAV domain models.
#[path = "../domain/mod.rs"]
pub mod domain;
/// Outbound contracts for batch export.
#[path = "../port-outbound/mod.rs"]
pub mod ports;

pub use application::ExportRoots;
pub use domain::{
    ExportReport, RsdAudio, RsdEncoding, RsdError, RsdHeader, SourceRootReport,
    WavAudio,
};
pub use ports::Exporter;
