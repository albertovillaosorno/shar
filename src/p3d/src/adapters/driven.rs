// File:
//   - driven.rs
// Path:
//   - src/p3d/src/adapters/driven.rs
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
//   - The p3d adapter boundary for adapters driven.
// - Must-Not:
//   - Change domain semantics or bypass application and port contracts.
// - Allows:
//   - Filesystem, JSON, CLI, Blender, or serialization work behind explicit
//   - ports.
// - Split-When:
//   - Split when driven contains two independently testable contracts.
// - Merge-When:
//   - Another p3d module owns the same adapters boundary with no distinct
//   - invariant.
// - Summary:
//   - Defines decoders for this module boundary.
// - Description:
//   - Defines driven data and behavior for p3d adapters.
// - Usage:
//   - Constructed by composition roots or tests and passed behind port traits.
// - Defaults:
//   - Adapter defaults stay local to the adapter constructor or config.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! This code exposes decoders at the adapter boundary for adapters driven.
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
mod schema_recovery;

pub use extractor::LosslessPackageExporter;
pub use filesystem_batch_exporter::FilesystemBatchExporter;
pub use package::write_lossless_package;
