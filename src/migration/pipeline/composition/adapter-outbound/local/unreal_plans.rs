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
//   - Projection from verified Unreal manifest records into canonical plans.
// - Must-Not:
//   - Read files, publish outputs, or contact Unreal Editor.
// - Allows:
//   - Manifest package policy, exact source hashes, and stable destinations.
// - Split-When:
//   - Split when another plan source gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns identical manifest-to-plan projection.
// - Summary:
//   - Unreal manifest plan projection.
// - Description:
//   - Builds import and construction operations without fabricating unsupported
//     world assembly or runtime binding operations.
// - Usage:
//   - Called after the canonical manifest text revision is known.
// - Defaults:
//   - FBX operations remain blocked until deterministic conversion publishes.
//

//! Unreal manifest plan projection.

use shar_sha256::domain::digest_hex;
use shar_unreal_conversion::domain::{
    ConversionPlan, NativeAssetFamily, OperationReadiness, PlanBundle,
    PlanContext, SourceFormat,
};

use crate::domain::package::unreal_manifest::{
    UnrealImportManifest, UnrealPackageRecord, UnrealSourceRecord, object_path,
};

const ENGINE_CONTRACT_REVISION: &str = "shar-unreal-porting-contract-v1";
const TARGET_ENGINE_VERSION: &str = "5.8";
const TARGET_PLATFORM: &str = "editor";

impl UnrealImportManifest {
    /// Build the six canonical plan families for this manifest.
    ///
    /// # Errors
    ///
    /// Returns an error when manifest records cannot form a collision-free,
    /// deterministic plan bundle.
    pub fn plan_bundle(
        &self,
        manifest_revision: &str,
    ) -> Result<PlanBundle, String> {
        let mut operations = Vec::new();
        for source in &self.sources {
            if let Some(operation) =
                direct_import_operation(source, &self.packages)?
            {
                operations.push(operation);
            }
        }
        for package in &self.packages {
            if let Some(operation) =
                package_operation(package, &self.sources, manifest_revision)?
            {
                operations.push(operation);
            }
        }
        PlanBundle::build(
            &PlanContext {
                source_manifest_revision: manifest_revision.to_owned(),
                engine_contract_revision: ENGINE_CONTRACT_REVISION.to_owned(),
                target_engine_version: TARGET_ENGINE_VERSION.to_owned(),
                target_platform: TARGET_PLATFORM.to_owned(),
            },
            operations,
        )
    }
}

fn direct_import_operation(
    source: &UnrealSourceRecord,
    packages: &[UnrealPackageRecord],
) -> Result<Option<ConversionPlan>, String> {
    let Some(import) = &source.direct_import else {
        return Ok(None);
    };
    let package = packages
        .iter()
        .find(|package| package.package_id == source.package_id)
        .ok_or_else(|| format!("source {} has no owning package", source.id))?;
    let (source_format, target_family) =
        match source.evidence.file_extension.as_str() {
            "png" => (SourceFormat::Image, NativeAssetFamily::Texture),
            "wav" => (SourceFormat::Wav, NativeAssetFamily::Audio),
            "mov" => (SourceFormat::Hap, NativeAssetFamily::Media),
            extension => {
                return Err(format!(
                    "unsupported direct import extension: {extension}"
                ));
            },
        };
    Ok(Some(ConversionPlan {
        package_identity: source.package_id.clone(),
        source_identity: source.id.clone(),
        source_format,
        target_family,
        source_path: source.evidence.path.clone(),
        source_revision: source.evidence.sha256.clone(),
        destination: import.object_path.clone(),
        target_class: import.target_class.to_owned(),
        importer: import.importer.to_owned(),
        import_profile: import.import_profile.to_owned(),
        dependencies: Vec::new(),
        readiness: OperationReadiness::Ready,
        world_owned: is_world_category(&package.category),
        runtime_bound: true,
    }))
}

fn package_operation(
    package: &UnrealPackageRecord,
    sources: &[UnrealSourceRecord],
    manifest_revision: &str,
) -> Result<Option<ConversionPlan>, String> {
    match package.disposition {
        "requires-fbx" => fbx_operation(package, sources).map(Some),
        "requires-editor-factory" => {
            Ok(Some(construction_operation(package, manifest_revision)))
        },
        "direct-editor-import" | "metadata-only" => Ok(None),
        disposition => {
            Err(format!("unsupported package disposition: {disposition}"))
        },
    }
}

fn fbx_operation(
    package: &UnrealPackageRecord,
    sources: &[UnrealSourceRecord],
) -> Result<ConversionPlan, String> {
    let source_path = package
        .expected_staged_files
        .first()
        .cloned()
        .ok_or_else(|| {
            format!("FBX package {} has no staged path", package.package_id)
        })?;
    Ok(ConversionPlan {
        package_identity: package.package_id.clone(),
        source_identity: format!("{}-fbx", package.package_id),
        source_format: SourceFormat::Fbx,
        target_family: NativeAssetFamily::Model,
        source_path,
        source_revision: aggregate_package_revision(
            &package.package_id,
            sources,
        )?,
        destination: object_path(&package.package_path, &package.asset_name),
        target_class: package.target_kind.to_owned(),
        importer: package.importer.to_owned(),
        import_profile: package.import_profile.to_owned(),
        dependencies: Vec::new(),
        readiness: OperationReadiness::RequiresConversion,
        world_owned: is_world_category(&package.category),
        runtime_bound: true,
    })
}

fn construction_operation(
    package: &UnrealPackageRecord,
    manifest_revision: &str,
) -> ConversionPlan {
    ConversionPlan {
        package_identity: package.package_id.clone(),
        source_identity: format!("{}-normalized-json", package.package_id),
        source_format: SourceFormat::Json,
        target_family: NativeAssetFamily::StructuredData,
        source_path: "manifest.jsonl".to_owned(),
        source_revision: manifest_revision.to_owned(),
        destination: object_path(&package.package_path, &package.asset_name),
        target_class: package.target_kind.to_owned(),
        importer: package.importer.to_owned(),
        import_profile: package.import_profile.to_owned(),
        dependencies: Vec::new(),
        readiness: OperationReadiness::RequiresEditorFactory,
        world_owned: is_world_category(&package.category),
        runtime_bound: true,
    }
}

fn aggregate_package_revision(
    package_id: &str,
    sources: &[UnrealSourceRecord],
) -> Result<String, String> {
    let mut members = sources
        .iter()
        .filter(|source| source.package_id == package_id)
        .collect::<Vec<_>>();
    if members.is_empty() {
        return Err(format!("package {package_id} has no source evidence"));
    }
    members.sort_by(|left, right| left.id.cmp(&right.id));
    let mut preimage = String::new();
    for source in members {
        preimage.push_str(&source.id);
        preimage.push('\n');
        preimage.push_str(&source.evidence.sha256);
        preimage.push('\n');
    }
    Ok(digest_hex(preimage.as_bytes()))
}

fn is_world_category(category: &str) -> bool {
    matches!(
        category,
        "terrain-world" | "props" | "cars" | "characters" | "missions"
    )
}
