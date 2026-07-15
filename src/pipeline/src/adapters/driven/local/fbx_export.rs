// File:
//   - fbx_export.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/fbx_export.rs
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
//   - The fbx-export contract for pipeline phase three.
// - Must-Not:
//   - Violate repository architecture, path, provenance, or output rules.
// - Allows:
//   - Operations required to validate and execute fbx-export.
// - Split-When:
//   - Split when fbx-export contains two independently testable contracts.
// - Merge-When:
//   - Another pipeline module owns the same module boundary with no distinct
//   - invariant.
// - Summary:
//   - Phase-three package-driven FBX export execution.
// - Description:
//   - Defines fbx-export data and behavior for pipeline phase three.
// - Usage:
//   - Used by pipeline phase three code that needs fbx-export.
// - Defaults:
//   - No implicit output outside the repository is allowed.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
// - docs/adr/fbx/export/fbx-output-contract-boundary.md
//
// Large file:
//   - true
//   - Reason: Phase-three FBX export keeps package resolution, member
//   - classification, character assembly, material resolution, and the
//   - capability report together because they form one export transaction;
//   - split when one stage gains an independently testable contract.
//

//! Phase-three package-driven FBX export execution.
//!
//! The exporter consumes only generated package-index evidence: the selected
//! row supplies member ids, roles, kinds, and safe relative paths, and every
//! decoded component is loaded through the fbx crate adapters. Nothing is
//! rediscovered from local filesystem layout, and every member id receives an
//! explicit capability outcome in the deterministic report. Binary FBX 7.7
//! is the sole FBX representation; optional review support emits scripts.
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use fbx::adapters::driven::binary_character_writer::{
    CharacterBinaryFbxSummary, EmbeddedTexture, write_binary_character_fbx,
};
use fbx::adapters::driven::blender_review_helper::{
    HelperSummary, write_review_helper,
};
use fbx::adapters::driven::decoded_animation_source::load_animation_clips;
use fbx::adapters::driven::decoded_component_source::{
    DecodedComponentError, DecodedComponentSource,
};
use fbx::adapters::driven::decoded_skin_source::load_character;
use fbx::adapters::driven::maya_import_helper::{
    Summary as MayaImportHelperSummary, write as write_maya_import_helper,
};
use fbx::domain::animation::AnimationClip;
use fbx::domain::character::CharacterAsset;
use fbx::domain::texture::MaterialBinding;
use fbx::ports::component_source::ComponentSource as _;
use schoenwald_filesystem::adapters::driving::local::{
    file_len as local_file_len, read_bytes as local_read_bytes,
    write_text as local_write_text,
};

use super::fbx_manifest::stable_file_stem;
use crate::domain::package::{
    ConversionFamily, PhaseThreePackageIndex, PhaseThreePackageMember,
    PhaseThreePackagePlanner, PhaseThreePackageRow, PhaseThreePackageSelector,
};
use crate::domain::{PipelineError, StageReport, escape_json};
use crate::ports::FbxExportOptions;

/// Package category supported by the current character export pass.
const CHARACTERS_CATEGORY: &str = "characters";
/// Shared character rig and texture dependency subcategory.
const CHARACTER_SHARED_SUBCATEGORY: &str = "characters/rig/common";

/// One deterministic capability decision for the export report.
struct CapabilityItem {
    /// Stable member id or derived evidence id.
    id: String,
    /// Controlled outcome value in kebab-case.
    outcome: &'static str,
    /// Deterministic decision reason.
    reason: String,
}

/// Optional experimental Blender helper tracked by the export report.
struct BlenderHelperArtifact {
    /// Generated standalone Blender helper script.
    script_path: PathBuf,
    /// Native timing evidence preserved by the helper.
    summary: HelperSummary,
}

/// Inputs required to materialize one optional Blender helper.
struct BlenderHelperRequest<'request> {
    /// Stable package identity used in diagnostics.
    package_id: &'request str,
    /// Package output directory containing the sibling FBX.
    package_dir: &'request Path,
    /// Stable artifact stem shared by the FBX and helper.
    file_stem: &'request str,
    /// Final sibling FBX path referenced by the helper.
    fbx_path: &'request Path,
    /// Native skeletal clips establishing source timing.
    animations: &'request [AnimationClip],
}

/// Optional helper output and capability evidence returned together.
struct BlenderHelperOutput {
    /// Generated files and timing summary.
    artifact: BlenderHelperArtifact,
    /// Capability-report item describing the optional conversion.
    capability: CapabilityItem,
}

/// Optional Maya import helper tracked by the export report.
struct MayaHelperArtifact {
    /// Generated standalone Maya import script.
    script_path: PathBuf,
    /// Generated helper file count.
    summary: MayaImportHelperSummary,
}

/// Inputs required to materialize one optional Maya import helper.
struct MayaHelperRequest<'request> {
    /// Stable package identity used in diagnostics.
    package_id: &'request str,
    /// Package output directory containing the sibling FBX.
    package_dir: &'request Path,
    /// Stable artifact stem shared by the FBX and helper.
    file_stem: &'request str,
    /// Final sibling binary FBX path referenced by the helper.
    fbx_path: &'request Path,
    /// Validated skeletal clips establishing the exported frame rate.
    animations: &'request [AnimationClip],
}

/// Optional Maya helper and capability evidence returned together.
struct MayaHelperOutput {
    /// Generated script and file-count summary.
    artifact: MayaHelperArtifact,
    /// Capability-report item describing the optional helper.
    capability: CapabilityItem,
}

/// Classified member paths driving one character export.
#[derive(Default)]
struct ClassifiedMembers<'row> {
    /// Skeleton members in package order.
    skeletons: Vec<&'row PhaseThreePackageMember>,
    /// Skin members in package order.
    skins: Vec<&'row PhaseThreePackageMember>,
    /// Composite drawable members in package order.
    composites: Vec<&'row PhaseThreePackageMember>,
    /// Animation clip members deferred to a later capability pass.
    animations: Vec<&'row PhaseThreePackageMember>,
    /// Controller members deferred to a later capability pass.
    controllers: Vec<&'row PhaseThreePackageMember>,
    /// Texture members preserved or staged by material resolution.
    textures: Vec<&'row PhaseThreePackageMember>,
    /// Material members resolved through decoded shader evidence.
    materials: Vec<&'row PhaseThreePackageMember>,
    /// Metadata members preserved for traceability.
    metadata: Vec<&'row PhaseThreePackageMember>,
    /// Members outside the character contract.
    unsupported: Vec<&'row PhaseThreePackageMember>,
}

/// Export one selected model package to a character FBX artifact.
///
/// # Errors
///
/// Returns an error when the package cannot be resolved, is not a supported
/// character package, or one component fails validation or serialization.
pub(super) fn export_fbx_package(
    index_path: &Path,
    selector: &PhaseThreePackageSelector,
    output_dir: &Path,
    base_root: &Path,
    options: FbxExportOptions,
) -> Result<StageReport, PipelineError> {
    let index = PhaseThreePackageIndex::read(index_path)
        .map_err(|error| PipelineError::new(error.to_string()))?;
    let package = selector
        .resolve(&index)
        .map_err(|error| PipelineError::new(error.to_string()))?;
    validate_character_package(package)?;
    let members = classify_members(package)?;
    let package_dir = output_dir.join(&package.package_id);
    let texture_staging_dir = package_dir.join(".texture-staging");
    remove_texture_staging_dir(&package_dir.join("textures"))?;
    remove_texture_staging_dir(&texture_staging_dir)?;
    let character = build_character(
        package, &members, base_root,
    )?;
    let animation_package = resolve_animation_package(
        &index, package,
    )?;
    let animations = build_animation_clips(
        animation_package,
        &character.bones,
        base_root,
    )?;
    let (materials, mut capability_items) = resolve_materials(
        &index,
        package,
        &members,
        base_root,
        &texture_staging_dir,
    )?;
    capability_items.extend(
        animation_capability_items(
            animation_package,
            &animations,
        ),
    );
    let file_stem = stable_file_stem(&package.subcategory);
    let fbx_path = package_dir.join(format!("{file_stem}.fbx"));
    let summary = write_embedded_character_fbx(
        &character,
        &materials,
        &animations,
        &texture_staging_dir,
        &fbx_path,
        &package.package_id,
    )?;
    let helper_output = write_optional_blender_helper(
        options.blender_helper,
        &BlenderHelperRequest {
            package_id: &package.package_id,
            package_dir: &package_dir,
            file_stem: &file_stem,
            fbx_path: &fbx_path,
            animations: &animations,
        },
    )?;
    let helper_artifact = if let Some(output) = helper_output {
        capability_items.push(output.capability);
        Some(output.artifact)
    } else {
        None
    };
    let maya_output = write_optional_maya_helper(
        options.maya,
        &MayaHelperRequest {
            package_id: &package.package_id,
            package_dir: &package_dir,
            file_stem: &file_stem,
            fbx_path: &fbx_path,
            animations: &animations,
        },
    )?;
    let maya_artifact = if let Some(output) = maya_output {
        capability_items.push(output.capability);
        Some(output.artifact)
    } else {
        None
    };
    capability_items.extend(member_capability_items(&members));
    let report_path = package_dir.join("capability-report.json");
    write_capability_report(
        &report_path,
        &package.package_id,
        capability_items,
    )?;
    stage_report(
        package,
        &summary,
        &fbx_path,
        &report_path,
        helper_artifact.as_ref(),
        maya_artifact.as_ref(),
    )
}

/// Require one selected package to be a supported character FBX model.
fn validate_character_package(
    package: &PhaseThreePackageRow
) -> Result<(), PipelineError> {
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
    if package.category != CHARACTERS_CATEGORY {
        return Err(
            PipelineError::new(
                format!(
                    concat!(
                        "fbx-export supports only character packages; ",
                        "package {} has category {}",
                    ),
                    package.package_id, package.category
                ),
            ),
        );
    }
    Ok(())
}

/// Materialize one requested Blender helper and capability decision.
fn write_optional_blender_helper(
    enabled: bool,
    request: &BlenderHelperRequest<'_>,
) -> Result<Option<BlenderHelperOutput>, PipelineError> {
    if !enabled {
        return Ok(None);
    }
    let script_path = request
        .package_dir
        .join(
            format!(
                "{}.blender.py",
                request.file_stem
            ),
        );
    let summary = write_review_helper(
        request.fbx_path,
        request.animations,
        &script_path,
    )
    .map_err(
        |error| {
            PipelineError::new(
                format!(
                    "Blender helper failed for {}: {error:?}",
                    request.package_id
                ),
            )
        },
    )?;
    let capability = CapabilityItem {
        id: "derived:blender-review-helper".to_owned(),
        outcome: "converted",
        reason: format!(
            concat!(
                "experimental unsupported Blender helper may not work; ",
                "it preserves native {} fps timing",
            ),
            summary.source_fps
        ),
    };
    Ok(
        Some(
            BlenderHelperOutput {
                artifact: BlenderHelperArtifact {
                    script_path,
                    summary,
                },
                capability,
            },
        ),
    )
}

/// Materialize one requested Maya import helper and capability decision.
fn write_optional_maya_helper(
    enabled: bool,
    request: &MayaHelperRequest<'_>,
) -> Result<Option<MayaHelperOutput>, PipelineError> {
    if !enabled {
        return Ok(None);
    }
    let script_path = request
        .package_dir
        .join(
            format!(
                "{}.maya.py",
                request.file_stem
            ),
        );
    let frame_rate = request
        .animations
        .first()
        .map(|clip| clip.frame_rate);
    let summary = write_maya_import_helper(
        request.fbx_path,
        frame_rate,
        &script_path,
    )
    .map_err(
        |error| {
            PipelineError::new(
                format!(
                    "Maya import helper failed for {}: {error:?}",
                    request.package_id
                ),
            )
        },
    )?;
    Ok(
        Some(
            MayaHelperOutput {
                artifact: MayaHelperArtifact {
                    script_path,
                    summary,
                },
                capability: CapabilityItem {
                    id: "derived:maya-import-helper".to_owned(),
                    outcome: "converted",
                    reason: concat!(
                        "optional Maya script imports the canonical binary \
                         FBX 7.7 and configures Maya to the validated \
                         exported frame rate when animations are present; ",
                        "no alternate FBX or Maya-native scene is emitted",
                    )
                    .to_owned(),
                },
            },
        ),
    )
}

/// Classify package members into character export families.
fn classify_members(
    package: &PhaseThreePackageRow
) -> Result<ClassifiedMembers<'_>, PipelineError> {
    let mut classified = ClassifiedMembers::default();
    for member in package.members() {
        match member
            .kind
            .as_str()
        {
            "p3d-skeleton" => classified
                .skeletons
                .push(member),
            "p3d-skin" => classified
                .skins
                .push(member),
            "p3d-composite-drawable" => classified
                .composites
                .push(member),
            "p3d-animation" => classified
                .animations
                .push(member),
            "p3d-controller" => classified
                .controllers
                .push(member),
            "p3d-texture" => classified
                .textures
                .push(member),
            "p3d-shader" => classified
                .materials
                .push(member),
            "package-manifest" => classified
                .metadata
                .push(member),
            _ => classified
                .unsupported
                .push(member),
        }
    }
    if classified
        .skins
        .is_empty()
    {
        return Err(
            PipelineError::new(
                format!(
                    "package {} has no skin members; animation-set and effect \
                     packages are a later capability pass",
                    package.package_id
                ),
            ),
        );
    }
    if classified
        .skeletons
        .len()
        != 1
    {
        return Err(
            PipelineError::new(
                format!(
                    "package {} must reference exactly one skeleton, found {}",
                    package.package_id,
                    classified
                        .skeletons
                        .len()
                ),
            ),
        );
    }
    if let Some(member) = classified
        .unsupported
        .first()
    {
        return Err(
            PipelineError::new(
                format!(
                    "package {} member {} has unsupported kind {}",
                    package.package_id, member.id, member.kind
                ),
            ),
        );
    }
    Ok(classified)
}

/// Assemble the validated character aggregate from classified members.
fn build_character(
    package: &PhaseThreePackageRow,
    members: &ClassifiedMembers<'_>,
    base_root: &Path,
) -> Result<CharacterAsset, PipelineError> {
    let skeleton_member = members
        .skeletons
        .first()
        .ok_or_else(
            || {
                PipelineError::new(
                    format!(
                        "package {} lost its skeleton member during \
                         classification",
                        package.package_id
                    ),
                )
            },
        )?;
    let skeleton_path = base_root.join(&skeleton_member.path);
    let skin_paths: Vec<PathBuf> = members
        .skins
        .iter()
        .map(|member| base_root.join(&member.path))
        .collect();
    let skin_path_refs: Vec<&Path> = skin_paths
        .iter()
        .map(PathBuf::as_path)
        .collect();
    let composite_paths: Vec<PathBuf> = members
        .composites
        .iter()
        .map(|member| base_root.join(&member.path))
        .collect();
    let composite_path_refs: Vec<&Path> = composite_paths
        .iter()
        .map(PathBuf::as_path)
        .collect();
    load_character(
        &stable_file_stem(&package.subcategory),
        &skeleton_path,
        &skin_path_refs,
        &composite_path_refs,
    )
    .map_err(
        |error| {
            PipelineError::new(
                format!(
                    "character assembly failed for {}: {error:?}",
                    package.package_id
                ),
            )
        },
    )
}

/// Resolve the unique companion animation-set row from package taxonomy.
fn resolve_animation_package<'index>(
    index: &'index PhaseThreePackageIndex,
    package: &PhaseThreePackageRow,
) -> Result<Option<&'index PhaseThreePackageRow>, PipelineError> {
    let Some(identity_root) = package
        .subcategory
        .strip_suffix("/base-model")
    else {
        return Ok(None);
    };
    let target = format!("{identity_root}/animation-set");
    let matches = index
        .packages()
        .iter()
        .filter(
            |candidate| {
                candidate.category == CHARACTERS_CATEGORY
                    && candidate.subcategory == target
            },
        )
        .collect::<Vec<_>>();
    match matches.as_slice() {
        [] => Ok(None),
        [animation_package] => Ok(Some(*animation_package)),
        _ => Err(
            PipelineError::new(
                format!(
                    "character package {} has multiple animation-set rows for \
                     subcategory {target}",
                    package.package_id
                ),
            ),
        ),
    }
}

/// Load every skeletal animation clip from one companion index row.
fn build_animation_clips(
    animation_package: Option<&PhaseThreePackageRow>,
    bones: &[fbx::domain::skeleton::Bone],
    base_root: &Path,
) -> Result<Vec<AnimationClip>, PipelineError> {
    let Some(package) = animation_package else {
        return Ok(Vec::new());
    };
    let paths = package
        .members()
        .iter()
        .filter(
            |member| {
                member.kind == "p3d-animation"
                    && member.source_chunk_kind == "animation"
            },
        )
        .map(|member| base_root.join(&member.path))
        .collect::<Vec<_>>();
    if paths.is_empty() {
        return Err(
            PipelineError::new(
                format!(
                    "animation-set package {} has no skeletal animation \
                     members",
                    package.package_id
                ),
            ),
        );
    }
    let path_refs = paths
        .iter()
        .map(PathBuf::as_path)
        .collect::<Vec<_>>();
    load_animation_clips(
        &path_refs, bones,
    )
    .map_err(
        |error| {
            PipelineError::new(
                format!(
                    "animation-set assembly failed for {}: {error:?}",
                    package.package_id
                ),
            )
        },
    )
}

/// Produce capability evidence for companion skeletal animation conversion.
fn animation_capability_items(
    animation_package: Option<&PhaseThreePackageRow>,
    clips: &[AnimationClip],
) -> Vec<CapabilityItem> {
    let Some(package) = animation_package else {
        return Vec::new();
    };
    let mut items = package
        .members()
        .iter()
        .filter(
            |member| {
                member.kind == "p3d-animation"
                    && member.source_chunk_kind == "animation"
            },
        )
        .map(
            |member| CapabilityItem {
                id: member
                    .id
                    .clone(),
                outcome: "converted",
                reason: "companion skeletal clip exported as an FBX animation \
                         stack"
                    .to_owned(),
            },
        )
        .collect::<Vec<_>>();
    for clip in clips {
        if !clip
            .ignored_group_ids
            .is_empty()
        {
            items.push(
                CapabilityItem {
                    id: format!(
                        "animation-helper-groups:{}",
                        clip.name
                    ),
                    outcome: "preserved-as-metadata",
                    reason: format!(
                        "{} non-deforming helper groups were not bound to \
                         skeleton bones",
                        clip.ignored_group_ids
                            .len()
                    ),
                },
            );
        }
    }
    items
}

/// Resolve one shader texture reference to a unique index-published PNG.
fn resolve_shared_texture_member<'index>(
    index: &'index PhaseThreePackageIndex,
    texture_reference: &str,
) -> Result<
    Option<(
        &'index PhaseThreePackageRow,
        &'index PhaseThreePackageMember,
    )>,
    PipelineError,
> {
    let expected_file_name =
        normalized_texture_png_file_name(texture_reference)?;
    let matches = index
        .packages()
        .iter()
        .flat_map(
            |package| {
                package
                    .members()
                    .iter()
                    .map(
                        move |member| {
                            (
                                package, member,
                            )
                        },
                    )
            },
        )
        .filter(
            |(package, member)| {
                package.category == CHARACTERS_CATEGORY
                    && package.subcategory == CHARACTER_SHARED_SUBCATEGORY
                    && member.kind == "p3d-texture"
                    && member.source_chunk_kind == "texture"
                    && Path::new(&member.path)
                        .file_name()
                        .and_then(|value| value.to_str())
                        .is_some_and(|name| name == expected_file_name)
            },
        )
        .collect::<Vec<_>>();
    match matches.as_slice() {
        [] => Ok(None),
        [resolved] => Ok(Some(*resolved)),
        _ => Err(
            PipelineError::new(
                format!(
                    "texture reference {texture_reference} resolves to \
                     multiple package-index PNG members"
                ),
            ),
        ),
    }
}

/// Normalize one safe decoded texture reference to its staged PNG file name.
fn normalized_texture_png_file_name(
    texture_reference: &str
) -> Result<String, PipelineError> {
    let normalized_reference = texture_reference.trim_end_matches('\u{0}');
    let mut components = Path::new(normalized_reference).components();
    if normalized_reference.is_empty()
        || normalized_reference != normalized_reference.trim()
        || !matches!(
            components.next(),
            Some(std::path::Component::Normal(_))
        )
        || components
            .next()
            .is_some()
    {
        return Err(
            PipelineError::new(
                format!(
                    "invalid shader texture reference: {normalized_reference}"
                ),
            ),
        );
    }
    let stem = normalized_reference
        .rsplit_once('.')
        .filter(
            |(_, extension)| {
                extension.eq_ignore_ascii_case("bmp")
                    || extension.eq_ignore_ascii_case("png")
            },
        )
        .map_or(
            normalized_reference,
            |(value, _)| value,
        );
    if stem.is_empty() {
        return Err(
            PipelineError::new(
                format!(
                    "invalid shader texture reference: {normalized_reference}"
                ),
            ),
        );
    }
    Ok(format!("{stem}.png"))
}

/// Serialize one self-contained FBX and always remove private texture staging.
fn write_embedded_character_fbx(
    character: &CharacterAsset,
    materials: &[MaterialBinding],
    animations: &[AnimationClip],
    texture_staging_dir: &Path,
    fbx_path: &Path,
    package_id: &str,
) -> Result<CharacterBinaryFbxSummary, PipelineError> {
    let export_result = (|| {
        let embedded_textures = read_embedded_textures(
            materials,
            texture_staging_dir,
        )?;
        write_binary_character_fbx(
            character,
            materials,
            &embedded_textures,
            animations,
            fbx_path,
        )
        .map_err(
            |error| {
                let context = "character FBX serialization failed";
                PipelineError::new(
                    format!("{context} for {package_id}: {error:?}"),
                )
            },
        )
    })();
    let cleanup_result = remove_texture_staging_dir(texture_staging_dir);
    let summary = export_result?;
    cleanup_result?;
    Ok(summary)
}

/// Read staged PNGs into deterministic binary FBX texture payloads.
fn read_embedded_textures(
    materials: &[MaterialBinding],
    texture_staging_dir: &Path,
) -> Result<Vec<EmbeddedTexture>, PipelineError> {
    let file_names: BTreeSet<&str> = materials
        .iter()
        .filter_map(
            |binding| {
                binding
                    .texture_file_name
                    .as_deref()
            },
        )
        .collect();
    let mut textures = Vec::with_capacity(file_names.len());
    for file_name in file_names {
        let path = texture_staging_dir.join(file_name);
        let content = local_read_bytes(&path).map_err(
            |error| {
                PipelineError::new(
                    format!(
                        "failed to read embedded texture {}: {error}",
                        path.display()
                    ),
                )
            },
        )?;
        textures.push(
            EmbeddedTexture {
                file_name: file_name.to_owned(),
                content,
            },
        );
    }
    Ok(textures)
}

/// Remove the private texture staging directory before or after one export.
fn remove_texture_staging_dir(path: &Path) -> Result<(), PipelineError> {
    if !path.exists() {
        return Ok(());
    }
    if !path.is_dir() {
        return Err(
            PipelineError::new(
                format!(
                    "texture staging path is not a directory: {}",
                    path.display()
                ),
            ),
        );
    }
    std::fs::remove_dir_all(path).map_err(
        |error| {
            PipelineError::new(
                format!(
                    "failed to remove texture staging directory {}: {error}",
                    path.display()
                ),
            )
        },
    )
}

/// Select the material identity when texture staging is deferred.
fn deferred_material_identity(
    _shader_member_identity: &str,
    decoded_material_identity: &str,
) -> String {
    decoded_material_identity.to_owned()
}

/// Resolve one shader and preserve any cross-package texture evidence.
fn resolve_material_binding(
    source: &DecodedComponentSource,
    index: &PhaseThreePackageIndex,
    base_root: &Path,
    shader: &str,
) -> Result<
    (
        MaterialBinding,
        Option<CapabilityItem>,
    ),
    PipelineError,
> {
    match source.resolve_material(shader) {
        Ok(binding) => Ok(
            (
                binding, None,
            ),
        ),
        Err(DecodedComponentError::MissingTexture {
            shader: shader_name,
            texture,
            ..
        }) => {
            let Some((owner, member)) = resolve_shared_texture_member(
                index, &texture,
            )?
            else {
                let binding = MaterialBinding::new(
                    deferred_material_identity(
                        shader,
                        &shader_name,
                    ),
                    None,
                )
                .map_err(
                    |error| {
                        PipelineError::new(
                            format!(
                                "material binding failed for {shader}: \
                                 {error:?}"
                            ),
                        )
                    },
                )?;
                let item = CapabilityItem {
                    id: format!("texture-reference:{texture}"),
                    outcome: "deferred",
                    reason: format!(
                        "shader {shader_name} has no unique shared PNG"
                    ),
                };
                return Ok(
                    (
                        binding,
                        Some(item),
                    ),
                );
            };
            let binding = source
                .resolve_material_with_external_texture(
                    shader,
                    &base_root.join(&member.path),
                )
                .map_err(
                    |error| {
                        PipelineError::new(
                            format!(
                                "shared texture failed for {shader}: {error:?}"
                            ),
                        )
                    },
                )?;
            let item = CapabilityItem {
                id: format!("texture-reference:{texture}"),
                outcome: "converted",
                reason: format!(
                    "shader {shader_name} uses package {} member {}",
                    owner.package_id, member.id
                ),
            };
            Ok(
                (
                    binding,
                    Some(item),
                ),
            )
        }
        Err(error) => Err(
            PipelineError::new(
                format!(
                    "material resolution failed for shader {shader}: {error:?}"
                ),
            ),
        ),
    }
}

/// Resolve every used shader through private texture staging for embedding.
fn resolve_materials(
    index: &PhaseThreePackageIndex,
    package: &PhaseThreePackageRow,
    members: &ClassifiedMembers<'_>,
    base_root: &Path,
    texture_staging_dir: &Path,
) -> Result<
    (
        Vec<MaterialBinding>,
        Vec<CapabilityItem>,
    ),
    PipelineError,
> {
    let package_root = base_root.join(&package.package_root);
    let source = DecodedComponentSource::new(
        package_root,
        texture_staging_dir.to_path_buf(),
    );
    let mut shader_names: Vec<String> = members
        .materials
        .iter()
        .filter_map(
            |member| {
                Path::new(&member.path)
                    .file_stem()
                    .and_then(|stem| stem.to_str())
                    .map(str::to_owned)
            },
        )
        .collect();
    shader_names.sort();
    shader_names.dedup();
    let mut bindings = Vec::with_capacity(shader_names.len());
    let mut items = Vec::new();
    for shader in &shader_names {
        let (binding, optional_capability) = resolve_material_binding(
            &source, index, base_root, shader,
        )?;
        bindings.push(binding);
        if let Some(capability_item) = optional_capability {
            items.push(capability_item);
        }
    }
    Ok(
        (
            bindings, items,
        ),
    )
}

/// Append one uniform capability outcome for a member collection.
fn append_capability_items(
    items: &mut Vec<CapabilityItem>,
    members: &[&PhaseThreePackageMember],
    outcome: &'static str,
    reason: &'static str,
) {
    items.extend(
        members
            .iter()
            .map(
                |member| CapabilityItem {
                    id: member
                        .id
                        .clone(),
                    outcome,
                    reason: reason.to_owned(),
                },
            ),
    );
}

/// Produce deterministic capability items for every classified member.
fn member_capability_items(
    members: &ClassifiedMembers<'_>
) -> Vec<CapabilityItem> {
    let mut items = Vec::new();
    append_capability_items(
        &mut items,
        &members.skeletons,
        "converted",
        "skeleton exported as FBX limb-node hierarchy with bind pose",
    );
    append_capability_items(
        &mut items,
        &members.skins,
        "converted",
        "skin exported with normals, UVs, and weighted clusters",
    );
    append_capability_items(
        &mut items,
        &members.composites,
        "converted",
        "composite drawable validated against skeleton and skins",
    );
    append_capability_items(
        &mut items,
        &members.materials,
        "converted",
        "shader exported as an FBX material binding",
    );
    for member in &members.textures {
        let embeddable = Path::new(&member.path)
            .extension()
            .and_then(|extension| extension.to_str())
            .is_some_and(|extension| extension.eq_ignore_ascii_case("png"));
        items.push(
            CapabilityItem {
                id: member
                    .id
                    .clone(),
                outcome: if embeddable {
                    "converted"
                } else {
                    "preserved-as-metadata"
                },
                reason: if embeddable {
                    "referenced PNG embedded in binary FBX Video.Content"
                        .to_owned()
                } else {
                    "texture metadata preserved for traceability".to_owned()
                },
            },
        );
    }
    append_capability_items(
        &mut items,
        &members.animations,
        "deferred",
        "package-local texture animation remains deferred",
    );
    append_capability_items(
        &mut items,
        &members.controllers,
        "deferred",
        "package-local controller behavior remains deferred",
    );
    append_capability_items(
        &mut items,
        &members.metadata,
        "preserved-as-metadata",
        "package manifest preserved for traceability",
    );
    items
}

/// Write the deterministic capability report next to the FBX artifact.
fn write_capability_report(
    path: &Path,
    package_id: &str,
    mut items: Vec<CapabilityItem>,
) -> Result<(), PipelineError> {
    items.sort_by(
        |left, right| {
            left.id
                .cmp(&right.id)
        },
    );
    let mut json = String::new();
    json.push_str("{\n");
    json.push_str("  \"package_id\": \"");
    json.push_str(&escape_json(package_id));
    json.push_str("\",\n");
    json.push_str("  \"items\": [\n");
    for (position, item) in items
        .iter()
        .enumerate()
    {
        json.push_str("    {\"id\": \"");
        json.push_str(&escape_json(&item.id));
        json.push_str("\", \"outcome\": \"");
        json.push_str(item.outcome);
        json.push_str("\", \"reason\": \"");
        json.push_str(&escape_json(&item.reason));
        json.push_str("\"}");
        if position.saturating_add(1) < items.len() {
            json.push(',');
        }
        json.push('\n');
    }
    json.push_str("  ]\n");
    json.push_str("}\n");
    local_write_text(
        path, &json, true,
    )
    .map_err(
        |error| {
            PipelineError::new(
                format!("failed to write capability report: {error}"),
            )
        },
    )?;
    Ok(())
}

/// Build the stage report for one completed character export.
fn stage_report(
    package: &PhaseThreePackageRow,
    summary: &CharacterBinaryFbxSummary,
    fbx_path: &Path,
    report_path: &Path,
    helper: Option<&BlenderHelperArtifact>,
    maya: Option<&MayaHelperArtifact>,
) -> Result<StageReport, PipelineError> {
    let fbx_bytes = file_len(fbx_path)?;
    let report_bytes = file_len(report_path)?;
    let (helper_files, helper_bytes, helper_note) =
        if let Some(artifact) = helper {
            let bytes = file_len(&artifact.script_path)?;
            (
                artifact
                    .summary
                    .files,
                bytes,
                format!(
                    "experimental-unsupported:{}:{}fps",
                    artifact
                        .script_path
                        .display(),
                    artifact
                        .summary
                        .source_fps
                ),
            )
        } else {
            (
                0,
                0,
                "disabled".to_owned(),
            )
        };
    let (maya_files, maya_bytes, maya_note) = if let Some(artifact) = maya {
        (
            artifact
                .summary
                .files,
            file_len(&artifact.script_path)?,
            artifact
                .script_path
                .display()
                .to_string(),
        )
    } else {
        (
            0,
            0,
            "disabled".to_owned(),
        )
    };
    let files_with_blender = StageReport::checked_file_total(
        "fbx-export",
        2,
        helper_files,
    )?;
    let files = StageReport::checked_file_total(
        "fbx-export",
        files_with_blender,
        maya_files,
    )?;
    let bytes_with_report = StageReport::checked_byte_total(
        "fbx-export",
        fbx_bytes,
        report_bytes,
    )?;
    let bytes_with_blender = StageReport::checked_byte_total(
        "fbx-export",
        bytes_with_report,
        helper_bytes,
    )?;
    let bytes = StageReport::checked_byte_total(
        "fbx-export",
        bytes_with_blender,
        maya_bytes,
    )?;
    Ok(
        StageReport {
            name: "fbx-export",
            files,
            bytes,
            note: format!(
                "package={} output={} bones={} geometries={} clusters={} \
                 materials={} textures={} animations={} blender_helper={} \
                 maya_helper={}",
                package.package_id,
                fbx_path.display(),
                summary.bones,
                summary.geometries,
                summary.clusters,
                summary.materials,
                summary.textures,
                summary.animations,
                helper_note,
                maya_note
            ),
        },
    )
}

/// Supports the `file_len` operation within this deterministic export
/// boundary.
fn file_len(path: &Path) -> Result<u64, PipelineError> {
    local_file_len(path).map_err(
        |error| {
            PipelineError::new(
                format!("failed to stat export artifact: {error}"),
            )
        },
    )
}

#[cfg(test)]
mod tests {
    use super::{deferred_material_identity, normalized_texture_png_file_name};

    #[test]
    fn deferred_material_preserves_decoded_shader_identity() {
        assert_eq!(
            deferred_material_identity(
                "char_swatches_lit_m_",
                "char_swatches_lit_m",
            ),
            "char_swatches_lit_m"
        );
    }

    #[test]
    fn normalizes_trailing_nul_padded_texture_reference() {
        let result = normalized_texture_png_file_name(
            "char_swatches_lit.bmp\u{0}\u{0}\u{0}",
        );

        assert!(
            result.is_ok(),
            "fixed-width texture padding should normalize: {result:?}"
        );
        assert_eq!(
            result
                .ok()
                .as_deref(),
            Some("char_swatches_lit.png")
        );
    }
}
