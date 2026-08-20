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
//   - Component ledger tests unit tests.
// - Must-Not:
//   - Own production behavior or broaden the tested API surface.
// - Allows:
//   - Private test fixtures and assertions for the owning source module.
// - Split-When:
//   - Split when an independent fixture family gains separate ownership.
// - Merge-When:
//   - Merge when another test module owns the identical evidence.
// - Summary:
//   - Component ledger tests unit tests.
// - Description:
//   - Preserves unit-test access through a test-only path module.
// - Usage:
//   - Included only by the owning source module under cfg(test).
// - Defaults:
//   - Test setup and assertions fail explicitly.
//

//! Component ledger tests unit tests.

use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

use super::{
    UnrealHapPackagePlan, component_ledger_files_exist, movie_outputs_complete,
};

static CASE_ID: AtomicUsize = AtomicUsize::new(0);

fn case_dir(label: &str) -> Result<PathBuf, String> {
    let case = std::env::temp_dir().join(format!(
        "shar-pipeline-{label}-{}-{}",
        std::process::id(),
        CASE_ID.fetch_add(1, Ordering::Relaxed),
    ));
    if case.exists() {
        fs::remove_dir_all(&case).map_err(|error| error.to_string())?;
    }
    fs::create_dir_all(case.join("components"))
        .map_err(|error| error.to_string())?;
    Ok(case)
}

#[test]
fn rejects_nested_or_duplicate_component_counts() -> Result<(), String> {
    let case = case_dir("component-count-ownership")?;
    fs::write(case.join("components/component.json"), "{}")
        .map_err(|error| error.to_string())?;
    for (label, header) in [
        (
            "nested",
            concat!(
                "{\"schema\":\"p3d.package.v1\",",
                "\"metadata\":{\"component_count\":1}}",
            ),
        ),
        (
            "duplicate",
            concat!(
                "{\"schema\":\"p3d.package.v1\",",
                "\"component_count\":1,\"component_count\":2}",
            ),
        ),
    ] {
        fs::write(
            case.join("components.jsonl"),
            format!(
                "{header}
{{\"path\":\"component.json\"}}
"
            ),
        )
        .map_err(|error| error.to_string())?;
        if component_ledger_files_exist(&case) {
            return Err(format!("{label} count field must be rejected"));
        }
    }
    fs::remove_dir_all(&case).map_err(|error| error.to_string())?;
    Ok(())
}

#[test]
fn rejects_nested_or_duplicate_component_path_fields() -> Result<(), String> {
    let case = case_dir("component-path-ownership")?;
    fs::write(case.join("components/component.json"), "{}")
        .map_err(|error| error.to_string())?;
    for (label, row) in [
        ("nested", "{\"metadata\":{\"path\":\"component.json\"}}"),
        (
            "duplicate",
            concat!(
                "{\"path\":\"component.json\",",
                "\"path\":\"other.json\"}",
            ),
        ),
    ] {
        fs::write(
            case.join("components.jsonl"),
            format!(
                "{{\"schema\":\"p3d.package.v1\",\"component_count\":1}}
{row}
"
            ),
        )
        .map_err(|error| error.to_string())?;
        if component_ledger_files_exist(&case) {
            return Err(format!("{label} path field must be rejected"));
        }
    }
    fs::remove_dir_all(&case).map_err(|error| error.to_string())?;
    Ok(())
}

#[test]
fn rejects_empty_movie_outputs() -> Result<(), String> {
    let case = case_dir("empty-movie-outputs")?;
    let plan = UnrealHapPackagePlan::for_movie(&case, "intro")
        .map_err(|error| error.to_string())?;
    fs::create_dir_all(&plan.movie_directory)
        .map_err(|error| error.to_string())?;
    fs::write(&plan.hap_video_path, []).map_err(|error| error.to_string())?;
    fs::write(&plan.manifest_path, []).map_err(|error| error.to_string())?;
    for required_path in [
        &plan.source_probe_path,
        &plan.decode_report_path,
        &plan.timing_manifest_path,
    ] {
        fs::write(required_path, []).map_err(|error| error.to_string())?;
    }
    fs::write(plan.movie_directory.join("audio_track_01.wav"), [])
        .map_err(|error| error.to_string())?;
    let complete = movie_outputs_complete(&plan);
    fs::remove_dir_all(&case).map_err(|error| error.to_string())?;
    if complete {
        return Err("zero-byte movie outputs must be incomplete".to_owned());
    }
    Ok(())
}

#[test]
fn rejects_invalid_unrelated_json_scalars() -> Result<(), String> {
    let case = case_dir("invalid-json-scalars")?;
    fs::write(case.join("components/component.json"), "{}")
        .map_err(|error| error.to_string())?;
    for invalid in ["garbage", "01", "1.", "1e"] {
        let header = format!(
            "{{\"schema\":\"p3d.package.v1\",\"component_count\":1,\"\
                 broken\":{invalid}}}"
        );
        fs::write(
            case.join("components.jsonl"),
            format!(
                "{header}
{{\"path\":\"component.json\"}}
"
            ),
        )
        .map_err(|error| error.to_string())?;
        if component_ledger_files_exist(&case) {
            return Err(format!("invalid scalar {invalid} must be rejected"));
        }
    }
    fs::remove_dir_all(&case).map_err(|error| error.to_string())?;
    Ok(())
}

#[test]
fn rejects_trailing_component_json_commas() -> Result<(), String> {
    let case = case_dir("trailing-json-commas")?;
    fs::write(case.join("components/component.json"), "{}")
        .map_err(|error| error.to_string())?;
    for (label, header, row) in [
        (
            "header",
            concat!(
                "{\"schema\":\"p3d.package.v1\",",
                "\"component_count\":1,}",
            ),
            "{\"path\":\"component.json\"}",
        ),
        (
            "row",
            concat!(
                "{\"schema\":\"p3d.package.v1\",",
                "\"component_count\":1}",
            ),
            "{\"path\":\"component.json\",}",
        ),
    ] {
        fs::write(
            case.join("components.jsonl"),
            format!(
                "{header}
{row}
"
            ),
        )
        .map_err(|error| error.to_string())?;
        if component_ledger_files_exist(&case) {
            return Err(format!("trailing {label} comma must be rejected"));
        }
    }
    fs::remove_dir_all(&case).map_err(|error| error.to_string())?;
    Ok(())
}

#[test]
fn rejects_unframed_or_trailing_component_json() -> Result<(), String> {
    let case = case_dir("malformed-component-json")?;
    fs::write(case.join("components/component.json"), "{}")
        .map_err(|error| error.to_string())?;
    for (label, row) in [
        ("unframed", "\"path\":\"component.json\""),
        ("trailing", "{\"path\":\"component.json\"}garbage"),
    ] {
        fs::write(
            case.join("components.jsonl"),
            format!(
                "{{\"schema\":\"p3d.package.v1\",\"component_count\":1}}
{row}
"
            ),
        )
        .map_err(|error| error.to_string())?;
        if component_ledger_files_exist(&case) {
            return Err(format!("{label} JSON row must be rejected"));
        }
    }
    fs::remove_dir_all(&case).map_err(|error| error.to_string())?;
    Ok(())
}

#[test]
fn rejects_component_paths_outside_the_package() -> Result<(), String> {
    let case = case_dir("outside-paths")?;
    let outside = case.join("outside.json");
    fs::write(&outside, "{}").map_err(|error| error.to_string())?;
    let absolute = outside.to_string_lossy().replace('\\', "\\\\");
    for (label, path) in [
        ("parent", "../outside.json".to_owned()),
        ("absolute", absolute),
    ] {
        fs::write(
            case.join("components.jsonl"),
            format!(
                "{{\"schema\":\"p3d.package.v1\",\"component_count\":1}}
                     {{\"path\":\"{path}\"}}
"
            ),
        )
        .map_err(|error| error.to_string())?;
        if component_ledger_files_exist(&case) {
            return Err(format!("{label} path must be rejected"));
        }
    }
    fs::remove_dir_all(&case).map_err(|error| error.to_string())?;
    Ok(())
}

#[test]
fn rejects_case_aliases_for_component_paths() -> Result<(), String> {
    let case = case_dir("case-aliases")?;
    fs::write(case.join("components/component.json"), "{}")
        .map_err(|error| error.to_string())?;
    fs::write(
        case.join("components.jsonl"),
        concat!(
            "{\"schema\":\"p3d.package.v1\",",
            "\"component_count\":2}
",
            "{\"path\":\"component.json\"}
",
            "{\"path\":\"COMPONENT.JSON\"}
",
        ),
    )
    .map_err(|error| error.to_string())?;
    let complete = component_ledger_files_exist(&case);
    fs::remove_dir_all(&case).map_err(|error| error.to_string())?;
    if complete {
        return Err("case aliases must not count twice".to_owned());
    }
    Ok(())
}

#[test]
fn rejects_duplicate_component_paths() -> Result<(), String> {
    let case = case_dir("duplicate-paths")?;
    fs::write(case.join("components/component.json"), "{}")
        .map_err(|error| error.to_string())?;
    let row = "{\"path\":\"component.json\"}";
    fs::write(
        case.join("components.jsonl"),
        format!(
            "{{\"schema\":\"p3d.package.v1\",\"component_count\":2}}
{row}
{row}
"
        ),
    )
    .map_err(|error| error.to_string())?;
    let complete = component_ledger_files_exist(&case);
    fs::remove_dir_all(&case).map_err(|error| error.to_string())?;
    if complete {
        return Err("duplicate component paths must be rejected".to_owned());
    }
    Ok(())
}

#[test]
fn rejects_invalid_component_ledger_counts() -> Result<(), String> {
    for (label, count_field) in [
        ("mismatch", "\"component_count\":2"),
        ("missing", "\"other_count\":1"),
        ("nonnumeric", "\"component_count\":\"1\""),
    ] {
        let case = case_dir(label)?;
        fs::write(case.join("components/component.json"), "{}")
            .map_err(|error| error.to_string())?;
        let header = format!("{{\"schema\":\"p3d.package.v1\",{count_field}}}");
        fs::write(
            case.join("components.jsonl"),
            format!(
                "{header}
{{\"path\":\"component.json\"}}
"
            ),
        )
        .map_err(|error| error.to_string())?;
        let complete = component_ledger_files_exist(&case);
        fs::remove_dir_all(&case).map_err(|error| error.to_string())?;
        if complete {
            return Err(format!("{label} count must be rejected"));
        }
    }
    Ok(())
}

#[test]
fn rejects_invalid_component_ledger_headers() -> Result<(), String> {
    for (label, header) in [
        ("wrong-schema", "{\"schema\":\"wrong\"}"),
        ("missing-schema", "{\"component_count\":1}"),
    ] {
        let case = case_dir(label)?;
        fs::write(case.join("components/component.json"), "{}")
            .map_err(|error| error.to_string())?;
        fs::write(
            case.join("components.jsonl"),
            format!(
                "{header}
{{\"path\":\"component.json\"}}
"
            ),
        )
        .map_err(|error| error.to_string())?;
        let complete = component_ledger_files_exist(&case);
        fs::remove_dir_all(&case).map_err(|error| error.to_string())?;
        if complete {
            return Err(format!("{label} header must be rejected"));
        }
    }
    Ok(())
}

#[test]
fn rejects_empty_component_ledgers() -> Result<(), String> {
    for (label, contents) in [
        ("empty", ""),
        (
            "header-only",
            concat!(
                "{\"schema\":\"p3d.package.v1\",",
                "\"byte_len\":0,\"chunk_count\":0,",
                "\"component_count\":0}\n",
            ),
        ),
    ] {
        let case = case_dir(label)?;
        fs::write(case.join("components.jsonl"), contents)
            .map_err(|error| error.to_string())?;
        let complete = component_ledger_files_exist(&case);
        fs::remove_dir_all(&case).map_err(|error| error.to_string())?;
        if complete {
            return Err(format!("{label} ledger must remain incomplete"));
        }
    }
    Ok(())
}
