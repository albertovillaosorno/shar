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
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use fbx::adapters::driven::binary_character_writer::{
    CharacterBinaryFbxSummary, EmbeddedTexture, write_binary_character_fbx,
    write_binary_character_fbx_embedded,
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
use fbx::adapters::driven::semantic_character_texture::request::{
    ExtraMaterialRequest, GroupAddressRequest,
};
use fbx::adapters::driven::semantic_character_texture::{
    PreparedSemanticCharacter, SemanticTextureArtifactError,
    SemanticTextureRequest, prepare_semantic_character,
    publish_prepared_semantic_character,
};
use fbx::domain::animation::AnimationClip;
use fbx::domain::character::CharacterAsset;
use fbx::domain::texture::MaterialBinding;
use fbx::ports::component_source::ComponentSource as _;
use schoenwald_filesystem::adapters::driving::local::{
    file_len as local_file_len, read_bytes as local_read_bytes,
    write_text as local_write_text,
};
use serde_json::{Value, json};
use shar_sha256::digest_hex;

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
/// Full general animation bank used by manually verified non-playable exports.
const GENERAL_CHARACTER_ANIMATION_SUBCATEGORY: &str =
    "characters/homer/animation-set";

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
    /// Rigid prop mesh members in package order.
    meshes: Vec<&'row PhaseThreePackageMember>,
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

/// Semantic group ownership returned by exact shader classification.
type SemanticGroupOwnership = (
    Vec<GroupAddressRequest>,
    Option<GroupAddressRequest>,
    BTreeSet<String>,
);

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
    let texture_dir = package_dir.join("textures");
    let texture_staging_dir = package_dir.join(".texture-staging");
    ensure_texture_output_absent(&texture_dir)?;
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
    let export_target = CharacterExportTarget {
        texture_staging_dir: &texture_staging_dir,
        texture_dir: &texture_dir,
        fbx_path: &fbx_path,
        package_id: &package.package_id,
    };
    let summary = serialize_selected_texture_storage(
        &character,
        &materials,
        &animations,
        &export_target,
        options.embed_textures,
    )?;
    capability_items.push(texture_storage_capability(options.embed_textures));
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
    let maya_artifact = materialize_optional_maya_helper(
        options.maya,
        &MayaHelperRequest {
            package_id: &package.package_id,
            package_dir: &package_dir,
            file_stem: &file_stem,
            fbx_path: &fbx_path,
            animations: &animations,
        },
        &mut capability_items,
    )?;
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

/// Prepare, publish, and verify one catalog character package.
pub(super) fn export_prepared_character_package(
    index: &PhaseThreePackageIndex,
    package: &PhaseThreePackageRow,
    output_root: &Path,
    base_root: &Path,
) -> Result<Value, PipelineError> {
    validate_character_package(package)?;
    let members = classify_members(package)?;
    let character = build_character(
        package, &members, base_root,
    )?;
    let animation_package = resolve_animation_package(
        index, package,
    )?;
    let input_dir = output_root
        .join(".texture-inputs")
        .join(&package.package_id);
    remove_texture_staging_dir(&input_dir)?;
    let result = (|| {
        let (materials, _capabilities) = resolve_materials(
            index, package, &members, base_root, &input_dir,
        )?;
        let mut request = semantic_request(
            index,
            package,
            &members,
            &character,
            &materials,
            animation_package,
            base_root,
            &input_dir,
        )?;
        let source_topology = topology_counts(&character)?;
        let (prepared, selected_mode) =
            prepare_with_source_fallback(&mut request)?;
        let prepared_topology = topology_counts(&prepared.character)?;
        if prepared_topology != source_topology
            || prepared
                .character
                .bones
                .len()
                != character
                    .bones
                    .len()
        {
            return Err(
                PipelineError::new(
                    format!(
                        "semantic preparation changed topology or rig for {}",
                        package.package_id
                    ),
                ),
            );
        }
        let package_dir = output_root.join(&package.package_id);
        let summary = publish_prepared_semantic_character(
            &package_dir,
            &prepared,
        )
        .map_err(
            |error| {
                PipelineError::new(
                    format!(
                        "prepared character publication failed for {}: {error}",
                        package.package_id
                    ),
                )
            },
        )?;
        verify_summary(
            package,
            &summary,
            &prepared,
            source_topology,
        )?;
        catalog_entry(
            package,
            animation_package,
            &package_dir,
            &prepared,
            &summary,
            source_topology,
            selected_mode,
        )
    })();
    let cleanup = remove_texture_staging_dir(&input_dir);
    let entry = result?;
    cleanup?;
    Ok(entry)
}

/// Build one explicit semantic request from decoded package evidence.
#[expect(
    clippy::too_many_arguments,
    clippy::too_many_lines,
    reason = "Package evidence and semantic policy form one request \
              transaction."
)]
fn semantic_request(
    index: &PhaseThreePackageIndex,
    package: &PhaseThreePackageRow,
    members: &ClassifiedMembers<'_>,
    character: &CharacterAsset,
    materials: &[MaterialBinding],
    animation_package: Option<&PhaseThreePackageRow>,
    base_root: &Path,
    input_dir: &Path,
) -> Result<SemanticTextureRequest, PipelineError> {
    let binding_by_material = materials
        .iter()
        .map(
            |binding| {
                (
                    binding
                        .material_name
                        .as_str(),
                    binding,
                )
            },
        )
        .collect::<BTreeMap<_, _>>();
    let (body_groups, eye_group, extra_material_names) =
        semantic_group_ownership(character)?;
    let body_texture_path = body_texture_path(
        character,
        &body_groups,
        &binding_by_material,
        input_dir,
    )?;
    let mut extra_materials = Vec::new();
    let mut untextured_materials = Vec::new();
    for material_name in extra_material_names {
        let binding = binding_by_material
            .get(material_name.as_str())
            .ok_or_else(
                || {
                    PipelineError::new(
                        format!(
                            "material {material_name} has no resolved binding \
                             for {}",
                            package.package_id
                        ),
                    )
                },
            )?;
        if let Some(output_file_name) = binding
            .texture_file_name
            .as_ref()
        {
            extra_materials.push(
                ExtraMaterialRequest {
                    material_name,
                    texture_path: input_dir.join(output_file_name),
                    output_file_name: output_file_name.clone(),
                },
            );
        } else {
            untextured_materials.push(material_name);
        }
    }
    let skeleton_path = members
        .skeletons
        .first()
        .map(|member| base_root.join(&member.path))
        .ok_or_else(
            || PipelineError::new("character catalog package lost skeleton"),
        )?;
    let skin_paths = members
        .skins
        .iter()
        .map(|member| base_root.join(&member.path))
        .collect();
    let mesh_paths = members
        .meshes
        .iter()
        .map(|member| base_root.join(&member.path))
        .collect();
    let composite_paths = members
        .composites
        .iter()
        .map(|member| base_root.join(&member.path))
        .collect();
    let general_animation_paths = animation_member_paths(
        animation_package,
        base_root,
    )?;
    let eye_frame_paths = eye_group
        .map(
            |_address| {
                shared_eye_frame_paths(
                    index, base_root,
                )
            },
        )
        .transpose()?;
    Ok(
        SemanticTextureRequest {
            character_name: package
                .package_id
                .clone(),
            skeleton_path,
            skin_paths,
            mesh_paths,
            composite_paths,
            general_animation_paths,
            character_animation_paths: Vec::new(),
            body_texture_path,
            body_texture_mode: "semantic-atlas".to_owned(),
            body_texture_address_mode: "tile".to_owned(),
            eye_frame_paths,
            body_groups,
            eye_group,
            color_overrides: Vec::new(),
            hair_luminance_ratio: 0.2,
            body_atlas_width: 2048,
            body_atlas_height: 2048,
            body_atlas_padding: 8,
            body_atlas_background: [
                128, 128, 128, 255,
            ],
            eye_output_size: 64,
            extra_materials,
            untextured_materials,
        },
    )
}

/// Classify body, eye, and accessory groups from exact shader identities.
fn semantic_group_ownership(
    character: &CharacterAsset
) -> Result<SemanticGroupOwnership, PipelineError> {
    let mut body_groups = Vec::new();
    let mut eye_group = None;
    let mut extra_materials = BTreeSet::new();
    for (part_index, part) in character
        .parts
        .iter()
        .enumerate()
    {
        for (group_index, group) in part
            .mesh
            .groups
            .iter()
            .enumerate()
        {
            let address = GroupAddressRequest {
                part_index,
                group_index,
            };
            let shader = group
                .shader
                .to_ascii_lowercase();
            if shader.contains("char_swatches") {
                body_groups.push(address);
            } else if shader.contains("eyeball") {
                if eye_group
                    .replace(address)
                    .is_some()
                {
                    return Err(
                        PipelineError::new(
                            format!(
                                "character {} has multiple eye groups",
                                character.name
                            ),
                        ),
                    );
                }
            } else {
                let _inserted = extra_materials.insert(
                    group
                        .shader
                        .clone(),
                );
            }
        }
    }
    if body_groups.is_empty() {
        return Err(
            PipelineError::new(
                format!(
                    "character {} has no body swatch group",
                    character.name
                ),
            ),
        );
    }
    Ok(
        (
            body_groups,
            eye_group,
            extra_materials,
        ),
    )
}

/// Require every selected body shader to resolve to identical source pixels.
fn body_texture_path(
    character: &CharacterAsset,
    body_groups: &[GroupAddressRequest],
    bindings: &BTreeMap<&str, &MaterialBinding>,
    input_dir: &Path,
) -> Result<PathBuf, PipelineError> {
    let mut candidates = BTreeMap::<String, PathBuf>::new();
    for address in body_groups {
        let group = character
            .parts
            .get(address.part_index)
            .and_then(
                |part| {
                    part.mesh
                        .groups
                        .get(address.group_index)
                },
            )
            .ok_or_else(|| PipelineError::new("body group disappeared"))?;
        let binding = bindings
            .get(
                group
                    .shader
                    .as_str(),
            )
            .ok_or_else(
                || {
                    PipelineError::new(
                        format!(
                            "body shader {} has no material binding",
                            group.shader
                        ),
                    )
                },
            )?;
        if let Some(file_name) = binding
            .texture_file_name
            .as_ref()
        {
            let _previous = candidates.insert(
                file_name.clone(),
                input_dir.join(file_name),
            );
        }
    }
    let mut selected = None;
    let mut selected_hash = None;
    for path in candidates.into_values() {
        let bytes = local_read_bytes(&path).map_err(
            |error| {
                PipelineError::new(
                    format!(
                        "body source texture read failed for {}: {error}",
                        path.display()
                    ),
                )
            },
        )?;
        let hash = digest_hex(&bytes);
        if selected_hash
            .as_ref()
            .is_some_and(|existing| existing != &hash)
        {
            return Err(
                PipelineError::new(
                    format!(
                        "character {} body shaders resolve to different \
                         source textures",
                        character.name
                    ),
                ),
            );
        }
        selected_hash = Some(hash);
        if selected.is_none() {
            selected = Some(path);
        }
    }
    selected
        .ok_or_else(|| PipelineError::new("body texture selection is empty"))
}

/// Resolve decoded animation member paths from one selected bank.
fn animation_member_paths(
    animation_package: Option<&PhaseThreePackageRow>,
    base_root: &Path,
) -> Result<Vec<PathBuf>, PipelineError> {
    let package = animation_package.ok_or_else(
        || PipelineError::new("character catalog animation package is missing"),
    )?;
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
                    "animation package {} has no decoded clips",
                    package.package_id
                ),
            ),
        );
    }
    Ok(paths)
}

/// Resolve the four shared eye-frame PNGs by exact portable file name.
fn shared_eye_frame_paths(
    index: &PhaseThreePackageIndex,
    base_root: &Path,
) -> Result<[PathBuf; 4], PipelineError> {
    let matches = index
        .packages()
        .iter()
        .filter(
            |package| {
                package.category == CHARACTERS_CATEGORY
                    && package.subcategory == CHARACTER_SHARED_SUBCATEGORY
            },
        )
        .collect::<Vec<_>>();
    let package = match matches.as_slice() {
        [package] => *package,
        [] => {
            return Err(
                PipelineError::new("shared character package is missing"),
            );
        }
        _ => {
            return Err(
                PipelineError::new("shared character package is ambiguous"),
            );
        }
    };
    let mut paths = BTreeMap::new();
    for member in package.members() {
        let Some(file_name) = Path::new(&member.path)
            .file_name()
            .and_then(|name| name.to_str())
        else {
            continue;
        };
        if [
            "eyeball.bmp.0.png",
            "eyeball.bmp.1.png",
            "eyeball.bmp.2.png",
            "eyeball.bmp.3.png",
        ]
        .contains(&file_name)
        {
            let _previous = paths.insert(
                file_name.to_owned(),
                base_root.join(&member.path),
            );
        }
    }
    [
        "eyeball.bmp.0.png",
        "eyeball.bmp.1.png",
        "eyeball.bmp.2.png",
        "eyeball.bmp.3.png",
    ]
    .map(
        |file_name| {
            paths
                .remove(file_name)
                .ok_or_else(
                    || {
                        PipelineError::new(
                            format!("shared eye frame is missing: {file_name}"),
                        )
                    },
                )
        },
    )
    .into_iter()
    .collect::<Result<Vec<_>, _>>()?
    .try_into()
    .map_err(
        |_paths: Vec<PathBuf>| PipelineError::new("eye frame count mismatch"),
    )
}

/// Prefer semantic-atlas output and fall back only on body-classifier failure.
fn prepare_with_source_fallback(
    request: &mut SemanticTextureRequest
) -> Result<
    (
        PreparedSemanticCharacter,
        &'static str,
    ),
    PipelineError,
> {
    match prepare_semantic_character(request) {
        Ok(prepared) => Ok(
            (
                prepared,
                "semantic-atlas",
            ),
        ),
        Err(SemanticTextureArtifactError::Body(_body_error)) => {
            "preserve-source".clone_into(&mut request.body_texture_mode);
            prepare_semantic_character(request)
                .map(
                    |prepared| {
                        (
                            prepared,
                            "preserve-source",
                        )
                    },
                )
                .map_err(
                    |error| {
                        PipelineError::new(
                            format!(
                                "source-preserving character preparation \
                                 failed: {error:?}"
                            ),
                        )
                    },
                )
        }
        Err(error) => Err(
            PipelineError::new(
                format!("semantic character preparation failed: {error:?}"),
            ),
        ),
    }
}

/// Count primitive groups, vertices, and triangles with checked arithmetic.
fn topology_counts(
    character: &CharacterAsset
) -> Result<[usize; 3], PipelineError> {
    let mut groups = 0_usize;
    let mut vertices = 0_usize;
    let mut triangles = 0_usize;
    for part in &character.parts {
        for group in &part
            .mesh
            .groups
        {
            groups = groups
                .checked_add(1)
                .ok_or_else(|| PipelineError::new("group count overflow"))?;
            vertices = vertices
                .checked_add(
                    group
                        .positions
                        .len(),
                )
                .ok_or_else(|| PipelineError::new("vertex count overflow"))?;
            triangles = triangles
                .checked_add(
                    group
                        .triangles
                        .len(),
                )
                .ok_or_else(|| PipelineError::new("triangle count overflow"))?;
        }
    }
    Ok(
        [
            groups, vertices, triangles,
        ],
    )
}

/// Require the binary writer summary to match the prepared aggregate exactly.
fn verify_summary(
    package: &PhaseThreePackageRow,
    summary: &CharacterBinaryFbxSummary,
    prepared: &PreparedSemanticCharacter,
    topology: [usize; 3],
) -> Result<(), PipelineError> {
    let [
        groups,
        _vertices,
        _triangles,
    ] = topology;
    if summary.bones
        != prepared
            .character
            .bones
            .len()
        || summary.geometries != groups
        || summary.animations
            != prepared
                .animations
                .len()
        || summary.animations == 0
        || summary.clusters == 0
        || summary.materials == 0
        || summary.textures == 0
    {
        return Err(
            PipelineError::new(
                format!(
                    "binary FBX summary is incomplete for {}: {summary:?}",
                    package.package_id
                ),
            ),
        );
    }
    Ok(())
}

/// Render one deterministic catalog entry after artifact verification.
#[expect(
    clippy::too_many_lines,
    reason = "Artifact checks and row rendering form one evidence transaction."
)]
fn catalog_entry(
    package: &PhaseThreePackageRow,
    animation_package: Option<&PhaseThreePackageRow>,
    package_dir: &Path,
    prepared: &PreparedSemanticCharacter,
    summary: &CharacterBinaryFbxSummary,
    topology: [usize; 3],
    selected_mode: &str,
) -> Result<Value, PipelineError> {
    let fbx_file_name = format!(
        "{}.fbx",
        prepared
            .artifacts
            .summary
            .character_id
    );
    let fbx_path = package_dir.join(&fbx_file_name);
    let fbx_bytes = local_read_bytes(&fbx_path).map_err(
        |error| {
            PipelineError::new(format!("FBX verification read failed: {error}"))
        },
    )?;
    verify_external_binary_fbx(
        package, &fbx_bytes,
    )?;
    let texture_dir = package_dir.join("textures");
    let mut texture_files = std::fs::read_dir(&texture_dir)
        .map_err(
            |error| {
                PipelineError::new(
                    format!("texture directory read failed: {error}"),
                )
            },
        )?
        .map(
            |entry| {
                entry.map_err(
                    |error| {
                        PipelineError::new(
                            format!("texture entry read failed: {error}"),
                        )
                    },
                )
            },
        )
        .collect::<Result<Vec<_>, _>>()?;
    texture_files.sort_by_key(std::fs::DirEntry::file_name);
    let mut texture_rows = Vec::new();
    for entry in texture_files {
        let path = entry.path();
        if !path.is_file() {
            return Err(
                PipelineError::new(
                    format!(
                        "unexpected texture directory entry: {}",
                        path.display()
                    ),
                ),
            );
        }
        let bytes = local_read_bytes(&path).map_err(
            |error| PipelineError::new(format!("texture read failed: {error}")),
        )?;
        let file_name = path
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or_else(
                || PipelineError::new("texture file name is not UTF-8"),
            )?;
        texture_rows.push(
            json!({
                "path": format!("{}/textures/{file_name}", package.package_id),
                "bytes": bytes.len(),
                "sha256": digest_hex(&bytes),
            }),
        );
    }
    for binding in &prepared.materials {
        if let Some(file_name) = binding
            .texture_file_name
            .as_deref()
            && !texture_dir
                .join(file_name)
                .is_file()
        {
            return Err(
                PipelineError::new(
                    format!(
                        "external texture reference is missing for {}: \
                         {file_name}",
                        package.package_id
                    ),
                ),
            );
        }
    }
    let texture_plan = local_read_bytes(&package_dir.join("texture-plan.json"))
        .map_err(
            |error| {
                PipelineError::new(format!("texture-plan read failed: {error}"))
            },
        )?;
    let [
        group_count,
        vertex_count,
        triangle_count,
    ] = topology;
    Ok(
        json!({
            "package_id": package.package_id,
            "subcategory": package.subcategory,
            "animation_package_id": animation_package
                .map(|row| &row.package_id),
            "body_mode": selected_mode,
            "eye_modeled": prepared.artifacts.eye_layer_pngs.is_some(),
            "eye_profile_sha256": prepared.artifacts.eye_profile_sha256,
            "fbx": {
                "path": format!("{}/{fbx_file_name}", package.package_id),
                "bytes": fbx_bytes.len(),
                "sha256": digest_hex(&fbx_bytes),
                "version": 7700_i32,
                "texture_storage": "external",
                "packed_images": 0_i32,
            },
            "texture_plan": {
                "path": format!("{}/texture-plan.json", package.package_id),
                "bytes": texture_plan.len(),
                "sha256": digest_hex(&texture_plan),
            },
            "textures": texture_rows,
            "source_and_output_topology": {
                "parts": prepared.character.parts.len(),
                "groups": group_count,
                "vertices": vertex_count,
                "triangles": triangle_count,
                "bones": prepared.character.bones.len(),
                "preserved": true,
            },
            "writer_summary": {
                "geometries": summary.geometries,
                "bones": summary.bones,
                "clusters": summary.clusters,
                "materials": summary.materials,
                "textures": summary.textures,
                "animations": summary.animations,
            },
        }),
    )
}

/// Verify canonical binary version, external textures, and no embedded payload.
fn verify_external_binary_fbx(
    package: &PhaseThreePackageRow,
    bytes: &[u8],
) -> Result<(), PipelineError> {
    const MAGIC: &[u8] = b"Kaydara FBX Binary  \0\x1a\0";
    let version = bytes
        .get(23..27)
        .and_then(|slice| <[u8; 4]>::try_from(slice).ok())
        .map(u32::from_le_bytes);
    if bytes.get(..MAGIC.len()) != Some(MAGIC)
        || version != Some(7700)
        || bytes
            .windows(b"Content".len())
            .any(|window| window == b"Content")
    {
        return Err(
            PipelineError::new(
                format!(
                    "binary FBX contract verification failed for {}",
                    package.package_id
                ),
            ),
        );
    }
    Ok(())
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

/// Materialize one optional Maya helper and append its capability evidence.
fn materialize_optional_maya_helper(
    enabled: bool,
    request: &MayaHelperRequest<'_>,
    capabilities: &mut Vec<CapabilityItem>,
) -> Result<Option<MayaHelperArtifact>, PipelineError> {
    let Some(output) = write_optional_maya_helper(
        enabled, request,
    )?
    else {
        return Ok(None);
    };
    capabilities.push(output.capability);
    Ok(Some(output.artifact))
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

/// Report the selected texture-storage policy explicitly.
fn texture_storage_capability(embed_textures: bool) -> CapabilityItem {
    if embed_textures {
        return CapabilityItem {
            id: "derived:texture-storage".to_owned(),
            outcome: "converted",
            reason: concat!(
                "legacy compatibility mode stores PNG payloads in ",
                "Video.Content and does not publish sibling textures",
            )
            .to_owned(),
        };
    }
    CapabilityItem {
        id: "derived:texture-storage".to_owned(),
        outcome: "converted",
        reason: concat!(
            "canonical mode omits Video.Content and references immutable ",
            "sibling files under textures/",
        )
        .to_owned(),
    }
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
            "p3d-mesh" => classified
                .meshes
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
    let mesh_paths: Vec<PathBuf> = members
        .meshes
        .iter()
        .map(|member| base_root.join(&member.path))
        .collect();
    let mesh_path_refs: Vec<&Path> = mesh_paths
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
        &mesh_path_refs,
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

/// Resolve the deterministic animation-set row for one character presentation.
fn resolve_animation_package<'index>(
    index: &'index PhaseThreePackageIndex,
    package: &PhaseThreePackageRow,
) -> Result<Option<&'index PhaseThreePackageRow>, PipelineError> {
    let candidates = animation_subcategory_candidates(&package.subcategory);
    for target in candidates {
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
            [] => {}
            [animation_package] => return Ok(Some(*animation_package)),
            _ => {
                return Err(
                    PipelineError::new(
                        format!(
                            "character package {} has multiple animation-set \
                             rows for subcategory {target}",
                            package.package_id
                        ),
                    ),
                );
            }
        }
    }
    Err(
        PipelineError::new(
            format!(
                "character package {} has no identity-specific or general \
                 animation-set row",
                package.package_id
            ),
        ),
    )
}

/// Return identity-specific then general animation subcategories in priority
/// order.
fn animation_subcategory_candidates(subcategory: &str) -> Vec<String> {
    let identity_root = subcategory
        .strip_suffix("/base-model")
        .or_else(
            || {
                subcategory
                    .split_once("/costume/")
                    .map(|(root, _costume)| root)
            },
        );
    let mut candidates = Vec::with_capacity(2);
    if let Some(root) = identity_root {
        candidates.push(format!("{root}/animation-set"));
    }
    if candidates
        .first()
        .is_none_or(
            |candidate| candidate != GENERAL_CHARACTER_ANIMATION_SUBCATEGORY,
        )
    {
        candidates.push(GENERAL_CHARACTER_ANIMATION_SUBCATEGORY.to_owned());
    }
    candidates
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
    let accepted_file_names = if expected_file_name == "char_swatches.png" {
        vec![
            expected_file_name,
            "char_swatches_lit.png".to_owned(),
        ]
    } else {
        vec![expected_file_name]
    };
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
                        .is_some_and(
                            |name| {
                                accepted_file_names
                                    .iter()
                                    .any(|item| item == name)
                            },
                        )
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

/// Immutable output paths and package identity for one character export.
struct CharacterExportTarget<'path> {
    /// Private decoded-texture staging directory.
    texture_staging_dir: &'path Path,
    /// Published external-texture directory.
    texture_dir: &'path Path,
    /// Final binary FBX path.
    fbx_path: &'path Path,
    /// Stable package identity used in diagnostics.
    package_id: &'path str,
}

/// Dispatch one character export through the selected texture-storage policy.
fn serialize_selected_texture_storage(
    character: &CharacterAsset,
    materials: &[MaterialBinding],
    animations: &[AnimationClip],
    target: &CharacterExportTarget<'_>,
    embed_textures: bool,
) -> Result<CharacterBinaryFbxSummary, PipelineError> {
    if embed_textures {
        return write_embedded_character_fbx(
            character,
            materials,
            animations,
            target.texture_staging_dir,
            target.fbx_path,
            target.package_id,
        );
    }
    write_external_character_fbx(
        character,
        materials,
        animations,
        target.texture_staging_dir,
        target.texture_dir,
        target.fbx_path,
        target.package_id,
    )
}

/// Publish sibling textures and write an external-reference FBX.
fn write_external_character_fbx(
    character: &CharacterAsset,
    materials: &[MaterialBinding],
    animations: &[AnimationClip],
    texture_staging_dir: &Path,
    texture_dir: &Path,
    fbx_path: &Path,
    package_id: &str,
) -> Result<CharacterBinaryFbxSummary, PipelineError> {
    std::fs::rename(
        texture_staging_dir,
        texture_dir,
    )
    .map_err(
        |error| {
            PipelineError::new(
                format!(
                    "failed to publish external texture directory {}: {error}",
                    texture_dir.display()
                ),
            )
        },
    )?;
    let export_result = write_binary_character_fbx(
        character, materials, animations, fbx_path,
    )
    .map_err(
        |error| {
            let context = "character FBX serialization failed";
            PipelineError::new(format!("{context} for {package_id}: {error:?}"))
        },
    );
    match export_result {
        Ok(summary) => Ok(summary),
        Err(error) => {
            remove_texture_staging_dir(texture_dir)?;
            Err(error)
        }
    }
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
        write_binary_character_fbx_embedded(
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

/// Reject one already published external texture directory.
fn ensure_texture_output_absent(path: &Path) -> Result<(), PipelineError> {
    if !path.exists() {
        return Ok(());
    }
    Err(
        PipelineError::new(
            format!(
                "external texture output already exists: {}",
                path.display()
            ),
        ),
    )
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
        &members.meshes,
        "converted",
        "composite prop exported as a rigid one-bone skinned part",
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
    use super::{
        GENERAL_CHARACTER_ANIMATION_SUBCATEGORY,
        animation_subcategory_candidates, deferred_material_identity,
        normalized_texture_png_file_name,
    };

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

    #[test]
    fn character_animation_candidates_prefer_identity_specific_banks() {
        assert_eq!(
            animation_subcategory_candidates("characters/apu/base-model"),
            vec![
                "characters/apu/animation-set".to_owned(),
                GENERAL_CHARACTER_ANIMATION_SUBCATEGORY.to_owned(),
            ]
        );
        assert_eq!(
            animation_subcategory_candidates("characters/lisa/costume/cool"),
            vec![
                "characters/lisa/animation-set".to_owned(),
                GENERAL_CHARACTER_ANIMATION_SUBCATEGORY.to_owned(),
            ]
        );
    }

    #[test]
    fn character_animation_candidates_use_general_bank_for_other_models() {
        assert_eq!(
            animation_subcategory_candidates("characters/krusty/base-model"),
            vec![
                "characters/krusty/animation-set".to_owned(),
                GENERAL_CHARACTER_ANIMATION_SUBCATEGORY.to_owned(),
            ]
        );
        assert_eq!(
            animation_subcategory_candidates("characters/boy1/crowd-model"),
            vec![GENERAL_CHARACTER_ANIMATION_SUBCATEGORY.to_owned()]
        );
        assert_eq!(
            animation_subcategory_candidates("characters/homer/base-model"),
            vec![GENERAL_CHARACTER_ANIMATION_SUBCATEGORY.to_owned()]
        );
    }
}
