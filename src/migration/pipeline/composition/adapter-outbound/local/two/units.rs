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
//   - Units outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Units outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for pipeline.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Units outbound adapter.

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
