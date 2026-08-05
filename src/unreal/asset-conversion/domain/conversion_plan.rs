// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT
//
// Boundary-Contract:
// - Owns:
//   - Deterministic Unreal conversion-plan values and validation.
// - Must-Not:
//   - Read files, contact Unreal Editor, or persist generated artifacts.
// - Allows:
//   - Verified normalized source evidence and stable Unreal destinations.
// - Split-When:
//   - Split when one plan family gains an independent schema lifecycle.
// - Merge-When:
//   - Merge when another module owns identical conversion planning.
// - Summary:
//   - Unreal conversion-plan domain contract.
// - Description:
//   - Builds the six canonical plan families required by the porting contract.
// - Usage:
//   - Built after source evidence and package identities are known.
// - Defaults:
//   - Unsafe paths, weak identities, collisions, and dependency drift fail.
//

//! Deterministic Unreal conversion-plan values and validation.

/// Schema shared by every canonical Unreal plan envelope.
pub const UNREAL_PLAN_SCHEMA: &str = "shar-schoenwald.unreal-plan.v1";
/// Schema for the compact plan-bundle index.
pub const UNREAL_PLAN_BUNDLE_SCHEMA: &str =
    "shar-schoenwald.unreal-plan-bundle.v1";

/// Normalized source representation accepted by Unreal conversion.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum SourceFormat {
    /// Structured normalized records.
    Json,
    /// Explicit decoded image interchange.
    Image,
    /// Pulse-code modulation audio.
    Wav,
    /// HAP-encoded video package evidence.
    Hap,
    /// Canonical binary FBX 7.7 model or animation evidence.
    Fbx,
}

impl SourceFormat {
    /// Return the canonical JSON token.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Json => "json",
            Self::Image => "image",
            Self::Wav => "wav",
            Self::Hap => "hap",
            Self::Fbx => "fbx",
        }
    }
}

/// Broad native Unreal target family.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum NativeAssetFamily {
    /// Structured native asset or table.
    StructuredData,
    /// Texture asset.
    Texture,
    /// Audio asset.
    Audio,
    /// Cinematic media asset.
    Media,
    /// Model, rig, animation, material, or camera asset.
    Model,
}

impl NativeAssetFamily {
    /// Return the canonical JSON token.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StructuredData => "structured-data",
            Self::Texture => "texture",
            Self::Audio => "audio",
            Self::Media => "media",
            Self::Model => "model",
        }
    }
}

/// Canonical plan family.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum PlanFamily {
    /// External-file import plan.
    AssetImport,
    /// Native-object construction plan.
    AssetConstruction,
    /// World actor and placement plan.
    WorldAssembly,
    /// Stable runtime-reference plan.
    RuntimeBinding,
    /// Generated-content validation plan.
    Validation,
    /// Cook and package plan.
    Package,
}

impl PlanFamily {
    /// Return every family in dependency order.
    #[must_use]
    pub const fn all() -> [Self; 6] {
        [
            Self::AssetImport,
            Self::AssetConstruction,
            Self::WorldAssembly,
            Self::RuntimeBinding,
            Self::Validation,
            Self::Package,
        ]
    }

    /// Return the canonical plan identity.
    #[must_use]
    pub const fn plan_id(self) -> &'static str {
        match self {
            Self::AssetImport => "asset-import-plan",
            Self::AssetConstruction => "asset-construction-plan",
            Self::WorldAssembly => "world-assembly-plan",
            Self::RuntimeBinding => "runtime-binding-plan",
            Self::Validation => "validation-plan",
            Self::Package => "package-plan",
        }
    }

    /// Return the generated filename.
    #[must_use]
    pub const fn filename(self) -> &'static str {
        match self {
            Self::AssetImport => "asset-import-plan.json",
            Self::AssetConstruction => "asset-construction-plan.json",
            Self::WorldAssembly => "world-assembly-plan.json",
            Self::RuntimeBinding => "runtime-binding-plan.json",
            Self::Validation => "validation-plan.json",
            Self::Package => "package-plan.json",
        }
    }

    /// Return direct plan dependency identities in lexical order.
    #[must_use]
    pub(crate) const fn dependency_ids(self) -> &'static [&'static str] {
        match self {
            Self::AssetImport => &[],
            Self::AssetConstruction => &["asset-import-plan"],
            Self::WorldAssembly => {
                &["asset-construction-plan", "asset-import-plan"]
            },
            Self::RuntimeBinding => &[
                "asset-construction-plan",
                "asset-import-plan",
                "world-assembly-plan",
            ],
            Self::Validation => &[
                "asset-construction-plan",
                "asset-import-plan",
                "runtime-binding-plan",
                "world-assembly-plan",
            ],
            Self::Package => &["validation-plan"],
        }
    }
}

/// Exact revision edge from one plan to a prerequisite plan.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanDependency {
    /// Stable prerequisite plan identity.
    pub plan_id: String,
    /// Exact lowercase SHA-256 revision of the prerequisite plan.
    pub revision: String,
}

/// Readiness of one editor operation.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum OperationReadiness {
    /// Referenced source bytes are already verified.
    Ready,
    /// An upstream deterministic conversion must publish the input.
    RequiresConversion,
    /// A repository-owned editor factory must construct the output.
    RequiresEditorFactory,
}

impl OperationReadiness {
    /// Return the canonical JSON token.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::RequiresConversion => "requires-conversion",
            Self::RequiresEditorFactory => "requires-editor-factory",
        }
    }
}

/// One deterministic source-to-Unreal operation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConversionPlan {
    /// Stable semantic package identity.
    pub package_identity: String,
    /// Stable source or aggregate evidence identity.
    pub source_identity: String,
    /// Accepted source format.
    pub source_format: SourceFormat,
    /// Native target family.
    pub target_family: NativeAssetFamily,
    /// Portable source or staging path.
    pub source_path: String,
    /// Lowercase SHA-256 source or aggregate revision.
    pub source_revision: String,
    /// Deterministic Unreal object path.
    pub destination: String,
    /// Expected Unreal class.
    pub target_class: String,
    /// Selected native importer or factory.
    pub importer: String,
    /// Versioned deterministic profile.
    pub import_profile: String,
    /// Sorted prerequisite operation identities.
    pub dependencies: Vec<String>,
    /// Source readiness for application.
    pub readiness: OperationReadiness,
    /// Whether placement evidence later owns this asset.
    pub world_owned: bool,
    /// Whether later runtime binding references this asset.
    pub runtime_bound: bool,
}

impl ConversionPlan {
    pub(crate) fn identity_preimage(&self) -> String {
        format!(
            "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
            self.package_identity,
            self.source_identity,
            self.source_format.as_str(),
            self.target_family.as_str(),
            self.source_path,
            self.source_revision,
            self.destination,
            self.importer,
            self.import_profile,
        )
    }

    pub(crate) const fn family(&self) -> PlanFamily {
        match self.source_format {
            SourceFormat::Json => PlanFamily::AssetConstruction,
            SourceFormat::Image
            | SourceFormat::Wav
            | SourceFormat::Hap
            | SourceFormat::Fbx => PlanFamily::AssetImport,
        }
    }

    pub(crate) fn validate(&self, own_identity: &str) -> Result<(), String> {
        validate_identity(&self.package_identity, "package identity")?;
        validate_identity(&self.source_identity, "source identity")?;
        validate_portable_path(&self.source_path)?;
        validate_sha256(&self.source_revision, "source revision")?;
        validate_destination(&self.destination)?;
        validate_identity(&self.target_class, "target class")?;
        validate_identity(&self.importer, "importer")?;
        validate_identity(&self.import_profile, "import profile")?;
        validate_dependencies(&self.dependencies, own_identity)
    }
}

/// Shared context for one complete plan bundle.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanContext {
    /// Revision of the canonical source import manifest.
    pub source_manifest_revision: String,
    /// Revision identity of the governing Unreal contracts.
    pub engine_contract_revision: String,
    /// Exact target Unreal Engine version.
    pub target_engine_version: String,
    /// Target platform or editor identity.
    pub target_platform: String,
}

impl PlanContext {
    pub(crate) fn validate(&self) -> Result<(), String> {
        validate_sha256(
            &self.source_manifest_revision,
            "source manifest revision",
        )?;
        validate_identity(
            &self.engine_contract_revision,
            "engine contract revision",
        )?;
        validate_identity(&self.target_engine_version, "engine version")?;
        validate_identity(&self.target_platform, "target platform")
    }
}

/// One rendered canonical plan artifact.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanArtifact {
    /// Plan family represented by the artifact.
    pub family: PlanFamily,
    /// Exact prerequisite plan identities and revisions.
    pub dependencies: Vec<PlanDependency>,
    /// SHA-256 revision of the canonical plan body.
    pub revision: String,
    /// Generated repository-relative filename.
    pub filename: String,
    /// Canonical UTF-8 JSON text ending in LF.
    pub json: String,
    /// Number of operations in the plan.
    pub operation_count: usize,
}

/// Complete deterministic six-plan bundle.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanBundle {
    pub(crate) artifacts: Vec<PlanArtifact>,
    pub(crate) index_revision: String,
    pub(crate) index_json: String,
}

impl PlanBundle {
    /// Return every plan artifact in dependency order.
    #[must_use]
    pub fn artifacts(&self) -> &[PlanArtifact] {
        &self.artifacts
    }

    /// Return the bundle-index revision.
    #[must_use]
    pub fn index_revision(&self) -> &str {
        &self.index_revision
    }

    /// Return the bundle-index JSON.
    #[must_use]
    pub fn index_json(&self) -> &str {
        &self.index_json
    }
}

fn validate_dependencies(
    dependencies: &[String],
    own_identity: &str,
) -> Result<(), String> {
    let mut previous: Option<&str> = None;
    for dependency in dependencies {
        validate_identity(dependency, "operation dependency")?;
        if dependency == own_identity {
            return Err("operation cannot depend on itself".to_owned());
        }
        if previous.is_some_and(|value| value >= dependency.as_str()) {
            return Err(
                "operation dependencies must be unique and sorted".to_owned()
            );
        }
        previous = Some(dependency);
    }
    Ok(())
}

fn validate_identity(value: &str, label: &str) -> Result<(), String> {
    if value.is_empty()
        || value.len() > 240
        || !value.is_ascii()
        || value.chars().any(char::is_control)
        || value.trim() != value
    {
        return Err(format!("invalid {label}: {value}"));
    }
    Ok(())
}

fn validate_portable_path(path: &str) -> Result<(), String> {
    if path.is_empty()
        || path.starts_with('/')
        || path.contains(char::from(92))
        || path.contains(':')
        || path
            .split('/')
            .any(|part| part.is_empty() || part == "." || part == "..")
    {
        return Err(format!("unsafe conversion source path: {path}"));
    }
    Ok(())
}

fn validate_sha256(value: &str, label: &str) -> Result<(), String> {
    if value.len() != 64
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
    {
        return Err(format!("invalid {label}: {value}"));
    }
    Ok(())
}

fn validate_destination(path: &str) -> Result<(), String> {
    if !path.starts_with("/Game/Generated/SHAR/")
        || path.len() > 240
        || path.contains("//")
        || path.chars().any(char::is_control)
    {
        return Err(format!("unsafe Unreal destination: {path}"));
    }
    let Some((package, object)) = path.rsplit_once('.') else {
        return Err(format!("Unreal destination has no object name: {path}"));
    };
    let Some((_parent, asset)) = package.rsplit_once('/') else {
        return Err(format!("Unreal destination has no package: {path}"));
    };
    if object != asset {
        return Err(format!(
            "Unreal destination object does not match package: {path}"
        ));
    }
    Ok(())
}

#[cfg(test)]
// jig-ignore-next-line: exact test-module path syntax is indivisible
#[path = "../../../../tests/unreal/asset-conversion/unit/domain/conversion_plan/tests.rs"]
mod tests;
