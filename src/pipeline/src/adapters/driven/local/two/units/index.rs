// File:
//   - index.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/two/units/index.rs
//
// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE
// Path-Rule:
//   - All paths in this header are repository-root relative.
//
// Boundary-Contract:
// - Owns:
//   - The index contract for pipeline phase two minor units.
// - Must-Not:
//   - Violate repository architecture, path, provenance, or output rules.
// - Allows:
//   - Operations required to validate and execute index.
// - Split-When:
//   - Split when index contains two independently testable contracts.
// - Merge-When:
//   - Another pipeline module owns the same module boundary with no distinct
//   - invariant.
// - Summary:
//   - Minor-unit package index.
// - Description:
//   - Defines index data and behavior for pipeline phase two minor units.
// - Usage:
//   - Used by pipeline phase two minor units code that needs index.
// - Defaults:
//   - No implicit output outside the repository is allowed.
//
// ADRs:
// - docs/adr/pipeline/minor-unit-taxonomy-value-case.md
//
// Large file:
//   - true
//   - Reason: Minor-unit package index keeps tightly coupled validation,
//   - ordering, and deterministic transformation invariants together; split
//   - when a stable independently testable sub-boundary is identified.
//

//! Minor-unit package index.
//!
//! This boundary keeps minor-unit package index explicit and returns
//! deterministic results to pipeline callers.
//! Minor-unit package index.
//!
//! The manifest is the row-level ledger. This index adds the package-level
//! ledger: every manifest minor-unit id must appear in exactly one package, and
//! each package exposes typed id buckets (`world_ids`, `texture_ids`,
//! `material_ids`, etc.) for downstream FBX/world exporters.

use std::collections::{BTreeMap, BTreeSet};
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

/// High-level package category for browsing and exporter routing.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(super) enum PackageCategory {
    /// Playable and non-playable character packages.
    Characters,
    /// Vehicle packages.
    Cars,
    /// Terrain, roads, interiors, and connected world chunks.
    TerrainWorld,
    /// Mission art and mission-specific assets.
    Missions,
    /// Collectible card art.
    Cards,
    /// UI screen/page packages.
    UiScreens,
    /// UI image packages loaded by frontend dynaload.
    UiImages,
    /// UI resource packages from Scrooby resource folders.
    UiResources,
    /// UI vehicle preview packages.
    UiVehiclePreviews,
    /// UI component packages.
    UiComponents,
    /// Language and localization packages.
    Language,
    /// Non-interactive sequence or cinematic art packages.
    Cinematics,
    /// Music packages.
    Music,
    /// Dialog voice packages.
    Dialog,
    /// Sound-effect and ambience packages.
    SoundEffects,
    /// Movie packages.
    Movies,
    /// Mission script packages.
    MissionScripts,
    /// Vehicle tuning script packages.
    VehicleTuning,
    /// Sound script packages.
    SoundScripts,
    /// Props, buildings, tools, effects, or miscellaneous art packages.
    Props,
    /// Extraction report packages.
    ExtractionReports,
    /// Game icon packages.
    GameIcons,
    /// Unmapped package category that must be fixed before export.
    Error,
}

impl PackageCategory {
    /// Stable category label used in index output.
    #[must_use]
    pub(super) const fn as_str(self) -> &'static str {
        match self {
            Self::Characters => "characters",
            Self::Cars => "cars",
            Self::TerrainWorld => "terrain-world",
            Self::Missions => "missions",
            Self::Cards => "cards",
            Self::UiScreens => "ui-screens",
            Self::UiImages => "ui-images",
            Self::UiResources => "ui-resources",
            Self::UiVehiclePreviews => "ui-vehicle-previews",
            Self::UiComponents => "ui-components",
            Self::Language => "language",
            Self::Cinematics => "cinematics",
            Self::Music => "music",
            Self::Dialog => "dialog",
            Self::SoundEffects => "sound-effects",
            Self::Movies => "movies",
            Self::MissionScripts => "mission-scripts",
            Self::VehicleTuning => "vehicle-tuning",
            Self::SoundScripts => "sound-scripts",
            Self::Props => "props",
            Self::ExtractionReports => "extraction-reports",
            Self::GameIcons => "game-icons",
            Self::Error => "error",
        }
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
                .map(
                    |key| TextKeyMember {
                        id: key.id,
                        key: key.key,
                        source_unit_id: key.source_unit_id,
                        subcategory: key.subcategory,
                    },
                )
                .collect(),
        };
        package.fail_closed_on_semantic_debt();
        package
    }

    /// Add a typed member id to the package.
    fn push(
        &mut self,
        member: PackageMember,
    ) {
        self.members
            .push(member);
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
    extracted_root: &Path
) -> PipelineOutcome<StageReport> {
    let packages = read_minor_unit_packages(extracted_root)?;
    let rendered = render_index_jsonl(&packages);
    let path = index_path(extracted_root);
    local_write_text(
        &path, &rendered, true,
    )
    .map_err(io_error(&path))?;
    Ok(
        StageReport {
            name: "minor-unit-index",
            files: packages.len(),
            bytes: u64::try_from(rendered.len()).unwrap_or(u64::MAX),
            note: format!(
                "indexed {} exact minor-unit packages from the manifest ledger",
                packages.len()
            ),
        },
    )
}

/// Read package index from the manifest.
///
/// # Errors
///
/// Returns an error when any required manifest field is missing or when any
/// manifest id is duplicated, missing from the package index, or assigned more
/// than once.
pub(super) fn read_minor_unit_packages(
    extracted_root: &Path
) -> PipelineOutcome<Vec<MinorUnitPackage>> {
    let manifest_path = taxonomy::manifest_path(extracted_root);
    let input =
        local_read_utf8(&manifest_path).map_err(io_error(&manifest_path))?;
    let mut packages = BTreeMap::<String, MinorUnitPackage>::new();
    let mut manifest_ids = BTreeSet::<MinorUnitId>::new();
    let manifest_path_text = manifest_path
        .display()
        .to_string();
    for (line_index, line) in input
        .lines()
        .enumerate()
    {
        if line
            .trim()
            .is_empty()
        {
            continue;
        }
        let row = MinorUnitRow::from_line(
            line,
            line_index.saturating_add(1),
            &manifest_path_text,
        )?;
        if !manifest_ids.insert(
            row.id
                .clone(),
        ) {
            return Err(
                PipelineError::new(
                    format!(
                        "{}:{} duplicates minor-unit id {}",
                        manifest_path_text,
                        line_index.saturating_add(1),
                        row.id
                            .as_str()
                    ),
                ),
            );
        }
        let root = package_root(&row.path);
        packages
            .entry(root.clone())
            .or_insert_with(|| MinorUnitPackage::new(root))
            .push(row.into_member());
    }
    let mut output = packages
        .into_values()
        .collect::<Vec<_>>();
    for package in &mut output {
        package
            .members
            .sort_by(
                |left, right| {
                    left.role
                        .cmp(&right.role)
                        .then_with(
                            || {
                                left.path
                                    .cmp(&right.path)
                            },
                        )
                        .then_with(
                            || {
                                left.id
                                    .cmp(&right.id)
                            },
                        )
                },
            );
        package.refresh_classification_from_members();
    }
    let mut derived = derived_text_packages(
        extracted_root,
        &output,
    )?;
    output.append(&mut derived);
    output.sort_by(
        |left, right| {
            left.package_id
                .cmp(&right.package_id)
        },
    );
    validate_package_coverage(
        &manifest_ids,
        &output,
    )?;
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
}

impl MinorUnitRow {
    /// Supports the `from_line` operation within this deterministic
    /// classification boundary.
    fn from_line(
        line: &str,
        line_number: usize,
        manifest_path: &str,
    ) -> PipelineOutcome<Self> {
        let id_text = required_field(
            line,
            "id",
            line_number,
            manifest_path,
        )?;
        if id_text == taxonomy::UNKNOWN {
            return Err(
                PipelineError::new(
                    format!(
                        "{manifest_path}:{line_number} cannot be indexed \
                         before metadata fill assigns a stable id"
                    ),
                ),
            );
        }
        let id = MinorUnitId::new(id_text).ok_or_else(
            || {
                PipelineError::new(
                    format!(
                        "{manifest_path}:{line_number} has an empty id field"
                    ),
                )
            },
        )?;
        let recovery_status = required_field(
            line,
            "recovery_status",
            line_number,
            manifest_path,
        )?;
        if recovery_status != "fully-decoded" {
            return Err(
                PipelineError::new(
                    format!(
                        "{manifest_path}:{line_number} cannot be indexed \
                         because recovery_status is {recovery_status}"
                    ),
                ),
            );
        }
        Ok(
            Self {
                id,
                path: required_field(
                    line,
                    "path",
                    line_number,
                    manifest_path,
                )?,
                type_: required_field(
                    line,
                    "type",
                    line_number,
                    manifest_path,
                )?,
                kind: required_field(
                    line,
                    "kind",
                    line_number,
                    manifest_path,
                )?,
                source_chunk_kind: required_field(
                    line,
                    "source_chunk_kind",
                    line_number,
                    manifest_path,
                )?,
            },
        )
    }

    /// Supports the `into_member` operation within this deterministic
    /// classification boundary.
    fn into_member(self) -> PackageMember {
        let role = role_from_fields(
            &self.type_,
            &self.kind,
            &self.source_chunk_kind,
        );
        PackageMember {
            id: self.id,
            role,
            path: self.path,
            type_: self.type_,
            kind: self.kind,
            source_chunk_kind: self.source_chunk_kind,
        }
    }
}

/// Supports the `required_field` operation within this deterministic
/// classification boundary.
fn required_field(
    line: &str,
    field: &str,
    line_number: usize,
    manifest_path: &str,
) -> PipelineOutcome<String> {
    read_string_field(
        line, field,
    )
    .ok_or_else(
        || {
            PipelineError::new(
                format!("{manifest_path}:{line_number} missing {field} field"),
            )
        },
    )
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
            if !indexed_ids.insert(
                member
                    .id
                    .clone(),
            ) {
                return Err(
                    PipelineError::new(
                        format!(
                            "minor-unit id {} is assigned to more than one \
                             package",
                            member
                                .id
                                .as_str()
                        ),
                    ),
                );
            }
        }
    }
    if indexed_ids.len() != manifest_ids.len() {
        return Err(
            PipelineError::new(
                format!(
                    "minor-unit index coverage mismatch: manifest has {} ids \
                     but index has {} ids",
                    manifest_ids.len(),
                    indexed_ids.len()
                ),
            ),
        );
    }
    for id in manifest_ids {
        if !indexed_ids.contains(id) {
            return Err(
                PipelineError::new(
                    format!(
                        "minor-unit id {} is not cataloged in any package",
                        id.as_str()
                    ),
                ),
            );
        }
    }
    for id in &indexed_ids {
        if !manifest_ids.contains(id) {
            return Err(
                PipelineError::new(
                    format!(
                        "minor-unit id {} is indexed but missing from the \
                         manifest",
                        id.as_str()
                    ),
                ),
            );
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
                member
                    .id
                    .as_str(),
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
    package: &MinorUnitPackage
) -> (
    PackageCategory,
    String,
) {
    if let Some(subcategory) = mission_script_subcategory_from_evidence(package)
    {
        return (
            PackageCategory::MissionScripts,
            subcategory,
        );
    }
    if let Some(subcategory) = vehicle_tuning_subcategory_from_evidence(package)
    {
        return (
            PackageCategory::VehicleTuning,
            subcategory,
        );
    }
    if let Some(classification) = car_classification_from_evidence(package) {
        return classification;
    }
    if let Some(subcategory) = mission_art_subcategory_from_evidence(package) {
        return (
            PackageCategory::Missions,
            subcategory,
        );
    }
    if let Some(subcategory) = dialog_subcategory_from_evidence(package) {
        return (
            PackageCategory::Dialog,
            subcategory,
        );
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
        return (
            PackageCategory::TerrainWorld,
            subcategory,
        );
    }
    (
        category_from_root(&package.package_root),
        subcategory_from_root(&package.package_root),
    )
}

/// Supports the `car_classification_from_evidence` operation within this
/// deterministic classification boundary.
fn car_classification_from_evidence(
    package: &MinorUnitPackage
) -> Option<(
    PackageCategory,
    String,
)> {
    cars::classification_from_package(package)
}

/// Supports the `cinematic_classification_from_evidence` operation within this
/// deterministic classification boundary.
fn cinematic_classification_from_evidence(
    package: &MinorUnitPackage
) -> Option<(
    PackageCategory,
    String,
)> {
    cinematics::classification_from_package(package)
}

/// Supports the `ui_image_classification_from_evidence` operation within this
/// deterministic classification boundary.
fn ui_image_classification_from_evidence(
    package: &MinorUnitPackage
) -> Option<(
    PackageCategory,
    String,
)> {
    ui_images::classification_from_package(package)
}

/// Supports the `ui_resource_classification_from_evidence` operation within
/// this deterministic classification boundary.
fn ui_resource_classification_from_evidence(
    package: &MinorUnitPackage
) -> Option<(
    PackageCategory,
    String,
)> {
    ui_resources::classification_from_package(package)
}

/// Supports the `terrain_world_subcategory_from_evidence` operation within this
/// deterministic classification boundary.
fn terrain_world_subcategory_from_evidence(
    package: &MinorUnitPackage
) -> Option<String> {
    let tokens = package_id_tokens(package);
    if category_from_root(&package.package_root)
        != PackageCategory::TerrainWorld
        && !terrain_world_tokens_identify_package(&tokens)
    {
        return None;
    }
    let role = terrain_world_role_from_tokens(&tokens)?;
    let detail = terrain_world_detail_from_tokens(
        &tokens, role,
    );
    if let Some(level) = terrain_world_level_from_tokens(&tokens) {
        return Some(format!("terrain-world/{level}/{role}{detail}"));
    }
    terrain_world_bonus_scope_from_tokens(&tokens)
        .map(|scope| format!("terrain-world/{scope}/{role}{detail}"))
}

/// Supports the `terrain_world_detail_from_tokens` operation within this
/// deterministic classification boundary.
fn terrain_world_detail_from_tokens(
    tokens: &[&str],
    role: &str,
) -> String {
    if !terrain_world_role_allows_detail(role) {
        return String::new();
    }
    terrain_world_detail_token(
        tokens, role,
    )
    .map_or_else(
        String::new,
        |token| format!("/{token}"),
    )
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
fn terrain_world_detail_token(
    tokens: &[&str],
    role: &str,
) -> Option<String> {
    let index = tokens
        .iter()
        .position(
            |token| match role {
                "data-records" => token.ends_with("data"),
                "interiors" => level_prefixed_role(
                    token, 'i',
                ),
                "race-props" => token.starts_with("sr") && token.ends_with('p'),
                "regions" => level_prefixed_role(
                    token, 'r',
                ),
                "segments" => terrain_bonus_token(token).is_some(),
                "zones" => level_prefixed_role(
                    token, 'z',
                ),
                _ => false,
            },
        )?;
    let token = tokens
        .get(index)
        .copied()?;
    if role == "regions"
        && let Some(suffix) = tokens.get(index.saturating_add(1))
        && matches!(
            *suffix, "dam"
        )
    {
        return Some(format!("{token}-{suffix}"));
    }
    Some(token.to_owned())
}

/// Supports the `terrain_world_tokens_identify_package` operation within this
/// deterministic classification boundary.
fn terrain_world_tokens_identify_package(tokens: &[&str]) -> bool {
    tokens
        .iter()
        .any(
            |token| {
                mission_level_from_token(token).is_some()
                    || terrain_bonus_token(token).is_some()
            },
        )
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
    tokens: &[&str]
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
        && lower
            .as_bytes()
            .first()
            .copied()
            == Some(b'b')
        && lower
            .as_bytes()
            .get(1..3)
            .is_some_and(
                |digits| {
                    digits
                        .iter()
                        .all(u8::is_ascii_digit)
                },
            ))
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
    } else if tokens
        .iter()
        .any(
            |token| {
                level_prefixed_role(
                    token, 'i',
                )
            },
        )
    {
        Some("interiors")
    } else if tokens
        .iter()
        .any(
            |token| {
                level_prefixed_role(
                    token, 'r',
                )
            },
        )
    {
        Some("regions")
    } else if tokens
        .iter()
        .any(
            |token| {
                level_prefixed_role(
                    token, 'z',
                )
            },
        )
    {
        Some("zones")
    } else if joined.contains("door") {
        Some("mission-doors")
    } else if tokens
        .iter()
        .any(|token| token.ends_with("data"))
    {
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
fn level_prefixed_role(
    token: &str,
    marker: char,
) -> bool {
    let mut chars = token.chars();
    chars.next() == Some('l')
        && chars
            .next()
            .is_some_and(|value| value.is_ascii_digit())
        && chars.next() == Some(marker)
}

/// Supports the `semantic_debt_error_subcategory` operation within this
/// deterministic classification boundary.
fn semantic_debt_error_subcategory(subcategory: &str) -> Option<&'static str> {
    subcategory
        .split('/')
        .find_map(semantic_debt_segment_error)
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
    package: &MinorUnitPackage
) -> Option<String> {
    if !has_member_kind(
        package,
        "mission-script",
    ) {
        return None;
    }
    let scope = mission_scope_from_package_evidence(package)
        .unwrap_or_else(|| "missions/bootstrap".to_owned());
    let detail = mission_script_detail_from_tokens(
        &package_id_tokens(package),
        &scope,
    );
    Some(format!("{scope}/scripts{detail}"))
}

/// Supports the `mission_script_detail_from_tokens` operation within this
/// deterministic classification boundary.
fn mission_script_detail_from_tokens(
    tokens: &[&str],
    scope: &str,
) -> String {
    if scope != "missions/bootstrap" {
        return String::new();
    }
    let Some(index) = tokens
        .iter()
        .position(|token| *token == "scripts")
    else {
        return String::new();
    };
    let detail_tokens = tokens
        .get(index.saturating_add(1)..)
        .unwrap_or(&[]);
    if detail_tokens.is_empty() {
        "/root".to_owned()
    } else {
        format!(
            "/{}",
            detail_tokens.join("-")
        )
    }
}

/// Supports the `vehicle_tuning_subcategory_from_evidence` operation within
/// this deterministic classification boundary.
fn vehicle_tuning_subcategory_from_evidence(
    package: &MinorUnitPackage
) -> Option<String> {
    if !has_member_kind(
        package,
        "vehicle-tuning",
    ) {
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
    let Some(index) = tokens
        .iter()
        .position(|token| *token == "cars")
    else {
        return String::new();
    };
    let detail_tokens = tokens
        .get(index.saturating_add(1)..)
        .unwrap_or(&[]);
    if detail_tokens.is_empty() {
        "/root".to_owned()
    } else {
        format!(
            "/{}",
            detail_tokens.join("-")
        )
    }
}

/// Supports the `mission_art_subcategory_from_evidence` operation within this
/// deterministic classification boundary.
fn mission_art_subcategory_from_evidence(
    package: &MinorUnitPackage
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
    let role = dominant_package_role(
        package, "assets",
    );
    let detail = mission_art_detail_from_tokens(&package_id_tokens(package));
    Some(format!("{scope}/{role}{detail}"))
}

/// Supports the `mission_art_detail_from_tokens` operation within this
/// deterministic classification boundary.
fn mission_art_detail_from_tokens(tokens: &[&str]) -> String {
    let Some(index) = tokens
        .iter()
        .position(|token| *token == "missions")
    else {
        return String::new();
    };
    let raw_tail = tokens
        .get(index.saturating_add(1)..)
        .unwrap_or(&[]);
    let detail_tokens = mission_detail_tokens(raw_tail);
    if detail_tokens.is_empty() {
        String::new()
    } else {
        format!(
            "/{}",
            detail_tokens.join("-")
        )
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
        return tokens
            .get(1..)
            .unwrap_or(&[]);
    }
    tokens
}

/// Supports the `dialog_subcategory_from_evidence` operation within this
/// deterministic classification boundary.
fn dialog_subcategory_from_evidence(
    package: &MinorUnitPackage
) -> Option<String> {
    if !package
        .members
        .iter()
        .any(|member| member.role == MinorUnitRole::Audio)
    {
        return None;
    }
    let package_id = package
        .package_id
        .as_str();
    if !package_id.starts_with("extracted-dialog")
        && !package_id.starts_with("extracted-lmlm-customfiles")
    {
        return None;
    }
    let tokens = package_id_tokens(package);
    let speaker = dialog_speaker_from_tokens(&tokens).unwrap_or("unknown");
    let kind = dialog_kind_from_tokens(
        package_id, &tokens,
    );
    let context = dialog_context_from_tokens(&tokens);
    let detail = dialog_detail_from_tokens(&tokens);
    let archive = dialog_archive_detail_from_tokens(&tokens);
    Some(format!("dialog/{speaker}/{kind}/{context}{detail}{archive}"))
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
    dialog_detail_after_marker(
        tokens, "tutorial",
    )
    .or_else(
        || {
            dialog_detail_after_marker(
                tokens, "convinit",
            )
        },
    )
    .or_else(
        || {
            dialog_detail_after_marker(
                tokens,
                "noboxconv",
            )
        },
    )
    .map_or_else(
        String::new,
        |detail| format!("/{detail}"),
    )
}

/// Supports the `dialog_detail_after_marker` operation within this
/// deterministic classification boundary.
fn dialog_detail_after_marker(
    tokens: &[&str],
    marker: &str,
) -> Option<String> {
    let marker_index = tokens
        .iter()
        .position(|token| *token == marker)?;
    let raw_detail_tokens = tokens.get(marker_index.saturating_add(1)..)?;
    let detail_tokens = raw_detail_tokens
        .strip_prefix(&["global"])
        .unwrap_or(raw_detail_tokens);
    (!detail_tokens.is_empty()).then(
        || {
            format!(
                "{marker}/{}",
                detail_tokens.join("-")
            )
        },
    )
}

/// Supports the `has_member_kind` operation within this deterministic
/// classification boundary.
fn has_member_kind(
    package: &MinorUnitPackage,
    kind: &str,
) -> bool {
    package
        .members
        .iter()
        .any(|member| member.kind == kind)
}

/// Supports the `mission_scope_from_package_evidence` operation within this
/// deterministic classification boundary.
fn mission_scope_from_package_evidence(
    package: &MinorUnitPackage
) -> Option<String> {
    let tokens = package_id_tokens(package);
    mission_scope_from_tokens(&tokens).or_else(
        || {
            package
                .members
                .iter()
                .find_map(
                    |member| {
                        member
                            .id
                            .as_str()
                            .split('-')
                            .find_map(mission_level_from_token)
                            .map(|level| format!("missions/{level}"))
                    },
                )
        },
    )
}

/// Supports the `mission_scope_from_tokens` operation within this deterministic
/// classification boundary.
fn mission_scope_from_tokens(tokens: &[&str]) -> Option<String> {
    for (index, token) in tokens
        .iter()
        .enumerate()
    {
        match *token {
            "generic" => return Some("missions/runtime".to_owned()),
            "h2h" | "level08" | "l8" => {
                return Some("missions/head-to-head".to_owned());
            }
            value if is_tutorial_mission_asset(value) => {
                return Some("missions/tutorial".to_owned());
            }
            value => {
                if let Some(level) = mission_level_from_token(value) {
                    let next = tokens
                        .get(index.saturating_add(1))
                        .copied();
                    if level == "level-01"
                        && next.is_some_and(is_tutorial_mission_asset)
                    {
                        return Some("missions/tutorial".to_owned());
                    }
                    return Some(format!("missions/{level}"));
                }
            }
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
        && bytes
            .first()
            .copied()
            == Some(b'l')
        && bytes
            .get(1)
            .copied()
            == Some(b'0')
        && let Some(raw_digit) = bytes
            .get(2)
            .copied()
    {
        let digit = char::from(raw_digit);
        if matches!(
            digit,
            '1'..='7'
        ) {
            return Some(format!("level-0{digit}"));
        }
    }
    if bytes.len() >= 2
        && bytes
            .first()
            .copied()
            == Some(b'l')
        && let Some(raw_digit) = bytes
            .get(1)
            .copied()
    {
        let digit = char::from(raw_digit);
        if matches!(
            digit,
            '1'..='7'
        ) {
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
    if let Some(index) = tokens
        .iter()
        .position(|token| *token == "customfiles")
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
    if let Some(index) = tokens
        .iter()
        .position(|token| *token == "conversations")
    {
        return tokens
            .get(index.saturating_add(1))
            .map(|speaker| speaker_name(speaker));
    }
    if let Some(index) = tokens
        .iter()
        .position(|token| *token == "dialog")
    {
        return tokens
            .get(index.saturating_add(1)..)
            .filter(
                |speaker_tokens| {
                    !speaker_tokens.is_empty()
                        && speaker_tokens.first() != Some(&"conversations")
                },
            )
            .map(|speaker_tokens| speaker_name(&speaker_tokens.join("-")));
    }
    None
}

/// Supports the `dialog_kind_from_tokens` operation within this deterministic
/// classification boundary.
fn dialog_kind_from_tokens(
    package_id: &str,
    tokens: &[&str],
) -> &'static str {
    if package_id.starts_with("extracted-lmlm-customfiles") {
        "mod-override"
    } else if tokens.contains(&"conversations") {
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
        .position(
            |token| {
                matches!(
                    *token,
                    "convinit" | "noboxconv"
                )
            },
        )
        .and_then(
            |index| {
                tokens
                    .get(index.saturating_add(2))
                    .copied()
            },
        )
        .filter(|topic| *topic != "free-roam")
}

/// Supports the `subcategory_from_root` operation within this deterministic
/// classification boundary.
fn subcategory_from_root(package_root: &str) -> String {
    if let Some(dialog) = dialog_subcategory(package_root) {
        return dialog;
    }
    if package_root == "extracted/art/cards" {
        return "cards/pickup-effects".to_owned();
    }
    if let Some(character) = character_subcategory(package_root) {
        return character;
    }
    if let Some(mission) = mission_subcategory(package_root) {
        return mission;
    }
    if let Some(script) = script_subcategory(package_root) {
        return script;
    }
    if let Some(language) = language_subcategory(package_root) {
        return language;
    }
    if let Some(prop) = prop_subcategory(package_root) {
        return prop;
    }
    category_from_root(package_root)
        .as_str()
        .to_owned()
}

/// Supports the `character_subcategory` operation within this deterministic
/// classification boundary.
fn character_subcategory(package_root: &str) -> Option<String> {
    let root = package_root.to_ascii_lowercase();
    let asset_part = root
        .strip_prefix("extracted/art/chars/")
        .or_else(|| root.strip_prefix("extracted/game/art/chars/"));
    let Some(asset) = asset_part else {
        if root == "extracted/game/art/chars" {
            return Some("characters/registry/package".to_owned());
        }
        return None;
    };
    if asset == "global" {
        return Some("characters/rig/common".to_owned());
    }
    if let Some(base) = asset.strip_suffix("_a") {
        return Some(
            format!(
                "characters/{}/animation-set",
                character_name(base)
            ),
        );
    }
    if let Some(base) = asset.strip_suffix("_electrocuted") {
        return Some(
            format!(
                "characters/{}/effect/electrocuted",
                character_name(base)
            ),
        );
    }
    // cspell:disable-next-line -- kickwave
    if let Some(base) = asset.strip_suffix("_kickwave") {
        return Some(
            format!(
                // cspell:disable-next-line -- kickwave
                "characters/{}/effect/kickwave",
                character_name(base)
            ),
        );
    }
    if let Some((character, costume)) = character_costume(asset) {
        return Some(format!("characters/{character}/costume/{costume}"));
    }
    if asset == "ndr_m" {
        return Some("characters/ned/base-model/ndr".to_owned());
    }
    let base = asset
        .strip_suffix("_m")
        .unwrap_or(asset);
    Some(
        format!(
            "characters/{}/{}",
            character_name(base),
            character_model_group(base)
        ),
    )
}

/// Supports the `character_costume` operation within this deterministic
/// classification boundary.
fn character_costume(
    asset: &str
) -> Option<(
    &'static str,
    &'static str,
)> {
    apu_costume(asset)
        .or_else(|| bart_costume(asset))
        .or_else(|| homer_costume(asset))
        .or_else(|| lisa_costume(asset))
        .or_else(|| marge_costume(asset))
        .or_else(|| barney_costume(asset))
}

/// Supports the `apu_costume` operation within this deterministic
/// classification boundary.
fn apu_costume(
    asset: &str
) -> Option<(
    &'static str,
    &'static str,
)> {
    match asset {
        // cspell:disable-next-line -- amer
        "a_amer_m" => Some(
            (
                "apu", "american",
            ),
        ),
        "a_army_m" => Some(
            (
                "apu", "army",
            ),
        ),
        // cspell:disable-next-line -- besh
        "a_besh_m" => Some(
            (
                "apu",
                "be-sharps",
            ),
        ),
        _ => None,
    }
}

/// Supports the `bart_costume` operation within this deterministic
/// classification boundary.
fn bart_costume(
    asset: &str
) -> Option<(
    &'static str,
    &'static str,
)> {
    match asset {
        "b_foot_m" => Some(
            (
                "bart", "football",
            ),
        ),
        "b_hugo_m" => Some(
            (
                "bart", "hugo",
            ),
        ),
        "b_man_m" => Some(
            (
                "bart", "bartman",
            ),
        ),
        // cspell:disable-next-line -- mili
        "b_mili_m" => Some(
            (
                "bart", "military",
            ),
        ),
        // cspell:disable-next-line -- ninj
        "b_ninj_m" => Some(
            (
                "bart", "ninja",
            ),
        ),
        "b_tall_m" => Some(
            (
                "bart", "tall",
            ),
        ),
        _ => None,
    }
}

/// Supports the `homer_costume` operation within this deterministic
/// classification boundary.
fn homer_costume(
    asset: &str
) -> Option<(
    &'static str,
    &'static str,
)> {
    match asset {
        // cspell:disable-next-line -- donu
        "h_donu_m" => Some(
            (
                "homer", "donut",
            ),
        ),
        "h_evil_m" => Some(
            (
                "homer", "evil",
            ),
        ),
        "h_fat_m" => Some(
            (
                "homer", "muumuu",
            ),
        ),
        // cspell:disable-next-line -- scuz
        "h_scuz_m" => Some(
            (
                "homer", "scuzzy",
            ),
        ),
        // cspell:disable-next-line -- stcr
        "h_stcr_m" => Some(
            (
                "homer",
                "stonecutter",
            ),
        ),
        // cspell:disable-next-line -- undr
        "h_undr_m" => Some(
            (
                "homer",
                "underwear",
            ),
        ),
        _ => None,
    }
}

/// Supports the `lisa_costume` operation within this deterministic
/// classification boundary.
fn lisa_costume(
    asset: &str
) -> Option<(
    &'static str,
    &'static str,
)> {
    match asset {
        "l_cool_m" => Some(
            (
                "lisa", "cool",
            ),
        ),
        "l_flor_m" => Some(
            (
                "lisa", "florida",
            ),
        ),
        // cspell:disable-next-line -- jers
        "l_jers_m" => Some(
            (
                "lisa", "jersey",
            ),
        ),
        _ => None,
    }
}

/// Supports the `marge_costume` operation within this deterministic
/// classification boundary.
fn marge_costume(
    asset: &str
) -> Option<(
    &'static str,
    &'static str,
)> {
    match asset {
        "m_pink_m" => Some(
            (
                "marge", "pink",
            ),
        ),
        // cspell:disable-next-line -- poli
        "m_poli_m" => Some(
            (
                "marge", "police",
            ),
        ),
        // cspell:disable-next-line -- pris
        "m_pris_m" => Some(
            (
                "marge", "prisoner",
            ),
        ),
        _ => None,
    }
}

/// Supports the `barney_costume` operation within this deterministic
/// classification boundary.
fn barney_costume(
    asset: &str
) -> Option<(
    &'static str,
    &'static str,
)> {
    match asset {
        // cspell:disable-next-line -- brn
        "brn_un_m" => Some(
            (
                "barney",
                "underwear",
            ),
        ),
        _ => None,
    }
}

/// Supports the `character_model_group` operation within this deterministic
/// classification boundary.
fn character_model_group(base: &str) -> &'static str {
    const CROWD_PREFIXES: &[&str] = &[
        "boy", "girl", "male", "fem",
        "olady", // cspell:disable-line -- olady
        "busm",  // cspell:disable-line -- busm
        "busw",  // cspell:disable-line -- busw
        "joger", // cspell:disable-line -- joger
        "sail", "const", "rednk", // cspell:disable-line -- rednk
        "zfem",  // cspell:disable-line -- zfem
        "zmale", // cspell:disable-line -- zmale
    ];

    if CROWD_PREFIXES
        .iter()
        .any(|prefix| base.starts_with(prefix))
    {
        "crowd-model"
    // cspell:disable-next-line -- franke
    } else if base.starts_with('z') || base == "witch" || base == "franke" {
        "halloween-model"
    } else {
        "base-model"
    }
}

/// Supports the `character_name` operation within this deterministic
/// classification boundary.
fn character_name(code: &str) -> String {
    match code {
        // cspell:disable-next-line -- askinn
        "askinn" => "agnes-skinner".to_owned(),
        "apu" => "apu".to_owned(),
        // cspell:disable-next-line -- brn
        "barney" | "brn" => "barney".to_owned(),
        "bart" => "bart".to_owned(),
        "beeman" => "bumblebee-man".to_owned(),
        "burns" => "burns".to_owned(),
        // cspell:disable-next-line -- captai
        "captai" => "sea-captain".to_owned(),
        "carl" => "carl".to_owned(),
        // cspell:disable-next-line -- cbg
        "cbg" => "comic-book-guy".to_owned(),
        "cletus" => "cletus".to_owned(),
        "dolph" => "dolph".to_owned(),
        "eddie" => "eddie".to_owned(),
        // cspell:disable-next-line -- franke
        "franke" => "frankenstein".to_owned(),
        "frink" => "frink".to_owned(),
        "gil" => "gil".to_owned(),
        // cspell:disable-next-line -- grandp
        "grandp" => "grampa".to_owned(),
        // cspell:disable-next-line -- hibber
        "hibber" => "hibbert".to_owned(),
        "homer" => "homer".to_owned(),
        "hooker" => "hooker".to_owned(),
        "jasper" => "jasper".to_owned(),
        "jimbo" => "jimbo".to_owned(),
        // cspell:disable-next-line -- kearne
        "kearne" => "kearney".to_owned(),
        "krusty" => "krusty".to_owned(),
        "lenny" => "lenny".to_owned(),
        "lisa" => "lisa".to_owned(),
        "lou" => "lou".to_owned(),
        "louie" => "louie".to_owned(),
        "marge" => "marge".to_owned(),
        // cspell:disable-next-line -- milhou
        "milhou" => "milhouse".to_owned(),
        // cspell:disable-next-line -- mobstr
        "mobstr" => "mobster".to_owned(),
        "moe" => "moe".to_owned(),
        // cspell:disable-next-line -- molema
        "molema" => "moleman".to_owned(),
        "ndr" | "ned" => "ned".to_owned(),
        "nelson" => "nelson".to_owned(),
        // cspell:disable-next-line -- npd
        "npd" => "apu-driver".to_owned(),
        // cspell:disable-next-line -- nps
        "nps" => "school-bus-driver".to_owned(),
        // cspell:disable-next-line -- nrivie
        "nrivie" => "riviera".to_owned(),
        // cspell:disable-next-line -- nuclea
        "nuclea" => "nuclear-worker".to_owned(),
        "otto" => "otto".to_owned(),
        "patty" => "patty".to_owned(),
        "ralph" => "ralph".to_owned(),
        "selma" => "selma".to_owned(),
        // cspell:disable-next-line -- skinne
        "skinne" => "skinner".to_owned(),
        // cspell:disable-next-line -- smithe
        "smithe" => "smithers".to_owned(),
        "snake" => "snake".to_owned(),
        "teen" => "squeaky-voiced-teen".to_owned(),
        "wiggum" => "wiggum".to_owned(),
        "willie" => "willie".to_owned(),
        other => other.replace(
            '_', "-",
        ),
    }
}

/// Supports the `dialog_subcategory` operation within this deterministic
/// classification boundary.
fn dialog_subcategory(package_root: &str) -> Option<String> {
    let root = package_root.to_ascii_lowercase();
    if let Some(name) = root.strip_prefix("extracted/dialog/conversations/") {
        let parts = name
            .split('/')
            .collect::<Vec<_>>();
        if let Some(speaker) = parts.first()
            && let Some(kind) = parts.get(1)
        {
            return Some(
                format!(
                    "dialog/{}/conversation/{}/{}",
                    speaker_name(speaker),
                    kind,
                    parts
                        .get(2)
                        .copied()
                        .unwrap_or("global")
                ),
            );
        }
    }
    if let Some(character) = root.strip_prefix("extracted/dialog/") {
        return Some(
            format!(
                "dialog/{}/ad-lib",
                speaker_name(character)
            ),
        );
    }
    if let Some(character) = root.strip_prefix("extracted/lmlm/customfiles/") {
        return Some(
            format!(
                "dialog/{}/mod-override",
                speaker_name(character)
            ),
        );
    }
    None
}

/// Supports the `mission_subcategory` operation within this deterministic
/// classification boundary.
fn mission_subcategory(package_root: &str) -> Option<String> {
    let root = package_root.to_ascii_lowercase();
    if let Some(rest) = root.strip_prefix("extracted/art/missions/") {
        let parts = rest
            .split('/')
            .collect::<Vec<_>>();
        return Some(
            match parts.as_slice() {
                [
                    "generic",
                    tail @ ..,
                ] => format!(
                    "missions/generic/{}",
                    tail.first()
                        .copied()
                        .unwrap_or("root")
                ),
                [
                    "h2h",
                    tail @ ..,
                ] => format!(
                    "missions/head-to-head/{}",
                    tail.first()
                        .copied()
                        .unwrap_or("root")
                ),
                [
                    level,
                    asset,
                    ..,
                ] if level.starts_with("level") => {
                    let normalized = normalize_level(level);
                    if *level == "level01" && is_tutorial_mission_asset(asset) {
                        format!("missions/tutorial/{asset}")
                    } else {
                        format!("missions/{normalized}/{asset}")
                    }
                }
                [only] => format!("missions/uncategorized/{only}"),
                [
                    head,
                    tail @ ..,
                ] => format!(
                    "missions/uncategorized/{}/{}",
                    head,
                    tail.first()
                        .copied()
                        .unwrap_or("root")
                ),
                [] => "missions/root".to_owned(),
            },
        );
    }
    None
}

/// Supports the `script_subcategory` operation within this deterministic
/// classification boundary.
fn script_subcategory(package_root: &str) -> Option<String> {
    let root = package_root.to_ascii_lowercase();
    if root == "extracted/game/scripts" {
        return Some("missions/bootstrap/scripts/root".to_owned());
    }
    if root == "extracted/game/scripts/missions" {
        return Some("missions/bootstrap/scripts/missions".to_owned());
    }
    if let Some(rest) = root.strip_prefix("extracted/game/scripts/missions/") {
        return Some(
            format!(
                "missions/{}/scripts",
                normalize_level(rest)
            ),
        );
    }
    if let Some(rest) =
        root.strip_prefix("extracted/game/scripts/cars/missions/")
    {
        return Some(
            format!(
                "missions/{}/vehicle-tuning",
                normalize_level(rest)
            ),
        );
    }
    if let Some(rest) = root.strip_prefix("extracted/game/scripts/cars/") {
        return Some(format!("vehicle-tuning/{rest}"));
    }
    if root == "extracted/game/scripts/cars" {
        return Some("vehicle-tuning/free-roam".to_owned());
    }
    if root.starts_with("extracted/scripts/sound/scripts") {
        return Some("sound-scripts/vehicle-dialog-routing".to_owned());
    }
    None
}

/// Supports the `language_subcategory` operation within this deterministic
/// classification boundary.
fn language_subcategory(package_root: &str) -> Option<String> {
    let root = package_root.to_ascii_lowercase();
    if root.starts_with("extracted/lmlm/customtext") {
        return Some("language/mod-overrides".to_owned());
    }
    if root.ends_with("/language") || root.contains("/language/") {
        if root.contains("/scrooby2/") {
            return Some("language/ui-text/scene-layouts".to_owned());
        }
        if root.contains("/scrooby/") {
            return Some("language/ui-text/sprite-layouts".to_owned());
        }
        return Some("language/ui-text".to_owned());
    }
    None
}

/// Supports the `prop_subcategory` operation within this deterministic
/// classification boundary.
fn prop_subcategory(package_root: &str) -> Option<String> {
    let root = package_root.to_ascii_lowercase();
    let asset = root
        .strip_prefix("extracted/art/")
        .or_else(|| root.strip_prefix("extracted/game/art/"))?;
    match asset {
        // cspell:disable-next-line -- atc
        "atc/atc" | "phonecamera" | "wrench" => Some(
            format!(
                "props/{}",
                asset.replace(
                    '_', "-"
                )
            ),
        ),
        _ => None,
    }
}

/// Supports the `normalize_level` operation within this deterministic
/// classification boundary.
fn normalize_level(value: &str) -> String {
    let lower = value.to_ascii_lowercase();
    if let Some(number) = lower.strip_prefix("level")
        && let Ok(parsed) = number.parse::<u8>()
    {
        return format!("level-{parsed:02}");
    }
    lower
}

/// Supports the `is_tutorial_mission_asset` operation within this deterministic
/// classification boundary.
fn is_tutorial_mission_asset(value: &str) -> bool {
    matches!(
        value,
        "m0" | "demo" | "democams" | "mission0cam" | "tutorial"
    )
}

/// Supports the `pedestrian_speaker_name` operation within this deterministic
/// classification boundary.
fn pedestrian_speaker_name(code_or_name: &str) -> Option<&'static str> {
    match code_or_name {
        "generic-boy-1" => Some("pedestrian-boy-1"),
        "generic-boy-2" => Some("pedestrian-boy-2"),
        "generic-female-1" => Some("pedestrian-woman-1"),
        "generic-female-2" => Some("pedestrian-woman-2"),
        "generic-girl-1" => Some("pedestrian-girl-1"),
        "generic-girl-2" => Some("pedestrian-girl-2"),
        "generic-male-1" => Some("pedestrian-man-1"),
        "generic-male-2" => Some("pedestrian-man-2"),
        _ => None,
    }
}

/// Supports the `speaker_name` operation within this deterministic
/// classification boundary.
fn speaker_name(code_or_name: &str) -> &'static str {
    if let Some(pedestrian) = pedestrian_speaker_name(code_or_name) {
        return pedestrian;
    }
    match code_or_name {
        // cspell:disable-next-line -- agn
        "agn" | "agnes" => "agnes",
        "apu" => "apu",
        // cspell:disable-next-line -- brn
        "brn" | "barney" => "barney",
        // cspell:disable-next-line -- brt
        "brt" | "bart" => "bart",
        "bur" | "burns" => "burns",
        // cspell:disable-next-line -- cbg
        "cbg" | "comic_book_guy" => "comic-book-guy",
        // cspell:disable-next-line -- clt
        "clt" | "cletus" => "cletus",
        "crl" | "carl" => "carl",
        // cspell:disable-next-line -- fla
        "fla" | "flanders" => "flanders",
        // cspell:disable-next-line -- frk
        "frk" | "dr.frink" | "frink" => "frink",
        "grp" | "grampa" | "grandpa" => "grampa",
        // cspell:disable-next-line -- hib
        "hib" | "dr.hibbert" => "hibbert",
        // cspell:disable-next-line -- hom
        "hom" | "homer" => "homer",
        "kea" | "kearney" => "kearney",
        // cspell:disable-next-line -- kru
        "kru" | "krusty" => "krusty",
        "len" | "lenny" => "lenny",
        "lis" | "lisa" => "lisa",
        "mil" | "milhouse" => "milhouse",
        "moe" => "moe",
        // cspell:disable-next-line -- mrg
        "mrg" | "marge" => "marge",
        // cspell:disable-next-line -- nel
        "nel" | "nelson" => "nelson",
        // cspell:disable-next-line -- nic
        "nic" | "dr.nick" => "dr-nick",
        // cspell:disable-next-line -- oto
        "oto" | "otto" => "otto",
        "pat" | "patty" => "patty",
        // cspell:disable-next-line -- ral
        "ral" | "ralph" => "ralph",
        "sea" | "captain" => "sea-captain",
        // cspell:disable-next-line -- skn
        "skn" | "skinner" => "skinner",
        // cspell:disable-next-line -- smi
        "smi" | "smithers" => "smithers",
        // cspell:disable-next-line -- snk
        "snk" | "snake" => "snake",
        // cspell:disable-next-line -- svt
        "svt" | "squeaky_voiced_teen" => "squeaky-voiced-teen",
        "wig" | "wiggum" => "wiggum",
        // cspell:disable-next-line -- zom
        "zom" | "zm1" | "zm2" | "zm3" => "zombie",
        other => Box::leak(
            other
                .replace(
                    '_', "-",
                )
                .into_boxed_str(),
        ),
    }
}

/// Supports the `category_from_root` operation within this deterministic
/// classification boundary.
pub(super) fn category_from_root(package_root: &str) -> PackageCategory {
    let root = package_root.to_ascii_lowercase();
    if root == "extracted" {
        PackageCategory::ExtractionReports
    } else if root == "game" {
        PackageCategory::GameIcons
    } else if root.starts_with("extracted/art/chars/")
        || root.starts_with("extracted/game/art/chars")
    {
        PackageCategory::Characters
    } else if root.starts_with("extracted/art/cars/") {
        PackageCategory::Cars
    } else if root.starts_with("extracted/art/frontend/dynaload/cars/")
        || root.starts_with("extracted/game/art/frontend/dynaload/cars")
    {
        PackageCategory::UiVehiclePreviews
    } else if root.starts_with("extracted/art/frontend/dynaload/images")
        || root.starts_with("extracted/game/art/frontend/dynaload/images")
    {
        PackageCategory::UiImages
    } else if root.starts_with("extracted/art/frontend/scrooby/resource/")
        || root.starts_with("extracted/art/frontend/scrooby2/resource/")
    {
        PackageCategory::UiResources
    } else if root.ends_with("/language")
        || root.contains("/language/")
        || root.starts_with("extracted/lmlm/customtext")
    {
        PackageCategory::Language
    } else if root.starts_with("extracted/art/frontend/scrooby/")
        || root.starts_with("extracted/art/frontend/scrooby2/")
        || root.starts_with("extracted/game/art/frontend/scrooby/")
        || root.starts_with("extracted/game/art/frontend/scrooby2")
    {
        PackageCategory::UiScreens
    } else if root == "extracted/lmlm" {
        PackageCategory::UiComponents
    } else if root.starts_with("extracted/art/missions/") {
        PackageCategory::Missions
    } else if root == "extracted/art/cards"
        || root.starts_with("extracted/art/cards/")
    {
        PackageCategory::Cards
    } else if root.starts_with("extracted/art/nis/")
        || root.starts_with("extracted/nis/")
    {
        PackageCategory::Cinematics
    } else if root.starts_with("extracted/music") {
        PackageCategory::Music
    } else if root.starts_with("extracted/dialog")
        || root.starts_with("extracted/lmlm/customfiles/")
    {
        PackageCategory::Dialog
    } else if root.starts_with("extracted/movies/")
        || root.starts_with("extracted/lmlm/movies/")
    {
        PackageCategory::Movies
    } else if root == "extracted/game/scripts/cars"
        || root.starts_with("extracted/game/scripts/cars/")
    {
        PackageCategory::VehicleTuning
    } else if root == "extracted/game/scripts"
        || root.starts_with("extracted/game/scripts/missions")
    {
        PackageCategory::MissionScripts
    } else if root.starts_with("extracted/scripts/sound/scripts") {
        PackageCategory::SoundScripts
    } else if root.starts_with("extracted/soundfx/")
        || root.starts_with("extracted/ambience/")
        || root.starts_with("extracted/carsound/")
        || root.starts_with("extracted/game/sound/")
        || root == "extracted/sound"
    {
        PackageCategory::SoundEffects
    } else if is_world_art_root(&root) {
        PackageCategory::TerrainWorld
    } else if root.starts_with("extracted/art/") {
        PackageCategory::Props
    } else {
        PackageCategory::Error
    }
}

/// Supports the `is_world_art_root` operation within this deterministic
/// classification boundary.
fn is_world_art_root(root: &str) -> bool {
    let Some(name) = root.strip_prefix("extracted/art/") else {
        return false;
    };
    let lower = name.to_ascii_lowercase();
    lower.contains("terra")
        || lower.starts_with('l')
            && lower
                .chars()
                .nth(1)
                .is_some_and(|value| value.is_ascii_digit())
        || lower.starts_with('b')
            && lower
                .chars()
                .nth(1)
                .is_some_and(|value| value.is_ascii_digit())
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
        }
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
        .map_or(
            path,
            |(head, _leaf)| head,
        )
        .to_owned()
}

/// Supports the `conversation_package_root` operation within this deterministic
/// classification boundary.
fn conversation_package_root(path: &str) -> Option<String> {
    let file_name = path.strip_prefix("extracted/dialog/conversations/")?;
    let stem = file_name.strip_suffix(".wav")?;
    let parts = stem
        .split('_')
        .collect::<Vec<_>>();
    let kind_index = parts
        .iter()
        .position(
            |part| {
                matches!(
                    *part,
                    "convinit" | "noboxconv" | "tutorial"
                )
            },
        )?;
    let speaker = parts.get(kind_index.saturating_add(1))?;
    let mission = parts
        .get(kind_index.saturating_add(2))
        .copied()
        .unwrap_or("global");
    let topic = parts
        .get(1)
        .copied()
        .unwrap_or("unknown");
    let kind = parts.get(kind_index)?;
    Some(
        format!(
            "extracted/dialog/conversations/{speaker}/{kind}/{mission}/{topic}"
        ),
    )
}

/// Supports the `io_error` operation within this deterministic classification
/// boundary.
fn io_error(path: &Path) -> impl FnOnce(std::io::Error) -> PipelineError + '_ {
    move |error| {
        PipelineError::new(
            format!(
                "{}: {error}",
                path.display()
            ),
        )
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::{
        MinorUnitId, MinorUnitPackage, MinorUnitRole, PackageCategory,
        PackageId, PackageMember, category_from_root, package_root,
        role_from_fields, subcategory_from_root, validate_package_coverage,
    };

    #[test]
    fn package_ids_collapse_separator_runs() {
        for (root, expected) in [
            (
                "/Extracted//Art/Homer V/",
                "extracted-art-homer-v",
            ),
            (
                "derived///language__menu",
                "derived-language-menu",
            ),
        ] {
            assert_eq!(
                PackageId::from_root(root).as_str(),
                expected,
            );
        }
    }

    #[test]
    fn package_root_groups_package_components() {
        assert_eq!(
            package_root("extracted/art/L1_TERRA/components/texture/a.png"),
            "extracted/art/L1_TERRA"
        );
        assert_eq!(
            package_root("extracted/movies/fmv1A/movie.mov"),
            "extracted/movies/fmv1A"
        );
    }

    #[test]
    fn character_subcategories_split_models_costumes_and_effects() {
        assert_eq!(
            subcategory_from_root("extracted/art/chars/homer_m"),
            "characters/homer/base-model"
        );
        assert_eq!(
            // cspell:disable-next-line -- stcr
            subcategory_from_root("extracted/art/chars/h_stcr_m"),
            "characters/homer/costume/stonecutter"
        );
        assert_eq!(
            subcategory_from_root("extracted/art/chars/homer_electrocuted"),
            "characters/homer/effect/electrocuted"
        );
        assert_eq!(
            subcategory_from_root("extracted/art/chars/b_man_m"),
            "characters/bart/costume/bartman"
        );
        assert_eq!(
            subcategory_from_root("extracted/art/chars/bart_a"),
            "characters/bart/animation-set"
        );
    }

    #[test]
    fn conversation_package_root_splits_by_speaker_kind_and_mission() {
        assert_eq!(
            package_root(
                // cspell:disable-next-line -- hom
                "extracted/dialog/conversations/c_accuse_1_convinit_hom_l1m7.\
                 wav"
            ),
            // cspell:disable-next-line -- hom
            "extracted/dialog/conversations/hom/convinit/l1m7/accuse"
        );
        assert_eq!(
            subcategory_from_root(
                // cspell:disable-next-line -- hom
                "extracted/dialog/conversations/hom/convinit/l1m7/accuse"
            ),
            "dialog/homer/conversation/convinit/l1m7"
        );
    }

    #[test]
    fn package_category_routes_export_domains() {
        assert_eq!(
            category_from_root("extracted/art/chars/homer_m"),
            PackageCategory::Characters
        );
        assert_eq!(
            category_from_root("extracted/art/cars/homer_v"),
            PackageCategory::Cars
        );
        assert_eq!(
            category_from_root("extracted/art/L1_TERRA"),
            PackageCategory::TerrainWorld
        );
        assert_eq!(
            subcategory_from_root("extracted/art/missions/level01/m0"),
            "missions/tutorial/m0"
        );
        assert_eq!(
            subcategory_from_root("extracted/art/missions/level02/m3"),
            "missions/level-02/m3"
        );
        assert_eq!(
            category_from_root("extracted/art/missions/l1m1"),
            PackageCategory::Missions
        );
        assert_eq!(
            category_from_root("extracted/art/cards"),
            PackageCategory::Cards
        );
        assert_eq!(
            category_from_root("extracted/music/sound/music"),
            PackageCategory::Music
        );
        assert_eq!(
            category_from_root("extracted/game/art/frontend/scrooby2/pages"),
            PackageCategory::UiScreens
        );
        assert_eq!(
            category_from_root("extracted/lmlm/CustomFiles/homer"),
            PackageCategory::Dialog
        );
        assert_eq!(
            category_from_root("extracted"),
            PackageCategory::ExtractionReports
        );
        assert_eq!(
            category_from_root("game"),
            PackageCategory::GameIcons
        );
        assert_eq!(
            category_from_root("extracted/art/frontend/dynaload/images/foo"),
            PackageCategory::UiImages
        );
        assert_eq!(
            category_from_root("extracted/art/frontend/scrooby2/resource/foo"),
            PackageCategory::UiResources
        );
        assert_eq!(
            category_from_root("extracted/art/frontend/dynaload/cars/homer_v"),
            PackageCategory::UiVehiclePreviews
        );
        assert_eq!(
            category_from_root("extracted/game/scripts/cars/Missions/level01"),
            PackageCategory::VehicleTuning
        );
        assert_eq!(
            category_from_root("extracted/game/scripts/missions/level01"),
            PackageCategory::MissionScripts
        );
        assert_eq!(
            category_from_root("extracted/scripts/sound/scripts"),
            PackageCategory::SoundScripts
        );
        assert_eq!(
            category_from_root("extracted/lmlm/CustomText"),
            PackageCategory::Language
        );
        assert_eq!(
            category_from_root("extracted/art/frontend/scrooby2/language"),
            PackageCategory::Language
        );
        assert_eq!(
            subcategory_from_root("extracted/art/cards"),
            "cards/pickup-effects"
        );
    }

    #[test]
    fn manifest_evidence_routes_mission_script_packages() -> Result<(), String>
    {
        let package = package_from_id(
            "extracted-game-scripts-missions-level03",
            vec![
                member_with_fields(
                    "script-l3m2",
                    MinorUnitRole::Script,
                    "script",
                    "mission-script",
                    "mfk",
                )?,
            ],
        );
        expect_package_classification(
            &package,
            PackageCategory::MissionScripts,
            "missions/level-03/scripts",
        )
    }

    #[test]
    fn manifest_evidence_routes_vehicle_tuning_packages() -> Result<(), String>
    {
        let package = package_from_id(
            "extracted-game-scripts-cars-missions-level04",
            vec![
                member_with_fields(
                    "vehicle-l4m1",
                    MinorUnitRole::Script,
                    "script",
                    "vehicle-tuning",
                    "con",
                )?,
            ],
        );
        expect_package_classification(
            &package,
            PackageCategory::VehicleTuning,
            "missions/level-04/vehicle-tuning/missions-level04",
        )
    }

    #[test]
    fn manifest_evidence_routes_tutorial_mission_art() -> Result<(), String> {
        let package = package_from_id(
            "extracted-art-missions-level01-m0",
            vec![
                member_with_fields(
                    "mission-model-l1m0",
                    MinorUnitRole::Model,
                    "model",
                    "p3d-model",
                    "mesh",
                )?,
            ],
        );
        expect_package_classification(
            &package,
            PackageCategory::Missions,
            "missions/tutorial/models/m0",
        )
    }

    #[test]
    fn manifest_evidence_routes_level08_to_head_to_head() -> Result<(), String>
    {
        let package = package_from_id(
            "extracted-art-missions-level08-chkpts",
            vec![
                member_with_fields(
                    "locator-head-to-head-checkpoint",
                    MinorUnitRole::Locator,
                    "locator",
                    "p3d-locator",
                    "srr_locator",
                )?,
            ],
        );
        expect_package_classification(
            &package,
            PackageCategory::Missions,
            "missions/head-to-head/assets/chkpts",
        )
    }

    #[test]
    fn manifest_evidence_routes_dialog_by_speaker_and_context()
    -> Result<(), String> {
        let package = package_from_id(
            // cspell:disable-next-line -- hom
            "extracted-dialog-conversations-hom-convinit-l1m7-accuse",
            vec![
                member_with_fields(
                    // cspell:disable-next-line -- hom
                    "dialog-l1m7-hom",
                    MinorUnitRole::Audio,
                    "audio",
                    "rsd-audio",
                    "wav-pcm",
                )?,
            ],
        );
        expect_package_classification(
            &package,
            PackageCategory::Dialog,
            "dialog/homer/conversation/mission/level-01/convinit/l1m7-accuse/\
             default",
        )
    }

    #[test]
    fn manifest_evidence_routes_unscoped_dialog_archives_to_free_roam()
    -> Result<(), String> {
        let package = package_from_id(
            "extracted-dialogf-dr-frink",
            vec![
                member_with_fields(
                    "audio-speaker-archive",
                    MinorUnitRole::Audio,
                    "audio",
                    "rsd-audio",
                    "wav-pcm",
                )?,
            ],
        );
        expect_package_classification(
            &package,
            PackageCategory::Dialog,
            "dialog/dr-frink/ad-lib/free-roam/french",
        )
    }

    #[test]
    fn mission_runtime_assets_use_concrete_scope() -> Result<(), String> {
        let package = package_from_id(
            "extracted-art-missions-generic-door",
            vec![
                member_with_fields(
                    "shared-mission-door",
                    MinorUnitRole::Animation,
                    "animation",
                    "p3d-animation",
                    "animation",
                )?,
            ],
        );
        expect_package_classification(
            &package,
            PackageCategory::Missions,
            "missions/runtime/animations/door",
        )
    }

    #[test]
    fn manifest_evidence_routes_mod_conversation_overrides()
    -> Result<(), String> {
        let package = package_from_id(
            "extracted-lmlm-customfiles-conversations",
            vec![
                member_with_fields(
                    "mod-conversation-audio",
                    MinorUnitRole::Audio,
                    "audio",
                    "audio-override",
                    "none",
                )?,
            ],
        );
        expect_package_classification(
            &package,
            PackageCategory::Dialog,
            "dialog/mod-conversations/mod-override/free-roam",
        )
    }

    #[test]
    fn manifest_evidence_routes_terrain_world_by_level_and_role()
    -> Result<(), String> {
        let terrain = package_from_id(
            "extracted-art-l04-fx",
            vec![
                member_with_fields(
                    "terrain-level-four-effects",
                    MinorUnitRole::World,
                    "world",
                    "p3d-mesh",
                    "mesh",
                )?,
            ],
        );
        expect_package_classification(
            &terrain,
            PackageCategory::TerrainWorld,
            "terrain-world/level-04/effects",
        )?;
        let interior = package_from_id(
            "extracted-art-l7i02",
            vec![
                member_with_fields(
                    "terrain-level-seven-interior",
                    MinorUnitRole::World,
                    "world",
                    "p3d-mesh",
                    "mesh",
                )?,
            ],
        );
        expect_package_classification(
            &interior,
            PackageCategory::TerrainWorld,
            "terrain-world/level-07/interiors/l7i02",
        )?;
        let bonus = package_from_id(
            "extracted-art-b02data",
            vec![
                member_with_fields(
                    "bonus-area-data",
                    MinorUnitRole::World,
                    "world",
                    "p3d-mesh",
                    "mesh",
                )?,
            ],
        );
        expect_package_classification(
            &bonus,
            PackageCategory::TerrainWorld,
            "terrain-world/bonus-area/data-records/b02data",
        )
    }

    #[test]
    fn manifest_evidence_routes_cars_by_vehicle_family() -> Result<(), String> {
        let character = package_from_id(
            "extracted-art-cars-homer-v",
            vec![
                member_with_fields(
                    "character-car-model",
                    MinorUnitRole::Model,
                    "model",
                    "p3d-mesh",
                    "mesh",
                )?,
            ],
        );
        expect_package_classification(
            &character,
            PackageCategory::Cars,
            "cars/character-rigs/homer-v",
        )?;
        let commercial = package_from_id(
            // cspell:disable-next-line -- ccola
            "extracted-art-cars-ccola",
            vec![
                member_with_fields(
                    "commercial-car-model",
                    MinorUnitRole::Model,
                    "model",
                    "p3d-mesh",
                    "mesh",
                )?,
            ],
        );
        expect_package_classification(
            &commercial,
            PackageCategory::Cars,
            // cspell:disable-next-line -- ccola
            "cars/commercial-vehicles/ccola",
        )?;
        let base = package_from_id(
            "extracted-art-cars-common",
            vec![
                member_with_fields(
                    "vehicle-runtime-base",
                    MinorUnitRole::Metadata,
                    "metadata",
                    "package-manifest",
                    "none",
                )?,
            ],
        );
        expect_package_classification(
            &base,
            PackageCategory::Cars,
            "cars/runtime-base/common",
        )
    }

    #[test]
    fn manifest_evidence_routes_frontend_vehicle_models_by_family()
    -> Result<(), String> {
        let character = package_from_id(
            "extracted-art-frontend-dynaload-cars-homer-v",
            vec![
                member_with_fields(
                    "frontend-character-vehicle-preview",
                    MinorUnitRole::Model,
                    "model",
                    "p3d-mesh",
                    "mesh",
                )?,
            ],
        );
        expect_package_classification(
            &character,
            PackageCategory::UiVehiclePreviews,
            "ui-vehicle-previews/character-rigs/homer-v",
        )?;
        let commercial = package_from_id(
            // cspell:disable-next-line -- ccola
            "extracted-art-frontend-dynaload-cars-ccola",
            vec![
                member_with_fields(
                    "frontend-commercial-vehicle-preview",
                    MinorUnitRole::Model,
                    "model",
                    "p3d-mesh",
                    "mesh",
                )?,
            ],
        );
        expect_package_classification(
            &commercial,
            PackageCategory::UiVehiclePreviews,
            // cspell:disable-next-line -- ccola
            "ui-vehicle-previews/commercial-vehicles/ccola",
        )?;

        let metadata = package_from_id(
            "extracted-game-art-frontend-dynaload-cars",
            vec![
                member_with_fields(
                    "frontend-vehicle-source-metadata",
                    MinorUnitRole::Metadata,
                    "metadata",
                    "none",
                    "none",
                )?,
            ],
        );
        expect_package_classification(
            &metadata,
            PackageCategory::UiVehiclePreviews,
            "ui-vehicle-previews/source-metadata",
        )
    }

    #[test]
    fn manifest_evidence_routes_media_and_screens() -> Result<(), String> {
        let music = package_from_id(
            "extracted-music02-sound-music-homer",
            vec![
                member_with_fields(
                    "homer-music-bank-entry",
                    MinorUnitRole::Audio,
                    "audio",
                    "runtime-asset",
                    "none",
                )?,
            ],
        );
        expect_package_classification(
            &music,
            PackageCategory::Music,
            "music/bank-02/character-homer",
        )?;
        let sound = package_from_id(
            "extracted-soundfx-sound-soundfx-interactive-props-spanish",
            vec![
                member_with_fields(
                    "localized-interactive-prop-sound",
                    MinorUnitRole::Audio,
                    "audio",
                    "runtime-asset",
                    "none",
                )?,
            ],
        );
        expect_package_classification(
            &sound,
            PackageCategory::SoundEffects,
            "sound-effects/effects/interactive-props/spanish",
        )?;
        let movie = package_from_id(
            "extracted-movies-fmv4",
            vec![
                member_with_fields(
                    "story-movie",
                    MinorUnitRole::Movie,
                    "movie",
                    "runtime-asset",
                    "none",
                )?,
            ],
        );
        expect_package_classification(
            &movie,
            PackageCategory::Movies,
            "movies/story/fmv4",
        )?;
        let screen = package_from_id(
            "extracted-art-frontend-scrooby-ingamel4",
            vec![
                member_with_fields(
                    "level-screen-layout",
                    MinorUnitRole::Ui,
                    "ui",
                    "p3d-scrooby-project",
                    "scrooby_project",
                )?,
            ],
        );
        expect_package_classification(
            &screen,
            PackageCategory::UiScreens,
            "ui-screens/sprite-layouts/in-game/level-04",
        )
    }

    #[test]
    fn manifest_evidence_routes_cinematics_by_scope_and_role()
    -> Result<(), String> {
        let level_gag = package_from_id(
            // cspell:disable-next-line -- azte
            "extracted-art-nis-gags-l04-azte",
            vec![
                member_with_fields(
                    "level-four-gag-scene",
                    MinorUnitRole::Scene,
                    "scene",
                    "p3d-scene",
                    "scene",
                )?,
            ],
        );
        expect_package_classification(
            &level_gag,
            PackageCategory::Cinematics,
            // cspell:disable-next-line -- azte
            "cinematics/gags/level-04/named/azte",
        )?;
        let numbered_gag = package_from_id(
            "extracted-art-nis-gags-gag0207",
            vec![
                member_with_fields(
                    "numbered-gag-scene",
                    MinorUnitRole::Scene,
                    "scene",
                    "p3d-scene",
                    "scene",
                )?,
            ],
        );
        expect_package_classification(
            &numbered_gag,
            PackageCategory::Cinematics,
            "cinematics/gags/series-02/numbered/gag0207",
        )?;
        let audio = package_from_id(
            "extracted-nis-sound-nis-spanish",
            vec![
                member_with_fields(
                    "localized-nis-audio",
                    MinorUnitRole::Audio,
                    "audio",
                    "rsd-audio",
                    "audio",
                )?,
            ],
        );
        expect_package_classification(
            &audio,
            PackageCategory::Cinematics,
            "cinematics/nis-audio/spanish",
        )
    }

    #[test]
    fn manifest_evidence_routes_ui_images_by_scope_and_role()
    -> Result<(), String> {
        let vehicle = package_from_id(
            "extracted-art-frontend-dynaload-images-cars2d-apu-vd",
            vec![
                member_with_fields(
                    "damaged-vehicle-preview",
                    MinorUnitRole::Texture,
                    "texture",
                    "png-image",
                    "image",
                )?,
            ],
        );
        expect_package_classification(
            &vehicle,
            PackageCategory::UiImages,
            "ui-images/vehicle-icons/damaged/apu",
        )?;
        let mission_icon = package_from_id(
            "extracted-art-frontend-dynaload-images-msnicons-location-house",
            vec![
                member_with_fields(
                    "mission-location-icon",
                    MinorUnitRole::Texture,
                    "texture",
                    "png-image",
                    "image",
                )?,
            ],
        );
        expect_package_classification(
            &mission_icon,
            PackageCategory::UiImages,
            "ui-images/mission-icons/locations/house",
        )?;

        let mission_metadata = package_from_id(
            "extracted-game-art-frontend-dynaload-images-msnicons",
            vec![
                member_with_fields(
                    "mission-icon-source-metadata",
                    MinorUnitRole::Metadata,
                    "metadata",
                    "none",
                    "none",
                )?,
            ],
        );
        expect_package_classification(
            &mission_metadata,
            PackageCategory::UiImages,
            "ui-images/mission-icons/source-metadata",
        )?;
        let metadata = package_from_id(
            "extracted-game-art-frontend-dynaload-images",
            vec![
                member_with_fields(
                    "unscoped-image-metadata",
                    MinorUnitRole::Metadata,
                    "metadata",
                    "none",
                    "none",
                )?,
            ],
        );
        expect_package_classification(
            &metadata,
            PackageCategory::UiImages,
            "ui-images/source-metadata",
        )
    }

    #[test]
    fn manifest_evidence_routes_ui_resources_by_scope_and_role()
    -> Result<(), String> {
        let card = package_from_id(
            "extracted-art-frontend-scrooby2-resource-frontend-card12",
            vec![
                member_with_fields(
                    "frontend-card-icon",
                    MinorUnitRole::Ui,
                    "ui",
                    "p3d-texture",
                    "texture",
                )?,
            ],
        );
        expect_package_classification(
            &card,
            PackageCategory::UiResources,
            "ui-resources/frontend/cards/card12",
        )?;
        let scene_resource = package_from_id(
            "extracted-art-frontend-scrooby-resource-pure3d-camset",
            vec![
                member_with_fields(
                    "frontend-scene-camera-set",
                    MinorUnitRole::Ui,
                    "ui",
                    "p3d-scene",
                    "scene",
                )?,
            ],
        );
        expect_package_classification(
            &scene_resource,
            PackageCategory::UiResources,
            "ui-resources/frontend-scenes/camera-sets/sprite-layouts/camset",
        )?;
        let loading = package_from_id(
            "extracted-art-frontend-scrooby2-resource-backend-loading0",
            vec![
                member_with_fields(
                    "backend-loading-icon",
                    MinorUnitRole::Ui,
                    "ui",
                    "p3d-texture",
                    "texture",
                )?,
            ],
        );
        expect_package_classification(
            &loading,
            PackageCategory::UiResources,
            "ui-resources/backend/loading/loading0",
        )
    }

    #[test]
    fn role_mapping_preserves_world_and_texture_ids() {
        assert_eq!(
            role_from_fields(
                "world",
                "p3d-road-network",
                "srr_road"
            ),
            MinorUnitRole::World
        );
        assert_eq!(
            role_from_fields(
                "image",
                "p3d-texture",
                "texture"
            ),
            MinorUnitRole::Texture
        );
    }

    #[test]
    fn coverage_rejects_uncataloged_manifest_id() -> Result<(), String> {
        let manifest_ids = id_set(
            &[
                "world-a",
                "texture-b",
            ],
        )?;
        let packages = vec![
            package(
                "extracted/art/L1_TERRA",
                &["world-a"],
            )?,
        ];
        let error = match validate_package_coverage(
            &manifest_ids,
            &packages,
        ) {
            Ok(()) => {
                return Err("missing package member should fail".to_owned());
            }
            Err(error) => error,
        };
        if !error
            .to_string()
            .contains("coverage mismatch")
        {
            return Err(format!("unexpected missing-id error: {error}"));
        }
        Ok(())
    }

    #[test]
    fn coverage_rejects_duplicate_index_id() -> Result<(), String> {
        let manifest_ids = id_set(&["world-a"])?;
        let packages = vec![
            package(
                "extracted/art/L1_TERRA",
                &["world-a"],
            )?,
            package(
                "extracted/art/L1_INTERIOR",
                &["world-a"],
            )?,
        ];
        let error = match validate_package_coverage(
            &manifest_ids,
            &packages,
        ) {
            Ok(()) => {
                return Err("duplicate package member should fail".to_owned());
            }
            Err(error) => error,
        };
        if !error
            .to_string()
            .contains("more than one package")
        {
            return Err(format!("unexpected duplicate-id error: {error}"));
        }
        Ok(())
    }

    #[test]
    fn coverage_accepts_exact_manifest_index_match() -> Result<(), String> {
        let manifest_ids = id_set(
            &[
                "world-a",
                "texture-b",
            ],
        )?;
        let packages = vec![
            package(
                "extracted/art/L1_TERRA",
                &[
                    "world-a",
                    "texture-b",
                ],
            )?,
        ];
        validate_package_coverage(
            &manifest_ids,
            &packages,
        )
        .map_err(|error| error.to_string())
    }

    fn expect_package_classification(
        package: &MinorUnitPackage,
        category: PackageCategory,
        subcategory: &str,
    ) -> Result<(), String> {
        if package.category != category {
            return Err(
                format!(
                    "expected category {:?}, got {:?}",
                    category, package.category
                ),
            );
        }
        if package.subcategory != subcategory {
            return Err(
                format!(
                    "expected subcategory {subcategory}, got {}",
                    package.subcategory
                ),
            );
        }
        Ok(())
    }

    fn id_set(values: &[&str]) -> Result<BTreeSet<MinorUnitId>, String> {
        let mut output = BTreeSet::new();
        for value in values {
            let id = minor_unit_id(value)?;
            let _inserted = output.insert(id);
        }
        Ok(output)
    }

    fn package_from_id(
        package_id: &str,
        members: Vec<PackageMember>,
    ) -> MinorUnitPackage {
        let mut package = MinorUnitPackage {
            package_id: PackageId(package_id.to_owned()),
            package_root: "manifest-evidence-group".to_owned(),
            category: PackageCategory::Error,
            subcategory: "error/unclassified".to_owned(),
            members,
            source_unit_ids: Vec::new(),
            text_keys: Vec::new(),
        };
        package.refresh_classification_from_members();
        package
    }

    fn package(
        root: &str,
        ids: &[&str],
    ) -> Result<MinorUnitPackage, String> {
        let mut package = MinorUnitPackage {
            package_id: PackageId::from_root(root),
            package_root: root.to_owned(),
            category: PackageCategory::Error,
            subcategory: "error/unclassified".to_owned(),
            members: ids
                .iter()
                .map(|id| member(id))
                .collect::<Result<Vec<_>, _>>()?,
            source_unit_ids: Vec::new(),
            text_keys: Vec::new(),
        };
        package.refresh_classification_from_members();
        Ok(package)
    }

    fn member(value: &str) -> Result<PackageMember, String> {
        member_with_fields(
            value,
            MinorUnitRole::World,
            "world",
            "p3d-world-dsg",
            "srr_fence_dsg",
        )
    }

    fn member_with_fields(
        value: &str,
        role: MinorUnitRole,
        type_: &str,
        kind: &str,
        source_chunk_kind: &str,
    ) -> Result<PackageMember, String> {
        Ok(
            PackageMember {
                id: minor_unit_id(value)?,
                role,
                path: format!(
                    "extracted/art/L1_TERRA/components/world/{value}.json"
                ),
                type_: type_.to_owned(),
                kind: kind.to_owned(),
                source_chunk_kind: source_chunk_kind.to_owned(),
            },
        )
    }

    fn minor_unit_id(value: &str) -> Result<MinorUnitId, String> {
        MinorUnitId::new(value.to_owned())
            .ok_or_else(|| "test id should be non-empty".to_owned())
    }
}
