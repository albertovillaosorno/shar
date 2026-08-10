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
//   - Unit evidence for pedestrian group declaration binding.
// - Must-Not:
//   - Infer pedestrian spawn or navigation policy.
// - Allows:
//   - Verify group structure, limits, and canonical character references.
// - Split-When:
//   - Runtime group selection needs independent fixtures.
// - Merge-When:
//   - Pedestrian groups move into complete level population tests.
// - Summary:
//   - Pedestrian group unit tests.
// - Description:
//   - Locks declarative group structure and character-package binding.
// - Usage:
//   - Compiled as a child of mission_ped_group.
// - Defaults:
//   - Invalid structure or missing package evidence fails closed.
//

use super::*;
use crate::domain::{compile_mission_scope_graphs, preflight_mission_script};

fn catalog() -> MissionReferenceCatalog {
    MissionReferenceCatalog::from_character_entries_for_tests(&[
        (
            "male6",
            "male6",
            "character-male6",
            "characters/male6/base-model",
        ),
        (
            "girl4",
            "girl4",
            "character-girl4",
            "characters/girl4/base-model",
        ),
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
                "ordinal": index + 1,
                "name": name,
                "args_raw": args_raw,
                "semantic_role": "mission-script",
                "arguments": arguments,
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
    let document = serde_json::json!({
        "schema":"shar-schoenwald.straggler.mission-script.v3",
        "source_extension":"mfk",
        "route_class":"mission",
        "source_bytes":64,
        "context_command_count":0,
        "context_adaptation_count":0,
        "context_adaptations":[],
        "context_finding_count":0,
        "context_findings":[],
        "statement_count":commands.len(),
        "unique_command_count":counts.len(),
        "load_p3d_reference_count":0,
        "mission_flow_command_count":0,
        "vehicle_physics_command_count":0,
        "semantic_family":"mission-script",
        "command_counts":counts,
        "source_statements":commands
            .iter()
            .map(|(name, arguments)| {
                format!("{name}({});", render_args(arguments))
            })
            .collect::<Vec<_>>(),
        "p3d_references":[],
        "command_invocations":invocations
    });
    let evidence = preflight_mission_script(
        &serde_json::to_string(&document).map_err(|error| error.to_string())?,
    )?;
    compile_mission_scope_graphs(&evidence)
}

#[test]
fn binds_complete_ped_group_to_character_packages() -> Result<(), String> {
    let scopes = scopes(&[
        ("createpedgroup", &["2"]),
        ("addped", &["male6", "2"]),
        ("addped", &["girl4", "1"]),
        ("closepedgroup", &[]),
    ])?;
    let report = preflight_mission_ped_groups(&catalog(), &scopes)?;
    let [group] = report.groups() else {
        return Err("ped group count changed".to_owned());
    };
    assert_eq!(group.group_index(), 2);
    assert_eq!(group.create_source_ordinal(), 1);
    assert_eq!(group.close_source_ordinal(), 4);
    let [male, girl] = group.members() else {
        return Err("ped group member count changed".to_owned());
    };
    assert_eq!(male.source_model(), "male6");
    assert_eq!(male.max_instances(), 2);
    assert_eq!(male.character().package_id(), "character-male6");
    assert_eq!(girl.source_model(), "girl4");
    assert_eq!(girl.max_instances(), 1);
    Ok(())
}

#[test]
fn rejects_malformed_ped_group_lifecycle() -> Result<(), String> {
    for commands in [
        vec![("addped", &["male6", "2"][..])],
        vec![("createpedgroup", &["0"][..]), ("closepedgroup", &[])],
        vec![("createpedgroup", &["0"][..]), ("addped", &["male6", "2"][..])],
    ] {
        let scopes = scopes(&commands)?;
        if preflight_mission_ped_groups(&catalog(), &scopes).is_ok() {
            return Err("malformed ped group lifecycle was accepted".to_owned());
        }
    }
    Ok(())
}

#[test]
fn rejects_ped_group_limits_and_missing_character() -> Result<(), String> {
    for commands in [
        vec![
            ("createpedgroup", &["10"][..]),
            ("addped", &["male6", "1"][..]),
            ("closepedgroup", &[]),
        ],
        vec![
            ("createpedgroup", &["0"][..]),
            ("addped", &["male6", "0"][..]),
            ("closepedgroup", &[]),
        ],
        vec![
            ("createpedgroup", &["0"][..]),
            ("addped", &["unknown", "1"][..]),
            ("closepedgroup", &[]),
        ],
    ] {
        let scopes = scopes(&commands)?;
        if preflight_mission_ped_groups(&catalog(), &scopes).is_ok() {
            return Err("invalid ped group member was accepted".to_owned());
        }
    }
    Ok(())
}
