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
//   - Deterministic Scrooby page-resource preload and lifetime evidence.
// - Must-Not:
//   - Publish authored resource names or paths, invent widget consumers, or
//     claim an unresolved preload has an Unreal backing asset.
// - Allows:
//   - Preserve package-local page ownership, page-local declaration order,
//     resource kind, and verified direct-import backing identities.
// - Split-When:
//   - Unreal runtime resource ownership gains an independent execution path.
// - Merge-When:
//   - Another artifact owns the identical page-resource lifecycle contract.
// - Summary:
//   - Publish public-safe Scrooby page-resource lifecycle evidence.
// - Description:
//   - Records every normalized page resource declaration as an eager preload
//     owned for the page lifetime, retaining exact page-local multiplicity and
//     declaration order without publishing authored names or filesystem paths.
// - Usage:
//   - Called by prepare-unreal after semantic binding and direct-import
//     resolution.
// - Defaults:
//   - Incomplete direct-import identities and unsafe transaction state fail
//     closed.
//

//! Scrooby page-resource lifecycle evidence publication.

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use serde_json::json;

use crate::domain::{PipelineError, PipelineOutcome};

use super::ui_scrooby_project::{
    ScroobyPageResourceLifecycle, ScroobyUiPreflight,
};

const SCHEMA: &str = "shar-schoenwald.scrooby-resource-lifecycle.v4";
const FILE: &str = "lifecycle.jsonl";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct ScroobyResourceLifecycleSummary {
    pub(super) preload_count: usize,
    pub(super) direct_import_backed_preload_count: usize,
    pub(super) normalized_package_backed_preload_count: usize,
    pub(super) normalized_entity_backed_preload_count: usize,
    pub(super) fully_direct_import_backed_package_count: usize,
}

pub(super) fn publish_scrooby_resource_lifecycle_catalog(
    preflight: &ScroobyUiPreflight,
    output_root: &Path,
) -> PipelineOutcome<ScroobyResourceLifecycleSummary> {
    let rows = preflight.page_resource_lifecycle();
    let summary = summarize(&rows)?;
    let rendered = render_catalog(preflight.package_count(), &rows, summary)?;
    publish_rendered(output_root, &rendered)?;
    Ok(summary)
}

fn summarize(
    rows: &[ScroobyPageResourceLifecycle],
) -> PipelineOutcome<ScroobyResourceLifecycleSummary> {
    let mut backed = 0usize;
    let mut package_backed = 0usize;
    let mut entity_backed = 0usize;
    let mut packages = BTreeMap::<&str, (usize, usize)>::new();
    for row in rows {
        let has_source = row.target_source_unit_id.is_some();
        let has_basis = row.target_source_match_basis.is_some();
        if has_source != has_basis {
            return Err(PipelineError::new(
                "Scrooby resource lifecycle backing identity is incomplete",
            ));
        }
        let has_package = row.target_package_id.is_some();
        let has_package_basis = row.target_package_match_basis.is_some();
        if has_package != has_package_basis {
            return Err(PipelineError::new(
                "Scrooby resource package backing identity is incomplete",
            ));
        }
        if has_package {
            package_backed = package_backed.saturating_add(1);
        }
        let has_entity = row.target_entity_ordinal.is_some();
        let has_entity_basis = row.target_entity_match_basis.is_some();
        if has_entity != has_entity_basis {
            return Err(PipelineError::new(
                "Scrooby resource entity backing identity is incomplete",
            ));
        }
        if has_entity && !has_package {
            return Err(PipelineError::new(
                "Scrooby resource entity backing has no package identity",
            ));
        }
        if has_entity {
            entity_backed = entity_backed.saturating_add(1);
        }
        let counts = packages.entry(&row.package_id).or_default();
        counts.0 = counts.0.saturating_add(1);
        if has_source {
            backed = backed.saturating_add(1);
            counts.1 = counts.1.saturating_add(1);
        }
    }
    let fully_backed = packages
        .values()
        .filter(|(total, resolved)| *total > 0 && total == resolved)
        .count();
    Ok(ScroobyResourceLifecycleSummary {
        preload_count: rows.len(),
        direct_import_backed_preload_count: backed,
        normalized_package_backed_preload_count: package_backed,
        normalized_entity_backed_preload_count: entity_backed,
        fully_direct_import_backed_package_count: fully_backed,
    })
}

fn render_catalog(
    package_count: usize,
    rows: &[ScroobyPageResourceLifecycle],
    summary: ScroobyResourceLifecycleSummary,
) -> PipelineOutcome<String> {
    let mut output = String::new();
    output.push_str(&serde_json::to_string(&json!({
        "schema": SCHEMA,
        "record_type": "header",
        "status": "complete",
        "package_count": package_count,
        "preload_count": summary.preload_count,
        "direct_import_backed_preload_count":
            summary.direct_import_backed_preload_count,
        "normalized_package_backed_preload_count":
            summary.normalized_package_backed_preload_count,
        "normalized_entity_backed_preload_count":
            summary.normalized_entity_backed_preload_count,
        "fully_direct_import_backed_package_count":
            summary.fully_direct_import_backed_package_count,
    }))
    .map_err(|error| {
        PipelineError::new(format!(
            "Scrooby resource lifecycle JSON failed: {error}"
        ))
    })?);
    output.push('\n');
    for row in rows {
        let mut value = json!({
            "schema": SCHEMA,
            "record_type": "page-resource-preload",
            "package_id": row.package_id,
            "page_ordinal": row.page_ordinal,
            "source_index": row.source_index,
            "resource_kind": row.resource_kind,
            "target_ordinal": row.target_ordinal,
            "load_policy": "eager-page-preload",
            "instance_policy": "per-page-declaration-occurrence",
            "lifetime_owner": "page",
            "release_policy": "page-destruction",
        });
        match (
            &row.target_source_unit_id,
            row.target_source_match_basis,
        ) {
            (Some(source_unit_id), Some(match_basis)) => {
                let object = value.as_object_mut().ok_or_else(|| {
                    PipelineError::new(
                        "Scrooby resource lifecycle row is not an object",
                    )
                })?;
                let _previous = object.insert(
                    "target_source_unit_id".to_owned(),
                    json!(source_unit_id),
                );
                let _previous = object.insert(
                    "target_source_match_basis".to_owned(),
                    json!(match_basis),
                );
            },
            (None, None) => {},
            _ => {
                return Err(PipelineError::new(
                    "Scrooby resource lifecycle backing identity is incomplete",
                ));
            },
        }
        match (&row.target_package_id, row.target_package_match_basis) {
            (Some(package_id), Some(match_basis)) => {
                let object = value.as_object_mut().ok_or_else(|| {
                    PipelineError::new(
                        "Scrooby resource lifecycle row is not an object",
                    )
                })?;
                let _previous = object.insert(
                    "target_package_id".to_owned(),
                    json!(package_id),
                );
                let _previous = object.insert(
                    "target_package_match_basis".to_owned(),
                    json!(match_basis),
                );
            },
            (None, None) => {},
            _ => {
                return Err(PipelineError::new(
                    "Scrooby resource package backing identity is incomplete",
                ));
            },
        }
        match (
            row.target_entity_ordinal,
            row.target_entity_match_basis,
        ) {
            (Some(entity_ordinal), Some(match_basis)) => {
                let object = value.as_object_mut().ok_or_else(|| {
                    PipelineError::new(
                        "Scrooby resource lifecycle row is not an object",
                    )
                })?;
                let _previous = object.insert(
                    "target_entity_ordinal".to_owned(),
                    json!(entity_ordinal),
                );
                let _previous = object.insert(
                    "target_entity_match_basis".to_owned(),
                    json!(match_basis),
                );
            },
            (None, None) => {},
            _ => {
                return Err(PipelineError::new(
                    "Scrooby resource entity backing identity is incomplete",
                ));
            },
        }
        output.push_str(&serde_json::to_string(&value).map_err(|error| {
            PipelineError::new(format!(
                "Scrooby resource lifecycle JSON failed: {error}"
            ))
        })?);
        output.push('\n');
    }
    Ok(output)
}

fn publish_rendered(output_root: &Path, rendered: &str) -> PipelineOutcome<()> {
    let name = output_root
        .file_name()
        .and_then(|value| value.to_str())
        .filter(|value| !value.is_empty())
        .ok_or_else(|| {
            PipelineError::new(
                "Scrooby resource lifecycle output has no portable name",
            )
        })?;
    let parent = output_root.parent().unwrap_or_else(|| Path::new("."));
    fs::create_dir_all(parent).map_err(|error| {
        io_error("create Scrooby resource lifecycle parent", &error)
    })?;
    let staging = parent.join(format!(".{name}.complete-staging"));
    let backup = parent.join(format!(".{name}.complete-backup"));
    ensure_absent(&staging, "Scrooby resource lifecycle staging")?;
    ensure_absent(&backup, "Scrooby resource lifecycle backup")?;
    let catalog = output_root.join(FILE);
    if let Ok(metadata) = fs::symlink_metadata(output_root) {
        if !metadata.is_dir() || metadata.file_type().is_symlink() {
            return Err(PipelineError::new(
                "Scrooby resource lifecycle output is not a regular directory",
            ));
        }
        if fs::read_to_string(&catalog).ok().as_deref() == Some(rendered) {
            return Ok(());
        }
    }

    fs::create_dir_all(&staging).map_err(|error| {
        io_error("create Scrooby resource lifecycle staging", &error)
    })?;
    let staged_catalog = staging.join(FILE);
    if let Err(error) = fs::write(&staged_catalog, rendered) {
        let _cleanup = fs::remove_dir_all(&staging);
        return Err(io_error("write Scrooby resource lifecycle", &error));
    }
    if fs::read_to_string(&staged_catalog)
        .map_err(|error| {
            io_error("read staged Scrooby resource lifecycle", &error)
        })?
        != rendered
    {
        let _cleanup = fs::remove_dir_all(&staging);
        return Err(PipelineError::new(
            "staged Scrooby resource lifecycle changed during read-back",
        ));
    }

    let had_output = match fs::symlink_metadata(output_root) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => false,
        Err(error) => {
            let _cleanup = fs::remove_dir_all(&staging);
            return Err(io_error(
                "inspect Scrooby resource lifecycle output",
                &error,
            ));
        },
        Ok(metadata) => {
            if !metadata.is_dir() || metadata.file_type().is_symlink() {
                let _cleanup = fs::remove_dir_all(&staging);
                return Err(PipelineError::new(
                    concat!(
                        "Scrooby resource lifecycle output is not a regular ",
                        "directory",
                    ),
                ));
            }
            if let Err(error) = fs::rename(output_root, &backup) {
                let _cleanup = fs::remove_dir_all(&staging);
                return Err(io_error(
                    "back up Scrooby resource lifecycle output",
                    &error,
                ));
            }
            true
        },
    };
    if let Err(error) = fs::rename(&staging, output_root) {
        let publish_error =
            io_error("publish Scrooby resource lifecycle", &error);
        let cleanup_error = fs::remove_dir_all(&staging).err();
        let rollback_error = had_output
            .then(|| fs::rename(&backup, output_root).err())
            .flatten();
        return match (rollback_error, cleanup_error) {
            (None, None) => Err(publish_error),
            (rollback, cleanup) => Err(PipelineError::new(format!(
                "{publish_error}; rollback={:?}; cleanup={:?}",
                rollback.map(|value| value.kind()),
                cleanup.map(|value| value.kind()),
            ))),
        };
    }

    let published = fs::read_to_string(output_root.join(FILE)).map_err(|error| {
        io_error("read published Scrooby resource lifecycle", &error)
    })?;
    if published != rendered {
        rollback_catalog(output_root, &backup, had_output)?;
        return Err(PipelineError::new(
            "published Scrooby resource lifecycle changed during read-back",
        ));
    }
    if had_output {
        fs::remove_dir_all(&backup).map_err(|error| {
            io_error("remove Scrooby resource lifecycle backup", &error)
        })?;
    }
    Ok(())
}

fn ensure_absent(path: &Path, label: &str) -> PipelineOutcome<()> {
    match fs::symlink_metadata(path) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(io_error(
            "inspect Scrooby resource lifecycle transaction",
            &error,
        )),
        Ok(_metadata) => Err(PipelineError::new(format!(
            "{label} already exists"
        ))),
    }
}

fn rollback_catalog(
    output_root: &Path,
    backup: &Path,
    had_output: bool,
) -> PipelineOutcome<()> {
    fs::remove_dir_all(output_root).map_err(|error| {
        io_error("remove invalid Scrooby resource lifecycle", &error)
    })?;
    if had_output {
        fs::rename(backup, output_root).map_err(|error| {
            io_error("restore previous Scrooby resource lifecycle", &error)
        })?;
    }
    Ok(())
}

fn io_error(action: &str, error: &std::io::Error) -> PipelineError {
    PipelineError::new(format!("{action} failed: {error}"))
}

#[cfg(test)]
// jig-ignore-next-line: canonical test module path is indivisible.
#[path = "../../../../../../tests/migration/pipeline/unit/adapter-outbound/local/ui_scrooby_resources_tests.rs"]
mod tests;
