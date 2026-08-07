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
//   - Plan domain module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Plan domain module.
// - Description:
//   - Implements the declared domain module responsibility for pipeline.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Plan domain module.

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
    /// Normalized semantic source awaiting a domain compiler.
    SemanticSource,
    /// `DataTable` target.
    DataTable,
    /// `StringTable` target.
    StringTable,
    /// `Font` target.
    Font,
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

/// Unreal representation that one FBX package can materialize directly.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FbxTargetKind {
    /// One complete static-mesh asset with no unresolved companion semantics.
    StaticMesh,
    /// One complete skeletal-mesh asset with no animation or runtime
    /// companions.
    SkeletalMesh,
    /// Geometry exists, but the package must split into multiple native assets.
    SemanticSplit,
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
    /// Direct Unreal representation or explicit semantic-split requirement.
    pub target_kind: FbxTargetKind,
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
        if package.has_fbx_geometry() && fbx_category(&package.category) {
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
    package.unit_ids.len() == package.ids_for_role(PackageRole::Metadata).len()
        && package.text_key_ids.is_empty()
}

/// Builds a non-importing plan that preserves metadata identifiers.
fn metadata_plan(package: &PhaseThreePackageRow) -> PhaseThreePackagePlan {
    PhaseThreePackagePlan {
        package_id: package.package_id.clone(),
        family: ConversionFamily::DoNotImport,
        fbx: None,
        unreal: Some(UnrealNativePlan {
            package_id: package.package_id.clone(),
            subcategory: package.subcategory.clone(),
            target_kind: UnrealTargetKind::Metadata,
            input_ids: package.unit_ids.clone(),
        }),
    }
}

/// Builds an FBX plan while keeping non-model companion ids attached.
fn fbx_plan(package: &PhaseThreePackageRow) -> PhaseThreePackagePlan {
    PhaseThreePackagePlan {
        package_id: package.package_id.clone(),
        family: ConversionFamily::FbxModel,
        fbx: Some(FbxModelPlan {
            package_id: package.package_id.clone(),
            subcategory: package.subcategory.clone(),
            target_kind: fbx_target_kind(package),
            model_ids: package.ids_for_role(PackageRole::Model).to_vec(),
            world_ids: package.ids_for_role(PackageRole::World).to_vec(),
            scene_ids: package.ids_for_role(PackageRole::Scene).to_vec(),
            locator_ids: package.ids_for_role(PackageRole::Locator).to_vec(),
            camera_ids: package.ids_for_role(PackageRole::Camera).to_vec(),
            animation_ids: package
                .ids_for_role(PackageRole::Animation)
                .to_vec(),
            texture_ids: package.ids_for_role(PackageRole::Texture).to_vec(),
            material_ids: package.ids_for_role(PackageRole::Material).to_vec(),
            physics_ids: package.ids_for_role(PackageRole::Physics).to_vec(),
        }),
        unreal: None,
    }
}

/// Select the only direct Unreal mesh representation justified by evidence.
fn fbx_target_kind(package: &PhaseThreePackageRow) -> FbxTargetKind {
    if single_static_mesh_package(package) {
        return FbxTargetKind::StaticMesh;
    }
    if single_skeletal_mesh_package(package) {
        return FbxTargetKind::SkeletalMesh;
    }
    FbxTargetKind::SemanticSplit
}

/// Require a package to contain only one static-mesh asset surface.
fn single_static_mesh_package(package: &PhaseThreePackageRow) -> bool {
    let mut has_mesh = false;
    package.members().iter().all(|member| match member.role {
        PackageRole::Model => {
            let exact =
                member.kind == "p3d-mesh" && member.source_chunk_kind == "mesh";
            has_mesh |= exact;
            exact
        },
        PackageRole::Material => {
            member.kind == "p3d-shader" && member.source_chunk_kind == "shader"
        },
        PackageRole::Texture => {
            member.kind == "p3d-texture"
                && matches!(
                    member.source_chunk_kind.as_str(),
                    "none" | "texture"
                )
        },
        PackageRole::Metadata => metadata_member_is_non_runtime(member),
        _ => false,
    }) && has_mesh
}

/// Require a package to contain only one skeletal-mesh asset surface.
fn single_skeletal_mesh_package(package: &PhaseThreePackageRow) -> bool {
    let mut has_skin = false;
    let mut has_skeleton = false;
    let mut has_composite = false;
    package.members().iter().all(|member| match member.role {
        PackageRole::Model => {
            match (member.kind.as_str(), member.source_chunk_kind.as_str()) {
                ("p3d-skin", "skin") => {
                    has_skin = true;
                    true
                },
                ("p3d-composite-drawable", "composite_drawable") => {
                    has_composite = true;
                    true
                },
                ("p3d-mesh", "mesh") => true,
                _ => false,
            }
        },
        PackageRole::Animation
            if member.kind == "p3d-skeleton"
                && member.source_chunk_kind == "skeleton" =>
        {
            has_skeleton = true;
            true
        },
        PackageRole::Material => {
            member.kind == "p3d-shader" && member.source_chunk_kind == "shader"
        },
        PackageRole::Texture => {
            member.kind == "p3d-texture"
                && matches!(
                    member.source_chunk_kind.as_str(),
                    "none" | "texture"
                )
        },
        PackageRole::Metadata => metadata_member_is_non_runtime(member),
        _ => false,
    }) && has_skin
        && has_skeleton
        && has_composite
}

/// Return whether metadata carries no additional runtime representation.
fn metadata_member_is_non_runtime(
    member: &super::index::PhaseThreePackageMember,
) -> bool {
    matches!(
        (member.kind.as_str(), member.source_chunk_kind.as_str()),
        ("package-manifest", "none") | ("p3d-export-info", "export_info")
    )
}

/// Builds an Unreal-native plan for packages outside the FBX boundary.
fn unreal_plan(package: &PhaseThreePackageRow) -> PhaseThreePackagePlan {
    let target_kind = unreal_target_kind(package);
    PhaseThreePackagePlan {
        package_id: package.package_id.clone(),
        family: ConversionFamily::UnrealNative,
        fbx: None,
        unreal: Some(UnrealNativePlan {
            package_id: package.package_id.clone(),
            subcategory: package.subcategory.clone(),
            target_kind,
            input_ids: unreal_input_ids(package),
        }),
    }
}

/// Selects the stable Unreal target kind from package evidence.
fn unreal_target_kind(package: &PhaseThreePackageRow) -> UnrealTargetKind {
    match package.category.as_str() {
        "language" if !package.text_key_ids.is_empty() => {
            UnrealTargetKind::StringTable
        },
        "language" if !package.ids_for_role(PackageRole::Ui).is_empty() => {
            UnrealTargetKind::UserInterface
        },
        "language" => UnrealTargetKind::SemanticSource,
        "ui-images" | "game-icons" | "cards" => UnrealTargetKind::Texture,
        "ui-screens" | "ui-components" => UnrealTargetKind::UserInterface,
        "ui-resources" if has_member_kind(package, "p3d-texture-font") => {
            UnrealTargetKind::Font
        },
        "ui-resources"
            if !package.ids_for_role(PackageRole::Text).is_empty() =>
        {
            UnrealTargetKind::StringTable
        },
        "cinematics"
            if !package.ids_for_role(PackageRole::Audio).is_empty() =>
        {
            UnrealTargetKind::SoundWave
        },
        "characters"
            if !package.ids_for_role(PackageRole::Texture).is_empty() =>
        {
            UnrealTargetKind::Texture
        },
        "dialog" | "music" | "sound-effects" => UnrealTargetKind::SoundWave,
        "movies" if !package.ids_for_role(PackageRole::Movie).is_empty() => {
            UnrealTargetKind::MediaSource
        },
        "movies" => UnrealTargetKind::Metadata,
        "missions" => UnrealTargetKind::SemanticSource,
        "mission-scripts" => UnrealTargetKind::SemanticSource,
        "vehicle-tuning" | "sound-scripts" => UnrealTargetKind::DataTable,
        "extraction-reports" => UnrealTargetKind::Metadata,
        _ => UnrealTargetKind::SemanticSource,
    }
}

/// Returns whether one package contains a physical member of an exact kind.
fn has_member_kind(package: &PhaseThreePackageRow, kind: &str) -> bool {
    package.members().iter().any(|member| member.kind == kind)
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
    ids.extend(package.text_key_ids.iter().cloned());
    ids
}

#[cfg(test)]
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../../tests/migration/pipeline/unit/domain/package/plan/tests.rs"]
mod tests;
