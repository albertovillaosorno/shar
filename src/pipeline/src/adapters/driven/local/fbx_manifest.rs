// File:
//   - fbx_manifest.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/fbx_manifest.rs
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
//   - The fbx contract for pipeline phase three.
// - Must-Not:
//   - Violate repository architecture, path, provenance, or output rules.
// - Allows:
//   - Operations required to validate and execute fbx.
// - Split-When:
//   - Split when fbx contains two independently testable contracts.
// - Merge-When:
//   - Another pipeline module owns the same module boundary with no distinct
//   - invariant.
// - Summary:
//   - Phase-three FBX adapter planning.
// - Description:
//   - Defines fbx data and behavior for pipeline phase three.
// - Usage:
//   - Used by pipeline phase three code that needs fbx.
// - Defaults:
//   - No implicit output outside the repository is allowed.
//
// ADRs:
// - docs/adr/pipeline/minor-unit-taxonomy-value-case.md
//
// Large file:
//   - true
//   - Reason: Phase-three FBX adapter planning keeps tightly coupled
//   - validation, ordering, and deterministic transformation invariants
//   - together; split when a stable independently testable sub-boundary is
//   - identified.
//

//! Phase-three FBX adapter planning.
//! Phase-three FBX adapter planning.

use std::path::{Path, PathBuf};

use schoenwald_filesystem::adapters::driving::local::{
    file_len as local_file_len, write_text as local_write_text,
};

use crate::domain::package::{
    ConversionFamily, FbxModelPlan, PhaseThreePackageIndex,
    PhaseThreePackagePlanner, PhaseThreePackageSelector,
};
use crate::domain::{PipelineError, StageReport, escape_json};

/// Deterministic FBX adapter manifest for one model-like package.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct PhaseThreeFbxManifest {
    /// Package id selected from the generated package index.
    pub package_id: String,
    /// Stable package subcategory.
    pub subcategory: String,
    /// Output FBX file name requested from the later FBX engine.
    pub output_fbx: String,
    /// Model ids needed by the generic FBX engine.
    pub model_ids: Vec<String>,
    /// World ids needed by the generic FBX engine.
    pub world_ids: Vec<String>,
    /// Scene ids needed to reconstruct the package hierarchy.
    pub scene_ids: Vec<String>,
    /// Locator ids needed to preserve authored attachment positions.
    pub locator_ids: Vec<String>,
    /// Camera ids needed to preserve authored viewpoints.
    pub camera_ids: Vec<String>,
    /// Animation ids that should travel with this package.
    pub animation_ids: Vec<String>,
    /// Texture ids referenced by the model-like package.
    pub texture_ids: Vec<String>,
    /// Material ids referenced by the model-like package.
    pub material_ids: Vec<String>,
    /// Physics ids retained for later native Unreal splitting.
    pub physics_ids: Vec<String>,
}

impl PhaseThreeFbxManifest {
    /// Create a deterministic manifest from a generic model plan.
    #[must_use]
    pub(super) fn from_plan(plan: &FbxModelPlan) -> Self {
        Self {
            package_id: plan
                .package_id
                .clone(),
            subcategory: plan
                .subcategory
                .clone(),
            output_fbx: format!(
                "{}.fbx",
                stable_file_stem(&plan.subcategory)
            ),
            model_ids: plan
                .model_ids
                .clone(),
            world_ids: plan
                .world_ids
                .clone(),
            scene_ids: plan
                .scene_ids
                .clone(),
            locator_ids: plan
                .locator_ids
                .clone(),
            camera_ids: plan
                .camera_ids
                .clone(),
            animation_ids: plan
                .animation_ids
                .clone(),
            texture_ids: plan
                .texture_ids
                .clone(),
            material_ids: plan
                .material_ids
                .clone(),
            physics_ids: plan
                .physics_ids
                .clone(),
        }
    }

    /// Render deterministic JSON without adding a registry dependency.
    #[must_use]
    pub(super) fn to_json(&self) -> String {
        let mut json = String::new();
        json.push_str("{\n");
        push_string_field(
            &mut json,
            "package_id",
            &self.package_id,
            true,
        );
        push_string_field(
            &mut json,
            "subcategory",
            &self.subcategory,
            true,
        );
        push_string_field(
            &mut json,
            "conversion_family",
            "fbx-model",
            true,
        );
        push_string_field(
            &mut json,
            "output_fbx",
            &self.output_fbx,
            true,
        );
        push_array_field(
            &mut json,
            "model_ids",
            &self.model_ids,
            true,
        );
        push_array_field(
            &mut json,
            "world_ids",
            &self.world_ids,
            true,
        );
        push_array_field(
            &mut json,
            "scene_ids",
            &self.scene_ids,
            true,
        );
        push_array_field(
            &mut json,
            "locator_ids",
            &self.locator_ids,
            true,
        );
        push_array_field(
            &mut json,
            "camera_ids",
            &self.camera_ids,
            true,
        );
        push_array_field(
            &mut json,
            "animation_ids",
            &self.animation_ids,
            true,
        );
        push_array_field(
            &mut json,
            "texture_ids",
            &self.texture_ids,
            true,
        );
        push_array_field(
            &mut json,
            "material_ids",
            &self.material_ids,
            true,
        );
        push_array_field(
            &mut json,
            "physics_ids",
            &self.physics_ids,
            false,
        );
        json.push_str("}\n");
        json
    }

    /// Write this manifest to a package output directory.
    ///
    /// # Errors
    ///
    /// Returns an error when the output directory or manifest file cannot be
    /// written.
    pub(super) fn write_to(
        &self,
        output_dir: &Path,
    ) -> Result<PathBuf, PipelineError> {
        let path = output_dir.join("fbx-manifest.json");
        local_write_text(
            &path,
            &self.to_json(),
            true,
        )
        .map_err(
            |error| {
                PipelineError::new(
                    format!("failed to write FBX manifest: {error}"),
                )
            },
        )?;
        Ok(path)
    }
}

/// Build and write the first phase-three FBX manifest for a selected package.
///
/// # Errors
///
/// Returns an error when the index cannot be read, the selector does not
/// resolve to one package, or the selected package is not model-like.
pub(super) fn write_phase_three_fbx_manifest(
    index_path: &Path,
    selector: &PhaseThreePackageSelector,
    output_dir: &Path,
) -> Result<StageReport, PipelineError> {
    let index = PhaseThreePackageIndex::read(index_path)
        .map_err(|error| PipelineError::new(error.to_string()))?;
    let package = selector
        .resolve(&index)
        .map_err(|error| PipelineError::new(error.to_string()))?;
    let plan = PhaseThreePackagePlanner::plan(package);
    if plan.family != ConversionFamily::FbxModel {
        return Err(
            PipelineError::new(
                format!(
                    "selected package is not an FBX model package: {}",
                    package.package_id
                ),
            ),
        );
    }
    let Some(fbx_plan) = plan.fbx else {
        return Err(
            PipelineError::new("missing FBX plan for selected package"),
        );
    };
    let manifest = PhaseThreeFbxManifest::from_plan(&fbx_plan);
    let path = manifest.write_to(output_dir)?;
    Ok(
        StageReport {
            name: "phase-three-fbx-manifest",
            files: 1,
            bytes: file_len(&path)?,
            note: format!(
                "package={} subcategory={} output={}",
                manifest.package_id,
                manifest.subcategory,
                path.display()
            ),
        },
    )
}

/// Supports the `file_len` operation within this deterministic classification
/// boundary.
fn file_len(path: &Path) -> Result<u64, PipelineError> {
    local_file_len(path).map_err(
        |error| PipelineError::new(format!("failed to stat manifest: {error}")),
    )
}

/// Supports the `stable_file_stem` operation within this deterministic
/// classification boundary.
pub(super) fn stable_file_stem(subcategory: &str) -> String {
    subcategory
        .chars()
        .map(
            |character| match character {
                'a'..='z' | '0'..='9' => character,
                'A'..='Z' => character.to_ascii_lowercase(),
                _ => '-',
            },
        )
        .collect::<String>()
        .trim_matches('-')
        .to_owned()
}

/// Supports the `push_string_field` operation within this deterministic
/// classification boundary.
fn push_string_field(
    json: &mut String,
    field: &str,
    value: &str,
    trailing_comma: bool,
) {
    json.push_str("  \"");
    json.push_str(field);
    json.push_str("\": \"");
    json.push_str(&escape_json(value));
    json.push('"');
    if trailing_comma {
        json.push(',');
    }
    json.push('\n');
}

/// Supports the `push_array_field` operation within this deterministic
/// classification boundary.
fn push_array_field(
    json: &mut String,
    field: &str,
    values: &[String],
    trailing_comma: bool,
) {
    json.push_str("  \"");
    json.push_str(field);
    json.push_str("\": [");
    for (index, value) in values
        .iter()
        .enumerate()
    {
        if index > 0 {
            json.push_str(", ");
        }
        json.push('"');
        json.push_str(&escape_json(value));
        json.push('"');
    }
    json.push(']');
    if trailing_comma {
        json.push(',');
    }
    json.push('\n');
}

#[cfg(test)]
mod tests {
    use super::PhaseThreeFbxManifest;
    use crate::domain::package::FbxModelPlan;

    #[test]
    fn renders_generic_fbx_manifest() -> Result<(), String> {
        let manifest = PhaseThreeFbxManifest::from_plan(
            &FbxModelPlan {
                package_id: "pkg".to_owned(),
                subcategory: "props/wrench".to_owned(),
                model_ids: vec!["model-a".to_owned()],
                world_ids: Vec::new(),
                scene_ids: Vec::new(),
                locator_ids: Vec::new(),
                camera_ids: Vec::new(),
                animation_ids: vec!["anim-a".to_owned()],
                texture_ids: vec!["texture-a".to_owned()],
                material_ids: vec!["material-a".to_owned()],
                physics_ids: Vec::new(),
            },
        );
        let json = manifest.to_json();
        if !json.contains("\"output_fbx\": \"props-wrench.fbx\"") {
            return Err("manifest should expose stable FBX name".to_owned());
        }
        if !json.contains("\"model_ids\": [\"model-a\"]") {
            return Err("manifest should include model ids".to_owned());
        }
        Ok(())
    }
}
