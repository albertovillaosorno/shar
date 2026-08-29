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
//   - Scrooby page-resource lifecycle publication regressions.
// - Must-Not:
//   - Depend on lawful source files or Unreal editor state.
// - Allows:
//   - Synthetic public-safe lifecycle rows and temporary publication roots.
// - Split-When:
//   - Resource execution gains independently testable state transitions.
// - Merge-When:
//   - Another test module owns the identical lifecycle publication behavior.
// - Summary:
//   - Pin deterministic public-safe Scrooby preload evidence.
// - Description:
//   - Proves page-owned preload order, backing summaries, and atomic reuse
//     without exposing authored resource identities.
// - Usage:
//   - Included only by the Scrooby resource-lifecycle adapter under cfg(test).
// - Defaults:
//   - Temporary fixtures are removed after assertions.
//

//! Scrooby page-resource lifecycle publication regressions.

use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

use super::{publish_rendered, render_catalog, summarize};
use crate::adapters::driven::local::ui_scrooby_project::
    ScroobyPageResourceLifecycle;

static CASE_ID: AtomicUsize = AtomicUsize::new(0);

type TestResult = Result<(), String>;

fn case_dir(label: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "shar-scrooby-resource-{label}-{}-{}",
        std::process::id(),
        CASE_ID.fetch_add(1, Ordering::Relaxed),
    ))
}

fn row(
    package_id: &str,
    page_ordinal: usize,
    source_index: usize,
    resource_kind: &'static str,
    target_ordinal: usize,
    source: Option<&str>,
) -> ScroobyPageResourceLifecycle {
    ScroobyPageResourceLifecycle {
        package_id: package_id.to_owned(),
        page_ordinal,
        source_index,
        resource_kind,
        target_ordinal,
        target_source_unit_id: source.map(str::to_owned),
        target_source_match_basis: source.map(|_| "filename-basename-exact"),
        target_package_id: None,
        target_package_match_basis: None,
        target_entity_ordinal: None,
        target_entity_match_basis: None,
        target_content_sha256: None,
        target_content_match_basis: None,
    }
}

#[test]
// jig-ignore-next-line: long identifier
fn renders_page_owned_preloads_without_authored_resource_identity() -> TestResult {
    let mut package_backed = row("package-a", 7, 4, "pure3d", 13, None);
    package_backed.target_package_id = Some("resource-package".to_owned());
    package_backed.target_package_match_basis =
        Some("project-resource-path-exact");
    let mut entity_backed = row("package-a", 7, 5, "image", 11, None);
    entity_backed.target_package_id = Some("package-a".to_owned());
    entity_backed.target_package_match_basis =
        Some("owner-joined-sprite-exact");
    entity_backed.target_entity_ordinal = Some(29);
    entity_backed.target_entity_match_basis = Some("full-filename-exact");
    let mut content_backed = row("package-a", 7, 6, "image", 15, None);
    content_backed.target_content_sha256 = Some("a".repeat(64));
    content_backed.target_content_match_basis =
        Some("equivalent-scoped-ui-raster-sha256");
    let rows = vec![
        row("package-a", 7, 2, "image", 11, Some("source-a")),
        package_backed,
        entity_backed,
        content_backed,
    ];
    let summary = summarize(&rows).map_err(|error| error.to_string())?;
    if summary.preload_count != 4
        || summary.direct_import_backed_preload_count != 1
        || summary.normalized_package_backed_preload_count != 2
        || summary.normalized_entity_backed_preload_count != 1
        || summary.equivalent_content_backed_preload_count != 1
        || summary.fully_direct_import_backed_package_count != 0
    {
        return Err(format!("unexpected lifecycle summary: {summary:?}"));
    }
    let rendered = render_catalog(2, &rows, summary)
        .map_err(|error| error.to_string())?;
    if rendered.contains(".png") || rendered.contains('/') {
        return Err(
            "lifecycle catalog exposed an authored resource path".into(),
        );
    }
    let values = rendered
        .lines()
        .map(serde_json::from_str::<serde_json::Value>)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| error.to_string())?;
    let header = values.first().ok_or("lifecycle header is missing")?;
    if header.get("preload_count").and_then(serde_json::Value::as_u64)
        != Some(4)
        || header
            .get("direct_import_backed_preload_count")
            .and_then(serde_json::Value::as_u64)
            != Some(1)
        || header
            .get("normalized_entity_backed_preload_count")
            .and_then(serde_json::Value::as_u64)
            != Some(1)
        || header
            .get("equivalent_content_backed_preload_count")
            .and_then(serde_json::Value::as_u64)
            != Some(1)
    {
        return Err(format!("unexpected lifecycle header: {header}"));
    }
    let payload = values
        .get(1..)
        .ok_or("lifecycle payload is missing")?;
    let indices = payload
        .iter()
        .map(|value| {
            value
                .get("source_index")
                .and_then(serde_json::Value::as_u64)
        })
        .collect::<Vec<_>>();
    if indices != [Some(2), Some(4), Some(5), Some(6)]
        || payload.get(1).and_then(|value| value.get("target_package_id"))
            .and_then(serde_json::Value::as_str)
            != Some("resource-package")
        || payload
            .get(1)
            .and_then(|value| value.get("target_package_match_basis"))
            .and_then(serde_json::Value::as_str)
            != Some("project-resource-path-exact")
        || payload
            .get(2)
            .and_then(|value| value.get("target_entity_ordinal"))
            .and_then(serde_json::Value::as_u64)
            != Some(29)
        || payload
            .get(2)
            .and_then(|value| value.get("target_entity_match_basis"))
            .and_then(serde_json::Value::as_str)
            != Some("full-filename-exact")
        || payload
            .get(3)
            .and_then(|value| value.get("target_content_sha256"))
            .and_then(serde_json::Value::as_str)
            != Some(concat!(
                "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            ))
        || payload
            .get(3)
            .and_then(|value| value.get("target_content_match_basis"))
            .and_then(serde_json::Value::as_str)
            != Some("equivalent-scoped-ui-raster-sha256")
        || payload.iter().any(|value| {
            value.get("load_policy").and_then(serde_json::Value::as_str)
                != Some("eager-page-preload")
                || value
                    .get("instance_policy")
                    .and_then(serde_json::Value::as_str)
                    != Some("per-page-declaration-occurrence")
                || value
                    .get("lifetime_owner")
                    .and_then(serde_json::Value::as_str)
                    != Some("page")
                || value
                    .get("release_policy")
                    .and_then(serde_json::Value::as_str)
                    != Some("page-destruction")
        })
    {
        return Err(format!("unexpected lifecycle rows: {payload:?}"));
    }
    Ok(())
}

#[test]
fn rejects_incomplete_direct_import_backing_identity() -> TestResult {
    let mut incomplete = row("package-a", 7, 2, "image", 11, Some("source-a"));
    incomplete.target_source_match_basis = None;
    let result = summarize(&[incomplete]);
    let Err(error) = result else {
        return Err("incomplete lifecycle backing identity was accepted".into());
    };
    if !error.to_string().contains("backing identity is incomplete") {
        return Err(format!("unexpected lifecycle identity error: {error}"));
    }
    Ok(())
}

#[test]
fn rejects_incomplete_normalized_package_backing_identity() -> TestResult {
    let mut incomplete = row("package-a", 7, 2, "pure3d", 11, None);
    incomplete.target_package_id = Some("resource-package".to_owned());
    let result = summarize(&[incomplete]);
    let Err(error) = result else {
        return Err(
            "incomplete normalized package identity was accepted".into(),
        );
    };
    if !error
        .to_string()
        .contains("resource package backing identity is incomplete")
    {
        return Err(format!("unexpected package identity error: {error}"));
    }
    Ok(())
}

#[test]
fn rejects_incomplete_equivalent_content_backing_identity() -> TestResult {
    let mut incomplete = row("package-a", 7, 2, "image", 11, None);
    incomplete.target_content_sha256 = Some("a".repeat(64));
    let result = summarize(&[incomplete]);
    let Err(error) = result else {
        return Err("incomplete content identity was accepted".into());
    };
    if !error
        .to_string()
        .contains("content backing identity is incomplete")
    {
        return Err(format!("unexpected content identity error: {error}"));
    }
    Ok(())
}

#[test]
fn rejects_incomplete_normalized_entity_backing_identity() -> TestResult {
    let mut incomplete = row("package-a", 7, 2, "image", 11, None);
    incomplete.target_package_id = Some("package-a".to_owned());
    incomplete.target_package_match_basis = Some("owner-joined-sprite-exact");
    incomplete.target_entity_ordinal = Some(29);
    let result = summarize(&[incomplete]);
    let Err(error) = result else {
        return Err("incomplete normalized entity identity was accepted".into());
    };
    if !error.to_string().contains("entity backing identity is incomplete") {
        return Err(format!("unexpected entity identity error: {error}"));
    }
    Ok(())
}

#[test]
fn rejects_entity_backing_without_package_identity() -> TestResult {
    let mut incomplete = row("package-a", 7, 2, "image", 11, None);
    incomplete.target_entity_ordinal = Some(29);
    incomplete.target_entity_match_basis = Some("full-filename-exact");
    let result = summarize(&[incomplete]);
    let Err(error) = result else {
        return Err(
            "package-less normalized entity identity was accepted".into(),
        );
    };
    if !error.to_string().contains("has no package identity") {
        return Err(format!("unexpected entity package error: {error}"));
    }
    Ok(())
}

#[test]
// jig-ignore-next-line: long identifier
fn publication_reuses_exact_catalog_and_rejects_transaction_debris() -> TestResult {
    let output = case_dir("publish");
    let rows = vec![row("package-a", 7, 2, "image", 11, None)];
    let summary = summarize(&rows).map_err(|error| error.to_string())?;
    let rendered = render_catalog(1, &rows, summary)
        .map_err(|error| error.to_string())?;
    publish_rendered(&output, &rendered).map_err(|error| error.to_string())?;
    publish_rendered(&output, &rendered).map_err(|error| error.to_string())?;
    let accepted = fs::read_to_string(output.join("lifecycle.jsonl"))
        .map_err(|error| error.to_string())?;
    let name = output.file_name().and_then(|value| value.to_str())
        .ok_or("resource output has no name")?;
    let parent = output.parent().ok_or("resource output has no parent")?;
    for (suffix, expected) in [
        ("complete-staging", "staging already exists"),
        ("complete-backup", "backup already exists"),
    ] {
        let debris = parent.join(format!(".{name}.{suffix}"));
        fs::create_dir_all(&debris).map_err(|error| error.to_string())?;
        let result = publish_rendered(&output, &rendered);
        fs::remove_dir_all(&debris).map_err(|error| error.to_string())?;
        let Err(error) = result else {
            return Err(
                "resource lifecycle reuse accepted transaction debris".into(),
            );
        };
        if !error.to_string().contains(expected) {
            return Err(format!("unexpected resource debris error: {error}"));
        }
        let unchanged = fs::read_to_string(output.join("lifecycle.jsonl"))
            .map_err(|error| error.to_string())?;
        if unchanged != accepted {
            return Err(
                "transaction debris changed accepted lifecycle catalog".into(),
            );
        }
    }
    fs::remove_dir_all(&output).map_err(|error| error.to_string())?;
    Ok(())
}
