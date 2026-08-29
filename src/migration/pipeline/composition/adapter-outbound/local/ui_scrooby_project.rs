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
//   - Resolve exact package-local authored identities and source-backed
//     normalized resource-package paths across UI evidence.
// - Split-When:
//   - UI artifact publication or WidgetBlueprint construction gains a
//     lifecycle.
// - Merge-When:
//   - Another adapter owns the identical Scrooby semantic binding preflight.
// - Summary:
//   - Validate Scrooby references and resource-package backing before Unreal.
// - Description:
//   - Requires every observed screen, image, style, text-bible, and Pure3D
//     reference to resolve to exactly one normalized resource declaration.
//     Package-backed P3D resources must also expose the inventory evidence that
//     Scrooby expects to find after loading that package. Embedded text styles
//     honor the owner's current inventory before any resource-file fallback.
// - Usage:
//   - Called by prepare-unreal after canonical package-index intake.
// - Defaults:
//   - Missing, ambiguous, malformed, or unsafe authored references fail closed;
//     absent optional resource packages remain explicitly unresolved.
//

//! Normalized Scrooby project semantic preflight.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use schoenwald_filesystem::resolve_under;
use serde_json::{Value, json};

use crate::domain::{
    PackageRole, PhaseThreePackageIndex, PhaseThreePackageRow, PipelineError,
    PipelineOutcome, UnrealImportManifest,
};

const BINDING_CATALOG_SCHEMA: &str =
    "shar-schoenwald.scrooby-binding-catalog.v6";
const BINDING_CATALOG_FILE: &str = "catalog.jsonl";

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

#[derive(Clone, Debug, Eq, PartialEq)]
struct ScroobyPackageBinding {
    source_ordinal: usize,
    source_index: usize,
    target_ordinal: usize,
    relation: &'static str,
    match_basis: &'static str,
    target_source_unit_id: Option<String>,
    target_source_match_basis: Option<&'static str>,
    target_package_id: Option<String>,
    target_package_match_basis: Option<&'static str>,
    target_entity_ordinal: Option<usize>,
    target_entity_match_basis: Option<&'static str>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ScroobyImageResourceSource {
    ordinal: usize,
    filename: String,
    joined_sprite_ordinal: Option<usize>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ScroobyPackageBindings {
    package_id: String,
    bindings: Vec<ScroobyPackageBinding>,
    image_resources: Vec<ScroobyImageResourceSource>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct ScroobyUiPreflight {
    packages: Vec<ScroobyPackageBindings>,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(super) struct ScroobyJoinedSpriteTarget {
    pub(super) package_id: String,
    pub(super) sprite_ordinal: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct ScroobyPageResourceLifecycle {
    pub(super) package_id: String,
    pub(super) page_ordinal: usize,
    pub(super) source_index: usize,
    pub(super) resource_kind: &'static str,
    pub(super) target_ordinal: usize,
    pub(super) target_source_unit_id: Option<String>,
    pub(super) target_source_match_basis: Option<&'static str>,
    pub(super) target_package_id: Option<String>,
    pub(super) target_package_match_basis: Option<&'static str>,
    pub(super) target_entity_ordinal: Option<usize>,
    pub(super) target_entity_match_basis: Option<&'static str>,
}

impl ScroobyUiPreflight {
    pub(super) const fn package_count(&self) -> usize {
        self.packages.len()
    }

    pub(super) fn binding_count(&self) -> usize {
        self.packages
            .iter()
            .map(|package| package.bindings.len())
            .sum()
    }

    pub(super) fn direct_import_binding_count(&self) -> usize {
        self.packages
            .iter()
            .flat_map(|package| &package.bindings)
            .filter(|binding| binding.target_source_unit_id.is_some())
            .count()
    }

    pub(super) fn normalized_package_binding_count(&self) -> usize {
        self.packages
            .iter()
            .flat_map(|package| &package.bindings)
            .filter(|binding| binding.target_package_id.is_some())
            .count()
    }

    pub(super) fn normalized_entity_binding_count(&self) -> usize {
        self.packages
            .iter()
            .flat_map(|package| &package.bindings)
            .filter(|binding| binding.target_entity_ordinal.is_some())
            .count()
    }

    pub(super) fn joined_sprite_targets(
        &self,
    ) -> Vec<ScroobyJoinedSpriteTarget> {
        let mut targets = BTreeSet::new();
        for binding in self
            .packages
            .iter()
            .flat_map(|package| &package.bindings)
        {
            if binding.target_package_match_basis
                != Some("owner-joined-sprite-exact")
                || binding.target_entity_match_basis
                    != Some("full-filename-exact")
            {
                continue;
            }
            if let (Some(package_id), Some(sprite_ordinal)) = (
                binding.target_package_id.as_ref(),
                binding.target_entity_ordinal,
            ) {
                let _inserted = targets.insert(ScroobyJoinedSpriteTarget {
                    package_id: package_id.clone(),
                    sprite_ordinal,
                });
            }
        }
        targets.into_iter().collect()
    }

    pub(super) fn page_resource_lifecycle(
        &self,
    ) -> Vec<ScroobyPageResourceLifecycle> {
        let mut rows = Vec::new();
        for package in &self.packages {
            for binding in &package.bindings {
                let resource_kind = match binding.relation {
                    "page-image-resource" => "image",
                    "page-pure3d-resource" => "pure3d",
                    "page-text-style" => "text-style",
                    "page-text-bible" => "text-bible",
                    _ => continue,
                };
                rows.push(ScroobyPageResourceLifecycle {
                    package_id: package.package_id.clone(),
                    page_ordinal: binding.source_ordinal,
                    source_index: binding.source_index,
                    resource_kind,
                    target_ordinal: binding.target_ordinal,
                    target_source_unit_id: binding
                        .target_source_unit_id
                        .clone(),
                    target_source_match_basis: binding
                        .target_source_match_basis,
                    target_package_id: binding.target_package_id.clone(),
                    target_package_match_basis: binding
                        .target_package_match_basis,
                    target_entity_ordinal: binding.target_entity_ordinal,
                    target_entity_match_basis: binding
                        .target_entity_match_basis,
                });
            }
        }
        rows.sort_by(|left, right| {
            (
                left.package_id.as_str(),
                left.page_ordinal,
                left.source_index,
                left.target_ordinal,
                left.resource_kind,
            )
                .cmp(&(
                    right.package_id.as_str(),
                    right.page_ordinal,
                    right.source_index,
                    right.target_ordinal,
                    right.resource_kind,
                ))
        });
        rows
    }

    fn to_catalog_jsonl(&self) -> PipelineOutcome<String> {
        let mut packages = self.packages.iter().collect::<Vec<_>>();
        packages.sort_by(|left, right| left.package_id.cmp(&right.package_id));
        let capacity = self.binding_count().saturating_add(1);
        let mut rows = Vec::with_capacity(capacity);
        rows.push(json!({
            "schema": BINDING_CATALOG_SCHEMA,
            "record_type": "header",
            "status": "complete",
            "package_count": self.package_count(),
            "binding_count": self.binding_count(),
            "direct_import_binding_count": self.direct_import_binding_count(),
            "normalized_package_binding_count":
                self.normalized_package_binding_count(),
            "normalized_entity_binding_count":
                self.normalized_entity_binding_count(),
        }));
        for package in packages {
            let mut bindings = package.bindings.iter().collect::<Vec<_>>();
            bindings.sort_by(|left, right| {
                (
                    left.source_ordinal,
                    left.relation,
                    left.source_index,
                    left.target_ordinal,
                    left.match_basis,
                )
                    .cmp(&(
                        right.source_ordinal,
                        right.relation,
                        right.source_index,
                        right.target_ordinal,
                        right.match_basis,
                    ))
            });
            for binding in bindings {
                let mut row = json!({
                    "schema": BINDING_CATALOG_SCHEMA,
                    "record_type": "binding",
                    "package_id": package.package_id,
                    "source_ordinal": binding.source_ordinal,
                    "source_index": binding.source_index,
                    "target_ordinal": binding.target_ordinal,
                    "relation": binding.relation,
                    "match_basis": binding.match_basis,
                });
                match (
                    &binding.target_source_unit_id,
                    binding.target_source_match_basis,
                ) {
                    (Some(source_unit_id), Some(source_match_basis)) => {
                        let object = row.as_object_mut().ok_or_else(|| {
                            PipelineError::new(
                                "Scrooby binding row is not a JSON object",
                            )
                        })?;
                        let _previous = object.insert(
                            "target_source_unit_id".to_owned(),
                            json!(source_unit_id),
                        );
                        let _previous = object.insert(
                            "target_source_match_basis".to_owned(),
                            json!(source_match_basis),
                        );
                    },
                    (None, None) => {},
                    _ => {
                        return Err(PipelineError::new(
                            "Scrooby direct-import binding is incomplete",
                        ));
                    },
                }
                match (
                    &binding.target_package_id,
                    binding.target_package_match_basis,
                ) {
                    (Some(package_id), Some(package_match_basis)) => {
                        let object = row.as_object_mut().ok_or_else(|| {
                            PipelineError::new(
                                "Scrooby binding row is not a JSON object",
                            )
                        })?;
                        let _previous = object.insert(
                            "target_package_id".to_owned(),
                            json!(package_id),
                        );
                        let _previous = object.insert(
                            "target_package_match_basis".to_owned(),
                            json!(package_match_basis),
                        );
                    },
                    (None, None) => {},
                    _ => {
                        return Err(PipelineError::new(
                            "Scrooby normalized-package binding is incomplete",
                        ));
                    },
                }
                match (
                    binding.target_entity_ordinal,
                    binding.target_entity_match_basis,
                ) {
                    (Some(entity_ordinal), Some(entity_match_basis)) => {
                        let object = row.as_object_mut().ok_or_else(|| {
                            PipelineError::new(
                                "Scrooby binding row is not a JSON object",
                            )
                        })?;
                        let _previous = object.insert(
                            "target_entity_ordinal".to_owned(),
                            json!(entity_ordinal),
                        );
                        let _previous = object.insert(
                            "target_entity_match_basis".to_owned(),
                            json!(entity_match_basis),
                        );
                    },
                    (None, None) => {},
                    _ => {
                        return Err(PipelineError::new(
                            "Scrooby normalized-entity binding is incomplete",
                        ));
                    },
                }
                rows.push(row);
            }
        }
        let mut rendered = String::new();
        for row in rows {
            rendered.push_str(&serde_json::to_string(&row).map_err(|error| {
                PipelineError::new(format!(
                    "Scrooby binding catalog JSON failed: {error}"
                ))
            })?);
            rendered.push('\n');
        }
        Ok(rendered)
    }
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
) -> PipelineOutcome<ScroobyUiPreflight> {
    let mut packages = Vec::new();
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
        let (mut package_bindings, image_resources) =
            preflight_scrooby_package_evidence(&root)?;
        bind_scrooby_normalized_resource_packages(
            index,
            extracted_root,
            package,
            &root,
            &mut package_bindings,
        )?;
        bind_scrooby_joined_image_entities(
            &package.package_id,
            &image_resources,
            &mut package_bindings,
        )?;
        packages.push(ScroobyPackageBindings {
            package_id: package.package_id.clone(),
            bindings: package_bindings,
            image_resources,
        });
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
    }
    packages.sort_by(|left, right| left.package_id.cmp(&right.package_id));
    Ok(ScroobyUiPreflight { packages })
}

/// Bind package-local image resources to verified direct texture imports.
///
/// # Errors
///
/// Returns an error when one authored image filename matches more than one
/// direct texture import in the same semantic package.
pub(super) fn bind_scrooby_direct_import_sources(
    preflight: &mut ScroobyUiPreflight,
    manifest: &UnrealImportManifest,
) -> PipelineOutcome<usize> {
    let mut resolved_count = 0usize;
    for package in &mut preflight.packages {
        let candidates = manifest
            .sources
            .iter()
            .filter(|source| {
                source.package_id == package.package_id
                    && source.role == PackageRole::Texture
                    && source.evidence.file_extension == "png"
                    && source.direct_import.as_ref().is_some_and(|import| {
                        import.target_class == "Texture2D"
                            && import.importer == "texture-factory"
                            && import.import_profile == "shar-texture-v1"
                    })
            })
            .map(|source| (source.id.as_str(), source.evidence.path.as_str()))
            .collect::<Vec<_>>();
        let mut source_by_ordinal = BTreeMap::new();
        for resource in &package.image_resources {
            if resource.joined_sprite_ordinal.is_some() {
                continue;
            }
            let source = resolve_exact_image_source(
                &resource.filename,
                candidates.iter().copied(),
            )?;
            if let Some(source_unit_id) = source
                && source_by_ordinal
                    .insert(resource.ordinal, source_unit_id.to_owned())
                    .is_some()
            {
                return Err(PipelineError::new(
                    "Scrooby image resource ordinal is duplicated",
                ));
            }
        }
        for binding in &mut package.bindings {
            binding.target_source_unit_id = None;
            binding.target_source_match_basis = None;
            if !matches!(
                binding.relation,
                "page-image-resource" | "sprite-image-resource"
            ) {
                continue;
            }
            let Some(source_unit_id) =
                source_by_ordinal.get(&binding.target_ordinal)
            else {
                continue;
            };
            binding.target_source_unit_id = Some(source_unit_id.clone());
            binding.target_source_match_basis = Some("filename-basename-exact");
            resolved_count = resolved_count.saturating_add(1);
        }
    }
    Ok(resolved_count)
}

pub(super) fn publish_scrooby_binding_catalog(
    preflight: &ScroobyUiPreflight,
    output_root: &Path,
) -> PipelineOutcome<usize> {
    let rendered = preflight.to_catalog_jsonl()?;
    let name = output_root
        .file_name()
        .and_then(|value| value.to_str())
        .filter(|value| !value.is_empty())
        .ok_or_else(|| {
            PipelineError::new("Scrooby binding output has no portable name")
        })?;
    let parent = output_root.parent().unwrap_or_else(|| Path::new("."));
    fs::create_dir_all(parent)
        .map_err(|error| io_error("create Scrooby binding parent", &error))?;
    let staging = parent.join(format!(".{name}.complete-staging"));
    let backup = parent.join(format!(".{name}.complete-backup"));
    ensure_binding_path_absent(&staging, "Scrooby binding staging")?;
    ensure_binding_path_absent(&backup, "Scrooby binding backup")?;
    let catalog = output_root.join(BINDING_CATALOG_FILE);
    if let Ok(metadata) = fs::symlink_metadata(output_root) {
        if !metadata.is_dir() || metadata.file_type().is_symlink() {
            return Err(PipelineError::new(
                "Scrooby binding output is not a regular directory",
            ));
        }
        if fs::read_to_string(&catalog).ok().as_deref() == Some(&rendered) {
            return Ok(preflight.binding_count());
        }
    }
    fs::create_dir_all(&staging)
        .map_err(|error| io_error("create Scrooby binding staging", &error))?;
    let staged_catalog = staging.join(BINDING_CATALOG_FILE);
    if let Err(error) = fs::write(&staged_catalog, &rendered) {
        let _cleanup = fs::remove_dir_all(&staging);
        return Err(io_error("write Scrooby binding catalog", &error));
    }
    if fs::read_to_string(&staged_catalog)
        .map_err(|error| {
            io_error("read staged Scrooby binding catalog", &error)
        })?
        != rendered
    {
        let _cleanup = fs::remove_dir_all(&staging);
        return Err(PipelineError::new(
            "staged Scrooby binding catalog changed during read-back",
        ));
    }
    let had_output = match fs::symlink_metadata(output_root) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => false,
        Err(error) => {
            let _cleanup = fs::remove_dir_all(&staging);
            return Err(io_error("inspect Scrooby binding output", &error));
        },
        Ok(metadata) => {
            if !metadata.is_dir() || metadata.file_type().is_symlink() {
                let _cleanup = fs::remove_dir_all(&staging);
                return Err(PipelineError::new(
                    "Scrooby binding output is not a regular directory",
                ));
            }
            fs::rename(output_root, &backup).map_err(|error| {
                io_error("back up Scrooby binding output", &error)
            })?;
            true
        },
    };
    if let Err(error) = fs::rename(&staging, output_root) {
        let publish_error = io_error("publish Scrooby binding catalog", &error);
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
    let published = fs::read_to_string(output_root.join(BINDING_CATALOG_FILE))
        .map_err(|error| {
            io_error("read published Scrooby binding catalog", &error)
        })?;
    if published != rendered {
        rollback_binding_catalog(output_root, &backup, had_output)?;
        return Err(PipelineError::new(
            "published Scrooby binding catalog changed during read-back",
        ));
    }
    if had_output {
        fs::remove_dir_all(&backup)
            .map_err(|error| {
                io_error("remove Scrooby binding backup", &error)
            })?;
    }
    Ok(preflight.binding_count())
}

fn ensure_binding_path_absent(path: &Path, label: &str) -> PipelineOutcome<()> {
    match fs::symlink_metadata(path) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(io_error(
            "inspect Scrooby binding transaction",
            &error,
        )),
        Ok(_metadata) => Err(PipelineError::new(format!(
            "{label} already exists"
        ))),
    }
}

fn rollback_binding_catalog(
    output_root: &Path,
    backup: &Path,
    had_output: bool,
) -> PipelineOutcome<()> {
    fs::remove_dir_all(output_root)
        .map_err(|error| {
            io_error("remove invalid Scrooby binding output", &error)
        })?;
    if had_output {
        fs::rename(backup, output_root).map_err(|error| {
            io_error("restore previous Scrooby binding output", &error)
        })?;
    }
    Ok(())
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

#[cfg(test)]
fn preflight_scrooby_package(
    root: &Path,
) -> PipelineOutcome<Vec<ScroobyPackageBinding>> {
    preflight_scrooby_package_evidence(root)
        .map(|(bindings, _resources)| bindings)
}

fn preflight_scrooby_package_evidence(
    root: &Path,
) -> PipelineOutcome<(
    Vec<ScroobyPackageBinding>,
    Vec<ScroobyImageResourceSource>,
)> {
    let decoded = read_scrooby_components(root)?;
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
    validate_project_structure(&decoded)?;
    validate_layout_children(&decoded)?;
    validate_screen_pages(&decoded)?;
    validate_page_resources(&decoded)?;
    validate_widget_resources(&decoded)?;
    let bindings = collect_named_bindings(&decoded)?;
    let image_resources = collect_image_resource_sources(root, &decoded)?;
    Ok((bindings, image_resources))
}

fn read_scrooby_components(root: &Path) -> PipelineOutcome<Vec<Component>> {
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
    Ok(decoded)
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

fn validate_project_structure(components: &[Component]) -> PipelineOutcome<()> {
    let project = components
        .iter()
        .find(|component| component.row.kind == "scrooby_project")
        .ok_or_else(|| PipelineError::new("Scrooby project is missing"))?;
    if project.row.parent_ordinal != Some(0) {
        return Err(PipelineError::new(
            "Scrooby project is not rooted at the package boundary",
        ));
    }
    let children = project
        .payload
        .get("children")
        .and_then(Value::as_array)
        .ok_or_else(|| {
            PipelineError::new("Scrooby project has no child inventory")
        })?;
    let mut expected = BTreeMap::<&str, usize>::new();
    for child in children {
        let id = required_string(child, "id_hex")?;
        let kind = match id {
            "0x00018001" => "scrooby_screen",
            "0x00018002" => "scrooby_page",
            _ => {
                return Err(PipelineError::new(
                    "Scrooby project declares an unsupported child kind",
                ));
            },
        };
        let count = expected.entry(kind).or_default();
        *count = count.checked_add(1).ok_or_else(|| {
            PipelineError::new("Scrooby project child count overflowed")
        })?;
    }
    let mut observed = BTreeMap::<&str, usize>::new();
    for component in components.iter().filter(|component| {
        matches!(
            component.row.kind.as_str(),
            "scrooby_page" | "scrooby_screen"
        )
    }) {
        if component.row.parent_ordinal != Some(project.row.ordinal) {
            return Err(PipelineError::new(
                "Scrooby project child has incorrect ancestry",
            ));
        }
        let count = observed.entry(component.row.kind.as_str()).or_default();
        *count = count.checked_add(1).ok_or_else(|| {
            PipelineError::new(
                "Scrooby observed project child count overflowed",
            )
        })?;
    }
    if expected != observed {
        return Err(PipelineError::new(
            "Scrooby project child inventory disagrees with ledger ancestry",
        ));
    }
    validate_page_layers(components)
}

fn validate_page_layers(components: &[Component]) -> PipelineOutcome<()> {
    let pages = components
        .iter()
        .filter(|component| component.row.kind == "scrooby_page")
        .map(|component| component.row.ordinal)
        .collect::<BTreeSet<_>>();
    let mut actual = BTreeMap::<usize, usize>::new();
    for layer in components
        .iter()
        .filter(|component| component.row.kind == "scrooby_layer")
    {
        let Some(parent) = layer.row.parent_ordinal else {
            return Err(PipelineError::new(
                "Scrooby layer has no owning page",
            ));
        };
        if !pages.contains(&parent) {
            return Err(PipelineError::new(
                "Scrooby layer has an unsupported owning parent",
            ));
        }
        let count = actual.entry(parent).or_default();
        *count = count.checked_add(1).ok_or_else(|| {
            PipelineError::new("Scrooby layer count overflowed")
        })?;
    }
    for page in components
        .iter()
        .filter(|component| component.row.kind == "scrooby_page")
    {
        let children = page
            .payload
            .get("children")
            .and_then(Value::as_array)
            .ok_or_else(|| {
                PipelineError::new("Scrooby page has no child inventory")
            })?;
        let mut declared = 0usize;
        for child in children {
            match required_string(child, "id_hex")? {
                "0x00018003" => {
                    declared = declared.checked_add(1).ok_or_else(|| {
                        PipelineError::new(
                            "Scrooby page layer count overflowed",
                        )
                    })?;
                },
                "0x00018100" | "0x00018101" | "0x00018104"
                | "0x00018105" => {},
                _ => {
                    return Err(PipelineError::new(
                        "Scrooby page declares an unsupported child kind",
                    ));
                },
            }
        }
        let observed = actual
            .get(&page.row.ordinal)
            .copied()
            .unwrap_or_default();
        if declared != observed {
            return Err(PipelineError::new(
                "Scrooby page layer inventory disagrees with ledger ancestry",
            ));
        }
    }
    Ok(())
}

fn validate_page_resources(components: &[Component]) -> PipelineOutcome<()> {
    let pages = components
        .iter()
        .filter(|component| component.row.kind == "scrooby_page")
        .map(|component| component.row.ordinal)
        .collect::<BTreeSet<_>>();
    for resource in components.iter().filter(|component| {
        is_page_resource_kind(&component.row.kind)
    }) {
        let Some(parent) = resource.row.parent_ordinal else {
            return Err(PipelineError::new(
                "Scrooby resource has no owning page",
            ));
        };
        if !pages.contains(&parent) {
            return Err(PipelineError::new(
                "Scrooby resource has an unsupported owning parent",
            ));
        }
    }

    let images = identity_counts(
        components,
        "scrooby_image_resource",
        "name",
    )?;
    let pure = identity_counts(
        components,
        "scrooby_pure3d_resource",
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
    for page in components
        .iter()
        .filter(|component| component.row.kind == "scrooby_page")
    {
        let children = page
            .payload
            .get("children")
            .and_then(Value::as_array)
            .ok_or_else(|| {
                PipelineError::new("Scrooby page has no child inventory")
            })?;
        for child in children {
            let id = required_string(child, "id_hex")?;
            if id == "0x00018003" {
                continue;
            }
            let name = trim_padding(required_string(child, "name")?);
            if name.is_empty() {
                return Err(PipelineError::new(
                    "Scrooby page resource name is empty",
                ));
            }
            match id {
                "0x00018100" => require_unique(
                    &images,
                    name,
                    "Scrooby page image resource",
                )?,
                "0x00018101" => require_unique(
                    &pure,
                    name,
                    "Scrooby page Pure3D resource",
                )?,
                "0x00018104" => require_unique(
                    &styles,
                    name,
                    "Scrooby page text style",
                )?,
                "0x00018105" => require_unique(
                    &bibles,
                    name,
                    "Scrooby page text bible",
                )?,
                _ => {
                    return Err(PipelineError::new(
                        "Scrooby page declares an unsupported child kind",
                    ));
                },
            }
        }
    }
    Ok(())
}

fn is_page_resource_kind(kind: &str) -> bool {
    matches!(
        kind,
        "scrooby_image_resource"
            | "scrooby_pure3d_resource"
            | "scrooby_text_style_resource"
            | "scrooby_text_bible_resource"
    )
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
    let pure = pure3d_resource_bindings(components)?;
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
                let _binding = resolve_pure3d_resource(&pure, name)?;
            },
            _ => {},
        }
    }
    Ok(())
}

fn collect_named_bindings(
    components: &[Component],
) -> PipelineOutcome<Vec<ScroobyPackageBinding>> {
    let pages = identity_ordinals(components, "scrooby_page", "name")?;
    let images = identity_ordinals(
        components,
        "scrooby_image_resource",
        "name",
    )?;
    let styles = identity_ordinals(
        components,
        "scrooby_text_style_resource",
        "name",
    )?;
    let bibles = identity_ordinals(
        components,
        "scrooby_text_bible_resource",
        "name",
    )?;
    let pure_names = identity_ordinals(
        components,
        "scrooby_pure3d_resource",
        "name",
    )?;
    let pure = pure3d_resource_bindings(components)?;
    let mut bindings = Vec::new();
    for component in components {
        match component.row.kind.as_str() {
            "scrooby_screen" => {
                for (source_index, page) in required_string_array(
                    &component.payload,
                    "page_names",
                )?
                .into_iter()
                .enumerate()
                {
                    push_binding(
                        &mut bindings,
                        component.row.ordinal,
                        source_index,
                        resolve_unique_ordinal(
                            &pages,
                            trim_padding(page),
                            "Scrooby page",
                        )?,
                        "screen-page",
                        "name",
                    );
                }
            },
            "scrooby_page" => {
                let children = component
                    .payload
                    .get("children")
                    .and_then(Value::as_array)
                    .ok_or_else(|| {
                        PipelineError::new(
                            "Scrooby page has no child inventory",
                        )
                    })?;
                for (source_index, child) in children.iter().enumerate() {
                    let id = required_string(child, "id_hex")?;
                    if id == "0x00018003" {
                        continue;
                    }
                    let name = trim_padding(required_string(child, "name")?);
                    let (targets, label, relation) = match id {
                        "0x00018100" => (
                            &images,
                            "Scrooby page image resource",
                            "page-image-resource",
                        ),
                        "0x00018104" => (
                            &styles,
                            "Scrooby page text style",
                            "page-text-style",
                        ),
                        "0x00018105" => (
                            &bibles,
                            "Scrooby page text bible",
                            "page-text-bible",
                        ),
                        "0x00018101" => {
                            push_binding(
                                &mut bindings,
                                component.row.ordinal,
                                source_index,
                                resolve_unique_ordinal(
                                    &pure_names,
                                    name,
                                    "Scrooby page Pure3D resource",
                                )?,
                                "page-pure3d-resource",
                                "name",
                            );
                            continue;
                        },
                        _ => {
                            return Err(PipelineError::new(
                                concat!(
                                    "Scrooby page declares an unsupported ",
                                    "child kind",
                                ),
                            ));
                        },
                    };
                    push_binding(
                        &mut bindings,
                        component.row.ordinal,
                        source_index,
                        resolve_unique_ordinal(targets, name, label)?,
                        relation,
                        "name",
                    );
                }
            },
            "scrooby_multi_sprite" => {
                for (source_index, image) in required_string_array(
                    &component.payload,
                    "image_names",
                )?
                .into_iter()
                .enumerate()
                {
                    push_binding(
                        &mut bindings,
                        component.row.ordinal,
                        source_index,
                        resolve_unique_ordinal(
                            &images,
                            trim_padding(image),
                            "Scrooby image resource",
                        )?,
                        "sprite-image-resource",
                        "name",
                    );
                }
            },
            "scrooby_multi_text" => {
                let style = trim_padding(required_string(
                    &component.payload,
                    "text_style",
                )?);
                push_binding(
                    &mut bindings,
                    component.row.ordinal,
                    0,
                    resolve_unique_ordinal(
                        &styles,
                        style,
                        "Scrooby text style",
                    )?,
                    "text-style-resource",
                    "name",
                );
            },
            "scrooby_string_text_bible" => {
                let bible = trim_padding(required_string(
                    &component.payload,
                    "bible_name",
                )?);
                push_binding(
                    &mut bindings,
                    component.row.ordinal,
                    0,
                    resolve_unique_ordinal(
                        &bibles,
                        bible,
                        "Scrooby text bible",
                    )?,
                    "string-text-bible",
                    "name",
                );
            },
            "scrooby_pure3d_object" => {
                let name = trim_padding(required_string(
                    &component.payload,
                    "filename",
                )?);
                let (target, basis) = resolve_pure3d_resource(&pure, name)?;
                push_binding(
                    &mut bindings,
                    component.row.ordinal,
                    0,
                    target,
                    "pure3d-object-resource",
                    basis,
                );
            },
            _ => {},
        }
    }
    Ok(bindings)
}

fn push_binding(
    bindings: &mut Vec<ScroobyPackageBinding>,
    source_ordinal: usize,
    source_index: usize,
    target_ordinal: usize,
    relation: &'static str,
    match_basis: &'static str,
) {
    bindings.push(ScroobyPackageBinding {
        source_ordinal,
        source_index,
        target_ordinal,
        relation,
        match_basis,
        target_source_unit_id: None,
        target_source_match_basis: None,
        target_package_id: None,
        target_package_match_basis: None,
        target_entity_ordinal: None,
        target_entity_match_basis: None,
    });
}

fn bind_scrooby_normalized_resource_packages(
    index: &PhaseThreePackageIndex,
    extracted_root: &Path,
    owner_package: &PhaseThreePackageRow,
    root: &Path,
    bindings: &mut [ScroobyPackageBinding],
) -> PipelineOutcome<()> {
    let decoded = read_scrooby_components(root)?;
    let project = decoded
        .iter()
        .find(|component| component.row.kind == "scrooby_project")
        .ok_or_else(|| PipelineError::new("Scrooby project is missing"))?;
    let resource_path = required_string(&project.payload, "resource_path")?;
    let mut package_by_ordinal =
        BTreeMap::<usize, (String, &'static str)>::new();
    // Image P3D resources use a distinct FullFilename inventory contract.
    // Keep them unresolved until that identity is modeled explicitly.
    for resource in decoded.iter().filter(|component| {
        matches!(
            component.row.kind.as_str(),
            "scrooby_pure3d_resource"
                | "scrooby_text_style_resource"
                | "scrooby_text_bible_resource"
        )
    }) {
        if let Some(target) = resolve_owner_text_style_inventory(
            extracted_root,
            owner_package,
            resource,
        )? {
            if package_by_ordinal
                .insert(
                    resource.row.ordinal,
                    (target.to_owned(), "owner-inventory-name-exact"),
                )
                .is_some()
            {
                return Err(PipelineError::new(
                    "Scrooby resource package ordinal is duplicated",
                ));
            }
            continue;
        }
        let filename = required_string(&resource.payload, "filename")?;
        let Some(target_root) = scrooby_resource_package_root(
            &owner_package.package_root,
            resource_path,
            filename,
        )? else {
            continue;
        };
        let matches = index
            .packages()
            .iter()
            .filter(|package| {
                package.package_root.eq_ignore_ascii_case(&target_root)
            })
            .collect::<Vec<_>>();
        let target = match matches.as_slice() {
            [] => continue,
            [target] => *target,
            _ => {
                return Err(PipelineError::new(
                    "Scrooby resource package path is ambiguous",
                ));
            },
        };
        validate_scrooby_resource_target_inventory(
            extracted_root,
            resource,
            target,
        )?;
        if package_by_ordinal
            .insert(
                resource.row.ordinal,
                (
                    target.package_id.clone(),
                    "project-resource-path-exact",
                ),
            )
            .is_some()
        {
            return Err(PipelineError::new(
                "Scrooby resource package ordinal is duplicated",
            ));
        }
    }
    for binding in bindings {
        if let Some((package_id, match_basis)) =
            package_by_ordinal.get(&binding.target_ordinal)
        {
            binding.target_package_id = Some(package_id.clone());
            binding.target_package_match_basis = Some(*match_basis);
        }
    }
    Ok(())
}

fn validate_scrooby_resource_target_inventory(
    extracted_root: &Path,
    resource: &Component,
    target: &PhaseThreePackageRow,
) -> PipelineOutcome<()> {
    if target.category() != "ui-resources" {
        return Err(PipelineError::new(
            "Scrooby resource package has an unsupported category",
        ));
    }
    let inventory_name = trim_padding(required_string(
        &resource.payload,
        "inventory_name",
    )?);
    if inventory_name.is_empty() {
        return Err(PipelineError::new(
            "Scrooby resource inventory name is empty",
        ));
    }
    let matching_kinds = scrooby_package_inventory_matching_kinds(
        extracted_root,
        target,
        inventory_name,
    )?;
    validate_scrooby_resource_inventory_kinds(
        &resource.row.kind,
        &matching_kinds,
    )
}

fn resolve_owner_text_style_inventory<'package>(
    extracted_root: &Path,
    owner_package: &'package PhaseThreePackageRow,
    resource: &Component,
) -> PipelineOutcome<Option<&'package str>> {
    if resource.row.kind != "scrooby_text_style_resource" {
        return Ok(None);
    }
    let inventory_name = trim_padding(required_string(
        &resource.payload,
        "inventory_name",
    )?);
    if inventory_name.is_empty() {
        return Ok(None);
    }
    let matches = scrooby_package_inventory_matching_kinds(
        extracted_root,
        owner_package,
        inventory_name,
    )?;
    if matches == ["texture_font"] {
        Ok(Some(owner_package.package_id.as_str()))
    } else {
        Ok(None)
    }
}

fn scrooby_package_inventory_matching_kinds<'package>(
    extracted_root: &Path,
    package: &'package PhaseThreePackageRow,
    inventory_name: &str,
) -> PipelineOutcome<Vec<&'package str>> {
    let target_root =
        resolve_normalized_package_root(extracted_root, &package.package_root)?;
    let package_prefix = format!("{}/", package.package_root);
    let mut matching_kinds = Vec::new();
    for member in package.members().iter().filter(|member| {
        member.source_chunk_kind != "none"
            && Path::new(&member.path)
                .extension()
                .is_some_and(|extension| {
                    extension.eq_ignore_ascii_case("json")
                })
    }) {
        let relative = member.path.strip_prefix(&package_prefix).ok_or_else(|| {
            PipelineError::new(
                "Scrooby target member is outside its normalized package",
            )
        })?;
        let path = resolve_under(&target_root, Path::new(relative)).map_err(
            |_error| {
                PipelineError::new(
                    "Scrooby target member escapes its normalized package",
                )
            },
        )?;
        let bytes = fs::read(&path).map_err(|error| {
            io_error("read Scrooby target inventory member", &error)
        })?;
        let payload = serde_json::from_slice::<Value>(&bytes).map_err(|error| {
            PipelineError::new(format!(
                "Scrooby target inventory JSON failed: {error}"
            ))
        })?;
        if payload
            .get("name")
            .and_then(Value::as_str)
            .map(trim_padding)
            == Some(inventory_name)
        {
            matching_kinds.push(member.source_chunk_kind.as_str());
        }
    }
    Ok(matching_kinds)
}

fn validate_scrooby_resource_inventory_kinds(
    resource_kind: &str,
    matching_kinds: &[&str],
) -> PipelineOutcome<()> {
    match resource_kind {
        "scrooby_pure3d_resource" if !matching_kinds.is_empty() => Ok(()),
        "scrooby_pure3d_resource" => Err(PipelineError::new(
            "Scrooby Pure3D resource target lacks its inventory entity",
        )),
        "scrooby_text_style_resource"
            if matching_kinds == ["texture_font"] => Ok(()),
        "scrooby_text_style_resource" => Err(PipelineError::new(
            "Scrooby text-style target lacks one exact texture font",
        )),
        "scrooby_text_bible_resource"
            if matching_kinds == ["text_bible"] => Ok(()),
        "scrooby_text_bible_resource" => Err(PipelineError::new(
            "Scrooby text-bible target lacks one exact text bible",
        )),
        _ => Err(PipelineError::new(
            "Scrooby resource target inventory kind is unsupported",
        )),
    }
}

fn scrooby_resource_package_root(
    owner_package_root: &str,
    resource_path: &str,
    filename: &str,
) -> PipelineOutcome<Option<String>> {
    let filename = trim_padding(filename);
    let Some(extension_offset) = filename.len().checked_sub(4) else {
        return Ok(None);
    };
    let Some(extension) = filename.get(extension_offset..) else {
        return Ok(None);
    };
    if !extension.eq_ignore_ascii_case(".p3d") {
        return Ok(None);
    }
    let without_extension = filename.get(..extension_offset).ok_or_else(|| {
        PipelineError::new("Scrooby resource filename cannot strip extension")
    })?;
    let parent = owner_package_root
        .rsplit_once('/')
        .map(|(parent, _name)| parent)
        .ok_or_else(|| {
            PipelineError::new("Scrooby package root has no parent")
        })?;
    let resource_segments = scrooby_relative_segments(resource_path)?;
    let filename_segments = scrooby_relative_segments(without_extension)?;
    let mut target = String::from(parent);
    for segment in resource_segments.into_iter().chain(filename_segments) {
        target.push('/');
        target.push_str(segment);
    }
    Ok(Some(target))
}

fn scrooby_relative_segments(value: &str) -> PipelineOutcome<Vec<&str>> {
    let value = trim_padding(value).trim_end_matches(['/', '\\']);
    if value.is_empty() || value.starts_with(['/', '\\']) {
        return Err(PipelineError::new(
            "Scrooby resource path is not a portable relative path",
        ));
    }
    let mut segments = Vec::new();
    for segment in value.split(['/', '\\']) {
        if segment.is_empty()
            || matches!(segment, "." | "..")
            || segment.contains(':')
        {
            return Err(PipelineError::new(
                "Scrooby resource path contains an unsafe segment",
            ));
        }
        segments.push(segment);
    }
    Ok(segments)
}

fn collect_image_resource_sources(
    root: &Path,
    components: &[Component],
) -> PipelineOutcome<Vec<ScroobyImageResourceSource>> {
    let components_root = root.join("components");
    let mut sprite_ordinals = BTreeMap::<String, Vec<usize>>::new();
    for row in read_ledger(root)?
        .into_iter()
        .filter(|row| row.kind == "sprite")
    {
        let path = resolve_under(&components_root, Path::new(&row.path))
            .map_err(|_error| {
                PipelineError::new("Scrooby joined sprite path escapes package")
            })?;
        let bytes = fs::read(&path)
            .map_err(|error| io_error("read Scrooby joined sprite", &error))?;
        let payload = serde_json::from_slice::<Value>(&bytes).map_err(|error| {
            PipelineError::new(format!(
                "Scrooby joined sprite JSON failed: {error}"
            ))
        })?;
        let name = trim_padding(required_string(&payload, "name")?).to_owned();
        sprite_ordinals.entry(name).or_default().push(row.ordinal);
    }
    components
        .iter()
        .filter(|component| component.row.kind == "scrooby_image_resource")
        .map(|component| {
            let filename = trim_padding(required_string(
                &component.payload,
                "filename",
            )?);
            let basename = exact_basename(filename).ok_or_else(|| {
                PipelineError::new(
                    "Scrooby image resource filename has no basename",
                )
            })?;
            let joined_sprite_ordinal = resolve_exact_joined_sprite_ordinal(
                basename,
                sprite_ordinals.get(basename).map(Vec::as_slice),
            )?;
            Ok(ScroobyImageResourceSource {
                ordinal: component.row.ordinal,
                filename: filename.to_owned(),
                joined_sprite_ordinal,
            })
        })
        .collect()
}

fn resolve_exact_joined_sprite_ordinal(
    basename: &str,
    candidates: Option<&[usize]>,
) -> PipelineOutcome<Option<usize>> {
    if basename.is_empty() {
        return Err(PipelineError::new(
            "Scrooby joined sprite basename is empty",
        ));
    }
    match candidates.unwrap_or_default() {
        [] => Ok(None),
        [ordinal] => Ok(Some(*ordinal)),
        _ => Err(PipelineError::new(
            "Scrooby joined sprite inventory identity is ambiguous",
        )),
    }
}

fn bind_scrooby_joined_image_entities(
    owner_package_id: &str,
    image_resources: &[ScroobyImageResourceSource],
    bindings: &mut [ScroobyPackageBinding],
) -> PipelineOutcome<()> {
    let joined = image_resources
        .iter()
        .filter_map(|resource| {
            resource
                .joined_sprite_ordinal
                .map(|ordinal| (resource.ordinal, ordinal))
        })
        .collect::<BTreeMap<_, _>>();
    for binding in bindings.iter_mut().filter(|binding| {
        matches!(
            binding.relation,
            "page-image-resource" | "sprite-image-resource"
        )
    }) {
        let Some(entity_ordinal) = joined.get(&binding.target_ordinal) else {
            continue;
        };
        if binding.target_package_id.is_some()
            || binding.target_package_match_basis.is_some()
            || binding.target_entity_ordinal.is_some()
            || binding.target_entity_match_basis.is_some()
        {
            return Err(PipelineError::new(
                "Scrooby joined image binding already has backing identity",
            ));
        }
        binding.target_package_id = Some(owner_package_id.to_owned());
        binding.target_package_match_basis =
            Some("owner-joined-sprite-exact");
        binding.target_entity_ordinal = Some(*entity_ordinal);
        binding.target_entity_match_basis = Some("full-filename-exact");
    }
    Ok(())
}

fn exact_basename(value: &str) -> Option<&str> {
    value
        .rsplit(['/', '\\'])
        .next()
        .filter(|name| !name.is_empty())
}

fn resolve_exact_image_source<'source>(
    filename: &str,
    candidates: impl Iterator<Item = (&'source str, &'source str)>,
) -> PipelineOutcome<Option<&'source str>> {
    let basename = exact_basename(filename).ok_or_else(|| {
        PipelineError::new("Scrooby image resource filename has no basename")
    })?;
    let mut matched = None;
    for (source_unit_id, source_path) in candidates {
        if exact_basename(source_path) != Some(basename) {
            continue;
        }
        if matched.replace(source_unit_id).is_some() {
            return Err(PipelineError::new(
                "Scrooby image resource direct import is ambiguous",
            ));
        }
    }
    Ok(matched)
}

fn identity_ordinals(
    components: &[Component],
    kind: &str,
    field: &str,
) -> PipelineOutcome<BTreeMap<String, Vec<usize>>> {
    let mut ordinals = BTreeMap::<String, Vec<usize>>::new();
    for component in components.iter().filter(|item| item.row.kind == kind) {
        let identity = trim_padding(required_string(&component.payload, field)?)
            .to_owned();
        ordinals.entry(identity).or_default().push(component.row.ordinal);
    }
    Ok(ordinals)
}

fn resolve_unique_ordinal(
    identities: &BTreeMap<String, Vec<usize>>,
    reference: &str,
    label: &str,
) -> PipelineOutcome<usize> {
    let Some(matches) = identities.get(reference) else {
        return Err(PipelineError::new(format!("{label} is missing")));
    };
    match matches.as_slice() {
        [ordinal] => Ok(*ordinal),
        _ => Err(PipelineError::new(format!("{label} is ambiguous"))),
    }
}

fn pure3d_resource_bindings(
    components: &[Component],
) -> PipelineOutcome<Vec<(usize, String, String)>> {
    components
        .iter()
        .filter(|component| component.row.kind == "scrooby_pure3d_resource")
        .map(|component| {
            Ok((
                component.row.ordinal,
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

fn resolve_pure3d_resource(
    resources: &[(usize, String, String)],
    reference: &str,
) -> PipelineOutcome<(usize, &'static str)> {
    let by_name = resources
        .iter()
        .filter(|(_ordinal, name, _inventory)| name == reference)
        .collect::<Vec<_>>();
    match by_name.as_slice() {
        [(ordinal, _name, _inventory)] => return Ok((*ordinal, "name")),
        [] => {},
        _ => {
            return Err(PipelineError::new(
                "Scrooby Pure3D resource name is ambiguous",
            ));
        },
    }
    let by_inventory = resources
        .iter()
        .filter(|(_ordinal, _name, inventory)| inventory == reference)
        .collect::<Vec<_>>();
    match by_inventory.as_slice() {
        [(ordinal, _name, _inventory)] => Ok((*ordinal, "inventory_name")),
        [] => Err(PipelineError::new("Scrooby Pure3D resource is missing")),
        _ => Err(PipelineError::new(
            "Scrooby Pure3D inventory identity is ambiguous",
        )),
    }
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
    value = value.trim_end_matches(char::from(0));
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
