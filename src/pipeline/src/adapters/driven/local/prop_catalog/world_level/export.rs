// File:
//   - export.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/prop_catalog/world_level/export.rs
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
//   - Per-package world preparation, fused-interior ownership, review
//     isolation, and FBX writes.
// - Must-Not:
//   - Select source packages, infer transforms, merge non-Halloween level
//     ownership, or serialize the root catalog.
// - Allows:
//   - Baked shared-origin geometry, geometry-key deduplication, additive
//     Halloween overlays, collision inspection, and isolated review galleries.
// - Summary:
//   - Publishes exterior packages and fused interior FBX artifacts.
//
// ADRs:
// - docs/adr/pipeline/unreal/world-assembly-from-normalized-chunks.md
// - docs/adr/fbx/export/fbx-output-contract-boundary.md
//
// Large file:
//   - true
//   - Reason: one package-collection transaction keeps coordinate joins,
//     collision recovery, material variants, review isolation, and writes
//     consistent.
//   - Split: extract package publication when another world consumer exists.
//   - Validation: canonical pipeline validation plus world-level tests.
//   - Review: required whenever another assembly responsibility is added.
//

//! Globally aligned world-package static analysis FBX assembly.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use fbx::adapters::driven::binary_character_writer::{
    ModelUvPolicy, write_binary_model_fbx_with_uv_policy,
};
use fbx::adapters::driven::decoded_component_source::read_mesh_for_analysis;
use fbx::domain::mesh::MeshAsset;
use fbx::domain::texture::MaterialBinding;
use shar_sha256::digest_hex;

use super::super::extraction::relative_art_root;
use super::super::inventory_common::portable_asset_name;
use super::super::material::canonicalize_world_static_materials;
use super::super::model::TextureRecord;
use super::super::prepared::PreparedTexture;
use super::super::texture_authority::SharedTextureAuthority;
use super::collision::load_intersect_meshes;
use super::coordinate::PackageCoordinates;
use super::interior::{
    InteriorGeometryOwnership, InteriorIdentity, identity_for_package,
    is_halloween_package, package_level, retain_unowned_triangles,
};
use super::inventory::{
    LevelMeshSource, is_direct_world_mesh, is_interior, object_role,
    package_meshes, package_scope,
};
use super::islands::split_distant_islands;
use super::layout::{
    MapBounds, apply_placement, collection_bounds, placement_for_scope,
    record_group_bounds, validate_group_bounds,
};
use super::model::{
    ExportedWorldCollection, WorldFbxRecord, WorldInteriorRecord,
    WorldPackageRecord, WorldSurfaceSemanticCounts,
};
use super::movement::apply_package_movement;
use super::movement_model::WorldCoordinateMovementRecord;
use super::transform::{bake_mesh, identity, mesh_bounds, translation};
use crate::domain::PipelineError;
use crate::domain::package::PhaseThreePackageRow;

/// Mutable content maps shared by all packages in one level.
#[derive(Clone, Default)]
struct MasterContent {
    /// Fully baked authored scene meshes; collision evidence is excluded.
    meshes: Vec<MeshAsset>,
    /// Definition-only meshes awaiting similarity-overlaid review placement.
    review: Vec<ReviewMesh>,
    /// Content-derived material bindings.
    materials: BTreeMap<String, MaterialBinding>,
    /// Content-derived texture payloads.
    textures: BTreeMap<String, PreparedTexture>,
    /// Canonical package records.
    packages: Vec<WorldPackageRecord>,
}

/// Immutable source paths and authorities used while appending packages.
struct PackageAppendContext<'sources> {
    /// Freshly extracted canonical game package root.
    canonical_root: &'sources Path,
    /// Coordinate-only reference package root.
    coordinate_root: &'sources Path,
    /// Package identities with usable coordinate references.
    reference_packages: &'sources BTreeSet<String>,
    /// Per-level material staging root.
    scratch_root: PathBuf,
    /// Cross-package texture authority.
    authority: &'sources SharedTextureAuthority,
}

/// One definition-only mesh awaiting review-layer placement.
#[derive(Clone)]
struct ReviewMesh {
    /// Canonical definition-only mesh preserved as a separate review object.
    mesh: MeshAsset,
    /// Coarse shape profile used only for review co-location.
    profile: ShapeProfile,
}

/// One transformed interior package awaiting fused publication.
struct PendingInterior {
    /// Stable semantic interior family.
    identity: InteriorIdentity,
    /// Narrative source level.
    level: u8,
    /// Whether this package contributes only Halloween additions.
    halloween: bool,
    /// Package provenance and counters.
    record: WorldPackageRecord,
    /// Transformed render content and presentation authority.
    content: MasterContent,
}

/// Coarse normalized shape evidence used only for review co-location.
#[derive(Clone, Copy)]
struct ShapeProfile {
    /// Total source vertex count.
    vertices: usize,
    /// Total source triangle count.
    triangles: usize,
    /// Positive axis extents in source units.
    extents: [f32; 3],
}

/// Package counter selected for checked assembly accounting.
#[derive(Clone, Copy)]
enum PackageCounter {
    /// Scenegraph placement supplied by the connected-map reference.
    ReferencePlacement,
    /// Scenegraph placement retaining a canonical matrix as fallback.
    CanonicalPlacement,
    /// Direct mesh using topology-verified reference coordinates.
    ReferenceCoordinate,
    /// Direct mesh retaining canonical coordinates as fallback.
    CanonicalCoordinate,
    /// Definition-only mesh routed to the separated review gallery.
    ReviewDefinition,
}

/// Export independently importable world-package scenes at one shared origin.
///
/// Main scene files are written directly below the output root so selecting all
/// root `*.fbx` files in Blender preserves their authored global coordinates.
/// Definition-only review galleries are isolated below `review/` and are not
/// part of the normal world import set.
///
/// # Errors
///
/// Returns an error when package loading, coordinate joins, collision
/// exclusion, material planning, texture publication, or any FBX write fails.
#[expect(
    clippy::too_many_lines,
    reason = "Collection publication keeps package and review FBXs atomic."
)]
pub(super) fn export_world_collection(
    packages: &[&PhaseThreePackageRow],
    canonical_root: &Path,
    coordinate_root: &Path,
    reference_packages: &BTreeSet<String>,
    scratch_root: &Path,
    output_root: &Path,
    authority: &SharedTextureAuthority,
) -> Result<ExportedWorldCollection, PipelineError> {
    let texture_root = output_root.join("textures");
    let review_root = output_root.join("review");
    let review_texture_root = review_root.join("textures");
    for directory in [
        &texture_root,
        &review_root,
    ] {
        fs::create_dir_all(directory).map_err(
            |error| {
                PipelineError::new(
                    format!("world package output directory failed: {error}"),
                )
            },
        )?;
    }

    let mut records = Vec::with_capacity(packages.len());
    let mut pending_interiors = Vec::<PendingInterior>::new();
    let mut coordinate_movements = Vec::<WorldCoordinateMovementRecord>::new();
    let mut all_textures = BTreeMap::<String, PreparedTexture>::new();
    let mut aggregate_semantics = WorldSurfaceSemanticCounts::default();
    let mut map_bounds = BTreeMap::<&'static str, MapBounds>::new();
    let mut has_review_fbx = false;
    let mut used_file_names = BTreeSet::new();

    for package in packages {
        let scope = package_scope(package)?;
        let mut package_content = MasterContent::default();
        let append_context = PackageAppendContext {
            canonical_root,
            coordinate_root,
            reference_packages,
            scratch_root: scratch_root.to_path_buf(),
            authority,
        };
        if let Some(movement) = append_package(
            &scope,
            package,
            &append_context,
            &mut package_content,
        )? {
            coordinate_movements.push(movement);
        }
        merge_textures(
            &mut all_textures,
            package_content
                .textures
                .values()
                .cloned()
                .collect(),
        )?;
        let mut record = package_content
            .packages
            .pop()
            .ok_or_else(
                || PipelineError::new("world package record is missing"),
            )?;
        let placement = placement_for_scope(&scope)?;
        apply_placement(
            &mut package_content.meshes,
            placement,
        )?;
        if !record.interior {
            record_group_bounds(
                &mut map_bounds,
                placement,
                collection_bounds(&package_content.meshes),
            );
            record.map_group = placement
                .group
                .map(str::to_owned);
            record.map_offset = placement.offset;
        }
        let (independent, breakable, interactable) =
            world_object_semantic_counts(&package_content.meshes);
        record.independent_item_geometries = independent;
        record.breakable_geometries = breakable;
        record.interactable_geometries = interactable;
        let stem = package_file_stem(package)?;
        if !used_file_names.insert(stem.clone()) {
            return Err(
                PipelineError::new(
                    format!("world package FBX identity repeats: {stem}"),
                ),
            );
        }

        let review = std::mem::take(&mut package_content.review);
        let review_materials = package_content
            .materials
            .clone();
        let review_textures = package_content
            .textures
            .clone();
        if !review.is_empty() {
            let mut review_content = MasterContent {
                meshes: Vec::new(),
                review,
                materials: review_materials,
                textures: review_textures,
                packages: Vec::new(),
            };
            record.review_similarity_groups =
                place_review_gallery(&mut review_content)?;
            record.review_fbx = write_content_fbx(
                &format!("{stem}-review"),
                &format!("review/{stem}.review.fbx"),
                &mut review_content,
                output_root,
                if record.interior {
                    ModelUvPolicy::Preserve
                } else {
                    ModelUvPolicy::Selective
                },
            )?;
            if let Some(artifact) = record
                .review_fbx
                .as_ref()
            {
                aggregate_semantics.add(artifact.surface_semantics);
                has_review_fbx = true;
            }
        }

        if record.interior {
            let identity = identity_for_package(&record.package_id)
                .ok_or_else(
                    || {
                        PipelineError::new(
                            format!(
                                "world interior identity is missing: {}",
                                record.package_id
                            ),
                        )
                    },
                )?;
            let level = package_level(&record.package_id).ok_or_else(
                || {
                    PipelineError::new(
                        format!(
                            "world interior level is missing: {}",
                            record.package_id
                        ),
                    )
                },
            )?;
            pending_interiors.push(
                PendingInterior {
                    identity,
                    level,
                    halloween: is_halloween_package(&record.package_id),
                    record,
                    content: package_content,
                },
            );
            continue;
        }

        let world_relative_path = format!("{stem}.fbx");
        record.world_fbx = write_content_fbx(
            &stem,
            &world_relative_path,
            &mut package_content,
            output_root,
            ModelUvPolicy::Selective,
        )?;
        if let Some(artifact) = record
            .world_fbx
            .as_ref()
        {
            aggregate_semantics.add(artifact.surface_semantics);
        }
        records.push(record);
    }
    validate_group_bounds(&map_bounds)?;

    let (interiors, interior_records, interior_semantics) =
        publish_fused_interiors(
            pending_interiors,
            output_root,
        )?;
    aggregate_semantics.add(interior_semantics);
    records.extend(interior_records);
    records.sort_by(
        |left, right| {
            left.package_id
                .cmp(&right.package_id)
        },
    );

    let textures = publish_textures(
        &all_textures,
        &texture_root,
    )?;
    if has_review_fbx {
        fs::create_dir_all(&review_texture_root).map_err(
            |error| {
                PipelineError::new(
                    format!("world review texture directory failed: {error}"),
                )
            },
        )?;
        let _review_records = publish_textures(
            &all_textures,
            &review_texture_root,
        )?;
    }
    Ok(
        ExportedWorldCollection {
            packages: records,
            interiors,
            coordinate_movements,
            textures,
            surface_semantics: aggregate_semantics,
        },
    )
}

/// Publish eight fused base interiors and four additive Halloween overlays.
#[expect(
    clippy::too_many_lines,
    reason = "one atomic transaction owns fused geometry, presentation, and \
              writes"
)]
fn publish_fused_interiors(
    pending: Vec<PendingInterior>,
    output_root: &Path,
) -> Result<
    (
        Vec<WorldInteriorRecord>,
        Vec<WorldPackageRecord>,
        WorldSurfaceSemanticCounts,
    ),
    PipelineError,
> {
    let mut groups = BTreeMap::<InteriorIdentity, Vec<PendingInterior>>::new();
    for package in pending {
        groups
            .entry(package.identity)
            .or_default()
            .push(package);
    }
    let mut interiors = Vec::with_capacity(groups.len());
    let mut records = Vec::new();
    let mut semantics = WorldSurfaceSemanticCounts::default();
    for (identity, mut packages) in groups {
        packages.sort_by(
            |left, right| {
                left.level
                    .cmp(&right.level)
                    .then_with(
                        || {
                            left.record
                                .package_id
                                .cmp(
                                    &right
                                        .record
                                        .package_id,
                                )
                        },
                    )
            },
        );
        let source_package_ids = packages
            .iter()
            .map(
                |package| {
                    package
                        .record
                        .package_id
                        .clone()
                },
            )
            .collect::<Vec<_>>();
        let base_source_package_ids = packages
            .iter()
            .filter(|package| !package.halloween)
            .map(
                |package| {
                    package
                        .record
                        .package_id
                        .clone()
                },
            )
            .collect::<Vec<_>>();
        let halloween_source_package_ids = packages
            .iter()
            .filter(|package| package.halloween)
            .map(
                |package| {
                    package
                        .record
                        .package_id
                        .clone()
                },
            )
            .collect::<Vec<_>>();
        if base_source_package_ids.is_empty() {
            return Err(
                PipelineError::new(
                    format!(
                        "interior base packages are missing: {}",
                        identity.id
                    ),
                ),
            );
        }
        if identity.halloween_overlay == halloween_source_package_ids.is_empty()
        {
            return Err(
                PipelineError::new(
                    format!(
                        "interior Halloween ownership changed: {}",
                        identity.id
                    ),
                ),
            );
        }

        let mut base = MasterContent::default();
        let mut overlay = MasterContent::default();
        let mut owned_geometry = InteriorGeometryOwnership::default();
        let mut removed_duplicate_triangles = 0_usize;
        for package in packages
            .iter_mut()
            .filter(|package| !package.halloween)
        {
            merge_content_presentation(
                &mut base,
                &package.content,
            )?;
            package
                .content
                .meshes
                .sort_by(
                    |left, right| {
                        left.name
                            .cmp(&right.name)
                    },
                );
            for mesh in package
                .content
                .meshes
                .drain(..)
            {
                let (retained, removed) = retain_unowned_triangles(
                    mesh,
                    &mut owned_geometry,
                )?;
                removed_duplicate_triangles = removed_duplicate_triangles
                    .checked_add(removed)
                    .ok_or_else(
                        || {
                            PipelineError::new(
                                "interior duplicate triangle count overflowed",
                            )
                        },
                    )?;
                if let Some(retained_mesh) = retained {
                    base.meshes
                        .push(retained_mesh);
                }
            }
        }
        for package in packages
            .iter_mut()
            .filter(|package| package.halloween)
        {
            merge_content_presentation(
                &mut overlay,
                &package.content,
            )?;
            package
                .content
                .meshes
                .sort_by(
                    |left, right| {
                        left.name
                            .cmp(&right.name)
                    },
                );
            for mesh in package
                .content
                .meshes
                .drain(..)
            {
                let (retained, removed) = retain_unowned_triangles(
                    mesh,
                    &mut owned_geometry,
                )?;
                removed_duplicate_triangles = removed_duplicate_triangles
                    .checked_add(removed)
                    .ok_or_else(
                        || {
                            PipelineError::new(
                                "interior duplicate triangle count overflowed",
                            )
                        },
                    )?;
                if let Some(retained_mesh) = retained {
                    overlay
                        .meshes
                        .push(retained_mesh);
                }
            }
        }
        let folder = format!(
            "interiors/{}-{}",
            identity.id, identity.name
        );
        let base_name = format!(
            "{}-{}",
            identity.id, identity.name
        );
        let base_fbx = write_content_fbx(
            &base_name,
            &format!("{folder}/{base_name}.fbx"),
            &mut base,
            output_root,
            ModelUvPolicy::Preserve,
        )?
        .ok_or_else(
            || {
                PipelineError::new(
                    format!(
                        "fused interior base is empty: {}",
                        identity.id
                    ),
                )
            },
        )?;
        semantics.add(base_fbx.surface_semantics);
        let halloween_fbx = if identity.halloween_overlay {
            let overlay_name = format!("{base_name}-halloween");
            let artifact = write_content_fbx(
                &overlay_name,
                &format!("{folder}/{overlay_name}.fbx"),
                &mut overlay,
                output_root,
                ModelUvPolicy::Preserve,
            )?
            .ok_or_else(
                || {
                    PipelineError::new(
                        format!(
                            "Halloween interior overlay is empty: {}",
                            identity.id
                        ),
                    )
                },
            )?;
            semantics.add(artifact.surface_semantics);
            Some(artifact)
        } else {
            if !overlay
                .meshes
                .is_empty()
            {
                return Err(
                    PipelineError::new(
                        format!(
                            "ordinary interior produced an overlay: {}",
                            identity.id
                        ),
                    ),
                );
            }
            None
        };
        interiors.push(
            WorldInteriorRecord {
                identity: identity
                    .id
                    .to_owned(),
                name: identity
                    .name
                    .to_owned(),
                source_package_ids,
                base_source_package_ids,
                halloween_source_package_ids,
                removed_duplicate_triangles,
                base_fbx,
                halloween_fbx,
            },
        );
        records.extend(
            packages
                .into_iter()
                .map(|package| package.record),
        );
    }
    Ok(
        (
            interiors, records, semantics,
        ),
    )
}

/// Merge one package's presentation authority into one fused interior artifact.
fn merge_content_presentation(
    target: &mut MasterContent,
    source: &MasterContent,
) -> Result<(), PipelineError> {
    merge_materials(
        &mut target.materials,
        source
            .materials
            .values()
            .cloned()
            .collect(),
    )?;
    merge_textures(
        &mut target.textures,
        source
            .textures
            .values()
            .cloned()
            .collect(),
    )
}
/// Write one non-empty package scene and return its stable artifact record.
fn write_content_fbx(
    scene_name: &str,
    relative_path: &str,
    content: &mut MasterContent,
    output_root: &Path,
    uv_policy: ModelUvPolicy,
) -> Result<Option<WorldFbxRecord>, PipelineError> {
    if content
        .meshes
        .is_empty()
    {
        return Ok(None);
    }
    content
        .meshes
        .sort_by(
            |left, right| {
                left.name
                    .cmp(&right.name)
            },
        );
    ensure_unique_names(&content.meshes)?;
    retain_used_presentation(content);
    let surface_semantics = world_surface_semantics(
        &content.meshes,
        &content.materials,
    )?;
    let path = output_root.join(relative_path);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(
            |error| {
                PipelineError::new(
                    format!("world FBX parent directory failed: {error}"),
                )
            },
        )?;
    }
    let summary = write_binary_model_fbx_with_uv_policy(
        scene_name,
        &content.meshes,
        &content
            .materials
            .values()
            .cloned()
            .collect::<Vec<_>>(),
        uv_policy,
        &path,
    )
    .map_err(
        |error| {
            PipelineError::new(
                format!("world package FBX write failed: {error:?}"),
            )
        },
    )?;
    let bytes = fs::read(&path).map_err(
        |error| {
            PipelineError::new(
                format!("world package FBX read failed: {error}"),
            )
        },
    )?;
    Ok(
        Some(
            WorldFbxRecord {
                path: relative_path.to_owned(),
                bytes: u64::try_from(bytes.len()).map_err(
                    |error| {
                        PipelineError::new(
                            format!(
                                "world package byte count overflowed: {error}"
                            ),
                        )
                    },
                )?,
                sha256: digest_hex(&bytes),
                summary,
                surface_semantics,
            },
        ),
    )
}

/// Build one unique portable file stem from the source package subcategory.
fn package_file_stem(
    package: &PhaseThreePackageRow
) -> Result<String, PipelineError> {
    let relative = package
        .subcategory
        .strip_prefix("terrain-world/")
        .unwrap_or(&package.subcategory)
        .replace(
            '/', "--",
        );
    portable_asset_name(
        &relative,
        110,
        "world package has no portable FBX identity",
    )
}

/// Load and append every recovered mesh from one normalized package.
#[expect(
    clippy::too_many_lines,
    reason = "Canonical geometry, coordinate joins, collision records, \
              material               authority, and package counters must \
              remain one append invariant."
)]
fn append_package(
    level: &str,
    package: &PhaseThreePackageRow,
    append_context: &PackageAppendContext<'_>,
    package_content: &mut MasterContent,
) -> Result<Option<WorldCoordinateMovementRecord>, PipelineError> {
    let relative = relative_art_root(package)?;
    let package_root = append_context
        .canonical_root
        .join(&relative);
    let reference_root = append_context
        .reference_packages
        .contains(&package.package_id)
        .then(
            || {
                append_context
                    .coordinate_root
                    .join(&relative)
            },
        );
    let sources = package_meshes(&package_root)?;
    let package_index = package_content
        .packages
        .len();
    package_content
        .packages
        .push(
            WorldPackageRecord {
                scope: level.to_owned(),
                package_id: package
                    .package_id
                    .clone(),
                subcategory: package
                    .subcategory
                    .clone(),
                coordinate_reference: reference_root.is_some(),
                source_meshes: sources.len(),
                discarded_degenerate_triangles: 0,
                authored_placements: 0,
                reference_placements: 0,
                canonical_placement_fallbacks: 0,
                reference_coordinate_meshes: 0,
                canonical_coordinate_meshes: 0,
                review_definitions: 0,
                independent_item_geometries: 0,
                breakable_geometries: 0,
                interactable_geometries: 0,
                excluded_collision_meshes: 0,
                reference_excluded_collision_meshes: 0,
                discarded_collision_triangles: 0,
                interior: is_interior(package),
                map_group: None,
                map_offset: [
                    0, 0, 0,
                ],
                coordinate_movement: None,
                review_similarity_groups: 0,
                world_fbx: None,
                review_fbx: None,
            },
        );
    let mut collisions = load_intersect_meshes(
        &package_root,
        reference_root.as_deref(),
        &package.package_id,
    )?;
    {
        let package_record = package_content
            .packages
            .get_mut(package_index)
            .ok_or_else(
                || PipelineError::new("world package record is missing"),
            )?;
        package_record.excluded_collision_meshes = collisions
            .meshes
            .len();
        package_record.reference_excluded_collision_meshes =
            collisions.reference_coordinate_meshes;
        package_record.discarded_collision_triangles =
            collisions.discarded_triangles;
    }
    if sources.is_empty() {
        let movement = apply_package_movement(
            level,
            is_interior(package),
            &package.package_id,
            &package_root,
            &mut package_content.meshes,
            &mut collisions.meshes,
        )?;
        record_movement_identity(
            package_content,
            package_index,
            movement.as_ref(),
        )?;
        return Ok(movement);
    }
    let package_scratch = append_context
        .scratch_root
        .join(&package.package_id);
    let (mut meshes, discarded_degenerate_triangles) = load_analysis_meshes(
        &sources,
        &package_root,
    )?;
    let coordinates = PackageCoordinates::resolve(
        &sources,
        &meshes,
        &package_root,
        reference_root.as_deref(),
    )?;
    let package_record = package_content
        .packages
        .get_mut(package_index)
        .ok_or_else(|| PipelineError::new("world package record is missing"))?;
    package_record.coordinate_reference = coordinates.uses_reference;
    package_record.discarded_degenerate_triangles =
        discarded_degenerate_triangles;
    let (materials, textures) = canonicalize_world_static_materials(
        &mut meshes,
        &package_root,
        &package_scratch,
        append_context.authority,
        &package.subcategory,
    )?;
    merge_materials(
        &mut package_content.materials,
        materials,
    )?;
    merge_textures(
        &mut package_content.textures,
        textures,
    )?;
    for (source, mesh) in sources
        .into_iter()
        .zip(meshes)
    {
        append_source_mesh(
            package,
            package_index,
            &source,
            mesh,
            &coordinates,
            package_content,
        )?;
    }
    if package_scratch.is_dir() {
        fs::remove_dir_all(&package_scratch).map_err(
            |error| {
                PipelineError::new(
                    format!("world package material cleanup failed: {error}"),
                )
            },
        )?;
    }
    let movement = apply_package_movement(
        level,
        is_interior(package),
        &package.package_id,
        &package_root,
        &mut package_content.meshes,
        &mut collisions.meshes,
    )?;
    record_movement_identity(
        package_content,
        package_index,
        movement.as_ref(),
    )?;
    Ok(movement)
}

/// Preserve one package movement identity beside its transformed evidence.
fn record_movement_identity(
    content: &mut MasterContent,
    package_index: usize,
    movement: Option<&WorldCoordinateMovementRecord>,
) -> Result<(), PipelineError> {
    let package = content
        .packages
        .get_mut(package_index)
        .ok_or_else(|| PipelineError::new("world package record is missing"))?;
    package.coordinate_movement = movement.map(
        |record| {
            record
                .id
                .clone()
        },
    );
    Ok(())
}

/// Load one package's render meshes under the analysis sanitation policy.
fn load_analysis_meshes(
    sources: &[LevelMeshSource],
    package_root: &Path,
) -> Result<
    (
        Vec<MeshAsset>,
        usize,
    ),
    PipelineError,
> {
    let recovered = sources
        .iter()
        .map(
            |source| {
                read_mesh_for_analysis(
                    package_root,
                    &source.member_id,
                )
                .map_err(
                    |error| {
                        PipelineError::new(
                            format!(
                                "world package mesh {} failed: {error:?}",
                                source.member_id
                            ),
                        )
                    },
                )
            },
        )
        .collect::<Result<Vec<_>, _>>()?;
    let discarded = recovered
        .iter()
        .try_fold(
            0_usize,
            |total, (_mesh, count)| {
                total
                    .checked_add(*count)
                    .ok_or_else(
                        || {
                            PipelineError::new(
                                concat!(
                                    "world package discarded triangle ",
                                    "count overflowed"
                                ),
                            )
                        },
                    )
            },
        )?;
    Ok(
        (
            recovered
                .into_iter()
                .map(|(mesh, _discarded)| mesh)
                .collect(),
            discarded,
        ),
    )
}

/// Route one canonical mesh to explicit placement, direct coordinates, or omit.
fn append_source_mesh(
    package: &PhaseThreePackageRow,
    package_index: usize,
    source: &LevelMeshSource,
    mesh: MeshAsset,
    coordinates: &PackageCoordinates,
    content: &mut MasterContent,
) -> Result<(), PipelineError> {
    let (matrices, reference_placement) = coordinates.placements(source);
    if matrices.is_empty() {
        return append_direct_or_omit(
            package,
            package_index,
            source,
            mesh,
            coordinates,
            content,
        );
    }
    for (ordinal, matrix) in matrices
        .iter()
        .enumerate()
    {
        let mut placed = mesh.clone();
        bake_mesh(
            &mut placed,
            matrix,
            scene_name(
                &package.package_id,
                &source.member_id,
                &role_marked_suffix(
                    &format!("placed-{ordinal:04}"),
                    source,
                    true,
                ),
            )?,
        )?;
        content
            .meshes
            .extend(split_distant_islands(placed)?);
        increment_package_count(
            &mut content.packages,
            package_index,
            if reference_placement {
                PackageCounter::ReferencePlacement
            } else {
                PackageCounter::CanonicalPlacement
            },
        )?;
    }
    Ok(())
}

/// Publish one direct-world canonical mesh or count one unplaced definition.
fn append_direct_or_omit(
    package: &PhaseThreePackageRow,
    package_index: usize,
    source: &LevelMeshSource,
    mut mesh: MeshAsset,
    coordinates: &PackageCoordinates,
    content: &mut MasterContent,
) -> Result<(), PipelineError> {
    if !is_direct_world_mesh(source) {
        bake_mesh(
            &mut mesh,
            &identity(),
            scene_name(
                &package.package_id,
                &source.member_id,
                &role_marked_suffix(
                    "review-definition",
                    source,
                    false,
                ),
            )?,
        )?;
        let profile = shape_profile(&mesh)?;
        content
            .review
            .push(
                ReviewMesh {
                    mesh,
                    profile,
                },
            );
        increment_package_count(
            &mut content.packages,
            package_index,
            PackageCounter::ReviewDefinition,
        )?;
        return Ok(());
    }
    let used_reference = coordinates.apply_direct_reference(
        source, &mut mesh,
    )?;
    let suffix = if used_reference {
        "direct-reference"
    } else {
        "direct-canonical"
    };
    bake_mesh(
        &mut mesh,
        &identity(),
        scene_name(
            &package.package_id,
            &source.member_id,
            &role_marked_suffix(
                suffix, source, false,
            ),
        )?,
    )?;
    content
        .meshes
        .extend(split_distant_islands(mesh)?);
    increment_package_count(
        &mut content.packages,
        package_index,
        if used_reference {
            PackageCounter::ReferenceCoordinate
        } else {
            PackageCounter::CanonicalCoordinate
        },
    )
}

/// Increment one package assembly counter without indexing or overflow.
fn increment_package_count(
    packages: &mut [WorldPackageRecord],
    package_index: usize,
    counter: PackageCounter,
) -> Result<(), PipelineError> {
    let package = packages
        .get_mut(package_index)
        .ok_or_else(|| PipelineError::new("world package record is missing"))?;
    match counter {
        PackageCounter::ReferencePlacement => {
            increment_value(&mut package.authored_placements)?;
            increment_value(&mut package.reference_placements)
        }
        PackageCounter::CanonicalPlacement => {
            increment_value(&mut package.authored_placements)?;
            increment_value(&mut package.canonical_placement_fallbacks)
        }
        PackageCounter::ReferenceCoordinate => {
            increment_value(&mut package.reference_coordinate_meshes)
        }
        PackageCounter::CanonicalCoordinate => {
            increment_value(&mut package.canonical_coordinate_meshes)
        }
        PackageCounter::ReviewDefinition => {
            increment_value(&mut package.review_definitions)
        }
    }
}

/// Increment one assembly counter with checked arithmetic.
fn increment_value(value: &mut usize) -> Result<(), PipelineError> {
    *value = value
        .checked_add(1)
        .ok_or_else(|| PipelineError::new("world package count overflowed"))?;
    Ok(())
}

/// Place definition-only meshes in deterministic similarity-overlaid groups.
#[expect(
    clippy::too_many_lines,
    reason = "Similarity assignment, deterministic grid placement, naming, \
              and baked review geometry are one reproducible gallery \
              operation."
)]
fn place_review_gallery(
    content: &mut MasterContent
) -> Result<usize, PipelineError> {
    if content
        .review
        .is_empty()
    {
        return Ok(0);
    }
    content
        .review
        .sort_by(
            |left, right| {
                left.mesh
                    .name
                    .cmp(
                        &right
                            .mesh
                            .name,
                    )
            },
        );
    let mut representatives = Vec::<ShapeProfile>::new();
    let mut assignments = Vec::with_capacity(
        content
            .review
            .len(),
    );
    for item in &content.review {
        let group = if let Some(group) = representatives
            .iter()
            .position(
                |representative| {
                    shape_similarity(
                        item.profile,
                        *representative,
                    ) >= 0.5_f32
                },
            ) {
            group
        } else {
            representatives.push(item.profile);
            representatives
                .len()
                .checked_sub(1)
                .ok_or_else(
                    || PipelineError::new("world review group underflowed"),
                )?
        };
        assignments.push(group);
    }
    let world_high = content
        .meshes
        .iter()
        .map(mesh_bounds)
        .fold(
            [
                0.0_f32, 0.0_f32, 0.0_f32,
            ],
            |high, (_low, item_high)| {
                let [
                    high_x,
                    high_y,
                    high_z,
                ] = high;
                let [
                    item_x,
                    item_y,
                    item_z,
                ] = item_high;
                [
                    high_x.max(item_x),
                    high_y.max(item_y),
                    high_z.max(item_z),
                ]
            },
        );
    let maximum_extent = representatives
        .iter()
        .flat_map(
            |profile| {
                let [
                    x,
                    _y,
                    z,
                ] = profile.extents;
                [
                    x, z,
                ]
            },
        )
        .fold(
            10.0_f32,
            f32::max,
        );
    let cell = maximum_extent.mul_add(
        1.0_f32, 20.0_f32,
    );
    let columns = square_columns(representatives.len());
    let review = std::mem::take(&mut content.review);
    for (mut item, group) in review
        .into_iter()
        .zip(assignments)
    {
        let column = group
            .checked_rem(columns)
            .ok_or_else(|| PipelineError::new("world review column failed"))?;
        let row = group
            .checked_div(columns)
            .ok_or_else(|| PipelineError::new("world review row failed"))?;
        let column_offset = review_grid_index(column)?.mul_add(
            1.0_f32, 1.0_f32,
        );
        let row_offset = review_grid_index(row)?.mul_add(
            1.0_f32, 1.0_f32,
        );
        let [
            world_x,
            _world_y,
            world_z,
        ] = world_high;
        let target = [
            cell.mul_add(
                column_offset,
                world_x,
            ),
            0.0_f32,
            cell.mul_add(
                row_offset, world_z,
            ),
        ];
        let (low, high) = mesh_bounds(&item.mesh);
        let [
            low_x,
            low_y,
            low_z,
        ] = low;
        let [
            high_x,
            _high_y,
            high_z,
        ] = high;
        let centre = [
            low_x.midpoint(high_x),
            low_y,
            low_z.midpoint(high_z),
        ];
        let final_name = portable_asset_name(
            &format!(
                "{}-review-group-{group:04}",
                item.mesh
                    .name
            ),
            120,
            "world review mesh has no portable identity",
        )?;
        let [
            target_x,
            target_y,
            target_z,
        ] = target;
        let [
            centre_x,
            centre_y,
            centre_z,
        ] = centre;
        bake_mesh(
            &mut item.mesh,
            &translation(
                [
                    target_x - centre_x,
                    target_y - centre_y,
                    target_z - centre_z,
                ],
            ),
            final_name,
        )?;
        content
            .meshes
            .push(item.mesh);
    }
    Ok(representatives.len())
}

/// Convert one small review-grid index without precision loss.
fn review_grid_index(value: usize) -> Result<f32, PipelineError> {
    u16::try_from(value)
        .map(f32::from)
        .map_err(
            |error| {
                PipelineError::new(
                    format!("world review grid index overflowed: {error}"),
                )
            },
        )
}

/// Build one coarse normalized shape profile for review-only co-location.
fn shape_profile(mesh: &MeshAsset) -> Result<ShapeProfile, PipelineError> {
    let vertices = mesh
        .groups
        .iter()
        .map(
            |group| {
                group
                    .positions
                    .len()
            },
        )
        .sum();
    let triangles = mesh
        .groups
        .iter()
        .map(
            |group| {
                group
                    .triangles
                    .len()
            },
        )
        .sum();
    if vertices == 0 || triangles == 0 {
        return Err(
            PipelineError::new(
                format!(
                    "world review mesh has empty geometry: {}",
                    mesh.name
                ),
            ),
        );
    }
    let (low, high) = mesh_bounds(mesh);
    let [
        low_x,
        low_y,
        low_z,
    ] = low;
    let [
        high_x,
        high_y,
        high_z,
    ] = high;
    Ok(
        ShapeProfile {
            vertices,
            triangles,
            extents: [
                (high_x - low_x)
                    .abs()
                    .max(0.001_f32),
                (high_y - low_y)
                    .abs()
                    .max(0.001_f32),
                (high_z - low_z)
                    .abs()
                    .max(0.001_f32),
            ],
        },
    )
}

/// Score coarse shape equivalence without merging or replacing source geometry.
fn shape_similarity(
    left: ShapeProfile,
    right: ShapeProfile,
) -> f32 {
    let components = [
        count_ratio(
            left.vertices,
            right.vertices,
        ),
        count_ratio(
            left.triangles,
            right.triangles,
        ),
        value_ratio(
            left.extents[0],
            right.extents[0],
        ),
        value_ratio(
            left.extents[1],
            right.extents[1],
        ),
        value_ratio(
            left.extents[2],
            right.extents[2],
        ),
    ];
    components
        .into_iter()
        .sum::<f32>()
        / 5.0_f32
}

/// Return one symmetric ratio in the inclusive zero-to-one range.
fn count_ratio(
    left: usize,
    right: usize,
) -> f32 {
    value_ratio(
        review_count_f32(left),
        review_count_f32(right),
    )
}

/// Convert one review-only count to coarse floating-point shape evidence.
#[expect(
    clippy::as_conversions,
    clippy::cast_precision_loss,
    reason = "Review conversion never changes source geometry."
)]
const fn review_count_f32(value: usize) -> f32 {
    value as f32
}

/// Return one symmetric positive-value ratio.
fn value_ratio(
    left: f32,
    right: f32,
) -> f32 {
    left.min(right)
        / left
            .max(right)
            .max(f32::EPSILON)
}

/// Return the smallest positive square width containing every review group.
const fn square_columns(groups: usize) -> usize {
    let mut columns = 1_usize;
    while columns.saturating_mul(columns) < groups {
        columns = columns.saturating_add(1);
    }
    columns
}

/// Count overlapping independently selectable and interaction geometry groups.
fn world_object_semantic_counts(
    meshes: &[MeshAsset]
) -> (
    usize,
    usize,
    usize,
) {
    let mut independent = 0_usize;
    let mut breakable = 0_usize;
    let mut interactable = 0_usize;
    for mesh in meshes {
        let geometries = mesh
            .groups
            .len();
        if mesh
            .name
            .contains("__independent-item")
            || mesh
                .name
                .contains("__independent-object")
        {
            independent = independent.saturating_add(geometries);
        }
        if mesh
            .name
            .contains("__breakable")
        {
            breakable = breakable.saturating_add(geometries);
        }
        if mesh
            .name
            .contains("__interactable")
        {
            interactable = interactable.saturating_add(geometries);
        }
    }
    (
        independent,
        breakable,
        interactable,
    )
}

/// Add one source-backed interaction marker to a scene-name suffix.
fn role_marked_suffix(
    base: &str,
    source: &LevelMeshSource,
    independent_placement: bool,
) -> String {
    object_role(source)
        .suffix()
        .map_or_else(
            || {
                if independent_placement {
                    format!("{base}__independent-item")
                } else {
                    base.to_owned()
                }
            },
            |role| format!("{base}__{role}"),
        )
}

/// Build one unique portable mesh identity for the final master scene.
fn scene_name(
    package_id: &str,
    member_id: &str,
    suffix: &str,
) -> Result<String, PipelineError> {
    portable_asset_name(
        &format!(
            "{}-{member_id}-{suffix}",
            package_id
                .strip_prefix("extracted-art-")
                .unwrap_or(package_id)
        ),
        120,
        "world package scene mesh has no portable identity",
    )
}

/// Count overlapping semantic materials and primitive-group geometries.
fn world_surface_semantics(
    meshes: &[MeshAsset],
    materials: &BTreeMap<String, MaterialBinding>,
) -> Result<WorldSurfaceSemanticCounts, PipelineError> {
    let mut counts = WorldSurfaceSemanticCounts::default();
    for material in materials.values() {
        let semantics = material.semantics;
        counts.transparent_materials = counts
            .transparent_materials
            .saturating_add(usize::from(semantics.is_transparent()));
        counts.glass_materials = counts
            .glass_materials
            .saturating_add(usize::from(semantics.is_glass()));
        counts.mirror_materials = counts
            .mirror_materials
            .saturating_add(usize::from(semantics.is_mirror()));
        counts.reflective_materials = counts
            .reflective_materials
            .saturating_add(usize::from(semantics.is_reflective()));
        counts.light_emitter_materials = counts
            .light_emitter_materials
            .saturating_add(usize::from(semantics.is_light_emitter()));
        counts.visual_effect_materials = counts
            .visual_effect_materials
            .saturating_add(usize::from(semantics.is_visual_effect()));
    }
    for group in meshes
        .iter()
        .flat_map(|mesh| &mesh.groups)
    {
        let semantics = materials
            .get(&group.shader)
            .ok_or_else(
                || {
                    PipelineError::new(
                        format!(
                            "world semantic material is missing: {}",
                            group.shader
                        ),
                    )
                },
            )?
            .semantics;
        counts.transparent_geometries = counts
            .transparent_geometries
            .saturating_add(usize::from(semantics.is_transparent()));
        counts.glass_geometries = counts
            .glass_geometries
            .saturating_add(usize::from(semantics.is_glass()));
        counts.mirror_geometries = counts
            .mirror_geometries
            .saturating_add(usize::from(semantics.is_mirror()));
        counts.reflective_geometries = counts
            .reflective_geometries
            .saturating_add(usize::from(semantics.is_reflective()));
        counts.light_emitter_geometries = counts
            .light_emitter_geometries
            .saturating_add(usize::from(semantics.is_light_emitter()));
        counts.visual_effect_geometries = counts
            .visual_effect_geometries
            .saturating_add(usize::from(semantics.is_visual_effect()));
    }
    Ok(counts)
}

/// Retain only materials and textures referenced by published geometry.
fn retain_used_presentation(content: &mut MasterContent) {
    let used_materials = content
        .meshes
        .iter()
        .flat_map(|mesh| &mesh.groups)
        .map(
            |group| {
                group
                    .shader
                    .clone()
            },
        )
        .collect::<BTreeSet<_>>();
    content
        .materials
        .retain(|name, _binding| used_materials.contains(name));
    let used_textures = content
        .materials
        .values()
        .filter_map(
            |binding| {
                binding
                    .texture_file_name
                    .clone()
            },
        )
        .collect::<BTreeSet<_>>();
    content
        .textures
        .retain(|name, _texture| used_textures.contains(name));
}

/// Merge content-derived material bindings without identity conflicts.
fn merge_materials(
    target: &mut BTreeMap<String, MaterialBinding>,
    materials: Vec<MaterialBinding>,
) -> Result<(), PipelineError> {
    for material in materials {
        match target.get(&material.material_name) {
            Some(existing) if existing != &material => {
                return Err(
                    PipelineError::new(
                        format!(
                            "world package material identity conflicts: {}",
                            material.material_name
                        ),
                    ),
                );
            }
            Some(_) => {}
            None => {
                let _previous = target.insert(
                    material
                        .material_name
                        .clone(),
                    material,
                );
            }
        }
    }
    Ok(())
}

/// Merge content-derived texture payloads without identity conflicts.
fn merge_textures(
    target: &mut BTreeMap<String, PreparedTexture>,
    textures: Vec<PreparedTexture>,
) -> Result<(), PipelineError> {
    for texture in textures {
        match target.get(&texture.file_name) {
            Some(existing) if existing != &texture => {
                return Err(
                    PipelineError::new(
                        format!(
                            "world package texture identity conflicts: {}",
                            texture.file_name
                        ),
                    ),
                );
            }
            Some(_) => {}
            None => {
                let _previous = target.insert(
                    texture
                        .file_name
                        .clone(),
                    texture,
                );
            }
        }
    }
    Ok(())
}

/// Verify the final static scene has unique non-empty mesh identities.
fn ensure_unique_names(meshes: &[MeshAsset]) -> Result<(), PipelineError> {
    let mut names = BTreeSet::new();
    for mesh in meshes {
        if !names.insert(
            mesh.name
                .as_str(),
        ) {
            return Err(
                PipelineError::new(
                    format!(
                        "world package repeats mesh name {}",
                        mesh.name
                    ),
                ),
            );
        }
    }
    if meshes.is_empty() {
        return Err(PipelineError::new("world package has no render meshes"));
    }
    Ok(())
}

/// Publish all level-local external textures and return catalog records.
fn publish_textures(
    textures: &BTreeMap<String, PreparedTexture>,
    output: &Path,
) -> Result<Vec<TextureRecord>, PipelineError> {
    let mut records = Vec::new();
    for texture in textures.values() {
        fs::write(
            output.join(&texture.file_name),
            &texture.bytes,
        )
        .map_err(
            |error| {
                PipelineError::new(
                    format!("world package texture write failed: {error}"),
                )
            },
        )?;
        records.push(
            TextureRecord {
                file_name: texture
                    .file_name
                    .clone(),
                bytes: u64::try_from(
                    texture
                        .bytes
                        .len(),
                )
                .map_err(
                    |error| {
                        PipelineError::new(
                            format!(
                                "world master texture byte count overflowed: \
                                 {error}"
                            ),
                        )
                    },
                )?,
                sha256: texture
                    .sha256
                    .clone(),
            },
        );
    }
    Ok(records)
}
