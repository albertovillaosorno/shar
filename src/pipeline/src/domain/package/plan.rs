// File:
//   - plan.rs
// Path:
//   - src/pipeline/src/domain/package/plan.rs
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
//   - The plan contract for pipeline phase three package.
// - Must-Not:
//   - Violate repository architecture, path, provenance, or output rules.
// - Allows:
//   - Operations required to validate and execute plan.
// - Split-When:
//   - Split when plan contains two independently testable contracts.
// - Merge-When:
//   - Another pipeline module owns the same module boundary with no distinct
//   - invariant.
// - Summary:
//   - Phase-three package conversion planner.
// - Description:
//   - Defines plan data and behavior for pipeline phase three package.
// - Usage:
//   - Used by pipeline phase three package code that needs plan.
// - Defaults:
//   - No implicit output outside the repository is allowed.
//
// ADRs:
// - docs/adr/pipeline/minor-unit-taxonomy-value-case.md
//
// Large file:
//   - true
//   - Reason: Phase-three package conversion planner keeps tightly coupled
//   - validation, ordering, and deterministic transformation invariants
//   - together; split when a stable independently testable sub-boundary is
//   - identified.
//

//! Phase-three package conversion planner.
//! Phase-three package conversion planner.

use super::index::{PackageRole, PhaseThreePackageRow};

/// High-level conversion family selected for a package.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConversionFamily {
    /// Model-like package that should produce clean FBX before Unreal import.
    FbxModel,
    /// Non-model data that should become an Unreal-native asset or table.
    UnrealNative,
    /// Runtime or source metadata that should not produce an imported asset.
    DoNotImport,
}

/// Unreal-native target kind for non-model packages.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UnrealTargetKind {
    /// `DataAsset` target.
    DataAsset,
    /// `DataTable` target.
    DataTable,
    /// `StringTable` target.
    StringTable,
    /// `Texture2D` or UI texture target.
    Texture,
    /// `UMG` or screen layout target.
    UserInterface,
    /// `SoundWave` target.
    SoundWave,
    /// `MediaSource` target.
    MediaSource,
    /// `StateTree` or mission-flow target.
    StateTree,
    /// Native subsystem or project-owned runtime code target.
    NativeSubsystem,
    /// Metadata-only target that should be retained for traceability.
    Metadata,
}

/// `FBX` planning output for model-like packages.
// The suffix keeps this value distinct from the adapter that executes the plan
// and from the package row that provides its validated source identifiers.
#[expect(
    clippy::module_name_repetitions,
    reason = "Public names preserve distinct FBX planning boundaries for \
              callers."
)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FbxModelPlan {
    /// Stable `FBX` package id.
    pub package_id: String,
    /// Stable package subcategory used to derive output identity.
    pub subcategory: String,
    /// Model ids to hand to the FBX adapter.
    pub model_ids: Vec<String>,
    /// World ids to hand to terrain/world FBX adapters.
    pub world_ids: Vec<String>,
    /// Scene ids required to assemble the exported model hierarchy.
    pub scene_ids: Vec<String>,
    /// Locator ids required to preserve attachment and trigger positions.
    pub locator_ids: Vec<String>,
    /// Camera ids required to preserve package-authored viewpoints.
    pub camera_ids: Vec<String>,
    /// Animation ids that must stay attached to the model package.
    pub animation_ids: Vec<String>,
    /// Texture ids referenced by the model package.
    pub texture_ids: Vec<String>,
    /// Material ids referenced by the model package.
    pub material_ids: Vec<String>,
    /// Physics ids that should be preserved for Unreal-native asset splitting.
    pub physics_ids: Vec<String>,
}

/// Unreal-native planning output for non-model packages.
// The suffix identifies immutable planning data rather than an imported Unreal
// object or the adapter that later materializes the selected target kind.
#[expect(
    clippy::module_name_repetitions,
    reason = "Public names preserve distinct Unreal planning boundaries for \
              callers."
)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnrealNativePlan {
    /// Stable Unreal package id.
    pub package_id: String,
    /// Stable package subcategory used to derive Unreal object identity.
    pub subcategory: String,
    /// Target Unreal asset kind.
    pub target_kind: UnrealTargetKind,
    /// Ids that the Unreal adapter must consume for this target.
    pub input_ids: Vec<String>,
}

/// Phase-three conversion plan for one package.
// The phase-qualified name prevents consumers from confusing this conversion
// contract with extraction-stage plans that carry different invariants.
#[expect(
    clippy::module_name_repetitions,
    reason = "Public names preserve distinct phase-three planning boundaries \
              for callers."
)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PhaseThreePackagePlan {
    /// Stable package id.
    pub package_id: String,
    /// High-level conversion family.
    pub family: ConversionFamily,
    /// Optional FBX plan.
    pub fbx: Option<FbxModelPlan>,
    /// Optional Unreal-native plan.
    pub unreal: Option<UnrealNativePlan>,
}

/// Stateless package planner.
#[derive(Clone, Copy, Debug, Default)]
pub struct PhaseThreePackagePlanner;

impl PhaseThreePackagePlanner {
    /// Build a phase-three conversion plan for one exact package row.
    #[must_use]
    pub fn plan(package: &PhaseThreePackageRow) -> PhaseThreePackagePlan {
        if package.has_error_ids() || metadata_only_package(package) {
            return metadata_plan(package);
        }
        if package.has_model_components() && fbx_category(&package.category) {
            return fbx_plan(package);
        }
        unreal_plan(package)
    }
}

/// Identifies categories whose model payload belongs at the FBX boundary.
fn fbx_category(category: &str) -> bool {
    matches!(
        category,
        "cars"
            | "characters"
            | "terrain-world"
            | "missions"
            | "props"
            | "cinematics"
            | "ui-vehicle-previews"
            | "ui-resources"
    )
}

/// Detects packages whose ids are retained only for traceability metadata.
fn metadata_only_package(package: &PhaseThreePackageRow) -> bool {
    package
        .unit_ids
        .len()
        == package
            .ids_for_role(PackageRole::Metadata)
            .len()
        && package
            .text_key_ids
            .is_empty()
}

/// Builds a non-importing plan that preserves metadata identifiers.
fn metadata_plan(package: &PhaseThreePackageRow) -> PhaseThreePackagePlan {
    PhaseThreePackagePlan {
        package_id: package
            .package_id
            .clone(),
        family: ConversionFamily::DoNotImport,
        fbx: None,
        unreal: Some(
            UnrealNativePlan {
                package_id: package
                    .package_id
                    .clone(),
                subcategory: package
                    .subcategory
                    .clone(),
                target_kind: UnrealTargetKind::Metadata,
                input_ids: package
                    .unit_ids
                    .clone(),
            },
        ),
    }
}

/// Builds an FBX plan while keeping non-model companion ids attached.
fn fbx_plan(package: &PhaseThreePackageRow) -> PhaseThreePackagePlan {
    PhaseThreePackagePlan {
        package_id: package
            .package_id
            .clone(),
        family: ConversionFamily::FbxModel,
        fbx: Some(
            FbxModelPlan {
                package_id: package
                    .package_id
                    .clone(),
                subcategory: package
                    .subcategory
                    .clone(),
                model_ids: package
                    .ids_for_role(PackageRole::Model)
                    .to_vec(),
                world_ids: package
                    .ids_for_role(PackageRole::World)
                    .to_vec(),
                scene_ids: package
                    .ids_for_role(PackageRole::Scene)
                    .to_vec(),
                locator_ids: package
                    .ids_for_role(PackageRole::Locator)
                    .to_vec(),
                camera_ids: package
                    .ids_for_role(PackageRole::Camera)
                    .to_vec(),
                animation_ids: package
                    .ids_for_role(PackageRole::Animation)
                    .to_vec(),
                texture_ids: package
                    .ids_for_role(PackageRole::Texture)
                    .to_vec(),
                material_ids: package
                    .ids_for_role(PackageRole::Material)
                    .to_vec(),
                physics_ids: package
                    .ids_for_role(PackageRole::Physics)
                    .to_vec(),
            },
        ),
        unreal: None,
    }
}

/// Builds an Unreal-native plan for packages outside the FBX boundary.
fn unreal_plan(package: &PhaseThreePackageRow) -> PhaseThreePackagePlan {
    let target_kind = unreal_target_kind(package);
    PhaseThreePackagePlan {
        package_id: package
            .package_id
            .clone(),
        family: ConversionFamily::UnrealNative,
        fbx: None,
        unreal: Some(
            UnrealNativePlan {
                package_id: package
                    .package_id
                    .clone(),
                subcategory: package
                    .subcategory
                    .clone(),
                target_kind,
                input_ids: unreal_input_ids(package),
            },
        ),
    }
}

/// Selects the stable Unreal target kind from package evidence.
fn unreal_target_kind(package: &PhaseThreePackageRow) -> UnrealTargetKind {
    match package
        .category
        .as_str()
    {
        "language" => UnrealTargetKind::StringTable,
        "ui-images" | "game-icons" | "cards" => UnrealTargetKind::Texture,
        "ui-screens" | "ui-components" => UnrealTargetKind::UserInterface,
        "dialog" | "music" | "sound-effects" => UnrealTargetKind::SoundWave,
        "movies" => UnrealTargetKind::MediaSource,
        "missions" | "mission-scripts" => UnrealTargetKind::StateTree,
        "vehicle-tuning" | "sound-scripts" => UnrealTargetKind::DataTable,
        "extraction-reports" => UnrealTargetKind::Metadata,
        _ => UnrealTargetKind::DataAsset,
    }
}

/// Collects the exact ids consumed by one Unreal-native target.
fn unreal_input_ids(package: &PhaseThreePackageRow) -> Vec<String> {
    let mut ids = Vec::new();
    for role in PackageRole::all() {
        if role == PackageRole::Error {
            continue;
        }
        ids.extend_from_slice(package.ids_for_role(role));
    }
    ids.extend(
        package
            .text_key_ids
            .iter()
            .cloned(),
    );
    ids
}

#[cfg(test)]
mod tests {
    use super::{ConversionFamily, PhaseThreePackagePlanner, UnrealTargetKind};
    use crate::domain::package::index::PhaseThreePackageRow;

    fn row(
        category: &str,
        subcategory: &str,
        role_field: &str,
    ) -> Result<PhaseThreePackageRow, String> {
        let mut json = concat!(
            "{\"package_id\":\"pkg\",\"package_root\":\"pkg\",",
            "\"package_category\":\"CATEGORY\",",
            "\"package_subcategory\":\"SUBCATEGORY\",",
            "\"unit_count\":1,\"text_key_count\":0,",
            "\"unit_ids\":[\"unit-a\"],\"world_ids\":[],",
            "\"texture_ids\":[],\"material_ids\":[],",
            "\"model_ids\":[],\"physics_ids\":[],",
            "\"animation_ids\":[],\"scene_ids\":[],",
            "\"locator_ids\":[],\"camera_ids\":[],",
            "\"light_ids\":[],\"particle_ids\":[],",
            "\"controller_ids\":[],\"audio_ids\":[],",
            "\"movie_ids\":[],\"script_ids\":[],",
            "\"text_ids\":[],\"ui_ids\":[],",
            "\"metadata_ids\":[],\"error_ids\":[],",
            "\"source_unit_ids\":[],\"text_key_ids\":[],",
            "\"members\":[],\"text_keys\":[]}",
        )
        .replace(
            "SUBCATEGORY",
            subcategory,
        )
        .replace(
            "CATEGORY", category,
        );
        let empty_field = format!("\"{role_field}\":[]");
        let filled_field = format!("\"{role_field}\":[\"unit-a\"]");
        json = json.replace(
            &empty_field,
            &filled_field,
        );
        let role = role_field
            .strip_suffix("_ids")
            .ok_or_else(|| format!("invalid role field: {role_field}"))?;
        let member = format!(
            concat!(
                "\"members\":[{{",
                "\"id\":\"unit-a\",",
                "\"role\":\"{}\",",
                "\"path\":\"extracted/unit-a.bin\",",
                "\"type\":\"test\",",
                "\"kind\":\"test\",",
                "\"source_chunk_kind\":\"test\"}}]"
            ),
            role,
        );
        json = json.replace(
            "\"members\":[]",
            &member,
        );
        PhaseThreePackageRow::from_json_line(&json)
            .map_err(|error| error.to_string())
    }

    #[test]
    fn preserves_scene_assembly_roles_in_fbx_plans() -> Result<(), String> {
        for role_field in [
            "scene_ids",
            "locator_ids",
            "camera_ids",
        ] {
            let package = row(
                "cars",
                "cars/character-rigs/homer-v",
                role_field,
            )?;
            let plan = PhaseThreePackagePlanner::plan(&package);
            let Some(fbx) = plan.fbx else {
                return Err(
                    format!("{role_field} package should produce an FBX plan"),
                );
            };
            let retained = [
                fbx.model_ids,
                fbx.world_ids,
                fbx.scene_ids,
                fbx.locator_ids,
                fbx.camera_ids,
                fbx.animation_ids,
                fbx.texture_ids,
                fbx.material_ids,
                fbx.physics_ids,
            ]
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();
            if retained != ["unit-a".to_owned()] {
                return Err(
                    format!("FBX plan dropped {role_field}: {retained:?}"),
                );
            }
        }
        Ok(())
    }

    #[test]
    fn routes_model_packages_to_fbx() -> Result<(), String> {
        let package = row(
            "cars",
            "cars/character-rigs/homer-v",
            "model_ids",
        )?;
        let plan = PhaseThreePackagePlanner::plan(&package);
        if plan.family != ConversionFamily::FbxModel {
            return Err("car model package should route to FBX".to_owned());
        }
        let Some(fbx) = plan.fbx else {
            return Err("fbx plan should exist".to_owned());
        };
        if fbx.model_ids != ["unit-a".to_owned()] {
            return Err("fbx plan should carry model ids".to_owned());
        }
        Ok(())
    }

    #[test]
    fn excludes_provenance_sources_from_unreal_inputs() -> Result<(), String> {
        let json = concat!(
            "{\"package_id\":\"derived-language\",",
            "\"package_root\":\"derived/language\",",
            "\"package_category\":\"language\",",
            "\"package_subcategory\":\"language/objectives\",",
            "\"unit_count\":0,\"text_key_count\":1,",
            "\"unit_ids\":[],\"world_ids\":[],",
            "\"texture_ids\":[],\"material_ids\":[],",
            "\"model_ids\":[],\"physics_ids\":[],",
            "\"animation_ids\":[],\"scene_ids\":[],",
            "\"locator_ids\":[],\"camera_ids\":[],",
            "\"light_ids\":[],\"particle_ids\":[],",
            "\"controller_ids\":[],\"audio_ids\":[],",
            "\"movie_ids\":[],\"script_ids\":[],",
            "\"text_ids\":[],\"ui_ids\":[],",
            "\"metadata_ids\":[],\"error_ids\":[],",
            "\"source_unit_ids\":[\"source-a\"],",
            "\"text_key_ids\":[\"text-a\"],",
            "\"members\":[],",
            "\"text_keys\":[{",
            "\"id\":\"text-a\",",
            "\"key\":\"HELLO\",",
            "\"source_unit_id\":\"source-a\",",
            "\"subcategory\":\"language/objectives\"}]}",
        );
        let package = PhaseThreePackageRow::from_json_line(json)
            .map_err(|error| error.to_string())?;
        let plan = PhaseThreePackagePlanner::plan(&package);
        let unreal = plan
            .unreal
            .ok_or_else(
                || {
                    "derived text package should produce an Unreal plan"
                        .to_owned()
                },
            )?;
        if unreal.input_ids != ["text-a".to_owned()] {
            return Err(
                format!(
                    "provenance sources leaked into Unreal inputs: {:?}",
                    unreal.input_ids,
                ),
            );
        }
        Ok(())
    }

    #[test]
    fn routes_dialog_voice_to_unreal_sound_waves() -> Result<(), String> {
        let package = row(
            "dialog",
            "dialog/homer/ad-lib/free-roam/default",
            "audio_ids",
        )?;
        let plan = PhaseThreePackagePlanner::plan(&package);
        if plan.family != ConversionFamily::UnrealNative {
            return Err("dialog should route to Unreal-native data".to_owned());
        }
        let Some(unreal) = plan.unreal else {
            return Err("unreal plan should exist".to_owned());
        };
        if unreal.target_kind != UnrealTargetKind::SoundWave {
            return Err("dialog voice should target sound waves".to_owned());
        }
        Ok(())
    }

    #[test]
    fn routes_metadata_only_packages_to_do_not_import() -> Result<(), String> {
        let package = row(
            "ui-images",
            "ui-images/source-metadata/root",
            "metadata_ids",
        )?;
        let plan = PhaseThreePackagePlanner::plan(&package);
        if plan.family != ConversionFamily::DoNotImport {
            return Err("metadata-only package should not import".to_owned());
        }
        let Some(unreal) = plan.unreal else {
            return Err("metadata plan should exist".to_owned());
        };
        if unreal.target_kind != UnrealTargetKind::Metadata {
            return Err("metadata package should target metadata".to_owned());
        }
        Ok(())
    }
}
