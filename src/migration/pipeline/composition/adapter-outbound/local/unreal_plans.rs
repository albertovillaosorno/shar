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

use std::collections::{BTreeMap, BTreeSet};

use shar_sha256::domain::digest_hex;
use shar_unreal_conversion::domain::{
    ConversionPlan, NativeAssetFamily, OperationReadiness, PlanBundle,
    PlanContext, SemanticBlockerClass, SourceFormat,
};

use crate::domain::package::unreal_manifest::{
    UnrealFbxArtifactEvidence, UnrealImportManifest, UnrealPackageRecord,
    UnrealSourceRecord, UnrealUiRasterArtifactEvidence, object_path,
};

const ENGINE_CONTRACT_REVISION: &str = "shar-unreal-porting-contract-v1";
const TARGET_ENGINE_VERSION: &str = "5.8.1";
const TARGET_PLATFORM: &str = "editor";

impl UnrealImportManifest {
    /// Build the six canonical plan families without generated FBX evidence.
    ///
    /// # Errors
    ///
    /// Returns an error when manifest records cannot form a collision-free,
    /// deterministic plan bundle.
    pub fn plan_bundle(
        &self,
        manifest_revision: &str,
    ) -> Result<PlanBundle, String> {
        self.build_plan_bundle(manifest_revision, None, None)
    }

    /// Build the six canonical plan families from one complete FBX catalog.
    ///
    /// # Errors
    ///
    /// Returns an error when the catalog is partial, duplicated, stale, unsafe,
    /// or does not correspond exactly to the manifest's model packages.
    pub fn plan_bundle_with_complete_fbx_catalog(
        &self,
        manifest_revision: &str,
        fbx_catalog: &[UnrealFbxArtifactEvidence],
    ) -> Result<PlanBundle, String> {
        self.build_plan_bundle(manifest_revision, Some(fbx_catalog), None)
    }

    /// Build canonical plans with complete generated model and UI catalogs.
    ///
    /// # Errors
    ///
    /// Returns an error when either supplied catalog is partial, duplicated,
    /// stale, unsafe, or cannot be claimed by its exact semantic package.
    pub(crate) fn plan_bundle_with_complete_generated_catalogs(
        &self,
        manifest_revision: &str,
        fbx_catalog: Option<&[UnrealFbxArtifactEvidence]>,
        ui_raster_catalog: &[UnrealUiRasterArtifactEvidence],
    ) -> Result<PlanBundle, String> {
        self.build_plan_bundle(
            manifest_revision,
            fbx_catalog,
            Some(ui_raster_catalog),
        )
    }

    fn build_plan_bundle(
        &self,
        manifest_revision: &str,
        fbx_catalog: Option<&[UnrealFbxArtifactEvidence]>,
        ui_raster_catalog: Option<&[UnrealUiRasterArtifactEvidence]>,
    ) -> Result<PlanBundle, String> {
        let require_complete_fbx = fbx_catalog.is_some();
        let mut fbx_by_package = BTreeMap::new();
        if let Some(entries) = fbx_catalog {
            for entry in entries {
                validate_fbx_evidence(entry)?;
                if fbx_by_package
                    .insert(entry.package_id.as_str(), entry)
                    .is_some()
                {
                    return Err(
                        "generated FBX catalog contains a duplicate package"
                            .to_owned(),
                    );
                }
            }
        }

        let require_complete_ui_raster = ui_raster_catalog.is_some();
        let mut ui_raster_by_package = BTreeMap::new();
        if let Some(entries) = ui_raster_catalog {
            for entry in entries {
                validate_ui_raster_evidence(entry)?;
                if ui_raster_by_package
                    .insert(entry.package_id.as_str(), entry)
                    .is_some()
                {
                    return Err(concat!(
                        "generated UI raster catalog contains a duplicate ",
                        "package",
                    )
                    .to_owned());
                }
            }
        }

        let packages_by_id = self
            .packages
            .iter()
            .map(|package| (package.package_id.as_str(), package))
            .collect::<BTreeMap<_, _>>();
        let ui_sprite_packages =
            exact_ui_sprite_packages(&self.packages, &self.sources);
        let mut operations = Vec::new();
        let mut generated = GeneratedCatalogState {
            fbx_by_package,
            require_complete_fbx,
            ui_raster_by_package,
            require_complete_ui_raster,
            ui_sprite_packages,
            promoted_ui_packages: BTreeSet::new(),
        };
        for source in &self.sources {
            let package = packages_by_id
                .get(source.package_id.as_str())
                .copied()
                .ok_or_else(|| {
                    format!("source {} has no owning package", source.id)
                })?;
            if let Some(operation) = direct_import_operation(source, package)? {
                operations.push(operation);
            }
        }
        for package in &self.packages {
            if let Some(operation) = package_operation(
                package,
                &self.sources,
                manifest_revision,
                &mut generated,
            )? {
                operations.push(operation);
            }
        }
        if !generated.fbx_by_package.is_empty() {
            return Err("generated FBX catalog contains an unclaimed package"
                .to_owned());
        }
        if !generated.ui_raster_by_package.is_empty() {
            return Err(
                "generated UI raster catalog contains an unclaimed package"
                    .to_owned(),
            );
        }
        let semantic_blockers = semantic_blocker_classes(
            &self.packages,
            &generated.promoted_ui_packages,
        )?;
        PlanBundle::build_with_semantic_blockers(
            &PlanContext {
                source_manifest_revision: manifest_revision.to_owned(),
                engine_contract_revision: ENGINE_CONTRACT_REVISION.to_owned(),
                target_engine_version: TARGET_ENGINE_VERSION.to_owned(),
                target_platform: TARGET_PLATFORM.to_owned(),
            },
            operations,
            semantic_blockers,
        )
    }
}

fn semantic_blocker_classes(
    packages: &[UnrealPackageRecord],
    promoted_ui_packages: &BTreeSet<String>,
) -> Result<Vec<SemanticBlockerClass>, String> {
    let mut counts = BTreeMap::<(String, String, String), usize>::new();
    for package in packages {
        if package.disposition != "requires-semantic-conversion"
            || promoted_ui_packages.contains(&package.package_id)
        {
            continue;
        }
        let key = (
            package.category.clone(),
            package.target_kind.to_owned(),
            package.import_profile.to_owned(),
        );
        let count = counts.entry(key).or_default();
        *count = count.checked_add(1).ok_or_else(|| {
            "semantic blocker class count overflowed".to_owned()
        })?;
    }
    Ok(counts
        .into_iter()
        .map(|((category, target_kind, import_profile), count)| {
            SemanticBlockerClass {
                category,
                target_kind,
                import_profile,
                count,
            }
        })
        .collect())
}

fn direct_import_operation(
    source: &UnrealSourceRecord,
    package: &UnrealPackageRecord,
) -> Result<Option<ConversionPlan>, String> {
    let Some(import) = &source.direct_import else {
        return Ok(None);
    };
    let (source_format, target_family) =
        match source.evidence.file_extension.as_str() {
            "png" => (SourceFormat::Image, NativeAssetFamily::Texture),
            "wav" => (SourceFormat::Wav, NativeAssetFamily::Audio),
            "mov" => (SourceFormat::Hap, NativeAssetFamily::Media),
            extension => {
                return Err(format!(
                    "unsupported direct import extension: {extension}"
                ));
            }
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

struct GeneratedCatalogState<'catalog> {
    fbx_by_package:
        BTreeMap<&'catalog str, &'catalog UnrealFbxArtifactEvidence>,
    require_complete_fbx: bool,
    ui_raster_by_package:
        BTreeMap<&'catalog str, &'catalog UnrealUiRasterArtifactEvidence>,
    require_complete_ui_raster: bool,
    ui_sprite_packages: BTreeSet<String>,
    promoted_ui_packages: BTreeSet<String>,
}

fn package_operation(
    package: &UnrealPackageRecord,
    sources: &[UnrealSourceRecord],
    manifest_revision: &str,
    generated: &mut GeneratedCatalogState<'_>,
) -> Result<Option<ConversionPlan>, String> {
    match package.disposition {
        "requires-fbx" => fbx_operation(
            package,
            sources,
            &mut generated.fbx_by_package,
            generated.require_complete_fbx,
        )
        .map(Some),
        "requires-editor-factory" => {
            Ok(Some(construction_operation(package, manifest_revision)))
        }
        "requires-semantic-conversion"
            if generated.ui_sprite_packages.contains(&package.package_id) =>
        {
            let verified = generated
                .ui_raster_by_package
                .remove(package.package_id.as_str());
            if generated.require_complete_ui_raster && verified.is_none() {
                return Err(concat!(
                    "complete generated UI raster catalog is missing a ",
                    "required package",
                )
                .to_owned());
            }
            verified.map_or(Ok(None), |evidence| {
                let _inserted = generated
                    .promoted_ui_packages
                    .insert(package.package_id.clone());
                Ok(Some(ui_raster_operation(package, evidence)))
            })
        }
        "direct-editor-import"
        | "metadata-only"
        | "requires-semantic-conversion" => Ok(None),
        disposition => {
            Err(format!("unsupported package disposition: {disposition}"))
        }
    }
}

fn exact_ui_sprite_packages(
    packages: &[UnrealPackageRecord],
    sources: &[UnrealSourceRecord],
) -> BTreeSet<String> {
    #[derive(Clone, Copy, Default)]
    struct Coverage {
        has_sprite: bool,
        has_image: bool,
        history_count: usize,
        requires_history: bool,
        has_other: bool,
    }

    let mut coverage = packages
        .iter()
        .filter(|package| {
            matches!(package.category.as_str(), "ui-images" | "ui-resources")
                && package.disposition == "requires-semantic-conversion"
        })
        .map(|package| {
            (
                package.package_id.as_str(),
                Coverage {
                    requires_history: package.category == "ui-resources",
                    ..Coverage::default()
                },
            )
        })
        .collect::<BTreeMap<_, _>>();
    for source in sources {
        let Some(state) = coverage.get_mut(source.package_id.as_str()) else {
            continue;
        };
        match source.evidence.source_chunk_kind.as_str() {
            "sprite" => state.has_sprite = true,
            "image" => state.has_image = true,
            "history" => {
                state.history_count = state.history_count.saturating_add(1)
            }
            _ if is_package_manifest_bookkeeping(source) => {}
            _ => state.has_other = true,
        }
    }
    coverage
        .into_iter()
        .filter(|(_package_id, state)| {
            let history_matches = if state.requires_history {
                state.history_count == 1
            } else {
                state.history_count == 0
            };
            state.has_sprite
                && state.has_image
                && history_matches
                && !state.has_other
        })
        .map(|(package_id, _state)| package_id.to_owned())
        .collect()
}

fn is_package_manifest_bookkeeping(source: &UnrealSourceRecord) -> bool {
    source.role.as_str() == "metadata"
        && source.evidence.unit_type == "metadata"
        && source.evidence.kind == "package-manifest"
        && source.evidence.source_chunk_kind == "none"
        && source.evidence.file_extension == "jsonl"
        && source.evidence.unreal_import_relation == "editor-only-metadata"
        && source.evidence.future_normalization == "keep"
        && source.evidence.path == source.evidence.source_path
}

fn ui_raster_operation(
    package: &UnrealPackageRecord,
    evidence: &UnrealUiRasterArtifactEvidence,
) -> ConversionPlan {
    ConversionPlan {
        package_identity: package.package_id.clone(),
        source_identity: format!("{}-ui-raster", package.package_id),
        source_format: SourceFormat::Image,
        target_family: NativeAssetFamily::Texture,
        source_path: evidence.path.clone(),
        source_revision: evidence.sha256.clone(),
        destination: object_path(&package.package_path, &package.asset_name),
        target_class: "Texture2D".to_owned(),
        importer: "texture-factory".to_owned(),
        import_profile: "shar-texture-v1".to_owned(),
        dependencies: Vec::new(),
        readiness: OperationReadiness::Ready,
        world_owned: false,
        runtime_bound: true,
    }
}

fn fbx_operation<'catalog>(
    package: &UnrealPackageRecord,
    sources: &[UnrealSourceRecord],
    fbx_by_package: &mut BTreeMap<
        &'catalog str,
        &'catalog UnrealFbxArtifactEvidence,
    >,
    require_complete_fbx: bool,
) -> Result<ConversionPlan, String> {
    let source_path = package
        .expected_staged_files
        .first()
        .cloned()
        .ok_or_else(|| {
            format!("FBX package {} has no staged path", package.package_id)
        })?;
    let verified = fbx_by_package.remove(package.package_id.as_str());
    if require_complete_fbx && verified.is_none() {
        return Err(
            "complete generated FBX catalog is missing a required package"
                .to_owned(),
        );
    }
    let (source_revision, readiness) = if let Some(evidence) = verified {
        if evidence.path != source_path {
            return Err(
                "generated FBX catalog path does not match its package"
                    .to_owned(),
            );
        }
        (evidence.sha256.clone(), OperationReadiness::Ready)
    } else {
        (
            aggregate_package_revision(&package.package_id, sources)?,
            OperationReadiness::RequiresConversion,
        )
    };
    Ok(ConversionPlan {
        package_identity: package.package_id.clone(),
        source_identity: format!("{}-fbx", package.package_id),
        source_format: SourceFormat::Fbx,
        target_family: NativeAssetFamily::Model,
        source_path,
        source_revision,
        destination: object_path(&package.package_path, &package.asset_name),
        target_class: package.target_kind.to_owned(),
        importer: package.importer.to_owned(),
        import_profile: package.import_profile.to_owned(),
        dependencies: Vec::new(),
        readiness,
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

fn validate_fbx_evidence(
    evidence: &UnrealFbxArtifactEvidence,
) -> Result<(), String> {
    let id = evidence.package_id.as_bytes();
    if id.is_empty()
        || !id.first().is_some_and(u8::is_ascii_alphanumeric)
        || !id.last().is_some_and(u8::is_ascii_alphanumeric)
        || id.windows(2).any(|pair| pair == b"--")
        || !id.iter().copied().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-'
        })
    {
        return Err(
            "generated FBX package identity is not canonical".to_owned()
        );
    }
    if evidence.path.is_empty()
        || evidence.path.starts_with('/')
        || evidence.path.contains(char::from(92))
        || evidence.path.contains(':')
        || evidence.path.chars().any(char::is_control)
        || evidence
            .path
            .split('/')
            .any(|part| part.is_empty() || part == "." || part == "..")
    {
        return Err("generated FBX catalog path is unsafe".to_owned());
    }
    if evidence.size_bytes < 27 {
        return Err("generated FBX artifact is too small".to_owned());
    }
    if evidence.sha256.len() != 64
        || !evidence
            .sha256
            .bytes()
            .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
    {
        return Err("generated FBX digest is not canonical".to_owned());
    }
    if evidence.fbx_version != 7700 {
        return Err("generated FBX version is not supported".to_owned());
    }
    Ok(())
}

fn validate_ui_raster_evidence(
    evidence: &UnrealUiRasterArtifactEvidence,
) -> Result<(), String> {
    let id = evidence.package_id.as_bytes();
    if id.is_empty()
        || !id.first().is_some_and(u8::is_ascii_alphanumeric)
        || !id.last().is_some_and(u8::is_ascii_alphanumeric)
        || id.windows(2).any(|pair| pair == b"--")
        || !id.iter().copied().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-'
        })
    {
        return Err(
            "generated UI raster package identity is not canonical".to_owned()
        );
    }
    let expected_path =
        format!("ui-raster-assets/rasters/{}.png", evidence.package_id);
    if evidence.path != expected_path {
        return Err(
            "generated UI raster path does not match its package identity"
                .to_owned(),
        );
    }
    if evidence.size_bytes < 8 {
        return Err("generated UI raster artifact is too small".to_owned());
    }
    for (label, digest) in [
        ("digest", evidence.sha256.as_str()),
        ("source revision", evidence.source_revision.as_str()),
    ] {
        if digest.len() != 64
            || !digest.bytes().all(|byte| {
                byte.is_ascii_digit() || matches!(byte, b'a'..=b'f')
            })
        {
            return Err(format!(
                "generated UI raster {label} is not canonical"
            ));
        }
    }
    if evidence.width == 0 || evidence.height == 0 || evidence.tile_count == 0 {
        return Err(
            "generated UI raster dimensions and tile count must be positive"
                .to_owned(),
        );
    }
    Ok(())
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
