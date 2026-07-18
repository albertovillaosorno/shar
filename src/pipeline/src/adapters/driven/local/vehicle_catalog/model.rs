// File:
//   - model.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/vehicle_catalog/model.rs
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
//   - In-memory catalog records for one published vehicle batch.
// - Must-Not:
//   - Read files, assemble geometry, or write artifacts.
// - Allows:
//   - Typed immutable publication evidence.
// - Summary:
//   - Vehicle catalog record types.
//
// Large file:
//   - false
//

//! Vehicle catalog record types.

use fbx::adapters::driven::binary_character_writer::CharacterBinaryFbxSummary;

/// One semantic geometry object published inside a vehicle FBX.
#[derive(Clone, Debug)]
pub(super) struct PartRecord {
    pub(super) name: String,
    pub(super) source_mesh: String,
    pub(super) role: &'static str,
    pub(super) shader: String,
    pub(super) bones: Vec<String>,
}

/// One published texture payload and its semantic state role.
#[derive(Clone, Debug)]
pub(super) struct TextureRecord {
    pub(super) path: String,
    pub(super) role: &'static str,
    pub(super) bytes: u64,
    pub(super) sha256: String,
}

/// One completed vehicle artifact and catalog record.
#[derive(Clone, Debug)]
pub(super) struct VehicleRecord {
    pub(super) vehicle: String,
    pub(super) package_id: String,
    pub(super) subcategory: String,
    pub(super) fbx_path: String,
    pub(super) fbx_bytes: u64,
    pub(super) fbx_sha256: String,
    pub(super) summary: CharacterBinaryFbxSummary,
    pub(super) parts: Vec<PartRecord>,
    pub(super) deferred_geometry: Vec<String>,
    pub(super) animations: Vec<String>,
    pub(super) effect_animation_sidecars: Vec<String>,
    pub(super) textures: Vec<TextureRecord>,
    pub(super) shaders: Vec<String>,
}
