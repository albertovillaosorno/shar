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
//   - Mission and config command parser unit tests.
// - Must-Not:
//   - Read repository game data or mutate generated extraction evidence.
// - Allows:
//   - Synthetic command statements with quotes, comments, and nested calls.
// - Split-When:
//   - Split when mission and vehicle command grammars become independent.
// - Merge-When:
//   - Merge when command parsing no longer has an independent contract.
// - Summary:
//   - Command parser tests.
// - Description:
//   - Proves canonical argument extraction before semantic compilation.
// - Usage:
//   - Included only by the command adapter under cfg(test).
// - Defaults:
//   - Trailing comments and delimiters never enter command arguments.
//

//! Command parser unit tests.

use std::path::Path;

use super::{parse_call, split_arguments};

#[test]
fn strips_trailing_line_comments_from_call_arguments() -> Result<(), String> {
    let statement = r#"AddBonusMission("sr1"); // street race"#;
    let Some((command, raw)) = parse_call(statement) else {
        return Err("commented command was not parsed".to_owned());
    };
    if command != "AddBonusMission" || raw != r#""sr1""# {
        return Err(format!("unexpected parsed call: {command}({raw})"));
    }
    if split_arguments(&raw) != ["sr1"] {
        return Err("comment entered AddBonusMission arguments".to_owned());
    }
    Ok(())
}

#[test]
fn preserves_backslashes_and_strips_reward_comments() -> Result<(), String> {
    // jig-ignore-next-line: literal
    let statement = r#"BindReward("plowk_v", "art\cars\plowk_v.p3d", "car", "forsale", 1, 150, "simpson" ); // Barney"#;
    let Some((command, raw)) = parse_call(statement) else {
        return Err("reward command was not parsed".to_owned());
    };
    if command != "BindReward" {
        return Err("reward command identity changed".to_owned());
    }
    let args = split_arguments(&raw);
    let expected = [
        "plowk_v",
        "art\\cars\\plowk_v.p3d",
        "car",
        "forsale",
        "1",
        "150",
        "simpson",
    ];
    if args != expected {
        return Err(format!("reward arguments changed: {args:?}"));
    }
    Ok(())
}

#[test]
fn uses_matching_parenthesis_outside_quoted_text() -> Result<(), String> {
    let statement = r#"Command("literal ) text", Nested(1, 2)); // note"#;
    let Some((_command, raw)) = parse_call(statement) else {
        return Err("nested command was not parsed".to_owned());
    };
    if raw != r#""literal ) text", Nested(1, 2)"# {
        return Err(format!("matching parenthesis was wrong: {raw}"));
    }
    Ok(())
}

fn invocation(
    ordinal: usize,
    name: &str,
    arguments: &[&str],
) -> super::CommandInvocation {
    super::CommandInvocation {
        ordinal,
        name: name.to_owned(),
        args_raw: arguments.join(", "),
        arguments: arguments.iter().map(|value| (*value).to_owned()).collect(),
        semantic_role: "mission-script".to_owned(),
    }
}

#[test]
fn preserves_nested_argument_groups() -> Result<(), String> {
    let args = split_arguments(r#""literal", Nested(1, 2), "tail""#);
    if args != ["literal", "Nested(1, 2)", "tail"] {
        return Err(format!("nested arguments split incorrectly: {args:?}"));
    }
    Ok(())
}

#[test]
fn accepts_balanced_reviewed_mission_context() -> Result<(), String> {
    let invocations = [
        invocation(1, "selectmission", &["m1"]),
        invocation(2, "addstage", &["0"]),
        invocation(3, "addobjective", &["goto"]),
        invocation(4, "closeobjective", &[]),
        invocation(5, "addcondition", &["timeout"]),
        invocation(6, "closecondition", &[]),
        invocation(7, "closestage", &[]),
        invocation(8, "closemission", &[]),
    ];
    let findings = super::context::validate(
        Path::new("scripts/missions/test.mfk"),
        &invocations,
    )
    .findings;
    if super::context::findings_json(&findings) != "[]" {
        return Err("balanced mission context produced a finding".to_owned());
    }
    Ok(())
}

#[test]
fn reports_orphan_condition_close_without_cascade() -> Result<(), String> {
    let invocations = [
        invocation(1, "selectmission", &["m6sd"]),
        invocation(2, "addstage", &["0"]),
        invocation(3, "closecondition", &[]),
        invocation(4, "closestage", &[]),
        invocation(5, "closemission", &[]),
    ];
    let findings = super::context::findings_json(
        &super::context::validate(
            Path::new("scripts/missions/test.mfk"),
            &invocations,
        )
        .findings,
    );
    if !findings.contains("condition-close-without-open-condition") {
        return Err("orphan condition close was not reported".to_owned());
    }
    if findings.matches("code").count() != 1 {
        return Err(format!("orphan close cascaded findings: {findings}"));
    }
    Ok(())
}

#[test]
fn resynchronizes_stage_close_with_open_condition() -> Result<(), String> {
    let invocations = [
        invocation(1, "selectmission", &["m7"]),
        invocation(2, "addstage", &["0"]),
        invocation(3, "addcondition", &["keepbarrel", "2"]),
        invocation(4, "closestage", &[]),
        invocation(5, "addstage", &["0"]),
        invocation(6, "addobjective", &["goto"]),
        invocation(7, "closeobjective", &[]),
        invocation(8, "closestage", &[]),
        invocation(9, "closemission", &[]),
    ];
    let findings = super::context::findings_json(
        &super::context::validate(
            Path::new("scripts/missions/test.mfk"),
            &invocations,
        )
        .findings,
    );
    if !findings.contains("stage-close-with-open-context") {
        return Err("open condition at stage close was not reported".to_owned());
    }
    if findings.matches("code").count() != 1 {
        return Err(format!(
            "stage-close resync cascaded findings: {findings}"
        ));
    }
    Ok(())
}

#[test]
fn reports_reviewed_context_arity_drift() -> Result<(), String> {
    let invocations = [
        invocation(1, "selectmission", &["m1", "unexpected"]),
        invocation(2, "closemission", &[]),
    ];
    let findings = super::context::findings_json(
        &super::context::validate(
            Path::new("scripts/missions/test.mfk"),
            &invocations,
        )
        .findings,
    );
    if !findings.contains("invalid-context-command-arity") {
        return Err("reviewed command arity drift was not reported".to_owned());
    }
    Ok(())
}

#[test]
fn renders_context_evidence_for_balanced_mission_script() -> Result<(), String>
{
    let source = concat!(
        "SelectMission(\"m1\");\n",
        "AddStage(0);\n",
        "AddObjective(\"goto\");\n",
        "CloseObjective();\n",
        "CloseStage();\n",
        "CloseMission();\n",
    );
    let mut json = super::super::json::JsonObject::new();
    super::append_summary(
        &mut json,
        source,
        "mfk",
        Path::new("scripts/missions/test.mfk"),
    );
    let rendered = json.finish();
    if !rendered.contains("\"context_command_count\":6")
        || !rendered.contains("\"context_adaptation_count\":0")
        || !rendered.contains("\"context_adaptations\":[]")
        || !rendered.contains("\"context_finding_count\":0")
        || !rendered.contains("\"context_findings\":[]")
    {
        return Err(format!("context evidence is incomplete: {rendered}"));
    }
    Ok(())
}

#[test]
fn renders_structural_findings_without_repairing_source() -> Result<(), String>
{
    let source = concat!(
        "SelectMission(\"m7\");\n",
        "AddStage(0);\n",
        "AddCondition(\"keepbarrel\", 2);\n",
        "CloseStage();\n",
        "CloseMission();\n",
    );
    let mut json = super::super::json::JsonObject::new();
    super::append_summary(
        &mut json,
        source,
        "mfk",
        Path::new("scripts/missions/test.mfk"),
    );
    let rendered = json.finish();
    if !rendered.contains("\"context_finding_count\":1")
        || !rendered.contains("stage-close-with-open-context")
        || !rendered.contains("\"command\":\"closestage\"")
    {
        return Err(format!("structural finding was not rendered: {rendered}"));
    }
    Ok(())
}
#[test]
fn adapts_only_exact_reviewed_legacy_context_windows() -> Result<(), String> {
    let l2 = [
        invocation(1, "selectmission", &["m6sd"]),
        invocation(2, "addstage", &["0"]),
        invocation(68, "addstagemusicchange", &[]),
        invocation(69, "setstagemusicalwayson", &[]),
        invocation(70, "closecondition", &[]),
        invocation(71, "closestage", &[]),
        invocation(72, "closemission", &[]),
    ];
    let adapted = super::context::validate(
        Path::new("scripts/missions/level02/m6sdi.mfk"),
        &l2,
    );
    if !adapted.findings.is_empty()
        || super::context::adaptations_json(&adapted.adaptations)
                        // jig-ignore-next-line: literal
            != "[{\"ordinal\":70,\"command\":\"closecondition\",\"code\":\"legacy-l2-m6sdi-ignore-orphan-condition-close-v1\"}]"
    {
        return Err("reviewed level02 mission adaptation changed".to_owned());
    }
    let wrong_path = super::context::validate(
        Path::new("scripts/missions/level02/other.mfk"),
        &l2,
    );
    if super::context::findings_json(&wrong_path.findings)
        .matches("condition-close-without-open-condition")
        .count()
        != 1
        || !wrong_path.adaptations.is_empty()
    {
        return Err(
            "legacy adaptation escaped its exact logical path".to_owned()
        );
    }

    let l7 = [
        invocation(1, "selectmission", &["m7"]),
        invocation(2, "addstage", &["0"]),
        invocation(112, "stagestartmusicevent", &["L7_drama"]),
        invocation(113, "addcondition", &["keepbarrel", "2"]),
        invocation(114, "showstagecomplete", &[]),
        invocation(115, "closestage", &[]),
        invocation(116, "closemission", &[]),
    ];
    let adapted = super::context::validate(
        Path::new("scripts/missions/level07/m7i.mfk"),
        &l7,
    );
    if !adapted.findings.is_empty()
        || super::context::adaptations_json(&adapted.adaptations)
                        // jig-ignore-next-line: literal
            != "[{\"ordinal\":114,\"command\":\"showstagecomplete\",\"code\":\"legacy-l7-m7i-close-keepbarrel-before-stage-complete-v1\"}]"
    {
        return Err("reviewed level07 mission adaptation changed".to_owned());
    }
    Ok(())
}

#[test]
fn adaptation_fingerprint_drift_fails_closed() -> Result<(), String> {
    let drifted = [
        invocation(1, "selectmission", &["m7"]),
        invocation(2, "addstage", &["0"]),
        invocation(112, "stagestartmusicevent", &["L7_drama"]),
        invocation(113, "addcondition", &["keepbarrel", "3"]),
        invocation(114, "showstagecomplete", &[]),
        invocation(115, "closestage", &[]),
        invocation(116, "closemission", &[]),
    ];
    let validation = super::context::validate(
        Path::new("scripts/missions/level07/m7i.mfk"),
        &drifted,
    );
    let findings = super::context::findings_json(&validation.findings);
    if !validation.adaptations.is_empty()
        || !findings.contains("stage-close-with-open-context")
    {
        return Err("drifted legacy adaptation did not fail closed".to_owned());
    }
    Ok(())
}

#[test]
fn p3d_summary_keeps_only_primary_load_reference() -> Result<(), String> {
    let source =
        r#"LoadP3DFile("art\l01_fx.p3d", "GMA_LEVEL_OTHER");"#;
    let mut json = super::super::json::JsonObject::new();
    super::append_summary(
        &mut json,
        source,
        "mfk",
        Path::new("scripts/missions/level01/level.mfk"),
    );
    let rendered = json.finish();
    let value = serde_json::from_str::<serde_json::Value>(&rendered)
        .map_err(|error| error.to_string())?;
    let references = value
        .get("p3d_references")
        .and_then(serde_json::Value::as_array)
        .ok_or_else(|| "P3D reference summary is missing".to_owned())?;
    if references != &[serde_json::Value::String(
        r"art\l01_fx.p3d".to_owned(),
    )] {
        return Err(format!("unexpected P3D references: {references:?}"));
    }
    let arguments = value
        .get("command_invocations")
        .and_then(serde_json::Value::as_array)
        .and_then(|rows| rows.first())
        .and_then(|row| row.get("arguments"))
        .and_then(serde_json::Value::as_array)
        .ok_or_else(|| "load invocation arguments are missing".to_owned())?;
    if arguments.len() != 2
        || arguments.get(1).and_then(serde_json::Value::as_str)
            != Some("GMA_LEVEL_OTHER")
    {
        // jig-ignore-next-line: literal
        return Err("optional LoadP3DFile argument was not preserved".to_owned());
    }
    Ok(())
}
