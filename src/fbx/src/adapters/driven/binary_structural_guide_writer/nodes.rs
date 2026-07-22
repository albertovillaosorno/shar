// File:
//   - nodes.rs
// Path:
//   - src/fbx/src/adapters/driven/binary_structural_guide_writer/nodes.rs
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
//   - Deterministic FBX node constructors for the structural guide.
// - Must-Not:
//   - Validate topology, choose source content, or write files.
// - Allows:
//   - Typed properties, scene objects, materials, textures, and connections.
// - Summary:
//   - Keeps low-level FBX spelling out of guide document planning.
//
// LARGE-FILE:
// - owner: Structural-guide FBX adapter
// - reason: FBX property and object spellings form one serializer vocabulary.
// - split: Geometry arrays and document planning remain separate.
// - validation: Structural-guide writer tests and Blender import audit.
// - review: Split if another guide material representation appears.
//

//! Serializer-local structural-guide FBX node constructors.

use super::super::binary_fbx::{BinaryNode, BinaryProperty};
use super::{
    STRUCTURAL_GUIDE_ASSET_NAME, STRUCTURAL_GUIDE_MATERIAL_NAME,
    STRUCTURAL_GUIDE_TEXTURE_PATH, StructuralGuideFbxError,
};

pub(super) const GEOMETRY_ID: u64 = 1_100_001;
pub(super) const MODEL_ID: u64 = 1_100_002;
pub(super) const MATERIAL_ID: u64 = 1_100_003;
pub(super) const TEXTURE_ID: u64 = 1_100_004;
pub(super) const VIDEO_ID: u64 = 1_100_005;

pub(super) fn i32_node(
    name: &str,
    value: i32,
) -> BinaryNode {
    BinaryNode::leaf(
        name,
        vec![BinaryProperty::I32(value)],
    )
}

pub(super) fn string_node(
    name: &str,
    value: &str,
) -> BinaryNode {
    BinaryNode::leaf(
        name,
        vec![string(value)],
    )
}

pub(super) fn string(value: &str) -> BinaryProperty {
    BinaryProperty::String(value.to_owned())
}

pub(super) fn id_property(
    id: u64
) -> Result<BinaryProperty, StructuralGuideFbxError> {
    i64::try_from(id)
        .map(BinaryProperty::I64)
        .map_err(|error| StructuralGuideFbxError::Encoding(error.to_string()))
}

pub(super) fn name_class(
    name: &str,
    class: &str,
) -> BinaryProperty {
    string(&format!("{name}\u{0}\u{1}{class}"))
}

pub(super) fn name_class_node(
    node_name: &str,
    name: &str,
    class: &str,
) -> BinaryNode {
    BinaryNode::leaf(
        node_name,
        vec![
            name_class(
                name, class,
            ),
        ],
    )
}

pub(super) fn property(
    name: &str,
    type_name: &str,
    flags: &str,
    values: Vec<BinaryProperty>,
) -> BinaryNode {
    let mut properties = vec![
        string(name),
        string(type_name),
        string(""),
        string(flags),
    ];
    properties.extend(values);
    BinaryNode::leaf(
        "P", properties,
    )
}

pub(super) fn integer_property(
    name: &str,
    value: i32,
) -> BinaryNode {
    property(
        name,
        "int",
        "Integer",
        vec![BinaryProperty::I32(value)],
    )
}

pub(super) fn double_property(
    name: &str,
    value: f64,
) -> BinaryNode {
    property(
        name,
        "double",
        "Number",
        vec![BinaryProperty::F64(value)],
    )
}

pub(super) fn vector_property(
    name: &str,
    value: [f64; 3],
) -> BinaryNode {
    property(
        name,
        name,
        "A",
        value
            .into_iter()
            .map(BinaryProperty::F64)
            .collect(),
    )
}

pub(super) fn color_property(
    name: &str,
    value: [f64; 3],
) -> BinaryNode {
    property(
        name,
        "Color",
        "",
        value
            .into_iter()
            .map(BinaryProperty::F64)
            .collect(),
    )
}

pub(super) fn xref_string_property(
    name: &str,
    value: &str,
) -> BinaryNode {
    property(
        name,
        "KString",
        "XRefUrl",
        vec![string(value)],
    )
}

pub(super) fn layer_element(
    element_type: &str,
    typed_index: i32,
) -> BinaryNode {
    BinaryNode::branch(
        "LayerElement",
        vec![
            string_node(
                "Type",
                element_type,
            ),
            i32_node(
                "TypedIndex",
                typed_index,
            ),
        ],
    )
}

pub(super) fn model_node() -> Result<BinaryNode, StructuralGuideFbxError> {
    Ok(
        BinaryNode::new(
            "Model",
            vec![
                id_property(MODEL_ID)?,
                name_class(
                    STRUCTURAL_GUIDE_ASSET_NAME,
                    "Model",
                ),
                string("Mesh"),
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
                            [
                                0.0, 0.0, 0.0,
                            ],
                        ),
                        vector_property(
                            "Lcl Rotation",
                            [
                                0.0, 0.0, 0.0,
                            ],
                        ),
                        vector_property(
                            "Lcl Scaling",
                            [
                                1.0, 1.0, 1.0,
                            ],
                        ),
                        property(
                            "Visibility",
                            "Visibility",
                            "A",
                            vec![BinaryProperty::F64(1.0)],
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

pub(super) fn material_node() -> Result<BinaryNode, StructuralGuideFbxError> {
    Ok(
        BinaryNode::new(
            "Material",
            vec![
                id_property(MATERIAL_ID)?,
                name_class(
                    STRUCTURAL_GUIDE_MATERIAL_NAME,
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
                    "lambert",
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
                                1.0, 1.0, 1.0,
                            ],
                        ),
                        color_property(
                            "AmbientColor",
                            [
                                0.0, 0.0, 0.0,
                            ],
                        ),
                        color_property(
                            "EmissiveColor",
                            [
                                0.0, 0.0, 0.0,
                            ],
                        ),
                        double_property(
                            "EmissiveFactor",
                            0.0,
                        ),
                        double_property(
                            "TransparencyFactor",
                            0.0,
                        ),
                    ],
                ),
            ],
        ),
    )
}

pub(super) fn texture_node() -> Result<BinaryNode, StructuralGuideFbxError> {
    Ok(
        BinaryNode::new(
            "Texture",
            vec![
                id_property(TEXTURE_ID)?,
                name_class(
                    STRUCTURAL_GUIDE_MATERIAL_NAME,
                    "Texture",
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
                    STRUCTURAL_GUIDE_MATERIAL_NAME,
                    "Texture",
                ),
                name_class_node(
                    "Media",
                    STRUCTURAL_GUIDE_MATERIAL_NAME,
                    "Video",
                ),
                string_node(
                    "FileName",
                    STRUCTURAL_GUIDE_TEXTURE_PATH,
                ),
                string_node(
                    "RelativeFilename",
                    STRUCTURAL_GUIDE_TEXTURE_PATH,
                ),
                BinaryNode::branch(
                    "Properties70",
                    vec![
                        integer_property(
                            "CurrentTextureBlendMode",
                            1,
                        ),
                        double_property(
                            "TranslationU",
                            0.0,
                        ),
                        double_property(
                            "TranslationV",
                            0.0,
                        ),
                        double_property(
                            "ScalingU", 1.0,
                        ),
                        double_property(
                            "ScalingV", 1.0,
                        ),
                        integer_property(
                            "WrapModeU",
                            1,
                        ),
                        integer_property(
                            "WrapModeV",
                            1,
                        ),
                    ],
                ),
            ],
        ),
    )
}

pub(super) fn video_node() -> Result<BinaryNode, StructuralGuideFbxError> {
    Ok(
        BinaryNode::new(
            "Video",
            vec![
                id_property(VIDEO_ID)?,
                name_class(
                    STRUCTURAL_GUIDE_MATERIAL_NAME,
                    "Video",
                ),
                string("Clip"),
            ],
            vec![
                string_node(
                    "Type", "Clip",
                ),
                BinaryNode::branch(
                    "Properties70",
                    vec![
                        xref_string_property(
                            "Path",
                            STRUCTURAL_GUIDE_TEXTURE_PATH,
                        ),
                        xref_string_property(
                            "RelPath",
                            STRUCTURAL_GUIDE_TEXTURE_PATH,
                        ),
                    ],
                ),
                i32_node(
                    "UseMipMap",
                    0,
                ),
                string_node(
                    "Filename",
                    STRUCTURAL_GUIDE_TEXTURE_PATH,
                ),
                string_node(
                    "RelativeFilename",
                    STRUCTURAL_GUIDE_TEXTURE_PATH,
                ),
            ],
        ),
    )
}

pub(super) fn object_connection(
    child: u64,
    parent: u64,
) -> Result<BinaryNode, StructuralGuideFbxError> {
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

pub(super) fn property_connection(
    child: u64,
    parent: u64,
    property_name: &str,
) -> Result<BinaryNode, StructuralGuideFbxError> {
    Ok(
        BinaryNode::leaf(
            "C",
            vec![
                string("OP"),
                id_property(child)?,
                id_property(parent)?,
                string(property_name),
            ],
        ),
    )
}
