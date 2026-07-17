// File:
//   - binary_character_writer.rs
// Path:
//   - src/fbx/src/adapters/driven/binary_character_writer.rs
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
//   - Native binary FBX 7.7 serialization for validated character packages.
// - Must-Not:
//   - Decode source assets, rediscover package membership, or invoke Blender.
// - Allows:
//   - Typed FBX nodes for character geometry, materials, skeletons, skin,
//   - bind poses, connections, and deterministic file persistence.
// - Split-When:
//   - One FBX object family gains a separately testable serialization contract.
// - Merge-When:
//   - Another writer owns the same binary character document and identity map.
// - Summary:
//   - Writes one deterministic binary FBX character artifact.
// - Description:
//   - Serializes one phase-three-resolved character aggregate as a
//   - Maya-compatible binary FBX 7.7 node tree.
// - Usage:
//   - Called by the package-driven pipeline character export transaction.
// - Defaults:
//   - Emits binary only, uses raw arrays, fixed metadata, Y-up axes, and meter
//   - units expressed as one hundred centimeters per scene unit.
//
// ADRs:
// - docs/adr/pipeline/fbx/hexagonal-scene-export.md
//
// Large file:
//   - true
//   - Reason: The writer keeps one ordered FBX object tree and connection map
//   - together so deterministic ids, bind matrices, and section ordering can
//   - be audited against the canonical binary contract; split when one
//   - object family gains an independent public contract.
//

//! Native binary FBX 7.7 writer for skinned character packages.
//!
//! The pipeline emits this binary document as the sole `.fbx` representation.
//! Optional review helpers import this same artifact rather than serializing
//! alternate FBX encodings.

use std::collections::{BTreeMap, BTreeSet};
use std::fs::OpenOptions;
use std::io::{ErrorKind, Write as _};
use std::path::Path;

use schoenwald_filesystem::adapters::driving::local;

use super::binary_animation::{
    BinaryAnimationCounts, BinaryAnimationPlan, build_animation_plan,
};
use super::binary_character_input::{
    BoneTransform, CharacterInputError, MaterialSlot, POSE_ID, bone_transforms,
    material_slots,
};
use super::binary_fbx::{
    BinaryNode, BinaryProperty, CREATION_TIME, DETERMINISTIC_FILE_ID,
    encode_binary_document,
};
use super::binary_identity::{
    BinaryIdentityError, GeometryIds, bone_ids, cluster_id, geometry_ids,
};
use crate::domain::animation::AnimationClip;
use crate::domain::character::CharacterAsset;
use crate::domain::mesh::PrimitiveGroup;
use crate::domain::texture::MaterialBinding;
use crate::domain::transform::affine_inverse::{InverseError, invert_affine};
use crate::domain::transform::matrix::{TrsParts, compose, multiply};

/// Deterministic parent rotating the completed character around the FBX up
/// axis.
const EXPORT_ROOT_ID: u64 = 1_000_001;
/// Export-root transform shared by the scene hierarchy and every global bind
/// record.
const EXPORT_ROOT_TRANSFORM: TrsParts = TrsParts {
    translation: [
        0.0_f64, 0.0_f64, 0.0_f64,
    ],
    rotation_degrees: [
        0.0_f64, 180.0_f64, 0.0_f64,
    ],
    scale: [
        1.0_f64, 1.0_f64, 1.0_f64,
    ],
};
/// PNG signature required by the current decoded texture pipeline.
const PNG_SIGNATURE: &[u8; 8] = b"\x89PNG\r\n\x1a\n";

/// One texture payload embedded directly in a binary FBX video object.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EmbeddedTexture {
    /// Portable file name referenced by the FBX texture and video objects.
    pub file_name: String,
    /// Exact encoded image bytes stored in `Video.Content`.
    pub content: Vec<u8>,
}

/// Texture-storage policy selected for one character document.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CharacterTextureStorage {
    /// Reference sibling files under `textures/` without `Video.Content`.
    External,
    /// Preserve legacy self-contained `Video.Content` payloads explicitly.
    Embedded,
}

/// Deterministic summary of one binary character FBX document.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CharacterBinaryFbxSummary {
    /// Geometry objects written to the document.
    pub geometries: usize,
    /// Limb-node bones written to the document.
    pub bones: usize,
    /// Skin clusters written to the document.
    pub clusters: usize,
    /// Deduplicated materials written to the document.
    pub materials: usize,
    /// Textures referenced by written materials.
    pub textures: usize,
    /// Skeletal animation stacks written to the document.
    pub animations: usize,
}

/// Texture-storage policy and optional payload lookup for one document.
struct TexturePayloadContext<'map, 'name, 'content> {
    /// Embedded payloads keyed by portable external texture file name.
    embedded: &'map BTreeMap<&'name str, &'content [u8]>,
    /// Selected external or compatibility-embedded storage policy.
    storage: CharacterTextureStorage,
}

/// One binary-writer geometry group with raw FBX identity.
struct BinaryGroup<'character> {
    /// Deterministic object ids allocated inside the binary document.
    ids: GeometryIds,
    /// Raw object name for typed binary FBX strings.
    object_name: String,
    /// Validated primitive group borrowed from the aggregate.
    group: &'character PrimitiveGroup,
    /// Influences borrowed from the aggregate for this group.
    influences: &'character [crate::domain::skin::SkinInfluence],
    /// Bones influencing this group in stable ordinal order.
    used_bones: Vec<String>,
}

/// Flatten aggregate parts into binary-writer geometry groups.
fn binary_groups(
    character: &CharacterAsset
) -> Result<Vec<BinaryGroup<'_>>, BinaryIdentityError> {
    let mut groups = Vec::new();
    for part in &character.parts {
        for (group, influences) in part
            .mesh
            .groups
            .iter()
            .zip(&part.group_influences)
        {
            let ids = geometry_ids(groups.len())?;
            let used_bones = influences
                .iter()
                .map(
                    |influence| {
                        influence
                            .bone_id
                            .clone()
                    },
                )
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect();
            groups.push(
                BinaryGroup {
                    ids,
                    object_name: format!(
                        "{}_{}",
                        part.mesh
                            .name,
                        group.index
                    ),
                    group,
                    influences,
                    used_bones,
                },
            );
        }
    }
    Ok(groups)
}

/// Count all geometry-and-bone cluster pairs.
fn binary_cluster_count(groups: &[BinaryGroup<'_>]) -> usize {
    groups
        .iter()
        .map(
            |group| {
                group
                    .used_bones
                    .len()
            },
        )
        .sum()
}

/// Validate and index exact PNG payloads by portable file name.
fn embedded_texture_contents<'texture>(
    materials: &[MaterialBinding],
    embedded_textures: &'texture [EmbeddedTexture],
) -> Result<BTreeMap<&'texture str, &'texture [u8]>, CharacterBinaryFbxError> {
    let required: BTreeSet<&str> = materials
        .iter()
        .filter_map(
            |binding| {
                binding
                    .texture_file_name
                    .as_deref()
            },
        )
        .collect();
    let mut contents = BTreeMap::new();
    for texture in embedded_textures {
        let path = Path::new(&texture.file_name);
        let valid_name = path
            .file_name()
            .and_then(|value| value.to_str())
            .is_some_and(|value| value == texture.file_name)
            && texture.file_name
                == texture
                    .file_name
                    .trim();
        if !valid_name {
            return Err(
                CharacterBinaryFbxError::InvalidEmbeddedTextureName {
                    file_name: texture
                        .file_name
                        .clone(),
                },
            );
        }
        if !texture
            .content
            .starts_with(PNG_SIGNATURE)
        {
            return Err(
                CharacterBinaryFbxError::InvalidEmbeddedTextureContent {
                    file_name: texture
                        .file_name
                        .clone(),
                },
            );
        }
        if contents
            .insert(
                texture
                    .file_name
                    .as_str(),
                texture
                    .content
                    .as_slice(),
            )
            .is_some()
        {
            return Err(
                CharacterBinaryFbxError::DuplicateEmbeddedTexture {
                    file_name: texture
                        .file_name
                        .clone(),
                },
            );
        }
    }
    for file_name in &required {
        if !contents.contains_key(file_name) {
            return Err(
                CharacterBinaryFbxError::MissingEmbeddedTexture {
                    file_name: (*file_name).to_owned(),
                },
            );
        }
    }
    if let Some(file_name) = contents
        .keys()
        .find(|file_name| !required.contains(**file_name))
    {
        return Err(
            CharacterBinaryFbxError::UnexpectedEmbeddedTexture {
                file_name: (*file_name).to_owned(),
            },
        );
    }
    Ok(contents)
}

/// Internal typed character document used by binary serialization.
struct CharacterFbxDocument {
    /// Complete ordered top-level FBX node list.
    nodes: Vec<BinaryNode>,
    /// Deterministic semantic object counts.
    summary: CharacterBinaryFbxSummary,
}

/// Write one validated character as an external-texture binary FBX 7.7 file.
///
/// # Errors
///
/// Returns an error when serializer input validation fails, a public value
/// does not fit the FBX 7.7 field width, binary encoding fails, or the
/// create-new artifact write cannot complete.
pub fn write_binary_character_fbx(
    character: &CharacterAsset,
    materials: &[MaterialBinding],
    animations: &[AnimationClip],
    path: &Path,
) -> Result<CharacterBinaryFbxSummary, CharacterBinaryFbxError> {
    write_binary_character_fbx_with_storage(
        character,
        materials,
        &[],
        CharacterTextureStorage::External,
        animations,
        path,
    )
}

/// Write one validated character with explicit embedded compatibility payloads.
///
/// # Errors
///
/// Returns an error under the same conditions as
/// [`write_binary_character_fbx`] or when embedded payload validation fails.
pub fn write_binary_character_fbx_embedded(
    character: &CharacterAsset,
    materials: &[MaterialBinding],
    embedded_textures: &[EmbeddedTexture],
    animations: &[AnimationClip],
    path: &Path,
) -> Result<CharacterBinaryFbxSummary, CharacterBinaryFbxError> {
    write_binary_character_fbx_with_storage(
        character,
        materials,
        embedded_textures,
        CharacterTextureStorage::Embedded,
        animations,
        path,
    )
}

/// Serialize one character with an explicit texture-storage policy.
fn write_binary_character_fbx_with_storage(
    character: &CharacterAsset,
    materials: &[MaterialBinding],
    embedded_textures: &[EmbeddedTexture],
    texture_storage: CharacterTextureStorage,
    animations: &[AnimationClip],
    path: &Path,
) -> Result<CharacterBinaryFbxSummary, CharacterBinaryFbxError> {
    let document = build_character_document(
        character,
        materials,
        embedded_textures,
        texture_storage,
        animations,
    )?;
    let bytes = encode_binary_document(&document.nodes).map_err(
        |error| CharacterBinaryFbxError::Encoding {
            reason: format!("{error:?}"),
        },
    )?;
    persist(
        path, &bytes,
    )?;
    Ok(document.summary)
}

/// Build one internal typed character FBX document.
fn build_character_document(
    character: &CharacterAsset,
    materials: &[MaterialBinding],
    embedded_textures: &[EmbeddedTexture],
    texture_storage: CharacterTextureStorage,
    animations: &[AnimationClip],
) -> Result<CharacterFbxDocument, CharacterBinaryFbxError> {
    let material_slots = material_slots(
        character, materials,
    )
    .map_err(CharacterBinaryFbxError::from)?;
    let embedded_texture_contents = match texture_storage {
        CharacterTextureStorage::External => BTreeMap::new(),
        CharacterTextureStorage::Embedded => embedded_texture_contents(
            materials,
            embedded_textures,
        )?,
    };
    let bone_transforms =
        bone_transforms(character).map_err(CharacterBinaryFbxError::from)?;
    let groups =
        binary_groups(character).map_err(CharacterBinaryFbxError::from)?;
    let bone_ordinals: BTreeMap<&str, usize> = character
        .bones
        .iter()
        .enumerate()
        .map(
            |(ordinal, bone)| {
                (
                    bone.id
                        .as_str(),
                    ordinal,
                )
            },
        )
        .collect();
    let animation_plan = build_animation_plan(
        animations,
        &bone_ordinals,
    )
    .map_err(
        |error| CharacterBinaryFbxError::AnimationPlan {
            reason: format!("{error:?}"),
        },
    )?;
    let texture_payloads = TexturePayloadContext {
        embedded: &embedded_texture_contents,
        storage: texture_storage,
    };
    let nodes = document_nodes(
        character,
        &groups,
        &material_slots,
        &texture_payloads,
        &bone_transforms,
        &bone_ordinals,
        &animation_plan,
    )?;
    let summary = CharacterBinaryFbxSummary {
        geometries: groups.len(),
        bones: character
            .bones
            .len(),
        clusters: binary_cluster_count(&groups),
        materials: material_slots.len(),
        textures: material_slots
            .values()
            .filter(
                |slot| {
                    slot.binding
                        .texture_file_name
                        .is_some()
                },
            )
            .count(),
        animations: animations.len(),
    };
    Ok(
        CharacterFbxDocument {
            nodes,
            summary,
        },
    )
}

/// Build the complete ordered top-level node list.
fn document_nodes(
    character: &CharacterAsset,
    groups: &[BinaryGroup<'_>],
    material_slots: &BTreeMap<String, MaterialSlot<'_>>,
    texture_payloads: &TexturePayloadContext<'_, '_, '_>,
    bone_transforms: &[BoneTransform],
    bone_ordinals: &BTreeMap<&str, usize>,
    animation_plan: &BinaryAnimationPlan,
) -> Result<Vec<BinaryNode>, CharacterBinaryFbxError> {
    Ok(
        vec![
            header_extension(),
            BinaryNode::leaf(
                "FileId",
                vec![BinaryProperty::Bytes(DETERMINISTIC_FILE_ID.to_vec())],
            ),
            BinaryNode::leaf(
                "CreationTime",
                vec![BinaryProperty::String(CREATION_TIME.to_owned())],
            ),
            BinaryNode::leaf(
                "Creator",
                vec![BinaryProperty::String("SHAR".to_owned())],
            ),
            global_settings(animation_plan),
            documents(&animation_plan.active_stack_name),
            BinaryNode::branch(
                "References",
                Vec::new(),
            ),
            definitions(
                groups,
                material_slots,
                character
                    .bones
                    .len(),
                animation_plan.counts,
            )?,
            objects(
                character,
                groups,
                material_slots,
                texture_payloads,
                bone_transforms,
                bone_ordinals,
                animation_plan,
            )?,
            connections(
                character,
                groups,
                material_slots,
                bone_ordinals,
                animation_plan,
            )?,
            animation_plan
                .takes
                .clone(),
        ],
    )
}

/// Build deterministic file-header metadata.
fn header_extension() -> BinaryNode {
    BinaryNode::branch(
        "FBXHeaderExtension",
        vec![
            i32_node(
                "FBXHeaderVersion",
                1_003,
            ),
            i32_node(
                "FBXVersion",
                7_700,
            ),
            i32_node(
                "EncryptionType",
                0,
            ),
            BinaryNode::branch(
                "CreationTimeStamp",
                vec![
                    i32_node(
                        "Version", 1_000,
                    ),
                    i32_node(
                        "Year", 1_970,
                    ),
                    i32_node(
                        "Month", 1,
                    ),
                    i32_node(
                        "Day", 1,
                    ),
                    i32_node(
                        "Hour", 10,
                    ),
                    i32_node(
                        "Minute", 0,
                    ),
                    i32_node(
                        "Second", 0,
                    ),
                    i32_node(
                        "Millisecond",
                        0,
                    ),
                ],
            ),
            string_node(
                "Creator", "SHAR",
            ),
        ],
    )
}

/// Build Y-up, right-handed, meter-unit global settings.
fn global_settings(animation_plan: &BinaryAnimationPlan) -> BinaryNode {
    let frame_rate = animation_plan
        .frame_rate
        .unwrap_or(24.0_f64);
    let time_mode = frame_rate_time_mode(frame_rate);
    BinaryNode::branch(
        "GlobalSettings",
        vec![
            i32_node(
                "Version", 1_000,
            ),
            BinaryNode::branch(
                "Properties70",
                vec![
                    integer_property(
                        "UpAxis", 1,
                    ),
                    integer_property(
                        "UpAxisSign",
                        1,
                    ),
                    integer_property(
                        "FrontAxis",
                        2,
                    ),
                    integer_property(
                        "FrontAxisSign",
                        1,
                    ),
                    integer_property(
                        "CoordAxis",
                        0,
                    ),
                    integer_property(
                        "CoordAxisSign",
                        1,
                    ),
                    integer_property(
                        "OriginalUpAxis",
                        1,
                    ),
                    integer_property(
                        "OriginalUpAxisSign",
                        1,
                    ),
                    double_property(
                        "UnitScaleFactor",
                        100.0,
                    ),
                    double_property(
                        "OriginalUnitScaleFactor",
                        100.0,
                    ),
                    color_property(
                        "AmbientColor",
                        [
                            0.0, 0.0, 0.0,
                        ],
                    ),
                    string_property(
                        "DefaultCamera",
                        "Producer Perspective",
                    ),
                    enum_property(
                        "TimeMode", time_mode,
                    ),
                    time_property(
                        "TimeSpanStart",
                        0,
                    ),
                    time_property(
                        "TimeSpanStop",
                        animation_plan.max_stop_time,
                    ),
                    double_property(
                        "CustomFrameRate",
                        frame_rate,
                    ),
                ],
            ),
        ],
    )
}

/// Map one source rate to an FBX global-settings time mode.
const fn frame_rate_time_mode(frame_rate: f64) -> i32 {
    if frame_rate.to_bits() == 30.0_f64.to_bits() {
        6_i32
    } else if frame_rate.to_bits() == 24.0_f64.to_bits() {
        11_i32
    } else {
        14_i32
    }
}

/// Build the single deterministic scene document declaration.
fn documents(active_stack_name: &str) -> BinaryNode {
    BinaryNode::branch(
        "Documents",
        vec![
            i32_node(
                "Count", 1,
            ),
            BinaryNode::new(
                "Document",
                vec![
                    BinaryProperty::I64(1),
                    BinaryProperty::String("Scene".to_owned()),
                    BinaryProperty::String("Scene".to_owned()),
                ],
                vec![
                    BinaryNode::branch(
                        "Properties70",
                        vec![
                            BinaryNode::leaf(
                                "P",
                                vec![
                                    string("SourceObject"),
                                    string("object"),
                                    string(""),
                                    string(""),
                                ],
                            ),
                            BinaryNode::leaf(
                                "P",
                                vec![
                                    string("ActiveAnimStackName"),
                                    string("KString"),
                                    string(""),
                                    string(""),
                                    string(active_stack_name),
                                ],
                            ),
                        ],
                    ),
                    BinaryNode::leaf(
                        "RootNode",
                        vec![BinaryProperty::I64(0)],
                    ),
                ],
            ),
        ],
    )
}

/// Build definitions with explicit counts for every emitted object family.
// One ordered section keeps all object-family counts and ids auditable.
#[expect(
    clippy::too_many_lines,
    reason = "Ordered sections preserve deterministic FBX family counts and \
              ids"
)]
fn definitions(
    groups: &[BinaryGroup<'_>],
    material_slots: &BTreeMap<String, MaterialSlot<'_>>,
    bone_count: usize,
    animation_counts: BinaryAnimationCounts,
) -> Result<BinaryNode, CharacterBinaryFbxError> {
    let texture_count = material_slots
        .values()
        .filter(
            |slot| {
                slot.binding
                    .texture_file_name
                    .is_some()
            },
        )
        .count();
    let cluster_total = binary_cluster_count(groups);
    let model_count = groups
        .len()
        .checked_add(bone_count)
        .and_then(|count| count.checked_add(1))
        .ok_or(
            CharacterBinaryFbxError::CountOverflow {
                context: "model definitions",
            },
        )?;
    let deformer_count = groups
        .len()
        .checked_add(cluster_total)
        .ok_or(
            CharacterBinaryFbxError::CountOverflow {
                context: "deformer definitions",
            },
        )?;
    let counts = [
        (
            "GlobalSettings",
            1,
        ),
        (
            "Geometry",
            groups.len(),
        ),
        (
            "Model",
            model_count,
        ),
        (
            "Material",
            material_slots.len(),
        ),
        (
            "Texture",
            texture_count,
        ),
        (
            "Video",
            texture_count,
        ),
        (
            "Deformer",
            deformer_count,
        ),
        (
            "Pose", 1,
        ),
        (
            "NodeAttribute",
            bone_count,
        ),
        (
            "AnimationStack",
            animation_counts.stacks,
        ),
        (
            "AnimationLayer",
            animation_counts.layers,
        ),
        (
            "AnimationCurveNode",
            animation_counts.curve_nodes,
        ),
        (
            "AnimationCurve",
            animation_counts.curves,
        ),
    ];
    let object_count = counts
        .iter()
        .try_fold(
            0_usize,
            |total, (_, count)| {
                total
                    .checked_add(*count)
                    .ok_or(
                        CharacterBinaryFbxError::CountOverflow {
                            context: "definition object count",
                        },
                    )
            },
        )?;
    let mut children = vec![
        i32_node(
            "Version", 100,
        ),
        i32_node(
            "Count",
            count_i32(
                object_count,
                "definition object count",
            )?,
        ),
    ];
    for (object_type, count) in counts {
        if count == 0 {
            continue;
        }
        let mut object_children = vec![
            i32_node(
                "Count",
                count_i32(
                    count,
                    "object type count",
                )?,
            ),
        ];
        if let Some(template) = animation_definition_template(object_type) {
            object_children.push(template);
        }
        children.push(
            BinaryNode::new(
                "ObjectType",
                vec![string(object_type)],
                object_children,
            ),
        );
    }
    Ok(
        BinaryNode::branch(
            "Definitions",
            children,
        ),
    )
}

/// Return the Maya-compatible property template for one emitted family.
fn animation_definition_template(object_type: &str) -> Option<BinaryNode> {
    let template = match object_type {
        "Model" => property_template(
            "FbxNode",
            model_definition_properties(),
        ),
        "AnimationStack" => property_template(
            "FbxAnimStack",
            animation_stack_definition_properties(),
        ),
        "AnimationLayer" => property_template(
            "FbxAnimLayer",
            animation_layer_definition_properties(),
        ),
        "AnimationCurveNode" => property_template(
            "FbxAnimCurveNode",
            animation_curve_node_definition_properties(),
        ),
        _ => return None,
    };
    Some(template)
}

/// Build one property-template node from its ordered defaults.
fn property_template(
    template_name: &str,
    properties: Vec<BinaryNode>,
) -> BinaryNode {
    BinaryNode::new(
        "PropertyTemplate",
        vec![string(template_name)],
        vec![
            BinaryNode::branch(
                "Properties70",
                properties,
            ),
        ],
    )
}

/// Build the transform defaults required by Maya model objects.
fn model_definition_properties() -> Vec<BinaryNode> {
    vec![
        definition_property(
            "QuaternionInterpolate",
            "enum",
            "",
            "",
            vec![BinaryProperty::I32(0)],
        ),
        definition_property(
            "RotationOrder",
            "enum",
            "",
            "",
            vec![BinaryProperty::I32(0)],
        ),
        definition_property(
            "DefaultAttributeIndex",
            "int",
            "Integer",
            "",
            vec![BinaryProperty::I32(-1)],
        ),
        definition_property(
            "Lcl Translation",
            "Lcl Translation",
            "",
            "A",
            vector_defaults(0.0),
        ),
        definition_property(
            "Lcl Rotation",
            "Lcl Rotation",
            "",
            "A",
            vector_defaults(0.0),
        ),
        definition_property(
            "Lcl Scaling",
            "Lcl Scaling",
            "",
            "A",
            vector_defaults(1.0),
        ),
    ]
}

/// Build the timing defaults required by Maya animation stacks.
fn animation_stack_definition_properties() -> Vec<BinaryNode> {
    vec![
        definition_property(
            "Description",
            "KString",
            "",
            "",
            vec![string("")],
        ),
        definition_property(
            "LocalStart",
            "KTime",
            "Time",
            "",
            vec![BinaryProperty::I64(0)],
        ),
        definition_property(
            "LocalStop",
            "KTime",
            "Time",
            "",
            vec![BinaryProperty::I64(0)],
        ),
        definition_property(
            "ReferenceStart",
            "KTime",
            "Time",
            "",
            vec![BinaryProperty::I64(0)],
        ),
        definition_property(
            "ReferenceStop",
            "KTime",
            "Time",
            "",
            vec![BinaryProperty::I64(0)],
        ),
    ]
}

/// Build the blend defaults required by Maya animation layers.
fn animation_layer_definition_properties() -> Vec<BinaryNode> {
    vec![
        definition_property(
            "Weight",
            "Number",
            "",
            "A",
            vec![BinaryProperty::F64(100.0)],
        ),
        definition_property(
            "Mute",
            "bool",
            "",
            "",
            vec![BinaryProperty::I32(0)],
        ),
        definition_property(
            "Solo",
            "bool",
            "",
            "",
            vec![BinaryProperty::I32(0)],
        ),
        definition_property(
            "Lock",
            "bool",
            "",
            "",
            vec![BinaryProperty::I32(0)],
        ),
        definition_property(
            "Color",
            "ColorRGB",
            "Color",
            "",
            vector_defaults(0.8),
        ),
        definition_property(
            "BlendMode",
            "enum",
            "",
            "",
            vec![BinaryProperty::I32(0)],
        ),
        definition_property(
            "RotationAccumulationMode",
            "enum",
            "",
            "",
            vec![BinaryProperty::I32(0)],
        ),
        definition_property(
            "ScaleAccumulationMode",
            "enum",
            "",
            "",
            vec![BinaryProperty::I32(0)],
        ),
        definition_property(
            "BlendModeBypass",
            "ULongLong",
            "",
            "",
            vec![BinaryProperty::I64(0)],
        ),
    ]
}

/// Build the compound defaults required by Maya curve nodes.
fn animation_curve_node_definition_properties() -> Vec<BinaryNode> {
    vec![
        definition_property(
            "d",
            "Compound",
            "",
            "",
            Vec::new(),
        ),
    ]
}

/// Build three equal scalar defaults for one transform or color property.
fn vector_defaults(value: f64) -> Vec<BinaryProperty> {
    vec![
        BinaryProperty::F64(value),
        BinaryProperty::F64(value),
        BinaryProperty::F64(value),
    ]
}

/// Build one property-template entry with ordered scalar defaults.
fn definition_property(
    name: &str,
    property_type: &str,
    label: &str,
    flags: &str,
    mut defaults: Vec<BinaryProperty>,
) -> BinaryNode {
    let mut properties = vec![
        string(name),
        string(property_type),
        string(label),
        string(flags),
    ];
    properties.append(&mut defaults);
    BinaryNode::leaf(
        "P", properties,
    )
}

/// Build every object in deterministic id order.
// One ordered object pass keeps ids, bind matrices, and object types aligned.
#[expect(
    clippy::too_many_lines,
    reason = "Ordered assembly preserves deterministic FBX object ids and \
              links"
)]
fn objects(
    character: &CharacterAsset,
    groups: &[BinaryGroup<'_>],
    material_slots: &BTreeMap<String, MaterialSlot<'_>>,
    texture_payloads: &TexturePayloadContext<'_, '_, '_>,
    bone_transforms: &[BoneTransform],
    bone_ordinals: &BTreeMap<&str, usize>,
    animation_plan: &BinaryAnimationPlan,
) -> Result<BinaryNode, CharacterBinaryFbxError> {
    let mut children = vec![export_root_node()?];
    for group in groups {
        children.push(geometry_node(group)?);
        children.push(
            mesh_model_node(
                group
                    .ids
                    .model,
                &group.object_name,
            )?,
        );
        children.push(
            skin_deformer_node(
                group
                    .ids
                    .deformer,
                &group.object_name,
            )?,
        );
    }
    for slot in material_slots.values() {
        children.push(material_node(slot)?);
        if let Some(texture_file) = &slot
            .binding
            .texture_file_name
        {
            let relative_path = texture_relative_path(
                texture_file,
                texture_payloads.storage,
            );
            let content = match texture_payloads.storage {
                CharacterTextureStorage::External => None,
                CharacterTextureStorage::Embedded => Some(
                    *texture_payloads
                        .embedded
                        .get(texture_file.as_str())
                        .ok_or_else(
                            || CharacterBinaryFbxError::MissingEmbeddedTexture {
                                file_name: texture_file.clone(),
                            },
                        )?,
                ),
            };
            children.push(
                texture_node(
                    slot,
                    &relative_path,
                )?,
            );
            children.push(
                video_node(
                    slot,
                    &relative_path,
                    content,
                )?,
            );
        }
    }
    for (ordinal, bone) in character
        .bones
        .iter()
        .enumerate()
    {
        let ids = bone_ids(ordinal).map_err(CharacterBinaryFbxError::from)?;
        let transform = bone_transforms
            .get(ordinal)
            .ok_or_else(
                || CharacterBinaryFbxError::MissingBoneTransform {
                    bone: bone
                        .id
                        .clone(),
                },
            )?;
        children.push(
            limb_model_node(
                ids.model,
                &bone.id,
                &transform.local_parts,
            )?,
        );
        children.push(
            limb_attribute_node(
                ids.attribute,
                &bone.id,
            )?,
        );
    }
    for (group_ordinal, group) in groups
        .iter()
        .enumerate()
    {
        for bone_id in &group.used_bones {
            let bone_ordinal = *bone_ordinals
                .get(bone_id.as_str())
                .ok_or_else(
                    || CharacterBinaryFbxError::UnknownBone {
                        bone: bone_id.clone(),
                    },
                )?;
            let transform = bone_transforms
                .get(bone_ordinal)
                .ok_or_else(
                    || CharacterBinaryFbxError::MissingBoneTransform {
                        bone: bone_id.clone(),
                    },
                )?;
            let id = cluster_id(
                group_ordinal,
                bone_ordinal,
            )
            .map_err(CharacterBinaryFbxError::from)?;
            children.push(
                cluster_node(
                    id,
                    &group.object_name,
                    bone_id,
                    group.influences,
                    &transform.global_bind,
                )?,
            );
        }
    }
    children.push(
        bind_pose_node(
            groups,
            bone_transforms,
        )?,
    );
    children.extend(
        animation_plan
            .objects
            .iter()
            .cloned(),
    );
    Ok(
        BinaryNode::branch(
            "Objects", children,
        ),
    )
}

/// Expand triangle indices into FBX polygon-vertex sentinel encoding.
// The FBX format encodes each polygon end as `-vertex - 1` after domain index
// validation proves the conversion remains within the signed 64-bit range.
#[expect(
    clippy::arithmetic_side_effects,
    reason = "Validated FBX polygon-end encoding requires checked negation \
              and               subtraction for the final vertex sentinel."
)]
fn polygon_vertex_indices(group: &PrimitiveGroup) -> Vec<i64> {
    let mut output = Vec::with_capacity(
        group
            .triangles
            .len()
            * 3,
    );
    for triangle in &group.triangles {
        output.push(i64::from(triangle[0]));
        output.push(i64::from(triangle[1]));
        output.push(-i64::from(triangle[2]) - 1);
    }
    output
}

/// Flatten triangle vertices into the primary UV index stream.
fn uv_indices(group: &PrimitiveGroup) -> Vec<u32> {
    group
        .triangles
        .iter()
        .flat_map(
            |triangle| {
                triangle
                    .iter()
                    .copied()
            },
        )
        .collect()
}

/// Build one mesh geometry object with normals, UVs, and material mapping.
fn geometry_node(
    flattened: &BinaryGroup<'_>
) -> Result<BinaryNode, CharacterBinaryFbxError> {
    let group = flattened.group;
    let positions = group
        .positions
        .iter()
        .flat_map(
            |position| {
                position
                    .iter()
                    .copied()
                    .map(f64::from)
            },
        )
        .collect();
    let polygon_indices = polygon_vertex_indices(group)
        .into_iter()
        .map(
            |value| {
                i32::try_from(value).map_err(
                    |_conversion_error| {
                        CharacterBinaryFbxError::IndexExceedsI32 {
                            context: "polygon vertex index",
                            value,
                        }
                    },
                )
            },
        )
        .collect::<Result<Vec<_>, _>>()?;
    let mut children = vec![
        BinaryNode::leaf(
            "Vertices",
            vec![BinaryProperty::F64Array(positions)],
        ),
        BinaryNode::leaf(
            "PolygonVertexIndex",
            vec![BinaryProperty::I32Array(polygon_indices)],
        ),
        i32_node(
            "GeometryVersion",
            124,
        ),
    ];
    if group.has_normals() {
        children.push(normal_layer(group)?);
    }
    if group.has_uvs() {
        children.push(uv_layer(group)?);
    }
    if group.has_colors() {
        children.push(color_layer(group)?);
    }
    children.push(material_layer());
    children.push(layer_node(group));
    Ok(
        BinaryNode::new(
            "Geometry",
            vec![
                id_property(
                    flattened
                        .ids
                        .geometry,
                )?,
                name_class(
                    &flattened.object_name,
                    "Geometry",
                ),
                string("Mesh"),
            ],
            children,
        ),
    )
}

/// Build one per-polygon-corner normal layer.
fn normal_layer(
    group: &PrimitiveGroup
) -> Result<BinaryNode, CharacterBinaryFbxError> {
    let mut normals = Vec::with_capacity(
        group
            .triangles
            .len()
            .checked_mul(9)
            .ok_or(
                CharacterBinaryFbxError::CountOverflow {
                    context: "corner normals",
                },
            )?,
    );
    for triangle in &group.triangles {
        for vertex in triangle {
            let index = usize::try_from(*vertex).map_err(
                |_conversion_error| {
                    CharacterBinaryFbxError::IndexExceedsUsize {
                        context: "normal vertex",
                        value: u64::from(*vertex),
                    }
                },
            )?;
            let normal = group
                .normals
                .get(index)
                .ok_or(
                    CharacterBinaryFbxError::VertexOutOfBounds {
                        context: "normal vertex",
                        vertex: index,
                        vertices: group
                            .normals
                            .len(),
                    },
                )?;
            normals.extend(
                normal
                    .iter()
                    .copied()
                    .map(f64::from),
            );
        }
    }
    Ok(
        BinaryNode::new(
            "LayerElementNormal",
            vec![BinaryProperty::I32(0)],
            vec![
                i32_node(
                    "Version", 101,
                ),
                string_node(
                    "Name", "",
                ),
                string_node(
                    "MappingInformationType",
                    "ByPolygonVertex",
                ),
                string_node(
                    "ReferenceInformationType",
                    "Direct",
                ),
                BinaryNode::leaf(
                    "Normals",
                    vec![BinaryProperty::F64Array(normals)],
                ),
            ],
        ),
    )
}

/// Build one primary UV layer preserving source orientation.
fn uv_layer(
    group: &PrimitiveGroup
) -> Result<BinaryNode, CharacterBinaryFbxError> {
    let coordinates = group
        .uvs
        .iter()
        .flat_map(
            |uv| {
                [
                    f64::from(uv[0]),
                    f64::from(uv[1]),
                ]
            },
        )
        .collect();
    let indices = uv_indices(group)
        .into_iter()
        .map(
            |value| {
                i32::try_from(value).map_err(
                    |_conversion_error| {
                        CharacterBinaryFbxError::IndexExceedsI32 {
                            context: "UV index",
                            value: i64::from(value),
                        }
                    },
                )
            },
        )
        .collect::<Result<Vec<_>, _>>()?;
    Ok(
        BinaryNode::new(
            "LayerElementUV",
            vec![BinaryProperty::I32(0)],
            vec![
                i32_node(
                    "Version", 101,
                ),
                string_node(
                    "Name",
                    "UVChannel_1",
                ),
                string_node(
                    "MappingInformationType",
                    "ByPolygonVertex",
                ),
                string_node(
                    "ReferenceInformationType",
                    "IndexToDirect",
                ),
                BinaryNode::leaf(
                    "UV",
                    vec![BinaryProperty::F64Array(coordinates)],
                ),
                BinaryNode::leaf(
                    "UVIndex",
                    vec![BinaryProperty::I32Array(indices)],
                ),
            ],
        ),
    )
}

/// Build one primary per-vertex color layer in normalized RGBA order.
fn color_layer(
    group: &PrimitiveGroup
) -> Result<BinaryNode, CharacterBinaryFbxError> {
    let colors = group
        .colors
        .iter()
        .flat_map(
            |color| {
                color
                    .iter()
                    .copied()
                    .map(f64::from)
            },
        )
        .collect();
    let indices = uv_indices(group)
        .into_iter()
        .map(
            |value| {
                i32::try_from(value).map_err(
                    |_conversion_error| {
                        CharacterBinaryFbxError::IndexExceedsI32 {
                            context: "color index",
                            value: i64::from(value),
                        }
                    },
                )
            },
        )
        .collect::<Result<Vec<_>, _>>()?;
    Ok(
        BinaryNode::new(
            "LayerElementColor",
            vec![BinaryProperty::I32(0)],
            vec![
                i32_node(
                    "Version", 101,
                ),
                string_node(
                    "Name",
                    "ColorSet_1",
                ),
                string_node(
                    "MappingInformationType",
                    "ByPolygonVertex",
                ),
                string_node(
                    "ReferenceInformationType",
                    "IndexToDirect",
                ),
                BinaryNode::leaf(
                    "Colors",
                    vec![BinaryProperty::F64Array(colors)],
                ),
                BinaryNode::leaf(
                    "ColorIndex",
                    vec![BinaryProperty::I32Array(indices)],
                ),
            ],
        ),
    )
}

/// Build the all-same material layer.
fn material_layer() -> BinaryNode {
    BinaryNode::new(
        "LayerElementMaterial",
        vec![BinaryProperty::I32(0)],
        vec![
            i32_node(
                "Version", 101,
            ),
            string_node(
                "Name", "",
            ),
            string_node(
                "MappingInformationType",
                "AllSame",
            ),
            string_node(
                "ReferenceInformationType",
                "IndexToDirect",
            ),
            BinaryNode::leaf(
                "Materials",
                vec![BinaryProperty::I32Array(vec![0])],
            ),
        ],
    )
}

/// Build layer metadata describing the available element channels.
fn layer_node(group: &PrimitiveGroup) -> BinaryNode {
    let mut elements = vec![
        i32_node(
            "Version", 100,
        ),
    ];
    if group.has_normals() {
        elements.push(layer_element("LayerElementNormal"));
    }
    if group.has_uvs() {
        elements.push(layer_element("LayerElementUV"));
    }
    if group.has_colors() {
        elements.push(layer_element("LayerElementColor"));
    }
    elements.push(layer_element("LayerElementMaterial"));
    BinaryNode::new(
        "Layer",
        vec![BinaryProperty::I32(0)],
        elements,
    )
}

/// Build one typed layer-element reference.
fn layer_element(element_type: &str) -> BinaryNode {
    BinaryNode::branch(
        "LayerElement",
        vec![
            string_node(
                "Type",
                element_type,
            ),
            i32_node(
                "TypedIndex",
                0,
            ),
        ],
    )
}

/// Build the export-space parent that reverses the authored forward direction.
fn export_root_node() -> Result<BinaryNode, CharacterBinaryFbxError> {
    model_node(
        EXPORT_ROOT_ID,
        "SHAR_Export_Root",
        "Null",
        &EXPORT_ROOT_TRANSFORM,
    )
}

/// Build one mesh model at the scene origin.
fn mesh_model_node(
    id: u64,
    name: &str,
) -> Result<BinaryNode, CharacterBinaryFbxError> {
    model_node(
        id,
        name,
        "Mesh",
        &TrsParts {
            translation: [
                0.0, 0.0, 0.0,
            ],
            rotation_degrees: [
                0.0, 0.0, 0.0,
            ],
            scale: [
                1.0, 1.0, 1.0,
            ],
        },
    )
}

/// Build one skeleton limb-node model.
fn limb_model_node(
    id: u64,
    name: &str,
    parts: &TrsParts,
) -> Result<BinaryNode, CharacterBinaryFbxError> {
    model_node(
        id, name, "LimbNode", parts,
    )
}

/// Build one model with local translation, rotation, and scale properties.
fn model_node(
    id: u64,
    name: &str,
    model_type: &str,
    parts: &TrsParts,
) -> Result<BinaryNode, CharacterBinaryFbxError> {
    Ok(
        BinaryNode::new(
            "Model",
            vec![
                id_property(id)?,
                name_class(
                    name, "Model",
                ),
                string(model_type),
            ],
            vec![
                i32_node(
                    "Version", 232,
                ),
                BinaryNode::branch(
                    "Properties70",
                    vec![
                        integer_property(
                            "DefaultAttributeIndex",
                            0,
                        ),
                        vector_property(
                            "Lcl Translation",
                            parts.translation,
                        ),
                        vector_property(
                            "Lcl Rotation",
                            parts.rotation_degrees,
                        ),
                        vector_property(
                            "Lcl Scaling",
                            parts.scale,
                        ),
                    ],
                ),
                BinaryNode::leaf(
                    "Shading",
                    vec![BinaryProperty::Bool(true)],
                ),
                string_node(
                    "Culling",
                    "CullingOff",
                ),
            ],
        ),
    )
}

/// Build one material object.
fn material_node(
    slot: &MaterialSlot<'_>
) -> Result<BinaryNode, CharacterBinaryFbxError> {
    Ok(
        BinaryNode::new(
            "Material",
            vec![
                id_property(
                    slot.ids
                        .material,
                )?,
                name_class(
                    &slot
                        .binding
                        .material_name,
                    "Material",
                ),
                string(""),
            ],
            vec![
                i32_node(
                    "Version", 102,
                ),
                string_node(
                    "ShadingModel",
                    "phong",
                ),
                i32_node(
                    "MultiLayer",
                    0,
                ),
                BinaryNode::branch(
                    "Properties70",
                    vec![
                        color_property(
                            "DiffuseColor",
                            [
                                0.8, 0.8, 0.8,
                            ],
                        ),
                        color_property(
                            "AmbientColor",
                            [
                                0.2, 0.2, 0.2,
                            ],
                        ),
                        color_property(
                            "SpecularColor",
                            [
                                0.0, 0.0, 0.0,
                            ],
                        ),
                        double_property(
                            "SpecularFactor",
                            0.0,
                        ),
                        double_property(
                            "Shininess",
                            0.0,
                        ),
                        color_property(
                            "ReflectionColor",
                            [
                                0.0, 0.0, 0.0,
                            ],
                        ),
                        double_property(
                            "ReflectionFactor",
                            0.0,
                        ),
                    ],
                ),
            ],
        ),
    )
}

/// Build the portable FBX path for one validated texture file name.
fn texture_relative_path(
    file_name: &str,
    storage: CharacterTextureStorage,
) -> String {
    match storage {
        CharacterTextureStorage::External => format!("textures/{file_name}"),
        CharacterTextureStorage::Embedded => file_name.to_owned(),
    }
}

/// Build one texture object referencing its video clip.
fn texture_node(
    slot: &MaterialSlot<'_>,
    relative_path: &str,
) -> Result<BinaryNode, CharacterBinaryFbxError> {
    let name = &slot
        .binding
        .material_name;
    Ok(
        BinaryNode::new(
            "Texture",
            vec![
                id_property(
                    slot.ids
                        .texture,
                )?,
                name_class(
                    name, "Texture",
                ),
                string(""),
            ],
            vec![
                string_node(
                    "Type",
                    "TextureVideoClip",
                ),
                i32_node(
                    "Version", 202,
                ),
                name_class_node(
                    "TextureName",
                    name,
                    "Texture",
                ),
                name_class_node(
                    "Media", name, "Video",
                ),
                string_node(
                    "FileName",
                    relative_path,
                ),
                string_node(
                    "RelativeFilename",
                    relative_path,
                ),
            ],
        ),
    )
}

/// Build one video clip with an optional compatibility payload.
fn video_node(
    slot: &MaterialSlot<'_>,
    relative_path: &str,
    content: Option<&[u8]>,
) -> Result<BinaryNode, CharacterBinaryFbxError> {
    let name = &slot
        .binding
        .material_name;
    let mut children = vec![
        string_node(
            "Type", "Clip",
        ),
        BinaryNode::branch(
            "Properties70",
            vec![
                xref_string_property(
                    "Path",
                    relative_path,
                ),
                xref_string_property(
                    "RelPath",
                    relative_path,
                ),
            ],
        ),
        i32_node(
            "UseMipMap",
            0,
        ),
        string_node(
            "Filename",
            relative_path,
        ),
        string_node(
            "RelativeFilename",
            relative_path,
        ),
    ];
    if let Some(payload) = content {
        children.push(
            BinaryNode::leaf(
                "Content",
                vec![BinaryProperty::Bytes(payload.to_vec())],
            ),
        );
    }
    Ok(
        BinaryNode::new(
            "Video",
            vec![
                id_property(
                    slot.ids
                        .video,
                )?,
                name_class(
                    name, "Video",
                ),
                string("Clip"),
            ],
            children,
        ),
    )
}

/// Build one skeleton node attribute.
fn limb_attribute_node(
    id: u64,
    name: &str,
) -> Result<BinaryNode, CharacterBinaryFbxError> {
    Ok(
        BinaryNode::new(
            "NodeAttribute",
            vec![
                id_property(id)?,
                name_class(
                    name,
                    "NodeAttribute",
                ),
                string("LimbNode"),
            ],
            vec![
                BinaryNode::branch(
                    "Properties70",
                    vec![
                        BinaryNode::leaf(
                            "P",
                            vec![
                                string("Size"),
                                string("double"),
                                string("Number"),
                                string(""),
                                BinaryProperty::F64(1.0),
                            ],
                        ),
                    ],
                ),
                string_node(
                    "TypeFlags",
                    "Skeleton",
                ),
            ],
        ),
    )
}

/// Build one skin deformer container.
fn skin_deformer_node(
    id: u64,
    name: &str,
) -> Result<BinaryNode, CharacterBinaryFbxError> {
    Ok(
        BinaryNode::new(
            "Deformer",
            vec![
                id_property(id)?,
                name_class(
                    name, "Deformer",
                ),
                string("Skin"),
            ],
            vec![
                i32_node(
                    "Version", 101,
                ),
                BinaryNode::leaf(
                    // cspell:disable-next-line -- Acuracy
                    "Link_DeformAcuracy",
                    vec![BinaryProperty::F64(50.0)],
                ),
            ],
        ),
    )
}

/// Build one skin cluster with index, weight, and bind arrays.
fn cluster_node(
    id: u64,
    group_name: &str,
    bone_id: &str,
    influences: &[crate::domain::skin::SkinInfluence],
    global_bind: &[f64; 16],
) -> Result<BinaryNode, CharacterBinaryFbxError> {
    let mut indexes = Vec::new();
    let mut weights = Vec::new();
    for influence in influences {
        if influence.bone_id != bone_id {
            continue;
        }
        indexes.push(
            i32::try_from(influence.vertex_index).map_err(
                |_conversion_error| CharacterBinaryFbxError::IndexExceedsI32 {
                    context: "cluster vertex index",
                    value: i64::from(influence.vertex_index),
                },
            )?,
        );
        weights.push(f64::from(influence.weight));
    }
    let inverse_bind = invert_affine(global_bind).map_err(
        |error| CharacterBinaryFbxError::UnsupportedBindMatrix {
            bone: bone_id.to_owned(),
            error,
        },
    )?;
    let export_root_bind = compose(&EXPORT_ROOT_TRANSFORM);
    let export_space_global_bind = multiply(
        global_bind,
        &export_root_bind,
    );
    Ok(
        BinaryNode::new(
            "Deformer",
            vec![
                id_property(id)?,
                name_class(
                    &format!("{group_name}_{bone_id}"),
                    "SubDeformer",
                ),
                string("Cluster"),
            ],
            vec![
                i32_node(
                    "Version", 100,
                ),
                BinaryNode::leaf(
                    "UserData",
                    vec![
                        string(""),
                        string(""),
                    ],
                ),
                BinaryNode::leaf(
                    "Indexes",
                    vec![BinaryProperty::I32Array(indexes)],
                ),
                BinaryNode::leaf(
                    "Weights",
                    vec![BinaryProperty::F64Array(weights)],
                ),
                BinaryNode::leaf(
                    "Transform",
                    vec![BinaryProperty::F64Array(inverse_bind.to_vec())],
                ),
                BinaryNode::leaf(
                    "TransformLink",
                    vec![
                        BinaryProperty::F64Array(
                            export_space_global_bind.to_vec(),
                        ),
                    ],
                ),
                BinaryNode::leaf(
                    "TransformAssociateModel",
                    vec![BinaryProperty::F64Array(export_root_bind.to_vec())],
                ),
            ],
        ),
    )
}

/// Build one world-space bind pose for the export root, meshes, and skeleton.
fn bind_pose_node(
    groups: &[BinaryGroup<'_>],
    bone_transforms: &[BoneTransform],
) -> Result<BinaryNode, CharacterBinaryFbxError> {
    let model_node_count = groups
        .len()
        .checked_add(bone_transforms.len())
        .ok_or(
            CharacterBinaryFbxError::CountOverflow {
                context: "bind pose model nodes",
            },
        )?;
    let node_count = model_node_count
        .checked_add(1)
        .ok_or(
            CharacterBinaryFbxError::CountOverflow {
                context: "bind pose nodes",
            },
        )?;
    let export_root_bind = compose(&EXPORT_ROOT_TRANSFORM);
    let mut children = vec![
        string_node(
            "Type", "BindPose",
        ),
        i32_node(
            "Version", 100,
        ),
        i32_node(
            "NbPoseNodes",
            count_i32(
                node_count,
                "bind pose nodes",
            )?,
        ),
        pose_node(
            EXPORT_ROOT_ID,
            &export_root_bind,
        )?,
    ];
    for group in groups {
        children.push(
            pose_node(
                group
                    .ids
                    .model,
                &export_root_bind,
            )?,
        );
    }
    for (ordinal, transform) in bone_transforms
        .iter()
        .enumerate()
    {
        let ids = bone_ids(ordinal).map_err(CharacterBinaryFbxError::from)?;
        let export_space_global_bind = multiply(
            &transform.global_bind,
            &export_root_bind,
        );
        let bone_pose = pose_node(
            ids.model,
            &export_space_global_bind,
        )?;
        children.push(bone_pose);
    }
    Ok(
        BinaryNode::new(
            "Pose",
            vec![
                id_property(POSE_ID)?,
                name_class(
                    "BindPose", "Pose",
                ),
                string("BindPose"),
            ],
            children,
        ),
    )
}

/// Build one pose-node matrix entry.
fn pose_node(
    node_id: u64,
    matrix: &[f64; 16],
) -> Result<BinaryNode, CharacterBinaryFbxError> {
    Ok(
        BinaryNode::branch(
            "PoseNode",
            vec![
                BinaryNode::leaf(
                    "Node",
                    vec![id_property(node_id)?],
                ),
                BinaryNode::leaf(
                    "Matrix",
                    vec![BinaryProperty::F64Array(matrix.to_vec())],
                ),
            ],
        ),
    )
}

/// Build every object and property connection in stable order.
// One ordered graph pass preserves parent-child direction across object kinds.
#[expect(
    clippy::too_many_lines,
    reason = "Ordered FBX links preserve deterministic parent and child graph \
              directions."
)]
fn connections(
    character: &CharacterAsset,
    groups: &[BinaryGroup<'_>],
    material_slots: &BTreeMap<String, MaterialSlot<'_>>,
    bone_ordinals: &BTreeMap<&str, usize>,
    animation_plan: &BinaryAnimationPlan,
) -> Result<BinaryNode, CharacterBinaryFbxError> {
    let mut children = vec![
        object_connection(
            EXPORT_ROOT_ID,
            0,
        )?,
    ];
    for group in groups {
        children.push(
            object_connection(
                group
                    .ids
                    .geometry,
                group
                    .ids
                    .model,
            )?,
        );
        children.push(
            object_connection(
                group
                    .ids
                    .model,
                EXPORT_ROOT_ID,
            )?,
        );
        let slot = material_slots
            .get(
                &group
                    .group
                    .shader,
            )
            .ok_or_else(
                || CharacterBinaryFbxError::MissingMaterialBinding {
                    shader: group
                        .group
                        .shader
                        .clone(),
                },
            )?;
        children.push(
            object_connection(
                slot.ids
                    .material,
                group
                    .ids
                    .model,
            )?,
        );
        children.push(
            object_connection(
                group
                    .ids
                    .deformer,
                group
                    .ids
                    .geometry,
            )?,
        );
    }
    for slot in material_slots.values() {
        if slot
            .binding
            .texture_file_name
            .is_some()
        {
            children.push(
                object_connection(
                    slot.ids
                        .video,
                    slot.ids
                        .texture,
                )?,
            );
            children.push(
                property_connection(
                    slot.ids
                        .texture,
                    slot.ids
                        .material,
                    "DiffuseColor",
                )?,
            );
        }
    }
    for (ordinal, bone) in character
        .bones
        .iter()
        .enumerate()
    {
        let ids = bone_ids(ordinal).map_err(CharacterBinaryFbxError::from)?;
        children.push(
            object_connection(
                ids.attribute,
                ids.model,
            )?,
        );
        let parent_id = match &bone.parent_id {
            Some(parent) => {
                let parent_ordinal = *bone_ordinals
                    .get(parent.as_str())
                    .ok_or_else(
                        || CharacterBinaryFbxError::UnknownBone {
                            bone: parent.clone(),
                        },
                    )?;
                bone_ids(parent_ordinal)
                    .map_err(CharacterBinaryFbxError::from)?
                    .model
            }
            None => EXPORT_ROOT_ID,
        };
        children.push(
            object_connection(
                ids.model, parent_id,
            )?,
        );
    }
    for (group_ordinal, group) in groups
        .iter()
        .enumerate()
    {
        for bone_id in &group.used_bones {
            let bone_ordinal = *bone_ordinals
                .get(bone_id.as_str())
                .ok_or_else(
                    || CharacterBinaryFbxError::UnknownBone {
                        bone: bone_id.clone(),
                    },
                )?;
            let cluster = cluster_id(
                group_ordinal,
                bone_ordinal,
            )
            .map_err(CharacterBinaryFbxError::from)?;
            children.push(
                object_connection(
                    cluster,
                    group
                        .ids
                        .deformer,
                )?,
            );
            let bone = bone_ids(bone_ordinal)
                .map_err(CharacterBinaryFbxError::from)?;
            children.push(
                object_connection(
                    bone.model, cluster,
                )?,
            );
        }
    }
    children.extend(
        animation_plan
            .connections
            .iter()
            .cloned(),
    );
    Ok(
        BinaryNode::branch(
            "Connections",
            children,
        ),
    )
}

/// Build one object-to-object connection.
fn object_connection(
    child: u64,
    parent: u64,
) -> Result<BinaryNode, CharacterBinaryFbxError> {
    Ok(
        BinaryNode::leaf(
            "C",
            vec![
                string("OO"),
                id_property(child)?,
                id_property(parent)?,
            ],
        ),
    )
}

/// Build one object-to-property connection.
fn property_connection(
    child: u64,
    parent: u64,
    property: &str,
) -> Result<BinaryNode, CharacterBinaryFbxError> {
    Ok(
        BinaryNode::leaf(
            "C",
            vec![
                string("OP"),
                id_property(child)?,
                id_property(parent)?,
                string(property),
            ],
        ),
    )
}

/// Build one vector property entry.
fn vector_property(
    name: &str,
    value: [f64; 3],
) -> BinaryNode {
    BinaryNode::leaf(
        "P",
        vec![
            string(name),
            string(name),
            string(""),
            string("A"),
            BinaryProperty::F64(value[0]),
            BinaryProperty::F64(value[1]),
            BinaryProperty::F64(value[2]),
        ],
    )
}

/// Build one RGB color property entry.
fn color_property(
    name: &str,
    value: [f64; 3],
) -> BinaryNode {
    BinaryNode::leaf(
        "P",
        vec![
            string(name),
            string("ColorRGB"),
            string("Color"),
            string(""),
            BinaryProperty::F64(value[0]),
            BinaryProperty::F64(value[1]),
            BinaryProperty::F64(value[2]),
        ],
    )
}

/// Build one integer property entry.
fn integer_property(
    name: &str,
    value: i32,
) -> BinaryNode {
    BinaryNode::leaf(
        "P",
        vec![
            string(name),
            string("int"),
            string("Integer"),
            string(""),
            BinaryProperty::I32(value),
        ],
    )
}

/// Build one enum property entry.
fn enum_property(
    name: &str,
    value: i32,
) -> BinaryNode {
    BinaryNode::leaf(
        "P",
        vec![
            string(name),
            string("enum"),
            string(""),
            string(""),
            BinaryProperty::I32(value),
        ],
    )
}

/// Build one double property entry.
fn double_property(
    name: &str,
    value: f64,
) -> BinaryNode {
    BinaryNode::leaf(
        "P",
        vec![
            string(name),
            string("double"),
            string("Number"),
            string(""),
            BinaryProperty::F64(value),
        ],
    )
}

/// Build one time property entry.
fn time_property(
    name: &str,
    value: i64,
) -> BinaryNode {
    BinaryNode::leaf(
        "P",
        vec![
            string(name),
            string("KTime"),
            string("Time"),
            string(""),
            BinaryProperty::I64(value),
        ],
    )
}

/// Build one external-reference string property entry.
fn xref_string_property(
    name: &str,
    value: &str,
) -> BinaryNode {
    BinaryNode::leaf(
        "P",
        vec![
            string(name),
            string("KString"),
            string("XRefUrl"),
            string(""),
            string(value),
        ],
    )
}

/// Build one string property entry.
fn string_property(
    name: &str,
    value: &str,
) -> BinaryNode {
    BinaryNode::leaf(
        "P",
        vec![
            string(name),
            string("KString"),
            string(""),
            string(""),
            string(value),
        ],
    )
}

/// Build one scalar i32 node.
fn i32_node(
    name: &str,
    value: i32,
) -> BinaryNode {
    BinaryNode::leaf(
        name,
        vec![BinaryProperty::I32(value)],
    )
}

/// Build one scalar string node.
fn string_node(
    name: &str,
    value: &str,
) -> BinaryNode {
    BinaryNode::leaf(
        name,
        vec![string(value)],
    )
}

/// Build one owned string property.
fn string(value: &str) -> BinaryProperty {
    BinaryProperty::String(value.to_owned())
}

/// Build one binary FBX object name with its class separator.
fn name_class(
    name: &str,
    class: &str,
) -> BinaryProperty {
    BinaryProperty::String(format!("{name}\0\x01{class}"))
}

/// Build one scalar binary FBX name-and-class node.
fn name_class_node(
    name: &str,
    value: &str,
    class: &str,
) -> BinaryNode {
    BinaryNode::leaf(
        name,
        vec![
            name_class(
                value, class,
            ),
        ],
    )
}

/// Convert one deterministic unsigned object id to an FBX signed id.
fn id_property(id: u64) -> Result<BinaryProperty, CharacterBinaryFbxError> {
    let narrowed = i64::try_from(id).map_err(
        |_conversion_error| CharacterBinaryFbxError::IdExceedsI64 {
            id,
        },
    )?;
    Ok(BinaryProperty::I64(narrowed))
}

/// Convert one object count to an FBX signed 32-bit count.
fn count_i32(
    count: usize,
    context: &'static str,
) -> Result<i32, CharacterBinaryFbxError> {
    i32::try_from(count).map_err(
        |_conversion_error| CharacterBinaryFbxError::CountExceedsI32 {
            context,
            count,
        },
    )
}

/// Persist one complete binary document without overwriting existing output.
fn persist(
    path: &Path,
    bytes: &[u8],
) -> Result<(), CharacterBinaryFbxError> {
    let Some(parent) = path.parent() else {
        return Err(
            CharacterBinaryFbxError::MissingParent(
                path.display()
                    .to_string(),
            ),
        );
    };
    local::create_dir_all(parent).map_err(
        |source| CharacterBinaryFbxError::CreateDir {
            path: parent
                .display()
                .to_string(),
            source: source.to_string(),
        },
    )?;
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(
            |source| {
                if source.kind() == ErrorKind::AlreadyExists {
                    CharacterBinaryFbxError::OutputExists(
                        path.display()
                            .to_string(),
                    )
                } else {
                    CharacterBinaryFbxError::Write {
                        path: path
                            .display()
                            .to_string(),
                        source: source.to_string(),
                    }
                }
            },
        )?;
    file.write_all(bytes)
        .map_err(
            |source| CharacterBinaryFbxError::Write {
                path: path
                    .display()
                    .to_string(),
                source: source.to_string(),
            },
        )
}

/// Binary character FBX serialization failure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CharacterBinaryFbxError {
    /// Character serializer preparation rejected the aggregate or identity.
    CharacterInput {
        /// Stable debug representation of the checked input failure.
        reason: String,
    },
    /// Binary animation planning rejected timing, ids, or transforms.
    AnimationPlan {
        /// Stable debug representation of the checked planning failure.
        reason: String,
    },
    /// Binary container encoding failed.
    Encoding {
        /// Stable debug representation of the checked encoder failure.
        reason: String,
    },
    /// One object-family count overflowed platform arithmetic.
    CountOverflow {
        /// Count operation that overflowed.
        context: &'static str,
    },
    /// One count did not fit an FBX signed 32-bit field.
    CountExceedsI32 {
        /// Count field being narrowed.
        context: &'static str,
        /// Rejected count.
        count: usize,
    },
    /// One unsigned object id did not fit an FBX signed 64-bit field.
    IdExceedsI64 {
        /// Rejected object id.
        id: u64,
    },
    /// One signed index did not fit an FBX signed 32-bit array element.
    IndexExceedsI32 {
        /// Index family being narrowed.
        context: &'static str,
        /// Rejected index value.
        value: i64,
    },
    /// One unsigned index did not fit the host collection index width.
    IndexExceedsUsize {
        /// Index family being narrowed.
        context: &'static str,
        /// Rejected index value.
        value: u64,
    },
    /// One vertex index escaped its validated source array.
    VertexOutOfBounds {
        /// Array role being indexed.
        context: &'static str,
        /// Rejected vertex index.
        vertex: usize,
        /// Available vertex count.
        vertices: usize,
    },
    /// A material binding disappeared after serializer input validation.
    MissingMaterialBinding {
        /// Shader identity without a binding.
        shader: String,
    },
    /// One expected skeleton bone was absent from the ordinal map.
    UnknownBone {
        /// Missing bone identity.
        bone: String,
    },
    /// One precomputed bone transform was absent.
    MissingBoneTransform {
        /// Bone identity without a transform.
        bone: String,
    },
    /// One global bind matrix could not be inverted for its cluster.
    UnsupportedBindMatrix {
        /// Bone identity with the unsupported bind matrix.
        bone: String,
        /// Affine inversion failure.
        error: InverseError,
    },
    /// One embedded texture file name was not one portable path segment.
    InvalidEmbeddedTextureName {
        /// Rejected file name.
        file_name: String,
    },
    /// One embedded texture did not contain a PNG payload.
    InvalidEmbeddedTextureContent {
        /// Rejected file name.
        file_name: String,
    },
    /// Two embedded payloads used the same file name.
    DuplicateEmbeddedTexture {
        /// Duplicated file name.
        file_name: String,
    },
    /// A referenced material texture had no embedded payload.
    MissingEmbeddedTexture {
        /// Missing file name.
        file_name: String,
    },
    /// An embedded payload was not referenced by any material.
    UnexpectedEmbeddedTexture {
        /// Unexpected file name.
        file_name: String,
    },
    /// Output path already contains an artifact owned by another operation.
    OutputExists(String),
    /// Output path did not have a parent directory.
    MissingParent(String),
    /// Output directory could not be created.
    CreateDir {
        /// Directory path.
        path: String,
        /// IO error detail.
        source: String,
    },
    /// FBX file could not be written.
    Write {
        /// FBX path.
        path: String,
        /// IO error detail.
        source: String,
    },
}

impl From<CharacterInputError> for CharacterBinaryFbxError {
    fn from(error: CharacterInputError) -> Self {
        Self::CharacterInput {
            reason: format!("{error:?}"),
        }
    }
}

impl From<BinaryIdentityError> for CharacterBinaryFbxError {
    fn from(error: BinaryIdentityError) -> Self {
        Self::CharacterInput {
            reason: format!("{error:?}"),
        }
    }
}

#[cfg(test)]
#[test]
fn near_standard_rate_uses_custom_time_mode() {
    assert_eq!(
        frame_rate_time_mode(30.000_000_000_5_f64),
        14_i32
    );
}
