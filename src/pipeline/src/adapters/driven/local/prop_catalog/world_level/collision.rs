// File:
//   - collision.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/prop_catalog/world_level/
//     collision.rs
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
//   - Original intersect topology and optional coordinate-only transplantation.
// - Must-Not:
//   - Copy reference topology, materials, UVs, names, or write FBX.
// - Allows:
//   - Count validation, repeated-index sanitation, and position replacement
//     when
//   - canonical and reference topology match exactly.
// - Summary:
//   - Publishes filterable master-world collision review meshes.
//
// ADRs:
// - docs/adr/pipeline/unreal/world-assembly-from-normalized-chunks.md
// - docs/adr/fbx/export/fbx-output-contract-boundary.md
//
// Large file:
//   - false
//

//! Collision review meshes using original topology and optional reference
//! coordinates.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use fbx::domain::mesh::{MeshAsset, PrimitiveGroup};
use serde_json::Value;

use super::super::inventory_common::portable_asset_name;
use crate::domain::PipelineError;

/// Canonical untextured collision material identity.
pub(super) const COLLISION_MATERIAL: &str = "material-collision";

/// One package's collision surfaces and sanitation accounting.
pub(super) struct CollisionBatch {
    /// Canonical collision meshes prepared for FBX publication.
    pub(super) meshes: Vec<MeshAsset>,
    /// Meshes using topology-verified coordinate-reference positions.
    pub(super) reference_coordinate_meshes: usize,
    /// Repeated-index triangles discarded from canonical topology.
    pub(super) discarded_triangles: usize,
}

/// Decoded intersect DSG document needed for safe coordinate transplantation.
#[derive(Clone)]
struct IntersectDocument {
    /// Decoded source schema identity.
    schema: String,
    /// Declared source index count.
    num_indices: usize,
    /// Canonical collision triangle indices.
    indices: Vec<u32>,
    /// Declared source position count.
    num_positions: usize,
    /// Canonical or coordinate-reference positions.
    positions: Vec<[f32; 3]>,
}

/// Load canonical collision topology and use matching reference positions only.
pub(super) fn load_intersect_meshes(
    canonical_root: &Path,
    reference_root: Option<&Path>,
    package_id: &str,
) -> Result<CollisionBatch, PipelineError> {
    let canonical = intersect_documents(canonical_root)?;
    let reference = reference_root.map_or_else(
        || Ok(BTreeMap::new()),
        intersect_documents,
    )?;
    let mut meshes = Vec::with_capacity(canonical.len());
    let mut reference_coordinate_meshes = 0_usize;
    let mut discarded_triangles = 0_usize;
    for (stem, canonical_document) in canonical {
        let reference_positions = reference
            .get(&stem)
            .filter(
                |document| {
                    topology_matches(
                        &canonical_document,
                        document,
                    )
                },
            )
            .map(
                |document| {
                    document
                        .positions
                        .clone()
                },
            );
        reference_coordinate_meshes = reference_coordinate_meshes
            .checked_add(usize::from(reference_positions.is_some()))
            .ok_or_else(
                || {
                    PipelineError::new(
                        "world collision reference count overflowed",
                    )
                },
            )?;
        let (mesh, discarded) = collision_mesh(
            package_id,
            &stem,
            canonical_document,
            reference_positions,
        )?;
        discarded_triangles = discarded_triangles
            .checked_add(discarded)
            .ok_or_else(
                || {
                    PipelineError::new(
                        "world collision discard count overflowed",
                    )
                },
            )?;
        meshes.push(mesh);
    }
    Ok(
        CollisionBatch {
            meshes,
            reference_coordinate_meshes,
            discarded_triangles,
        },
    )
}

/// Read every intersect document by stable file stem.
fn intersect_documents(
    package_root: &Path
) -> Result<BTreeMap<String, IntersectDocument>, PipelineError> {
    let root = package_root
        .join("components")
        .join("srr_intersect_dsg");
    if !root.is_dir() {
        return Ok(BTreeMap::new());
    }
    let mut paths = fs::read_dir(&root)
        .map_err(
            |error| {
                PipelineError::new(
                    format!(
                        "world collision directory read failed for {}: {error}",
                        root.display()
                    ),
                )
            },
        )?
        .map(
            |entry| {
                entry
                    .map(|value| value.path())
                    .map_err(|error| PipelineError::new(error.to_string()))
            },
        )
        .collect::<Result<Vec<PathBuf>, _>>()?;
    paths.retain(
        |path| {
            path.is_file()
                && path
                    .extension()
                    .and_then(|value| value.to_str())
                    .is_some_and(|value| value.eq_ignore_ascii_case("json"))
        },
    );
    paths.sort();
    let mut documents = BTreeMap::new();
    for path in paths {
        let value: Value = serde_json::from_slice(
            &fs::read(&path)
                .map_err(|error| PipelineError::new(error.to_string()))?,
        )
        .map_err(
            |error| {
                PipelineError::new(
                    format!(
                        "world collision JSON failed for {}: {error}",
                        path.display()
                    ),
                )
            },
        )?;
        let document = parse_document(
            &value, &path,
        )?;
        validate_document(
            &document, &path,
        )?;
        let stem = path
            .file_stem()
            .and_then(|component| component.to_str())
            .ok_or_else(
                || PipelineError::new("world collision file has no UTF-8 stem"),
            )?
            .to_ascii_lowercase();
        if documents
            .insert(
                stem.clone(),
                document,
            )
            .is_some()
        {
            return Err(
                PipelineError::new(
                    format!("world collision stem is ambiguous: {stem}"),
                ),
            );
        }
    }
    Ok(documents)
}

/// Parse one intersect JSON document without adding a direct serde dependency.
fn parse_document(
    value: &Value,
    path: &Path,
) -> Result<IntersectDocument, PipelineError> {
    let schema = value
        .get("schema")
        .and_then(Value::as_str)
        .ok_or_else(
            || {
                PipelineError::new(
                    format!(
                        "world collision schema is missing for {}",
                        path.display()
                    ),
                )
            },
        )?
        .to_owned();
    let num_indices = json_usize(
        value,
        "num_indices",
        path,
    )?;
    let indices = value
        .get("indices")
        .and_then(Value::as_array)
        .ok_or_else(
            || {
                PipelineError::new(
                    format!(
                        "world collision indices are missing for {}",
                        path.display()
                    ),
                )
            },
        )?
        .iter()
        .map(
            |item| {
                item.as_u64()
                    .and_then(|number| u32::try_from(number).ok())
                    .ok_or_else(
                        || {
                            PipelineError::new(
                                format!(
                                    "world collision index is invalid for {}",
                                    path.display()
                                ),
                            )
                        },
                    )
            },
        )
        .collect::<Result<Vec<_>, _>>()?;
    let num_positions = json_usize(
        value,
        "num_positions",
        path,
    )?;
    let positions = value
        .get("positions")
        .and_then(Value::as_array)
        .ok_or_else(
            || {
                PipelineError::new(
                    format!(
                        "world collision positions are missing for {}",
                        path.display()
                    ),
                )
            },
        )?
        .iter()
        .map(
            |item| {
                json_position(
                    item, path,
                )
            },
        )
        .collect::<Result<Vec<_>, _>>()?;
    Ok(
        IntersectDocument {
            schema,
            num_indices,
            indices,
            num_positions,
            positions,
        },
    )
}

/// Read one non-negative JSON integer as usize.
fn json_usize(
    value: &Value,
    key: &str,
    path: &Path,
) -> Result<usize, PipelineError> {
    value
        .get(key)
        .and_then(Value::as_u64)
        .and_then(|number| usize::try_from(number).ok())
        .ok_or_else(
            || {
                PipelineError::new(
                    format!(
                        "world collision {key} is invalid for {}",
                        path.display()
                    ),
                )
            },
        )
}

/// Read one finite three-component collision position.
fn json_position(
    value: &Value,
    path: &Path,
) -> Result<[f32; 3], PipelineError> {
    let components = value
        .as_array()
        .ok_or_else(
            || {
                PipelineError::new(
                    format!(
                        "world collision position is invalid for {}",
                        path.display()
                    ),
                )
            },
        )?;
    if components.len() != 3 {
        return Err(
            PipelineError::new(
                format!(
                    "world collision position width is invalid for {}",
                    path.display()
                ),
            ),
        );
    }
    let mut position = [0.0_f32; 3];
    for (axis, component) in components
        .iter()
        .enumerate()
    {
        let source_number = component
            .as_f64()
            .ok_or_else(
                || {
                    PipelineError::new(
                        format!(
                            "invalid world collision position component for {}",
                            path.display()
                        ),
                    )
                },
            )?;
        let number = checked_position_component(
            source_number,
            path,
        )?;
        let target = position
            .get_mut(axis)
            .ok_or_else(
                || {
                    PipelineError::new(
                        "world collision position axis overflowed",
                    )
                },
            )?;
        *target = number;
    }
    Ok(position)
}

/// Narrow one finite JSON number to the collision mesh scalar contract.
#[expect(
    clippy::as_conversions,
    clippy::cast_possible_truncation,
    reason = "The result is immediately checked for finite f32 range before \
              use."
)]
fn checked_position_component(
    value: f64,
    path: &Path,
) -> Result<f32, PipelineError> {
    let narrowed = value as f32;
    if !narrowed.is_finite() {
        return Err(
            PipelineError::new(
                format!(
                    "world collision position exceeds f32 range for {}",
                    path.display()
                ),
            ),
        );
    }
    Ok(narrowed)
}

/// Return whether reference positions can be transplanted without topology
/// drift.
fn topology_matches(
    canonical: &IntersectDocument,
    reference: &IntersectDocument,
) -> bool {
    canonical.indices == reference.indices
        && canonical
            .positions
            .len()
            == reference
                .positions
                .len()
}

/// Build one FBX-ready collision mesh from canonical topology.
fn collision_mesh(
    package_id: &str,
    stem: &str,
    canonical: IntersectDocument,
    reference_positions: Option<Vec<[f32; 3]>>,
) -> Result<
    (
        MeshAsset,
        usize,
    ),
    PipelineError,
> {
    let (indices, discarded) = sanitized_indices(&canonical.indices)?;
    let positions = reference_positions.unwrap_or(canonical.positions);
    let name = portable_asset_name(
        &format!("collision-{package_id}-{stem}"),
        120,
        "world collision mesh has no portable identity",
    )?;
    let group = PrimitiveGroup::new(
        0,
        COLLISION_MATERIAL,
        positions,
        Vec::new(),
        &indices,
    )
    .map_err(
        |error| {
            PipelineError::new(
                format!(
                    "world collision topology failed for {name}: {error:?}"
                ),
            )
        },
    )?;
    let mesh = MeshAsset::new(
        name,
        vec![group],
    )
    .map_err(
        |error| {
            PipelineError::new(
                format!("world collision mesh construction failed: {error:?}"),
            )
        },
    )?;
    Ok(
        (
            mesh, discarded,
        ),
    )
}

/// Validate schema and declared array counts before topology conversion.
fn validate_document(
    document: &IntersectDocument,
    path: &Path,
) -> Result<(), PipelineError> {
    if document.schema != "intersect_dsg" {
        return Err(
            PipelineError::new(
                format!(
                    "world collision schema mismatch for {}: {}",
                    path.display(),
                    document.schema
                ),
            ),
        );
    }
    if document.num_indices
        != document
            .indices
            .len()
        || document.num_positions
            != document
                .positions
                .len()
        || document
            .indices
            .is_empty()
        || document
            .positions
            .is_empty()
        || !document
            .indices
            .len()
            .is_multiple_of(3)
    {
        return Err(
            PipelineError::new(
                format!(
                    "world collision declared counts are invalid for {}",
                    path.display()
                ),
            ),
        );
    }
    Ok(())
}

/// Remove repeated-index triangles while preserving every valid triangle.
fn sanitized_indices(
    indices: &[u32]
) -> Result<
    (
        Vec<u32>,
        usize,
    ),
    PipelineError,
> {
    let (triangles, remainder) = indices.as_chunks::<3>();
    if !remainder.is_empty() {
        return Err(
            PipelineError::new(
                "world collision index count is not divisible by three",
            ),
        );
    }
    let mut retained = Vec::with_capacity(indices.len());
    let mut discarded = 0_usize;
    for triangle in triangles {
        let first = triangle[0];
        let second = triangle[1];
        let third = triangle[2];
        if first == second || first == third || second == third {
            discarded = discarded
                .checked_add(1)
                .ok_or_else(
                    || PipelineError::new("world collision discard overflowed"),
                )?;
            continue;
        }
        retained.extend_from_slice(triangle);
    }
    if retained.is_empty() {
        return Err(
            PipelineError::new(
                "world collision surface has no valid triangles",
            ),
        );
    }
    Ok(
        (
            retained, discarded,
        ),
    )
}

#[cfg(test)]
mod tests {
    use super::{IntersectDocument, topology_matches};

    #[test]
    fn coordinate_reference_requires_exact_original_topology() {
        let canonical = IntersectDocument {
            schema: "intersect_dsg".to_owned(),
            num_indices: 3,
            indices: vec![
                0, 1, 2,
            ],
            num_positions: 3,
            positions: vec![
                [0.0; 3], [1.0; 3], [2.0; 3],
            ],
        };
        let mut moved = canonical.clone();
        moved.positions = vec![
            [10.0; 3], [11.0; 3], [12.0; 3],
        ];
        assert!(
            topology_matches(
                &canonical, &moved
            )
        );
        moved.indices = vec![
            0, 2, 1,
        ];
        assert!(
            !topology_matches(
                &canonical, &moved
            )
        );
    }
}
