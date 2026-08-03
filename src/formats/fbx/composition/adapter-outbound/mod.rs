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
//   - Implements the declared outbound adapter responsibility for fbx.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Adapter outbound outbound adapter.

mod binary_animation;
/// Serializer-local character validation for the binary writer.
mod binary_character_input;
/// Native binary FBX 7.7 character writer.
pub mod binary_character_writer;
/// Typed FBX 7.7 binary container encoder.
mod binary_fbx;
/// Create-new binary FBX filesystem persistence adapter.
mod binary_fbx_storage;
/// Deterministic object identity inside one binary FBX document.
mod binary_identity;
/// Canonical one-mesh FBX 7.7 structural-guide writer.
pub mod binary_structural_guide_writer;
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
