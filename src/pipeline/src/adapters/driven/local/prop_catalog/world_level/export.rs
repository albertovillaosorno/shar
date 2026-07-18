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
//   - Master-world mesh preparation, review-layer assembly, and FBX write.
// - Must-Not:
//   - Select source packages or serialize the root catalog.
// - Allows:
//   - Static analysis freezing and a separated definition-only gallery.
// - Summary:
//   - Publishes one separated static master-world FBX for all seven levels.
//
// ADRs:
// - docs/adr/pipeline/unreal/world-assembly-from-normalized-chunks.md
// - docs/adr/fbx/export/fbx-output-contract-boundary.md
//
// Large file:
//   - true
//   - Reason: one master assembly transaction keeps mesh, collision, material,
//     gallery, and artifact counts atomic until a second consumer exists.
//

//! Separated master-world static analysis FBX assembly.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use fbx::adapters::driven::binary_character_writer::write_binary_model_fbx;
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
use super::collision::{COLLISION_MATERIAL, load_intersect_meshes};
use super::coordinate::PackageCoordinates;
use super::inventory::{
    LevelMeshSource, is_direct_world_mesh, is_interior, package_meshes,
};
use super::model::{ExportedWorldMaster, LevelPackageRecord};
use super::transform::{bake_mesh, identity, mesh_bounds, translation};
use crate::domain::PipelineError;
use crate::domain::package::PhaseThreePackageRow;

/// Mutable content maps shared by all packages in one level.
#[derive(Default)]
struct MasterContent {
    /// Fully baked authored scene and collision meshes.
    meshes: Vec<MeshAsset>,
    /// Definition-only meshes awaiting similarity-overlaid review placement.
    review: Vec<ReviewMesh>,
    /// Content-derived material bindings.
    materials: BTreeMap<String, MaterialBinding>,
    /// Content-derived texture payloads.
    textures: BTreeMap<String, PreparedTexture>,
    /// Canonical package records.
    packages: Vec<LevelPackageRecord>,
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
struct ReviewMesh {
    mesh: MeshAsset,
    profile: ShapeProfile,
}

/// Coarse normalized shape evidence used only for review co-location.
#[derive(Clone, Copy)]
struct ShapeProfile {
    vertices: usize,
    triangles: usize,
    extents: [f32; 3],
}

/// Aggregate package counters for the completed master scene.
struct MasterTotals {
    /// Recovered source mesh count.
    source_meshes: usize,
    /// Repeated-index degenerate triangle count discarded for analysis.
    discarded_degenerate_triangles: usize,
    /// Authored world placement count.
    authored_placements: usize,
    /// Placements supplied by connected-map references.
    reference_placements: usize,
    /// Placements retaining canonical matrices as fallback.
    canonical_placement_fallbacks: usize,
    /// Direct meshes using topology-verified reference coordinates.
    reference_coordinate_meshes: usize,
    /// Direct meshes retaining canonical coordinates as fallback.
    canonical_coordinate_meshes: usize,
    /// Definition-only meshes placed in similarity-overlaid review groups.
    review_definitions: usize,
    /// Collision review mesh count.
    collision_meshes: usize,
    /// Collision meshes using topology-verified reference coordinates.
    reference_collision_meshes: usize,
    /// Repeated-index collision triangles discarded.
    discarded_collision_triangles: usize,
    /// Explicit interior package count.
    interior_packages: usize,
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

/// Export one separated master-world scene in canonical level order.
///
/// # Errors
///
/// Returns an error when package loading, coordinate joins, collision recovery,
/// review placement, material planning, texture publication, or FBX writing
/// fails.
pub(super) fn export_master_scene(
    packages: &BTreeMap<String, Vec<&PhaseThreePackageRow>>,
    canonical_root: &Path,
    coordinate_root: &Path,
    reference_packages: &BTreeSet<String>,
    scratch_root: &Path,
    output_root: &Path,
    authority: &SharedTextureAuthority,
) -> Result<ExportedWorldMaster, PipelineError> {
    let texture_root = output_root.join("textures");
    fs::create_dir_all(&texture_root).map_err(
        |error| {
            PipelineError::new(
                format!("world master texture directory failed: {error}"),
            )
        },
    )?;
    let mut content = MasterContent::default();
    let collision_material = MaterialBinding::new(
        COLLISION_MATERIAL,
        None,
    )
    .map_err(
        |error| {
            PipelineError::new(
                format!("world master collision material failed: {error:?}"),
            )
        },
    )?;
    let _previous = content
        .materials
        .insert(
            COLLISION_MATERIAL.to_owned(),
            collision_material,
        );
    for (level, rows) in packages {
        let context = PackageAppendContext {
            canonical_root,
            coordinate_root,
            reference_packages,
            scratch_root: scratch_root.join(format!("level-{level}")),
            authority,
        };
        for package in rows {
            append_package(
                level,
                package,
                &context,
                &mut content,
            )?;
        }
    }
    let review_similarity_groups = place_review_gallery(&mut content)?;
    content
        .meshes
        .sort_by(
            |left, right| {
                left.name
                    .cmp(&right.name)
            },
        );
    ensure_unique_names(&content.meshes)?;
    retain_used_presentation(&mut content);
    let texture_records = publish_textures(
        &content.textures,
        &texture_root,
    )?;
    let fbx_path = output_root.join("world-master.fbx");
    let summary = write_binary_model_fbx(
        "world-master",
        &content.meshes,
        &content
            .materials
            .into_values()
            .collect::<Vec<_>>(),
        &fbx_path,
    )
    .map_err(
        |error| {
            PipelineError::new(
                format!("world master FBX write failed: {error:?}"),
            )
        },
    )?;
    let fbx_bytes = fs::read(&fbx_path).map_err(
        |error| {
            PipelineError::new(format!("world master FBX read failed: {error}"))
        },
    )?;
    let totals = master_totals(&content.packages);
    Ok(
        ExportedWorldMaster {
            fbx_path: "world-master.fbx".to_owned(),
            fbx_bytes: u64::try_from(fbx_bytes.len()).map_err(
                |error| {
                    PipelineError::new(
                        format!(
                            "world master FBX byte count overflowed: {error}"
                        ),
                    )
                },
            )?,
            fbx_sha256: digest_hex(&fbx_bytes),
            summary,
            textures: texture_records,
            packages: content.packages,
            source_levels: packages.len(),
            source_meshes: totals.source_meshes,
            discarded_degenerate_triangles: totals
                .discarded_degenerate_triangles,
            authored_placements: totals.authored_placements,
            reference_placements: totals.reference_placements,
            canonical_placement_fallbacks: totals.canonical_placement_fallbacks,
            reference_coordinate_meshes: totals.reference_coordinate_meshes,
            canonical_coordinate_meshes: totals.canonical_coordinate_meshes,
            review_definitions: totals.review_definitions,
            review_similarity_groups,
            collision_meshes: totals.collision_meshes,
            reference_collision_meshes: totals.reference_collision_meshes,
            discarded_collision_triangles: totals.discarded_collision_triangles,
            interior_packages: totals.interior_packages,
        },
    )
}

/// Load and append every recovered mesh from one normalized package.
fn append_package(
    level: &str,
    package: &PhaseThreePackageRow,
    context: &PackageAppendContext<'_>,
    content: &mut MasterContent,
) -> Result<(), PipelineError> {
    let relative = relative_art_root(package)?;
    let package_root = context
        .canonical_root
        .join(&relative);
    let reference_root = context
        .reference_packages
        .contains(&package.package_id)
        .then(
            || {
                context
                    .coordinate_root
                    .join(&relative)
            },
        );
    let sources = package_meshes(&package_root)?;
    let package_index = content
        .packages
        .len();
    content
        .packages
        .push(
            LevelPackageRecord {
                level: level.to_owned(),
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
                collision_meshes: 0,
                reference_collision_meshes: 0,
                discarded_collision_triangles: 0,
                interior_packages: usize::from(is_interior(package)),
            },
        );
    let collisions = load_intersect_meshes(
        &package_root,
        reference_root.as_deref(),
        &package.package_id,
    )?;
    {
        let package_record = content
            .packages
            .get_mut(package_index)
            .ok_or_else(
                || PipelineError::new("world master package record is missing"),
            )?;
        package_record.collision_meshes = collisions
            .meshes
            .len();
        package_record.reference_collision_meshes =
            collisions.reference_coordinate_meshes;
        package_record.discarded_collision_triangles =
            collisions.discarded_triangles;
    }
    content
        .meshes
        .extend(collisions.meshes);
    if sources.is_empty() {
        return Ok(());
    }
    let package_scratch = context
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
    let package_record = content
        .packages
        .get_mut(package_index)
        .ok_or_else(
            || PipelineError::new("world master package record is missing"),
        )?;
    package_record.coordinate_reference = coordinates.uses_reference;
    package_record.discarded_degenerate_triangles =
        discarded_degenerate_triangles;
    let (materials, textures) = canonicalize_world_static_materials(
        &mut meshes,
        &package_root,
        &package_scratch,
        context.authority,
        &package.subcategory,
    )?;
    merge_materials(
        &mut content.materials,
        materials,
    )?;
    merge_textures(
        &mut content.textures,
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
            content,
        )?;
    }
    if package_scratch.is_dir() {
        fs::remove_dir_all(&package_scratch).map_err(
            |error| {
                PipelineError::new(
                    format!("world master material cleanup failed: {error}"),
                )
            },
        )?;
    }
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
                                "world master mesh {} failed: {error:?}",
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
                                    "world master discarded triangle ",
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
                &format!("placed-{ordinal:04}"),
            )?,
        )?;
        content
            .meshes
            .push(placed);
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
                "review-definition",
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
            suffix,
        )?,
    )?;
    content
        .meshes
        .push(mesh);
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
    packages: &mut [LevelPackageRecord],
    package_index: usize,
    counter: PackageCounter,
) -> Result<(), PipelineError> {
    let package = packages
        .get_mut(package_index)
        .ok_or_else(
            || PipelineError::new("world master package record is missing"),
        )?;
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
        .ok_or_else(
            || PipelineError::new("world master package count overflowed"),
        )?;
    Ok(())
}

/// Aggregate package counters for one level result.
fn master_totals(packages: &[LevelPackageRecord]) -> MasterTotals {
    MasterTotals {
        source_meshes: packages
            .iter()
            .map(|package| package.source_meshes)
            .sum(),
        discarded_degenerate_triangles: packages
            .iter()
            .map(|package| package.discarded_degenerate_triangles)
            .sum(),
        authored_placements: packages
            .iter()
            .map(|package| package.authored_placements)
            .sum(),
        reference_placements: packages
            .iter()
            .map(|package| package.reference_placements)
            .sum(),
        canonical_placement_fallbacks: packages
            .iter()
            .map(|package| package.canonical_placement_fallbacks)
            .sum(),
        reference_coordinate_meshes: packages
            .iter()
            .map(|package| package.reference_coordinate_meshes)
            .sum(),
        canonical_coordinate_meshes: packages
            .iter()
            .map(|package| package.canonical_coordinate_meshes)
            .sum(),
        review_definitions: packages
            .iter()
            .map(|package| package.review_definitions)
            .sum(),
        collision_meshes: packages
            .iter()
            .map(|package| package.collision_meshes)
            .sum(),
        reference_collision_meshes: packages
            .iter()
            .map(|package| package.reference_collision_meshes)
            .sum(),
        discarded_collision_triangles: packages
            .iter()
            .map(|package| package.discarded_collision_triangles)
            .sum(),
        interior_packages: packages
            .iter()
            .map(|package| package.interior_packages)
            .sum(),
    }
}

/// Place definition-only meshes in deterministic similarity-overlaid groups.
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
        let group = representatives
            .iter()
            .position(
                |representative| {
                    shape_similarity(
                        item.profile,
                        *representative,
                    ) >= 0.5
                },
            )
            .unwrap_or_else(
                || {
                    representatives.push(item.profile);
                    representatives.len() - 1
                },
            );
        assignments.push(group);
    }
    let world_high = content
        .meshes
        .iter()
        .map(mesh_bounds)
        .fold(
            [0.0_f32; 3],
            |mut high, (_low, item_high)| {
                for axis in 0..3 {
                    high[axis] = high[axis].max(item_high[axis]);
                }
                high
            },
        );
    let cell = representatives
        .iter()
        .flat_map(
            |profile| {
                [
                    profile.extents[0],
                    profile.extents[2],
                ]
            },
        )
        .fold(
            10.0_f32,
            f32::max,
        )
        + 20.0;
    let columns = square_columns(representatives.len());
    let review = std::mem::take(&mut content.review);
    for (mut item, group) in review
        .into_iter()
        .zip(assignments)
    {
        let column = group % columns;
        let row = group / columns;
        let target = [
            world_high[0] + cell * (column as f32 + 1.0),
            0.0,
            world_high[2] + cell * (row as f32 + 1.0),
        ];
        let (low, high) = mesh_bounds(&item.mesh);
        let centre = [
            (low[0] + high[0]) * 0.5,
            low[1],
            (low[2] + high[2]) * 0.5,
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
        bake_mesh(
            &mut item.mesh,
            &translation(
                [
                    target[0] - centre[0],
                    target[1] - centre[1],
                    target[2] - centre[2],
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
    Ok(
        ShapeProfile {
            vertices,
            triangles,
            extents: [
                (high[0] - low[0])
                    .abs()
                    .max(0.001),
                (high[1] - low[1])
                    .abs()
                    .max(0.001),
                (high[2] - low[2])
                    .abs()
                    .max(0.001),
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
        / components.len() as f32
}

/// Return one symmetric ratio in the inclusive zero-to-one range.
fn count_ratio(
    left: usize,
    right: usize,
) -> f32 {
    value_ratio(
        left as f32,
        right as f32,
    )
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
fn square_columns(groups: usize) -> usize {
    let mut columns = 1_usize;
    while columns.saturating_mul(columns) < groups {
        columns = columns.saturating_add(1);
    }
    columns
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
        "world master scene mesh has no portable identity",
    )
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
                            "world master material identity conflicts: {}",
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
                            "world master texture identity conflicts: {}",
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
                        "world master repeats mesh name {}",
                        mesh.name
                    ),
                ),
            );
        }
    }
    if meshes.is_empty() {
        return Err(PipelineError::new("world master has no render meshes"));
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
                    format!("world master texture write failed: {error}"),
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
