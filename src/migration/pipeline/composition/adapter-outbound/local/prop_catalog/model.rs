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
//   - Model outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Model outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for pipeline.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Model outbound adapter.

use std::cmp::Ordering;
use std::path::PathBuf;

use fbx::adapters::driven::binary_character_writer::CharacterBinaryFbxSummary;
use serde_json::Value;

/// Catalog family that owns one prop source occurrence.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(super) enum PropFamily {
    /// Collectible-card and related model geometry from the cards package.
    Cards,
    /// Mission-specific models, including race flags and finish-line geometry.
    Missions,
    /// Dynamic, animated, and breakable models embedded in world packages.
    TerrainWorld,
}

impl PropFamily {
    /// Stable directory and JSON label.
    pub(super) const fn as_str(self) -> &'static str {
        match self {
            Self::Cards => "cards",
            Self::Missions => "missions",
            Self::TerrainWorld => "terrain-world",
        }
    }
}

/// FBX representation justified by normalized model evidence.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(super) enum PropRoute {
    /// Meshes connect directly to the FBX export root with no synthetic rig.
    Static,
    /// Rigid meshes retain their authored skeleton and exact PTRN clip.
    RigidAnimated,
}

impl PropRoute {
    /// Stable JSON route label.
    pub(super) const fn as_str(self) -> &'static str {
        match self {
            Self::Static => "static-model",
            Self::RigidAnimated => "rigid-animated-model",
        }
    }
}

/// One exact source billboard child retained as deferred presentation evidence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct DeferredBillboardQuadBinding {
    /// Authored child identity.
    pub(super) identity: String,
    /// Decoded source child schema version.
    pub(super) version: u32,
    /// Authored billboard orientation mode.
    pub(super) billboard_mode: String,
    /// Exact authored source-space translation bits.
    pub(super) translation_bits: [u32; 3],
    /// Packed authored AARRGGBB colour.
    pub(super) colour: u32,
    /// Exact authored UV-corner bits in source order.
    pub(super) uv_bits: [[u32; 2]; 4],
    /// Exact authored width bits.
    pub(super) width_bits: u32,
    /// Exact authored height bits.
    pub(super) height_bits: u32,
    /// Exact authored camera-distance bits.
    pub(super) distance_bits: u32,
    /// Exact authored UV-offset bits.
    pub(super) uv_offset_bits: [u32; 2],
    /// Source schema version of the optional display-info child.
    pub(super) display_info_version: Option<u32>,
    /// Exact authored display-rotation bits in WXYZ order.
    pub(super) rotation_wxyz_bits: [u32; 4],
    /// Authored display cutoff mode.
    pub(super) cutoff_mode: String,
    /// Exact authored UV-offset-range bits.
    pub(super) uv_offset_range_bits: [u32; 2],
    /// Exact authored source-range bits.
    pub(super) source_range_bits: u32,
    /// Exact authored edge-range bits.
    pub(super) edge_range_bits: u32,
    /// Source schema version of the optional perspective-info child.
    pub(super) perspective_info_version: Option<u32>,
    /// Authored perspective-scaling flag.
    pub(super) perspective: bool,
}

/// One exact shader parameter retained for a deferred billboard occurrence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct DeferredShaderParameterBinding {
    /// Authored parameter value kind.
    pub(super) kind: String,
    /// Authored parameter token.
    pub(super) param: String,
    /// Exact decoded JSON value.
    pub(super) value: Value,
}

/// One same-name shader occurrence retained without selecting it as
/// authoritative.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct DeferredShaderOccurrenceBinding {
    /// Stable phase-three package-member identity for this occurrence.
    pub(super) package_member_id: String,
    /// Exact normalized shader member id.
    pub(super) member_id: String,
    /// Exact source component ordinal.
    pub(super) source_ordinal: usize,
    /// Optional decoded schema marker.
    pub(super) schema: Option<String>,
    /// Authored logical shader identity.
    pub(super) identity: String,
    /// Decoded source shader version.
    pub(super) version: u32,
    /// Optional platform shader identity exactly as decoded.
    pub(super) platform_shader_name: Option<String>,
    /// Optional authored binary translucency flag.
    pub(super) translucency: Option<u32>,
    /// Optional authored vertex-needs mask.
    pub(super) vertex_needs: Option<u32>,
    /// Optional authored vertex mask.
    pub(super) vertex_mask: Option<u32>,
    /// Optional validated source parameter count.
    pub(super) parameter_count: Option<u32>,
    /// Canonical source texture token when declared.
    pub(super) texture_reference: Option<String>,
    /// Shader parameters in authored source order.
    pub(super) params: Vec<DeferredShaderParameterBinding>,
}

/// One preferred physical texture occurrence retained without choosing it.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct DeferredTextureOccurrenceBinding {
    /// Generated owning package identity.
    pub(super) package_id: String,
    /// Generated owning package subcategory.
    pub(super) subcategory: String,
    /// Stable phase-three package-member identity for this occurrence.
    pub(super) package_member_id: String,
    /// Exact normalized texture member identity.
    pub(super) member_id: String,
    /// Exact source texture component ordinal.
    pub(super) source_ordinal: usize,
    /// Exact lowercase SHA-256 of the normalized PNG payload.
    pub(super) sha256: String,
}

/// One logical shader texture reference and every preferred physical source.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct DeferredTextureReferenceBinding {
    /// Canonical decoded shader texture token.
    pub(super) identity: String,
    /// Every physical occurrence in the existing preferred authority scope.
    pub(super) occurrences: Vec<DeferredTextureOccurrenceBinding>,
}

/// One exact source billboard group retained as deferred presentation evidence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct DeferredBillboardBinding {
    /// Decoded source group schema version.
    pub(super) version: u32,
    /// Authored shader identity.
    pub(super) shader_identity: String,
    /// Every same-name normalized shader occurrence in source-ordinal order.
    pub(super) shader_occurrences: Vec<DeferredShaderOccurrenceBinding>,
    /// Logical texture references and preferred physical source occurrences.
    pub(super) texture_references: Vec<DeferredTextureReferenceBinding>,
    /// Authored depth-test flag.
    pub(super) z_test: u32,
    /// Authored depth-write flag.
    pub(super) z_write: u32,
    /// Authored fog flag.
    pub(super) fog: u32,
    /// Child quads in authored source order.
    pub(super) quads: Vec<DeferredBillboardQuadBinding>,
}

/// One exact source controller binding for deferred render evidence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct DeferredControllerBinding {
    /// Authored controller identity.
    pub(super) controller_identity: String,
    /// Normalized controller component family.
    pub(super) controller_kind: String,
    /// Stable phase-three package-member identity for the controller.
    pub(super) controller_package_member_id: String,
    /// Exact normalized controller member id.
    pub(super) controller_member_id: String,
    /// Exact source controller component ordinal.
    pub(super) controller_source_ordinal: usize,
    /// Decoded source controller schema version.
    pub(super) controller_version: usize,
    /// Decoded source controller type.
    pub(super) controller_type: String,
    /// Exact finite source frame-offset bits.
    pub(super) frame_offset_bits: u32,
    /// Authored animation identity declared by the controller.
    pub(super) animation_identity: String,
    /// Stable phase-three package-member identity for the animation.
    pub(super) animation_package_member_id: Option<String>,
    /// Exact normalized animation member id when resolvable.
    pub(super) animation_member_id: Option<String>,
    /// Exact source animation component ordinal when resolvable.
    pub(super) animation_source_ordinal: Option<usize>,
    /// Decoded source animation schema version when resolvable.
    pub(super) animation_version: Option<usize>,
    /// Decoded source animation type when resolvable.
    pub(super) animation_type: Option<String>,
    /// Strict validated BQG source payload without runtime interpretation.
    pub(super) animation_source: Option<Value>,
}

/// One composite render binding retained when the current FBX route cannot
/// faithfully represent the referenced non-mesh component.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct DeferredRenderBinding {
    /// Zero-based authored position in the composite prop array.
    pub(super) composite_prop_index: usize,
    /// Authored referenced render identity.
    pub(super) source_identity: String,
    /// Authored skeleton joint index for the render member.
    pub(super) skeleton_joint_id: usize,
    /// Authored translucency flag.
    pub(super) is_translucent: bool,
    /// Exact authored optional sort order at source f32 width.
    pub(super) sort_order_bits: Option<u32>,
    /// Exact resolved normalized component family when available.
    pub(super) component_kind: Option<String>,
    /// Stable phase-three package-member identity when available.
    pub(super) component_package_member_id: Option<String>,
    /// Exact resolved normalized component member id when available.
    pub(super) component_member_id: Option<String>,
    /// Exact source component ordinal when available.
    pub(super) source_ordinal: Option<usize>,
    /// Exact source billboard presentation evidence when available.
    pub(super) billboard: Option<DeferredBillboardBinding>,
    /// Exact source controller and animation relationship when available.
    pub(super) controller: Option<DeferredControllerBinding>,
}

/// Source-backed ordering used for selected primary world meshes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum WorldPrimaryMeshOrder {
    /// Non-composite static meshes follow ledger source ordinals.
    SourceOrdinal,
    /// Composite-backed meshes follow authored composite prop order.
    CompositeProp,
}

impl WorldPrimaryMeshOrder {
    /// Stable catalog label for the authored ordering rule.
    #[must_use]
    pub(super) const fn as_str(self) -> &'static str {
        match self {
            Self::SourceOrdinal => "source_ordinal",
            Self::CompositeProp => "composite_prop",
        }
    }
}

/// One exact primary world component occurrence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct WorldPrimaryMemberBinding {
    /// Stable phase-three package-member identity.
    pub(super) package_member_id: String,
    /// Exact normalized component member id.
    pub(super) member_id: String,
    /// Exact source component ordinal.
    pub(super) source_ordinal: usize,
}

/// One selected primary mesh plus its authored composite relationship.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct WorldPrimarySelectedMeshBinding {
    /// Exact physical selected mesh occurrence.
    pub(super) member: WorldPrimaryMemberBinding,
    /// Authored composite prop position when selected through a composite.
    pub(super) composite_prop_index: Option<usize>,
    /// Authored skeleton joint when selected through a composite.
    pub(super) skeleton_joint_id: Option<usize>,
    /// Authored translucency when selected through a composite.
    pub(super) is_translucent: Option<bool>,
    /// Exact authored optional sort order at source f32 width.
    pub(super) sort_order_bits: Option<u32>,
}

/// Exact same-package particle resources that share one effect identity.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct WorldPrimaryParticlePairBinding {
    /// Exact particle-system factory occurrence.
    pub(super) factory: WorldPrimaryMemberBinding,
    /// Exact particle-system occurrence whose factory name matches the effect.
    pub(super) system: WorldPrimaryMemberBinding,
}

/// One authored composite effect with bounded runtime particle interpretation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct WorldPrimaryEffectBinding {
    /// Zero-based authored effect position.
    pub(super) composite_effect_index: usize,
    /// Exact authored effect identity.
    pub(super) source_identity: String,
    /// Authored skeleton joint index.
    pub(super) skeleton_joint_id: usize,
    /// Authored translucency flag.
    pub(super) is_translucent: bool,
    /// Exact authored optional sort order at source f32 width.
    pub(super) sort_order_bits: Option<u32>,
    /// Exact same-package factory/system pair when uniquely source-backed.
    pub(super) package_particle_pair: Option<WorldPrimaryParticlePairBinding>,
    /// Exact particle-system entity selected by the shipped `tEffect` lookup.
    pub(super) runtime_effect_system: Option<WorldPrimaryMemberBinding>,
}

/// Exact physical provenance for the primary world-model route.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct WorldPrimarySourceBinding {
    /// Owning world or physics source occurrence.
    pub(super) owner: WorldPrimaryMemberBinding,
    /// Authored ordering rule used for selected meshes.
    pub(super) mesh_order: WorldPrimaryMeshOrder,
    /// Selected mesh occurrences and authored composite bindings, if any.
    pub(super) selected_meshes: Vec<WorldPrimarySelectedMeshBinding>,
    /// Authored effect relationships on the matched composite.
    pub(super) composite_effects: Vec<WorldPrimaryEffectBinding>,
    /// Composite occurrence that authored mesh selection when present.
    pub(super) matched_composite: Option<WorldPrimaryMemberBinding>,
    /// Authored skeleton relationship when present, even on a static route.
    pub(super) referenced_skeleton: Option<WorldPrimaryMemberBinding>,
    /// Exact PTRN animation occurrence consumed by the animated route.
    pub(super) exported_ptrn_animation: Option<WorldPrimaryMemberBinding>,
}

/// One normalized source occurrence before semantic deduplication.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct PropCandidate {
    /// Prop catalog family.
    pub(super) family: PropFamily,
    /// Generated package-index identity.
    pub(super) package_id: String,
    /// Generated package subcategory.
    pub(super) subcategory: String,
    /// Package root relative to the normalized staging directory.
    pub(super) relative_root: PathBuf,
    /// Top-level container kind or direct component family.
    pub(super) owner_kind: String,
    /// Human-readable source owner identity.
    pub(super) owner_name: String,
    /// Stable container key inside the package.
    pub(super) container_key: String,
    /// Selected decoded mesh member ids without family or extension.
    pub(super) mesh_ids: Vec<String>,
    /// Exact primary world source provenance when this is a world candidate.
    pub(super) world_primary_source: Option<WorldPrimarySourceBinding>,
    /// Source-backed non-mesh composite render bindings deferred from FBX.
    pub(super) deferred_render_bindings: Vec<DeferredRenderBinding>,
    /// Composite member id for one rigid animated route.
    pub(super) composite_id: Option<String>,
    /// Skeleton member id for one rigid animated route.
    pub(super) skeleton_id: Option<String>,
    /// Exact PTRN animation member id for one rigid animated route.
    pub(super) animation_id: Option<String>,
    /// Justified static or rigid-animation representation.
    pub(super) route: PropRoute,
}

impl Ord for PropCandidate {
    fn cmp(&self, other: &Self) -> Ordering {
        (
            self.family,
            &self.package_id,
            &self.owner_name,
            &self.container_key,
        )
            .cmp(&(
                other.family,
                &other.package_id,
                &other.owner_name,
                &other.container_key,
            ))
    }
}

impl PartialOrd for PropCandidate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// One source occurrence retained as provenance for a deduplicated asset.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct PropAlias {
    /// Source package id.
    pub(super) package_id: String,
    /// Source package subcategory.
    pub(super) subcategory: String,
    /// Top-level container kind.
    pub(super) owner_kind: String,
    /// Human-readable source owner identity.
    pub(super) owner_name: String,
    /// Stable source container key.
    pub(super) container_key: String,
    /// Exact primary world source provenance when this is a world occurrence.
    pub(super) world_primary_source: Option<WorldPrimarySourceBinding>,
    /// Source-backed non-mesh composite render bindings deferred from FBX.
    pub(super) deferred_render_bindings: Vec<DeferredRenderBinding>,
}

impl From<&PropCandidate> for PropAlias {
    fn from(candidate: &PropCandidate) -> Self {
        Self {
            package_id: candidate.package_id.clone(),
            subcategory: candidate.subcategory.clone(),
            owner_kind: candidate.owner_kind.clone(),
            owner_name: candidate.owner_name.clone(),
            container_key: candidate.container_key.clone(),
            world_primary_source: candidate.world_primary_source.clone(),
            deferred_render_bindings:
                candidate.deferred_render_bindings.clone(),
        }
    }
}

/// One referenced external texture written beside an FBX.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct TextureRecord {
    /// Portable file name under the asset texture directory.
    pub(super) file_name: String,
    /// Exact byte count.
    pub(super) bytes: u64,
    /// Lowercase SHA-256.
    pub(super) sha256: String,
}

/// One deduplicated and published model asset.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct ExportedProp {
    /// Stable content-derived asset id.
    pub(super) asset_id: String,
    /// Prop catalog family.
    pub(super) family: PropFamily,
    /// Static or rigid-animation representation.
    pub(super) route: PropRoute,
    /// Semantic model signature used for deduplication.
    pub(super) signature: String,
    /// Relative FBX path from catalog root.
    pub(super) fbx_path: String,
    /// FBX byte count.
    pub(super) fbx_bytes: u64,
    /// FBX SHA-256.
    pub(super) fbx_sha256: String,
    /// Binary object-family summary.
    pub(super) summary: CharacterBinaryFbxSummary,
    /// Referenced texture records in file-name order.
    pub(super) textures: Vec<TextureRecord>,
    /// Canonical source and all duplicate occurrences.
    pub(super) aliases: Vec<PropAlias>,
}

/// Aggregate counters emitted by one complete batch.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(super) struct PropCatalogCounts {
    /// Source packages re-extracted from the game tree.
    pub(super) source_packages: usize,
    /// Model-bearing occurrences before deduplication.
    pub(super) occurrences: usize,
    /// Unique FBX assets after semantic deduplication.
    pub(super) assets: usize,
    /// Unique card-package FBX assets.
    pub(super) card_assets: usize,
    /// Unique mission FBX assets.
    pub(super) mission_assets: usize,
    /// Static unique assets.
    pub(super) static_assets: usize,
    /// Rigid animated unique assets.
    pub(super) animated_assets: usize,
}
