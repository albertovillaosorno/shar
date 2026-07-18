// File:
//   - decoded_billboard_source.rs
// Path:
//   - src/fbx/src/adapters/driven/decoded_billboard_source.rs
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
//   - Translation of decoded Pure3D billboard quad groups into static FBX
//   - inspection geometry.
// - Must-Not:
//   - Recreate camera-facing runtime behavior, select packages, or publish FBX.
// - Allows:
//   - Preserve authored dimensions, local transforms, UVs, colors, and shaders.
// - Summary:
//   - Converts source billboard emitters into separately named mesh evidence.
//
// Large file:
//   - false
//

//! Decoded billboard quad-group source adapter.

use std::fs;
use std::path::Path;

use serde::Deserialize;

use crate::domain::mesh::{MeshAsset, PrimitiveGroup};

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
    let bytes = fs::read(path).map_err(
        |error| DecodedBillboardError::Read {
            path: path
                .display()
                .to_string(),
            source: error.to_string(),
        },
    )?;
    let document: QuadGroupDocument = serde_json::from_slice(&bytes).map_err(
        |error| DecodedBillboardError::Parse {
            path: path
                .display()
                .to_string(),
            source: error.to_string(),
        },
    )?;
    if document.schema != "quad_group" || document.version != 0 {
        return Err(DecodedBillboardError::UnsupportedDocument);
    }
    let name = clean_identity(&document.name)?;
    if !name.eq_ignore_ascii_case(requested_id) {
        return Err(
            DecodedBillboardError::IdentityMismatch {
                requested: requested_id.to_owned(),
                decoded: name,
            },
        );
    }
    let shader = clean_identity(&document.shader)?;
    if document
        .quads
        .len()
        != document.num_quads
    {
        return Err(
            DecodedBillboardError::QuadCountMismatch {
                declared: document.num_quads,
                actual: document
                    .quads
                    .len(),
            },
        );
    }
    let groups = document
        .quads
        .iter()
        .enumerate()
        .map(
            |(index, quad)| {
                quad_group(
                    index, &shader, quad,
                )
            },
        )
        .collect::<Result<Vec<_>, _>>()?;
    MeshAsset::new(
        name, groups,
    )
    .map_err(|error| DecodedBillboardError::Mesh(format!("{error:?}")))
}

/// Build one four-vertex inspection plane from one source billboard quad.
fn quad_group(
    index: usize,
    shader: &str,
    quad: &QuadDocument,
) -> Result<PrimitiveGroup, DecodedBillboardError> {
    if quad.version != 2
        || !quad
            .width
            .is_finite()
        || !quad
            .height
            .is_finite()
        || quad.width <= 0.0
        || quad.height <= 0.0
        || quad
            .translation
            .iter()
            .any(|value| !value.is_finite())
        || quad
            .uvs
            .iter()
            .flatten()
            .any(|value| !value.is_finite())
        || quad
            .uv_offset
            .iter()
            .any(|value| !value.is_finite())
    {
        return Err(
            DecodedBillboardError::InvalidQuad {
                name: clean_identity(&quad.name)
                    .unwrap_or_else(|_| "quad".to_owned()),
            },
        );
    }
    let rotation = normalized_quaternion(
        quad.rotation_wxyz,
        &quad.name,
    )?;
    let half_width = quad.width * 0.5;
    let half_height = quad.height * 0.5;
    let local = [
        [
            -half_width,
            -half_height,
            0.0,
        ],
        [
            half_width,
            -half_height,
            0.0,
        ],
        [
            half_width,
            half_height,
            0.0,
        ],
        [
            -half_width,
            half_height,
            0.0,
        ],
    ];
    let positions = local
        .map(
            |value| {
                add(
                    rotate(
                        value, rotation,
                    ),
                    quad.translation,
                )
            },
        )
        .to_vec();
    let uvs = quad
        .uvs
        .map(
            |uv| {
                [
                    uv[0] + quad.uv_offset[0],
                    uv[1] + quad.uv_offset[1],
                ]
            },
        )
        .to_vec();
    let normal = rotate(
        [
            0.0, 0.0, 1.0,
        ],
        rotation,
    );
    let color = decode_argb(quad.colour);
    PrimitiveGroup::new(
        index,
        shader,
        positions,
        uvs,
        &[
            0, 1, 2, 0, 2, 3,
        ],
    )
    .and_then(|group| group.with_normals(vec![normal; 4]))
    .and_then(|group| group.with_colors(vec![color; 4]))
    .map_err(|error| DecodedBillboardError::Mesh(format!("{error:?}")))
}

/// Normalize one source WXYZ quaternion or reject unsupported evidence.
fn normalized_quaternion(
    value: [f32; 4],
    name: &str,
) -> Result<[f32; 4], DecodedBillboardError> {
    if value
        .iter()
        .any(|component| !component.is_finite())
    {
        return Err(
            DecodedBillboardError::InvalidQuad {
                name: clean_identity(name)
                    .unwrap_or_else(|_| "quad".to_owned()),
            },
        );
    }
    let length = value
        .iter()
        .map(|component| component * component)
        .sum::<f32>()
        .sqrt();
    if !length.is_finite() || length <= f32::EPSILON {
        return Err(
            DecodedBillboardError::InvalidQuad {
                name: clean_identity(name)
                    .unwrap_or_else(|_| "quad".to_owned()),
            },
        );
    }
    Ok(value.map(|component| component / length))
}

/// Rotate one vector by a unit WXYZ quaternion.
fn rotate(
    vector: [f32; 3],
    quaternion: [f32; 4],
) -> [f32; 3] {
    let [
        w,
        x,
        y,
        z,
    ] = quaternion;
    let [
        vx,
        vy,
        vz,
    ] = vector;
    let tx = 2.0_f32
        * z.mul_add(
            -vy,
            y * vz,
        );
    let ty = 2.0_f32
        * x.mul_add(
            -vz,
            z * vx,
        );
    let tz = 2.0_f32
        * y.mul_add(
            -vx,
            x * vy,
        );
    [
        z.mul_add(
            -ty,
            y.mul_add(
                tz,
                w.mul_add(
                    tx, vx,
                ),
            ),
        ),
        x.mul_add(
            -tz,
            z.mul_add(
                tx,
                w.mul_add(
                    ty, vy,
                ),
            ),
        ),
        y.mul_add(
            -tx,
            x.mul_add(
                ty,
                w.mul_add(
                    tz, vz,
                ),
            ),
        ),
    ]
}

/// Add two three-component vectors.
fn add(
    left: [f32; 3],
    right: [f32; 3],
) -> [f32; 3] {
    [
        left[0] + right[0],
        left[1] + right[1],
        left[2] + right[2],
    ]
}

/// Decode one PDDI AARRGGBB color into normalized RGBA channels.
fn decode_argb(value: u32) -> [f32; 4] {
    let [
        alpha,
        red,
        green,
        blue,
    ] = value.to_be_bytes();
    [
        f32::from(red) / 255.0_f32,
        f32::from(green) / 255.0_f32,
        f32::from(blue) / 255.0_f32,
        f32::from(alpha) / 255.0_f32,
    ]
}

/// Remove fixed-width source padding while preserving the authored identity.
fn clean_identity(value: &str) -> Result<String, DecodedBillboardError> {
    let clean = value
        .trim_end_matches('\0')
        .trim();
    if clean.is_empty()
        || clean
            .chars()
            .any(char::is_control)
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
    /// Source depth-test flag retained for schema validation.
    #[serde(rename = "z_test")]
    _z_test: u32,
    /// Source depth-write flag retained for schema validation.
    #[serde(rename = "z_write")]
    _z_write: u32,
    /// Source fog flag retained for schema validation.
    #[serde(rename = "fog")]
    _fog: u32,
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
    /// Source billboard mode retained for schema validation.
    #[serde(rename = "billboard_mode")]
    _billboard_mode: String,
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
    /// Source distance retained for schema validation.
    #[serde(rename = "distance")]
    _distance: f32,
    /// Authored UV translation.
    uv_offset: [f32; 2],
    /// Authored display rotation in WXYZ order.
    rotation_wxyz: [f32; 4],
    /// Source cutoff mode retained for schema validation.
    #[serde(rename = "cutoff_mode")]
    _cutoff_mode: String,
    /// Source UV range retained for schema validation.
    #[serde(rename = "uv_offset_range")]
    _uv_offset_range: [f32; 2],
    /// Source display range retained for schema validation.
    #[serde(rename = "source_range")]
    _source_range: f32,
    /// Source edge range retained for schema validation.
    #[serde(rename = "edge_range")]
    _edge_range: f32,
    /// Source perspective flag retained for schema validation.
    #[serde(rename = "perspective")]
    _perspective: bool,
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
