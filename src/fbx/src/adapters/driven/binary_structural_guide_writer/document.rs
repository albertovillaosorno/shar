// File:
//   - document.rs
// Path:
//   - src/fbx/src/adapters/driven/binary_structural_guide_writer/document.rs
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
//   - The fixed FBX 7.7 document graph for one structural-guide mesh.
// - Must-Not:
//   - Modify geometry, add helpers, animations, cameras, lights, or embedded
//     content.
// - Allows:
//   - The canonical world FBX Y-up meter settings, one shared ReflectX export
//     root, one geometry/model/material/texture/video, and deterministic
//     connections.
// - Summary:
//   - Builds the exact canonical structural-guide FBX scene graph.
//
// LARGE-FILE:
// - owner: Structural-guide FBX adapter
// - reason: Header, global settings, definitions, objects, and connections form
//   one fixed interoperable document contract.
// - split: Geometry arrays and reusable node spelling remain child modules.
// - validation: Binary node tests and independent import verification.
// - review: Split if another scene profile shares these settings.
//

//! Fixed FBX 7.7 structural-guide document graph.

use super::super::binary_fbx::{
    BinaryNode, BinaryProperty, CREATION_TIME, DETERMINISTIC_FILE_ID,
};
use super::geometry::geometry_node;
use super::nodes::{
    EXPORT_ROOT_ID, GEOMETRY_ID, MATERIAL_ID, MODEL_ID, TEXTURE_ID, VIDEO_ID,
    color_property, export_root_node, i32_node, integer_property,
    material_node, model_node, object_connection, property,
    property_connection, string, string_node, texture_node, video_node,
};
use super::{StructuralGuideFbxError, StructuralGuideMesh};

pub(super) fn build_document(
    mesh: &StructuralGuideMesh,
) -> Result<Vec<BinaryNode>, StructuralGuideFbxError> {
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
                vec![string("SHAR")],
            ),
            global_settings(),
            documents(),
            BinaryNode::branch(
                "References",
                Vec::new(),
            ),
            definitions(),
            objects(mesh)?,
            connections()?,
            BinaryNode::branch(
                "Takes",
                Vec::new(),
            ),
        ],
    )
}

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

fn global_settings() -> BinaryNode {
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
                    property(
                        "UnitScaleFactor",
                        "double",
                        "Number",
                        vec![BinaryProperty::F64(100.0)],
                    ),
                    property(
                        "OriginalUnitScaleFactor",
                        "double",
                        "Number",
                        vec![BinaryProperty::F64(100.0)],
                    ),
                    color_property(
                        "AmbientColor",
                        [
                            0.0, 0.0, 0.0,
                        ],
                    ),
                    property(
                        "DefaultCamera",
                        "KString",
                        "",
                        vec![string("Producer Perspective")],
                    ),
                    integer_property(
                        "TimeMode", 11,
                    ),
                    property(
                        "TimeSpanStart",
                        "KTime",
                        "Time",
                        vec![BinaryProperty::I64(0)],
                    ),
                    property(
                        "TimeSpanStop",
                        "KTime",
                        "Time",
                        vec![BinaryProperty::I64(0)],
                    ),
                    property(
                        "CustomFrameRate",
                        "double",
                        "Number",
                        vec![BinaryProperty::F64(24.0)],
                    ),
                ],
            ),
        ],
    )
}

fn documents() -> BinaryNode {
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
                    string("Scene"),
                    string("Scene"),
                ],
                vec![
                    BinaryNode::branch(
                        "Properties70",
                        vec![
                            property(
                                "SourceObject",
                                "object",
                                "",
                                vec![],
                            ),
                            property(
                                "ActiveAnimStackName",
                                "KString",
                                "",
                                vec![string("")],
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

fn definitions() -> BinaryNode {
    let families = [
        (
            "GlobalSettings",
            1_i32,
        ),
        (
            "Geometry", 1_i32,
        ),
        (
            "Model", 2_i32,
        ),
        (
            "Material", 1_i32,
        ),
        (
            "Texture", 1_i32,
        ),
        (
            "Video", 1_i32,
        ),
    ];
    let mut children = vec![
        i32_node(
            "Version", 100,
        ),
        i32_node(
            "Count", 6,
        ),
    ];
    children.extend(
        families
            .into_iter()
            .map(
                |(family, count)| {
                    BinaryNode::new(
                        "ObjectType",
                        vec![string(family)],
                        vec![
                            i32_node(
                                "Count", count,
                            ),
                        ],
                    )
                },
            ),
    );
    BinaryNode::branch(
        "Definitions",
        children,
    )
}

fn objects(
    mesh: &StructuralGuideMesh,
) -> Result<BinaryNode, StructuralGuideFbxError> {
    Ok(
        BinaryNode::branch(
            "Objects",
            vec![
                export_root_node()?,
                geometry_node(mesh)?,
                model_node()?,
                material_node()?,
                texture_node()?,
                video_node()?,
            ],
        ),
    )
}

fn connections() -> Result<BinaryNode, StructuralGuideFbxError> {
    Ok(
        BinaryNode::branch(
            "Connections",
            vec![
                object_connection(
                    GEOMETRY_ID,
                    MODEL_ID,
                )?,
                object_connection(
                    MODEL_ID,
                    EXPORT_ROOT_ID,
                )?,
                object_connection(
                    EXPORT_ROOT_ID,
                    0,
                )?,
                object_connection(
                    MATERIAL_ID,
                    MODEL_ID,
                )?,
                property_connection(
                    TEXTURE_ID,
                    MATERIAL_ID,
                    "DiffuseColor",
                )?,
                object_connection(
                    VIDEO_ID, TEXTURE_ID,
                )?,
            ],
        ),
    )
}
