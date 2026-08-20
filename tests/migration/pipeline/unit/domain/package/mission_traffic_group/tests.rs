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
//   - Unit evidence for traffic group declaration binding.
// - Must-Not:
//   - Infer traffic spawn or parked-car policy.
// - Allows:
//   - Verify group structure, limits, flags, and canonical vehicle references.
// - Split-When:
//   - Active traffic-group selection needs independent fixtures.
// - Merge-When:
//   - Traffic groups move into complete level population tests.
// - Summary:
//   - Traffic group unit tests.
// - Description:
//   - Locks declarative traffic structure and vehicle-package binding.
// - Usage:
//   - Compiled as a child of mission_traffic_group.
// - Defaults:
//   - Invalid structure or missing package evidence fails closed.
//

//! Unit evidence for traffic model-group declarations.

use super::*;
use crate::domain::compile_mission_scope_graphs;
use crate::preflight_mission_script;

fn catalog() -> MissionReferenceCatalog {
    MissionReferenceCatalog::from_vehicle_entries_for_tests(&[
        ("famil_v", "vehicle-famil", "cars/famil_v/base-model"),
        ("pickupA", "vehicle-pickup", "cars/pickupa/base-model"),
    ])
}

fn scopes(commands: &[(&str, &[&str])]) -> Result<MissionScopeReport, String> {
    let render_args = |arguments: &[&str]| {
        arguments
            .iter()
            .map(|argument| {
                if argument.parse::<i64>().is_ok() {
                    (*argument).to_owned()
                } else {
                    format!("\"{argument}\"")
                }
            })
            .collect::<Vec<_>>()
            .join(",")
    };
    let invocations = commands
        .iter()
        .enumerate()
        .map(|(index, (name, arguments))| {
            let args_raw = render_args(arguments);
            serde_json::json!({
                "ordinal":index.saturating_add(1),
                "name":name,
                "args_raw":args_raw,
                "semantic_role":"mission-script",
                "arguments":arguments,
            })
        })
        .collect::<Vec<_>>();
    let mut counts = serde_json::Map::new();
    for (name, _) in commands {
        *counts
            .entry((*name).to_owned())
            .or_insert(serde_json::Value::from(0)) =
            serde_json::Value::from(
                commands.iter().filter(|(other, _)| other == name).count(),
            );
    }
    let statements = commands
        .iter()
        .map(|(name, arguments)| format!("{name}({});", render_args(arguments)))
        .collect::<Vec<_>>();
    let document = serde_json::json!({
        "schema":"shar-schoenwald.straggler.mission-script.v3",
        "source_extension":"mfk","route_class":"mission","source_bytes":64,
        "context_command_count":0,"context_adaptation_count":0,
        "context_adaptations":[],
        "context_finding_count":0,"context_findings":[],
        "statement_count":commands.len(),"unique_command_count":counts.len(),
        "load_p3d_reference_count":0,"mission_flow_command_count":0,
        "vehicle_physics_command_count":0,"semantic_family":"mission-script",
        "command_counts":counts,"source_statements":statements,
        "p3d_references":[],"command_invocations":invocations
    });
    let evidence = preflight_mission_script(
        &serde_json::to_string(&document).map_err(|error| error.to_string())?,
    )?;
    compile_mission_scope_graphs(&evidence)
}

#[test]
fn binds_complete_traffic_group_and_big_flag() -> Result<(), String> {
    let scopes = scopes(&[
        ("createtrafficgroup", &["0"]),
        ("addtrafficmodel", &["famil_v", "2"]),
        ("addtrafficmodel", &["pickupA", "1", "1"]),
        ("closetrafficgroup", &[]),
    ])?;
    let report = preflight_mission_traffic_groups(&catalog(), &scopes)?;
    let [group] = report.groups() else {
        return Err("traffic group count changed".to_owned());
    };
    assert_eq!(group.group_index(), 0);
    assert_eq!(group.create_source_ordinal(), 1);
    assert_eq!(group.close_source_ordinal(), 4);
    let [family, pickup] = group.members() else {
        return Err("traffic member count changed".to_owned());
    };
    assert_eq!(family.source_model(), "famil_v");
    assert_eq!(family.max_instances(), 2);
    assert_eq!(family.big_flag(), None);
    assert!(!family.is_big());
    assert_eq!(family.vehicle().package_id(), "vehicle-famil");
    assert_eq!(pickup.source_model(), "pickupA");
    assert_eq!(pickup.big_flag(), Some(1));
    assert!(pickup.is_big());
    assert_eq!(pickup.vehicle().package_id(), "vehicle-pickup");
    Ok(())
}

#[test]
fn preserves_non_one_numeric_big_flag_as_not_big() -> Result<(), String> {
    let scopes = scopes(&[
        ("createtrafficgroup", &["0"]),
        ("addtrafficmodel", &["famil_v", "1", "2"]),
        ("closetrafficgroup", &[]),
    ])?;
    let report = preflight_mission_traffic_groups(&catalog(), &scopes)?;
    let [group] = report.groups() else {
        return Err("traffic group count changed".to_owned());
    };
    let [member] = group.members() else {
        return Err("traffic member count changed".to_owned());
    };
    assert_eq!(member.big_flag(), Some(2));
    assert!(!member.is_big());
    Ok(())
}

#[test]
fn rejects_malformed_traffic_groups() -> Result<(), String> {
    for commands in [
        vec![("addtrafficmodel", &["famil_v", "1"][..])],
        vec![
            ("createtrafficgroup", &["0"][..]),
            ("closetrafficgroup", &[]),
        ],
        vec![
            ("createtrafficgroup", &["10"][..]),
            ("addtrafficmodel", &["famil_v", "1"][..]),
            ("closetrafficgroup", &[]),
        ],
        vec![
            ("createtrafficgroup", &["0"][..]),
            ("addtrafficmodel", &["famil_v", "0"][..]),
            ("closetrafficgroup", &[]),
        ],
        vec![
            ("createtrafficgroup", &["0"][..]),
            ("addtrafficmodel", &["unknown", "1"][..]),
            ("closetrafficgroup", &[]),
        ],
        vec![
            ("createtrafficgroup", &["0"][..]),
            ("addtrafficmodel", &["famil_v", "1", "big"][..]),
            ("closetrafficgroup", &[]),
        ],
    ] {
        let scopes = scopes(&commands)?;
        if preflight_mission_traffic_groups(&catalog(), &scopes).is_ok() {
            return Err("malformed traffic group was accepted".to_owned());
        }
    }
    Ok(())
}
