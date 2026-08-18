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
//   - Unit evidence for authored mission registration order and siblings.
// - Must-Not:
//   - Infer unlock, prerequisite, completion, or progression behavior.
// - Allows:
//   - Verify source order and exact init/load sibling identities.
// - Split-When:
//   - Runtime mission progression requires independent fixtures.
// - Merge-When:
//   - Final level mission registry tests own this exact cross-script relation.
// - Summary:
//   - Authored mission registration adapter tests.
// - Description:
//   - Proves AddMission order and sibling selection remain source-backed.
// - Usage:
//   - Compiled only by the local mission-order adapter.
// - Defaults:
//   - Missing or mismatched siblings fail closed.
//

//! Unit evidence for authored mission registration order.

use serde_json::json;

use super::*;
use crate::domain::{MissionScriptEvidence, preflight_mission_script};

fn evidence(
    name: Option<&str>,
    ids: &[&str],
) -> Result<MissionScriptEvidence, String> {
    if name.is_none() {
        let value = json!({
            "schema":"shar-schoenwald.straggler.mission-script.v3",
            "source_extension":"mfk","route_class":"mission","source_bytes":0,
            "context_command_count":0,"context_adaptation_count":0,
            "context_adaptations":[],"context_finding_count":0,
            "context_findings":[],"statement_count":0,"unique_command_count":0,
            "load_p3d_reference_count":0,"mission_flow_command_count":0,
            "vehicle_physics_command_count":0,
            "semantic_family":"mission-script",
            "command_counts":{},"source_statements":[],"p3d_references":[],
            "command_invocations":[]
        });
        return preflight_mission_script(
            &serde_json::to_string(&value).map_err(|error| error.to_string())?,
        );
    }
    let name = name.unwrap_or_default();
    if name == "selectmission" {
        let [id] = ids else {
            return Err("selectmission fixture requires one id".to_owned());
        };
        let value = json!({
            "schema":"shar-schoenwald.straggler.mission-script.v3",
            "source_extension":"mfk","route_class":"mission","source_bytes":64,
            "context_command_count":2,"context_adaptation_count":0,
            "context_adaptations":[],"context_finding_count":0,
            "context_findings":[],"statement_count":2,"unique_command_count":2,
            "load_p3d_reference_count":0,"mission_flow_command_count":2,
            "vehicle_physics_command_count":0,
            "semantic_family":"mission-script",
            "command_counts":{"selectmission":1,"closemission":1},
            "source_statements":[
                format!("SelectMission(\"{id}\");"),
                "CloseMission();"
            ],
            "p3d_references":[],
            "command_invocations":[
                {"ordinal":1,"name":"selectmission",
                 "args_raw":format!("\"{id}\""),
                 "semantic_role":"mission-script","arguments":[id]},
                {"ordinal":2,"name":"closemission","args_raw":"",
                 "semantic_role":"mission-script","arguments":[]}
            ]
        });
        return preflight_mission_script(
            &serde_json::to_string(&value).map_err(|error| error.to_string())?,
        );
    }
    let invocations = ids
        .iter()
        .enumerate()
        .map(|(index, id)| json!({
            "ordinal":index.saturating_add(1),"name":name,
            "args_raw":format!("\"{id}\""),"semantic_role":"mission-script",
            "arguments":[id]
        }))
        .collect::<Vec<_>>();
    let statements = ids
        .iter()
        .map(|id| {
            if name == "addmission" {
                format!("AddMission(\"{id}\");")
            } else {
                format!("SelectMission(\"{id}\");")
            }
        })
        .collect::<Vec<_>>();
    let value = json!({
        "schema":"shar-schoenwald.straggler.mission-script.v3",
        "source_extension":"mfk","route_class":"mission","source_bytes":64,
        "context_command_count":0,"context_adaptation_count":0,
        "context_adaptations":[],"context_finding_count":0,
        "context_findings":[],"statement_count":ids.len(),
        "unique_command_count":1,
        "load_p3d_reference_count":0,"mission_flow_command_count":ids.len(),
        "vehicle_physics_command_count":0,"semantic_family":"mission-script",
        "command_counts":{name:ids.len()},"source_statements":statements,
        "p3d_references":[],"command_invocations":invocations
    });
    preflight_mission_script(
        &serde_json::to_string(&value).map_err(|error| error.to_string())?,
    )
}

fn snapshot(
    path: &str,
    name: Option<&str>,
    ids: &[&str],
) -> Result<MissionLocatorScriptSnapshot, String> {
    Ok(MissionLocatorScriptSnapshot::new(
        path.to_owned(),
        evidence(name, ids)?,
        Vec::new(),
    ))
}

#[test]
fn preserves_authored_registration_order_and_siblings() -> Result<(), String> {
    let snapshots = vec![
        snapshot(
            "extracted/game/scripts/missions/level01/level.mfk.json",
            Some("addmission"),
            &["m1", "m2"],
        )?,
        snapshot(
            "extracted/game/scripts/missions/level01/m1i.mfk.json",
            Some("selectmission"),
            &["m1"],
        )?,
        snapshot(
            "extracted/game/scripts/missions/level01/m1l.mfk.json",
            None,
            &[],
        )?,
        snapshot(
            "extracted/game/scripts/missions/level01/m2i.mfk.json",
            Some("selectmission"),
            &["m2"],
        )?,
        snapshot(
            "extracted/game/scripts/missions/level01/m2l.mfk.json",
            None,
            &[],
        )?,
    ];
    let reports = build_mission_order_source_reports(&snapshots)?;
    let [report] = reports.as_slice() else {
        return Err("mission registration report count drifted".to_owned());
    };
    assert_eq!(
        report.source_path(),
        "extracted/game/scripts/missions/level01/level.mfk.json"
    );
    let [first, second] = report.registrations() else {
        return Err("mission registration count drifted".to_owned());
    };
    assert_eq!(first.source_ordinal(), 1);
    assert_eq!(first.sequence_ordinal(), 0);
    assert_eq!(first.mission_id(), "m1");
    assert!(first.init_source_path().ends_with("/m1i.mfk.json"));
    assert!(first.load_source_path().ends_with("/m1l.mfk.json"));
    assert_eq!(second.source_ordinal(), 2);
    assert_eq!(second.sequence_ordinal(), 1);
    assert_eq!(second.mission_id(), "m2");
    Ok(())
}

#[test]
fn supports_demo_registration_family() -> Result<(), String> {
    let snapshots = vec![
        snapshot(
            "extracted/game/scripts/missions/level03/demo.mfk.json",
            Some("addmission"),
            &["d1"],
        )?,
        snapshot(
            "extracted/game/scripts/missions/level03/d1i.mfk.json",
            Some("selectmission"),
            &["d1"],
        )?,
        snapshot(
            "extracted/game/scripts/missions/level03/d1l.mfk.json",
            None,
            &[],
        )?,
    ];
    let reports = build_mission_order_source_reports(&snapshots)?;
    let [report] = reports.as_slice() else {
        return Err("demo registration report count drifted".to_owned());
    };
    let [registration] = report.registrations() else {
        return Err("demo registration count drifted".to_owned());
    };
    assert_eq!(registration.mission_id(), "d1");
    Ok(())
}

#[test]
fn rejects_missing_or_mismatched_registration_siblings() -> Result<(), String> {
    let missing_load = vec![
        snapshot(
            "extracted/game/scripts/missions/level01/level.mfk.json",
            Some("addmission"),
            &["m1"],
        )?,
        snapshot(
            "extracted/game/scripts/missions/level01/m1i.mfk.json",
            Some("selectmission"),
            &["m1"],
        )?,
    ];
    let missing_result = build_mission_order_source_reports(&missing_load);
    let Err(error) = missing_result else {
        return Err("missing load sibling unexpectedly passed".to_owned());
    };
    assert!(error.contains("load sibling is missing"));

    let mismatch = vec![
        snapshot(
            "extracted/game/scripts/missions/level01/level.mfk.json",
            Some("addmission"),
            &["m1"],
        )?,
        snapshot(
            "extracted/game/scripts/missions/level01/m1i.mfk.json",
            Some("selectmission"),
            &["m2"],
        )?,
        snapshot(
            "extracted/game/scripts/missions/level01/m1l.mfk.json",
            None,
            &[],
        )?,
    ];
    let mismatch_result = build_mission_order_source_reports(&mismatch);
    let Err(error) = mismatch_result else {
        return Err(
            "mismatched mission selection unexpectedly passed".to_owned(),
        );
    };
    assert!(error.contains("selects a different id"));
    Ok(())
}
