// File:
//   - movement_catalog.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/prop_catalog/world_level/
//     movement_catalog.rs
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
//   - Stable JSON projection of coordinate movement and transformed records.
// - Must-Not:
//   - Read source packages, transform values, or write files.
// - Allows:
//   - Render movement matrices, subject contracts, counts, and evidence.
// - Summary:
//   - World coordinate movement manifest projection.
//
// ADRs:
// - docs/adr/pipeline/unreal/world-assembly-from-normalized-chunks.md
//
// Large file:
//   - false
//

//! JSON projection for package-level coordinate movements.

use serde_json::{Value, json};

use super::model::ExportedWorldCollection;
use super::movement_model::{
    WorldCoordinateMovementRecord, WorldMovedCoordinateRecord,
};
use crate::domain::coordinate_movement::CoordinateMatrix;

/// Render the complete coordinate movement manifest.
pub(super) fn coordinate_movements_value(
    collection: &ExportedWorldCollection
) -> Value {
    json!({
        "schema": "shar.world-coordinate-movements.v1",
        "coordinate_space": concat!(
            "normalized SHAR source space before FBX root ",
            "conversion"
        ),
        "matrix_convention": "row-major row-vector affine matrices",
        "contract": concat!(
            "one named movement owns geometry, collision, doors, object ",
            "placements, character and object spawns, mission placements, ",
            "triggers, cameras, locators, and lights; consumers must never ",
            "reconstruct movement from mesh bounds"
        ),
        "source_mutation": false,
        "movements": collection
            .coordinate_movements
            .iter()
            .map(movement_value)
            .collect::<Vec<_>>()
    })
}

/// Render one movement and its transformed coordinate records.
fn movement_value(record: &WorldCoordinateMovementRecord) -> Value {
    json!({
        "id": record.id,
        "package_id": record.package_id,
        "matrix": matrix_rows(&record.matrix),
        "subjects": record.subjects,
        "moved_render_meshes": record.moved_render_meshes,
        "moved_collision_meshes": record.moved_collision_meshes,
        "source_render_bounds": record.source_render_bounds.map(bounds_value),
        "moved_render_bounds": record.moved_render_bounds.map(bounds_value),
        "moved_coordinate_records": record.records.len(),
        "record_counts": subject_counts(&record.records),
        "records": record
            .records
            .iter()
            .map(record_value)
            .collect::<Vec<_>>()
    })
}

/// Render one axis-aligned bound as explicit low and high vectors.
fn bounds_value(
    bounds: (
        [f32; 3],
        [f32; 3],
    )
) -> Value {
    json!({
        "low": bounds.0,
        "high": bounds.1
    })
}

/// Render one transformed point, direction, or placement record.
fn record_value(record: &WorldMovedCoordinateRecord) -> Value {
    json!({
        "source_path": record.source_path,
        "identity": record.identity,
        "subject": record.subject,
        "source_position": record.source_position,
        "moved_position": record.moved_position,
        "source_direction": record.source_direction,
        "moved_direction": record.moved_direction,
        "source_matrix": record.source_matrix.as_ref().map(matrix_rows),
        "moved_matrix": record.moved_matrix.as_ref().map(matrix_rows)
    })
}

/// Count transformed records by stable coordinate subject.
fn subject_counts(records: &[WorldMovedCoordinateRecord]) -> Value {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for record in records {
        let count = counts
            .entry(&record.subject)
            .or_default();
        *count = count.saturating_add(1);
    }
    json!(counts)
}

/// Render one affine matrix as four explicit rows.
const fn matrix_rows(matrix: &CoordinateMatrix) -> [[f32; 4]; 4] {
    [
        [
            matrix[0], matrix[1], matrix[2], matrix[3],
        ],
        [
            matrix[4], matrix[5], matrix[6], matrix[7],
        ],
        [
            matrix[8], matrix[9], matrix[10], matrix[11],
        ],
        [
            matrix[12], matrix[13], matrix[14], matrix[15],
        ],
    ]
}
