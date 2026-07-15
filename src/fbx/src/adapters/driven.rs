// File:
//   - driven.rs
// Path:
//   - src/fbx/src/adapters/driven.rs
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
//   - The fbx adapter boundary for adapters driven.
// - Must-Not:
//   - Change domain semantics or bypass application and port contracts.
// - Allows:
//   - Filesystem, JSON, CLI, Blender, or serialization work behind explicit
//   - ports.
// - Split-When:
//   - Split when driven contains two independently testable contracts.
// - Merge-When:
//   - Another fbx module owns the same adapters boundary with no distinct
//   - invariant.
// - Summary:
//   - Replaceable Blender command adapter boundary.
// - Description:
//   - Defines driven data and behavior for fbx adapters.
// - Usage:
//   - Constructed by composition roots or tests and passed behind port traits.
// - Defaults:
//   - Adapter defaults stay local to the adapter constructor or config.
//
// ADRs:
// - docs/adr/pipeline/fbx/hexagonal-scene-export.md
// - docs/adr/pipeline/unreal/unreal-manifest-and-package-taxonomy.md
//
// Large file:
//   - false
//

//! Canonical binary FBX 7.7 writer and optional review helpers.
/// Binary animation graph planner used by the canonical writer.
mod binary_animation;
/// Serializer-local character validation for the binary writer.
mod binary_character_input;
/// Native binary FBX 7.7 character writer.
pub mod binary_character_writer;
/// Typed FBX 7.7 binary container encoder.
mod binary_fbx;
/// Deterministic object identity inside one binary FBX document.
mod binary_identity;
/// Replaceable Blender command adapter boundary.
pub mod blender_review_helper;
pub mod blender_scene_writer;
/// Decoded skeletal animation source adapter.
pub mod decoded_animation_source;
/// Decoded component source adapter.
pub mod decoded_component_source;
pub mod decoded_skin_source;
/// Generated package-index reader adapter.
pub mod generated_package_index;
/// Optional Maya script that imports the canonical sibling FBX.
pub mod maya_import_helper;
/// In-memory semantic character texture artifact transaction.
pub mod semantic_character_texture;
/// Deterministic PNG byte adapter for semantic character textures.
pub mod semantic_texture_png;
