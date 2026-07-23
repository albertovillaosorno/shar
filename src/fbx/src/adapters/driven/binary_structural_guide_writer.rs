// File:
//   - binary_structural_guide_writer.rs
// Path:
//   - src/fbx/src/adapters/driven/binary_structural_guide_writer.rs
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
//   - The validated one-mesh, one-material structural-guide FBX adapter.
// - Must-Not:
//   - Pack atlas pixels, read source assets, add helpers, or infer placement.
// - Allows:
//   - Four fixed UV channels, an optional source-owned normal layer, FBX 7.7
//     encoding, and atomic create-new persistence.
// - Summary:
//   - Writes the canonical Unreal structural-guide interchange mesh.
//
// ADRs:
// - docs/adr/fbx/export/fbx-output-contract-boundary.md
// - docs/adr/pipeline/unreal/world-assembly-from-normalized-chunks.md
//
// LARGE-FILE:
// - owner: Structural-guide FBX adapter
// - reason: Public payload, validation, typed errors, and atomic write form one
//   externally consumed serialization boundary.
// - split: Document and node construction live in responsibility-focused child
//   modules.
// - validation: fbx strict validation and structural-guide round-trip tests.
// - review: Split when another structural-guide container format is added.
//

//! Canonical one-mesh structural-guide FBX writer.

use std::fs::OpenOptions;
use std::io::{ErrorKind, Write as _};
use std::path::Path;

use super::binary_fbx::encode_binary_document;

mod document;
mod geometry;
mod nodes;

/// Exact structural-guide object and asset identity.
pub const STRUCTURAL_GUIDE_ASSET_NAME: &str =
    "SM_SHAR_StructuralGuide_Canonical";
/// Exact single material identity.
pub const STRUCTURAL_GUIDE_MATERIAL_NAME: &str = "M_SHAR_StructuralGuide_Atlas";
/// Exact external atlas path written into the FBX.
pub const STRUCTURAL_GUIDE_TEXTURE_PATH: &str =
    "textures/T_SHAR_StructuralGuide_Atlas.png";
/// Exact UV channel names in import order.
pub const STRUCTURAL_GUIDE_UV_NAMES: [&str; 4] = [
    "UV0_Atlas",
    "UV1_Source",
    "UV2_AtlasOffset",
    "UV3_AtlasScale",
];

/// One fully baked structural-guide mesh.
#[derive(Clone, Debug, PartialEq)]
pub struct StructuralGuideMesh {
    /// Post-world-FBX positions in source meters.
    pub positions: Vec<[f32; 3]>,
    /// Source-owned normalized normals aligned with positions. Empty omits
    /// the FBX normal layer when any combined source group lacks normals.
    pub normals: Vec<[f32; 3]>,
    /// Triangles using 32-bit indices.
    pub triangles: Vec<[u32; 3]>,
    /// Final atlas UV values aligned with positions and usable as imported
    /// UV0.
    pub atlas_uvs: Vec<[f32; 2]>,
    /// Original source UV values aligned with positions for audit only.
    pub source_uvs: Vec<[f32; 2]>,
    /// Atlas useful-texel offsets aligned with positions for audit only.
    pub atlas_offsets: Vec<[f32; 2]>,
    /// Atlas useful-texel scales aligned with positions for audit only.
    pub atlas_scales: Vec<[f32; 2]>,
}

/// Verified result of one structural-guide write.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StructuralGuideFbxSummary {
    /// Vertices written to the single geometry object.
    pub vertices: usize,
    /// Triangles written to the single geometry object.
    pub triangles: usize,
    /// Minimum bounds in post-world-FBX source meters.
    pub bounds_min_meters: [f32; 3],
    /// Maximum bounds in post-world-FBX source meters.
    pub bounds_max_meters: [f32; 3],
}

/// Structural-guide validation, encoding, or persistence failure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StructuralGuideFbxError {
    /// Mesh did not contain positions or triangles.
    EmptyMesh,
    /// One aligned vertex channel used a different length.
    ChannelLengthMismatch {
        /// Channel identity.
        channel: &'static str,
        /// Required position count.
        positions: usize,
        /// Actual channel count.
        values: usize,
    },
    /// One floating-point component was non-finite.
    NonFiniteValue {
        /// Channel identity.
        channel: &'static str,
        /// Element index.
        element: usize,
        /// Component axis.
        axis: usize,
    },
    /// One normal was zero-length or materially non-unit.
    InvalidNormal {
        /// Rejected normal index.
        vertex: usize,
    },
    /// One triangle referenced a missing vertex.
    IndexOutOfBounds {
        /// Rejected index.
        index: u32,
        /// Available positions.
        positions: usize,
    },
    /// One triangle repeated a vertex index.
    DegenerateTriangle {
        /// Rejected triangle ordinal.
        triangle: usize,
    },
    /// Binary FBX document construction or encoding failed.
    Encoding(String),
    /// Output path had no parent directory.
    MissingParent(String),
    /// Output artifact already existed.
    OutputExists(String),
    /// Parent directory creation or file persistence failed.
    Write {
        /// Affected path.
        path: String,
        /// Stable filesystem diagnostic.
        source: String,
    },
}

/// Validate and write one external-texture FBX 7.7 structural guide.
///
/// # Errors
///
/// Returns a typed error when mesh invariants, FBX encoding, or create-new
/// persistence fails.
pub fn write_binary_structural_guide_fbx(
    mesh: &StructuralGuideMesh,
    path: &Path,
) -> Result<StructuralGuideFbxSummary, StructuralGuideFbxError> {
    let summary = validate_mesh(mesh)?;
    let nodes = document::build_document(mesh)?;
    let bytes = encode_binary_document(&nodes).map_err(
        |error| StructuralGuideFbxError::Encoding(format!("{error:?}")),
    )?;
    persist(
        path, &bytes,
    )?;
    Ok(summary)
}

/// Validate one complete mesh and calculate exact bounds.
fn validate_mesh(
    mesh: &StructuralGuideMesh,
) -> Result<StructuralGuideFbxSummary, StructuralGuideFbxError> {
    if mesh
        .positions
        .is_empty()
        || mesh
            .triangles
            .is_empty()
    {
        return Err(StructuralGuideFbxError::EmptyMesh);
    }
    if !mesh
        .normals
        .is_empty()
        && mesh
            .normals
            .len()
            != mesh
                .positions
                .len()
    {
        return Err(
            StructuralGuideFbxError::ChannelLengthMismatch {
                channel: "normal",
                positions: mesh
                    .positions
                    .len(),
                values: mesh
                    .normals
                    .len(),
            },
        );
    }
    for (name, values) in [
        (
            STRUCTURAL_GUIDE_UV_NAMES[0],
            mesh.atlas_uvs
                .len(),
        ),
        (
            STRUCTURAL_GUIDE_UV_NAMES[1],
            mesh.source_uvs
                .len(),
        ),
        (
            STRUCTURAL_GUIDE_UV_NAMES[2],
            mesh.atlas_offsets
                .len(),
        ),
        (
            STRUCTURAL_GUIDE_UV_NAMES[3],
            mesh.atlas_scales
                .len(),
        ),
    ] {
        if values
            != mesh
                .positions
                .len()
        {
            return Err(
                StructuralGuideFbxError::ChannelLengthMismatch {
                    channel: name,
                    positions: mesh
                        .positions
                        .len(),
                    values,
                },
            );
        }
    }
    validate_finite3(
        "position",
        &mesh.positions,
    )?;
    validate_finite3(
        "normal",
        &mesh.normals,
    )?;
    validate_finite2(
        STRUCTURAL_GUIDE_UV_NAMES[0],
        &mesh.atlas_uvs,
    )?;
    validate_finite2(
        STRUCTURAL_GUIDE_UV_NAMES[1],
        &mesh.source_uvs,
    )?;
    validate_finite2(
        STRUCTURAL_GUIDE_UV_NAMES[2],
        &mesh.atlas_offsets,
    )?;
    validate_finite2(
        STRUCTURAL_GUIDE_UV_NAMES[3],
        &mesh.atlas_scales,
    )?;
    for (vertex, normal) in mesh
        .normals
        .iter()
        .enumerate()
    {
        let length_squared = normal
            .iter()
            .map(|value| value * value)
            .sum::<f32>();
        if !(0.999..=1.001).contains(&length_squared) {
            return Err(
                StructuralGuideFbxError::InvalidNormal {
                    vertex,
                },
            );
        }
    }
    for (ordinal, triangle) in mesh
        .triangles
        .iter()
        .enumerate()
    {
        let [
            first,
            second,
            third,
        ] = *triangle;
        if first == second || first == third || second == third {
            return Err(
                StructuralGuideFbxError::DegenerateTriangle {
                    triangle: ordinal,
                },
            );
        }
        for index in triangle {
            if usize::try_from(*index).map_or(
                true,
                |value| {
                    value
                        >= mesh
                            .positions
                            .len()
                },
            ) {
                return Err(
                    StructuralGuideFbxError::IndexOutOfBounds {
                        index: *index,
                        positions: mesh
                            .positions
                            .len(),
                    },
                );
            }
        }
    }
    let mut bounds_min = mesh.positions[0];
    let mut bounds_max = mesh.positions[0];
    for position in mesh
        .positions
        .iter()
        .skip(1)
    {
        for axis in 0..3 {
            bounds_min[axis] = bounds_min[axis].min(position[axis]);
            bounds_max[axis] = bounds_max[axis].max(position[axis]);
        }
    }
    Ok(
        StructuralGuideFbxSummary {
            vertices: mesh
                .positions
                .len(),
            triangles: mesh
                .triangles
                .len(),
            bounds_min_meters: bounds_min,
            bounds_max_meters: bounds_max,
        },
    )
}

fn validate_finite3(
    channel: &'static str,
    values: &[[f32; 3]],
) -> Result<(), StructuralGuideFbxError> {
    for (element, value) in values
        .iter()
        .enumerate()
    {
        if let Some(axis) = value
            .iter()
            .position(|component| !component.is_finite())
        {
            return Err(
                StructuralGuideFbxError::NonFiniteValue {
                    channel,
                    element,
                    axis,
                },
            );
        }
    }
    Ok(())
}

fn validate_finite2(
    channel: &'static str,
    values: &[[f32; 2]],
) -> Result<(), StructuralGuideFbxError> {
    for (element, value) in values
        .iter()
        .enumerate()
    {
        if let Some(axis) = value
            .iter()
            .position(|component| !component.is_finite())
        {
            return Err(
                StructuralGuideFbxError::NonFiniteValue {
                    channel,
                    element,
                    axis,
                },
            );
        }
    }
    Ok(())
}

fn persist(path: &Path, bytes: &[u8]) -> Result<(), StructuralGuideFbxError> {
    let parent = path
        .parent()
        .ok_or_else(
            || {
                StructuralGuideFbxError::MissingParent(
                    path.display()
                        .to_string(),
                )
            },
        )?;
    std::fs::create_dir_all(parent).map_err(
        |source| StructuralGuideFbxError::Write {
            path: parent
                .display()
                .to_string(),
            source: source.to_string(),
        },
    )?;
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(
            |source| {
                if source.kind() == ErrorKind::AlreadyExists {
                    StructuralGuideFbxError::OutputExists(
                        path.display()
                            .to_string(),
                    )
                } else {
                    StructuralGuideFbxError::Write {
                        path: path
                            .display()
                            .to_string(),
                        source: source.to_string(),
                    }
                }
            },
        )?;
    file.write_all(bytes)
        .map_err(
            |source| StructuralGuideFbxError::Write {
                path: path
                    .display()
                    .to_string(),
                source: source.to_string(),
            },
        )
}
