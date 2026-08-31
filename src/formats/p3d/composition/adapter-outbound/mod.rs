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
//   - Adapter outbound outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Adapter outbound outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for p3d.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Adapter outbound outbound adapter.

mod dds;
pub mod decoders;
pub mod expression;
/// Item.
pub mod extractor;
mod filesystem_batch_artifact;
mod filesystem_batch_cache;
mod filesystem_batch_exporter;
mod image;
mod json;
/// Item.
pub mod package;
mod root_identity;
mod sprite_raster;

pub use dds::{DecodedRgbaImage, decode_legacy_dds};
pub use extractor::LosslessPackageExporter;
pub use filesystem_batch_exporter::FilesystemBatchExporter;
pub use package::write_lossless_package;
pub use sprite_raster::{SpriteRasterLayout, assemble_sprite_rgba};
