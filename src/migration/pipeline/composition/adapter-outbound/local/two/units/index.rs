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
//   - Index outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Index outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for pipeline.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Index outbound adapter.

use std::collections::{BTreeMap, BTreeSet};
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use schoenwald_filesystem::adapters::driving::local::{
    read_utf8 as local_read_utf8, write_text as local_write_text,
};

use super::super::text::{TextPackageDraft, derive_text_packages};
use super::index_render::render_index_jsonl;
use super::metadata_fill::read_string_field;
use super::{
    audio_video, cars, cinematics, taxonomy, ui_images, ui_resources,
    ui_screens, ui_vehicle_previews,
};
use crate::domain::{PipelineError, StageReport};

mod classification;

pub(super) use classification::{PackageCategory, category_from_root};
use classification::{
    is_tutorial_mission_asset, speaker_name, subcategory_from_root,
};

/// Result.
type PipelineOutcome<T> = Result<T, PipelineError>;

/// Package index file name.
pub(super) const INDEX_FILE_NAME: &str = "index.jsonl";

/// Opaque minor-unit id from the manifest.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(super) struct MinorUnitId(String);

impl MinorUnitId {
    /// Create a minor-unit id when the manifest provided a non-empty value.
    #[must_use]
    pub(super) fn new(value: String) -> Option<Self> {
        if value.is_empty() {
            None
        } else {
            Some(Self(value))
        }
    }

    /// Return the manifest id text.
    #[must_use]
    pub(super) fn as_str(&self) -> &str {
        &self.0
    }
}

/// Deterministic package id derived from the exact package root.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(super) struct PackageId(String);

impl PackageId {
    /// Create a package id from a manifest package root.
    #[must_use]
    pub(super) fn from_root(root: &str) -> Self {
        let mut package_id = String::with_capacity(root.len());
        let mut separator_pending = false;
        for character in root.chars() {
            if character.is_ascii_alphanumeric() {
                if separator_pending && !package_id.is_empty() {
                    package_id.push('-');
                }
                package_id.push(character.to_ascii_lowercase());
                separator_pending = false;
            } else if !package_id.is_empty() {
                separator_pending = true;
            }
        }
        Self(package_id)
    }

    /// Return the package id text.
    #[must_use]
    pub(super) fn as_str(&self) -> &str {
        &self.0
    }
}

/// Typed id bucket used by package consumers.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(super) enum MinorUnitRole {
    /// World geometry, roads, fences, paths, or DSG records.
    World,
    /// Texture image payload.
    Texture,
    /// Material or shader payload.
    Material,
    /// Mesh/model payload.
    Model,
    /// Collision or physics payload.
    Physics,
    /// Animation or skeleton payload.
    Animation,
    /// Scene graph payload.
    Scene,
    /// Locator payload.
    Locator,
    /// Camera payload.
    Camera,
    /// Light payload.
    Light,
    /// Particle payload.
    Particle,
    /// Controller payload.
    Controller,
    /// Audio payload.
    Audio,
    /// Movie payload.
    Movie,
    /// Script payload.
    Script,
    /// Text, table, or localization payload.
    Text,
    /// UI layout, font, or Scrooby project payload.
    Ui,
    /// Metadata that belongs to the package but is not imported directly.
    Metadata,
    /// Unmapped package member that must be fixed before export.
    Error,
}

impl MinorUnitRole {
    /// Stable role label used in index output.
    #[must_use]
    pub(super) const fn as_str(self) -> &'static str {
        match self {
            Self::World => "world",
            Self::Texture => "texture",
            Self::Material => "material",
            Self::Model => "model",
            Self::Physics => "physics",
            Self::Animation => "animation",
            Self::Scene => "scene",
            Self::Locator => "locator",
            Self::Camera => "camera",
            Self::Light => "light",
            Self::Particle => "particle",
            Self::Controller => "controller",
            Self::Audio => "audio",
            Self::Movie => "movie",
            Self::Script => "script",
            Self::Text => "text",
            Self::Ui => "ui",
            Self::Metadata => "metadata",
            Self::Error => "error",
        }
    }
}

/// One derived text key exposed as an invocable package member.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct TextKeyMember {
    /// Deterministic key id.
    pub id: String,
    /// Stable localization key.
    pub key: String,
    /// Source minor-unit id that owns the physical text file.
    pub source_unit_id: String,
    /// Stable text package subcategory.
    pub subcategory: String,
}

/// One package member from the manifest.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct PackageMember {
    /// Manifest id.
    pub id: MinorUnitId,
    /// Typed downstream role.
    pub role: MinorUnitRole,
    /// Manifest path used for local file resolution.
    pub path: String,
    /// Manifest type column.
    pub type_: String,
    /// Manifest kind column.
    pub kind: String,
    /// Manifest source chunk kind.
    pub source_chunk_kind: String,
    /// Manifest source chunk ordinal, or `none` for loose sources.
    pub source_chunk_ordinal: String,
}

/// One exact package made of typed minor-unit ids.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct MinorUnitPackage {
    /// Deterministic package id.
    pub package_id: PackageId,
    /// Exact manifest root shared by the package members.
    pub package_root: String,
    /// High-level category used to browse and route packages.
    pub category: PackageCategory,
    /// Hierarchical package subcategory used for exporter lookup.
    pub subcategory: String,
    /// Members that together form the package.
    pub members: Vec<PackageMember>,
    /// Source units needed by derived packages without duplicating coverage.
    pub source_unit_ids: Vec<String>,
    /// Derived text keys exposed for importable language packages.
    pub text_keys: Vec<TextKeyMember>,
}

impl MinorUnitPackage {
    /// Create an empty package for one root.
    fn new(package_root: String) -> Self {
        Self {
            package_id: PackageId::from_root(&package_root),
            category: PackageCategory::Error,
            subcategory: "error/unclassified".to_owned(),
            package_root,
            members: Vec::new(),
            source_unit_ids: Vec::new(),
            text_keys: Vec::new(),
        }
    }

    /// Create a derived text package without claiming manifest coverage.
    fn from_text_package(draft: TextPackageDraft) -> Self {
        let mut package = Self {
            package_id: PackageId::from_root(&draft.package_root),
            package_root: draft.package_root,
            category: PackageCategory::Language,
            subcategory: draft.subcategory,
            members: Vec::new(),
            source_unit_ids: draft.source_unit_ids,
            text_keys: draft
                .keys
                .into_iter()
                .map(|key| TextKeyMember {
                    id: key.id,
                    key: key.key,
                    source_unit_id: key.source_unit_id,
                    subcategory: key.subcategory,
                })
                .collect(),
        };
        package.fail_closed_on_semantic_debt();
        package
    }

    /// Add a typed member id to the package.
    fn push(&mut self, member: PackageMember) {
        self.members.push(member);
    }

    /// Recompute routing after all manifest members are available so public
    /// categories can use typed row evidence instead of being locked to the
    /// grouping key that only exists for local file resolution.
    fn refresh_classification_from_members(&mut self) {
        let (category, subcategory) =
            classification_from_manifest_evidence(self);
        self.category = category;
        self.subcategory = subcategory;
        self.fail_closed_on_semantic_debt();
    }

    /// Convert soft taxonomy placeholders into explicit errors because an
    /// importable package must name a concrete invocation scope. The package
    /// still keeps its member ids, allowing a later derived package to
    /// reference the same source records from every concrete consumer
    /// instead of hiding them behind a successful catch-all bucket.
    fn fail_closed_on_semantic_debt(&mut self) {
        if self.category == PackageCategory::Error {
            return;
        }
        if let Some(error) = semantic_debt_error_subcategory(&self.subcategory)
        {
            self.category = PackageCategory::Error;
            self.subcategory = error.to_owned();
        }
    }
}

/// Write the package index beside the manifest.
///
/// # Errors
///
/// Returns an error when the manifest is missing, malformed, contains a row
pub(in crate::adapters::driven::local) fn write_minor_unit_index(
    extracted_root: &Path,
) -> PipelineOutcome<StageReport> {
    let packages = read_minor_unit_packages(extracted_root)?;
    let rendered = render_index_jsonl(&packages);
    let path = index_path(extracted_root);
    local_write_text(&path, &rendered, true).map_err(io_error(&path))?;
    Ok(StageReport {
        name: "minor-unit-index",
        files: packages.len(),
        bytes: u64::try_from(rendered.len()).unwrap_or(u64::MAX),
        note: format!(
            "indexed {} exact minor-unit packages from the manifest ledger",
            packages.len()
        ),
    })
}

/// Read package index from the manifest.
///
/// # Errors
///
/// Returns an error when any required manifest field is missing or when any
/// manifest id is duplicated, missing from the package index, or assigned more
/// than once.
pub(super) fn read_minor_unit_packages(
    extracted_root: &Path,
) -> PipelineOutcome<Vec<MinorUnitPackage>> {
    let manifest_path = taxonomy::manifest_path(extracted_root);
    let input =
        local_read_utf8(&manifest_path).map_err(io_error(&manifest_path))?;
    let mut packages = BTreeMap::<String, MinorUnitPackage>::new();
    let mut manifest_ids = BTreeSet::<MinorUnitId>::new();
    let manifest_path_text = manifest_path.display().to_string();
    for (line_index, line) in input.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let row = MinorUnitRow::from_line(
            line,
            line_index.saturating_add(1),
            &manifest_path_text,
        )?;
        if !manifest_ids.insert(row.id.clone()) {
            return Err(PipelineError::new(format!(
                "{}:{} duplicates minor-unit id {}",
                manifest_path_text,
                line_index.saturating_add(1),
                row.id.as_str()
            )));
        }
        let root = package_root(&row.path);
        packages
            .entry(root.clone())
            .or_insert_with(|| MinorUnitPackage::new(root))
            .push(row.into_member(extracted_root)?);
    }
    let mut output = packages.into_values().collect::<Vec<_>>();
    for package in &mut output {
        package.members.sort_by(|left, right| {
            left.role
                .cmp(&right.role)
                .then_with(|| left.path.cmp(&right.path))
                .then_with(|| left.id.cmp(&right.id))
        });
        package.refresh_classification_from_members();
    }
    let mut derived = derived_text_packages(extracted_root, &output)?;
    output.append(&mut derived);
    output.sort_by(|left, right| left.package_id.cmp(&right.package_id));
    validate_package_coverage(&manifest_ids, &output)?;
    Ok(output)
}

/// Returns the canonical package-index output path.
#[must_use]
pub(super) fn index_path(extracted_root: &Path) -> PathBuf {
    taxonomy::output_dir(extracted_root).join(INDEX_FILE_NAME)
}

/// Groups `MinorUnitRow` evidence for deterministic package classification.
struct MinorUnitRow {
    /// Stores `id` evidence required by this deterministic record.
    id: MinorUnitId,
    /// Stores `path` evidence required by this deterministic record.
    path: String,
    /// Stores `type_` evidence required by this deterministic record.
    type_: String,
    /// Stores `kind` evidence required by this deterministic record.
    kind: String,
    /// Stores `source_chunk_kind` evidence required by this deterministic
    /// record.
    source_chunk_kind: String,
    /// Stores exact source chunk ordinal provenance, or `none` for loose files.
    source_chunk_ordinal: String,
}

impl MinorUnitRow {
    /// Supports the `from_line` operation within this deterministic
    /// classification boundary.
    fn from_line(
        line: &str,
        line_number: usize,
        manifest_path: &str,
    ) -> PipelineOutcome<Self> {
        let id_text = required_field(line, "id", line_number, manifest_path)?;
        if id_text == taxonomy::UNKNOWN {
            return Err(PipelineError::new(format!(
                "{manifest_path}:{line_number} cannot be indexed \
                         before metadata fill assigns a stable id"
            )));
        }
        let id = MinorUnitId::new(id_text).ok_or_else(|| {
            PipelineError::new(format!(
                "{manifest_path}:{line_number} has an empty id field"
            ))
        })?;
        let recovery_status = required_field(
            line,
            "recovery_status",
            line_number,
            manifest_path,
        )?;
        if recovery_status != "fully-decoded" {
            return Err(PipelineError::new(format!(
                "{manifest_path}:{line_number} cannot be indexed \
                         because recovery_status is {recovery_status}"
            )));
        }
        Ok(Self {
            id,
            path: required_field(line, "path", line_number, manifest_path)?,
            type_: required_field(line, "type", line_number, manifest_path)?,
            kind: required_field(line, "kind", line_number, manifest_path)?,
            source_chunk_kind: required_field(
                line,
                "source_chunk_kind",
                line_number,
                manifest_path,
            )?,
            source_chunk_ordinal: required_field(
                line,
                "source_chunk_ordinal",
                line_number,
                manifest_path,
            )?,
        })
    }

    /// Supports the `into_member` operation within this deterministic
    /// classification boundary.
    // jig-ignore-next-line: long identifier
    fn into_member(self, extracted_root: &Path) -> PipelineOutcome<PackageMember> {
        let mut role =
            role_from_fields(&self.type_, &self.kind, &self.source_chunk_kind);
        if role == MinorUnitRole::Model
            && self.kind == "p3d-mesh"
            && self.source_chunk_kind == "mesh"
            && mesh_has_no_primitive_groups(extracted_root, &self.path)?
        {
            role = MinorUnitRole::Metadata;
        }
        Ok(PackageMember {
            id: self.id,
            role,
            path: self.path,
            type_: self.type_,
            kind: self.kind,
            source_chunk_kind: self.source_chunk_kind,
            source_chunk_ordinal: self.source_chunk_ordinal,
        })
    }
}

/// Maximum decoded mesh prefix needed to classify physical geometry.
const MESH_GEOMETRY_PREFIX_BYTES: usize = 512;

/// Return whether one decoded P3D mesh carries no physical primitive groups.
fn mesh_has_no_primitive_groups(
    extracted_root: &Path,
    member_path: &str,
) -> PipelineOutcome<bool> {
    let relative = Path::new(member_path);
    let path = if relative.is_absolute() {
        relative.to_path_buf()
    } else {
        extracted_root
            .parent()
            .unwrap_or_else(|| Path::new("."))
            .join(relative)
    };
    let mut file = File::open(&path).map_err(io_error(&path))?;
    let mut prefix = [0_u8; MESH_GEOMETRY_PREFIX_BYTES];
    let length = file.read(&mut prefix).map_err(io_error(&path))?;
    let bounded = prefix.get(..length).ok_or_else(|| {
        PipelineError::new("decoded mesh header read exceeded its buffer")
    })?;
    decoded_mesh_has_no_primitive_groups(bounded)
}

/// Inspect canonical decoded mesh header evidence without reading full
/// geometry.
// jig-ignore-next-line: long identifier
fn decoded_mesh_has_no_primitive_groups(prefix: &[u8]) -> PipelineOutcome<bool> {
    let schema = json_field_value(prefix, b"schema").ok_or_else(|| {
        PipelineError::new("decoded P3D mesh omitted geometry schema")
    })?;
    if !schema.starts_with(b"\"mesh\"") {
        return Err(PipelineError::new(
            "decoded P3D mesh has an unexpected geometry schema",
        ));
    }
    let groups = json_field_value(prefix, b"prim_groups").ok_or_else(|| {
        PipelineError::new(
            "decoded P3D mesh primitive groups exceed bounded header evidence",
        )
    })?;
    let Some(groups) = groups.strip_prefix(b"[") else {
        return Err(PipelineError::new(
            "decoded P3D mesh primitive groups are not an array",
        ));
    };
    let first = groups
        .iter()
        .copied()
        .find(|byte| !byte.is_ascii_whitespace())
        // jig-ignore-next-line: literal
        .ok_or_else(|| PipelineError::new("decoded P3D mesh array is truncated"))?;
    match first {
        b']' => Ok(true),
        b'{' => Ok(false),
        _ => Err(PipelineError::new(
            "decoded P3D mesh primitive-group array is malformed",
        )),
    }
}

fn json_field_value<'prefix>(
    prefix: &'prefix [u8],
    field: &[u8],
) -> Option<&'prefix [u8]> {
    let field_width = field.len().checked_add(2)?;
    let field_start = prefix.windows(field_width).position(|window| {
        window.first() == Some(&b'"')
            && window.last() == Some(&b'"')
            && window.get(1..field_width.saturating_sub(1)) == Some(field)
    })?;
    let mut cursor = field_start.checked_add(field_width)?;
    while prefix.get(cursor).is_some_and(u8::is_ascii_whitespace) {
        cursor = cursor.checked_add(1)?;
    }
    if prefix.get(cursor) != Some(&b':') {
        return None;
    }
    cursor = cursor.checked_add(1)?;
    while prefix.get(cursor).is_some_and(u8::is_ascii_whitespace) {
        cursor = cursor.checked_add(1)?;
    }
    prefix.get(cursor..)
}

/// Supports the `required_field` operation within this deterministic
/// classification boundary.
fn required_field(
    line: &str,
    field: &str,
    line_number: usize,
    manifest_path: &str,
) -> PipelineOutcome<String> {
    read_string_field(line, field).ok_or_else(|| {
        PipelineError::new(format!(
            "{manifest_path}:{line_number} missing {field} field"
        ))
    })
}

/// Supports the `validate_package_coverage` operation within this deterministic
/// classification boundary.
fn validate_package_coverage(
    manifest_ids: &BTreeSet<MinorUnitId>,
    packages: &[MinorUnitPackage],
) -> PipelineOutcome<()> {
    let mut indexed_ids = BTreeSet::<MinorUnitId>::new();
    for package in packages {
        for member in &package.members {
            if !indexed_ids.insert(member.id.clone()) {
                return Err(PipelineError::new(format!(
                    "minor-unit id {} is assigned to more than one \
                             package",
                    member.id.as_str()
                )));
            }
        }
    }
    if indexed_ids.len() != manifest_ids.len() {
        return Err(PipelineError::new(format!(
            "minor-unit index coverage mismatch: manifest has {} ids \
                     but index has {} ids",
            manifest_ids.len(),
            indexed_ids.len()
        )));
    }
    for id in manifest_ids {
        if !indexed_ids.contains(id) {
            return Err(PipelineError::new(format!(
                "minor-unit id {} is not cataloged in any package",
                id.as_str()
            )));
        }
    }
    for id in &indexed_ids {
        if !manifest_ids.contains(id) {
            return Err(PipelineError::new(format!(
                "minor-unit id {} is indexed but missing from the \
                         manifest",
                id.as_str()
            )));
        }
    }
    Ok(())
}

/// Supports the `derived_text_packages` operation within this deterministic
/// classification boundary.
fn derived_text_packages(
    extracted_root: &Path,
    packages: &[MinorUnitPackage],
) -> PipelineOutcome<Vec<MinorUnitPackage>> {
    let mut output = Vec::new();
    for package in packages {
        for member in &package.members {
            if member.role != MinorUnitRole::Text {
                continue;
            }
            for draft in derive_text_packages(
                extracted_root,
                member.id.as_str(),
                &member.path,
                &member.kind,
            )? {
                output.push(MinorUnitPackage::from_text_package(draft));
            }
        }
    }
    Ok(output)
}

/// Supports the `classification_from_manifest_evidence` operation within this
/// deterministic classification boundary.
fn classification_from_manifest_evidence(
    package: &MinorUnitPackage,
) -> (PackageCategory, String) {
    if let Some(subcategory) = mission_script_subcategory_from_evidence(package)
    {
        return (PackageCategory::MissionScripts, subcategory);
    }
    if let Some(subcategory) = vehicle_tuning_subcategory_from_evidence(package)
    {
        return (PackageCategory::VehicleTuning, subcategory);
    }
    if let Some(classification) = car_classification_from_evidence(package) {
        return classification;
    }
    if let Some(subcategory) = mission_art_subcategory_from_evidence(package) {
        return (PackageCategory::Missions, subcategory);
    }
    if let Some(subcategory) = dialog_subcategory_from_evidence(package) {
        return (PackageCategory::Dialog, subcategory);
    }
    if let Some(classification) =
        audio_video::classification_from_package(package)
    {
        return classification;
    }
    if let Some(classification) =
        ui_screens::classification_from_package(package)
    {
        return classification;
    }
    if let Some(classification) =
        cinematic_classification_from_evidence(package)
    {
        return classification;
    }
    if let Some(classification) = ui_image_classification_from_evidence(package)
    {
        return classification;
    }
    if let Some(classification) =
        ui_vehicle_previews::classification_from_package(package)
    {
        return classification;
    }
    if let Some(classification) =
        ui_resource_classification_from_evidence(package)
    {
        return classification;
    }
    if let Some(subcategory) = terrain_world_subcategory_from_evidence(package)
    {
        return (PackageCategory::TerrainWorld, subcategory);
    }
    (
        category_from_root(&package.package_root),
        subcategory_from_root(&package.package_root),
    )
}

/// Supports the `car_classification_from_evidence` operation within this
/// deterministic classification boundary.
fn car_classification_from_evidence(
    package: &MinorUnitPackage,
) -> Option<(PackageCategory, String)> {
    cars::classification_from_package(package)
}

/// Supports the `cinematic_classification_from_evidence` operation within this
/// deterministic classification boundary.
fn cinematic_classification_from_evidence(
    package: &MinorUnitPackage,
) -> Option<(PackageCategory, String)> {
    cinematics::classification_from_package(package)
}

/// Supports the `ui_image_classification_from_evidence` operation within this
/// deterministic classification boundary.
fn ui_image_classification_from_evidence(
    package: &MinorUnitPackage,
) -> Option<(PackageCategory, String)> {
    ui_images::classification_from_package(package)
}

/// Supports the `ui_resource_classification_from_evidence` operation within
/// this deterministic classification boundary.
fn ui_resource_classification_from_evidence(
    package: &MinorUnitPackage,
) -> Option<(PackageCategory, String)> {
    ui_resources::classification_from_package(package)
}

/// Supports the `terrain_world_subcategory_from_evidence` operation within this
/// deterministic classification boundary.
fn terrain_world_subcategory_from_evidence(
    package: &MinorUnitPackage,
) -> Option<String> {
    let tokens = package_id_tokens(package);
    if category_from_root(&package.package_root)
        != PackageCategory::TerrainWorld
        && !terrain_world_tokens_identify_package(&tokens)
    {
        return None;
    }
    let role = terrain_world_role_from_tokens(&tokens)?;
    let detail = terrain_world_detail_from_tokens(&tokens, role);
    if let Some(level) = terrain_world_level_from_tokens(&tokens) {
        return Some(format!("terrain-world/{level}/{role}{detail}"));
    }
    terrain_world_bonus_scope_from_tokens(&tokens)
        .map(|scope| format!("terrain-world/{scope}/{role}{detail}"))
}

/// Supports the `terrain_world_detail_from_tokens` operation within this
/// deterministic classification boundary.
fn terrain_world_detail_from_tokens(tokens: &[&str], role: &str) -> String {
    if !terrain_world_role_allows_detail(role) {
        return String::new();
    }
    terrain_world_detail_token(tokens, role)
        .map_or_else(String::new, |token| format!("/{token}"))
}

/// Supports the `terrain_world_role_allows_detail` operation within this
/// deterministic classification boundary.
fn terrain_world_role_allows_detail(role: &str) -> bool {
    matches!(
        role,
        "data-records"
            | "interiors"
            | "race-props"
            | "regions"
            | "segments"
            | "zones"
    )
}

/// Supports the `terrain_world_detail_token` operation within this
/// deterministic classification boundary.
fn terrain_world_detail_token(tokens: &[&str], role: &str) -> Option<String> {
    let index = tokens.iter().position(|token| match role {
        "data-records" => token.ends_with("data"),
        "interiors" => level_prefixed_role(token, 'i'),
        "race-props" => token.starts_with("sr") && token.ends_with('p'),
        "regions" => level_prefixed_role(token, 'r'),
        "segments" => terrain_bonus_token(token).is_some(),
        "zones" => level_prefixed_role(token, 'z'),
        _ => false,
    })?;
    let token = tokens.get(index).copied()?;
    if role == "regions"
        && let Some(suffix) = tokens.get(index.saturating_add(1))
        && matches!(*suffix, "dam")
    {
        return Some(format!("{token}-{suffix}"));
    }
    Some(token.to_owned())
}

/// Supports the `terrain_world_tokens_identify_package` operation within this
/// deterministic classification boundary.
fn terrain_world_tokens_identify_package(tokens: &[&str]) -> bool {
    tokens.iter().any(|token| {
        mission_level_from_token(token).is_some()
            || terrain_bonus_token(token).is_some()
    })
}

/// Supports the `terrain_world_level_from_tokens` operation within this
/// deterministic classification boundary.
fn terrain_world_level_from_tokens(tokens: &[&str]) -> Option<String> {
    tokens
        .iter()
        .find_map(|token| mission_level_from_token(token))
}

/// Supports the `terrain_world_bonus_scope_from_tokens` operation within this
/// deterministic classification boundary.
fn terrain_world_bonus_scope_from_tokens(
    tokens: &[&str],
) -> Option<&'static str> {
    tokens
        .iter()
        .any(|token| terrain_bonus_token(token).is_some())
        .then_some("bonus-area")
}

/// Supports the `terrain_bonus_token` operation within this deterministic
/// classification boundary.
fn terrain_bonus_token(token: &str) -> Option<&str> {
    let lower = token.to_ascii_lowercase();
    (lower.len() >= 3
        && lower.as_bytes().first().copied() == Some(b'b')
        && lower
            .as_bytes()
            .get(1..3)
            .is_some_and(|digits| digits.iter().all(u8::is_ascii_digit)))
    .then_some(token)
}

/// Supports the `terrain_world_role_from_tokens` operation within this
/// deterministic classification boundary.
fn terrain_world_role_from_tokens(tokens: &[&str]) -> Option<&'static str> {
    let joined = tokens.join("-");
    if tokens.contains(&"terra") {
        Some("terrain-mesh")
    } else if tokens.contains(&"fx") {
        Some("effects")
    } else if tokens
        .iter()
        .any(|token| token.starts_with("sr") && token.ends_with('p'))
    {
        Some("race-props")
    } else if tokens.iter().any(|token| level_prefixed_role(token, 'i')) {
        Some("interiors")
    } else if tokens.iter().any(|token| level_prefixed_role(token, 'r')) {
        Some("regions")
    } else if tokens.iter().any(|token| level_prefixed_role(token, 'z')) {
        Some("zones")
    } else if joined.contains("door") {
        Some("mission-doors")
    } else if tokens.iter().any(|token| token.ends_with("data")) {
        Some("data-records")
    } else if tokens
        .iter()
        .any(|token| terrain_bonus_token(token).is_some())
    {
        Some("segments")
    } else {
        None
    }
}

/// Supports the `level_prefixed_role` operation within this deterministic
/// classification boundary.
fn level_prefixed_role(token: &str, marker: char) -> bool {
    let mut chars = token.chars();
    chars.next() == Some('l')
        && chars.next().is_some_and(|value| value.is_ascii_digit())
        && chars.next() == Some(marker)
}

/// Supports the `semantic_debt_error_subcategory` operation within this
/// deterministic classification boundary.
fn semantic_debt_error_subcategory(subcategory: &str) -> Option<&'static str> {
    subcategory.split('/').find_map(semantic_debt_segment_error)
}

/// Supports the `semantic_debt_segment_error` operation within this
/// deterministic classification boundary.
fn semantic_debt_segment_error(segment: &str) -> Option<&'static str> {
    if segment.contains("unknown") {
        Some("error/unresolved-identity")
    } else if segment.contains("generic") || segment.contains("misc") {
        Some("error/vague-classification")
    } else if segment.contains("context") {
        Some("error/incomplete-context")
    } else if segment.contains("shared") {
        Some("error/duplicated-membership-required")
    } else if segment.contains("global") {
        Some("error/missing-invocation-scope")
    } else {
        None
    }
}

/// Supports the `mission_script_subcategory_from_evidence` operation within
/// this deterministic classification boundary.
fn mission_script_subcategory_from_evidence(
    package: &MinorUnitPackage,
) -> Option<String> {
    if !has_member_kind(package, "mission-script") {
        return None;
    }
    let scope = mission_scope_from_package_evidence(package)
        .unwrap_or_else(|| "missions/bootstrap".to_owned());
    let detail =
        mission_script_detail_from_tokens(&package_id_tokens(package), &scope);
    Some(format!("{scope}/scripts{detail}"))
}

/// Supports the `mission_script_detail_from_tokens` operation within this
/// deterministic classification boundary.
fn mission_script_detail_from_tokens(tokens: &[&str], scope: &str) -> String {
    if scope != "missions/bootstrap" {
        return String::new();
    }
    let Some(index) = tokens.iter().position(|token| *token == "scripts")
    else {
        return String::new();
    };
    let detail_tokens = tokens.get(index.saturating_add(1)..).unwrap_or(&[]);
    if detail_tokens.is_empty() {
        "/root".to_owned()
    } else {
        format!("/{}", detail_tokens.join("-"))
    }
}

/// Supports the `vehicle_tuning_subcategory_from_evidence` operation within
/// this deterministic classification boundary.
fn vehicle_tuning_subcategory_from_evidence(
    package: &MinorUnitPackage,
) -> Option<String> {
    if !has_member_kind(package, "vehicle-tuning") {
        return None;
    }
    let scope = mission_scope_from_package_evidence(package)
        .unwrap_or_else(|| "vehicle-tuning/free-roam".to_owned());
    let detail = vehicle_tuning_detail_from_tokens(&package_id_tokens(package));
    if scope.starts_with("missions/") {
        Some(format!("{scope}/vehicle-tuning{detail}"))
    } else {
        Some(format!("{scope}{detail}"))
    }
}

/// Supports the `vehicle_tuning_detail_from_tokens` operation within this
/// deterministic classification boundary.
fn vehicle_tuning_detail_from_tokens(tokens: &[&str]) -> String {
    let Some(index) = tokens.iter().position(|token| *token == "cars") else {
        return String::new();
    };
    let detail_tokens = tokens.get(index.saturating_add(1)..).unwrap_or(&[]);
    if detail_tokens.is_empty() {
        "/root".to_owned()
    } else {
        format!("/{}", detail_tokens.join("-"))
    }
}

/// Supports the `mission_art_subcategory_from_evidence` operation within this
/// deterministic classification boundary.
fn mission_art_subcategory_from_evidence(
    package: &MinorUnitPackage,
) -> Option<String> {
    if !package
        .package_id
        .as_str()
        .starts_with("extracted-art-missions-")
    {
        return None;
    }
    let scope = mission_scope_from_package_evidence(package)
        .unwrap_or_else(|| "missions/bootstrap".to_owned());
    let role = dominant_package_role(package, "assets");
    let detail = mission_art_detail_from_tokens(&package_id_tokens(package));
    Some(format!("{scope}/{role}{detail}"))
}

/// Supports the `mission_art_detail_from_tokens` operation within this
/// deterministic classification boundary.
fn mission_art_detail_from_tokens(tokens: &[&str]) -> String {
    let Some(index) = tokens.iter().position(|token| *token == "missions")
    else {
        return String::new();
    };
    let raw_tail = tokens.get(index.saturating_add(1)..).unwrap_or(&[]);
    let detail_tokens = mission_detail_tokens(raw_tail);
    if detail_tokens.is_empty() {
        String::new()
    } else {
        format!("/{}", detail_tokens.join("-"))
    }
}

/// Supports the `mission_detail_tokens` operation within this deterministic
/// classification boundary.
fn mission_detail_tokens<'a>(tokens: &'a [&str]) -> &'a [&'a str] {
    if let Some(first) = tokens.first()
        && (*first == "generic"
            || *first == "h2h"
            || *first == "level08"
            || is_tutorial_mission_asset(first)
            || mission_level_from_token(first).is_some())
    {
        return tokens.get(1..).unwrap_or(&[]);
    }
    tokens
}

/// Supports the `dialog_subcategory_from_evidence` operation within this
/// deterministic classification boundary.
fn dialog_subcategory_from_evidence(
    package: &MinorUnitPackage,
) -> Option<String> {
    if !package
        .members
        .iter()
        .any(|member| member.role == MinorUnitRole::Audio)
    {
        return None;
    }
    let package_id = package.package_id.as_str();
    if !package_id.starts_with("extracted-dialog") {
        return None;
    }
    let tokens = package_id_tokens(package);
    let speaker = dialog_speaker_from_tokens(&tokens).unwrap_or("unknown");
    let kind = dialog_kind_from_tokens(&tokens);
    let context = dialog_context_from_tokens(&tokens);
    let detail = dialog_detail_from_tokens(&tokens);
    let archive = dialog_archive_detail_from_tokens(&tokens);
    Some(format!(
        "dialog/{speaker}/{kind}/{context}{detail}{archive}"
    ))
}

/// Supports the `dialog_archive_detail_from_tokens` operation within this
/// deterministic classification boundary.
fn dialog_archive_detail_from_tokens(tokens: &[&str]) -> String {
    let Some(archive) = dialog_archive_from_tokens(tokens) else {
        return String::new();
    };
    format!("/{archive}")
}

/// Supports the `dialog_archive_from_tokens` operation within this
/// deterministic classification boundary.
fn dialog_archive_from_tokens(tokens: &[&str]) -> Option<&'static str> {
    if tokens.contains(&"dialogf") {
        Some("french")
    } else if tokens.contains(&"dialogg") {
        Some("german")
    } else if tokens.contains(&"dialogs") {
        Some("spanish")
    } else if tokens.contains(&"dialog") {
        Some("default")
    } else {
        None
    }
}

/// Supports the `dialog_detail_from_tokens` operation within this deterministic
/// classification boundary.
fn dialog_detail_from_tokens(tokens: &[&str]) -> String {
    dialog_detail_after_marker(tokens, "tutorial")
        .or_else(|| dialog_detail_after_marker(tokens, "convinit"))
        .or_else(|| dialog_detail_after_marker(tokens, "noboxconv"))
        .map_or_else(String::new, |detail| format!("/{detail}"))
}

/// Supports the `dialog_detail_after_marker` operation within this
/// deterministic classification boundary.
fn dialog_detail_after_marker(tokens: &[&str], marker: &str) -> Option<String> {
    let marker_index = tokens.iter().position(|token| *token == marker)?;
    let raw_detail_tokens = tokens.get(marker_index.saturating_add(1)..)?;
    let detail_tokens = raw_detail_tokens
        .strip_prefix(&["global"])
        .unwrap_or(raw_detail_tokens);
    (!detail_tokens.is_empty())
        .then(|| format!("{marker}/{}", detail_tokens.join("-")))
}

/// Supports the `has_member_kind` operation within this deterministic
/// classification boundary.
fn has_member_kind(package: &MinorUnitPackage, kind: &str) -> bool {
    package.members.iter().any(|member| member.kind == kind)
}

/// Supports the `mission_scope_from_package_evidence` operation within this
/// deterministic classification boundary.
fn mission_scope_from_package_evidence(
    package: &MinorUnitPackage,
) -> Option<String> {
    let tokens = package_id_tokens(package);
    mission_scope_from_tokens(&tokens).or_else(|| {
        package.members.iter().find_map(|member| {
            member
                .id
                .as_str()
                .split('-')
                .find_map(mission_level_from_token)
                .map(|level| format!("missions/{level}"))
        })
    })
}

/// Supports the `mission_scope_from_tokens` operation within this deterministic
/// classification boundary.
fn mission_scope_from_tokens(tokens: &[&str]) -> Option<String> {
    for (index, token) in tokens.iter().enumerate() {
        match *token {
            "generic" => return Some("missions/runtime".to_owned()),
            "h2h" | "level08" | "l8" => {
                return Some("missions/head-to-head".to_owned());
            },
            value if is_tutorial_mission_asset(value) => {
                return Some("missions/tutorial".to_owned());
            },
            value => {
                if let Some(level) = mission_level_from_token(value) {
                    let next = tokens.get(index.saturating_add(1)).copied();
                    if level == "level-01"
                        && next.is_some_and(is_tutorial_mission_asset)
                    {
                        return Some("missions/tutorial".to_owned());
                    }
                    return Some(format!("missions/{level}"));
                }
            },
        }
    }
    None
}

/// Supports the `mission_level_from_token` operation within this deterministic
/// classification boundary.
fn mission_level_from_token(token: &str) -> Option<String> {
    let lower = token.to_ascii_lowercase();
    if let Some(number) = lower.strip_prefix("level")
        && let Ok(parsed) = number.parse::<u8>()
        && (1..=7).contains(&parsed)
    {
        return Some(format!("level-{parsed:02}"));
    }
    let bytes = lower.as_bytes();
    if bytes.len() >= 3
        && bytes.first().copied() == Some(b'l')
        && bytes.get(1).copied() == Some(b'0')
        && let Some(raw_digit) = bytes.get(2).copied()
    {
        let digit = char::from(raw_digit);
        if matches!(digit, '1'..='7') {
            return Some(format!("level-0{digit}"));
        }
    }
    if bytes.len() >= 2
        && bytes.first().copied() == Some(b'l')
        && let Some(raw_digit) = bytes.get(1).copied()
    {
        let digit = char::from(raw_digit);
        if matches!(digit, '1'..='7') {
            return Some(format!("level-0{digit}"));
        }
    }
    None
}

/// Supports the `package_id_tokens` operation within this deterministic
/// classification boundary.
pub(super) fn package_id_tokens(package: &MinorUnitPackage) -> Vec<&str> {
    package
        .package_id
        .as_str()
        .split('-')
        .filter(|token| !token.is_empty())
        .collect()
}

/// Supports the `dominant_package_role` operation within this deterministic
/// classification boundary.
fn dominant_package_role(
    package: &MinorUnitPackage,
    fallback: &'static str,
) -> &'static str {
    if package
        .members
        .iter()
        .any(|member| member.role == MinorUnitRole::Script)
    {
        "scripts"
    } else if package
        .members
        .iter()
        .any(|member| member.role == MinorUnitRole::Camera)
    {
        "cameras"
    } else if package
        .members
        .iter()
        .any(|member| member.role == MinorUnitRole::Animation)
    {
        "animations"
    } else if package
        .members
        .iter()
        .any(|member| member.role == MinorUnitRole::Model)
    {
        "models"
    } else if package
        .members
        .iter()
        .any(|member| member.role == MinorUnitRole::Texture)
    {
        "textures"
    } else if package
        .members
        .iter()
        .any(|member| member.role == MinorUnitRole::World)
    {
        "world"
    } else {
        fallback
    }
}

/// Supports the `dialog_speaker_from_tokens` operation within this
/// deterministic classification boundary.
fn dialog_speaker_from_tokens(tokens: &[&str]) -> Option<&'static str> {
    if tokens
        .get(1)
        .is_some_and(|token| token.starts_with("dialog") && *token != "dialog")
        && let Some(speaker_tokens) = tokens.get(2..)
        && !speaker_tokens.is_empty()
    {
        let speaker = speaker_tokens.join("-");
        return Some(speaker_name(&speaker));
    }
    if let Some(index) = tokens.iter().position(|token| *token == "customfiles")
    {
        if tokens
            .get(index.saturating_add(1))
            .is_some_and(|token| *token == "conversations")
        {
            return Some("mod-conversations");
        }
        return tokens
            .get(index.saturating_add(1)..)
            .filter(|speaker_tokens| !speaker_tokens.is_empty())
            .map(|speaker_tokens| speaker_name(&speaker_tokens.join("-")));
    }
    if let Some(index) =
        tokens.iter().position(|token| *token == "conversations")
    {
        return tokens
            .get(index.saturating_add(1))
            .map(|speaker| speaker_name(speaker));
    }
    if let Some(index) = tokens.iter().position(|token| *token == "dialog") {
        return tokens
            .get(index.saturating_add(1)..)
            .filter(|speaker_tokens| {
                !speaker_tokens.is_empty()
                    && speaker_tokens.first() != Some(&"conversations")
            })
            .map(|speaker_tokens| speaker_name(&speaker_tokens.join("-")));
    }
    None
}

/// Supports the `dialog_kind_from_tokens` operation within this deterministic
/// classification boundary.
fn dialog_kind_from_tokens(tokens: &[&str]) -> &'static str {
    if tokens.contains(&"conversations") {
        "conversation"
    } else {
        "ad-lib"
    }
}

/// Supports the `dialog_context_from_tokens` operation within this
/// deterministic classification boundary.
fn dialog_context_from_tokens(tokens: &[&str]) -> String {
    if tokens.contains(&"tutorial") {
        return "tutorial".to_owned();
    }
    if let Some(level) = tokens
        .iter()
        .find_map(|token| mission_level_from_token(token))
    {
        return format!("mission/{level}");
    }
    if let Some(topic) = conversation_topic_from_tokens(tokens) {
        return format!("conversation-topic/{topic}");
    }
    "free-roam".to_owned()
}

/// Supports the `conversation_topic_from_tokens` operation within this
/// deterministic classification boundary.
fn conversation_topic_from_tokens<'a>(tokens: &[&'a str]) -> Option<&'a str> {
    tokens
        .iter()
        .position(|token| matches!(*token, "convinit" | "noboxconv"))
        .and_then(|index| tokens.get(index.saturating_add(2)).copied())
        .filter(|topic| *topic != "free-roam")
}

/// Supports the `role_from_fields` operation within this deterministic
/// classification boundary.
fn role_from_fields(
    type_: &str,
    kind: &str,
    source_chunk_kind: &str,
) -> MinorUnitRole {
    match type_ {
        "world" => MinorUnitRole::World,
        "image" => MinorUnitRole::Texture,
        "material" => MinorUnitRole::Material,
        "model" => MinorUnitRole::Model,
        "physics" => MinorUnitRole::Physics,
        "animation" => MinorUnitRole::Animation,
        "scene" => MinorUnitRole::Scene,
        "locator" => MinorUnitRole::Locator,
        "camera" => MinorUnitRole::Camera,
        "light" => MinorUnitRole::Light,
        "particle" => MinorUnitRole::Particle,
        "controller" => MinorUnitRole::Controller,
        "audio" => MinorUnitRole::Audio,
        "movie-video" | "movie-audio" => MinorUnitRole::Movie,
        "script" => MinorUnitRole::Script,
        "text" | "table" | "localization" => MinorUnitRole::Text,
        "ui" => MinorUnitRole::Ui,
        "metadata" | "config" => MinorUnitRole::Metadata,
        _ if source_chunk_kind == "texture" || kind == "p3d-texture" => {
            MinorUnitRole::Texture
        },
        _ if kind == "p3d-shader" => MinorUnitRole::Material,
        _ if kind == "p3d-mesh" => MinorUnitRole::Model,
        _ => MinorUnitRole::Error,
    }
}

/// Supports the `package_root` operation within this deterministic
/// classification boundary.
fn package_root(path: &str) -> String {
    if let Some(conversation) = conversation_package_root(path) {
        return conversation;
    }
    if let Some((head, _tail)) = path.split_once("/components/") {
        return head.to_owned();
    }
    if let Some(rest) = path.strip_prefix("extracted/movies/")
        && let Some((movie, _tail)) = rest.split_once('/')
    {
        return format!("extracted/movies/{movie}");
    }
    path.rsplit_once('/')
        .map_or(path, |(head, _leaf)| head)
        .to_owned()
}

/// Supports the `conversation_package_root` operation within this deterministic
/// classification boundary.
fn conversation_package_root(path: &str) -> Option<String> {
    let file_name = path.strip_prefix("extracted/dialog/conversations/")?;
    let stem = file_name.strip_suffix(".wav")?;
    let parts = stem.split('_').collect::<Vec<_>>();
    let kind_index = parts.iter().position(|part| {
        matches!(*part, "convinit" | "noboxconv" | "tutorial")
    })?;
    let speaker = parts.get(kind_index.saturating_add(1))?;
    let mission = parts
        .get(kind_index.saturating_add(2))
        .copied()
        .unwrap_or("global");
    let topic = parts.get(1).copied().unwrap_or("unknown");
    let kind = parts.get(kind_index)?;
    Some(format!(
        "extracted/dialog/conversations/{speaker}/{kind}/{mission}/{topic}"
    ))
}

/// Supports the `io_error` operation within this deterministic classification
/// boundary.
fn io_error(path: &Path) -> impl FnOnce(std::io::Error) -> PipelineError + '_ {
    move |error| PipelineError::new(format!("{}: {error}", path.display()))
}

#[cfg(test)]
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../../../../../tests/migration/pipeline/unit/adapter-outbound/local/two/units/index/tests.rs"]
mod tests;
