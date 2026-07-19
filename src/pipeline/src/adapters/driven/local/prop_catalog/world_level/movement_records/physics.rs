// File:
//   - physics.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/prop_catalog/world_level/
//     movement_records/physics.rs
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
//   - Static-physics primitive center and basis movement evidence.
// - Must-Not:
//   - Rebuild collision topology, alter primitive lengths, or write native
//     data.
// - Allows:
//   - Traverse decoded physics JSON and transform oriented-box coordinates.
// - Summary:
//   - Moves static collision primitive placements with the package transform.
//
// ADRs:
// - docs/adr/pipeline/unreal/world-assembly-from-normalized-chunks.md
//
// Large file:
//   - false
//

//! Static physics placement movement evidence.

use serde_json::Value;

use super::super::movement_model::WorldMovedCoordinateRecord;
use super::{SourceComponent, identity, movement_error, vector3};
use crate::domain::PipelineError;
use crate::domain::coordinate_movement::CoordinateMovement;

/// Collect every oriented collision-box center and basis vector.
pub(super) fn collect_physics(
    components: &[SourceComponent],
    movement: CoordinateMovement,
    records: &mut Vec<WorldMovedCoordinateRecord>,
) -> Result<(), PipelineError> {
    for component in components {
        let owner_identity = identity(
            &component.value,
            &component.relative_path,
        );
        let mut ordinal = 0_usize;
        walk_value(
            &component.value,
            component,
            &owner_identity,
            movement,
            &mut ordinal,
            records,
        )?;
    }
    Ok(())
}

/// Recursively visit arrays and objects until oriented-box vectors are found.
fn walk_value(
    value: &Value,
    component: &SourceComponent,
    owner_identity: &str,
    movement: CoordinateMovement,
    ordinal: &mut usize,
    records: &mut Vec<WorldMovedCoordinateRecord>,
) -> Result<(), PipelineError> {
    match value {
        Value::Array(values) => {
            for child in values {
                walk_value(
                    child,
                    component,
                    owner_identity,
                    movement,
                    ordinal,
                    records,
                )?;
            }
        }
        Value::Object(values) => {
            if values
                .get("kind")
                .and_then(Value::as_str)
                .is_some_and(|kind| kind == "obbox")
            {
                collect_oriented_box(
                    value,
                    component,
                    owner_identity,
                    movement,
                    *ordinal,
                    records,
                )?;
                *ordinal = ordinal
                    .checked_add(1)
                    .ok_or_else(
                        || {
                            PipelineError::new(
                                "physics movement ordinal overflowed",
                            )
                        },
                    )?;
            }
            for child in values.values() {
                walk_value(
                    child,
                    component,
                    owner_identity,
                    movement,
                    ordinal,
                    records,
                )?;
            }
        }
        _ => {}
    }
    Ok(())
}

/// Transform one oriented-box center and each basis vector.
fn collect_oriented_box(
    value: &Value,
    component: &SourceComponent,
    owner_identity: &str,
    movement: CoordinateMovement,
    ordinal: usize,
    records: &mut Vec<WorldMovedCoordinateRecord>,
) -> Result<(), PipelineError> {
    let vectors = value
        .get("vectors")
        .and_then(Value::as_array)
        .ok_or_else(
            || PipelineError::new("oriented physics box has no vectors"),
        )?;
    let Some(center_value) = vectors.first() else {
        return Err(PipelineError::new("oriented physics box has no center"));
    };
    let primitive_identity = format!("{owner_identity}/physics-{ordinal:04}");
    let center = vector3(
        center_value,
        "physics center",
    )?;
    let moved_center = movement
        .transform_point(center)
        .map_err(movement_error(&primitive_identity))?;
    records.push(
        WorldMovedCoordinateRecord::point(
            component
                .relative_path
                .clone(),
            format!("{primitive_identity}/center"),
            "collision",
            center,
            moved_center,
        ),
    );
    for (basis_ordinal, basis_value) in vectors
        .iter()
        .skip(1)
        .enumerate()
    {
        let basis = vector3(
            basis_value,
            "physics basis",
        )?;
        let moved_basis = movement
            .transform_direction(basis)
            .map_err(movement_error(&primitive_identity))?;
        records.push(
            WorldMovedCoordinateRecord::direction(
                component
                    .relative_path
                    .clone(),
                format!("{primitive_identity}/basis-{basis_ordinal:02}"),
                "collision",
                basis,
                moved_basis,
            ),
        );
    }
    Ok(())
}
