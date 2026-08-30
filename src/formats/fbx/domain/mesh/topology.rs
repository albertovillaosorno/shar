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
//   - Topology domain module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Topology domain module.
// - Description:
//   - Implements the declared domain module responsibility for fbx.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Topology domain module.

use super::error::MeshError;

/// Convert one exact triangle-list index stream into triangles.
///
/// # Errors
///
/// Returns an error when the stream is empty or has a partial triangle.
pub fn triangulate_triangle_list(
    indices: &[u32],
) -> Result<Vec<[u32; 3]>, MeshError> {
    if indices.is_empty() {
        return Err(MeshError::UnsupportedIndexCount(0));
    }
    let (triangles, remainder) = indices.as_chunks::<3>();
    if remainder.is_empty() {
        Ok(triangles.to_vec())
    } else {
        Err(MeshError::UnsupportedIndexCount(indices.len()))
    }
}

/// Convert decoded indices into triangles.
///
/// # Errors
///
/// Returns an error when the index list is neither a triangle list nor one
/// quad.
pub fn triangulate_indices(
    indices: &[u32],
) -> Result<Vec<[u32; 3]>, MeshError> {
    if let Ok(triangles) = triangulate_triangle_list(indices) {
        return Ok(triangles);
    }
    if let [first, second, third, fourth] = indices {
        return Ok(vec![[*first, *second, *third], [*first, *third, *fourth]]);
    }
    Err(MeshError::UnsupportedIndexCount(indices.len()))
}

/// Convert one triangle-strip index stream into triangles.
///
/// Degenerate stitching triangles that repeat a vertex are skipped, and the
/// winding of each emitted triangle follows the strip position parity so the
/// original facing survives the conversion.
///
/// # Errors
///
/// Returns an error when the strip is shorter than one triangle.
pub fn triangulate_strip(indices: &[u32]) -> Result<Vec<[u32; 3]>, MeshError> {
    if indices.len() < 3 {
        return Err(MeshError::UnsupportedIndexCount(indices.len()));
    }
    let mut triangles = Vec::new();
    for (position, window) in indices.windows(3).enumerate() {
        let [first, second, third] = window else {
            continue;
        };
        if first == second || second == third || first == third {
            continue;
        }
        if position % 2 == 0 {
            triangles.push([*first, *second, *third]);
        } else {
            triangles.push([*second, *first, *third]);
        }
    }
    if triangles.is_empty() {
        return Err(MeshError::UnsupportedIndexCount(indices.len()));
    }
    Ok(triangles)
}


#[cfg(test)]
#[path = "../../../../../tests/formats/fbx/unit/domain/mesh/topology/tests.rs"]
mod tests;
