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
//   - Deterministic Unreal import-manifest planning.
// - Must-Not:
//   - Read files, execute Unreal Editor, or publish generated artifacts.
// - Allows:
//   - Validated package-index rows and exact source-content evidence.
//   - Versioned editor-facing manifest and summary values.
// - Split-When:
//   - Split when another manifest schema gains an independent lifecycle.
// - Merge-When:
//   - Merge when another domain module owns identical Unreal staging policy.
// - Summary:
//   - Unreal import-manifest domain contract.
// - Description:
//   - Converts semantic package plans and verified source evidence into one
//     collision-free editor-facing manifest.
// - Usage:
//   - Built after minor-unit audit and package indexing.
// - Defaults:
//   - Missing evidence, unsafe paths, and collisions fail closed.
//

//! Unreal import-manifest domain contract.

mod render;

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use super::{
    ConversionFamily, FbxTargetKind, PackageRole, PhaseThreePackageIndex,
    PhaseThreePackageMember, PhaseThreePackagePlanner, UnrealTargetKind,
};

/// Versioned JSONL manifest schema consumed by editor automation.
pub const UNREAL_IMPORT_MANIFEST_SCHEMA: &str =
    "shar-schoenwald.unreal-import-manifest.v2";
/// Versioned summary schema emitted beside the JSONL manifest.
pub const UNREAL_IMPORT_SUMMARY_SCHEMA: &str =
    "shar-schoenwald.unreal-import-summary.v2";
/// Maximum Unreal object path accepted by this staging contract.
const MAX_UNREAL_OBJECT_PATH_BYTES: usize = 240;

/// Verified source evidence supplied by the filesystem adapter.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnrealSourceEvidence {
    /// Stable minor-unit identity.
    pub id: String,
    /// Canonical repository-relative source path.
    pub path: String,
    /// File extension without a leading dot.
    pub file_extension: String,
    /// Controlled minor-unit type.
    pub unit_type: String,
    /// Controlled minor-unit subtype.
    pub subtype: String,
    /// Controlled minor-unit kind.
    pub kind: String,
    /// Human-readable source function.
    pub function: String,
    /// Source schema identity.
    pub schema: String,
    /// Extraction or conversion provenance.
    pub origin: String,
    /// Canonical provenance source path.
    pub source_path: String,
    /// Source chunk identity.
    pub source_chunk_kind: String,
    /// Exact source size.
    pub size_bytes: u64,
    /// Exact lowercase SHA-256 source digest.
    pub sha256: String,
    /// Declared Unreal import relationship.
    pub unreal_import_relation: String,
    /// Declared remaining normalization step.
    pub future_normalization: String,
}

/// Verified generated FBX artifact supplied by the filesystem adapter.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnrealFbxArtifactEvidence {
    /// Semantic package that owns the generated model artifact.
    pub package_id: String,
    /// Canonical repository-relative generated FBX path.
    pub path: String,
    /// Exact generated file size.
    pub size_bytes: u64,
    /// Exact lowercase SHA-256 digest of the generated FBX bytes.
    pub sha256: String,
    /// Binary FBX file version read from the file header.
    pub fbx_version: u32,
}

/// Verified generated UI-sprite raster supplied by the filesystem adapter.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnrealUiRasterArtifactEvidence {
    /// Semantic package that owns the generated raster artifact.
    pub package_id: String,
    /// Canonical repository-relative generated PNG path.
    pub path: String,
    /// Exact generated file size.
    pub size_bytes: u64,
    /// Exact lowercase SHA-256 digest of the generated PNG bytes.
    pub sha256: String,
    /// Revision binding normalized sprite metadata and ordered DDS children.
    pub source_revision: String,
    /// Verified raster width.
    pub width: u32,
    /// Verified raster height.
    pub height: u32,
    /// Number of normalized DDS tiles consumed by the compiler.
    pub tile_count: usize,
}

/// Complete deterministic Unreal import manifest.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnrealImportManifest {
    pub(crate) packages: Vec<UnrealPackageRecord>,
    pub(crate) sources: Vec<UnrealSourceRecord>,
    pub(crate) summary: UnrealImportSummary,
}

/// Aggregate import-manifest counters.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct UnrealImportSummary {
    pub(crate) packages: usize,
    pub(crate) sources: usize,
    pub(crate) direct_imports: usize,
    pub(crate) requires_fbx: usize,
    pub(crate) requires_editor_factory: usize,
    pub(crate) requires_semantic_conversion: usize,
    pub(crate) metadata_only: usize,
}

/// One semantic package plan for Unreal staging.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct UnrealPackageRecord {
    pub(crate) package_id: String,
    pub(crate) package_root: String,
    pub(crate) category: String,
    pub(crate) subcategory: String,
    pub(crate) conversion_family: &'static str,
    pub(crate) disposition: &'static str,
    pub(crate) target_kind: &'static str,
    pub(crate) importer: &'static str,
    pub(crate) import_profile: &'static str,
    pub(crate) package_path: String,
    pub(crate) asset_name: String,
    pub(crate) expected_staged_files: Vec<String>,
    pub(crate) expected_unreal_objects: Vec<String>,
    pub(crate) source_count: usize,
    pub(crate) source_unit_ids: Vec<String>,
    pub(crate) text_key_ids: Vec<String>,
    pub(crate) reason: Option<&'static str>,
}

/// One verified source member assigned to a semantic package.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct UnrealSourceRecord {
    pub(crate) package_id: String,
    pub(crate) id: String,
    pub(crate) role: PackageRole,
    pub(crate) evidence: UnrealSourceEvidence,
    pub(crate) direct_import: Option<DirectImportRecord>,
}

/// Direct editor import task for one already normalized source file.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct DirectImportRecord {
    pub(crate) importer: &'static str,
    pub(crate) import_profile: &'static str,
    pub(crate) target_class: &'static str,
    pub(crate) package_path: String,
    pub(crate) asset_name: String,
    pub(crate) object_path: String,
}

/// Stable policy selected for one package plan.
struct PackagePolicy {
    conversion_family: &'static str,
    disposition: &'static str,
    target_kind: &'static str,
    importer: &'static str,
    import_profile: &'static str,
    reason: Option<&'static str>,
}

impl UnrealImportManifest {
    /// Build one deterministic Unreal staging manifest.
    ///
    /// # Errors
    ///
    /// Returns an error when source evidence is missing, duplicated, stale,
    /// inconsistent with the package index, or maps to colliding Unreal paths.
    pub fn build(
        index: &PhaseThreePackageIndex,
        evidence: Vec<UnrealSourceEvidence>,
    ) -> Result<Self, String> {
        let mut evidence_by_id = BTreeMap::new();
        for source in evidence {
            validate_source_identity(&source.id)?;
            if evidence_by_id.insert(source.id.clone(), source).is_some() {
                return Err("duplicate Unreal source evidence id".to_owned());
            }
        }
        let mut packages = Vec::with_capacity(index.packages().len());
        let mut sources = Vec::new();
        let mut object_paths = BTreeSet::new();
        let mut staged_paths = BTreeSet::new();
        let mut summary = UnrealImportSummary::default();

        for package in index.packages() {
            let plan = PhaseThreePackagePlanner::plan(package);
            let mut policy = package_policy(
                plan.family,
                plan.fbx.as_ref().map(|model_plan| model_plan.target_kind),
                plan.unreal.as_ref().map(|unreal| unreal.target_kind),
            );
            let package_name = unreal_name(&package.package_id);
            let package_path = format!(
                "/Game/Generated/SHAR/{}/{}",
                unreal_name(&package.category),
                package_name,
            );
            validate_unreal_package_path(&package_path)?;
            let mut expected_staged_files = Vec::new();
            let mut expected_unreal_objects = Vec::new();
            let mut package_sources = Vec::new();

            for member in package.members() {
                let source =
                    evidence_by_id.remove(&member.id).ok_or_else(|| {
                        format!(
                            "package {} has no source evidence for {}",
                            package.package_id, member.id,
                        )
                    })?;
                validate_source_match(member, &source)?;
                let direct_import = direct_import_for_source(
                    &package_path,
                    member.role,
                    &source,
                )?;
                if let Some(import) = &direct_import {
                    claim_path(
                        &mut object_paths,
                        &import.object_path,
                        "Unreal object",
                    )?;
                    expected_unreal_objects.push(import.object_path.clone());
                    summary.direct_imports =
                        summary.direct_imports.saturating_add(1);
                }
                package_sources.push(UnrealSourceRecord {
                    package_id: package.package_id.clone(),
                    id: source.id.clone(),
                    role: member.role,
                    evidence: source,
                    direct_import,
                });
            }

            let has_direct_import = package_sources
                .iter()
                .any(|source| source.direct_import.is_some());
            resolve_effective_policy(
                plan.family,
                has_direct_import,
                &mut policy,
            );
            add_package_outputs(
                plan.family,
                policy.disposition,
                policy.target_kind,
                &package_name,
                &package_path,
                &mut expected_staged_files,
                &mut expected_unreal_objects,
                &mut staged_paths,
                &mut object_paths,
                &mut summary,
            )?;
            sources.extend(package_sources);
            packages.push(UnrealPackageRecord {
                package_id: package.package_id.clone(),
                package_root: package.package_root.clone(),
                category: package.category.clone(),
                subcategory: package.subcategory.clone(),
                conversion_family: policy.conversion_family,
                disposition: policy.disposition,
                target_kind: policy.target_kind,
                importer: policy.importer,
                import_profile: policy.import_profile,
                package_path,
                asset_name: package_name,
                expected_staged_files,
                expected_unreal_objects,
                source_count: package.members().len(),
                source_unit_ids: package.source_unit_ids.clone(),
                text_key_ids: package.text_key_ids.clone(),
                reason: policy.reason,
            });
        }

        if let Some((id, _source)) = evidence_by_id.first_key_value() {
            return Err(format!(
                "source evidence is not claimed by the package index: {id}"
            ));
        }
        packages.sort_by(|left, right| left.package_id.cmp(&right.package_id));
        sources.sort_by(|left, right| left.package_id.cmp(&right.package_id));
        summary.packages = packages.len();
        summary.sources = sources.len();
        Ok(Self {
            packages,
            sources,
            summary,
        })
    }

    /// Serialize the complete manifest as canonical JSONL.
    #[must_use]
    pub fn to_jsonl(&self) -> String {
        render::manifest_jsonl(self)
    }

    /// Serialize the compact machine-readable summary.
    #[must_use]
    pub fn summary_json(&self) -> String {
        render::summary_json(self)
    }

    /// Return the semantic package count.
    #[must_use]
    pub const fn package_count(&self) -> usize {
        self.summary.packages
    }

    /// Return the verified physical source count.
    #[must_use]
    pub const fn source_count(&self) -> usize {
        self.summary.sources
    }
}

fn resolve_effective_policy(
    family: ConversionFamily,
    has_direct_import: bool,
    policy: &mut PackagePolicy,
) {
    if family == ConversionFamily::UnrealNative
        && policy.disposition == "direct-editor-import"
        && !has_direct_import
    {
        policy.disposition = "requires-editor-factory";
        policy.reason = Some(
            "normalized package has no source compatible with direct import",
        );
    }
}

#[expect(
    clippy::too_many_arguments,
    reason = "One package transaction updates collision and summary evidence."
)]
fn add_package_outputs(
    family: ConversionFamily,
    disposition: &str,
    target_kind: &str,
    package_name: &str,
    package_path: &str,
    staged_files: &mut Vec<String>,
    unreal_objects: &mut Vec<String>,
    staged_paths: &mut BTreeSet<String>,
    object_paths: &mut BTreeSet<String>,
    summary: &mut UnrealImportSummary,
) -> Result<(), String> {
    match family {
        ConversionFamily::FbxModel if disposition == "requires-fbx" => {
            let staged = format!(
                "fbx-assets/packages/{package_name}/{package_name}.fbx"
            );
            claim_path(staged_paths, &staged, "staged file")?;
            staged_files.push(staged);
            let object = object_path(package_path, package_name);
            claim_path(object_paths, &object, "Unreal object")?;
            unreal_objects.push(object);
            match target_kind {
                "StaticMesh" => {},
                "SkeletalMesh" => {
                    let skeleton_name = format!("{package_name}_Skeleton");
                    let skeleton = object_path(package_path, &skeleton_name);
                    claim_path(object_paths, &skeleton, "Unreal object")?;
                    unreal_objects.push(skeleton);
                },
                _ => {
                    return Err(
                        "requires-fbx package has unsupported target kind"
                            .to_owned(),
                    );
                },
            }
            summary.requires_fbx = summary.requires_fbx.saturating_add(1);
        },
        ConversionFamily::FbxModel
            if disposition == "requires-semantic-conversion" =>
        {
            summary.requires_semantic_conversion =
                summary.requires_semantic_conversion.saturating_add(1);
        },
        ConversionFamily::FbxModel => {
            return Err("unsupported FBX package disposition".to_owned());
        },
        ConversionFamily::UnrealNative if disposition == "metadata-only" => {
            summary.metadata_only = summary.metadata_only.saturating_add(1);
        },
        ConversionFamily::UnrealNative
            if disposition == "requires-editor-factory" =>
        {
            let object = object_path(package_path, package_name);
            claim_path(object_paths, &object, "Unreal object")?;
            unreal_objects.push(object);
            summary.requires_editor_factory =
                summary.requires_editor_factory.saturating_add(1);
        },
        ConversionFamily::UnrealNative
            if disposition == "requires-semantic-conversion" =>
        {
            summary.requires_semantic_conversion =
                summary.requires_semantic_conversion.saturating_add(1);
        },
        ConversionFamily::UnrealNative => {},
        ConversionFamily::DoNotImport => {
            summary.metadata_only = summary.metadata_only.saturating_add(1);
        },
    }
    Ok(())
}

fn validate_source_identity(value: &str) -> Result<(), String> {
    let bytes = value.as_bytes();
    if bytes.is_empty()
        || !bytes.first().is_some_and(u8::is_ascii_alphanumeric)
        || !bytes.last().is_some_and(u8::is_ascii_alphanumeric)
        || bytes.windows(2).any(|pair| pair == b"--")
        || !bytes.iter().copied().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-'
        })
    {
        return Err("Unreal source evidence id is not canonical".to_owned());
    }
    Ok(())
}

fn validate_source_extension(value: &str) -> Result<(), String> {
    if value.is_empty()
        || value.len() > 16
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit())
    {
        return Err(
            "Unreal source evidence extension is not canonical".to_owned()
        );
    }
    Ok(())
}

fn validate_source_match(
    member: &PhaseThreePackageMember,
    source: &UnrealSourceEvidence,
) -> Result<(), String> {
    validate_portable_source_path(&source.path, "path")?;
    validate_portable_source_path(&source.source_path, "provenance path")?;
    if member.path != source.path
        || member.unit_type != source.unit_type
        || member.kind != source.kind
        || member.source_chunk_kind != source.source_chunk_kind
    {
        return Err(format!(
            "source evidence for {} disagrees with package-index member",
            member.id
        ));
    }
    validate_source_extension(&source.file_extension)?;
    let path_extension = Path::new(&source.path)
        .extension()
        .and_then(|value| value.to_str())
        .unwrap_or_default();
    if path_extension != source.file_extension {
        return Err(format!(
            "source {} file extension disagrees with its path",
            source.id
        ));
    }
    if source.sha256.len() != 64
        || !source
            .sha256
            .bytes()
            .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
    {
        return Err(format!("source {} has an invalid SHA-256", source.id));
    }
    Ok(())
}

fn validate_portable_source_path(
    path: &str,
    label: &str,
) -> Result<(), String> {
    if path.is_empty()
        || path.starts_with('/')
        || path.contains(char::from(92))
        || path.contains(':')
        || path.chars().any(char::is_control)
        || path
            .split('/')
            .any(|part| part.is_empty() || part == "." || part == "..")
    {
        return Err(format!("unsafe Unreal source {label}"));
    }
    Ok(())
}

fn package_policy(
    family: ConversionFamily,
    fbx_target: Option<FbxTargetKind>,
    target: Option<UnrealTargetKind>,
) -> PackagePolicy {
    match family {
        ConversionFamily::FbxModel => {
            fbx_policy(fbx_target.unwrap_or(FbxTargetKind::SemanticSplit))
        },
        ConversionFamily::DoNotImport => PackagePolicy {
            conversion_family: "do-not-import",
            disposition: "metadata-only",
            target_kind: "Metadata",
            importer: "none",
            import_profile: "traceability-only-v1",
            reason: Some("package contains traceability metadata only"),
        },
        ConversionFamily::UnrealNative => native_policy(target),
    }
}

/// Select direct FBX import only when one package maps to one mesh asset.
const fn fbx_policy(target: FbxTargetKind) -> PackagePolicy {
    match target {
        FbxTargetKind::StaticMesh => PackagePolicy {
            conversion_family: "fbx-model",
            disposition: "requires-fbx",
            target_kind: "StaticMesh",
            importer: "asset-tools-fbx",
            import_profile: "shar-fbx-static-v1",
            reason: Some(
                "single static-mesh package requires deterministic FBX export",
            ),
        },
        FbxTargetKind::SkeletalMesh => PackagePolicy {
            conversion_family: "fbx-model",
            disposition: "requires-fbx",
            target_kind: "SkeletalMesh",
            importer: "asset-tools-fbx",
            import_profile: "shar-fbx-skeletal-v1",
            reason: Some(
                concat!(
                    "single skeletal-mesh package requires companion-aware ",
                    "FBX import",
                ),
            ),
        },
        FbxTargetKind::SemanticSplit => PackagePolicy {
            conversion_family: "fbx-model",
            disposition: "requires-semantic-conversion",
            target_kind: "CompositeModel",
            importer: "semantic-converter",
            import_profile: "shar-fbx-semantic-split-v1",
            reason: Some(
                concat!(
                    "geometry package requires deterministic native asset ",
                    "splitting",
                ),
            ),
        },
    }
}

fn native_policy(target: Option<UnrealTargetKind>) -> PackagePolicy {
    let target = target.unwrap_or(UnrealTargetKind::SemanticSource);
    let (target_kind, importer, profile, direct) = match target {
        UnrealTargetKind::SemanticSource => (
            "SemanticSource",
            "semantic-converter",
            "shar-semantic-source-v1",
            false,
        ),
        UnrealTargetKind::DataTable => (
            "DataTable",
            "shar-data-table-factory",
            "shar-data-table-v1",
            false,
        ),
        UnrealTargetKind::StringTable => (
            "StringTable",
            "shar-string-table-factory",
            "shar-string-table-v1",
            false,
        ),
        UnrealTargetKind::Font => {
            ("Font", "shar-font-factory", "shar-font-v1", false)
        },
        UnrealTargetKind::Texture => {
            ("Texture2D", "texture-factory", "shar-texture-v1", true)
        },
        UnrealTargetKind::UserInterface => {
            ("WidgetBlueprint", "shar-ui-factory", "shar-ui-v1", false)
        },
        UnrealTargetKind::SoundWave => {
            ("SoundWave", "sound-wave-factory", "shar-audio-v1", true)
        },
        UnrealTargetKind::MediaSource => (
            "FileMediaSource",
            "media-source-movie",
            "shar-hap-movie-v1",
            true,
        ),
        UnrealTargetKind::StateTree => (
            "StateTree",
            "shar-state-tree-factory",
            "shar-state-tree-v1",
            false,
        ),
        UnrealTargetKind::NativeSubsystem => (
            "NativeSubsystem",
            "project-code",
            "shar-native-subsystem-v1",
            false,
        ),
        UnrealTargetKind::Metadata => {
            ("Metadata", "none", "traceability-only-v1", false)
        },
    };
    PackagePolicy {
        conversion_family: "unreal-native",
        disposition: if direct {
            "direct-editor-import"
        } else if target == UnrealTargetKind::Metadata {
            "metadata-only"
        } else if target == UnrealTargetKind::SemanticSource {
            "requires-semantic-conversion"
        } else {
            "requires-editor-factory"
        },
        target_kind,
        importer,
        import_profile: profile,
        reason: if direct {
            None
        } else if target == UnrealTargetKind::Metadata {
            Some("package contains traceability metadata only")
        } else if target == UnrealTargetKind::SemanticSource {
            Some("normalized source requires a deterministic semantic compiler")
        } else {
            Some("target requires a SHARBridge editor factory")
        },
    }
}

fn direct_import_for_source(
    package_path: &str,
    role: PackageRole,
    source: &UnrealSourceEvidence,
) -> Result<Option<DirectImportRecord>, String> {
    let policy = match (role, source.file_extension.as_str()) {
        (PackageRole::Texture, "png") => {
            Some(("texture-factory", "shar-texture-v1", "Texture2D"))
        },
        (PackageRole::Audio, "wav") => {
            Some(("sound-wave-factory", "shar-audio-v1", "SoundWave"))
        },
        (PackageRole::Movie, "mov") => {
            Some(("media-source-movie", "shar-hap-movie-v1", "FileMediaSource"))
        },
        _ => None,
    };
    let Some((importer, import_profile, target_class)) = policy else {
        return Ok(None);
    };
    let asset_name = unreal_name(&source.id);
    let object_path = object_path(package_path, &asset_name);
    validate_unreal_object_path(&object_path)?;
    Ok(Some(DirectImportRecord {
        importer,
        import_profile,
        target_class,
        package_path: package_path.to_owned(),
        asset_name,
        object_path,
    }))
}

fn unreal_name(value: &str) -> String {
    let mut output = String::with_capacity(value.len());
    let mut separator = false;
    for character in value.chars() {
        if character.is_ascii_alphanumeric() {
            if separator && !output.is_empty() {
                output.push('_');
            }
            output.push(character);
            separator = false;
        } else if !output.is_empty() {
            separator = true;
        }
    }
    if output.is_empty() {
        return "Asset".to_owned();
    }
    if output.as_bytes().first().is_some_and(u8::is_ascii_digit) {
        return format!("A_{output}");
    }
    output
}

pub(crate) fn object_path(package_path: &str, asset_name: &str) -> String {
    format!("{package_path}/{asset_name}.{asset_name}")
}

fn claim_path(
    paths: &mut BTreeSet<String>,
    path: &str,
    label: &str,
) -> Result<(), String> {
    if paths.insert(path.to_ascii_lowercase()) {
        Ok(())
    } else {
        Err(format!("case-insensitive {label} collision"))
    }
}

fn validate_unreal_package_path(path: &str) -> Result<(), String> {
    if !path.starts_with("/Game/Generated/SHAR/")
        || path.len() > MAX_UNREAL_OBJECT_PATH_BYTES
        || path.contains("//")
        || path.chars().any(char::is_control)
    {
        return Err("unsafe Unreal package path".to_owned());
    }
    for segment in path.split('/').filter(|segment| !segment.is_empty()) {
        if !segment
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
        {
            return Err("unsafe Unreal package segment".to_owned());
        }
    }
    Ok(())
}

fn validate_unreal_object_path(path: &str) -> Result<(), String> {
    if path.len() > MAX_UNREAL_OBJECT_PATH_BYTES {
        return Err("Unreal object path exceeds limit".to_owned());
    }
    let Some((package, object)) = path.rsplit_once('.') else {
        return Err("Unreal object path has no object name".to_owned());
    };
    let Some((_parent, asset)) = package.rsplit_once('/') else {
        return Err("Unreal object path has no package".to_owned());
    };
    validate_unreal_package_path(package)?;
    if object != asset {
        return Err("Unreal object name does not match package".to_owned());
    }
    Ok(())
}

#[cfg(test)]
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../../tests/migration/pipeline/unit/domain/package/unreal_manifest/tests.rs"]
mod tests;
