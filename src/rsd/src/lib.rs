// File:
//   - lib.rs
// Path:
//   - src/rsd/src/lib.rs
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
//   - The rsd public library facade for lib.
// - Must-Not:
//   - Violate repository architecture, path, provenance, or output rules.
// - Allows:
//   - Operations required to validate and execute public crate facade.
// - Split-When:
//   - Split when public crate facade contains two independently testable
//   - contracts.
// - Merge-When:
//   - Another rsd module owns the same library facade boundary with no distinct
//   - invariant.
// - Summary:
//   - Exposes the RSD library surface for deterministic audio export workflows.
// - Description:
//   - Defines public crate facade data and behavior for rsd root.
// - Usage:
//   - Imported by workspace crates through the public library surface.
// - Defaults:
//   - No implicit output outside the repository is allowed.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! The RSD library facade exposes audio export layers without selecting files.
#![cfg_attr(
    windows,
    feature(windows_by_handle)
)]

pub mod adapters;
pub mod application;
#[path = "domain/domain.rs"]
pub mod domain;
/// Outbound contracts for batch export.
pub mod ports;

pub use application::ExportRoots;
pub use domain::{
    ExportReport, RsdAudio, RsdEncoding, RsdError, RsdHeader, SourceRootReport,
    WavAudio,
};
pub use ports::Exporter;
