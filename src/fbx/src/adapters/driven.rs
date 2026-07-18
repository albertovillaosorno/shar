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
//   - Filesystem, JSON, CLI, or serialization work behind explicit ports.
// - Split-When:
//   - Split when driven contains two independently testable contracts.
// - Merge-When:
//   - Another fbx module owns the same adapters boundary with no distinct
//   - invariant.
// - Summary:
//   - Native binary FBX and decoded-source adapter boundary.
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

//! Canonical binary FBX 7.7 writer and decoded-source adapters.
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
/// Conservative horizontal UV correction policy.
mod binary_uv_policy;
/// Decoded skeletal animation source adapter.
pub mod decoded_animation_source;
/// Decoded billboard quad-group source adapter.
pub mod decoded_billboard_source;
/// Decoded component source adapter.
pub mod decoded_component_source;
/// Selected rigid-prop source adapter.
pub mod decoded_rigid_prop_source;
/// Decoded skeleton, skin, and composite source adapter.
pub mod decoded_skin_source;
/// Generated package-index reader adapter.
pub mod generated_package_index;
/// In-memory semantic character texture artifact transaction.
pub mod semantic_character_texture;
/// Deterministic PNG byte adapter for semantic character textures.
pub mod semantic_texture_png;
