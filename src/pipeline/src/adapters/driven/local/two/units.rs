// File:
//   - units.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/two/units.rs
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
//   - The minor units contract for pipeline phase two.
// - Must-Not:
//   - Violate repository architecture, path, provenance, or output rules.
// - Allows:
//   - Operations required to validate and execute minor units.
// - Split-When:
//   - Split when minor units contains two independently testable contracts.
// - Merge-When:
//   - Another pipeline module owns the same module boundary with no distinct
//   - invariant.
// - Summary:
//   - Audit minor units module.
// - Description:
//   - Defines minor units data and behavior for pipeline phase two.
// - Usage:
//   - Used by pipeline phase two code that needs minor units.
// - Defaults:
//   - No implicit output outside the repository is allowed.
//
// ADRs:
// - docs/adr/pipeline/minor-unit-taxonomy-value-case.md
//
// Large file:
//   - false
//

//! Audit minor units module.
//!
//! This boundary keeps audit minor units module explicit and returns
//! deterministic results to pipeline callers.
/// Audio and movie package classifier.
mod audio_video;
pub(in crate::adapters::driven::local) mod audit_minor_units;
/// Vehicle package classifier.
mod cars;
/// Cinematic package classifier.
mod cinematics;
/// Editor.
pub(in crate::adapters::driven::local) mod editor;
/// Minor-unit package index.
pub(in crate::adapters::driven::local) mod index;
/// Minor-unit package index renderer.
mod index_render;
/// Manifest minor unit.
pub(in crate::adapters::driven::local) mod manifest_minor_unit;
/// Metadata.
mod metadata;
/// Metadata fill.
pub(in crate::adapters::driven::local) mod metadata_fill;
/// Taxonomy.
mod taxonomy;
/// UI image package classifier.
mod ui_images;
/// UI resource package classifier.
mod ui_resources;
/// UI screen package classifier.
mod ui_screens;
/// UI vehicle preview package classifier.
mod ui_vehicle_previews;
