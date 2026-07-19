// File:
//   - light.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/prop_catalog/world_level/
//     movement_records/light.rs
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
//   - Authored light-position and direction movement evidence.
// - Must-Not:
//   - Create Blender lights, alter illumination, or infer emitter materials.
// - Allows:
//   - Transform decoded light points and directional vectors.
// - Summary:
//   - Moves source light coordinates with the package transform.
//
// ADRs:
// - docs/adr/pipeline/unreal/world-assembly-from-normalized-chunks.md
//
// Large file:
//   - false
//

//! Source light coordinate movement evidence.

use serde_json::Value;

use super::super::movement_model::WorldMovedCoordinateRecord;
use super::{SourceComponent, identity, movement_error, vector3};
use crate::domain::PipelineError;
use crate::domain::coordinate_movement::CoordinateMovement;

/// Collect every decoded light position and direction.
pub(super) fn collect_lights(
    components: &[SourceComponent],
    movement: CoordinateMovement,
    records: &mut Vec<WorldMovedCoordinateRecord>,
) -> Result<(), PipelineError> {
    for component in components {
        let light_identity = identity(
            &component.value,
            &component.relative_path,
        );
        let Some(extras) = component
            .value
            .get("extras")
            .and_then(Value::as_array)
        else {
            continue;
        };
        for (ordinal, extra) in extras
            .iter()
            .enumerate()
        {
            let Some(kind) = extra
                .get("kind")
                .and_then(Value::as_str)
            else {
                continue;
            };
            let Some(value) = extra.get("value") else {
                continue;
            };
            match kind {
                "position" => {
                    let source = vector3(
                        value,
                        "light position",
                    )?;
                    let moved = movement
                        .transform_point(source)
                        .map_err(movement_error(&light_identity))?;
                    records.push(
                        WorldMovedCoordinateRecord::point(
                            component
                                .relative_path
                                .clone(),
                            format!("{light_identity}/position-{ordinal:02}"),
                            "light",
                            source,
                            moved,
                        ),
                    );
                }
                "direction" => {
                    let source = vector3(
                        value,
                        "light direction",
                    )?;
                    let moved = movement
                        .transform_direction(source)
                        .map_err(movement_error(&light_identity))?;
                    records.push(
                        WorldMovedCoordinateRecord::direction(
                            component
                                .relative_path
                                .clone(),
                            format!("{light_identity}/direction-{ordinal:02}"),
                            "light",
                            source,
                            moved,
                        ),
                    );
                }
                _ => {}
            }
        }
    }
    Ok(())
}
