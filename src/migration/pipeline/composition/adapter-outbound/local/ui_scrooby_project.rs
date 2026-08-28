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
//   - Semantic preflight of normalized Scrooby project references.
// - Must-Not:
//   - Invent widget runtime behavior, signed layout semantics, or Unreal
//     assets.
// - Allows:
//   - Resolve exact package-local authored identities across normalized UI
//     data.
// - Split-When:
//   - UI artifact publication or WidgetBlueprint construction gains a
//     lifecycle.
// - Merge-When:
//   - Another adapter owns the identical Scrooby semantic binding preflight.
// - Summary:
//   - Validate package-local Scrooby references before Unreal planning.
// - Description:
//   - Requires every observed screen, image, style, text-bible, and Pure3D
//     reference to resolve to exactly one normalized resource declaration.
// - Usage:
//   - Called by prepare-unreal after canonical package-index intake.
// - Defaults:
//   - Missing, ambiguous, malformed, or cross-package references fail closed.
//

//! Normalized Scrooby project semantic preflight.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use schoenwald_filesystem::resolve_under;
use serde_json::Value;

use crate::domain::{PhaseThreePackageIndex, PipelineError, PipelineOutcome};

#[derive(Clone, Debug, Eq, PartialEq)]
struct LedgerRow {
    ordinal: usize,
    parent_ordinal: Option<usize>,
    kind: String,
    path: String,
}

#[derive(Clone, Debug)]
struct Component {
    row: LedgerRow,
    payload: Value,
}

/// Validate every indexed normalized Scrooby project package.
///
/// # Errors
///
/// Returns an error when indexed Scrooby evidence disagrees with the normalized
/// ledger or any authored package-local reference is missing or ambiguous.
pub(super) fn preflight_scrooby_ui_projects(
    index: &PhaseThreePackageIndex,
    extracted_root: &Path,
) -> PipelineOutcome<usize> {
    let mut count = 0usize;
    for package in index.packages() {
        let scrooby_members = package
            .members()
            .iter()
            .filter(|member| member.source_chunk_kind.starts_with("scrooby_"))
            .count();
        if scrooby_members == 0 {
            continue;
        }
        let project_count = package
            .members()
            .iter()
            .filter(|member| member.source_chunk_kind == "scrooby_project")
            .count();
        if project_count != 1
            || !matches!(package.category(), "ui-screens" | "language")
        {
            return Err(PipelineError::new(
                "Scrooby project package has an unsupported package shape",
            ));
        }
        let root = resolve_normalized_package_root(
            extracted_root,
            &package.package_root,
        )?;
        preflight_scrooby_package(&root)?;
        let indexed = package
            .members()
            .iter()
            .filter(|member| member.source_chunk_kind.starts_with("scrooby_"))
            .map(|member| member.path.as_str())
            .collect::<BTreeSet<_>>();
        let root_name = extracted_root
            .file_name()
            .and_then(|value| value.to_str())
            .ok_or_else(|| {
                PipelineError::new(
                    "Scrooby extracted root has no portable name",
                )
            })?;
        let prefix = format!("{root_name}/");
        let package_relative = package
            .package_root
            .strip_prefix(&prefix)
            .unwrap_or(&package.package_root);
        let ledger = read_ledger(&root)?;
        let expected = ledger
            .iter()
            .filter(|row| row.kind.starts_with("scrooby_"))
            .map(|row| {
                format!(
                    "{root_name}/{package_relative}/components/{}",
                    row.path,
                )
            })
            .collect::<BTreeSet<_>>();
        let expected_refs = expected.iter().map(String::as_str).collect();
        if indexed != expected_refs {
            return Err(PipelineError::new(
                "Scrooby package index disagrees with normalized components",
            ));
        }
        count = count.checked_add(1).ok_or_else(|| {
            PipelineError::new("Scrooby project count overflowed")
        })?;
    }
    Ok(count)
}

fn resolve_normalized_package_root(
    extracted_root: &Path,
    published_root: &str,
) -> PipelineOutcome<PathBuf> {
    let root_name = extracted_root
        .file_name()
        .and_then(|value| value.to_str())
        .filter(|value| !value.is_empty())
        .ok_or_else(|| {
            PipelineError::new(
                "Scrooby extracted root has no portable basename",
            )
        })?;
    let prefix = format!("{root_name}/");
    let relative = published_root.strip_prefix(&prefix).ok_or_else(|| {
        PipelineError::new("Scrooby package root is outside extracted evidence")
    })?;
    resolve_under(extracted_root, Path::new(relative)).map_err(|_error| {
        PipelineError::new("Scrooby package root escapes extracted evidence")
    })
}

fn preflight_scrooby_package(root: &Path) -> PipelineOutcome<()> {
    let rows = read_ledger(root)?;
    let components = root.join("components");
    let mut decoded = Vec::new();
    for row in rows.into_iter().filter(|row| row.kind.starts_with("scrooby_")) {
        let path = resolve_under(&components, Path::new(&row.path)).map_err(
            |_error| {
                PipelineError::new("Scrooby component path escapes package")
            },
        )?;
        let bytes = fs::read(&path)
            .map_err(|error| io_error("read Scrooby component", &error))?;
        let payload = serde_json::from_slice::<Value>(&bytes).map_err(|error| {
            PipelineError::new(format!(
                "Scrooby component JSON failed: {error}"
            ))
        })?;
        if payload.get("schema").and_then(Value::as_str) != Some(&row.kind) {
            return Err(PipelineError::new(
                "Scrooby component schema disagrees with ledger kind",
            ));
        }
        decoded.push(Component { row, payload });
    }
    if decoded
        .iter()
        .filter(|component| component.row.kind == "scrooby_project")
        .count()
        != 1
    {
        return Err(PipelineError::new(
            "Scrooby package must contain exactly one project",
        ));
    }
    validate_layout_children(&decoded)?;
    validate_screen_pages(&decoded)?;
    validate_widget_resources(&decoded)
}

fn read_ledger(root: &Path) -> PipelineOutcome<Vec<LedgerRow>> {
    let path = root.join("components.jsonl");
    let text = fs::read_to_string(&path)
        .map_err(|error| io_error("read Scrooby component ledger", &error))?;
    let mut rows = Vec::new();
    for line in text.lines().skip(1) {
        let value = serde_json::from_str::<Value>(line).map_err(|error| {
            PipelineError::new(format!("Scrooby ledger JSON failed: {error}"))
        })?;
        let ordinal = required_usize(&value, "ordinal")?;
        let parent_ordinal = value
            .get("parent_ordinal")
            .and_then(Value::as_u64)
            .and_then(|value| usize::try_from(value).ok());
        let kind = required_string(&value, "kind")?.to_owned();
        let path = required_string(&value, "path")?.to_owned();
        rows.push(LedgerRow {
            ordinal,
            parent_ordinal,
            kind,
            path,
        });
    }
    Ok(rows)
}

fn validate_layout_children(components: &[Component]) -> PipelineOutcome<()> {
    let owners = components
        .iter()
        .filter(|component| {
            matches!(
                component.row.kind.as_str(),
                "scrooby_layer" | "scrooby_group" | "scrooby_multi_text"
            )
        })
        .map(|component| component.row.ordinal)
        .collect::<BTreeSet<_>>();
    let mut actual = BTreeMap::<usize, BTreeMap<&str, usize>>::new();
    for component in components {
        if !is_layout_child_kind(&component.row.kind) {
            continue;
        }
        let Some(parent) = component.row.parent_ordinal else {
            return Err(PipelineError::new(
                "Scrooby layout child has no owning parent",
            ));
        };
        if !owners.contains(&parent) {
            return Err(PipelineError::new(
                "Scrooby layout child has an unsupported owning parent",
            ));
        }
        let counts = actual.entry(parent).or_default();
        let count = counts.entry(component.row.kind.as_str()).or_default();
        *count = count.checked_add(1).ok_or_else(|| {
            PipelineError::new("Scrooby layout child count overflowed")
        })?;
    }
    for owner in components.iter().filter(|component| {
        owners.contains(&component.row.ordinal)
    }) {
        let children = owner
            .payload
            .get("children")
            .and_then(Value::as_array)
            .ok_or_else(|| {
                PipelineError::new(
                    "Scrooby layout owner has no child inventory",
                )
            })?;
        let mut expected = BTreeMap::<&str, usize>::new();
        for child in children {
            let id = required_string(child, "id_hex")?;
            let kind = layout_child_kind(&owner.row.kind, id)?;
            let count = expected.entry(kind).or_default();
            *count = count.checked_add(1).ok_or_else(|| {
                PipelineError::new("Scrooby declared child count overflowed")
            })?;
        }
        let observed = actual
            .get(&owner.row.ordinal)
            .cloned()
            .unwrap_or_default();
        if expected != observed {
            return Err(PipelineError::new(
                "Scrooby layout child inventory disagrees with ledger ancestry",
            ));
        }
    }
    Ok(())
}

fn is_layout_child_kind(kind: &str) -> bool {
    matches!(
        kind,
        "scrooby_group"
            | "scrooby_multi_sprite"
            | "scrooby_multi_text"
            | "scrooby_pure3d_object"
            | "scrooby_polygon"
            | "scrooby_string_text_bible"
            | "scrooby_string_hardcoded"
    )
}

fn layout_child_kind(owner: &str, id: &str) -> PipelineOutcome<&'static str> {
    let kind = match id {
        "0x00018004" => "scrooby_group",
        "0x00018006" => "scrooby_multi_sprite",
        "0x00018007" => "scrooby_multi_text",
        "0x00018008" => "scrooby_pure3d_object",
        "0x00018009" => "scrooby_polygon",
        "0x0001800b" => "scrooby_string_text_bible",
        "0x0001800c" => "scrooby_string_hardcoded",
        _ => {
            return Err(PipelineError::new(
                "Scrooby layout owner declares an unsupported child kind",
            ));
        },
    };
    let allowed = if owner == "scrooby_multi_text" {
        matches!(
            kind,
            "scrooby_string_text_bible" | "scrooby_string_hardcoded"
        )
    } else {
        !matches!(
            kind,
            "scrooby_string_text_bible" | "scrooby_string_hardcoded"
        )
    };
    if !allowed {
        return Err(PipelineError::new(
            "Scrooby child kind is invalid for its layout owner",
        ));
    }
    Ok(kind)
}

fn validate_screen_pages(components: &[Component]) -> PipelineOutcome<()> {
    let pages = identity_counts(components, "scrooby_page", "name")?;
    for component in components
        .iter()
        .filter(|component| component.row.kind == "scrooby_screen")
    {
        for page in required_string_array(&component.payload, "page_names")? {
            require_unique(&pages, trim_padding(page), "Scrooby page")?;
        }
    }
    Ok(())
}

fn validate_widget_resources(components: &[Component]) -> PipelineOutcome<()> {
    let images = identity_counts(
        components,
        "scrooby_image_resource",
        "name",
    )?;
    let styles = identity_counts(
        components,
        "scrooby_text_style_resource",
        "name",
    )?;
    let bibles = identity_counts(
        components,
        "scrooby_text_bible_resource",
        "name",
    )?;
    let pure = resource_identities(components, "scrooby_pure3d_resource")?;
    for component in components {
        match component.row.kind.as_str() {
            "scrooby_multi_sprite" => {
                for image in
                    required_string_array(&component.payload, "image_names")?
                {
                    require_unique(
                        &images,
                        trim_padding(image),
                        "Scrooby image resource",
                    )?;
                }
            },
            "scrooby_multi_text" => {
                let style = required_string(&component.payload, "text_style")?;
                require_unique(
                    &styles,
                    trim_padding(style),
                    "Scrooby text style",
                )?;
            },
            "scrooby_string_text_bible" => {
                let bible = required_string(&component.payload, "bible_name")?;
                require_unique(
                    &bibles,
                    trim_padding(bible),
                    "Scrooby text bible",
                )?;
            },
            "scrooby_pure3d_object" => {
                let name = trim_padding(required_string(
                    &component.payload,
                    "filename",
                )?);
                require_pure3d_resource(&pure, name)?;
            },
            _ => {},
        }
    }
    Ok(())
}

fn identity_counts(
    components: &[Component],
    kind: &str,
    field: &str,
) -> PipelineOutcome<BTreeMap<String, usize>> {
    let mut counts = BTreeMap::<String, usize>::new();
    for component in components.iter().filter(|item| item.row.kind == kind) {
        let identity = trim_padding(required_string(&component.payload, field)?)
            .to_owned();
        let count = counts.entry(identity).or_default();
        *count = count.checked_add(1).ok_or_else(|| {
            PipelineError::new("Scrooby resource identity count overflowed")
        })?;
    }
    Ok(counts)
}

fn resource_identities(
    components: &[Component],
    kind: &str,
) -> PipelineOutcome<Vec<(String, String)>> {
    components
        .iter()
        .filter(|component| component.row.kind == kind)
        .map(|component| {
            Ok((
                trim_padding(required_string(&component.payload, "name")?)
                    .to_owned(),
                trim_padding(required_string(
                    &component.payload,
                    "inventory_name",
                )?)
                .to_owned(),
            ))
        })
        .collect()
}

fn require_pure3d_resource(
    resources: &[(String, String)],
    reference: &str,
) -> PipelineOutcome<()> {
    let by_name = resources
        .iter()
        .filter(|(name, _inventory)| name == reference)
        .count();
    if by_name == 1 {
        return Ok(());
    }
    if by_name > 1 {
        return Err(PipelineError::new(
            "Scrooby Pure3D resource name is ambiguous",
        ));
    }
    let by_inventory = resources
        .iter()
        .filter(|(_name, inventory)| inventory == reference)
        .count();
    if by_inventory == 1 {
        Ok(())
    } else if by_inventory == 0 {
        Err(PipelineError::new("Scrooby Pure3D resource is missing"))
    } else {
        Err(PipelineError::new(
            "Scrooby Pure3D inventory identity is ambiguous",
        ))
    }
}

fn require_unique(
    identities: &BTreeMap<String, usize>,
    reference: &str,
    label: &str,
) -> PipelineOutcome<()> {
    match identities.get(reference).copied().unwrap_or_default() {
        1 => Ok(()),
        0 => Err(PipelineError::new(format!("{label} is missing"))),
        _ => Err(PipelineError::new(format!("{label} is ambiguous"))),
    }
}

fn required_string<'value>(
    value: &'value Value,
    field: &str,
) -> PipelineOutcome<&'value str> {
    value.get(field).and_then(Value::as_str).ok_or_else(|| {
        PipelineError::new(format!("Scrooby {field} is not a string"))
    })
}

fn required_usize(value: &Value, field: &str) -> PipelineOutcome<usize> {
    value
        .get(field)
        .and_then(Value::as_u64)
        .and_then(|value| usize::try_from(value).ok())
        .ok_or_else(|| {
            PipelineError::new(format!("Scrooby {field} is not an integer"))
        })
}

fn required_string_array<'value>(
    value: &'value Value,
    field: &str,
) -> PipelineOutcome<Vec<&'value str>> {
    value
        .get(field)
        .and_then(Value::as_array)
        .ok_or_else(|| {
            PipelineError::new(format!("Scrooby {field} is not an array"))
        })?
        .iter()
        .map(|entry| {
            entry.as_str().ok_or_else(|| {
                PipelineError::new(format!(
                    "Scrooby {field} contains a non-string"
                ))
            })
        })
        .collect()
}

fn trim_padding(mut value: &str) -> &str {
    while let Some(trimmed) = value.strip_suffix("\\x00") {
        value = trimmed;
    }
    value
}

fn io_error(action: &str, error: &std::io::Error) -> PipelineError {
    PipelineError::new(format!("{action} failed: {error}"))
}

#[cfg(test)]
// jig-ignore-next-line: canonical test module path is indivisible.
#[path = "../../../../../../tests/migration/pipeline/unit/adapter-outbound/local/ui_scrooby_project_tests.rs"]
mod tests;
