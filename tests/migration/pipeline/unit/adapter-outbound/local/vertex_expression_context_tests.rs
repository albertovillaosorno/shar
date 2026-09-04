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
//   - Vertex-expression runtime-context regressions.
// - Must-Not:
//   - Depend on the lawful source installation or Unreal state.
// - Allows:
//   - Synthetic normalized mixer, group, skin, and offset-list evidence.
// - Split-When:
//   - Morph-target publication gains separate fixture ownership.
// - Merge-When:
//   - Another test module owns the identical runtime relationship boundary.
// - Summary:
//   - Pins package-local expression-to-offset runtime mapping.
// - Description:
//   - Proves exact key-index scans while preserving legal missing matches.
// - Usage:
//   - Included only by the vertex-expression context adapter under cfg(test).
// - Defaults:
//   - Temporary fixtures are removed after each assertion.
//

//! Vertex-expression runtime-context regressions.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

use super::preflight_vertex_expression_package;

static CASE_ID: AtomicUsize = AtomicUsize::new(0);

type TestResult = Result<(), String>;

fn case_dir(label: &str) -> Result<PathBuf, String> {
    let root = std::env::temp_dir().join(format!(
        "shar-vertex-expression-{label}-{}-{}",
        std::process::id(),
        CASE_ID.fetch_add(1, Ordering::Relaxed),
    ));
    if root.exists() {
        fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    }
    fs::create_dir_all(root.join("components"))
        .map_err(|error| error.to_string())?;
    Ok(root)
}

fn write_component(
    root: &Path,
    ordinal: usize,
    kind: &str,
    file: &str,
    payload: &str,
) -> Result<String, String> {
    let relative = format!("{kind}/{file}.json");
    let path = root.join("components").join(&relative);
    let parent = path.parent().ok_or("component path has no parent")?;
    fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    fs::write(path, payload).map_err(|error| error.to_string())?;
    Ok(format!(
        concat!(
            r#"{{"ordinal":{},"parent_ordinal":0,"kind":"{}","#,
            r#""path":"{}"}}"#,
        ),
        ordinal, kind, relative,
    ))
}

fn write_package(
    root: &Path,
    second_key_index: u32,
    include_second_list: bool,
) -> TestResult {
    let mixer = write_component(
        root,
        1,
        "vertex_expression_mixer",
        "mixer",
        concat!(
            r#"{"schema":"vertex_expression_mixer","type":3,"#,
            r#""target_name":"Face\u0000","#,
            r#""expression_group_name":"Group\u0000"}"#,
        ),
    )?;
    let group = write_component(
        root,
        2,
        "vertex_expression_group",
        "group",
        &format!(
            concat!(
                r#"{{"schema":"vertex_expression_group","name":"Group\u0000","#,
                r#""target_name":"DifferentTarget","expressions":["#,
                r#"{{"keys":[0.5,1.0],"indices":[7,{}]}}]}}"#,
            ),
            second_key_index,
        ),
    )?;
    let second = if include_second_list {
        format!(
            concat!(
                r#",{{"key_index":{},"offsets":["#,
                r#"{{"vertex_index":1,"offset":[1,0,0]}}]}}"#,
            ),
            second_key_index,
        )
    } else {
        String::new()
    };
    let skin = write_component(
        root,
        3,
        "skin",
        "skin",
        &format!(
            concat!(
                r#"{{"schema":"skin","name":"Face\u0000","#,
                r#""expression_offsets":{{"offset_lists":["#,
                r#"{{"key_index":7,"offsets":["#,
                r#"{{"vertex_index":0,"offset":[0,1,0]}},"#,
                r#"{{"vertex_index":2,"offset":[0,0,1]}}]}}{}]}}}}"#,
            ),
            second,
        ),
    )?;
    let ledger = format!(
        "{{\"schema\":\"p3d.package.v1\",\"component_count\":3}}\n\
         {mixer}\n{group}\n{skin}\n"
    );
    fs::write(root.join("components.jsonl"), ledger)
        .map_err(|error| error.to_string())
}

#[test]
fn maps_keys_to_all_matching_offset_lists() -> TestResult {
    let root = case_dir("matching")?;
    write_package(&root, 7, true)?;
    let report = preflight_vertex_expression_package(&root, "pkg")
        .map_err(|error| error.to_string())?;
    fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    if report.mixer_count() != 1 || report.key_count() != 2 {
        return Err("vertex-expression context count drifted".to_owned());
    }
    if report.matched_key_count() != 2 {
        return Err("matching expression keys were not resolved".to_owned());
    }
    let first = report.keys.first().ok_or("first key is missing")?;
    if first.offset_list_indices != [0, 1]
        || first.offset_count != 3
        || first.group_target_matches_mixer
    {
        return Err("runtime key-index scan semantics drifted".to_owned());
    }
    Ok(())
}

#[test]
fn preserves_key_without_offset_list() -> TestResult {
    let root = case_dir("missing-key-list")?;
    write_package(&root, 9, false)?;
    let report = preflight_vertex_expression_package(&root, "pkg")
        .map_err(|error| error.to_string())?;
    fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    if report.key_count() != 2 || report.matched_key_count() != 1 {
        return Err("missing offset list was not preserved as an empty match".to_owned());
    }
    let second = report.keys.get(1).ok_or("second key is missing")?;
    if !second.offset_list_indices.is_empty() || second.offset_count != 0 {
        return Err("missing offset list invented runtime offsets".to_owned());
    }
    Ok(())
}

#[test]
fn rejects_missing_package_local_target_skin() -> TestResult {
    let root = case_dir("missing-skin")?;
    write_package(&root, 9, false)?;
    let skin = root.join("components/skin/skin.json");
    let payload = fs::read_to_string(&skin).map_err(|error| error.to_string())?;
    let changed = payload.replace(r"Face\u0000", r"Other\u0000");
    if changed == payload {
        return Err("target skin fixture identity was not found".to_owned());
    }
    fs::write(&skin, changed).map_err(|error| error.to_string())?;
    let result = preflight_vertex_expression_package(&root, "pkg");
    fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    let Err(error) = result else {
        return Err("missing target skin passed migration preflight".to_owned());
    };
    if !error.to_string().contains("package-local target skin") {
        return Err(format!("unexpected missing-target error: {error}"));
    }
    Ok(())
}
