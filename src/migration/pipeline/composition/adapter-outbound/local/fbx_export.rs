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
//   - Fbx export outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Fbx export outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for pipeline.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Fbx export outbound adapter.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use fbx::adapters::driven::binary_character_writer::{
    CharacterBinaryFbxSummary, write_binary_character_fbx,
    write_binary_model_fbx,
};
use fbx::adapters::driven::decoded_component_source::{
    DecodedComponentError, DecodedComponentSource, read_indexed_mesh,
};
use fbx::adapters::driven::decoded_skin_source::load_character;
use fbx::adapters::driven::semantic_character_texture::request::{
    ExtraMaterialRequest, GroupAddressRequest,
};
use fbx::adapters::driven::semantic_character_texture::{
    PreparedSemanticCharacter, SemanticTextureArtifactError,
    SemanticTextureRequest, prepare_semantic_character,
    publish_prepared_semantic_character,
};
use fbx::domain::character::CharacterAsset;
use fbx::domain::mesh::MeshAsset;
use fbx::domain::texture::MaterialBinding;
use fbx::ports::component_source::ComponentSource as _;
use schoenwald_filesystem::adapters::driving::local::{
    create_dir_all as local_create_dir_all, file_len as local_file_len,
    path_kind as local_path_kind, read_bytes as local_read_bytes,
    write_text as local_write_text,
};
use schoenwald_filesystem::domain::PathKind;
use serde_json::{Value, json};
use shar_sha256::digest_hex;

use super::fbx_manifest::stable_file_stem;
use crate::domain::package::{
    ConversionFamily, FbxTargetKind, PackageRole, PhaseThreePackageIndex,
    PhaseThreePackageMember, PhaseThreePackagePlanner, PhaseThreePackageRow,
    PhaseThreePackageSelector,
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

/// Export one selected canonical model package through atomic publication.
///
/// Static and self-contained skeletal sources are built and verified below a
/// hidden sibling directory before one rename publishes the package. Composite
/// packages fail before any output path is created.
///
/// # Errors
///
/// Returns an error when the package cannot be resolved, requires semantic
/// splitting, either transaction path already exists, or conversion,
/// verification, cleanup, or atomic publication fails.
pub(super) fn export_fbx_package(
    index_path: &Path,
    selector: &PhaseThreePackageSelector,
    output_dir: &Path,
    base_root: &Path,
    _options: FbxExportOptions,
) -> Result<StageReport, PipelineError> {
    let index = PhaseThreePackageIndex::read_for_unreal(index_path)
        .map_err(|error| PipelineError::new(error.to_string()))?;
    let package = selector
        .resolve(&index)
        .map_err(|error| PipelineError::new(error.to_string()))?;
    let plan = PhaseThreePackagePlanner::plan(package);
    let target_kind =
        plan.fbx
            .as_ref()
            .map(|fbx| fbx.target_kind)
            .ok_or_else(|| {
                PipelineError::new(format!(
                    "selected package is not an FBX model package: {}",
                    package.package_id
                ))
            })?;
    if target_kind == FbxTargetKind::SemanticSplit {
        return Err(PipelineError::new(format!(
            "package requires semantic splitting before FBX export: {}",
            package.package_id
        )));
    }
    export_transactional_package(
        &index,
        package,
        target_kind,
        output_dir,
        base_root,
    )
}

/// Export one package directly into the complete catalog staging layout.
///
/// The caller owns the root transaction. This helper only owns the package
/// directory it creates and deliberately strips the one-off capability report
/// so the catalog inventory contains only promotable FBX/PNG artifacts.
pub(super) fn export_catalog_package(
    index: &PhaseThreePackageIndex,
    package: &PhaseThreePackageRow,
    packages_root: &Path,
    base_root: &Path,
) -> Result<(), PipelineError> {
    let target_kind = PhaseThreePackagePlanner::plan(package)
        .fbx
        .as_ref()
        .map(|fbx| fbx.target_kind)
        .ok_or_else(|| {
            PipelineError::new(format!(
                "catalog package is not an FBX model package: {}",
                package.package_id
            ))
        })?;
    if target_kind == FbxTargetKind::SemanticSplit {
        return Err(PipelineError::new(format!(
            "catalog package still requires semantic splitting: {}",
            package.package_id
        )));
    }
    let package_name = package.package_id.replace('-', "_");
    let package_dir = packages_root.join(&package_name);
    ensure_transaction_path_missing(&package_dir, "FBX catalog package")?;
    local_create_dir_all(&package_dir).map_err(|error| {
        fbx_io_error("create FBX catalog package", &error)
    })?;
    let reported_package_dir = PathBuf::from("packages").join(&package_name);
    let result = match target_kind {
        FbxTargetKind::StaticMesh => export_lossless_static_package(
            index,
            package,
            &package_dir,
            &reported_package_dir,
            base_root,
        ),
        FbxTargetKind::SkeletalMesh => export_single_skeletal_package(
            index,
            package,
            &package_dir,
            &reported_package_dir,
            base_root,
        ),
        FbxTargetKind::SemanticSplit => {
            return Err(PipelineError::new(
                "catalog package still requires semantic splitting",
            ));
        },
    };
    let _stage_report = result?;
    let generated_stem = stable_file_stem(&package.subcategory);
    let generated_fbx = package_dir.join(format!("{generated_stem}.fbx"));
    let canonical_fbx = package_dir.join(format!("{package_name}.fbx"));
    if generated_fbx != canonical_fbx {
        std::fs::rename(&generated_fbx, &canonical_fbx)
            .map_err(|error| fbx_io_error("normalize catalog FBX name", &error))?;
    }
    let report = package_dir.join("capability-report.json");
    std::fs::remove_file(&report)
        .map_err(|error| fbx_io_error("remove catalog diagnostic report", &error))?;
    Ok(())
}

/// Build one package below owned staging and publish it with one rename.
fn export_transactional_package(
    index: &PhaseThreePackageIndex,
    package: &PhaseThreePackageRow,
    target_kind: FbxTargetKind,
    output_dir: &Path,
    base_root: &Path,
) -> Result<StageReport, PipelineError> {
    let destination = output_dir.join(&package.package_id);
    let reported_package_dir = PathBuf::from(&package.package_id);
    let staging = single_package_staging_path(output_dir, &package.package_id);
    ensure_transaction_path_missing(&destination, "FBX package output")?;
    ensure_transaction_path_missing(&staging, "FBX package staging")?;
    local_create_dir_all(&staging)
        .map_err(|error| fbx_io_error("create FBX package staging", &error))?;
    let result = match target_kind {
        FbxTargetKind::StaticMesh => export_lossless_static_package(
            index,
            package,
            &staging,
            &reported_package_dir,
            base_root,
        ),
        FbxTargetKind::SkeletalMesh => export_single_skeletal_package(
            index,
            package,
            &staging,
            &reported_package_dir,
            base_root,
        ),
        FbxTargetKind::SemanticSplit => Err(PipelineError::new(
            "semantic split reached the FBX publication transaction",
        )),
    };
    let report = match result {
        Ok(report) => report,
        Err(error) => {
            return Err(cleanup_after_transaction_failure(&staging, error));
        },
    };
    if let Err(error) = std::fs::rename(&staging, &destination) {
        let publication_error = fbx_io_error("publish FBX package", &error);
        return Err(cleanup_after_transaction_failure(
            &staging,
            publication_error,
        ));
    }
    Ok(report)
}

/// Preserve the primary transaction failure while attempting owned cleanup.
fn cleanup_after_transaction_failure(
    staging: &Path,
    primary: PipelineError,
) -> PipelineError {
    match cleanup_owned_staging(staging) {
        Ok(()) => primary,
        Err(cleanup) => PipelineError::new(format!(
            "{primary}; FBX package staging cleanup also failed: {cleanup}"
        )),
    }
}

/// Derive one hidden sibling staging path from a canonical package identity.
fn single_package_staging_path(output_dir: &Path, package_id: &str) -> PathBuf {
    output_dir.join(format!(".{package_id}.fbx-staging"))
}

/// Reject every pre-existing transaction path, including special file kinds.
fn ensure_transaction_path_missing(
    path: &Path,
    label: &str,
) -> Result<(), PipelineError> {
    match local_path_kind(path)
        .map_err(|error| fbx_io_error("inspect FBX transaction path", &error))?
    {
        PathKind::Missing => Ok(()),
        kind => Err(PipelineError::new(format!(
            "{label} already exists as {kind:?}"
        ))),
    }
}

/// Remove only the hidden staging directory owned by the current transaction.
fn cleanup_owned_staging(staging: &Path) -> Result<(), PipelineError> {
    match local_path_kind(staging)
        .map_err(|error| fbx_io_error("inspect FBX package staging", &error))?
    {
        PathKind::Missing => Ok(()),
        PathKind::Directory => std::fs::remove_dir_all(staging)
            .map_err(|error| fbx_io_error("clean FBX package staging", &error)),
        kind => Err(PipelineError::new(format!(
            "FBX package staging changed kind before cleanup: {kind:?}"
        ))),
    }
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
    let character = build_character(package, &members, base_root)?;
    let animation_package = resolve_animation_package(index, package)?;
    let input_dir = output_root
        .join(".texture-inputs")
        .join(&package.package_id);
    remove_texture_staging_dir(&input_dir)?;
    let result = (|| {
        let (materials, _capabilities) =
            resolve_materials(index, package, &members, base_root, &input_dir)?;
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
            || prepared.character.bones.len() != character.bones.len()
        {
            return Err(PipelineError::new(format!(
                "semantic preparation changed topology or rig for {}",
                package.package_id
            )));
        }
        let package_dir = output_root.join(&package.package_id);
        let summary =
            publish_prepared_semantic_character(&package_dir, &prepared)
                .map_err(|error| {
                    PipelineError::new(format!(
                        "prepared character publication failed for {}: {error}",
                        package.package_id
                    ))
                })?;
        verify_summary(package, &summary, &prepared, source_topology)?;
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
        .map(|binding| (binding.material_name.as_str(), binding))
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
            .ok_or_else(|| {
                PipelineError::new(format!(
                    "material {material_name} has no resolved binding \
                             for {}",
                    package.package_id
                ))
            })?;
        if let Some(output_file_name) = binding.texture_file_name.as_ref() {
            extra_materials.push(ExtraMaterialRequest {
                material_name,
                texture_path: input_dir.join(output_file_name),
                output_file_name: output_file_name.clone(),
            });
        } else {
            untextured_materials.push(material_name);
        }
    }
    let skeleton_path = members
        .skeletons
        .first()
        .map(|member| base_root.join(&member.path))
        .ok_or_else(|| {
            PipelineError::new("character catalog package lost skeleton")
        })?;
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
    let general_animation_paths =
        animation_member_paths(animation_package, base_root)?;
    let eye_frame_paths = eye_group
        .map(|_address| shared_eye_frame_paths(index, base_root))
        .transpose()?;
    Ok(SemanticTextureRequest {
        character_name: package.package_id.clone(),
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
        body_atlas_background: [128, 128, 128, 255],
        eye_output_size: 64,
        extra_materials,
        untextured_materials,
    })
}

/// Classify body, eye, and accessory groups from exact shader identities.
fn semantic_group_ownership(
    character: &CharacterAsset,
) -> Result<SemanticGroupOwnership, PipelineError> {
    let mut body_groups = Vec::new();
    let mut eye_group = None;
    let mut extra_materials = BTreeSet::new();
    for (part_index, part) in character.parts.iter().enumerate() {
        for (group_index, group) in part.mesh.groups.iter().enumerate() {
            let address = GroupAddressRequest { part_index, group_index };
            let shader = group.shader.to_ascii_lowercase();
            if shader.contains("char_swatches") {
                body_groups.push(address);
            } else if shader.contains("eyeball") {
                if eye_group.replace(address).is_some() {
                    return Err(PipelineError::new(format!(
                        "character {} has multiple eye groups",
                        character.name
                    )));
                }
            } else {
                let _inserted = extra_materials.insert(group.shader.clone());
            }
        }
    }
    if body_groups.is_empty() {
        return Err(PipelineError::new(format!(
            "character {} has no body swatch group",
            character.name
        )));
    }
    Ok((body_groups, eye_group, extra_materials))
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
            .and_then(|part| part.mesh.groups.get(address.group_index))
            .ok_or_else(|| PipelineError::new("body group disappeared"))?;
        let binding = bindings.get(group.shader.as_str()).ok_or_else(|| {
            PipelineError::new(format!(
                "body shader {} has no material binding",
                group.shader
            ))
        })?;
        if let Some(file_name) = binding.texture_file_name.as_ref() {
            let _previous =
                candidates.insert(file_name.clone(), input_dir.join(file_name));
        }
    }
    let mut selected = None;
    let mut selected_hash = None;
    for path in candidates.into_values() {
        let bytes = local_read_bytes(&path).map_err(|error| {
            PipelineError::new(format!(
                "body source texture read failed for {}: {error}",
                path.display()
            ))
        })?;
        let hash = digest_hex(&bytes);
        if selected_hash
            .as_ref()
            .is_some_and(|existing| existing != &hash)
        {
            return Err(PipelineError::new(format!(
                "character {} body shaders resolve to different \
                         source textures",
                character.name
            )));
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
    let package = animation_package.ok_or_else(|| {
        PipelineError::new("character catalog animation package is missing")
    })?;
    let paths = package
        .members()
        .iter()
        .filter(|member| {
            member.kind == "p3d-animation"
                && member.source_chunk_kind == "animation"
        })
        .map(|member| base_root.join(&member.path))
        .collect::<Vec<_>>();
    if paths.is_empty() {
        return Err(PipelineError::new(format!(
            "animation package {} has no decoded clips",
            package.package_id
        )));
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
        .filter(|package| {
            package.category == CHARACTERS_CATEGORY
                && package.subcategory == CHARACTER_SHARED_SUBCATEGORY
        })
        .collect::<Vec<_>>();
    let package = match matches.as_slice() {
        [package] => *package,
        [] => {
            return Err(PipelineError::new(
                "shared character package is missing",
            ));
        },
        _ => {
            return Err(PipelineError::new(
                "shared character package is ambiguous",
            ));
        },
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
            let _previous = paths
                .insert(file_name.to_owned(), base_root.join(&member.path));
        }
    }
    [
        "eyeball.bmp.0.png",
        "eyeball.bmp.1.png",
        "eyeball.bmp.2.png",
        "eyeball.bmp.3.png",
    ]
    .map(|file_name| {
        paths.remove(file_name).ok_or_else(|| {
            PipelineError::new(format!(
                "shared eye frame is missing: {file_name}"
            ))
        })
    })
    .into_iter()
    .collect::<Result<Vec<_>, _>>()?
    .try_into()
    .map_err(|_paths: Vec<PathBuf>| {
        PipelineError::new("eye frame count mismatch")
    })
}

/// Prefer semantic-atlas output and fall back only on body-classifier failure.
fn prepare_with_source_fallback(
    request: &mut SemanticTextureRequest,
) -> Result<(PreparedSemanticCharacter, &'static str), PipelineError> {
    match prepare_semantic_character(request) {
        Ok(prepared) => Ok((prepared, "semantic-atlas")),
        Err(SemanticTextureArtifactError::Body(_body_error)) => {
            "preserve-source".clone_into(&mut request.body_texture_mode);
            prepare_semantic_character(request)
                .map(|prepared| (prepared, "preserve-source"))
                .map_err(|error| {
                    PipelineError::new(format!(
                        "source-preserving character preparation \
                                 failed: {error:?}"
                    ))
                })
        },
        Err(error) => Err(PipelineError::new(format!(
            "semantic character preparation failed: {error:?}"
        ))),
    }
}

/// Count primitive groups, vertices, and triangles with checked arithmetic.
fn topology_counts(
    character: &CharacterAsset,
) -> Result<[usize; 3], PipelineError> {
    let mut groups = 0_usize;
    let mut vertices = 0_usize;
    let mut triangles = 0_usize;
    for part in &character.parts {
        for group in &part.mesh.groups {
            groups = groups
                .checked_add(1)
                .ok_or_else(|| PipelineError::new("group count overflow"))?;
            vertices = vertices
                .checked_add(group.positions.len())
                .ok_or_else(|| PipelineError::new("vertex count overflow"))?;
            triangles = triangles
                .checked_add(group.triangles.len())
                .ok_or_else(|| PipelineError::new("triangle count overflow"))?;
        }
    }
    Ok([groups, vertices, triangles])
}

/// Require the binary writer summary to match the prepared aggregate exactly.
fn verify_summary(
    package: &PhaseThreePackageRow,
    summary: &CharacterBinaryFbxSummary,
    prepared: &PreparedSemanticCharacter,
    topology: [usize; 3],
) -> Result<(), PipelineError> {
    let [groups, _vertices, _triangles] = topology;
    if summary.bones != prepared.character.bones.len()
        || summary.geometries != groups
        || summary.animations != prepared.animations.len()
        || summary.animations == 0
        || summary.clusters == 0
        || summary.materials == 0
        || summary.textures == 0
    {
        return Err(PipelineError::new(format!(
            "binary FBX summary is incomplete for {}: {summary:?}",
            package.package_id
        )));
    }
    Ok(())
}

/// Render one deterministic catalog entry after artifact verification.
fn catalog_entry(
    package: &PhaseThreePackageRow,
    animation_package: Option<&PhaseThreePackageRow>,
    package_dir: &Path,
    prepared: &PreparedSemanticCharacter,
    summary: &CharacterBinaryFbxSummary,
    topology: [usize; 3],
    selected_mode: &str,
) -> Result<Value, PipelineError> {
    let fbx_file_name =
        format!("{}.fbx", prepared.artifacts.summary.character_id);
    let fbx_path = package_dir.join(&fbx_file_name);
    let fbx_bytes = local_read_bytes(&fbx_path).map_err(|error| {
        PipelineError::new(format!("FBX verification read failed: {error}"))
    })?;
    verify_external_binary_fbx(package, &fbx_bytes)?;
    let texture_dir = package_dir.join("textures");
    let mut texture_files = std::fs::read_dir(&texture_dir)
        .map_err(|error| {
            PipelineError::new(format!(
                "texture directory read failed: {error}"
            ))
        })?
        .map(|entry| {
            entry.map_err(|error| {
                PipelineError::new(format!(
                    "texture entry read failed: {error}"
                ))
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    texture_files.sort_by_key(std::fs::DirEntry::file_name);
    let mut texture_rows = Vec::new();
    for entry in texture_files {
        let path = entry.path();
        if !path.is_file() {
            return Err(PipelineError::new(format!(
                "unexpected texture directory entry: {}",
                path.display()
            )));
        }
        let bytes = local_read_bytes(&path).map_err(|error| {
            PipelineError::new(format!("texture read failed: {error}"))
        })?;
        let file_name =
            path.file_name().and_then(|name| name.to_str()).ok_or_else(
                || PipelineError::new("texture file name is not UTF-8"),
            )?;
        texture_rows.push(json!({
            "path": format!("{}/textures/{file_name}", package.package_id),
            "bytes": bytes.len(),
            "sha256": digest_hex(&bytes),
        }));
    }
    for binding in &prepared.materials {
        if let Some(file_name) = binding.texture_file_name.as_deref()
            && !texture_dir.join(file_name).is_file()
        {
            return Err(PipelineError::new(format!(
                "external texture reference is missing for {}: \
                         {file_name}",
                package.package_id
            )));
        }
    }
    let texture_plan = local_read_bytes(&package_dir.join("texture-plan.json"))
        .map_err(|error| {
            PipelineError::new(format!("texture-plan read failed: {error}"))
        })?;
    let [group_count, vertex_count, triangle_count] = topology;
    Ok(json!({
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
    }))
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
        return Err(PipelineError::new(format!(
            "binary FBX contract verification failed for {}",
            package.package_id
        )));
    }
    Ok(())
}

/// Export one static package whose complete semantic surface fits FBX geometry.
fn export_lossless_static_package(
    index: &PhaseThreePackageIndex,
    package: &PhaseThreePackageRow,
    package_dir: &Path,
    reported_package_dir: &Path,
    base_root: &Path,
) -> Result<StageReport, PipelineError> {
    let texture_dir = package_dir.join("textures");
    let meshes = load_indexed_static_meshes(package, base_root)?;
    let mesh_refs = meshes.iter().collect::<Vec<_>>();
    let materials = resolve_indexed_package_materials(
        index,
        package,
        &mesh_refs,
        base_root,
        &texture_dir,
    )?;
    let asset_name = stable_file_stem(&package.subcategory);
    let fbx_path = package_dir.join(format!("{asset_name}.fbx"));
    let summary =
        write_binary_model_fbx(&asset_name, &meshes, &materials, &fbx_path)
            .map_err(|_error| {
                PipelineError::new(format!(
                    "static FBX serialization failed for {}",
                    package.package_id
                ))
            })?;
    let fbx_bytes = local_read_bytes(&fbx_path).map_err(|error| {
        fbx_io_error("read static FBX for verification", &error)
    })?;
    verify_external_binary_fbx(package, &fbx_bytes)?;
    let report_path = package_dir.join("capability-report.json");
    write_capability_report(
        &report_path,
        &package.package_id,
        static_member_capability_items(package),
    )?;
    stage_report(
        package,
        &summary,
        &fbx_path,
        &report_path,
        &reported_package_dir.join(format!("{asset_name}.fbx")),
    )
}

/// Load static meshes only from exact physical paths published by the index.
fn load_indexed_static_meshes(
    package: &PhaseThreePackageRow,
    base_root: &Path,
) -> Result<Vec<MeshAsset>, PipelineError> {
    package
        .members()
        .iter()
        .filter(|member| member.role == PackageRole::Model)
        .map(|member| {
            read_indexed_mesh(&base_root.join(&member.path)).map_err(|_error| {
                PipelineError::new(format!(
                    "indexed mesh decode failed for {} member {}",
                    package.package_id, member.id
                ))
            })
        })
        .collect()
}

/// Resolve exact package-index shaders and require exact mesh usage coverage.
fn resolve_indexed_package_materials(
    index: &PhaseThreePackageIndex,
    package: &PhaseThreePackageRow,
    meshes: &[&MeshAsset],
    base_root: &Path,
    texture_staging_dir: &Path,
) -> Result<Vec<MaterialBinding>, PipelineError> {
    let package_root = base_root.join(&package.package_root);
    let source = DecodedComponentSource::new(
        package_root,
        texture_staging_dir.to_path_buf(),
    );
    let mut declared = BTreeMap::new();
    for member in package
        .members()
        .iter()
        .filter(|member| member.role == PackageRole::Material)
    {
        let shader_path = base_root.join(&member.path);
        let binding = match source.resolve_indexed_material(&shader_path) {
            Ok(binding) => binding,
            Err(DecodedComponentError::MissingTexture {
                shader,
                texture,
                ..
            }) => {
                let Some((_owner, texture_member)) =
                    resolve_shared_texture_member(index, &texture)?
                else {
                    return Err(PipelineError::new(format!(
                        concat!(
                            "indexed shader member {} ({}) has no unique ",
                            "package-index PNG for {}"
                        ),
                        member.id, shader, texture
                    )));
                };
                source
                    .resolve_indexed_material_with_external_texture(
                        &shader_path,
                        &base_root.join(&texture_member.path),
                    )
                    .map_err(|_error| {
                        PipelineError::new(format!(
                            concat!(
                                "indexed shared texture resolution failed for ",
                                "{} member {}"
                            ),
                            package.package_id, member.id
                        ))
                    })?
            },
            Err(_error) => {
                return Err(PipelineError::new(format!(
                    "indexed shader decode failed for {} member {}",
                    package.package_id, member.id
                )));
            },
        };
        let key = binding.material_name.to_ascii_lowercase();
        if declared.insert(key, binding).is_some() {
            return Err(PipelineError::new(format!(
                "duplicate authored shader identity in {}",
                package.package_id
            )));
        }
    }
    let used = meshes
        .iter()
        .flat_map(|mesh| mesh.groups.iter())
        .map(|group| group.shader.to_ascii_lowercase())
        .collect::<BTreeSet<_>>();
    let declared_ids = declared.keys().cloned().collect::<BTreeSet<_>>();
    if used != declared_ids {
        return Err(PipelineError::new(format!(
            "package shader membership does not exactly cover mesh usage for {}",
            package.package_id
        )));
    }
    Ok(declared.into_values().collect())
}

/// Report every member in one exact lossless static package.
fn static_member_capability_items(
    package: &PhaseThreePackageRow,
) -> Vec<CapabilityItem> {
    package
        .members()
        .iter()
        .map(|member| {
            let (outcome, reason) = match member.role {
                PackageRole::Model => (
                    "converted",
                    "indexed mesh exported with authored topology and surfaces",
                ),
                PackageRole::Material => (
                    "converted",
                    "indexed shader exported as an FBX material binding",
                ),
                PackageRole::Texture
                    if member.source_chunk_kind == "texture" =>
                {
                    (
                        "converted",
                        "indexed PNG published as an external FBX texture",
                    )
                },
                PackageRole::Texture | PackageRole::Metadata => (
                    "preserved-as-metadata",
                    "source evidence retained outside the interchange payload",
                ),
                _ => ("deferred", "unsupported static-package evidence"),
            };
            CapabilityItem {
                id: member.id.clone(),
                outcome,
                reason: reason.to_owned(),
            }
        })
        .collect()
}

/// Export one self-contained skeletal mesh without companion animation banks.
fn export_single_skeletal_package(
    index: &PhaseThreePackageIndex,
    package: &PhaseThreePackageRow,
    package_dir: &Path,
    reported_package_dir: &Path,
    base_root: &Path,
) -> Result<StageReport, PipelineError> {
    validate_character_package(package)?;
    let members = classify_members(package)?;
    if !members.animations.is_empty() || !members.controllers.is_empty() {
        return Err(PipelineError::new(format!(
            "single skeletal package contains runtime animation companions: {}",
            package.package_id
        )));
    }
    let texture_dir = package_dir.join("textures");
    let character = build_character(package, &members, base_root)?;
    let mesh_refs = character
        .parts
        .iter()
        .map(|part| &part.mesh)
        .collect::<Vec<_>>();
    let materials = resolve_indexed_package_materials(
        index,
        package,
        &mesh_refs,
        base_root,
        &texture_dir,
    )?;
    let file_stem = stable_file_stem(&package.subcategory);
    let fbx_path = package_dir.join(format!("{file_stem}.fbx"));
    let summary =
        write_binary_character_fbx(&character, &materials, &[], &fbx_path)
            .map_err(|_error| {
                PipelineError::new(format!(
                    "skeletal FBX serialization failed for {}",
                    package.package_id
                ))
            })?;
    verify_single_skeletal_summary(package, &character, &summary)?;
    let fbx_bytes = local_read_bytes(&fbx_path).map_err(|error| {
        fbx_io_error("read skeletal FBX for verification", &error)
    })?;
    verify_external_binary_fbx(package, &fbx_bytes)?;
    let report_path = package_dir.join("capability-report.json");
    write_capability_report(
        &report_path,
        &package.package_id,
        member_capability_items(&members),
    )?;
    stage_report(
        package,
        &summary,
        &fbx_path,
        &report_path,
        &reported_package_dir.join(format!("{file_stem}.fbx")),
    )
}

/// Verify one direct skeletal-mesh artifact has no imported animation payload.
fn verify_single_skeletal_summary(
    package: &PhaseThreePackageRow,
    character: &CharacterAsset,
    summary: &CharacterBinaryFbxSummary,
) -> Result<(), PipelineError> {
    let groups = character
        .parts
        .iter()
        .map(|part| part.mesh.groups.len())
        .sum::<usize>();
    if summary.bones != character.bones.len()
        || summary.geometries != groups
        || summary.clusters == 0
        || summary.materials == 0
        || summary.animations != 0
    {
        return Err(PipelineError::new(format!(
            "single skeletal FBX summary is invalid for {}: {summary:?}",
            package.package_id
        )));
    }
    Ok(())
}

/// Require one selected package to be a supported character FBX model.
fn validate_character_package(
    package: &PhaseThreePackageRow,
) -> Result<(), PipelineError> {
    let plan = PhaseThreePackagePlanner::plan(package);
    if plan.family != ConversionFamily::FbxModel {
        return Err(PipelineError::new(format!(
            "selected package is not an FBX model package: {}",
            package.package_id
        )));
    }
    if package.category != CHARACTERS_CATEGORY {
        return Err(PipelineError::new(format!(
            concat!(
                "fbx-export supports only character packages; ",
                "package {} has category {}",
            ),
            package.package_id, package.category
        )));
    }
    Ok(())
}

/// Classify package members into character export families.
fn classify_members(
    package: &PhaseThreePackageRow,
) -> Result<ClassifiedMembers<'_>, PipelineError> {
    let mut classified = ClassifiedMembers::default();
    for member in package.members() {
        match member.kind.as_str() {
            "p3d-skeleton" => classified.skeletons.push(member),
            "p3d-skin" => classified.skins.push(member),
            "p3d-mesh" => classified.meshes.push(member),
            "p3d-composite-drawable" => classified.composites.push(member),
            "p3d-animation" => classified.animations.push(member),
            "p3d-controller" => classified.controllers.push(member),
            "p3d-texture" => classified.textures.push(member),
            "p3d-shader" => classified.materials.push(member),
            "package-manifest" => classified.metadata.push(member),
            _ => classified.unsupported.push(member),
        }
    }
    if classified.skins.is_empty() {
        return Err(PipelineError::new(format!(
            "package {} has no skin members; animation-set and effect \
                     packages are a later capability pass",
            package.package_id
        )));
    }
    if classified.skeletons.len() != 1 {
        return Err(PipelineError::new(format!(
            "package {} must reference exactly one skeleton, found {}",
            package.package_id,
            classified.skeletons.len()
        )));
    }
    if let Some(member) = classified.unsupported.first() {
        return Err(PipelineError::new(format!(
            "package {} member {} has unsupported kind {}",
            package.package_id, member.id, member.kind
        )));
    }
    Ok(classified)
}

/// Assemble the validated character aggregate from classified members.
fn build_character(
    package: &PhaseThreePackageRow,
    members: &ClassifiedMembers<'_>,
    base_root: &Path,
) -> Result<CharacterAsset, PipelineError> {
    let skeleton_member = members.skeletons.first().ok_or_else(|| {
        PipelineError::new(format!(
            "package {} lost its skeleton member during \
                         classification",
            package.package_id
        ))
    })?;
    let skeleton_path = base_root.join(&skeleton_member.path);
    let skin_paths: Vec<PathBuf> = members
        .skins
        .iter()
        .map(|member| base_root.join(&member.path))
        .collect();
    let skin_path_refs: Vec<&Path> =
        skin_paths.iter().map(PathBuf::as_path).collect();
    let mesh_paths: Vec<PathBuf> = members
        .meshes
        .iter()
        .map(|member| base_root.join(&member.path))
        .collect();
    let mesh_path_refs: Vec<&Path> =
        mesh_paths.iter().map(PathBuf::as_path).collect();
    let composite_paths: Vec<PathBuf> = members
        .composites
        .iter()
        .map(|member| base_root.join(&member.path))
        .collect();
    let composite_path_refs: Vec<&Path> =
        composite_paths.iter().map(PathBuf::as_path).collect();
    load_character(
        &stable_file_stem(&package.subcategory),
        &skeleton_path,
        &skin_path_refs,
        &mesh_path_refs,
        &composite_path_refs,
    )
    .map_err(|_error| {
        PipelineError::new(format!(
            "character assembly failed for {}",
            package.package_id
        ))
    })
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
            .filter(|candidate| {
                candidate.category == CHARACTERS_CATEGORY
                    && candidate.subcategory == target
            })
            .collect::<Vec<_>>();
        match matches.as_slice() {
            [] => {},
            [animation_package] => return Ok(Some(*animation_package)),
            _ => {
                return Err(PipelineError::new(format!(
                    "character package {} has multiple animation-set \
                             rows for subcategory {target}",
                    package.package_id
                )));
            },
        }
    }
    Err(PipelineError::new(format!(
        "character package {} has no identity-specific or general \
                 animation-set row",
        package.package_id
    )))
}

/// Return identity-specific then general animation subcategories in priority
/// order.
fn animation_subcategory_candidates(subcategory: &str) -> Vec<String> {
    let identity_root = subcategory.strip_suffix("/base-model").or_else(|| {
        subcategory
            .split_once("/costume/")
            .map(|(root, _costume)| root)
    });
    let mut candidates = Vec::with_capacity(2);
    if let Some(root) = identity_root {
        candidates.push(format!("{root}/animation-set"));
    }
    if candidates.first().is_none_or(|candidate| {
        candidate != GENERAL_CHARACTER_ANIMATION_SUBCATEGORY
    }) {
        candidates.push(GENERAL_CHARACTER_ANIMATION_SUBCATEGORY.to_owned());
    }
    candidates
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
        vec![expected_file_name, "char_swatches_lit.png".to_owned()]
    } else {
        vec![expected_file_name]
    };
    let matches = index
        .packages()
        .iter()
        .flat_map(|package| {
            package
                .members()
                .iter()
                .map(move |member| (package, member))
        })
        .filter(|(package, member)| {
            package.category == CHARACTERS_CATEGORY
                && package.subcategory == CHARACTER_SHARED_SUBCATEGORY
                && member.kind == "p3d-texture"
                && member.source_chunk_kind == "texture"
                && Path::new(&member.path)
                    .file_name()
                    .and_then(|value| value.to_str())
                    .is_some_and(|name| {
                        accepted_file_names.iter().any(|item| item == name)
                    })
        })
        .collect::<Vec<_>>();
    match matches.as_slice() {
        [] => Ok(None),
        [resolved] => Ok(Some(*resolved)),
        _ => Err(PipelineError::new(format!(
            "texture reference {texture_reference} resolves to \
                     multiple package-index PNG members"
        ))),
    }
}

/// Normalize one safe decoded texture reference to its staged PNG file name.
fn normalized_texture_png_file_name(
    texture_reference: &str,
) -> Result<String, PipelineError> {
    let normalized_reference = texture_reference.trim_end_matches('\u{0}');
    let mut components = Path::new(normalized_reference).components();
    if normalized_reference.is_empty()
        || normalized_reference != normalized_reference.trim()
        || !matches!(components.next(), Some(std::path::Component::Normal(_)))
        || components.next().is_some()
    {
        return Err(PipelineError::new(format!(
            "invalid shader texture reference: {normalized_reference}"
        )));
    }
    let stem = normalized_reference
        .rsplit_once('.')
        .filter(|(_, extension)| {
            extension.eq_ignore_ascii_case("bmp")
                || extension.eq_ignore_ascii_case("png")
        })
        .map_or(normalized_reference, |(value, _)| value);
    if stem.is_empty() {
        return Err(PipelineError::new(format!(
            "invalid shader texture reference: {normalized_reference}"
        )));
    }
    Ok(format!("{stem}.png"))
}

/// Remove the private texture staging directory before or after one export.
fn remove_texture_staging_dir(path: &Path) -> Result<(), PipelineError> {
    if !path.exists() {
        return Ok(());
    }
    if !path.is_dir() {
        return Err(PipelineError::new(
            "texture staging path is not a directory",
        ));
    }
    std::fs::remove_dir_all(path).map_err(|error| {
        fbx_io_error("remove texture staging directory", &error)
    })
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
) -> Result<(MaterialBinding, Option<CapabilityItem>), PipelineError> {
    match source.resolve_material(shader) {
        Ok(binding) => Ok((binding, None)),
        Err(DecodedComponentError::MissingTexture {
            shader: shader_name,
            texture,
            ..
        }) => {
            let Some((owner, member)) =
                resolve_shared_texture_member(index, &texture)?
            else {
                let binding = MaterialBinding::new(
                    deferred_material_identity(shader, &shader_name),
                    None,
                )
                .map_err(|error| {
                    PipelineError::new(format!(
                        "material binding failed for {shader}: \
                                 {error:?}"
                    ))
                })?;
                let item = CapabilityItem {
                    id: format!("texture-reference:{texture}"),
                    outcome: "deferred",
                    reason: format!(
                        "shader {shader_name} has no unique shared PNG"
                    ),
                };
                return Ok((binding, Some(item)));
            };
            let binding = source
                .resolve_material_with_external_texture(
                    shader,
                    &base_root.join(&member.path),
                )
                .map_err(|error| {
                    PipelineError::new(format!(
                        "shared texture failed for {shader}: {error:?}"
                    ))
                })?;
            let item = CapabilityItem {
                id: format!("texture-reference:{texture}"),
                outcome: "converted",
                reason: format!(
                    "shader {shader_name} uses package {} member {}",
                    owner.package_id, member.id
                ),
            };
            Ok((binding, Some(item)))
        },
        Err(error) => Err(PipelineError::new(format!(
            "material resolution failed for shader {shader}: {error:?}"
        ))),
    }
}

/// Resolve every used shader through private texture staging for embedding.
fn resolve_materials(
    index: &PhaseThreePackageIndex,
    package: &PhaseThreePackageRow,
    members: &ClassifiedMembers<'_>,
    base_root: &Path,
    texture_staging_dir: &Path,
) -> Result<(Vec<MaterialBinding>, Vec<CapabilityItem>), PipelineError> {
    let package_root = base_root.join(&package.package_root);
    let source = DecodedComponentSource::new(
        package_root,
        texture_staging_dir.to_path_buf(),
    );
    let mut shader_names: Vec<String> = members
        .materials
        .iter()
        .filter_map(|member| {
            Path::new(&member.path)
                .file_stem()
                .and_then(|stem| stem.to_str())
                .map(str::to_owned)
        })
        .collect();
    shader_names.sort();
    shader_names.dedup();
    let mut bindings = Vec::with_capacity(shader_names.len());
    let mut items = Vec::new();
    for shader in &shader_names {
        let (binding, optional_capability) =
            resolve_material_binding(&source, index, base_root, shader)?;
        bindings.push(binding);
        if let Some(capability_item) = optional_capability {
            items.push(capability_item);
        }
    }
    Ok((bindings, items))
}

/// Append one uniform capability outcome for a member collection.
fn append_capability_items(
    items: &mut Vec<CapabilityItem>,
    members: &[&PhaseThreePackageMember],
    outcome: &'static str,
    reason: &'static str,
) {
    items.extend(members.iter().map(|member| CapabilityItem {
        id: member.id.clone(),
        outcome,
        reason: reason.to_owned(),
    }));
}

/// Produce deterministic capability items for every classified member.
fn member_capability_items(
    members: &ClassifiedMembers<'_>,
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
        items.push(CapabilityItem {
            id: member.id.clone(),
            outcome: if embeddable {
                "converted"
            } else {
                "preserved-as-metadata"
            },
            reason: if embeddable {
                "referenced PNG published as an external FBX texture".to_owned()
            } else {
                "texture metadata preserved for traceability".to_owned()
            },
        });
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
    items.sort_by(|left, right| left.id.cmp(&right.id));
    let mut json = String::new();
    json.push_str("{\n");
    json.push_str("  \"package_id\": \"");
    json.push_str(&escape_json(package_id));
    json.push_str("\",\n");
    json.push_str("  \"items\": [\n");
    for (position, item) in items.iter().enumerate() {
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
    local_write_text(path, &json, true)
        .map_err(|error| fbx_io_error("write FBX capability report", &error))?;
    Ok(())
}

/// Build the stage report for one completed character export.
fn stage_report(
    package: &PhaseThreePackageRow,
    summary: &CharacterBinaryFbxSummary,
    fbx_path: &Path,
    report_path: &Path,
    reported_fbx_path: &Path,
) -> Result<StageReport, PipelineError> {
    let fbx_bytes = file_len(fbx_path)?;
    let report_bytes = file_len(report_path)?;
    let bytes =
        StageReport::checked_byte_total("fbx-export", fbx_bytes, report_bytes)?;
    Ok(StageReport {
        name: "fbx-export",
        files: 2,
        bytes,
        note: format!(
            "package={} output={} bones={} geometries={} clusters={} \
                 materials={} textures={} animations={}",
            package.package_id,
            reported_fbx_path.display(),
            summary.bones,
            summary.geometries,
            summary.clusters,
            summary.materials,
            summary.textures,
            summary.animations
        ),
    })
}

/// Supports the `file_len` operation within this deterministic export
/// boundary.
fn file_len(path: &Path) -> Result<u64, PipelineError> {
    local_file_len(path)
        .map_err(|error| fbx_io_error("stat FBX export artifact", &error))
}

/// Build one public-safe FBX I/O diagnostic without physical path text.
fn fbx_io_error(action: &str, error: &std::io::Error) -> PipelineError {
    PipelineError::new(format!("{action} failed ({:?})", error.kind()))
}

#[cfg(test)]
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../../../tests/migration/pipeline/unit/adapter-outbound/local/fbx_export/tests.rs"]
mod tests;
