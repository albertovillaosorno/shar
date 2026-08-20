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
//   - Unit evidence for vehicle-select canonical reference binding.
// - Must-Not:
//   - Infer vehicle-selection UI, ownership, or unlock behavior.
// - Allows:
//   - Verify exact P3D, vehicle, and character package resolution.
// - Split-When:
//   - Selection policy requires independent runtime fixtures.
// - Merge-When:
//   - Level registry tests own these exact registrations.
// - Summary:
//   - Vehicle-select registration unit tests.
// - Description:
//   - Proves all three authored identities resolve canonically.
// - Usage:
//   - Compiled with the package-domain unit suite.
// - Defaults:
//   - Missing, ambiguous, or symbolic references fail closed.
//

//! Unit evidence for vehicle-select registration bindings.

use serde_json::json;

use super::*;
use crate::preflight_mission_script;

fn evidence(vehicle: &str) -> Result<MissionScriptEvidence, String> {
    let value = json!({
        "schema":"shar-schoenwald.straggler.mission-script.v3",
        "source_extension":"mfk","route_class":"mission","source_bytes":64,
        "context_command_count":0,"context_adaptation_count":0,
        "context_adaptations":[],"context_finding_count":0,
        "context_findings":[],"statement_count":1,"unique_command_count":1,
        "load_p3d_reference_count":0,"mission_flow_command_count":0,
        "vehicle_physics_command_count":0,"semantic_family":"mission-script",
        "command_counts":{"addvehicleselectinfo":1},
        "source_statements":[format!(
            concat!(
                "AddVehicleSelectInfo(\"ART/CARS/snake_v.p3d\",",
                "\"{}\",\"snake\");"
            ),
            vehicle
        )],
        "p3d_references":[],
        "command_invocations":[{
            "ordinal":1,"name":"addvehicleselectinfo",
            "args_raw":format!(
                concat!(
                    "\"ART/CARS/snake_v.p3d\",",
                    "\"{}\",\"snake\""
                ),
                vehicle
            ),
            "semantic_role":"mission-script",
            "arguments":["ART/CARS/snake_v.p3d",vehicle,"snake"]
        }]
    });
    preflight_mission_script(
        &serde_json::to_string(&value).map_err(|error| error.to_string())?,
    )
}


fn catalogs() -> (MissionReferenceCatalog, MissionP3dReferenceCatalog) {
    let references =
        MissionReferenceCatalog::from_character_and_vehicle_entries_for_tests(
            &[(
                "snake",
                "snake",
                "character-snake",
                "characters/snake/base-model",
            )],
            &[("snake_v", "vehicle-snake", "cars/snake_v/base-model")],
        );
    let p3d = MissionP3dReferenceCatalog::from_entries_for_tests(&[(
        "extracted/art/cars/snake_v",
        "vehicle-snake-p3d",
        "extracted/art/cars/snake_v",
    )]);
    (references, p3d)
}

#[test]
fn binds_vehicle_select_registration_canonically() -> Result<(), String> {
    let (references, p3d) = catalogs();
    let report = preflight_mission_vehicle_selects(
        &evidence("snake_v")?,
        &references,
        &p3d,
    )?;
    let [binding] = report.bindings() else {
        return Err("vehicle-select binding count drifted".to_owned());
    };
    assert_eq!(binding.source_ordinal(), 1);
    assert_eq!(binding.p3d().source_reference(), "ART/CARS/snake_v.p3d");
    assert_eq!(binding.p3d().package_root(), "extracted/art/cars/snake_v");
    assert_eq!(binding.vehicle().source_id(), "snake_v");
    assert_eq!(binding.character().source_id(), "snake");
    Ok(())
}

#[test]
fn rejects_symbolic_vehicle_select_registration() -> Result<(), String> {
    let (references, p3d) = catalogs();
    let result = preflight_mission_vehicle_selects(
        &evidence("current")?,
        &references,
        &p3d,
    );
    let Err(error) = result else {
        return Err("symbolic vehicle-select registration must fail".to_owned());
    };
    assert!(error.contains("cannot be current"));
    Ok(())
}
