// File:
//   - lib.rs
// Path:
//   - src/p3d/src/lib.rs
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
//   - The p3d public library facade for lib.
// - Must-Not:
//   - Violate repository architecture, path, provenance, or output rules.
// - Allows:
//   - Operations required to validate and execute public crate facade.
// - Split-When:
//   - Split when public crate facade contains two independently testable
//   - contracts.
// - Merge-When:
//   - Another p3d module owns the same library facade boundary with no distinct
//   - invariant.
// - Summary:
//   - `Pure3D` extraction and schema recovery library.
// - Description:
//   - Defines public crate facade data and behavior for p3d root.
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

//! `Pure3D` extraction and schema recovery library.
//!
//! This boundary keeps `pure3d` extraction and schema recovery library
//! explicit and returns deterministic results to p3d callers.
/// Inbound and outbound adapters.
pub mod adapters;
/// Package-level application commands.
pub mod application;
/// Pure package-independent chunk and extraction domain.
#[path = "domain/domain.rs"]
pub mod domain;
/// Outbound package export contracts.
pub mod ports;
/// Public Pure3D schema constants and definitions.
pub mod schema;

pub use adapters::driven::{LosslessPackageExporter, write_lossless_package};
pub use application::ExportPackage;
pub use domain::{ChunkKind, ChunkRecord, P3dDocument, P3dError, analyze_p3d};
pub use ports::PackageExporter;
