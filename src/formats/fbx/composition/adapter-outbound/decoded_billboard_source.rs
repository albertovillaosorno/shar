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
//   - Decoded billboard source outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Decoded billboard source outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for fbx.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Decoded billboard source outbound adapter.

use std::fs;
use std::path::Path;

use serde::Deserialize;

use crate::domain::mesh::{MeshAsset, PrimitiveGroup};

/// Exact decoded source evidence for one billboard quad group.
#[derive(Clone, Debug, PartialEq)]
pub struct BillboardSourceEvidence {
    /// Authored group identity after fixed-width NUL removal.
    pub group_identity: String,
    /// Supported source group schema version.
    pub version: u32,
    /// Authored shader identity after fixed-width NUL removal.
    pub shader_identity: String,
    /// Authored depth-test flag.
    pub z_test: u32,
    /// Authored depth-write flag.
    pub z_write: u32,
    /// Authored fog flag.
    pub fog: u32,
    /// Child quads in authored source order.
    pub quads: Vec<BillboardQuadEvidence>,
}

/// Exact decoded source evidence for one billboard child quad.
#[derive(Clone, Debug, PartialEq)]
pub struct BillboardQuadEvidence {
    /// Authored child identity after fixed-width NUL removal.
    pub identity: String,
    /// Supported source child schema version.
    pub version: u32,
    /// Authored billboard orientation mode.
    pub billboard_mode: String,
    /// Authored source-space translation.
    pub translation: [f32; 3],
    /// Packed authored AARRGGBB colour.
    pub colour: u32,
    /// Four authored UV corners in source order.
    pub uvs: [[f32; 2]; 4],
    /// Authored quad width.
    pub width: f32,
    /// Authored quad height.
    pub height: f32,
    /// Authored camera-distance parameter.
    pub distance: f32,
    /// Authored UV offset.
    pub uv_offset: [f32; 2],
    /// Source schema version of the optional display-info child.
    pub display_info_version: Option<u32>,
    /// Authored display rotation in WXYZ order.
    pub rotation_wxyz: [f32; 4],
    /// Authored display cutoff mode.
    pub cutoff_mode: String,
    /// Authored animated UV offset range.
    pub uv_offset_range: [f32; 2],
    /// Authored source-side display range.
    pub source_range: f32,
    /// Authored edge-fade display range.
    pub edge_range: f32,
    /// Source schema version of the optional perspective-info child.
    pub perspective_info_version: Option<u32>,
    /// Whether authored perspective scaling is enabled.
    pub perspective: bool,
}

/// Decode one extracted billboard quad group as exact source evidence.
///
/// # Errors
///
/// Returns an error when source JSON, identity, geometry, or quaternion
/// evidence is missing or inconsistent.
pub fn read_billboard_source_evidence(
    path: &Path,
    requested_id: &str,
) -> Result<BillboardSourceEvidence, DecodedBillboardError> {
    let bytes =
        fs::read(path).map_err(|error| DecodedBillboardError::Read {
            path: path.display().to_string(),
            source: error.to_string(),
        })?;
    let document: QuadGroupDocument =
        serde_json::from_slice(&bytes).map_err(|error| {
            DecodedBillboardError::Parse {
                path: path.display().to_string(),
                source: error.to_string(),
            }
        })?;
    if document.schema != "quad_group" || document.version != 0 {
        return Err(DecodedBillboardError::UnsupportedDocument);
    }
    let group_identity = clean_identity(&document.name)?;
    if !group_identity.eq_ignore_ascii_case(requested_id) {
        return Err(DecodedBillboardError::IdentityMismatch {
            requested: requested_id.to_owned(),
            decoded: group_identity,
        });
    }
    if document.quads.len() != document.num_quads {
        return Err(DecodedBillboardError::QuadCountMismatch {
            declared: document.num_quads,
            actual: document.quads.len(),
        });
    }
    let quads = document
        .quads
        .iter()
        .map(billboard_quad_evidence)
        .collect::<Result<Vec<_>, _>>()?;
    Ok(BillboardSourceEvidence {
        group_identity,
        version: document.version,
        shader_identity: clean_identity(&document.shader)?,
        z_test: document.z_test,
        z_write: document.z_write,
        fog: document.fog,
        quads,
    })
}

/// Decode one extracted billboard quad group as static inspection geometry.
///
/// # Errors
///
/// Returns an error when source JSON, identity, geometry, or quaternion
/// evidence is missing or inconsistent.
pub fn read_billboard_quad_group(
    path: &Path,
    requested_id: &str,
) -> Result<MeshAsset, DecodedBillboardError> {
    let evidence = read_billboard_source_evidence(path, requested_id)?;
    let groups = evidence
        .quads
        .iter()
        .enumerate()
        .map(|(index, quad)| quad_group(index, &evidence.shader_identity, quad))
        .collect::<Result<Vec<_>, _>>()?;
    MeshAsset::new(evidence.group_identity, groups)
        .map_err(|error| DecodedBillboardError::Mesh(format!("{error:?}")))
}

/// Build one four-vertex inspection plane from one source billboard quad.
fn quad_group(
    index: usize,
    shader: &str,
    quad: &BillboardQuadEvidence,
) -> Result<PrimitiveGroup, DecodedBillboardError> {
    let rotation = normalized_quaternion(quad.rotation_wxyz, &quad.identity)?;
    let half_width = quad.width * 0.5;
    let half_height = quad.height * 0.5;
    let local = [
        [-half_width, -half_height, 0.],
        [half_width, -half_height, 0.],
        [half_width, half_height, 0.],
        [-half_width, half_height, 0.],
    ];
    let positions = local
        .map(|value| add(rotate(value, rotation), quad.translation))
        .to_vec();
    let uvs = quad
        .uvs
        .map(|uv| [uv[0] + quad.uv_offset[0], uv[1] + quad.uv_offset[1]])
        .to_vec();
    let normal = rotate([0., 0., 1.], rotation);
    let color = decode_argb(quad.colour);
    let source_identity = quad.identity.clone();
    PrimitiveGroup::new(index, shader, positions, uvs, &[0, 1, 2, 0, 2, 3])
        .and_then(|group| group.with_source_identity(source_identity))
        .and_then(|group| group.with_normals(vec![normal; 4]))
        .and_then(|group| group.with_colors(vec![color; 4]))
        .map_err(|error| DecodedBillboardError::Mesh(format!("{error:?}")))
}

/// Validate and retain one exact billboard child record.
fn billboard_quad_evidence(
    quad: &QuadDocument,
) -> Result<BillboardQuadEvidence, DecodedBillboardError> {
    let identity = clean_identity(&quad.name)?;
    let finite = quad.width.is_finite()
        && quad.height.is_finite()
        && quad.distance.is_finite()
        && quad.source_range.is_finite()
        && quad.edge_range.is_finite()
        && quad.translation.iter().all(|value| value.is_finite())
        && quad.uvs.iter().flatten().all(|value| value.is_finite())
        && quad.uv_offset.iter().all(|value| value.is_finite())
        && quad.uv_offset_range.iter().all(|value| value.is_finite());
    if quad.version != 2 || !finite || quad.width <= 0. || quad.height <= 0. {
        return Err(DecodedBillboardError::InvalidQuad { name: identity });
    }
    let _normalized_rotation =
        normalized_quaternion(quad.rotation_wxyz, &quad.name)?;
    Ok(BillboardQuadEvidence {
        identity,
        version: quad.version,
        billboard_mode: quad.billboard_mode.clone(),
        translation: quad.translation,
        colour: quad.colour,
        uvs: quad.uvs,
        width: quad.width,
        height: quad.height,
        distance: quad.distance,
        uv_offset: quad.uv_offset,
        display_info_version: quad.display_info_version,
        rotation_wxyz: quad.rotation_wxyz,
        cutoff_mode: quad.cutoff_mode.clone(),
        uv_offset_range: quad.uv_offset_range,
        source_range: quad.source_range,
        edge_range: quad.edge_range,
        perspective_info_version: quad.perspective_info_version,
        perspective: quad.perspective,
    })
}

/// Normalize one source WXYZ quaternion or reject unsupported evidence.
fn normalized_quaternion(
    value: [f32; 4],
    name: &str,
) -> Result<[f32; 4], DecodedBillboardError> {
    if value.iter().any(|component| !component.is_finite()) {
        return Err(DecodedBillboardError::InvalidQuad {
            name: clean_identity(name).unwrap_or_else(|_| "quad".to_owned()),
        });
    }
    let length = value
        .iter()
        .map(|component| component * component)
        .sum::<f32>()
        .sqrt();
    if !length.is_finite() || length <= f32::EPSILON {
        return Err(DecodedBillboardError::InvalidQuad {
            name: clean_identity(name).unwrap_or_else(|_| "quad".to_owned()),
        });
    }
    Ok(value.map(|component| component / length))
}

/// Rotate one vector by a unit WXYZ quaternion.
fn rotate(vector: [f32; 3], quaternion: [f32; 4]) -> [f32; 3] {
    let [w, x, y, z] = quaternion;
    let [vx, vy, vz] = vector;
    let tx = 2f32 * z.mul_add(-vy, y * vz);
    let ty = 2f32 * x.mul_add(-vz, z * vx);
    let tz = 2f32 * y.mul_add(-vx, x * vy);
    [
        z.mul_add(-ty, y.mul_add(tz, w.mul_add(tx, vx))),
        x.mul_add(-tz, z.mul_add(tx, w.mul_add(ty, vy))),
        y.mul_add(-tx, x.mul_add(ty, w.mul_add(tz, vz))),
    ]
}

/// Add two three-component vectors.
fn add(left: [f32; 3], right: [f32; 3]) -> [f32; 3] {
    [left[0] + right[0], left[1] + right[1], left[2] + right[2]]
}

/// Decode one PDDI AARRGGBB color into normalized RGBA channels.
fn decode_argb(value: u32) -> [f32; 4] {
    let [alpha, red, green, blue] = value.to_be_bytes();
    [
        f32::from(red) / 255f32,
        f32::from(green) / 255f32,
        f32::from(blue) / 255f32,
        f32::from(alpha) / 255f32,
    ]
}

/// Remove fixed-width source padding while preserving the authored identity.
fn clean_identity(value: &str) -> Result<String, DecodedBillboardError> {
    let clean = value.trim_end_matches('\0');
    if clean.is_empty()
        || clean != clean.trim()
        || clean.chars().any(char::is_control)
    {
        return Err(DecodedBillboardError::InvalidIdentity(value.to_owned()));
    }
    Ok(clean.to_owned())
}

/// Decoded billboard-quad-group JSON document.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct QuadGroupDocument {
    /// Stable decoded schema identity.
    schema: String,
    /// Supported group schema version.
    version: u32,
    /// Authored group identity.
    name: String,
    /// Authored shader identity shared by the group.
    shader: String,
    /// Authored source depth-test flag.
    z_test: u32,
    /// Authored source depth-write flag.
    z_write: u32,
    /// Authored source fog flag.
    fog: u32,
    /// Declared number of child quads.
    num_quads: usize,
    /// Decoded child quad records.
    quads: Vec<QuadDocument>,
}

/// Decoded billboard-quad JSON record.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct QuadDocument {
    /// Authored quad identity.
    name: String,
    /// Supported quad schema version.
    version: u32,
    /// Authored source billboard orientation mode.
    billboard_mode: String,
    /// Authored translation in source coordinates.
    translation: [f32; 3],
    /// Packed source AARRGGBB color.
    colour: u32,
    /// Four authored UV corners.
    uvs: [[f32; 2]; 4],
    /// Authored quad width.
    width: f32,
    /// Authored quad height.
    height: f32,
    /// Authored source camera-distance parameter.
    distance: f32,
    /// Authored UV translation.
    uv_offset: [f32; 2],
    /// Source schema version of the optional display-info child.
    display_info_version: Option<u32>,
    /// Authored display rotation in WXYZ order.
    rotation_wxyz: [f32; 4],
    /// Authored source display cutoff mode.
    cutoff_mode: String,
    /// Authored source UV-offset range.
    uv_offset_range: [f32; 2],
    /// Authored source display range.
    source_range: f32,
    /// Authored source edge-fade range.
    edge_range: f32,
    /// Source schema version of the optional perspective-info child.
    perspective_info_version: Option<u32>,
    /// Authored source perspective-scaling flag.
    perspective: bool,
}

/// Decoded billboard source failure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DecodedBillboardError {
    /// Source JSON file could not be read.
    Read {
        /// Portable source path shown in diagnostics.
        path: String,
        /// Underlying filesystem failure.
        source: String,
    },
    /// Source JSON file could not be decoded.
    Parse {
        /// Portable source path shown in diagnostics.
        path: String,
        /// Underlying JSON failure.
        source: String,
    },
    /// Document schema or version is unsupported.
    UnsupportedDocument,
    /// One authored identity is empty or contains control characters.
    InvalidIdentity(String),
    /// Requested and decoded group identities disagree.
    IdentityMismatch {
        /// Requested component identity.
        requested: String,
        /// Decoded component identity.
        decoded: String,
    },
    /// Declared and decoded child counts disagree.
    QuadCountMismatch {
        /// Declared child count.
        declared: usize,
        /// Decoded child count.
        actual: usize,
    },
    /// One quad contains invalid geometry or transform evidence.
    InvalidQuad {
        /// Best available authored quad identity.
        name: String,
    },
    /// Domain mesh construction rejected decoded geometry.
    Mesh(String),
}
