// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT
//
// Boundary-Contract:
// - Owns:
//   - Mission participant package-reference resolver unit regressions.
// - Must-Not:
//   - Read the game tree, publish Unreal assets, or invent participant aliases.
// - Allows:
//   - Synthetic package catalogs and exact mission semantic reports.
// - Split-When:
//   - Locator-reference tests gain an independent package boundary.
// - Merge-When:
//   - Mission reference resolution loses independent policy.
// - Summary:
//   - Mission participant package-reference resolver tests.
// - Description:
//   - Proves canonical character and vehicle resolution plus fail-closed drift.
// - Usage:
//   - Included only by the mission-reference domain module under cfg(test).
// - Defaults:
//   - Missing and ambiguous physical participants fail closed.
//
//! Mission participant package-reference resolver tests.

use super::*;

fn catalog() -> MissionReferenceCatalog {
    MissionReferenceCatalog {
        characters: BTreeMap::from([
            ("bart".to_owned(), vec![CharacterCatalogEntry {
                participant_id: "bart".to_owned(),
                package_id: "character-bart".to_owned(),
                package_subcategory: "characters/bart/base-model".to_owned(),
            }]),
            ("brn_unf".to_owned(), vec![CharacterCatalogEntry {
                participant_id: "barney".to_owned(),
                package_id: "character-barney-underwear".to_owned(),
                package_subcategory: "characters/barney/costume/underwear"
                    .to_owned(),
            }]),
        ]),
        vehicles: BTreeMap::from([("cletu_v".to_owned(), vec![
            VehicleCatalogEntry {
                package_id: "car-cletu-v".to_owned(),
                package_subcategory: "cars/character-rigs/cletu-v".to_owned(),
            },
        ])]),
    }
}

#[test]
fn resolves_physical_vehicle_and_symbolic_current() -> Result<(), String> {
    let catalog = catalog();
    let vehicle = catalog.resolve_vehicle("CLETU_V")?;
    let MissionVehicleReference::Catalog(vehicle) = vehicle else {
        return Err("physical vehicle resolved as symbolic current".to_owned());
    };
    if vehicle.source_id() != "CLETU_V" || vehicle.package_id() != "car-cletu-v"
    {
        return Err("physical vehicle package reference drifted".to_owned());
    }
    if catalog.resolve_vehicle("current")? != MissionVehicleReference::Current {
        return Err("current vehicle token must remain symbolic".to_owned());
    }
    Ok(())
}

#[test]
fn preserves_character_costume_variant_and_canonical_participant()
-> Result<(), String> {
    let reference = catalog().resolve_character("brn_unf")?;
    if reference.participant_id() != "barney"
        || reference.package_id() != "character-barney-underwear"
        || reference.package_subcategory()
            != "characters/barney/costume/underwear"
    {
        return Err(
            "character costume resolution lost exact variant".to_owned()
        );
    }
    Ok(())
}

#[test]
fn missing_and_ambiguous_catalog_entries_fail_closed() -> Result<(), String> {
    let mut catalog = catalog();
    if catalog.resolve_character("missing").is_ok()
        || catalog.resolve_vehicle("missing").is_ok()
    {
        return Err("missing participant reference was accepted".to_owned());
    }
    let duplicate = catalog
        .characters
        .get("bart")
        .and_then(|entries| entries.first())
        .cloned()
        .ok_or_else(|| "test character disappeared".to_owned())?;
    catalog
        .characters
        .get_mut("bart")
        .ok_or_else(|| "test character bucket disappeared".to_owned())?
        .push(duplicate);
    if catalog.resolve_character("bart").is_ok() {
        return Err(
            "ambiguous character source identity was accepted".to_owned()
        );
    }
    Ok(())
}

fn participant_mission_json() -> Result<String, String> {
    let value = serde_json::json!({
        "schema": "shar-schoenwald.straggler.mission-script.v3",
        "source_extension": "mfk",
        "route_class": "mission",
        "source_bytes": 512,
        "context_command_count": 6,
        "context_adaptation_count": 0,
        "context_adaptations": [],
        "context_finding_count": 0,
        "context_findings": [],
        "statement_count": 10,
        "unique_command_count": 10,
        "load_p3d_reference_count": 0,
        "mission_flow_command_count": 7,
        "vehicle_physics_command_count": 0,
        "semantic_family": "mission-script",
        "command_counts": {
            "selectmission": 1,
            "initlevelplayervehicle": 1,
            "addstage": 1,
            "addstagevehicle": 1,
            "addobjective": 1,
            "addnpc": 1,
            "setobjtargetvehicle": 1,
            "closeobjective": 1,
            "closestage": 1,
            "closemission": 1
        },
        "source_statements": [
            "SelectMission(\"m1\");",
            "InitLevelPlayerVehicle(\"cletu_v\",\"start\",\"OTHER\");",
            "AddStage(\"locked\",\"car\",\"cletu_v\");",
            concat!(
                "AddStageVehicle(\"cletu_v\",\"carstart\",\"chase\",",
                r#""scripts\cars\cletu_v.con","none");"#
            ),
            "AddObjective(\"getin\",\"cletu_v\");",
            "AddNPC(\"brn_unf\",\"npc_loc\");",
            "SetObjTargetVehicle(\"cletu_v\");",
            "CloseObjective();",
            "CloseStage();",
            "CloseMission();"
        ],
        "p3d_references": [],
        "command_invocations": [
            {
                "ordinal":1,
                "name":"selectmission",
                "args_raw":"\"m1\"",
                "semantic_role":"mission-script",
                "arguments":["m1"]
            },
            {
                "ordinal":2,
                "name":"initlevelplayervehicle",
                "args_raw":"\"cletu_v\",\"start\",\"OTHER\"",
                "semantic_role":"mission-script",
                "arguments":["cletu_v","start","OTHER"]
            },
            {
                "ordinal":3,
                "name":"addstage",
                "args_raw":"\"locked\",\"car\",\"cletu_v\"",
                "semantic_role":"mission-stage",
                "arguments":["locked","car","cletu_v"]
            },
            {
                "ordinal":4,
                "name":"addstagevehicle",
                "args_raw":concat!(
                    "\"cletu_v\",\"carstart\",\"chase\",",
                    r#""scripts\cars\cletu_v.con","none""#
                ),
                "semantic_role":"mission-stage",
                "arguments":[
                    "cletu_v","carstart","chase",
                    r"scripts\cars\cletu_v.con","none"
                ]
            },
            {
                "ordinal":5,
                "name":"addobjective",
                "args_raw":"\"getin\",\"cletu_v\"",
                "semantic_role":"mission-objective",
                "arguments":["getin","cletu_v"]
            },
            {
                "ordinal":6,
                "name":"addnpc",
                "args_raw":"\"brn_unf\",\"npc_loc\"",
                "semantic_role":"mission-script",
                "arguments":["brn_unf","npc_loc"]
            },
            {
                "ordinal":7,
                "name":"setobjtargetvehicle",
                "args_raw":"\"cletu_v\"",
                "semantic_role":"mission-script",
                "arguments":["cletu_v"]
            },
            {
                "ordinal":8,
                "name":"closeobjective",
                "args_raw":"",
                "semantic_role":"mission-objective",
                "arguments":[]
            },
            {
                "ordinal":9,
                "name":"closestage",
                "args_raw":"",
                "semantic_role":"mission-stage",
                "arguments":[]
            },
            {
                "ordinal":10,
                "name":"closemission",
                "args_raw":"",
                "semantic_role":"mission-script",
                "arguments":[]
            }
        ]
    });
    serde_json::to_string(&value).map_err(|error| error.to_string())
}

#[test]
fn resolves_parameters_directives_and_stage_header_in_source_order()
-> Result<(), String> {
    let text = participant_mission_json()?;
    let evidence = crate::domain::preflight_mission_script(&text)?;
    let scopes = crate::domain::compile_mission_scope_graphs(&evidence)?;
    let objectives =
        crate::domain::preflight_mission_objective_semantics(&scopes)?;
    let conditions =
        crate::domain::preflight_mission_condition_semantics(&scopes)?;
    let initialization =
        crate::domain::preflight_mission_initialization(&scopes)?;
    let stages = crate::domain::preflight_mission_stage_semantics(&scopes)?;
    let report = preflight_mission_references(
        &catalog(),
        &scopes,
        &objectives,
        &conditions,
        &initialization,
        &stages,
    )?;
    let [mission] = report.missions() else {
        return Err("participant fixture changed mission count".to_owned());
    };
    if mission.mission_id() != "m1" || mission.participants().len() != 6 {
        return Err("participant resolution envelope drifted".to_owned());
    }
    let ordinals = mission
        .participants()
        .iter()
        .map(MissionResolvedParticipantReference::source_ordinal)
        .collect::<Vec<_>>();
    if ordinals != [2, 3, 4, 5, 6, 7] {
        return Err(format!("participant source order drifted: {ordinals:?}"));
    }
    let [initial, locked, stage_vehicle, objective_parameter, npc, target] =
        mission.participants()
    else {
        return Err("participant owner fixture count drifted".to_owned());
    };
    assert_eq!(initial.owner_stage_source_ordinal(), None);
    assert_eq!(initial.owner_stage_sequence_ordinal(), None);
    assert_eq!(initial.owner_objective_source_ordinal(), None);
    assert_eq!(initial.owner_condition_source_ordinal(), None);
    for reference in [locked, stage_vehicle] {
        assert_eq!(reference.owner_stage_source_ordinal(), Some(3));
        assert_eq!(reference.owner_stage_sequence_ordinal(), Some(0));
        assert_eq!(reference.owner_objective_source_ordinal(), None);
        assert_eq!(reference.owner_condition_source_ordinal(), None);
    }
    for reference in [objective_parameter, npc, target] {
        assert_eq!(reference.owner_stage_source_ordinal(), Some(3));
        assert_eq!(reference.owner_stage_sequence_ordinal(), Some(0));
        assert_eq!(reference.owner_objective_source_ordinal(), Some(5));
        assert_eq!(reference.owner_condition_source_ordinal(), None);
    }
    let costume = mission
        .participants()
        .iter()
        .find_map(|reference| match reference.reference() {
            MissionParticipantReference::Character(character)
                if character.source_id() == "brn_unf" =>
            {
                Some(character)
            },
            MissionParticipantReference::Character(_)
            | MissionParticipantReference::Vehicle(_) => None,
        })
        .ok_or_else(|| {
            "Barney costume reference was not resolved".to_owned()
        })?;
    if costume.participant_id() != "barney" {
        return Err(
            "Barney costume lost canonical participant identity".to_owned()
        );
    }
    Ok(())
}

#[test]
fn condition_participant_preserves_full_owner_chain() -> Result<(), String> {
    let binding =
        crate::domain::MissionConditionSemanticBinding::from_parts_for_tests(
        3,
        0,
        Some(5),
        8,
        "damage",
        crate::domain::MissionConditionScope::Objective,
        "legacy-mission-condition.damage.v1",
        vec![MissionConditionDirective::TargetVehicle {
            source_ordinal: 9,
            vehicle_id: "cletu_v".to_owned(),
        }],
    );
    let mut participants = Vec::new();
    resolve_condition(&catalog(), &binding, &mut participants)?;
    let [reference] = participants.as_slice() else {
        return Err("condition participant count drifted".to_owned());
    };
    assert_eq!(reference.owner_stage_source_ordinal(), Some(3));
    assert_eq!(reference.owner_stage_sequence_ordinal(), Some(0));
    assert_eq!(reference.owner_objective_source_ordinal(), Some(5));
    assert_eq!(reference.owner_condition_source_ordinal(), Some(8));
    assert_eq!(reference.source_ordinal(), 9);
    assert_eq!(
        reference.role(),
        MissionParticipantRole::ConditionTargetVehicle
    );
    Ok(())
}
