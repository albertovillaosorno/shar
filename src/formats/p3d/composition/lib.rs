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
//   - P3d lib.rs.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - P3d lib.rs.
// - Description:
//   - Implements the declared lib.rs responsibility for p3d.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! P3d lib.rs.

#[path = "adapters.rs"]
pub mod adapters;
/// Package-level application commands.
#[path = "application/mod.rs"]
pub mod application;
/// Pure package-independent chunk and extraction domain.
#[path = "../domain/mod.rs"]
pub mod domain;
/// Outbound package export contracts.
#[path = "ports/mod.rs"]
pub mod ports;
/// Public Pure3D schema constants and definitions.
#[path = "../contract/schema.rs"]
pub mod schema;

pub use adapters::driven::{
    DecodedRgbaImage, LosslessPackageExporter, SpriteRasterLayout,
    assemble_sprite_rgba, decode_legacy_dds, write_lossless_package,
};
pub use application::ExportPackage;
pub use domain::{ChunkKind, ChunkRecord, P3dDocument, P3dError, analyze_p3d};
pub use ports::PackageExporter;
