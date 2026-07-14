// File:
//   - schema.rs
// Path:
//   - src/p3d/src/schema.rs
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
//   - p3d module behavior for schema.
// - Must-Not:
//   - Violate repository architecture, path, provenance, or output rules.
// - Allows:
//   - Operations required to validate and execute schema.
// - Split-When:
//   - Split when schema contains two independently testable contracts.
// - Merge-When:
//   - Another p3d module owns the same module boundary with no distinct
//   - invariant.
// - Summary:
//   - Chunk id constant count.
// - Description:
//   - Defines schema data and behavior for p3d root.
// - Usage:
//   - Used by p3d root code that needs schema.
// - Defaults:
//   - No implicit output outside the repository is allowed.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - true
//   - Reason: src/p3d/src/schema.rs has 13380 effective lines after the
//   - required
//   - header and remains cohesive until a focused split lands.
//

//! Chunk id constant count.
//!
//! This boundary keeps chunk id constant count explicit and returns
//! deterministic results to p3d callers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Schemafile.
// The explicit schema file name distinguishes file summaries from field rows.
#[expect(
    clippy::module_name_repetitions,
    reason = "Callers need explicit schema file summaries across generated \
              taxonomy boundaries."
)]
pub struct SchemaFile {
    /// Schema key.
    pub schema_key: &'static str,
    /// Chunk count.
    pub chunk_count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Schemafield.
// The explicit schema field name distinguishes field rows from chunk records.
#[expect(
    clippy::module_name_repetitions,
    reason = "Callers need explicit schema field rows across generated \
              taxonomy boundaries."
)]
pub struct SchemaField {
    /// Ty.
    pub ty: &'static str,
    /// Name.
    pub name: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Chunkidconstant.
pub struct ChunkIdConstant {
    /// Authority key.
    pub authority_key: &'static str,
    /// Scope.
    pub scope: &'static str,
    /// Name.
    pub name: &'static str,
    /// Value.
    pub value: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Schemachunk.
pub struct ChunkDefinition {
    /// Schema key.
    pub schema_key: &'static str,
    /// Name.
    pub name: &'static str,
    /// Chunk id expr.
    pub chunk_id_expr: &'static str,
    /// Subchunks.
    pub subchunks: &'static [&'static str],
    /// Fields.
    pub fields: &'static [SchemaField],
}

/// Schema file count.
pub const SCHEMA_FILE_COUNT: usize = 88;
/// Schema chunk count.
pub const SCHEMA_CHUNK_COUNT: usize = 293;
/// Chunk id constant count.
pub const CHUNK_ID_CONSTANT_COUNT: usize = 734;

/// Schema files.
pub const SCHEMA_FILES: &[SchemaFile] = &[
    SchemaFile {
        schema_key: "schema_0001",
        chunk_count: 3,
    },
    SchemaFile {
        schema_key: "schema_0002",
        chunk_count: 4,
    },
    SchemaFile {
        schema_key: "schema_0003",
        chunk_count: 1,
    },
    SchemaFile {
        schema_key: "schema_0004",
        chunk_count: 1,
    },
    SchemaFile {
        schema_key: "schema_0005",
        chunk_count: 1,
    },
    SchemaFile {
        schema_key: "schema_0006",
        chunk_count: 1,
    },
    SchemaFile {
        schema_key: "schema_0007",
        chunk_count: 1,
    },
    SchemaFile {
        schema_key: "schema_0008",
        chunk_count: 1,
    },
    SchemaFile {
        schema_key: "schema_0009",
        chunk_count: 5,
    },
    SchemaFile {
        schema_key: "schema_0010",
        chunk_count: 1,
    },
    SchemaFile {
        schema_key: "schema_0011",
        chunk_count: 1,
    },
    SchemaFile {
        schema_key: "schema_0012",
        chunk_count: 8,
    },
    SchemaFile {
        schema_key: "schema_0013",
        chunk_count: 1,
    },
    SchemaFile {
        schema_key: "schema_0014",
        chunk_count: 17,
    },
    SchemaFile {
        schema_key: "schema_0015",
        chunk_count: 12,
    },
    SchemaFile {
        schema_key: "schema_0016",
        chunk_count: 8,
    },
    SchemaFile {
        schema_key: "schema_0017",
        chunk_count: 5,
    },
    SchemaFile {
        schema_key: "schema_0018",
        chunk_count: 1,
    },
    SchemaFile {
        schema_key: "schema_0019",
        chunk_count: 1,
    },
    SchemaFile {
        schema_key: "schema_0020",
        chunk_count: 1,
    },
    SchemaFile {
        schema_key: "schema_0021",
        chunk_count: 3,
    },
    SchemaFile {
        schema_key: "schema_0022",
        chunk_count: 4,
    },
    SchemaFile {
        schema_key: "schema_0023",
        chunk_count: 3,
    },
    SchemaFile {
        schema_key: "schema_0024",
        chunk_count: 1,
    },
    SchemaFile {
        schema_key: "schema_0025",
        chunk_count: 1,
    },
    SchemaFile {
        schema_key: "schema_0026",
        chunk_count: 1,
    },
    SchemaFile {
        schema_key: "schema_0027",
        chunk_count: 4,
    },
    SchemaFile {
        schema_key: "schema_0028",
        chunk_count: 8,
    },
    SchemaFile {
        schema_key: "schema_0029",
        chunk_count: 1,
    },
    SchemaFile {
        schema_key: "schema_0030",
        chunk_count: 4,
    },
    SchemaFile {
        schema_key: "schema_0031",
        chunk_count: 1,
    },
    SchemaFile {
        schema_key: "schema_0032",
        chunk_count: 1,
    },
    SchemaFile {
        schema_key: "schema_0033",
        chunk_count: 6,
    },
    SchemaFile {
        schema_key: "schema_0034",
        chunk_count: 1,
    },
    SchemaFile {
        schema_key: "schema_0035",
        chunk_count: 4,
    },
    SchemaFile {
        schema_key: "schema_0036",
        chunk_count: 1,
    },
    SchemaFile {
        schema_key: "schema_0037",
        chunk_count: 1,
    },
    SchemaFile {
        schema_key: "schema_0038",
        chunk_count: 1,
    },
    SchemaFile {
        schema_key: "schema_0039",
        chunk_count: 1,
    },
    SchemaFile {
        schema_key: "schema_0040",
        chunk_count: 1,
    },
    SchemaFile {
        schema_key: "schema_0041",
        chunk_count: 1,
    },
    SchemaFile {
        schema_key: "schema_0042",
        chunk_count: 1,
    },
    SchemaFile {
        schema_key: "schema_0043",
        chunk_count: 1,
    },
    SchemaFile {
        schema_key: "schema_0044",
        chunk_count: 5,
    },
    SchemaFile {
        schema_key: "schema_0045",
        chunk_count: 8,
    },
    SchemaFile {
        schema_key: "schema_0046",
        chunk_count: 1,
    },
    SchemaFile {
        schema_key: "schema_0047",
        chunk_count: 1,
    },
    SchemaFile {
        schema_key: "schema_0048",
        chunk_count: 1,
    },
    SchemaFile {
        schema_key: "schema_0049",
        chunk_count: 2,
    },
    SchemaFile {
        schema_key: "schema_0050",
        chunk_count: 3,
    },
    SchemaFile {
        schema_key: "schema_0051",
        chunk_count: 1,
    },
    SchemaFile {
        schema_key: "schema_0052",
        chunk_count: 6,
    },
    SchemaFile {
        schema_key: "schema_0053",
        chunk_count: 9,
    },
    SchemaFile {
        schema_key: "schema_0054",
        chunk_count: 1,
    },
    SchemaFile {
        schema_key: "schema_0055",
        chunk_count: 1,
    },
    SchemaFile {
        schema_key: "schema_0056",
        chunk_count: 4,
    },
    SchemaFile {
        schema_key: "schema_0057",
        chunk_count: 1,
    },
    SchemaFile {
        schema_key: "schema_0058",
        chunk_count: 13,
    },
    SchemaFile {
        schema_key: "schema_0059",
        chunk_count: 21,
    },
    SchemaFile {
        schema_key: "schema_0060",
        chunk_count: 1,
    },
    SchemaFile {
        schema_key: "schema_0061",
        chunk_count: 1,
    },
    SchemaFile {
        schema_key: "schema_0062",
        chunk_count: 1,
    },
    SchemaFile {
        schema_key: "schema_0063",
        chunk_count: 11,
    },
    SchemaFile {
        schema_key: "schema_0064",
        chunk_count: 1,
    },
    SchemaFile {
        schema_key: "schema_0065",
        chunk_count: 19,
    },
    SchemaFile {
        schema_key: "schema_0066",
        chunk_count: 1,
    },
    SchemaFile {
        schema_key: "schema_0067",
        chunk_count: 8,
    },
    SchemaFile {
        schema_key: "schema_0068",
        chunk_count: 3,
    },
    SchemaFile {
        schema_key: "schema_0069",
        chunk_count: 4,
    },
    SchemaFile {
        schema_key: "schema_0070",
        chunk_count: 1,
    },
    SchemaFile {
        schema_key: "schema_0071",
        chunk_count: 9,
    },
    SchemaFile {
        schema_key: "schema_0072",
        chunk_count: 1,
    },
    SchemaFile {
        schema_key: "schema_0073",
        chunk_count: 6,
    },
    SchemaFile {
        schema_key: "schema_0074",
        chunk_count: 1,
    },
    SchemaFile {
        schema_key: "schema_0075",
        chunk_count: 1,
    },
    SchemaFile {
        schema_key: "schema_0076",
        chunk_count: 2,
    },
    SchemaFile {
        schema_key: "schema_0077",
        chunk_count: 1,
    },
    SchemaFile {
        schema_key: "schema_0078",
        chunk_count: 3,
    },
    SchemaFile {
        schema_key: "schema_0079",
        chunk_count: 5,
    },
    SchemaFile {
        schema_key: "schema_0080",
        chunk_count: 1,
    },
    SchemaFile {
        schema_key: "schema_0081",
        chunk_count: 2,
    },
    SchemaFile {
        schema_key: "schema_0082",
        chunk_count: 1,
    },
    SchemaFile {
        schema_key: "schema_0083",
        chunk_count: 1,
    },
    SchemaFile {
        schema_key: "schema_0084",
        chunk_count: 1,
    },
    SchemaFile {
        schema_key: "schema_0085",
        chunk_count: 1,
    },
    SchemaFile {
        schema_key: "schema_0086",
        chunk_count: 1,
    },
    SchemaFile {
        schema_key: "schema_0087",
        chunk_count: 1,
    },
    SchemaFile {
        schema_key: "schema_0088",
        chunk_count: 1,
    },
];

/// Subchunks 0.
const SUBCHUNKS_0: &[&str] = &["tlAnimatedObjectAnimationChunk"];
/// Fields 0.
const FIELDS_0: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "string",
        name: "BaseObjectName",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumAnimations",
    },
];
/// Subchunks 1.
const SUBCHUNKS_1: &[&str] = &[];
/// Fields 1.
const FIELDS_1: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "string",
        name: "FactoryName",
    },
    SchemaField {
        ty: "ULONG",
        name: "StartingAnimation",
    },
];
/// Subchunks 2.
const SUBCHUNKS_2: &[&str] = &["tlFrameControllerChunk"];
/// Fields 2.
const FIELDS_2: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "float",
        name: "FrameRate",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumControllers",
    },
];
/// Subchunks 3.
const SUBCHUNKS_3: &[&str] = &[
    "tlAnimationSizeChunk",
    "tlAnimationGroupListChunk",
];
/// Fields 3.
const FIELDS_3: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "FOURCC",
        name: "AnimationType",
    },
    SchemaField {
        ty: "float",
        name: "NumFrames",
    },
    SchemaField {
        ty: "float",
        name: "FrameRate",
    },
    SchemaField {
        ty: "ULONG",
        name: "Cyclic",
    },
];
/// Subchunks 4.
const SUBCHUNKS_4: &[&str] = &[];
/// Fields 4.
const FIELDS_4: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "ULONG",
        name: "PC",
    },
    SchemaField {
        ty: "ULONG",
        name: "PS2",
    },
    SchemaField {
        ty: "ULONG",
        name: "XBOX",
    },
    SchemaField {
        ty: "ULONG",
        name: "GC",
    },
];
/// Subchunks 5.
const SUBCHUNKS_5: &[&str] = &[
    "tlIntChannelChunk",
    "tlFloat1ChannelChunk",
    "tlFloat2ChannelChunk",
    "tlVector1DOFChannelChunk",
    "tlVector2DOFChannelChunk",
    "tlVector3DOFChannelChunk",
    "tlQuaternionChannelChunk",
    "tlCompressedQuaternionChannelChunk",
    "tlStringChannelChunk",
    "tlEntityChannelChunk",
    "tlBoolChannelChunk",
    "tlColourChannelChunk",
    "tlEventChannelChunk",
];
/// Fields 5.
const FIELDS_5: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "GroupId",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumChannels",
    },
];
/// Subchunks 6.
const SUBCHUNKS_6: &[&str] = &["tlAnimationGroupChunk"];
/// Fields 6.
const FIELDS_6: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumGroups",
    },
];
/// Subchunks 7.
const SUBCHUNKS_7: &[&str] = &[
    "tlCompositeDrawableChunk16",
    "tlFrameControllerChunk",
    "tlMultiControllerChunk16",
    "tlCollisionObjectChunk",
    "tlBillboardQuadGroupChunk",
];
/// Fields 7.
const FIELDS_7: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "ULONG",
        name: "HasAlpha",
    },
];
/// Subchunks 8.
const SUBCHUNKS_8: &[&str] = &[
    "tlCompositeDrawableChunk16",
    "tlFrameControllerChunk",
    "tlMultiControllerChunk16",
    "tlBillboardQuadGroupChunk",
];
/// Fields 8.
const FIELDS_8: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "ULONG",
        name: "HasAlpha",
    },
];
/// Subchunks 9.
const SUBCHUNKS_9: &[&str] = &[
    "animation",
    "tlCompositeDrawableChunk16",
    "tlFrameControllerChunk",
    "tlMultiControllerChunk16",
    "tlCollisionObjectChunk",
    "tlBillboardQuadGroupChunk",
    "mesh",
    "tlPhysicsObjectChunk",
    "tlObjectAttributeChunk",
    "tlSkeletonChunk16",
];
/// Fields 9.
const FIELDS_9: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "ULONG",
        name: "HasAlpha",
    },
];
/// Subchunks 10.
const SUBCHUNKS_10: &[&str] = &[
    "tlStatePropChunk",
    "animation",
    "tlCompositeDrawableChunk16",
    "tlFrameControllerChunk",
    "tlMultiControllerChunk16",
    "tlCollisionObjectChunk",
    "tlBillboardQuadGroupChunk",
    "mesh",
    "tlPhysicsObjectChunk",
    "tlObjectAttributeChunk",
    "tlSkeletonChunk16",
    "tlAnimatedObjectFactoryChunk",
    "tlAnimatedObjectChunk",
];
/// Fields 10.
const FIELDS_10: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "UBYTE",
        name: "Version",
    },
    SchemaField {
        ty: "UBYTE",
        name: "HasAlpha",
    },
];
/// Subchunks 11.
const SUBCHUNKS_11: &[&str] = &[];
/// Fields 11.
const FIELDS_11: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "NumRows",
    },
    SchemaField {
        ty: "array( tlAttributeRow, NumRows )",
        name: "Table",
    },
];
/// Subchunks 12.
const SUBCHUNKS_12: &[&str] = &[];
/// Fields 12.
const FIELDS_12: &[SchemaField] = &[
    SchemaField {
        ty: "tlBox",
        name: "Box",
    },
];
/// Subchunks 13.
const SUBCHUNKS_13: &[&str] = &[];
/// Fields 13.
const FIELDS_13: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "string",
        name: "Shader",
    },
    SchemaField {
        ty: "COLOUR",
        name: "Colour",
    },
    SchemaField {
        ty: "float",
        name: "Width",
    },
    SchemaField {
        ty: "float",
        name: "Height",
    },
    SchemaField {
        ty: "ULONG",
        name: "ZTest",
    },
    SchemaField {
        ty: "ULONG",
        name: "ZWrite",
    },
    SchemaField {
        ty: "ULONG",
        name: "Fog",
    },
    SchemaField {
        ty: "ULONG",
        name: "BillboardMode",
    },
];
/// Subchunks 14.
const SUBCHUNKS_14: &[&str] = &[];
/// Fields 14.
const FIELDS_14: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "tlQuat",
        name: "Rotation",
    },
    SchemaField {
        ty: "FOURCC",
        name: "CutOffMode",
    },
    SchemaField {
        ty: "tlUV",
        name: "UVOffsetRange",
    },
    SchemaField {
        ty: "float",
        name: "SourceRange",
    },
    SchemaField {
        ty: "float",
        name: "EdgeRange",
    },
];
/// Subchunks 15.
const SUBCHUNKS_15: &[&str] = &[];
/// Fields 15.
const FIELDS_15: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "ULONG",
        name: "Perspective",
    },
];
/// Subchunks 16.
const SUBCHUNKS_16: &[&str] = &[
    "tlBillboardDisplayInfoChunk",
    "tlBillboardPerspectiveInfoChunk",
];
/// Fields 16.
const FIELDS_16: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "FOURCC",
        name: "BillboardMode",
    },
    SchemaField {
        ty: "tlPoint",
        name: "Translation",
    },
    SchemaField {
        ty: "COLOUR",
        name: "Colour",
    },
    SchemaField {
        ty: "tlUV",
        name: "Uv0",
    },
    SchemaField {
        ty: "tlUV",
        name: "Uv1",
    },
    SchemaField {
        ty: "tlUV",
        name: "Uv2",
    },
    SchemaField {
        ty: "tlUV",
        name: "Uv3",
    },
    SchemaField {
        ty: "float",
        name: "Width",
    },
    SchemaField {
        ty: "float",
        name: "Height",
    },
    SchemaField {
        ty: "float",
        name: "Distance",
    },
    SchemaField {
        ty: "tlUV",
        name: "UVOffset",
    },
];
/// Subchunks 17.
const SUBCHUNKS_17: &[&str] = &["tlBillboardQuadChunk"];
/// Fields 17.
const FIELDS_17: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "string",
        name: "Shader",
    },
    SchemaField {
        ty: "ULONG",
        name: "ZTest",
    },
    SchemaField {
        ty: "ULONG",
        name: "ZWrite",
    },
    SchemaField {
        ty: "ULONG",
        name: "Fog",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumQuads",
    },
];
/// Subchunks 18.
const SUBCHUNKS_18: &[&str] = &[
    "tlAnimatedObjectFactoryChunk",
    "tlAnimatedObjectChunk",
    "tlFrameControllerChunk",
    "mesh",
    "animation",
    "tlMultiControllerChunk16",
    "tlSkeletonChunk16",
    "particle_system_factory",
    "particle_system",
    "tlCompositeDrawableChunk16",
];
/// Fields 18.
const FIELDS_18: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "BreakableType",
    },
    SchemaField {
        ty: "ULONG",
        name: "MaxInstances",
    },
];
/// Subchunks 19.
const SUBCHUNKS_19: &[&str] = &[];
/// Fields 19.
const FIELDS_19: &[SchemaField] = &[
    SchemaField {
        ty: "tlSphere",
        name: "Sphere",
    },
];
/// Subchunks 20.
const SUBCHUNKS_20: &[&str] = &["tlCameraAnimChannelChunk16"];
/// Fields 20.
const FIELDS_20: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "AnimName",
    },
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "string",
        name: "CameraName",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumFrames",
    },
    SchemaField {
        ty: "float",
        name: "FrameRate",
    },
    SchemaField {
        ty: "ULONG",
        name: "Cyclic",
    },
];
/// Subchunks 21.
const SUBCHUNKS_21: &[&str] = &[
    "tlCameraAnimPosChannelChunk16",
    "tlCameraAnimLookChannelChunk16",
    "tlCameraAnimUpChannelChunk16",
    "tlCameraAnimFOVChannelChunk16",
    "tlCameraAnimNearClipChannelChunk16",
    "tlCameraAnimFarClipChannelChunk16",
];
/// Fields 21.
const FIELDS_21: &[SchemaField] = &[];
/// Subchunks 22.
const SUBCHUNKS_22: &[&str] = &["tlChannel3DOFChunk16"];
/// Fields 22.
const FIELDS_22: &[SchemaField] = &[];
/// Subchunks 23.
const SUBCHUNKS_23: &[&str] = &["tlChannel3DOFChunk16"];
/// Fields 23.
const FIELDS_23: &[SchemaField] = &[];
/// Subchunks 24.
const SUBCHUNKS_24: &[&str] = &["tlChannel3DOFChunk16"];
/// Fields 24.
const FIELDS_24: &[SchemaField] = &[];
/// Subchunks 25.
const SUBCHUNKS_25: &[&str] = &["tlChannel1DOFChunk16"];
/// Fields 25.
const FIELDS_25: &[SchemaField] = &[];
/// Subchunks 26.
const SUBCHUNKS_26: &[&str] = &["tlChannel1DOFChunk16"];
/// Fields 26.
const FIELDS_26: &[SchemaField] = &[];
/// Subchunks 27.
const SUBCHUNKS_27: &[&str] = &["tlChannel1DOFChunk16"];
/// Fields 27.
const FIELDS_27: &[SchemaField] = &[];
/// Subchunks 28.
const SUBCHUNKS_28: &[&str] = &[];
/// Fields 28.
const FIELDS_28: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "float",
        name: "FOV",
    },
    SchemaField {
        ty: "float",
        name: "AspectRatio",
    },
    SchemaField {
        ty: "float",
        name: "NearClip",
    },
    SchemaField {
        ty: "float",
        name: "FarClip",
    },
    SchemaField {
        ty: "float",
        name: "PositionX",
    },
    SchemaField {
        ty: "float",
        name: "PositionY",
    },
    SchemaField {
        ty: "float",
        name: "PositionZ",
    },
    SchemaField {
        ty: "float",
        name: "LookX",
    },
    SchemaField {
        ty: "float",
        name: "LookY",
    },
    SchemaField {
        ty: "float",
        name: "LookZ",
    },
    SchemaField {
        ty: "float",
        name: "UpX",
    },
    SchemaField {
        ty: "float",
        name: "UpY",
    },
    SchemaField {
        ty: "float",
        name: "UpZ",
    },
];
/// Subchunks 29.
const SUBCHUNKS_29: &[&str] = &[];
/// Fields 29.
const FIELDS_29: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "ULONG",
        name: "Interpolate",
    },
];
/// Subchunks 30.
const SUBCHUNKS_30: &[&str] = &["tlChannelInterpolationModeChunk"];
/// Fields 30.
const FIELDS_30: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "FOURCC",
        name: "Param",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumFrames",
    },
    SchemaField {
        ty: "array( UWORD, NumFrames )",
        name: "Frames",
    },
    SchemaField {
        ty: "array( ULONG, NumFrames )",
        name: "Values",
    },
];
/// Subchunks 31.
const SUBCHUNKS_31: &[&str] = &["tlChannelInterpolationModeChunk"];
/// Fields 31.
const FIELDS_31: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "FOURCC",
        name: "Param",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumFrames",
    },
    SchemaField {
        ty: "array( UWORD, NumFrames )",
        name: "Frames",
    },
    SchemaField {
        ty: "array( float, NumFrames )",
        name: "Values",
    },
];
/// Subchunks 32.
const SUBCHUNKS_32: &[&str] = &["tlChannelInterpolationModeChunk"];
/// Fields 32.
const FIELDS_32: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "FOURCC",
        name: "Param",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumFrames",
    },
    SchemaField {
        ty: "array( UWORD, NumFrames )",
        name: "Frames",
    },
    SchemaField {
        ty: "array( tlPoint2D, NumFrames )",
        name: "Values",
    },
];
/// Subchunks 33.
const SUBCHUNKS_33: &[&str] = &["tlChannelInterpolationModeChunk"];
/// Fields 33.
const FIELDS_33: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "FOURCC",
        name: "Param",
    },
    SchemaField {
        ty: "UWORD",
        name: "Mapping",
    },
    SchemaField {
        ty: "tlPoint",
        name: "Constants",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumFrames",
    },
    SchemaField {
        ty: "array( UWORD, NumFrames )",
        name: "Frames",
    },
    SchemaField {
        ty: "array( float, NumFrames )",
        name: "Values",
    },
];
/// Subchunks 34.
const SUBCHUNKS_34: &[&str] = &["tlChannelInterpolationModeChunk"];
/// Fields 34.
const FIELDS_34: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "FOURCC",
        name: "Param",
    },
    SchemaField {
        ty: "UWORD",
        name: "Mapping",
    },
    SchemaField {
        ty: "tlPoint",
        name: "Constants",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumFrames",
    },
    SchemaField {
        ty: "array( UWORD, NumFrames )",
        name: "Frames",
    },
    SchemaField {
        ty: "array( tlPoint2D, NumFrames )",
        name: "Values",
    },
];
/// Subchunks 35.
const SUBCHUNKS_35: &[&str] = &["tlChannelInterpolationModeChunk"];
/// Fields 35.
const FIELDS_35: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "FOURCC",
        name: "Param",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumFrames",
    },
    SchemaField {
        ty: "array( UWORD, NumFrames )",
        name: "Frames",
    },
    SchemaField {
        ty: "array( tlPoint, NumFrames )",
        name: "Values",
    },
];
/// Subchunks 36.
const SUBCHUNKS_36: &[&str] = &[];
/// Fields 36.
const FIELDS_36: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "FOURCC",
        name: "Format",
    },
];
/// Subchunks 37.
const SUBCHUNKS_37: &[&str] = &[
    "tlQuaternionFormatChunk",
    "tlChannelInterpolationModeChunk",
];
/// Fields 37.
const FIELDS_37: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "FOURCC",
        name: "Param",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumFrames",
    },
    SchemaField {
        ty: "array( UWORD, NumFrames )",
        name: "Frames",
    },
    SchemaField {
        ty: "array( tlQuat, NumFrames )",
        name: "Values",
    },
];
/// Subchunks 38.
const SUBCHUNKS_38: &[&str] = &["tlChannelInterpolationModeChunk"];
/// Fields 38.
const FIELDS_38: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "FOURCC",
        name: "Param",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumFrames",
    },
    SchemaField {
        ty: "array( UWORD, NumFrames )",
        name: "Frames",
    },
    SchemaField {
        ty: "array( tlCompressedQuat, NumFrames )",
        name: "Values",
    },
];
/// Subchunks 39.
const SUBCHUNKS_39: &[&str] = &[];
/// Fields 39.
const FIELDS_39: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "FOURCC",
        name: "Param",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumFrames",
    },
    SchemaField {
        ty: "array( UWORD, NumFrames )",
        name: "Frames",
    },
    SchemaField {
        ty: "array( string, NumFrames )",
        name: "Values",
    },
];
/// Subchunks 40.
const SUBCHUNKS_40: &[&str] = &[];
/// Fields 40.
const FIELDS_40: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "FOURCC",
        name: "Param",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumFrames",
    },
    SchemaField {
        ty: "array( UWORD, NumFrames )",
        name: "Frames",
    },
    SchemaField {
        ty: "array( string, NumFrames )",
        name: "Values",
    },
];
/// Subchunks 41.
const SUBCHUNKS_41: &[&str] = &[];
/// Fields 41.
const FIELDS_41: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "FOURCC",
        name: "Param",
    },
    SchemaField {
        ty: "WORD",
        name: "StartState",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumFrames",
    },
    SchemaField {
        ty: "array( UWORD, NumFrames )",
        name: "Values",
    },
];
/// Subchunks 42.
const SUBCHUNKS_42: &[&str] = &["tlChannelInterpolationModeChunk"];
/// Fields 42.
const FIELDS_42: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "FOURCC",
        name: "Param",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumFrames",
    },
    SchemaField {
        ty: "array( UWORD, NumFrames )",
        name: "Frames",
    },
    SchemaField {
        ty: "array( COLOUR, NumFrames )",
        name: "Values",
    },
];
/// Subchunks 43.
const SUBCHUNKS_43: &[&str] = &[];
/// Fields 43.
const FIELDS_43: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "DataFormat",
    },
    SchemaField {
        ty: "ULONG",
        name: "DataLen",
    },
    SchemaField {
        ty: "array( UBYTE, DataLen )",
        name: "Data",
    },
];
/// Subchunks 44.
const SUBCHUNKS_44: &[&str] = &["tlEventDataImageChunk"];
/// Fields 44.
const FIELDS_44: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "Param",
    },
    SchemaField {
        ty: "UWORD",
        name: "Time",
    },
];
/// Subchunks 45.
const SUBCHUNKS_45: &[&str] = &["tlEventChunk"];
/// Fields 45.
const FIELDS_45: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "FOURCC",
        name: "Param",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumFrames",
    },
];
/// Subchunks 46.
const SUBCHUNKS_46: &[&str] = &[
    "tlCollisionVolumeOwnerChunk",
    "tlSelfCollisionChunk",
    "tlCollisionVolumeChunk",
    "tlCollisionObjectAttributeChunk",
];
/// Fields 46.
const FIELDS_46: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "string",
        name: "StringData",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumSubObject",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumOwner",
    },
];
/// Subchunks 47.
const SUBCHUNKS_47: &[&str] = &[];
/// Fields 47.
const FIELDS_47: &[SchemaField] = &[
    SchemaField {
        ty: "UWORD",
        name: "StaticAttribute",
    },
    SchemaField {
        ty: "ULONG",
        name: "DefaultArea",
    },
    SchemaField {
        ty: "UWORD",
        name: "CanRoll",
    },
    SchemaField {
        ty: "UWORD",
        name: "CanSlide",
    },
    SchemaField {
        ty: "UWORD",
        name: "CanSpin",
    },
    SchemaField {
        ty: "UWORD",
        name: "CanBounce",
    },
    SchemaField {
        ty: "ULONG",
        name: "ExtraAttribute1",
    },
    SchemaField {
        ty: "ULONG",
        name: "ExtraAttribute2",
    },
    SchemaField {
        ty: "ULONG",
        name: "ExtraAttribute3",
    },
];
/// Subchunks 48.
const SUBCHUNKS_48: &[&str] = &["tlCollisionVolumeOwnerNameChunk"];
/// Fields 48.
const FIELDS_48: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "NumNames",
    },
];
/// Subchunks 49.
const SUBCHUNKS_49: &[&str] = &[];
/// Fields 49.
const FIELDS_49: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
];
/// Subchunks 50.
const SUBCHUNKS_50: &[&str] = &[];
/// Fields 50.
const FIELDS_50: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "JointIndex1",
    },
    SchemaField {
        ty: "ULONG",
        name: "JointIndex2",
    },
    SchemaField {
        ty: "UWORD",
        name: "SelfOnly1",
    },
    SchemaField {
        ty: "UWORD",
        name: "SelfOnly2",
    },
];
/// Subchunks 51.
const SUBCHUNKS_51: &[&str] = &[
    "tlBBoxVolumeChunk",
    "tlSphereVolumeChunk",
    "tlCylinderVolumeChunk",
    "tlOBBoxVolumeChunk",
    "tlWallVolumeChunk",
    "tlCollisionVolumeChunk",
];
/// Fields 51.
const FIELDS_51: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "ObjectReferenceIndex",
    },
    SchemaField {
        ty: "ULONG",
        name: "OwnerIndex",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumSubVolume",
    },
];
/// Subchunks 52.
const SUBCHUNKS_52: &[&str] = &[];
/// Fields 52.
const FIELDS_52: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "Nothing",
    },
];
/// Subchunks 53.
const SUBCHUNKS_53: &[&str] = &["tlCollisionVectorChunk"];
/// Fields 53.
const FIELDS_53: &[SchemaField] = &[
    SchemaField {
        ty: "float",
        name: "SphereRadius",
    },
];
/// Subchunks 54.
const SUBCHUNKS_54: &[&str] = &["tlCollisionVectorChunk"];
/// Fields 54.
const FIELDS_54: &[SchemaField] = &[
    SchemaField {
        ty: "float",
        name: "CylinderRadius",
    },
    SchemaField {
        ty: "float",
        name: "Length",
    },
    SchemaField {
        ty: "UWORD",
        name: "FlatEnd",
    },
];
/// Subchunks 55.
const SUBCHUNKS_55: &[&str] = &["tlCollisionVectorChunk"];
/// Fields 55.
const FIELDS_55: &[SchemaField] = &[
    SchemaField {
        ty: "float",
        name: "Length1",
    },
    SchemaField {
        ty: "float",
        name: "Length2",
    },
    SchemaField {
        ty: "float",
        name: "Length3",
    },
];
/// Subchunks 56.
const SUBCHUNKS_56: &[&str] = &["tlCollisionVectorChunk"];
/// Fields 56.
const FIELDS_56: &[SchemaField] = &[];
/// Subchunks 57.
const SUBCHUNKS_57: &[&str] = &[];
/// Fields 57.
const FIELDS_57: &[SchemaField] = &[
    SchemaField {
        ty: "float",
        name: "X",
    },
    SchemaField {
        ty: "float",
        name: "Y",
    },
    SchemaField {
        ty: "float",
        name: "Z",
    },
];
/// Subchunks 58.
const SUBCHUNKS_58: &[&str] = &[
    "tlCompositeDrawableSkinListChunk16",
    "tlCompositeDrawablePropListChunk16",
    "tlCompositeDrawableEffectListChunk16",
];
/// Fields 58.
const FIELDS_58: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "string",
        name: "SkeletonName",
    },
];
/// Subchunks 59.
const SUBCHUNKS_59: &[&str] = &["tlCompositeDrawableSkinChunk16"];
/// Fields 59.
const FIELDS_59: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "NumElements",
    },
];
/// Subchunks 60.
const SUBCHUNKS_60: &[&str] = &["tlCompositeDrawablePropChunk16"];
/// Fields 60.
const FIELDS_60: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "NumElements",
    },
];
/// Subchunks 61.
const SUBCHUNKS_61: &[&str] = &["tlCompositeDrawableSortOrderChunk16"];
/// Fields 61.
const FIELDS_61: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "IsTranslucent",
    },
];
/// Subchunks 62.
const SUBCHUNKS_62: &[&str] = &["tlCompositeDrawableSortOrderChunk16"];
/// Fields 62.
const FIELDS_62: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "IsTranslucent",
    },
    SchemaField {
        ty: "ULONG",
        name: "SkeletonJointID",
    },
];
/// Subchunks 63.
const SUBCHUNKS_63: &[&str] = &["tlCompositeDrawableEffectChunk16"];
/// Fields 63.
const FIELDS_63: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "NumElements",
    },
];
/// Subchunks 64.
const SUBCHUNKS_64: &[&str] = &["tlCompositeDrawableSortOrderChunk16"];
/// Fields 64.
const FIELDS_64: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "IsTranslucent",
    },
    SchemaField {
        ty: "ULONG",
        name: "SkeletonJointID",
    },
];
/// Subchunks 65.
const SUBCHUNKS_65: &[&str] = &[];
/// Fields 65.
const FIELDS_65: &[SchemaField] = &[
    SchemaField {
        ty: "float",
        name: "SortOrder",
    },
];
/// Subchunks 66.
const SUBCHUNKS_66: &[&str] = &[
    "tlCompoundMeshNodeChunk16",
    "tlCompositeSkinPropList16",
];
/// Fields 66.
const FIELDS_66: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "string",
        name: "SkeletonName",
    },
];
/// Subchunks 67.
const SUBCHUNKS_67: &[&str] = &[];
/// Fields 67.
const FIELDS_67: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "NumElements",
    },
];
/// Subchunks 68.
const SUBCHUNKS_68: &[&str] = &[];
/// Fields 68.
const FIELDS_68: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "NumElements",
    },
];
/// Subchunks 69.
const SUBCHUNKS_69: &[&str] = &[];
/// Fields 69.
const FIELDS_69: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
];
/// Subchunks 70.
const SUBCHUNKS_70: &[&str] = &[];
/// Fields 70.
const FIELDS_70: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "SkeletonJointID",
    },
];
/// Subchunks 71.
const SUBCHUNKS_71: &[&str] = &[
    "mesh",
    "tlPhysicsObjectChunk",
    "tlObjectAttributeChunk",
    "tlCollisionObjectChunk",
    "tlInstancesChunk",
];
/// Fields 71.
const FIELDS_71: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "ULONG",
        name: "HasAlpha",
    },
];
/// Subchunks 72.
const SUBCHUNKS_72: &[&str] = &[];
/// Fields 72.
const FIELDS_72: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "NumFrames",
    },
    SchemaField {
        ty: "array( UWORD, NumFrames )",
        name: "Times",
    },
    SchemaField {
        ty: "array( string, NumFrames )",
        name: "Names",
    },
];
/// Subchunks 73.
const SUBCHUNKS_73: &[&str] = &["mesh"];
/// Fields 73.
const FIELDS_73: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "ULONG",
        name: "HasAlpha",
    },
];
/// Subchunks 74.
const SUBCHUNKS_74: &[&str] = &[];
/// Fields 74.
const FIELDS_74: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "Value",
    },
];
/// Subchunks 75.
const SUBCHUNKS_75: &[&str] = &[];
/// Fields 75.
const FIELDS_75: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "string",
        name: "Value",
    },
];
/// Subchunks 76.
const SUBCHUNKS_76: &[&str] = &[
    "tlExportInfoNamedIntChunk16",
    "tlExportInfoNamedStringChunk16",
];
/// Fields 76.
const FIELDS_76: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
];
/// Subchunks 77.
const SUBCHUNKS_77: &[&str] = &["tlVertexOffsetExpressionChunk16"];
/// Fields 77.
const FIELDS_77: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "string",
        name: "TargetName",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumExpression",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumPreset",
    },
    SchemaField {
        ty: "array( ULONG, NumExpression )",
        name: "Stages",
    },
];
/// Subchunks 78.
const SUBCHUNKS_78: &[&str] = &[];
/// Fields 78.
const FIELDS_78: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "string",
        name: "ExpressionName",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumKeys",
    },
    SchemaField {
        ty: "array( UWORD, NumKeys )",
        name: "TimeIndex",
    },
    SchemaField {
        ty: "array( float, NumKeys )",
        name: "StateKeys",
    },
    SchemaField {
        ty: "array( float, NumKeys )",
        name: "WeightKeys",
    },
];
/// Subchunks 79.
const SUBCHUNKS_79: &[&str] = &["tlExpressionAnimChannelChunk16"];
/// Fields 79.
const FIELDS_79: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumFrames",
    },
    SchemaField {
        ty: "float",
        name: "FrameRate",
    },
    SchemaField {
        ty: "ULONG",
        name: "Cyclic",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumChannel",
    },
];
/// Subchunks 80.
const SUBCHUNKS_80: &[&str] = &[];
/// Fields 80.
const FIELDS_80: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "ULONG",
        name: "Type",
    },
    SchemaField {
        ty: "string",
        name: "TargetName",
    },
    SchemaField {
        ty: "string",
        name: "ExpressionGroupName",
    },
];
/// Subchunks 81.
const SUBCHUNKS_81: &[&str] = &[];
/// Fields 81.
const FIELDS_81: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumKeys",
    },
    SchemaField {
        ty: "array( float, NumKeys )",
        name: "Keys",
    },
    SchemaField {
        ty: "array( ULONG, NumKeys )",
        name: "Indices",
    },
];
/// Subchunks 82.
const SUBCHUNKS_82: &[&str] = &["tlExpressionChunk"];
/// Fields 82.
const FIELDS_82: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "string",
        name: "TargetName",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumExpressions",
    },
    SchemaField {
        ty: "array( ULONG, NumExpressions )",
        name: "Stages",
    },
];
/// Subchunks 83.
const SUBCHUNKS_83: &[&str] = &[];
/// Fields 83.
const FIELDS_83: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "Type",
    },
    SchemaField {
        ty: "string",
        name: "TargetName",
    },
    SchemaField {
        ty: "string",
        name: "ExpressionGroupName",
    },
];
/// Subchunks 84.
const SUBCHUNKS_84: &[&str] = &[];
/// Fields 84.
const FIELDS_84: &[SchemaField] = &[
    SchemaField {
        ty: "tlMatrix",
        name: "Matrix",
    },
];
/// Subchunks 85.
const SUBCHUNKS_85: &[&str] = &["tlWallChunk"];
/// Fields 85.
const FIELDS_85: &[SchemaField] = &[];
/// Subchunks 86.
const SUBCHUNKS_86: &[&str] = &["tlWallChunk"];
/// Fields 86.
const FIELDS_86: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "NumWalls",
    },
];
/// Subchunks 87.
const SUBCHUNKS_87: &[&str] = &[
    "tlFlexibleJointParametersChunk",
    "tlFlexibleLambdaJointParamChunk",
    "tlFlexibleJointDefinitionChunk",
];
/// Fields 87.
const FIELDS_87: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
];
/// Subchunks 88.
const SUBCHUNKS_88: &[&str] = &[];
/// Fields 88.
const FIELDS_88: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "float",
        name: "LambdaF",
    },
    SchemaField {
        ty: "float",
        name: "LambdaD",
    },
    SchemaField {
        ty: "float",
        name: "KappaF",
    },
    SchemaField {
        ty: "float",
        name: "KappaD",
    },
    SchemaField {
        ty: "float",
        name: "Wind1DKf",
    },
];
/// Subchunks 89.
const SUBCHUNKS_89: &[&str] = &[];
/// Fields 89.
const FIELDS_89: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "float",
        name: "Stretch1Dkf",
    },
    SchemaField {
        ty: "float",
        name: "Stretch1Dkd",
    },
    SchemaField {
        ty: "float",
        name: "Bend1Dkf",
    },
    SchemaField {
        ty: "float",
        name: "Bend1Dkd",
    },
    SchemaField {
        ty: "float",
        name: "Wind1DKf",
    },
];
/// Subchunks 90.
const SUBCHUNKS_90: &[&str] = &[];
/// Fields 90.
const FIELDS_90: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "string",
        name: "ParametersName",
    },
    SchemaField {
        ty: "string",
        name: "RestingPosParametersName",
    },
    SchemaField {
        ty: "string",
        name: "ConnectionParametersName",
    },
    SchemaField {
        ty: "UWORD",
        name: "Gravity",
    },
    SchemaField {
        ty: "UWORD",
        name: "SimMethod",
    },
    SchemaField {
        ty: "UWORD",
        name: "UseRestingPos",
    },
    SchemaField {
        ty: "UWORD",
        name: "RestMethod",
    },
    SchemaField {
        ty: "UWORD",
        name: "UpdateMethod",
    },
    SchemaField {
        ty: "UWORD",
        name: "UseVirtualJoint",
    },
    SchemaField {
        ty: "UWORD",
        name: "Solver",
    },
    SchemaField {
        ty: "float",
        name: "ExternalDensityFactor",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumJointIndex",
    },
    SchemaField {
        ty: "array( ULONG, NumJointIndex )",
        name: "FlexibleJointIndex",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumFixJointIndex",
    },
    SchemaField {
        ty: "array( ULONG, NumFixJointIndex )",
        name: "FlexibleFixJointIndex",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumEndOfBranchIndex",
    },
    SchemaField {
        ty: "array( ULONG, NumEndOfBranchIndex )",
        name: "EndOfBranchIndex",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumConnection",
    },
    SchemaField {
        ty: "array( FlexibleJointConnectionData, NumConnection )",
        name: "FlexibleJointConnection",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumHinge",
    },
    SchemaField {
        ty: "array( FlexibleJointHingeData, NumHinge )",
        name: "FlexibleJointHinge",
    },
];
/// Subchunks 91.
const SUBCHUNKS_91: &[&str] = &[
    "tlFlexibleObjectFixParticleChunk",
    "tlFlexibleObjectMapToVLChunk",
    "tlFlexibleObjectTriMapChunk",
    "tlFlexibleObjectEdgeMapChunk",
    "tlFlexibleObjectEdgeLengthChunk",
    "tlFlexibleObjectParamChunk",
    "tlFlexibleLambdaObjectParamChunk",
];
/// Fields 91.
const FIELDS_91: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "UWORD",
        name: "Dimension",
    },
    SchemaField {
        ty: "float",
        name: "ExternalDensityFactor",
    },
    SchemaField {
        ty: "UWORD",
        name: "Solver",
    },
    SchemaField {
        ty: "UWORD",
        name: "NbParticle",
    },
    SchemaField {
        ty: "array( FlexibleParticlePosData, NbParticle )",
        name: "FlexibleParticlePos",
    },
    SchemaField {
        ty: "UWORD",
        name: "NbPGM",
    },
    SchemaField {
        ty: "array( UWORD, NbPGM )",
        name: "PGMNbVertex",
    },
];
/// Subchunks 92.
const SUBCHUNKS_92: &[&str] = &[];
/// Fields 92.
const FIELDS_92: &[SchemaField] = &[
    SchemaField {
        ty: "UWORD",
        name: "FixParticleCount",
    },
    SchemaField {
        ty: "array( UWORD, FixParticleCount )",
        name: "FixParticle",
    },
];
/// Subchunks 93.
const SUBCHUNKS_93: &[&str] = &[];
/// Fields 93.
const FIELDS_93: &[SchemaField] = &[
    SchemaField {
        ty: "UWORD",
        name: "VCount",
    },
    SchemaField {
        ty: "array( UWORD, VCount )",
        name: "MapToVL",
    },
];
/// Subchunks 94.
const SUBCHUNKS_94: &[&str] = &[];
/// Fields 94.
const FIELDS_94: &[SchemaField] = &[
    SchemaField {
        ty: "UWORD",
        name: "TriMapCount",
    },
    SchemaField {
        ty: "array( FlexibleObjectTriMapData, TriMapCount )",
        name: "TriMap",
    },
];
/// Subchunks 95.
const SUBCHUNKS_95: &[&str] = &[];
/// Fields 95.
const FIELDS_95: &[SchemaField] = &[
    SchemaField {
        ty: "UWORD",
        name: "EdgeMapCount",
    },
    SchemaField {
        ty: "array( FlexibleObjectEdgeMapData, EdgeMapCount )",
        name: "EdgeMap",
    },
];
/// Subchunks 96.
const SUBCHUNKS_96: &[&str] = &[];
/// Fields 96.
const FIELDS_96: &[SchemaField] = &[
    SchemaField {
        ty: "UWORD",
        name: "EdgeLengthCount",
    },
    SchemaField {
        ty: "array( float, EdgeLengthCount )",
        name: "EdgeLength",
    },
];
/// Subchunks 97.
const SUBCHUNKS_97: &[&str] = &[];
/// Fields 97.
const FIELDS_97: &[SchemaField] = &[
    SchemaField {
        ty: "float",
        name: "Stretch1Dkf",
    },
    SchemaField {
        ty: "float",
        name: "Stretch1Dkd",
    },
    SchemaField {
        ty: "float",
        name: "Bend1Dkf",
    },
    SchemaField {
        ty: "float",
        name: "Bend1Dkd",
    },
    SchemaField {
        ty: "float",
        name: "Wind1DKf",
    },
    SchemaField {
        ty: "float",
        name: "Stretch2Dkf",
    },
    SchemaField {
        ty: "float",
        name: "Stretch2Dkd",
    },
    SchemaField {
        ty: "float",
        name: "Shear2Dkf",
    },
    SchemaField {
        ty: "float",
        name: "Shear2Dkd",
    },
    SchemaField {
        ty: "float",
        name: "Bend2Dkf",
    },
    SchemaField {
        ty: "float",
        name: "Bend2Dkd",
    },
    SchemaField {
        ty: "float",
        name: "Wind2DKf",
    },
];
/// Subchunks 98.
const SUBCHUNKS_98: &[&str] = &[];
/// Fields 98.
const FIELDS_98: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "float",
        name: "LambdaF",
    },
    SchemaField {
        ty: "float",
        name: "LambdaD",
    },
    SchemaField {
        ty: "float",
        name: "KappaF",
    },
    SchemaField {
        ty: "float",
        name: "KappaD",
    },
    SchemaField {
        ty: "float",
        name: "IotaF",
    },
    SchemaField {
        ty: "float",
        name: "IotaD",
    },
    SchemaField {
        ty: "float",
        name: "Wind1DKf",
    },
    SchemaField {
        ty: "float",
        name: "Wind2DKf",
    },
];
/// Subchunks 99.
const SUBCHUNKS_99: &[&str] = &[];
/// Fields 99.
const FIELDS_99: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "ID",
    },
    SchemaField {
        ty: "float",
        name: "Rotation",
    },
    SchemaField {
        ty: "float",
        name: "Elevation",
    },
    SchemaField {
        ty: "float",
        name: "Magnitude",
    },
    SchemaField {
        ty: "tlPoint",
        name: "TargetOffset",
    },
];
/// Subchunks 100.
const SUBCHUNKS_100: &[&str] = &[
    "texture",
    "tlTextureGlyphListChunk",
];
/// Fields 100.
const FIELDS_100: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "string",
        name: "Shader",
    },
    SchemaField {
        ty: "float",
        name: "FontSize",
    },
    SchemaField {
        ty: "float",
        name: "FontWidth",
    },
    SchemaField {
        ty: "float",
        name: "FontHeight",
    },
    SchemaField {
        ty: "float",
        name: "FontBaseLine",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumTextures",
    },
];
/// Subchunks 101.
const SUBCHUNKS_101: &[&str] = &[];
/// Fields 101.
const FIELDS_101: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "NumGlyphs",
    },
    SchemaField {
        ty: "array( tlTextureGlyph, NumGlyphs )",
        name: "Glyphs",
    },
];
/// Subchunks 102.
const SUBCHUNKS_102: &[&str] = &[
    "image",
    "tlImageGlyphListChunk",
];
/// Fields 102.
const FIELDS_102: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "FontSize",
    },
    SchemaField {
        ty: "ULONG",
        name: "FontWidth",
    },
    SchemaField {
        ty: "ULONG",
        name: "FontHeight",
    },
    SchemaField {
        ty: "ULONG",
        name: "FontBaseLine",
    },
];
/// Subchunks 103.
const SUBCHUNKS_103: &[&str] = &[];
/// Fields 103.
const FIELDS_103: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "NumGlyphs",
    },
    SchemaField {
        ty: "array( tlImageGlyph, NumGlyphs )",
        name: "Glyphs",
    },
];
/// Subchunks 104.
const SUBCHUNKS_104: &[&str] = &[];
/// Fields 104.
const FIELDS_104: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "FOURCC",
        name: "Type",
    },
    SchemaField {
        ty: "float",
        name: "FrameOffset",
    },
    SchemaField {
        ty: "string",
        name: "HierarchyName",
    },
    SchemaField {
        ty: "string",
        name: "AnimationName",
    },
];
/// Subchunks 105.
const SUBCHUNKS_105: &[&str] = &[];
/// Fields 105.
const FIELDS_105: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "ULONG",
        name: "Type",
    },
    SchemaField {
        ty: "string",
        name: "HierarchyName",
    },
    SchemaField {
        ty: "string",
        name: "AnimationName",
    },
];
/// Subchunks 106.
const SUBCHUNKS_106: &[&str] = &[
    "tlGameAttrIntParamChunk",
    "tlGameAttrFloatParamChunk",
    "tlGameAttrColourParamChunk",
    "tlGameAttrVectorParamChunk",
    "tlGameAttrMatrixParamChunk",
];
/// Fields 106.
const FIELDS_106: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumParams",
    },
];
/// Subchunks 107.
const SUBCHUNKS_107: &[&str] = &[];
/// Fields 107.
const FIELDS_107: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "ParamName",
    },
    SchemaField {
        ty: "ULONG",
        name: "Value",
    },
];
/// Subchunks 108.
const SUBCHUNKS_108: &[&str] = &[];
/// Fields 108.
const FIELDS_108: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "ParamName",
    },
    SchemaField {
        ty: "float",
        name: "Value",
    },
];
/// Subchunks 109.
const SUBCHUNKS_109: &[&str] = &[];
/// Fields 109.
const FIELDS_109: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "ParamName",
    },
    SchemaField {
        ty: "COLOUR",
        name: "Value",
    },
];
/// Subchunks 110.
const SUBCHUNKS_110: &[&str] = &[];
/// Fields 110.
const FIELDS_110: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "ParamName",
    },
    SchemaField {
        ty: "tlPoint",
        name: "Value",
    },
];
/// Subchunks 111.
const SUBCHUNKS_111: &[&str] = &[];
/// Fields 111.
const FIELDS_111: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "ParamName",
    },
    SchemaField {
        ty: "tlMatrix",
        name: "Value",
    },
];
/// Subchunks 112.
const SUBCHUNKS_112: &[&str] = &[];
/// Fields 112.
const FIELDS_112: &[SchemaField] = &[
    SchemaField {
        ty: "UWORD",
        name: "NumLines",
    },
    SchemaField {
        ty: "array( string, NumLines )",
        name: "History",
    },
];
/// Subchunks 113.
const SUBCHUNKS_113: &[&str] = &[];
/// Fields 113.
const FIELDS_113: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "ImageDataSize",
    },
    SchemaField {
        ty: "array( UBYTE, ImageDataSize )",
        name: "ImageData",
    },
];
/// Subchunks 114.
const SUBCHUNKS_114: &[&str] = &[];
/// Fields 114.
const FIELDS_114: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "FileName",
    },
];
/// Subchunks 115.
const SUBCHUNKS_115: &[&str] = &[
    "tlImageDataChunk",
    "tlImageFileNameChunk",
];
/// Fields 115.
const FIELDS_115: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "ULONG",
        name: "Width",
    },
    SchemaField {
        ty: "ULONG",
        name: "Height",
    },
    SchemaField {
        ty: "ULONG",
        name: "Bpp",
    },
    SchemaField {
        ty: "ULONG",
        name: "Palettized",
    },
    SchemaField {
        ty: "ULONG",
        name: "HasAlpha",
    },
    SchemaField {
        ty: "ULONG",
        name: "Format",
    },
];
/// Subchunks 116.
const SUBCHUNKS_116: &[&str] = &["image"];
/// Fields 116.
const FIELDS_116: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "ULONG",
        name: "Width",
    },
    SchemaField {
        ty: "ULONG",
        name: "Height",
    },
    SchemaField {
        ty: "ULONG",
        name: "Depth",
    },
    SchemaField {
        ty: "ULONG",
        name: "Bpp",
    },
    SchemaField {
        ty: "ULONG",
        name: "Palettized",
    },
    SchemaField {
        ty: "ULONG",
        name: "HasAlpha",
    },
    SchemaField {
        ty: "ULONG",
        name: "Format",
    },
];
/// Subchunks 117.
const SUBCHUNKS_117: &[&str] = &[
    "mesh",
    "tlInstancesChunk",
];
/// Fields 117.
const FIELDS_117: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "ULONG",
        name: "HasAlpha",
    },
];
/// Subchunks 118.
const SUBCHUNKS_118: &[&str] = &["scenegraph"];
/// Fields 118.
const FIELDS_118: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
];
/// Subchunks 119.
const SUBCHUNKS_119: &[&str] = &[
    "tlAnimDSGWrapperChunk",
    "tlAnimObjDSGWrapperChunk",
    "tlObjectAttributeChunk",
    "tlInstancesChunk",
];
/// Fields 119.
const FIELDS_119: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "ULONG",
        name: "HasAlpha",
    },
];
/// Subchunks 120.
const SUBCHUNKS_120: &[&str] = &[
    "mesh",
    "tlObjectAttributeChunk",
    "tlCollisionObjectChunk",
    "tlInstancesChunk",
];
/// Fields 120.
const FIELDS_120: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "ULONG",
        name: "HasAlpha",
    },
];
/// Subchunks 121.
const SUBCHUNKS_121: &[&str] = &[
    "particle_system_factory",
    "tlFrameControllerChunk",
    "shader",
    "animation",
    "texture",
];
/// Fields 121.
const FIELDS_121: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "ParticleType",
    },
    SchemaField {
        ty: "ULONG",
        name: "MaxInstances",
    },
];
/// Subchunks 122.
const SUBCHUNKS_122: &[&str] = &[
    "tlBBoxChunk",
    "tlBSphereChunk",
    "tlTerrainTypeChunk",
];
/// Fields 122.
const FIELDS_122: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "NumIndices",
    },
    SchemaField {
        ty: "array( ULONG, NumIndices )",
        name: "Indices",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumPositions",
    },
    SchemaField {
        ty: "array( tlPoint, NumPositions )",
        name: "Positions",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumNormals",
    },
    SchemaField {
        ty: "array( tlPoint, NumNormals )",
        name: "Normals",
    },
];
/// Subchunks 123.
const SUBCHUNKS_123: &[&str] = &[];
/// Fields 123.
const FIELDS_123: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "tlPoint",
        name: "Centre",
    },
    SchemaField {
        ty: "float",
        name: "Radius",
    },
    SchemaField {
        ty: "ULONG",
        name: "Type",
    },
];
/// Subchunks 124.
const SUBCHUNKS_124: &[&str] = &[
    "tlCompositeDrawableChunk16",
    "tlBillboardQuadGroupChunk",
    "mesh",
];
/// Fields 124.
const FIELDS_124: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumBillBoardQuads",
    },
];
/// Subchunks 125.
const SUBCHUNKS_125: &[&str] = &["tlLightAnimChannelChunk16"];
/// Fields 125.
const FIELDS_125: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "AnimName",
    },
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumFrames",
    },
    SchemaField {
        ty: "float",
        name: "FrameRate",
    },
    SchemaField {
        ty: "ULONG",
        name: "Cyclic",
    },
];
/// Subchunks 126.
const SUBCHUNKS_126: &[&str] = &[
    "tlLightAnimColourChannelChunk16",
    "tlLightAnimParamChannelChunk16",
    "tlLightAnimEnableChannelChunk16",
];
/// Fields 126.
const FIELDS_126: &[SchemaField] = &[];
/// Subchunks 127.
const SUBCHUNKS_127: &[&str] = &["tlKeyListColourChunk16"];
/// Fields 127.
const FIELDS_127: &[SchemaField] = &[];
/// Subchunks 128.
const SUBCHUNKS_128: &[&str] = &["tlChannel3DOFChunk16"];
/// Fields 128.
const FIELDS_128: &[SchemaField] = &[];
/// Subchunks 129.
const SUBCHUNKS_129: &[&str] = &["tlChannel1DOFChunk16"];
/// Fields 129.
const FIELDS_129: &[SchemaField] = &[];
/// Subchunks 130.
const SUBCHUNKS_130: &[&str] = &[
    "tlLightDirectionChunk",
    "tlLightConeParamChunk",
    "tlLightPositionChunk",
    "tlLightShadowChunk",
    "tlLightDecayRangeChunk",
    "tlLightIlluminationTypeChunk",
];
/// Fields 130.
const FIELDS_130: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "ULONG",
        name: "Type",
    },
    SchemaField {
        ty: "COLOUR",
        name: "Colour",
    },
    SchemaField {
        ty: "float",
        name: "Constant",
    },
    SchemaField {
        ty: "float",
        name: "Linear",
    },
    SchemaField {
        ty: "float",
        name: "Squared",
    },
    SchemaField {
        ty: "ULONG",
        name: "Enabled",
    },
];
/// Subchunks 131.
const SUBCHUNKS_131: &[&str] = &[];
/// Fields 131.
const FIELDS_131: &[SchemaField] = &[
    SchemaField {
        ty: "tlPoint",
        name: "Direction",
    },
];
/// Subchunks 132.
const SUBCHUNKS_132: &[&str] = &[];
/// Fields 132.
const FIELDS_132: &[SchemaField] = &[
    SchemaField {
        ty: "tlPoint",
        name: "Position",
    },
];
/// Subchunks 133.
const SUBCHUNKS_133: &[&str] = &[];
/// Fields 133.
const FIELDS_133: &[SchemaField] = &[
    SchemaField {
        ty: "float",
        name: "Phi",
    },
    SchemaField {
        ty: "float",
        name: "Theta",
    },
    SchemaField {
        ty: "float",
        name: "Falloff",
    },
    SchemaField {
        ty: "float",
        name: "Range",
    },
];
/// Subchunks 134.
const SUBCHUNKS_134: &[&str] = &[];
/// Fields 134.
const FIELDS_134: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "Shadow",
    },
];
/// Subchunks 135.
const SUBCHUNKS_135: &[&str] = &[];
/// Fields 135.
const FIELDS_135: &[SchemaField] = &[
    SchemaField {
        ty: "float",
        name: "RotationY",
    },
];
/// Subchunks 136.
const SUBCHUNKS_136: &[&str] = &["tlLightDecayRangeRotationYChunk"];
/// Fields 136.
const FIELDS_136: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "DecayRangeType",
    },
    SchemaField {
        ty: "tlPoint",
        name: "Inner",
    },
    SchemaField {
        ty: "tlPoint",
        name: "Outer",
    },
];
/// Subchunks 137.
const SUBCHUNKS_137: &[&str] = &["tlLightIlluminationTypeChunk"];
/// Fields 137.
const FIELDS_137: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "IlluminationType",
    },
];
/// Subchunks 138.
const SUBCHUNKS_138: &[&str] = &[];
/// Fields 138.
const FIELDS_138: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumLights",
    },
    SchemaField {
        ty: "array( string, NumLights )",
        name: "Lights",
    },
];
/// Subchunks 139.
const SUBCHUNKS_139: &[&str] = &[];
/// Fields 139.
const FIELDS_139: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "tlPoint",
        name: "Position",
    },
];
/// Subchunks 140.
const SUBCHUNKS_140: &[&str] = &[];
/// Fields 140.
const FIELDS_140: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "BeginSection",
    },
    SchemaField {
        ty: "ULONG",
        name: "EndSection",
    },
];
/// Subchunks 141.
const SUBCHUNKS_141: &[&str] = &[
    "tlPrimGroupChunk",
    "tlBBoxChunk",
    "tlBSphereChunk",
    "tlRenderStatusChunk",
    "tlExpressionOffsetsChunk",
];
/// Fields 141.
const FIELDS_141: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumPrimGroups",
    },
];
/// Subchunks 142.
const SUBCHUNKS_142: &[&str] = &[];
/// Fields 142.
const FIELDS_142: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "CastShadow",
    },
];
/// Subchunks 143.
const SUBCHUNKS_143: &[&str] = &[
    "tlMultiControllerTracksChunk16",
    "tlMultiControllerTrackChunk16",
];
/// Fields 143.
const FIELDS_143: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "float",
        name: "Length",
    },
    SchemaField {
        ty: "float",
        name: "Framerate",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumTracks",
    },
];
/// Subchunks 144.
const SUBCHUNKS_144: &[&str] = &[];
/// Fields 144.
const FIELDS_144: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "NumTracks",
    },
    SchemaField {
        ty: "array( tlMultiControllerTrackData, NumTracks )",
        name: "Tracks",
    },
];
/// Subchunks 145.
const SUBCHUNKS_145: &[&str] = &[];
/// Fields 145.
const FIELDS_145: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "float",
        name: "StartTime",
    },
    SchemaField {
        ty: "float",
        name: "EndTime",
    },
    SchemaField {
        ty: "float",
        name: "Scale",
    },
];
/// Subchunks 146.
const SUBCHUNKS_146: &[&str] = &[];
/// Fields 146.
const FIELDS_146: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "ClassType",
    },
    SchemaField {
        ty: "ULONG",
        name: "PhyPropID",
    },
    SchemaField {
        ty: "string",
        name: "Sound",
    },
];
/// Subchunks 147.
const SUBCHUNKS_147: &[&str] = &[];
/// Fields 147.
const FIELDS_147: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "COLOUR",
        name: "Colour",
    },
    SchemaField {
        ty: "tlUV",
        name: "Uv0",
    },
    SchemaField {
        ty: "tlUV",
        name: "Uv1",
    },
    SchemaField {
        ty: "tlUV",
        name: "Uv2",
    },
    SchemaField {
        ty: "tlUV",
        name: "Uv3",
    },
    SchemaField {
        ty: "tlUV",
        name: "UvOffset",
    },
    SchemaField {
        ty: "float",
        name: "Distance",
    },
    SchemaField {
        ty: "float",
        name: "Width",
    },
    SchemaField {
        ty: "float",
        name: "Height",
    },
];
/// Subchunks 148.
const SUBCHUNKS_148: &[&str] = &["tlLensFlareChunk"];
/// Fields 148.
const FIELDS_148: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "string",
        name: "ShaderName",
    },
    SchemaField {
        ty: "ULONG",
        name: "ZTest",
    },
    SchemaField {
        ty: "ULONG",
        name: "ZWrite",
    },
    SchemaField {
        ty: "ULONG",
        name: "Fog",
    },
    SchemaField {
        ty: "float",
        name: "SourceRadius",
    },
    SchemaField {
        ty: "float",
        name: "EdgeRadius",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumFlares",
    },
];
/// Subchunks 149.
const SUBCHUNKS_149: &[&str] = &[];
/// Fields 149.
const FIELDS_149: &[SchemaField] = &[
    SchemaField {
        ty: "float",
        name: "X",
    },
    SchemaField {
        ty: "float",
        name: "Y",
    },
    SchemaField {
        ty: "float",
        name: "Z",
    },
];
/// Subchunks 150.
const SUBCHUNKS_150: &[&str] = &["tlOpticVectorV14Chunk"];
/// Fields 150.
const FIELDS_150: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "string",
        name: "Shader",
    },
    SchemaField {
        ty: "COLOUR",
        name: "Colour",
    },
    SchemaField {
        ty: "ULONG",
        name: "TextureFrameRate",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumTextureFrames",
    },
    SchemaField {
        ty: "float",
        name: "Width",
    },
    SchemaField {
        ty: "float",
        name: "Height",
    },
    SchemaField {
        ty: "float",
        name: "SourceRadius",
    },
    SchemaField {
        ty: "float",
        name: "FallOffTime",
    },
    SchemaField {
        ty: "float",
        name: "DistanceCutOff",
    },
    SchemaField {
        ty: "ULONG",
        name: "PerspectiveScale",
    },
    SchemaField {
        ty: "float",
        name: "FrameRate",
    },
    SchemaField {
        ty: "ULONG",
        name: "BillboardMode",
    },
];
/// Subchunks 151.
const SUBCHUNKS_151: &[&str] = &[
    "tlLensFlareV14Chunk",
    "tlOpticVectorV14Chunk",
];
/// Fields 151.
const FIELDS_151: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumFlares",
    },
    SchemaField {
        ty: "float",
        name: "SourceRadius",
    },
    SchemaField {
        ty: "float",
        name: "FallOffTime",
    },
    SchemaField {
        ty: "float",
        name: "DistanceCutOff",
    },
    SchemaField {
        ty: "float",
        name: "FrameRate",
    },
];
/// Subchunks 152.
const SUBCHUNKS_152: &[&str] = &[];
/// Fields 152.
const FIELDS_152: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Shader",
    },
    SchemaField {
        ty: "COLOUR",
        name: "Colour",
    },
    SchemaField {
        ty: "float",
        name: "Distance",
    },
    SchemaField {
        ty: "float",
        name: "Width",
    },
    SchemaField {
        ty: "float",
        name: "Height",
    },
];
/// Subchunks 153.
const SUBCHUNKS_153: &[&str] = &[];
/// Fields 153.
const FIELDS_153: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "ULONG",
        name: "MaxInstances",
    },
];
/// Subchunks 154.
const SUBCHUNKS_154: &[&str] = &[
    "tlParticleInstancingInfoChunk",
    "tlBaseEmitterFactoryChunk",
    "tlSpriteEmitterFactoryChunk",
    "tlDrawableEmitterFactoryChunk",
];
/// Fields 154.
const FIELDS_154: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "float",
        name: "FrameRate",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumAnimFrames",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumOLFrames",
    },
    SchemaField {
        ty: "UWORD",
        name: "CycleAnim",
    },
    SchemaField {
        ty: "UWORD",
        name: "EnableSorting",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumEmitters",
    },
];
/// Subchunks 155.
const SUBCHUNKS_155: &[&str] = &[];
/// Fields 155.
const FIELDS_155: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "string",
        name: "FactoryName",
    },
];
/// Subchunks 156.
const SUBCHUNKS_156: &[&str] = &["animation"];
/// Fields 156.
const FIELDS_156: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
];
/// Subchunks 157.
const SUBCHUNKS_157: &[&str] = &["animation"];
/// Fields 157.
const FIELDS_157: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
];
/// Subchunks 158.
const SUBCHUNKS_158: &[&str] = &["animation"];
/// Fields 158.
const FIELDS_158: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
];
/// Subchunks 159.
const SUBCHUNKS_159: &[&str] = &[
    "tlParticleAnimationChunk",
    "tlEmitterAnimationChunk",
    "tlGeneratorAnimationChunk",
];
/// Fields 159.
const FIELDS_159: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "FOURCC",
        name: "ParticleType",
    },
    SchemaField {
        ty: "FOURCC",
        name: "GeneratorType",
    },
    SchemaField {
        ty: "ULONG",
        name: "ZTest",
    },
    SchemaField {
        ty: "ULONG",
        name: "ZWrite",
    },
    SchemaField {
        ty: "ULONG",
        name: "Fog",
    },
    SchemaField {
        ty: "ULONG",
        name: "MaxParticles",
    },
    SchemaField {
        ty: "ULONG",
        name: "InfiniteLife",
    },
    SchemaField {
        ty: "float",
        name: "RotationalCohesion",
    },
    SchemaField {
        ty: "float",
        name: "TranslationalCohesion",
    },
];
/// Subchunks 160.
const SUBCHUNKS_160: &[&str] = &["tlBaseEmitterFactoryChunk"];
/// Fields 160.
const FIELDS_160: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "string",
        name: "ShaderName",
    },
    SchemaField {
        ty: "FOURCC",
        name: "AngleMode",
    },
    SchemaField {
        ty: "float",
        name: "Angle",
    },
    SchemaField {
        ty: "FOURCC",
        name: "TextureAnimMode",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumTextureFrames",
    },
    SchemaField {
        ty: "ULONG",
        name: "TextureFrameRate",
    },
];
/// Subchunks 161.
const SUBCHUNKS_161: &[&str] = &["tlBaseEmitterFactoryChunk"];
/// Fields 161.
const FIELDS_161: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "string",
        name: "DrawableName",
    },
    SchemaField {
        ty: "FOURCC",
        name: "AngleMode",
    },
    SchemaField {
        ty: "float",
        name: "Angle",
    },
];
/// Subchunks 162.
const SUBCHUNKS_162: &[&str] = &[];
/// Fields 162.
const FIELDS_162: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "NumPoints",
    },
    SchemaField {
        ty: "array( tlPoint, NumPoints)",
        name: "Points",
    },
];
/// Subchunks 163.
const SUBCHUNKS_163: &[&str] = &[];
/// Fields 163.
const FIELDS_163: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumLights",
    },
    SchemaField {
        ty: "array( string, NumLights )",
        name: "Lights",
    },
    SchemaField {
        ty: "array( float, NumLights )",
        name: "LightScales",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumPhotons",
    },
    SchemaField {
        ty: "array( tlPhoton, NumPhotons )",
        name: "Photons",
    },
];
/// Subchunks 164.
const SUBCHUNKS_164: &[&str] = &[
    "tlPhysicsVectorChunk",
    "tlPhysicsInertiaMatrixChunk",
    "tlPhysicsJointChunk",
];
/// Fields 164.
const FIELDS_164: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "string",
        name: "StringData",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumJoints",
    },
    SchemaField {
        ty: "float",
        name: "Volume",
    },
    SchemaField {
        ty: "float",
        name: "RestingSensitivity",
    },
];
/// Subchunks 165.
const SUBCHUNKS_165: &[&str] = &[
    "tlPhysicsVectorChunk",
    "tlPhysicsInertiaMatrixChunk",
];
/// Fields 165.
const FIELDS_165: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "Index",
    },
    SchemaField {
        ty: "float",
        name: "Volume",
    },
    SchemaField {
        ty: "float",
        name: "Stiffness",
    },
    SchemaField {
        ty: "float",
        name: "MaxAngle",
    },
    SchemaField {
        ty: "float",
        name: "MinAngle",
    },
    SchemaField {
        ty: "ULONG",
        name: "DOF",
    },
];
/// Subchunks 166.
const SUBCHUNKS_166: &[&str] = &[];
/// Fields 166.
const FIELDS_166: &[SchemaField] = &[
    SchemaField {
        ty: "float",
        name: "X",
    },
    SchemaField {
        ty: "float",
        name: "Y",
    },
    SchemaField {
        ty: "float",
        name: "Z",
    },
];
/// Subchunks 167.
const SUBCHUNKS_167: &[&str] = &[];
/// Fields 167.
const FIELDS_167: &[SchemaField] = &[
    SchemaField {
        ty: "float",
        name: "XX",
    },
    SchemaField {
        ty: "float",
        name: "XY",
    },
    SchemaField {
        ty: "float",
        name: "XZ",
    },
    SchemaField {
        ty: "float",
        name: "YY",
    },
    SchemaField {
        ty: "float",
        name: "YZ",
    },
    SchemaField {
        ty: "float",
        name: "ZZ",
    },
];
/// Subchunks 168.
const SUBCHUNKS_168: &[&str] = &[
    "tlObjectAttributeChunk",
    "tlCollisionObjectChunk",
];
/// Fields 168.
const FIELDS_168: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
];
/// Subchunks 169.
const SUBCHUNKS_169: &[&str] = &[
    "tlPoseJointListChunk16",
    "tlPoseAnimMirroredChunk16",
];
/// Fields 169.
const FIELDS_169: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "string",
        name: "SkeletonName",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumFrames",
    },
    SchemaField {
        ty: "float",
        name: "FrameRate",
    },
    SchemaField {
        ty: "ULONG",
        name: "Cyclic",
    },
];
/// Subchunks 170.
const SUBCHUNKS_170: &[&str] = &["tlAnimChannelChunk16"];
/// Fields 170.
const FIELDS_170: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "ListType",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumJoints",
    },
];
/// Subchunks 171.
const SUBCHUNKS_171: &[&str] = &[];
/// Fields 171.
const FIELDS_171: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
];
/// Subchunks 172.
const SUBCHUNKS_172: &[&str] = &[
    "tlChannel1DOFChunk16",
    "tlChannel3DOFChunk16",
    "tlChannel1DOFAngleChunk16",
    "tlChannel3DOFAngleChunk16",
    "tlChannelQuaternionChunk16",
    "tlChannelStaticVectorChunk16",
    "tlChannelStaticAngleChunk16",
    "tlChannelStaticQuaternionChunk16",
];
/// Fields 172.
const FIELDS_172: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "ULONG",
        name: "PoseIndex",
    },
];
/// Subchunks 173.
const SUBCHUNKS_173: &[&str] = &[];
/// Fields 173.
const FIELDS_173: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "NumKeys",
    },
    SchemaField {
        ty: "ULONG",
        name: "Mapping",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumConstants",
    },
    SchemaField {
        ty: "array( float, NumConstants )",
        name: "Constants",
    },
    SchemaField {
        ty: "array( UWORD, NumKeys )",
        name: "TimeIndex",
    },
    SchemaField {
        ty: "array( float, NumKeys )",
        name: "Frames",
    },
];
/// Subchunks 174.
const SUBCHUNKS_174: &[&str] = &[];
/// Fields 174.
const FIELDS_174: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "NumKeys",
    },
    SchemaField {
        ty: "array( UWORD, NumKeys )",
        name: "TimeIndex",
    },
    SchemaField {
        ty: "array( tlTransKeyData, NumKeys )",
        name: "Frames",
    },
];
/// Subchunks 175.
const SUBCHUNKS_175: &[&str] = &[];
/// Fields 175.
const FIELDS_175: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "NumKeys",
    },
    SchemaField {
        ty: "ULONG",
        name: "Mapping",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumConstants",
    },
    SchemaField {
        ty: "array( float, NumConstants )",
        name: "Constants",
    },
    SchemaField {
        ty: "array( UWORD, NumKeys )",
        name: "TimeIndex",
    },
    SchemaField {
        ty: "array( UWORD, NumKeys )",
        name: "Frames",
    },
];
/// Subchunks 176.
const SUBCHUNKS_176: &[&str] = &[];
/// Fields 176.
const FIELDS_176: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "NumKeys",
    },
    SchemaField {
        ty: "array( UWORD, NumKeys )",
        name: "TimeIndex",
    },
    SchemaField {
        ty: "array( ULONG, NumKeys )",
        name: "Frames",
    },
];
/// Subchunks 177.
const SUBCHUNKS_177: &[&str] = &[];
/// Fields 177.
const FIELDS_177: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "NumKeys",
    },
    SchemaField {
        ty: "array( UWORD, NumKeys )",
        name: "TimeIndex",
    },
    SchemaField {
        ty: "array( tlQuatKeyData, NumKeys )",
        name: "Frames",
    },
];
/// Subchunks 178.
const SUBCHUNKS_178: &[&str] = &[];
/// Fields 178.
const FIELDS_178: &[SchemaField] = &[
    SchemaField {
        ty: "float",
        name: "X",
    },
    SchemaField {
        ty: "float",
        name: "Y",
    },
    SchemaField {
        ty: "float",
        name: "Z",
    },
];
/// Subchunks 179.
const SUBCHUNKS_179: &[&str] = &[];
/// Fields 179.
const FIELDS_179: &[SchemaField] = &[
    SchemaField {
        ty: "float",
        name: "X",
    },
    SchemaField {
        ty: "float",
        name: "Y",
    },
    SchemaField {
        ty: "float",
        name: "Z",
    },
];
/// Subchunks 180.
const SUBCHUNKS_180: &[&str] = &[];
/// Fields 180.
const FIELDS_180: &[SchemaField] = &[
    SchemaField {
        ty: "float",
        name: "W",
    },
    SchemaField {
        ty: "float",
        name: "X",
    },
    SchemaField {
        ty: "float",
        name: "Y",
    },
    SchemaField {
        ty: "float",
        name: "Z",
    },
];
/// Subchunks 181.
const SUBCHUNKS_181: &[&str] = &[];
/// Fields 181.
const FIELDS_181: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "NumKeys",
    },
    SchemaField {
        ty: "array( UWORD, NumKeys )",
        name: "Frames",
    },
    SchemaField {
        ty: "array( ULONG, NumKeys )",
        name: "Keys",
    },
];
/// Subchunks 182.
const SUBCHUNKS_182: &[&str] = &[
    "tlVertexShaderChunk",
    "tlPositionListChunk",
    "tlNormalListChunk",
    "tlPackedNormalListChunk",
    "tlUVListChunk",
    "tlColourListChunk",
    "tlMultiColourListChunk",
    "tlStripListChunk",
    "tlIndexListChunk",
    "tlMatrixListChunk",
    "tlWeightListChunk",
    "tlMatrixPaletteChunk",
    "tlInstanceInfoChunk",
    "tlPrimGroupMemoryImageVertexChunk",
    "tlPrimGroupMemoryImageIndexChunk",
    "tlPrimGroupMemoryImageVertexDescriptionChunk",
    "tlTangentListChunk",
    "tlBinormalListChunk",
    "tlOffsetListChunk",
];
/// Fields 182.
const FIELDS_182: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "string",
        name: "Shader",
    },
    SchemaField {
        ty: "ULONG",
        name: "PrimitiveType",
    },
    SchemaField {
        ty: "ULONG",
        name: "VertexType",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumVertices",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumIndices",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumMatrices",
    },
];
/// Subchunks 183.
const SUBCHUNKS_183: &[&str] = &[];
/// Fields 183.
const FIELDS_183: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "VertexShaderName",
    },
];
/// Subchunks 184.
const SUBCHUNKS_184: &[&str] = &[];
/// Fields 184.
const FIELDS_184: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "NumPositions",
    },
    SchemaField {
        ty: "array( tlPoint, NumPositions )",
        name: "Positions",
    },
];
/// Subchunks 185.
const SUBCHUNKS_185: &[&str] = &[];
/// Fields 185.
const FIELDS_185: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "NumNormals",
    },
    SchemaField {
        ty: "array( tlPoint, NumNormals )",
        name: "Normals",
    },
];
/// Subchunks 186.
const SUBCHUNKS_186: &[&str] = &[];
/// Fields 186.
const FIELDS_186: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "NumTangents",
    },
    SchemaField {
        ty: "array( tlPoint, NumTangents )",
        name: "Tangents",
    },
];
/// Subchunks 187.
const SUBCHUNKS_187: &[&str] = &[];
/// Fields 187.
const FIELDS_187: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "NumBinormals",
    },
    SchemaField {
        ty: "array( tlPoint, NumBinormals )",
        name: "Binormals",
    },
];
/// Subchunks 188.
const SUBCHUNKS_188: &[&str] = &[];
/// Fields 188.
const FIELDS_188: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "NumNormals",
    },
    SchemaField {
        ty: "array( UBYTE, NumNormals )",
        name: "Normals",
    },
];
/// Subchunks 189.
const SUBCHUNKS_189: &[&str] = &[];
/// Fields 189.
const FIELDS_189: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "NumUVs",
    },
    SchemaField {
        ty: "ULONG",
        name: "Channel",
    },
    SchemaField {
        ty: "array( tlUV, NumUVs )",
        name: "UVs",
    },
];
/// Subchunks 190.
const SUBCHUNKS_190: &[&str] = &[];
/// Fields 190.
const FIELDS_190: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "NumColours",
    },
    SchemaField {
        ty: "array( tlColour, NumColours )",
        name: "Colours",
    },
];
/// Subchunks 191.
const SUBCHUNKS_191: &[&str] = &[];
/// Fields 191.
const FIELDS_191: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "NumColours",
    },
    SchemaField {
        ty: "ULONG",
        name: "Channel",
    },
    SchemaField {
        ty: "array( tlColour, NumColours )",
        name: "Colours",
    },
];
/// Subchunks 192.
const SUBCHUNKS_192: &[&str] = &[];
/// Fields 192.
const FIELDS_192: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "NumStrips",
    },
    SchemaField {
        ty: "array( ULONG, NumStrips )",
        name: "Strips",
    },
];
/// Subchunks 193.
const SUBCHUNKS_193: &[&str] = &[];
/// Fields 193.
const FIELDS_193: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "NumIndices",
    },
    SchemaField {
        ty: "array( ULONG, NumIndices )",
        name: "Indices",
    },
];
/// Subchunks 194.
const SUBCHUNKS_194: &[&str] = &["tlOffsetListChunk"];
/// Fields 194.
const FIELDS_194: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "NumPrimGroups",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumOffsetLists",
    },
    SchemaField {
        ty: "array( ULONG, NumPrimGroups )",
        name: "PrimGroupIndices",
    },
];
/// Subchunks 195.
const SUBCHUNKS_195: &[&str] = &[];
/// Fields 195.
const FIELDS_195: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "NumOffsets",
    },
    SchemaField {
        ty: "ULONG",
        name: "KeyIndex",
    },
    SchemaField {
        ty: "array( tlVtxOffset, NumOffsets )",
        name: "Offsets",
    },
    SchemaField {
        ty: "ULONG",
        name: "PrimGroupIndex",
    },
];
/// Subchunks 196.
const SUBCHUNKS_196: &[&str] = &[];
/// Fields 196.
const FIELDS_196: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "NumMatrices",
    },
    SchemaField {
        ty: "array( COLOUR, NumMatrices )",
        name: "Matrices",
    },
];
/// Subchunks 197.
const SUBCHUNKS_197: &[&str] = &[];
/// Fields 197.
const FIELDS_197: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "NumWeights",
    },
    SchemaField {
        ty: "array( tlPoint, NumWeights )",
        name: "Weights",
    },
];
/// Subchunks 198.
const SUBCHUNKS_198: &[&str] = &[];
/// Fields 198.
const FIELDS_198: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "NumMatrices",
    },
    SchemaField {
        ty: "array( ULONG, NumMatrices )",
        name: "Matrices",
    },
];
/// Subchunks 199.
const SUBCHUNKS_199: &[&str] = &[];
/// Fields 199.
const FIELDS_199: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "NumInstances",
    },
    SchemaField {
        ty: "ULONG",
        name: "VertexCount",
    },
    SchemaField {
        ty: "ULONG",
        name: "IndexCount",
    },
];
/// Subchunks 200.
const SUBCHUNKS_200: &[&str] = &[];
/// Fields 200.
const FIELDS_200: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "ULONG",
        name: "Param",
    },
    SchemaField {
        ty: "ULONG",
        name: "MemoryImageVertexDescriptionSize",
    },
    SchemaField {
        ty: "array( UBYTE, MemoryImageVertexDescriptionSize )",
        name: "MemoryImageVertexDescription",
    },
];
/// Subchunks 201.
const SUBCHUNKS_201: &[&str] = &[];
/// Fields 201.
const FIELDS_201: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "ULONG",
        name: "Param",
    },
    SchemaField {
        ty: "ULONG",
        name: "MemoryImageVertexSize",
    },
    SchemaField {
        ty: "array( UBYTE, MemoryImageVertexSize )",
        name: "MemoryImageVertex",
    },
];
/// Subchunks 202.
const SUBCHUNKS_202: &[&str] = &[];
/// Fields 202.
const FIELDS_202: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "ULONG",
        name: "Param",
    },
    SchemaField {
        ty: "ULONG",
        name: "MemoryImageIndexSize",
    },
    SchemaField {
        ty: "array( UBYTE, MemoryImageIndexSize )",
        name: "MemoryImageIndex",
    },
];
/// Subchunks 203.
const SUBCHUNKS_203: &[&str] = &["tlRoadSegmentChunk"];
/// Fields 203.
const FIELDS_203: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "Type",
    },
    SchemaField {
        ty: "string",
        name: "StartIntersection",
    },
    SchemaField {
        ty: "string",
        name: "EndIntersection",
    },
    SchemaField {
        ty: "ULONG",
        name: "Density",
    },
    SchemaField {
        ty: "ULONG",
        name: "Speed",
    },
];
/// Subchunks 204.
const SUBCHUNKS_204: &[&str] = &[];
/// Fields 204.
const FIELDS_204: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "string",
        name: "RoadSegmentData",
    },
    SchemaField {
        ty: "tlMatrix",
        name: "HierarchyMatrix",
    },
    SchemaField {
        ty: "tlMatrix",
        name: "ScaleMatrix",
    },
];
/// Subchunks 205.
const SUBCHUNKS_205: &[&str] = &[];
/// Fields 205.
const FIELDS_205: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "Type",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumLanes",
    },
    SchemaField {
        ty: "ULONG",
        name: "HasShoulder",
    },
    SchemaField {
        ty: "tlPoint",
        name: "Direction",
    },
    SchemaField {
        ty: "tlPoint",
        name: "Top",
    },
    SchemaField {
        ty: "tlPoint",
        name: "Bottom",
    },
];
/// Subchunks 206.
const SUBCHUNKS_206: &[&str] = &["tlScenegraphRootChunk"];
/// Fields 206.
const FIELDS_206: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
];
/// Subchunks 207.
const SUBCHUNKS_207: &[&str] = &[
    "tlScenegraphBranchChunk",
    "tlScenegraphTransformChunk",
    "tlScenegraphDrawableChunk",
    "tlScenegraphAttachmentChunk",
    "tlScenegraphCameraChunk",
    "tlScenegraphLightGroupChunk",
    "tlScenegraphVisibilityChunk",
];
/// Fields 207.
const FIELDS_207: &[SchemaField] = &[];
/// Subchunks 208.
const SUBCHUNKS_208: &[&str] = &[
    "tlScenegraphBranchChunk",
    "tlScenegraphTransformChunk",
    "tlScenegraphDrawableChunk",
    "tlScenegraphAttachmentChunk",
    "tlScenegraphCameraChunk",
    "tlScenegraphLightGroupChunk",
    "tlScenegraphVisibilityChunk",
];
/// Fields 208.
const FIELDS_208: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "NChildren",
    },
];
/// Subchunks 209.
const SUBCHUNKS_209: &[&str] = &[
    "tlScenegraphTransformChunk",
    "tlScenegraphDrawableChunk",
    "tlScenegraphAttachmentChunk",
    "tlScenegraphCameraChunk",
    "tlScenegraphLightGroupChunk",
    "tlScenegraphVisibilityChunk",
];
/// Fields 209.
const FIELDS_209: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "NChildren",
    },
    SchemaField {
        ty: "tlMatrix",
        name: "Transform",
    },
];
/// Subchunks 210.
const SUBCHUNKS_210: &[&str] = &[
    "tlScenegraphBranchChunk",
    "tlScenegraphTransformChunk",
    "tlScenegraphDrawableChunk",
    "tlScenegraphAttachmentChunk",
    "tlScenegraphCameraChunk",
    "tlScenegraphLightGroupChunk",
    "tlScenegraphVisibilityChunk",
];
/// Fields 210.
const FIELDS_210: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "NChildren",
    },
    SchemaField {
        ty: "ULONG",
        name: "IsVisible",
    },
];
/// Subchunks 211.
const SUBCHUNKS_211: &[&str] = &["tlScenegraphAttachmentPointChunk"];
/// Fields 211.
const FIELDS_211: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "string",
        name: "DrawablePoseName",
    },
    SchemaField {
        ty: "ULONG",
        name: "NPoints",
    },
];
/// Subchunks 212.
const SUBCHUNKS_212: &[&str] = &[
    "tlScenegraphBranchChunk",
    "tlScenegraphTransformChunk",
    "tlScenegraphDrawableChunk",
    "tlScenegraphAttachmentChunk",
    "tlScenegraphCameraChunk",
    "tlScenegraphLightGroupChunk",
    "tlScenegraphVisibilityChunk",
];
/// Fields 212.
const FIELDS_212: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "Joint",
    },
];
/// Subchunks 213.
const SUBCHUNKS_213: &[&str] = &["tlScenegraphSortOrderChunk"];
/// Fields 213.
const FIELDS_213: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "string",
        name: "DrawableName",
    },
    SchemaField {
        ty: "ULONG",
        name: "IsTranslucent",
    },
];
/// Subchunks 214.
const SUBCHUNKS_214: &[&str] = &[];
/// Fields 214.
const FIELDS_214: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "string",
        name: "CameraName",
    },
];
/// Subchunks 215.
const SUBCHUNKS_215: &[&str] = &[];
/// Fields 215.
const FIELDS_215: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "string",
        name: "LightGroupName",
    },
];
/// Subchunks 216.
const SUBCHUNKS_216: &[&str] = &[];
/// Fields 216.
const FIELDS_216: &[SchemaField] = &[
    SchemaField {
        ty: "float",
        name: "SortOrder",
    },
];
/// Subchunks 217.
const SUBCHUNKS_217: &[&str] = &[
    "tlChannel1DOFChunk16",
    "tlChannel3DOFChunk16",
    "tlChannel1DOFAngleChunk16",
    "tlChannel3DOFAngleChunk16",
    "tlChannelStaticVectorChunk16",
    "tlChannelStaticAngleChunk16",
];
/// Fields 217.
const FIELDS_217: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "string",
        name: "NodeName",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumFrames",
    },
    SchemaField {
        ty: "float",
        name: "FrameRate",
    },
    SchemaField {
        ty: "ULONG",
        name: "Cyclic",
    },
];
/// Subchunks 218.
const SUBCHUNKS_218: &[&str] = &[
    "tlScroobyPageChunk",
    "tlScroobyScreenChunk",
];
/// Fields 218.
const FIELDS_218: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "ULONG",
        name: "ResolutionX",
    },
    SchemaField {
        ty: "ULONG",
        name: "ResolutionY",
    },
    SchemaField {
        ty: "string",
        name: "Platform",
    },
    SchemaField {
        ty: "string",
        name: "PagePath",
    },
    SchemaField {
        ty: "string",
        name: "ResourcePath",
    },
    SchemaField {
        ty: "string",
        name: "ScreenPath",
    },
];
/// Subchunks 219.
const SUBCHUNKS_219: &[&str] = &[];
/// Fields 219.
const FIELDS_219: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumberOfPages",
    },
    SchemaField {
        ty: "array( string, NumberOfPages )",
        name: "PageNames",
    },
];
/// Subchunks 220.
const SUBCHUNKS_220: &[&str] = &[
    "tlScroobyLayerChunk",
    "tlScroobyImageResourceChunk",
    "tlScroobyPure3dResourceChunk",
    "tlScroobyTextStyleResourceChunk",
    "tlScroobyTextBibleResourceChunk",
];
/// Fields 220.
const FIELDS_220: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "ULONG",
        name: "ResolutionX",
    },
    SchemaField {
        ty: "ULONG",
        name: "ResolutionY",
    },
];
/// Subchunks 221.
const SUBCHUNKS_221: &[&str] = &[
    "tlScroobyGroupChunk",
    "tlScroobyMultiTextChunk",
    "tlScroobyMultiSpriteChunk",
    "tlScroobyPolygonChunk",
    "tlScroobyPure3dObjectChunk",
];
/// Fields 221.
const FIELDS_221: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "ULONG",
        name: "Visible",
    },
    SchemaField {
        ty: "ULONG",
        name: "Editable",
    },
    SchemaField {
        ty: "ULONG",
        name: "Alpha",
    },
];
/// Subchunks 222.
const SUBCHUNKS_222: &[&str] = &[];
/// Fields 222.
const FIELDS_222: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "BibleName",
    },
    SchemaField {
        ty: "string",
        name: "StringId",
    },
];
/// Subchunks 223.
const SUBCHUNKS_223: &[&str] = &[];
/// Fields 223.
const FIELDS_223: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "String",
    },
];
/// Subchunks 224.
const SUBCHUNKS_224: &[&str] = &[
    "tlScroobyStringTextBibleChunk",
    "tlScroobyStringHardCodedChunk",
];
/// Fields 224.
const FIELDS_224: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "ULONG",
        name: "PositionX",
    },
    SchemaField {
        ty: "ULONG",
        name: "PositionY",
    },
    SchemaField {
        ty: "ULONG",
        name: "DimensionX",
    },
    SchemaField {
        ty: "ULONG",
        name: "DimensionY",
    },
    SchemaField {
        ty: "ULONG",
        name: "JustificationX",
    },
    SchemaField {
        ty: "ULONG",
        name: "JustificationY",
    },
    SchemaField {
        ty: "COLOUR",
        name: "Color",
    },
    SchemaField {
        ty: "ULONG",
        name: "Translucency",
    },
    SchemaField {
        ty: "float",
        name: "RotationValue",
    },
    SchemaField {
        ty: "string",
        name: "TextStyleName",
    },
    SchemaField {
        ty: "UBYTE",
        name: "ShadowEnabled",
    },
    SchemaField {
        ty: "COLOUR",
        name: "ShadowColour",
    },
    SchemaField {
        ty: "ULONG",
        name: "ShadowOffsetX",
    },
    SchemaField {
        ty: "ULONG",
        name: "ShadowOffsetY",
    },
    SchemaField {
        ty: "ULONG",
        name: "CurrentText",
    },
];
/// Subchunks 225.
const SUBCHUNKS_225: &[&str] = &[];
/// Fields 225.
const FIELDS_225: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "ULONG",
        name: "PositionX",
    },
    SchemaField {
        ty: "ULONG",
        name: "PositionY",
    },
    SchemaField {
        ty: "ULONG",
        name: "DimensionX",
    },
    SchemaField {
        ty: "ULONG",
        name: "DimensionY",
    },
    SchemaField {
        ty: "ULONG",
        name: "JustificationX",
    },
    SchemaField {
        ty: "ULONG",
        name: "JustificationY",
    },
    SchemaField {
        ty: "COLOUR",
        name: "Color",
    },
    SchemaField {
        ty: "ULONG",
        name: "Translucency",
    },
    SchemaField {
        ty: "float",
        name: "RotationValue",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumberOfImages",
    },
    SchemaField {
        ty: "array( string, NumberOfImages )",
        name: "ImageNames",
    },
];
/// Subchunks 226.
const SUBCHUNKS_226: &[&str] = &[];
/// Fields 226.
const FIELDS_226: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "ULONG",
        name: "Translucency",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumPoints",
    },
    SchemaField {
        ty: "array( tlPoint, NumPoints )",
        name: "Points",
    },
    SchemaField {
        ty: "array( tlColour, NumPoints )",
        name: "Colors",
    },
];
/// Subchunks 227.
const SUBCHUNKS_227: &[&str] = &[
    "tlScroobyGroupChunk",
    "tlScroobyMultiSpriteChunk",
    "tlScroobyMultiTextChunk",
    "tlScroobyPolygonChunk",
    "tlScroobyPure3dObjectChunk",
];
/// Fields 227.
const FIELDS_227: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "ULONG",
        name: "Alpha",
    },
];
/// Subchunks 228.
const SUBCHUNKS_228: &[&str] = &[];
/// Fields 228.
const FIELDS_228: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "ULONG",
        name: "PositionX",
    },
    SchemaField {
        ty: "ULONG",
        name: "PositionY",
    },
    SchemaField {
        ty: "ULONG",
        name: "DimensionX",
    },
    SchemaField {
        ty: "ULONG",
        name: "DimensionY",
    },
    SchemaField {
        ty: "ULONG",
        name: "JustificationX",
    },
    SchemaField {
        ty: "ULONG",
        name: "JustificationY",
    },
    SchemaField {
        ty: "COLOUR",
        name: "Color",
    },
    SchemaField {
        ty: "ULONG",
        name: "Translucency",
    },
    SchemaField {
        ty: "float",
        name: "RotationValue",
    },
    SchemaField {
        ty: "string",
        name: "Pure3dFilename",
    },
];
/// Subchunks 229.
const SUBCHUNKS_229: &[&str] = &["tlScroobyLanguageChunk"];
/// Fields 229.
const FIELDS_229: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumLanguages",
    },
    SchemaField {
        ty: "string",
        name: "Languages",
    },
];
/// Subchunks 230.
const SUBCHUNKS_230: &[&str] = &[];
/// Fields 230.
const FIELDS_230: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "UBYTE",
        name: "Language",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumStrings",
    },
    SchemaField {
        ty: "ULONG",
        name: "Modulo",
    },
    SchemaField {
        ty: "ULONG",
        name: "SizeOfBuffer",
    },
    SchemaField {
        ty: "array( ULONG, NumStrings )",
        name: "Hashes",
    },
    SchemaField {
        ty: "array( ULONG, NumStrings )",
        name: "Offsets",
    },
    SchemaField {
        ty: "array( UBYTE, SizeOfBuffer )",
        name: "Buffer",
    },
];
/// Subchunks 231.
const SUBCHUNKS_231: &[&str] = &[];
/// Fields 231.
const FIELDS_231: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "string",
        name: "Filename",
    },
];
/// Subchunks 232.
const SUBCHUNKS_232: &[&str] = &[];
/// Fields 232.
const FIELDS_232: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "string",
        name: "Filename",
    },
    SchemaField {
        ty: "string",
        name: "InventoryName",
    },
    SchemaField {
        ty: "string",
        name: "CameraName",
    },
    SchemaField {
        ty: "string",
        name: "AnimationName",
    },
];
/// Subchunks 233.
const SUBCHUNKS_233: &[&str] = &[];
/// Fields 233.
const FIELDS_233: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "string",
        name: "Filename",
    },
    SchemaField {
        ty: "string",
        name: "InventoryName",
    },
];
/// Subchunks 234.
const SUBCHUNKS_234: &[&str] = &[];
/// Fields 234.
const FIELDS_234: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "string",
        name: "Filename",
    },
    SchemaField {
        ty: "string",
        name: "InventoryName",
    },
];
/// Subchunks 235.
const SUBCHUNKS_235: &[&str] = &[];
/// Fields 235.
const FIELDS_235: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "string",
        name: "Filename",
    },
];
/// Subchunks 236.
const SUBCHUNKS_236: &[&str] = &[];
/// Fields 236.
const FIELDS_236: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "string",
        name: "Filename",
    },
];
/// Subchunks 237.
const SUBCHUNKS_237: &[&str] = &["texture"];
/// Fields 237.
const FIELDS_237: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "UBYTE",
        name: "ChildCount",
    },
];
/// Subchunks 238.
const SUBCHUNKS_238: &[&str] = &[
    "tlShaderDefinitionChunk",
    "tlShaderTextureParamChunk",
    "tlShaderIntParamChunk",
    "tlShaderFloatParamChunk",
    "tlShaderColourParamChunk",
    "tlShaderVectorParamChunk",
    "tlShaderMatrixParamChunk",
];
/// Fields 238.
const FIELDS_238: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "string",
        name: "PddiShaderName",
    },
    SchemaField {
        ty: "ULONG",
        name: "HasTranslucency",
    },
    SchemaField {
        ty: "ULONG",
        name: "VertexNeeds",
    },
    SchemaField {
        ty: "ULONG",
        name: "VertexMask",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumParams",
    },
];
/// Subchunks 239.
const SUBCHUNKS_239: &[&str] = &[];
/// Fields 239.
const FIELDS_239: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "longstring",
        name: "Definition",
    },
];
/// Subchunks 240.
const SUBCHUNKS_240: &[&str] = &[];
/// Fields 240.
const FIELDS_240: &[SchemaField] = &[
    SchemaField {
        ty: "FOURCC",
        name: "Param",
    },
    SchemaField {
        ty: "string",
        name: "Value",
    },
];
/// Subchunks 241.
const SUBCHUNKS_241: &[&str] = &[];
/// Fields 241.
const FIELDS_241: &[SchemaField] = &[
    SchemaField {
        ty: "FOURCC",
        name: "Param",
    },
    SchemaField {
        ty: "ULONG",
        name: "Value",
    },
];
/// Subchunks 242.
const SUBCHUNKS_242: &[&str] = &[];
/// Fields 242.
const FIELDS_242: &[SchemaField] = &[
    SchemaField {
        ty: "FOURCC",
        name: "Param",
    },
    SchemaField {
        ty: "float",
        name: "Value",
    },
];
/// Subchunks 243.
const SUBCHUNKS_243: &[&str] = &[];
/// Fields 243.
const FIELDS_243: &[SchemaField] = &[
    SchemaField {
        ty: "FOURCC",
        name: "Param",
    },
    SchemaField {
        ty: "COLOUR",
        name: "Value",
    },
];
/// Subchunks 244.
const SUBCHUNKS_244: &[&str] = &[];
/// Fields 244.
const FIELDS_244: &[SchemaField] = &[
    SchemaField {
        ty: "FOURCC",
        name: "Param",
    },
    SchemaField {
        ty: "tlPoint",
        name: "Value",
    },
];
/// Subchunks 245.
const SUBCHUNKS_245: &[&str] = &[];
/// Fields 245.
const FIELDS_245: &[SchemaField] = &[
    SchemaField {
        ty: "FOURCC",
        name: "Param",
    },
    SchemaField {
        ty: "tlMatrix",
        name: "Value",
    },
];
/// Subchunks 246.
const SUBCHUNKS_246: &[&str] = &[
    "tlPositionListChunk",
    "tlWeightListChunk",
    "tlMatrixListChunk",
    "tlTopologyChunk",
    "tlBBoxChunk",
    "tlBSphereChunk",
];
/// Fields 246.
const FIELDS_246: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "string",
        name: "SkeletonName",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumVertices",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumTriangles",
    },
];
/// Subchunks 247.
const SUBCHUNKS_247: &[&str] = &[];
/// Fields 247.
const FIELDS_247: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "NumTopology",
    },
    SchemaField {
        ty: "array( tlTopologyStruct, NumTopology )",
        name: "Topology",
    },
];
/// Subchunks 248.
const SUBCHUNKS_248: &[&str] = &[
    "tlPositionListChunk",
    "tlTopologyChunk",
    "tlBBoxChunk",
    "tlBSphereChunk",
];
/// Fields 248.
const FIELDS_248: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumVertices",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumTriangles",
    },
];
/// Subchunks 249.
const SUBCHUNKS_249: &[&str] = &["tlSkeletonJointChunk16"];
/// Fields 249.
const FIELDS_249: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumJoints",
    },
];
/// Subchunks 250.
const SUBCHUNKS_250: &[&str] = &[
    "tlSkeletonJointBonePreserveChunk16",
    "tlSkeletonJointMirrorMapChunk16",
];
/// Fields 250.
const FIELDS_250: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "Parent",
    },
    SchemaField {
        ty: "ULONG",
        name: "DOF",
    },
    SchemaField {
        ty: "ULONG",
        name: "FreeAxes",
    },
    SchemaField {
        ty: "ULONG",
        name: "PrimaryAxis",
    },
    SchemaField {
        ty: "ULONG",
        name: "SecondaryAxis",
    },
    SchemaField {
        ty: "ULONG",
        name: "TwistAxis",
    },
    SchemaField {
        ty: "tlMatrix",
        name: "RestPose",
    },
];
/// Subchunks 251.
const SUBCHUNKS_251: &[&str] = &[];
/// Fields 251.
const FIELDS_251: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "MappedJointIndex",
    },
    SchemaField {
        ty: "float",
        name: "XAxisMap",
    },
    SchemaField {
        ty: "float",
        name: "YAxisMap",
    },
    SchemaField {
        ty: "float",
        name: "ZAxisMap",
    },
];
/// Subchunks 252.
const SUBCHUNKS_252: &[&str] = &[];
/// Fields 252.
const FIELDS_252: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "PreserveBoneLengths",
    },
];
/// Subchunks 253.
const SUBCHUNKS_253: &[&str] = &[
    "tlPrimGroupChunk",
    "tlBBoxChunk",
    "tlBSphereChunk",
    "tlExpressionOffsetsChunk",
];
/// Fields 253.
const FIELDS_253: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "string",
        name: "SkeletonName",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumPrimGroups",
    },
];
/// Subchunks 254.
const SUBCHUNKS_254: &[&str] = &["tlSmartPropStateDataChunk"];
/// Fields 254.
const FIELDS_254: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "string",
        name: "ObjectFactoryName",
    },
    SchemaField {
        ty: "string",
        name: "Material",
    },
    SchemaField {
        ty: "ULONG",
        name: "MaterialEnum",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumBreakables",
    },
    SchemaField {
        ty: "ULONG",
        name: "RenderingCost",
    },
    SchemaField {
        ty: "ULONG",
        name: "SimulationCost",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumStates",
    },
];
/// Subchunks 255.
const SUBCHUNKS_255: &[&str] = &[
    "tlSmartPropVisibilitiesDataChunk",
    "tlSmartPropFrameControllerDataChunk",
    "tlSmartPropEventDataChunk",
    "tlSmartPropCallbackDataChunk",
    "tlSmartPropAppliedForceDataChunk",
    "tlSmartPropBreakableDataChunk",
    "tlSmartPropExtraAttributeDataChunk",
];
/// Fields 255.
const FIELDS_255: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "AutoTransition",
    },
    SchemaField {
        ty: "float",
        name: "OutFrame",
    },
    SchemaField {
        ty: "ULONG",
        name: "OutState",
    },
    SchemaField {
        ty: "ULONG",
        name: "CanSimulate",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumDrawables",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumFrameControllers",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumEvents",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumCallbacks",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumAppliedForces",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumExtraAttributes",
    },
];
/// Subchunks 256.
const SUBCHUNKS_256: &[&str] = &[];
/// Fields 256.
const FIELDS_256: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "Visible",
    },
];
/// Subchunks 257.
const SUBCHUNKS_257: &[&str] = &[];
/// Fields 257.
const FIELDS_257: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "Cyclic",
    },
    SchemaField {
        ty: "float",
        name: "MinFrame",
    },
    SchemaField {
        ty: "float",
        name: "MaxFrame",
    },
    SchemaField {
        ty: "float",
        name: "RelativeSpeed",
    },
];
/// Subchunks 258.
const SUBCHUNKS_258: &[&str] = &[];
/// Fields 258.
const FIELDS_258: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "State",
    },
    SchemaField {
        ty: "ULONG",
        name: "EventEnum",
    },
];
/// Subchunks 259.
const SUBCHUNKS_259: &[&str] = &[];
/// Fields 259.
const FIELDS_259: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "float",
        name: "OnFrame",
    },
    SchemaField {
        ty: "ULONG",
        name: "EventEnum",
    },
];
/// Subchunks 260.
const SUBCHUNKS_260: &[&str] = &[];
/// Fields 260.
const FIELDS_260: &[SchemaField] = &[
    SchemaField {
        ty: "float",
        name: "OnFrame",
    },
    SchemaField {
        ty: "float",
        name: "forceX",
    },
    SchemaField {
        ty: "float",
        name: "forceY",
    },
    SchemaField {
        ty: "float",
        name: "forceZ",
    },
    SchemaField {
        ty: "float",
        name: "posX",
    },
    SchemaField {
        ty: "float",
        name: "posY",
    },
    SchemaField {
        ty: "float",
        name: "posZ",
    },
];
/// Subchunks 261.
const SUBCHUNKS_261: &[&str] = &[];
/// Fields 261.
const FIELDS_261: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "IsBreakable",
    },
    SchemaField {
        ty: "ULONG",
        name: "Joint",
    },
    SchemaField {
        ty: "float",
        name: "OnFrame",
    },
    SchemaField {
        ty: "float",
        name: "forceX",
    },
    SchemaField {
        ty: "float",
        name: "forceY",
    },
    SchemaField {
        ty: "float",
        name: "forceZ",
    },
    SchemaField {
        ty: "float",
        name: "posX",
    },
    SchemaField {
        ty: "float",
        name: "posY",
    },
    SchemaField {
        ty: "float",
        name: "posZ",
    },
    SchemaField {
        ty: "string",
        name: "Material",
    },
    SchemaField {
        ty: "ULONG",
        name: "MaterialEnum",
    },
];
/// Subchunks 262.
const SUBCHUNKS_262: &[&str] = &[];
/// Fields 262.
const FIELDS_262: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "ExtraAttribute",
    },
];
/// Subchunks 263.
const SUBCHUNKS_263: &[&str] = &["image"];
/// Fields 263.
const FIELDS_263: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "NativeX",
    },
    SchemaField {
        ty: "ULONG",
        name: "NativeY",
    },
    SchemaField {
        ty: "string",
        name: "Shader",
    },
    SchemaField {
        ty: "ULONG",
        name: "ImageWidth",
    },
    SchemaField {
        ty: "ULONG",
        name: "ImageHeight",
    },
    SchemaField {
        ty: "ULONG",
        name: "ImageCount",
    },
    SchemaField {
        ty: "ULONG",
        name: "BlitBorder",
    },
];
/// Subchunks 264.
const SUBCHUNKS_264: &[&str] = &["tlStatePropStateDataChunk"];
/// Fields 264.
const FIELDS_264: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "string",
        name: "ObjectFactoryName",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumStates",
    },
];
/// Subchunks 265.
const SUBCHUNKS_265: &[&str] = &[
    "tlStatePropVisibilitiesDataChunk",
    "tlStatePropFrameControllerDataChunk",
    "tlStatePropEventDataChunk",
    "tlStatePropCallbackDataChunk",
];
/// Fields 265.
const FIELDS_265: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "AutoTransition",
    },
    SchemaField {
        ty: "ULONG",
        name: "OutState",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumDrawables",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumFrameControllers",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumEvents",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumCallbacks",
    },
    SchemaField {
        ty: "float",
        name: "OutFrame",
    },
];
/// Subchunks 266.
const SUBCHUNKS_266: &[&str] = &[];
/// Fields 266.
const FIELDS_266: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "Visible",
    },
];
/// Subchunks 267.
const SUBCHUNKS_267: &[&str] = &[];
/// Fields 267.
const FIELDS_267: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "Cyclic",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumberOfCycles",
    },
    SchemaField {
        ty: "ULONG",
        name: "HoldFrame",
    },
    SchemaField {
        ty: "float",
        name: "MinFrame",
    },
    SchemaField {
        ty: "float",
        name: "MaxFrame",
    },
    SchemaField {
        ty: "float",
        name: "RelativeSpeed",
    },
];
/// Subchunks 268.
const SUBCHUNKS_268: &[&str] = &[];
/// Fields 268.
const FIELDS_268: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "State",
    },
    SchemaField {
        ty: "ULONG",
        name: "EventEnum",
    },
];
/// Subchunks 269.
const SUBCHUNKS_269: &[&str] = &[];
/// Fields 269.
const FIELDS_269: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "EventEnum",
    },
    SchemaField {
        ty: "float",
        name: "OnFrame",
    },
];
/// Subchunks 270.
const SUBCHUNKS_270: &[&str] = &[
    "tlObjectAttributeChunk",
    "tlCollisionObjectChunk",
];
/// Fields 270.
const FIELDS_270: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
];
/// Subchunks 271.
const SUBCHUNKS_271: &[&str] = &[];
/// Fields 271.
const FIELDS_271: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumTypes",
    },
    SchemaField {
        ty: "array( UBYTE, NumTypes )",
        name: "Types",
    },
];
/// Subchunks 272.
const SUBCHUNKS_272: &[&str] = &["tlTextureAnimChannelChunk16"];
/// Fields 272.
const FIELDS_272: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "AnimName",
    },
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "string",
        name: "MaterialName",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumFrames",
    },
    SchemaField {
        ty: "float",
        name: "FrameRate",
    },
    SchemaField {
        ty: "ULONG",
        name: "Cyclic",
    },
];
/// Subchunks 273.
const SUBCHUNKS_273: &[&str] = &["tlEntityChannelChunk16"];
/// Fields 273.
const FIELDS_273: &[SchemaField] = &[];
/// Subchunks 274.
const SUBCHUNKS_274: &[&str] = &[
    "image",
    "tlVolumeImageChunk",
];
/// Fields 274.
const FIELDS_274: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "ULONG",
        name: "Width",
    },
    SchemaField {
        ty: "ULONG",
        name: "Height",
    },
    SchemaField {
        ty: "ULONG",
        name: "Bpp",
    },
    SchemaField {
        ty: "ULONG",
        name: "AlphaDepth",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumMipMaps",
    },
    SchemaField {
        ty: "ULONG",
        name: "TextureType",
    },
    SchemaField {
        ty: "ULONG",
        name: "Usage",
    },
    SchemaField {
        ty: "ULONG",
        name: "Priority",
    },
];
/// Subchunks 275.
const SUBCHUNKS_275: &[&str] = &["tlContiguousBinNodeChunk"];
/// Fields 275.
const FIELDS_275: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "NumNodes",
    },
    SchemaField {
        ty: "tlPoint",
        name: "BoundsMin",
    },
    SchemaField {
        ty: "tlPoint",
        name: "BoundsMax",
    },
];
/// Subchunks 276.
const SUBCHUNKS_276: &[&str] = &["tlSpatialNodeChunk"];
/// Fields 276.
const FIELDS_276: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "SubTreeSize",
    },
    SchemaField {
        ty: "ULONG",
        name: "ParentOffset",
    },
];
/// Subchunks 277.
const SUBCHUNKS_277: &[&str] = &[];
/// Fields 277.
const FIELDS_277: &[SchemaField] = &[
    SchemaField {
        ty: "UBYTE",
        name: "PlaneAxis",
    },
    SchemaField {
        ty: "float",
        name: "PlanePosn",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumSEntities",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumSPhys",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumIntersects",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumDPhys",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumFences",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumRoads",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumPaths",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumAnims",
    },
];
/// Subchunks 278.
const SUBCHUNKS_278: &[&str] = &[
    "tlColourOffsetListChunk",
    "tlVectorOffsetListChunk",
    "tlVector2OffsetListChunk",
];
/// Fields 278.
const FIELDS_278: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "string",
        name: "Name",
    },
];
/// Subchunks 279.
const SUBCHUNKS_279: &[&str] = &["tlOffsetIndexListChunk"];
/// Fields 279.
const FIELDS_279: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumOffsets",
    },
    SchemaField {
        ty: "array( tlColourOffset, NumOffsets )",
        name: "Offsets",
    },
];
/// Subchunks 280.
const SUBCHUNKS_280: &[&str] = &["tlOffsetIndexListChunk"];
/// Fields 280.
const FIELDS_280: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumOffsets",
    },
    SchemaField {
        ty: "FOURCC",
        name: "Param",
    },
    SchemaField {
        ty: "array( tlPoint, NumOffsets )",
        name: "Offsets",
    },
];
/// Subchunks 281.
const SUBCHUNKS_281: &[&str] = &["tlOffsetIndexListChunk"];
/// Fields 281.
const FIELDS_281: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumOffsets",
    },
    SchemaField {
        ty: "FOURCC",
        name: "Param",
    },
    SchemaField {
        ty: "array( tlUV, NumOffsets )",
        name: "Offsets",
    },
];
/// Subchunks 282.
const SUBCHUNKS_282: &[&str] = &[];
/// Fields 282.
const FIELDS_282: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumIndex",
    },
    SchemaField {
        ty: "array( ULONG, NumIndex )",
        name: "Index",
    },
];
/// Subchunks 283.
const SUBCHUNKS_283: &[&str] = &[];
/// Fields 283.
const FIELDS_283: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumKeys",
    },
    SchemaField {
        ty: "array( float, NumKeys )",
        name: "Keys",
    },
    SchemaField {
        ty: "array( ULONG, NumKeys )",
        name: "Indices",
    },
];
/// Subchunks 284.
const SUBCHUNKS_284: &[&str] = &["tlVisibilityAnimChannelChunk16"];
/// Fields 284.
const FIELDS_284: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "AnimName",
    },
    SchemaField {
        ty: "string",
        name: "SceneName",
    },
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumFrames",
    },
    SchemaField {
        ty: "float",
        name: "FrameRate",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumChannels",
    },
];
/// Subchunks 285.
const SUBCHUNKS_285: &[&str] = &[];
/// Fields 285.
const FIELDS_285: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Nodename",
    },
    SchemaField {
        ty: "WORD",
        name: "bStartState",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumFrames",
    },
    SchemaField {
        ty: "array( ULONG, NumFrames )",
        name: "Frames",
    },
];
/// Subchunks 286.
const SUBCHUNKS_286: &[&str] = &[];
/// Fields 286.
const FIELDS_286: &[SchemaField] = &[
    SchemaField {
        ty: "ULONG",
        name: "ID",
    },
    SchemaField {
        ty: "float",
        name: "MinMagnitude",
    },
    SchemaField {
        ty: "float",
        name: "MaxMagnitude",
    },
    SchemaField {
        ty: "float",
        name: "Elevation",
    },
    SchemaField {
        ty: "tlPoint",
        name: "TargetOffset",
    },
];
/// Subchunks 287.
const SUBCHUNKS_287: &[&str] = &[];
/// Fields 287.
const FIELDS_287: &[SchemaField] = &[
    SchemaField {
        ty: "tlPoint",
        name: "Start",
    },
    SchemaField {
        ty: "tlPoint",
        name: "End",
    },
    SchemaField {
        ty: "tlPoint",
        name: "Normal",
    },
];
/// Subchunks 288.
const SUBCHUNKS_288: &[&str] = &[
    "tlWBTriggerVolumeChunk",
    "tlWBSplineChunk",
];
/// Fields 288.
const FIELDS_288: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "Type",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumDataElements",
    },
    SchemaField {
        ty: "array( ULONG, NumDataElements )",
        name: "DataElements",
    },
    SchemaField {
        ty: "tlPoint",
        name: "Position",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumTriggers",
    },
];
/// Subchunks 289.
const SUBCHUNKS_289: &[&str] = &[];
/// Fields 289.
const FIELDS_289: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "Behaviour",
    },
    SchemaField {
        ty: "float",
        name: "MinRadius",
    },
    SchemaField {
        ty: "float",
        name: "MaxRadius",
    },
    SchemaField {
        ty: "ULONG",
        name: "TrackRail",
    },
    SchemaField {
        ty: "float",
        name: "TrackDist",
    },
    SchemaField {
        ty: "ULONG",
        name: "ReverseSense",
    },
    SchemaField {
        ty: "float",
        name: "FOV",
    },
    SchemaField {
        ty: "tlPoint",
        name: "TargetOffset",
    },
    SchemaField {
        ty: "tlPoint",
        name: "AxisPlay",
    },
    SchemaField {
        ty: "float",
        name: "PositionLag",
    },
    SchemaField {
        ty: "float",
        name: "TargetLag",
    },
];
/// Subchunks 290.
const SUBCHUNKS_290: &[&str] = &["tlWBRailCamChunk"];
/// Fields 290.
const FIELDS_290: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumCVs",
    },
    SchemaField {
        ty: "array( tlPoint, NumCVs )",
        name: "CVs",
    },
];
/// Subchunks 291.
const SUBCHUNKS_291: &[&str] = &[];
/// Fields 291.
const FIELDS_291: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "Type",
    },
    SchemaField {
        ty: "tlPoint",
        name: "Scale",
    },
    SchemaField {
        ty: "tlMatrix",
        name: "Matrix",
    },
];
/// Subchunks 292.
const SUBCHUNKS_292: &[&str] = &[
    "mesh",
    "tlMultiControllerChunk16",
    "tlFrameControllerChunk16",
    "tlSkeletonChunk16",
    "animation",
    "tlCompositeDrawableChunk16",
    "tlBillboardQuadGroupChunk",
    "tlLensFlareDSGChunk",
];
/// Fields 292.
const FIELDS_292: &[SchemaField] = &[
    SchemaField {
        ty: "string",
        name: "Name",
    },
    SchemaField {
        ty: "ULONG",
        name: "Version",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumMeshes",
    },
    SchemaField {
        ty: "ULONG",
        name: "NumBillBoardQuads",
    },
];

/// Schema chunks.
pub const SCHEMA_CHUNKS: &[ChunkDefinition] = &[
    ChunkDefinition {
        schema_key: "schema_0001",
        name: "tlAnimatedObjectFactoryChunk",
        chunk_id_expr: "Pure3D::AnimatedObject::FACTORY",
        subchunks: SUBCHUNKS_0,
        fields: FIELDS_0,
    },
    ChunkDefinition {
        schema_key: "schema_0001",
        name: "tlAnimatedObjectChunk",
        chunk_id_expr: "Pure3D::AnimatedObject::OBJECT",
        subchunks: SUBCHUNKS_1,
        fields: FIELDS_1,
    },
    ChunkDefinition {
        schema_key: "schema_0001",
        name: "tlAnimatedObjectAnimationChunk",
        chunk_id_expr: "Pure3D::AnimatedObject::ANIMATION",
        subchunks: SUBCHUNKS_2,
        fields: FIELDS_2,
    },
    ChunkDefinition {
        schema_key: "schema_0002",
        name: "animation",
        chunk_id_expr: "Pure3D::Animation::AnimationData::ANIMATION",
        subchunks: SUBCHUNKS_3,
        fields: FIELDS_3,
    },
    ChunkDefinition {
        schema_key: "schema_0002",
        name: "tlAnimationSizeChunk",
        chunk_id_expr: "Pure3D::Animation::AnimationData::SIZE",
        subchunks: SUBCHUNKS_4,
        fields: FIELDS_4,
    },
    ChunkDefinition {
        schema_key: "schema_0002",
        name: "tlAnimationGroupChunk",
        chunk_id_expr: "Pure3D::Animation::AnimationData::GROUP",
        subchunks: SUBCHUNKS_5,
        fields: FIELDS_5,
    },
    ChunkDefinition {
        schema_key: "schema_0002",
        name: "tlAnimationGroupListChunk",
        chunk_id_expr: "Pure3D::Animation::AnimationData::GROUP_LIST",
        subchunks: SUBCHUNKS_6,
        fields: FIELDS_6,
    },
    ChunkDefinition {
        schema_key: "schema_0003",
        name: "anim_coll_dsg",
        chunk_id_expr: "SRR2::ChunkID::ANIM_COLL_DSG",
        subchunks: SUBCHUNKS_7,
        fields: FIELDS_7,
    },
    ChunkDefinition {
        schema_key: "schema_0004",
        name: "anim_dsg",
        chunk_id_expr: "SRR2::ChunkID::ANIM_DSG",
        subchunks: SUBCHUNKS_8,
        fields: FIELDS_8,
    },
    ChunkDefinition {
        schema_key: "schema_0005",
        name: "tlAnimDSGWrapperChunk",
        chunk_id_expr: "SRR2::ChunkID::ANIM_DSG_WRAPPER",
        subchunks: SUBCHUNKS_9,
        fields: FIELDS_9,
    },
    ChunkDefinition {
        schema_key: "schema_0006",
        name: "tlAnimObjDSGWrapperChunk",
        chunk_id_expr: "SRR2::ChunkID::ANIM_OBJ_DSG_WRAPPER",
        subchunks: SUBCHUNKS_10,
        fields: FIELDS_10,
    },
    ChunkDefinition {
        schema_key: "schema_0007",
        name: "tlAttributeTableChunk",
        chunk_id_expr: "SRR2::ChunkID::ATTRIBUTE_TABLE",
        subchunks: SUBCHUNKS_11,
        fields: FIELDS_11,
    },
    ChunkDefinition {
        schema_key: "schema_0008",
        name: "tlBBoxChunk",
        chunk_id_expr: "Pure3D::Mesh::BOX",
        subchunks: SUBCHUNKS_12,
        fields: FIELDS_12,
    },
    ChunkDefinition {
        schema_key: "schema_0009",
        name: "tlBillboardQuadV14Chunk",
        chunk_id_expr: "Pure3D::BillboardObject::QUAD_V14",
        subchunks: SUBCHUNKS_13,
        fields: FIELDS_13,
    },
    ChunkDefinition {
        schema_key: "schema_0009",
        name: "tlBillboardDisplayInfoChunk",
        chunk_id_expr: "Pure3D::BillboardObject::DISPLAY_INFO",
        subchunks: SUBCHUNKS_14,
        fields: FIELDS_14,
    },
    ChunkDefinition {
        schema_key: "schema_0009",
        name: "tlBillboardPerspectiveInfoChunk",
        chunk_id_expr: "Pure3D::BillboardObject::PERSPECTIVE_INFO",
        subchunks: SUBCHUNKS_15,
        fields: FIELDS_15,
    },
    ChunkDefinition {
        schema_key: "schema_0009",
        name: "tlBillboardQuadChunk",
        chunk_id_expr: "Pure3D::BillboardObject::QUAD",
        subchunks: SUBCHUNKS_16,
        fields: FIELDS_16,
    },
    ChunkDefinition {
        schema_key: "schema_0009",
        name: "tlBillboardQuadGroupChunk",
        chunk_id_expr: "Pure3D::BillboardObject::QUAD_GROUP",
        subchunks: SUBCHUNKS_17,
        fields: FIELDS_17,
    },
    ChunkDefinition {
        schema_key: "schema_0010",
        name: "tlBreakableObjectChunk",
        chunk_id_expr: "SRR2::ChunkID::BREAKABLE_OBJECT",
        subchunks: SUBCHUNKS_18,
        fields: FIELDS_18,
    },
    ChunkDefinition {
        schema_key: "schema_0011",
        name: "tlBSphereChunk",
        chunk_id_expr: "Pure3D::Mesh::SPHERE",
        subchunks: SUBCHUNKS_19,
        fields: FIELDS_19,
    },
    ChunkDefinition {
        schema_key: "schema_0012",
        name: "tlCameraAnimChunk16",
        chunk_id_expr: "P3D_CAMERA_ANIM",
        subchunks: SUBCHUNKS_20,
        fields: FIELDS_20,
    },
    ChunkDefinition {
        schema_key: "schema_0012",
        name: "tlCameraAnimChannelChunk16",
        chunk_id_expr: "P3D_CAMERA_ANIM_CHANNEL",
        subchunks: SUBCHUNKS_21,
        fields: FIELDS_21,
    },
    ChunkDefinition {
        schema_key: "schema_0012",
        name: "tlCameraAnimPosChannelChunk16",
        chunk_id_expr: "P3D_CAMERA_ANIM_POS_CHANNEL",
        subchunks: SUBCHUNKS_22,
        fields: FIELDS_22,
    },
    ChunkDefinition {
        schema_key: "schema_0012",
        name: "tlCameraAnimLookChannelChunk16",
        chunk_id_expr: "P3D_CAMERA_ANIM_LOOK_CHANNEL",
        subchunks: SUBCHUNKS_23,
        fields: FIELDS_23,
    },
    ChunkDefinition {
        schema_key: "schema_0012",
        name: "tlCameraAnimUpChannelChunk16",
        chunk_id_expr: "P3D_CAMERA_ANIM_UP_CHANNEL",
        subchunks: SUBCHUNKS_24,
        fields: FIELDS_24,
    },
    ChunkDefinition {
        schema_key: "schema_0012",
        name: "tlCameraAnimFOVChannelChunk16",
        chunk_id_expr: "P3D_CAMERA_ANIM_FOV_CHANNEL",
        subchunks: SUBCHUNKS_25,
        fields: FIELDS_25,
    },
    ChunkDefinition {
        schema_key: "schema_0012",
        name: "tlCameraAnimNearClipChannelChunk16",
        chunk_id_expr: "P3D_CAMERA_ANIM_NEARCLIP_CHANNEL",
        subchunks: SUBCHUNKS_26,
        fields: FIELDS_26,
    },
    ChunkDefinition {
        schema_key: "schema_0012",
        name: "tlCameraAnimFarClipChannelChunk16",
        chunk_id_expr: "P3D_CAMERA_ANIM_FARCLIP_CHANNEL",
        subchunks: SUBCHUNKS_27,
        fields: FIELDS_27,
    },
    ChunkDefinition {
        schema_key: "schema_0013",
        name: "tlCameraChunk16",
        chunk_id_expr: "P3D_CAMERA",
        subchunks: SUBCHUNKS_28,
        fields: FIELDS_28,
    },
    ChunkDefinition {
        schema_key: "schema_0014",
        name: "tlChannelInterpolationModeChunk",
        chunk_id_expr: "Pure3D::Animation::ChannelData::INTERPOLATION_MODE",
        subchunks: SUBCHUNKS_29,
        fields: FIELDS_29,
    },
    ChunkDefinition {
        schema_key: "schema_0014",
        name: "tlIntChannelChunk",
        chunk_id_expr: "Pure3D::Animation::ChannelData::INT",
        subchunks: SUBCHUNKS_30,
        fields: FIELDS_30,
    },
    ChunkDefinition {
        schema_key: "schema_0014",
        name: "tlFloat1ChannelChunk",
        chunk_id_expr: "Pure3D::Animation::ChannelData::FLOAT_1",
        subchunks: SUBCHUNKS_31,
        fields: FIELDS_31,
    },
    ChunkDefinition {
        schema_key: "schema_0014",
        name: "tlFloat2ChannelChunk",
        chunk_id_expr: "Pure3D::Animation::ChannelData::FLOAT_2",
        subchunks: SUBCHUNKS_32,
        fields: FIELDS_32,
    },
    ChunkDefinition {
        schema_key: "schema_0014",
        name: "tlVector1DOFChannelChunk",
        chunk_id_expr: "Pure3D::Animation::ChannelData::VECTOR_1DOF",
        subchunks: SUBCHUNKS_33,
        fields: FIELDS_33,
    },
    ChunkDefinition {
        schema_key: "schema_0014",
        name: "tlVector2DOFChannelChunk",
        chunk_id_expr: "Pure3D::Animation::ChannelData::VECTOR_2DOF",
        subchunks: SUBCHUNKS_34,
        fields: FIELDS_34,
    },
    ChunkDefinition {
        schema_key: "schema_0014",
        name: "tlVector3DOFChannelChunk",
        chunk_id_expr: "Pure3D::Animation::ChannelData::VECTOR_3DOF",
        subchunks: SUBCHUNKS_35,
        fields: FIELDS_35,
    },
    ChunkDefinition {
        schema_key: "schema_0014",
        name: "tlQuaternionFormatChunk",
        chunk_id_expr: "Pure3D::Animation::ChannelData::QUATERNION_FORMAT",
        subchunks: SUBCHUNKS_36,
        fields: FIELDS_36,
    },
    ChunkDefinition {
        schema_key: "schema_0014",
        name: "tlQuaternionChannelChunk",
        chunk_id_expr: "Pure3D::Animation::ChannelData::QUATERNION",
        subchunks: SUBCHUNKS_37,
        fields: FIELDS_37,
    },
    ChunkDefinition {
        schema_key: "schema_0014",
        name: "tlCompressedQuaternionChannelChunk",
        chunk_id_expr: "Pure3D::Animation::ChannelData::COMPRESSED_QUATERNION",
        subchunks: SUBCHUNKS_38,
        fields: FIELDS_38,
    },
    ChunkDefinition {
        schema_key: "schema_0014",
        name: "tlStringChannelChunk",
        chunk_id_expr: "Pure3D::Animation::ChannelData::STRING",
        subchunks: SUBCHUNKS_39,
        fields: FIELDS_39,
    },
    ChunkDefinition {
        schema_key: "schema_0014",
        name: "tlEntityChannelChunk",
        chunk_id_expr: "Pure3D::Animation::ChannelData::ENTITY",
        subchunks: SUBCHUNKS_40,
        fields: FIELDS_40,
    },
    ChunkDefinition {
        schema_key: "schema_0014",
        name: "tlBoolChannelChunk",
        chunk_id_expr: "Pure3D::Animation::ChannelData::BOOL",
        subchunks: SUBCHUNKS_41,
        fields: FIELDS_41,
    },
    ChunkDefinition {
        schema_key: "schema_0014",
        name: "tlColourChannelChunk",
        chunk_id_expr: "Pure3D::Animation::ChannelData::COLOUR",
        subchunks: SUBCHUNKS_42,
        fields: FIELDS_42,
    },
    ChunkDefinition {
        schema_key: "schema_0014",
        name: "tlEventDataImageChunk",
        chunk_id_expr:
            "Pure3D::Animation::ChannelData::EVENT_OBJECT_DATA_IMAGE",
        subchunks: SUBCHUNKS_43,
        fields: FIELDS_43,
    },
    ChunkDefinition {
        schema_key: "schema_0014",
        name: "tlEventChunk",
        chunk_id_expr: "Pure3D::Animation::ChannelData::EVENT_OBJECT",
        subchunks: SUBCHUNKS_44,
        fields: FIELDS_44,
    },
    ChunkDefinition {
        schema_key: "schema_0014",
        name: "tlEventChannelChunk",
        chunk_id_expr: "Pure3D::Animation::ChannelData::EVENT",
        subchunks: SUBCHUNKS_45,
        fields: FIELDS_45,
    },
    ChunkDefinition {
        schema_key: "schema_0015",
        name: "tlCollisionObjectChunk",
        chunk_id_expr: "Simulation::Collision::OBJECT",
        subchunks: SUBCHUNKS_46,
        fields: FIELDS_46,
    },
    ChunkDefinition {
        schema_key: "schema_0015",
        name: "tlCollisionObjectAttributeChunk",
        chunk_id_expr: "Simulation::Collision::ATTRIBUTE",
        subchunks: SUBCHUNKS_47,
        fields: FIELDS_47,
    },
    ChunkDefinition {
        schema_key: "schema_0015",
        name: "tlCollisionVolumeOwnerChunk",
        chunk_id_expr: "Simulation::Collision::OWNER",
        subchunks: SUBCHUNKS_48,
        fields: FIELDS_48,
    },
    ChunkDefinition {
        schema_key: "schema_0015",
        name: "tlCollisionVolumeOwnerNameChunk",
        chunk_id_expr: "Simulation::Collision::OWNERNAME",
        subchunks: SUBCHUNKS_49,
        fields: FIELDS_49,
    },
    ChunkDefinition {
        schema_key: "schema_0015",
        name: "tlSelfCollisionChunk",
        chunk_id_expr: "Simulation::Collision::SELFCOLLISION",
        subchunks: SUBCHUNKS_50,
        fields: FIELDS_50,
    },
    ChunkDefinition {
        schema_key: "schema_0015",
        name: "tlCollisionVolumeChunk",
        chunk_id_expr: "Simulation::Collision::VOLUME",
        subchunks: SUBCHUNKS_51,
        fields: FIELDS_51,
    },
    ChunkDefinition {
        schema_key: "schema_0015",
        name: "tlBBoxVolumeChunk",
        chunk_id_expr: "Simulation::Collision::BBOX",
        subchunks: SUBCHUNKS_52,
        fields: FIELDS_52,
    },
    ChunkDefinition {
        schema_key: "schema_0015",
        name: "tlSphereVolumeChunk",
        chunk_id_expr: "Simulation::Collision::SPHERE",
        subchunks: SUBCHUNKS_53,
        fields: FIELDS_53,
    },
    ChunkDefinition {
        schema_key: "schema_0015",
        name: "tlCylinderVolumeChunk",
        chunk_id_expr: "Simulation::Collision::CYLINDER",
        subchunks: SUBCHUNKS_54,
        fields: FIELDS_54,
    },
    ChunkDefinition {
        schema_key: "schema_0015",
        name: "tlOBBoxVolumeChunk",
        chunk_id_expr: "Simulation::Collision::OBBOX",
        subchunks: SUBCHUNKS_55,
        fields: FIELDS_55,
    },
    ChunkDefinition {
        schema_key: "schema_0015",
        name: "tlWallVolumeChunk",
        chunk_id_expr: "Simulation::Collision::WALL",
        subchunks: SUBCHUNKS_56,
        fields: FIELDS_56,
    },
    ChunkDefinition {
        schema_key: "schema_0015",
        name: "tlCollisionVectorChunk",
        chunk_id_expr: "Simulation::Collision::VECTOR",
        subchunks: SUBCHUNKS_57,
        fields: FIELDS_57,
    },
    ChunkDefinition {
        schema_key: "schema_0016",
        name: "tlCompositeDrawableChunk16",
        chunk_id_expr: "P3D_COMPOSITE_DRAWABLE",
        subchunks: SUBCHUNKS_58,
        fields: FIELDS_58,
    },
    ChunkDefinition {
        schema_key: "schema_0016",
        name: "tlCompositeDrawableSkinListChunk16",
        chunk_id_expr: "P3D_COMPOSITE_DRAWABLE_SKIN_LIST",
        subchunks: SUBCHUNKS_59,
        fields: FIELDS_59,
    },
    ChunkDefinition {
        schema_key: "schema_0016",
        name: "tlCompositeDrawablePropListChunk16",
        chunk_id_expr: "P3D_COMPOSITE_DRAWABLE_PROP_LIST",
        subchunks: SUBCHUNKS_60,
        fields: FIELDS_60,
    },
    ChunkDefinition {
        schema_key: "schema_0016",
        name: "tlCompositeDrawableSkinChunk16",
        chunk_id_expr: "P3D_COMPOSITE_DRAWABLE_SKIN",
        subchunks: SUBCHUNKS_61,
        fields: FIELDS_61,
    },
    ChunkDefinition {
        schema_key: "schema_0016",
        name: "tlCompositeDrawablePropChunk16",
        chunk_id_expr: "P3D_COMPOSITE_DRAWABLE_PROP",
        subchunks: SUBCHUNKS_62,
        fields: FIELDS_62,
    },
    ChunkDefinition {
        schema_key: "schema_0016",
        name: "tlCompositeDrawableEffectListChunk16",
        chunk_id_expr: "P3D_COMPOSITE_DRAWABLE_EFFECT_LIST",
        subchunks: SUBCHUNKS_63,
        fields: FIELDS_63,
    },
    ChunkDefinition {
        schema_key: "schema_0016",
        name: "tlCompositeDrawableEffectChunk16",
        chunk_id_expr: "P3D_COMPOSITE_DRAWABLE_EFFECT",
        subchunks: SUBCHUNKS_64,
        fields: FIELDS_64,
    },
    ChunkDefinition {
        schema_key: "schema_0016",
        name: "tlCompositeDrawableSortOrderChunk16",
        chunk_id_expr: "P3D_COMPOSITE_DRAWABLE_SORTORDER",
        subchunks: SUBCHUNKS_65,
        fields: FIELDS_65,
    },
    ChunkDefinition {
        schema_key: "schema_0017",
        name: "tlCompositeSkinChunk16",
        chunk_id_expr: "P3D_COMPOSITE_SKIN",
        subchunks: SUBCHUNKS_66,
        fields: FIELDS_66,
    },
    ChunkDefinition {
        schema_key: "schema_0017",
        name: "tlCompoundMeshNodeChunk16",
        chunk_id_expr: "P3D_COMPOSITE_SKIN_SUBSKIN_LIST",
        subchunks: SUBCHUNKS_67,
        fields: FIELDS_67,
    },
    ChunkDefinition {
        schema_key: "schema_0017",
        name: "tlCompositeSkinPropList16",
        chunk_id_expr: "P3D_COMPOSITE_SKIN_PROP_LIST",
        subchunks: SUBCHUNKS_68,
        fields: FIELDS_68,
    },
    ChunkDefinition {
        schema_key: "schema_0017",
        name: "tlCompositeSkinProp",
        chunk_id_expr: "P3D_COMPOSITE_SKIN_SUBSKIN",
        subchunks: SUBCHUNKS_69,
        fields: FIELDS_69,
    },
    ChunkDefinition {
        schema_key: "schema_0017",
        name: "tlCompositeSkinProp",
        chunk_id_expr: "P3D_COMPOSITE_SKIN_PROP",
        subchunks: SUBCHUNKS_70,
        fields: FIELDS_70,
    },
    ChunkDefinition {
        schema_key: "schema_0018",
        name: "tlDynaPhysDSGChunk",
        chunk_id_expr: "SRR2::ChunkID::DYNA_PHYS_DSG",
        subchunks: SUBCHUNKS_71,
        fields: FIELDS_71,
    },
    ChunkDefinition {
        schema_key: "schema_0019",
        name: "tlEntityChannelChunk16",
        chunk_id_expr: "P3D_ENTITY_ANIM_CHANNEL",
        subchunks: SUBCHUNKS_72,
        fields: FIELDS_72,
    },
    ChunkDefinition {
        schema_key: "schema_0020",
        name: "entity_dsg",
        chunk_id_expr: "SRR2::ChunkID::ENTITY_DSG",
        subchunks: SUBCHUNKS_73,
        fields: FIELDS_73,
    },
    ChunkDefinition {
        schema_key: "schema_0021",
        name: "tlExportInfoNamedIntChunk16",
        chunk_id_expr: "P3D_EXPORT_NAMED_INT",
        subchunks: SUBCHUNKS_74,
        fields: FIELDS_74,
    },
    ChunkDefinition {
        schema_key: "schema_0021",
        name: "tlExportInfoNamedStringChunk16",
        chunk_id_expr: "P3D_EXPORT_NAMED_STRING",
        subchunks: SUBCHUNKS_75,
        fields: FIELDS_75,
    },
    ChunkDefinition {
        schema_key: "schema_0021",
        name: "tlExportInfoChunk16",
        chunk_id_expr: "P3D_EXPORT_INFO",
        subchunks: SUBCHUNKS_76,
        fields: FIELDS_76,
    },
    ChunkDefinition {
        schema_key: "schema_0022",
        name: "tlExpressionGroupChunk16",
        chunk_id_expr: "P3D_EXPRESSION_GROUP",
        subchunks: SUBCHUNKS_77,
        fields: FIELDS_77,
    },
    ChunkDefinition {
        schema_key: "schema_0022",
        name: "tlExpressionAnimChannelChunk16",
        chunk_id_expr: "P3D_EXPRESSION_ANIM_CHANNEL",
        subchunks: SUBCHUNKS_78,
        fields: FIELDS_78,
    },
    ChunkDefinition {
        schema_key: "schema_0022",
        name: "tlExpressionAnimChunk16",
        chunk_id_expr: "P3D_EXPRESSION_ANIM",
        subchunks: SUBCHUNKS_79,
        fields: FIELDS_79,
    },
    ChunkDefinition {
        schema_key: "schema_0022",
        name: "tlExpressionMixerChunk16",
        chunk_id_expr: "P3D_EXPRESSION_MIXER",
        subchunks: SUBCHUNKS_80,
        fields: FIELDS_80,
    },
    ChunkDefinition {
        schema_key: "schema_0023",
        name: "tlExpressionChunk",
        chunk_id_expr: "Pure3D::Expression::VERTEX_EXPRESSION",
        subchunks: SUBCHUNKS_81,
        fields: FIELDS_81,
    },
    ChunkDefinition {
        schema_key: "schema_0023",
        name: "tlExpressionGroupChunk",
        chunk_id_expr: "Pure3D::Expression::VERTEX_EXPRESSION_GROUP",
        subchunks: SUBCHUNKS_82,
        fields: FIELDS_82,
    },
    ChunkDefinition {
        schema_key: "schema_0023",
        name: "tlExpressionMixerChunk",
        chunk_id_expr: "Pure3D::Expression::VERTEX_EXPRESSION_MIXER",
        subchunks: SUBCHUNKS_83,
        fields: FIELDS_83,
    },
    ChunkDefinition {
        schema_key: "schema_0024",
        name: "tlExtraMatrixChunk",
        chunk_id_expr: "SRR2::ChunkID::EXTRA_MATRIX",
        subchunks: SUBCHUNKS_84,
        fields: FIELDS_84,
    },
    ChunkDefinition {
        schema_key: "schema_0025",
        name: "fence_dsg",
        chunk_id_expr: "SRR2::ChunkID::FENCE_DSG",
        subchunks: SUBCHUNKS_85,
        fields: FIELDS_85,
    },
    ChunkDefinition {
        schema_key: "schema_0026",
        name: "tlFenceLineChunk",
        chunk_id_expr: "SRR2::ChunkID::FENCELINE",
        subchunks: SUBCHUNKS_86,
        fields: FIELDS_86,
    },
    ChunkDefinition {
        schema_key: "schema_0027",
        name: "tlFlexibleJointChunk",
        chunk_id_expr: "Simulation::Flexible::JOINT",
        subchunks: SUBCHUNKS_87,
        fields: FIELDS_87,
    },
    ChunkDefinition {
        schema_key: "schema_0027",
        name: "tlFlexibleLambdaJointParamChunk",
        chunk_id_expr: "Simulation::Flexible::JOINT_LAMBDA",
        subchunks: SUBCHUNKS_88,
        fields: FIELDS_88,
    },
    ChunkDefinition {
        schema_key: "schema_0027",
        name: "tlFlexibleJointParametersChunk",
        chunk_id_expr: "Simulation::Flexible::JOINT_PARAMETERS",
        subchunks: SUBCHUNKS_89,
        fields: FIELDS_89,
    },
    ChunkDefinition {
        schema_key: "schema_0027",
        name: "tlFlexibleJointDefinitionChunk",
        chunk_id_expr: "Simulation::Flexible::JOINT_DEFINITION",
        subchunks: SUBCHUNKS_90,
        fields: FIELDS_90,
    },
    ChunkDefinition {
        schema_key: "schema_0028",
        name: "tlFlexibleObjectChunk",
        chunk_id_expr: "Simulation::Flexible::OBJECT",
        subchunks: SUBCHUNKS_91,
        fields: FIELDS_91,
    },
    ChunkDefinition {
        schema_key: "schema_0028",
        name: "tlFlexibleObjectFixParticleChunk",
        chunk_id_expr: "Simulation::Flexible::FIX_PARTICLE",
        subchunks: SUBCHUNKS_92,
        fields: FIELDS_92,
    },
    ChunkDefinition {
        schema_key: "schema_0028",
        name: "tlFlexibleObjectMapToVLChunk",
        chunk_id_expr: "Simulation::Flexible::MAP_VL",
        subchunks: SUBCHUNKS_93,
        fields: FIELDS_93,
    },
    ChunkDefinition {
        schema_key: "schema_0028",
        name: "tlFlexibleObjectTriMapChunk",
        chunk_id_expr: "Simulation::Flexible::TRI_MAP",
        subchunks: SUBCHUNKS_94,
        fields: FIELDS_94,
    },
    ChunkDefinition {
        schema_key: "schema_0028",
        name: "tlFlexibleObjectEdgeMapChunk",
        chunk_id_expr: "Simulation::Flexible::EDGE_MAP",
        subchunks: SUBCHUNKS_95,
        fields: FIELDS_95,
    },
    ChunkDefinition {
        schema_key: "schema_0028",
        name: "tlFlexibleObjectEdgeLengthChunk",
        chunk_id_expr: "Simulation::Flexible::EDGE_LEN",
        subchunks: SUBCHUNKS_96,
        fields: FIELDS_96,
    },
    ChunkDefinition {
        schema_key: "schema_0028",
        name: "tlFlexibleObjectParamChunk",
        chunk_id_expr: "Simulation::Flexible::OBJECT_PARAMETERS",
        subchunks: SUBCHUNKS_97,
        fields: FIELDS_97,
    },
    ChunkDefinition {
        schema_key: "schema_0028",
        name: "tlFlexibleLambdaObjectParamChunk",
        chunk_id_expr: "Simulation::Flexible::OBJECT_LAMBDA",
        subchunks: SUBCHUNKS_98,
        fields: FIELDS_98,
    },
    ChunkDefinition {
        schema_key: "schema_0029",
        name: "tlFollowCamDataChunk",
        chunk_id_expr: "SRR2::ChunkID::FOLLOWCAM",
        subchunks: SUBCHUNKS_99,
        fields: FIELDS_99,
    },
    ChunkDefinition {
        schema_key: "schema_0030",
        name: "tlTextureFontChunk",
        chunk_id_expr: "Pure3D::Font::TEXTURE_FONT",
        subchunks: SUBCHUNKS_100,
        fields: FIELDS_100,
    },
    ChunkDefinition {
        schema_key: "schema_0030",
        name: "tlTextureGlyphListChunk",
        chunk_id_expr: "Pure3D::Font::TEXTURE_GLYPH_LIST",
        subchunks: SUBCHUNKS_101,
        fields: FIELDS_101,
    },
    ChunkDefinition {
        schema_key: "schema_0030",
        name: "tlImageFontChunk",
        chunk_id_expr: "Pure3D::Font::IMAGE_FONT",
        subchunks: SUBCHUNKS_102,
        fields: FIELDS_102,
    },
    ChunkDefinition {
        schema_key: "schema_0030",
        name: "tlImageGlyphListChunk",
        chunk_id_expr: "Pure3D::Font::IMAGE_GLYPH_LIST",
        subchunks: SUBCHUNKS_103,
        fields: FIELDS_103,
    },
    ChunkDefinition {
        schema_key: "schema_0031",
        name: "tlFrameControllerChunk",
        chunk_id_expr:
            "Pure3D::Animation::FrameControllerData::FRAME_CONTROLLER",
        subchunks: SUBCHUNKS_104,
        fields: FIELDS_104,
    },
    ChunkDefinition {
        schema_key: "schema_0032",
        name: "tlFrameControllerChunk16",
        chunk_id_expr: "P3D_FRAME_CONTROLLER",
        subchunks: SUBCHUNKS_105,
        fields: FIELDS_105,
    },
    ChunkDefinition {
        schema_key: "schema_0033",
        name: "game_attr",
        chunk_id_expr: "Pure3D::GameAttr::GAME_ATTR",
        subchunks: SUBCHUNKS_106,
        fields: FIELDS_106,
    },
    ChunkDefinition {
        schema_key: "schema_0033",
        name: "tlGameAttrIntParamChunk",
        chunk_id_expr: "Pure3D::GameAttr::INT_PARAM",
        subchunks: SUBCHUNKS_107,
        fields: FIELDS_107,
    },
    ChunkDefinition {
        schema_key: "schema_0033",
        name: "tlGameAttrFloatParamChunk",
        chunk_id_expr: "Pure3D::GameAttr::FLOAT_PARAM",
        subchunks: SUBCHUNKS_108,
        fields: FIELDS_108,
    },
    ChunkDefinition {
        schema_key: "schema_0033",
        name: "tlGameAttrColourParamChunk",
        chunk_id_expr: "Pure3D::GameAttr::COLOUR_PARAM",
        subchunks: SUBCHUNKS_109,
        fields: FIELDS_109,
    },
    ChunkDefinition {
        schema_key: "schema_0033",
        name: "tlGameAttrVectorParamChunk",
        chunk_id_expr: "Pure3D::GameAttr::VECTOR_PARAM",
        subchunks: SUBCHUNKS_110,
        fields: FIELDS_110,
    },
    ChunkDefinition {
        schema_key: "schema_0033",
        name: "tlGameAttrMatrixParamChunk",
        chunk_id_expr: "Pure3D::GameAttr::MATRIX_PARAM",
        subchunks: SUBCHUNKS_111,
        fields: FIELDS_111,
    },
    ChunkDefinition {
        schema_key: "schema_0034",
        name: "tlHistoryChunk16",
        chunk_id_expr: "P3D_HISTORY",
        subchunks: SUBCHUNKS_112,
        fields: FIELDS_112,
    },
    ChunkDefinition {
        schema_key: "schema_0035",
        name: "tlImageDataChunk",
        chunk_id_expr: "Pure3D::Texture::IMAGE_DATA",
        subchunks: SUBCHUNKS_113,
        fields: FIELDS_113,
    },
    ChunkDefinition {
        schema_key: "schema_0035",
        name: "tlImageFileNameChunk",
        chunk_id_expr: "Pure3D::Texture::IMAGE_FILENAME",
        subchunks: SUBCHUNKS_114,
        fields: FIELDS_114,
    },
    ChunkDefinition {
        schema_key: "schema_0035",
        name: "image",
        chunk_id_expr: "Pure3D::Texture::IMAGE",
        subchunks: SUBCHUNKS_115,
        fields: FIELDS_115,
    },
    ChunkDefinition {
        schema_key: "schema_0035",
        name: "tlVolumeImageChunk",
        chunk_id_expr: "Pure3D::Texture::VOLUME_IMAGE",
        subchunks: SUBCHUNKS_116,
        fields: FIELDS_116,
    },
    ChunkDefinition {
        schema_key: "schema_0036",
        name: "tlInstaEntityDSGChunk",
        chunk_id_expr: "SRR2::ChunkID::INSTA_ENTITY_DSG",
        subchunks: SUBCHUNKS_117,
        fields: FIELDS_117,
    },
    ChunkDefinition {
        schema_key: "schema_0037",
        name: "tlInstancesChunk",
        chunk_id_expr: "SRR2::ChunkID::INSTANCES",
        subchunks: SUBCHUNKS_118,
        fields: FIELDS_118,
    },
    ChunkDefinition {
        schema_key: "schema_0038",
        name: "tlInstAnimDynaPhysDSGChunk",
        chunk_id_expr: "SRR2::ChunkID::INSTA_ANIM_DYNA_PHYS_DSG",
        subchunks: SUBCHUNKS_119,
        fields: FIELDS_119,
    },
    ChunkDefinition {
        schema_key: "schema_0039",
        name: "tlInstaStaticPhysDSGChunk",
        chunk_id_expr: "SRR2::ChunkID::INSTA_STATIC_PHYS_DSG",
        subchunks: SUBCHUNKS_120,
        fields: FIELDS_120,
    },
    ChunkDefinition {
        schema_key: "schema_0040",
        name: "tlInstParticleSystemChunk",
        chunk_id_expr: "SRR2::ChunkID::INST_PARTICLE_SYSTEM",
        subchunks: SUBCHUNKS_121,
        fields: FIELDS_121,
    },
    ChunkDefinition {
        schema_key: "schema_0041",
        name: "intersect_dsg",
        chunk_id_expr: "SRR2::ChunkID::INTERSECT_DSG",
        subchunks: SUBCHUNKS_122,
        fields: FIELDS_122,
    },
    ChunkDefinition {
        schema_key: "schema_0042",
        name: "intersection",
        chunk_id_expr: "SRR2::ChunkID::INTERSECTION",
        subchunks: SUBCHUNKS_123,
        fields: FIELDS_123,
    },
    ChunkDefinition {
        schema_key: "schema_0043",
        name: "tlLensFlareDSGChunk",
        chunk_id_expr: "SRR2::ChunkID::LENS_FLARE_DSG",
        subchunks: SUBCHUNKS_124,
        fields: FIELDS_124,
    },
    ChunkDefinition {
        schema_key: "schema_0044",
        name: "tlLightAnimChunk16",
        chunk_id_expr: "P3D_LIGHT_ANIM",
        subchunks: SUBCHUNKS_125,
        fields: FIELDS_125,
    },
    ChunkDefinition {
        schema_key: "schema_0044",
        name: "tlLightAnimChannelChunk16",
        chunk_id_expr: "P3D_LIGHT_ANIM_CHANNEL",
        subchunks: SUBCHUNKS_126,
        fields: FIELDS_126,
    },
    ChunkDefinition {
        schema_key: "schema_0044",
        name: "tlLightAnimColourChannelChunk16",
        chunk_id_expr: "P3D_LIGHT_ANIM_COLOUR_CHANNEL",
        subchunks: SUBCHUNKS_127,
        fields: FIELDS_127,
    },
    ChunkDefinition {
        schema_key: "schema_0044",
        name: "tlLightAnimParamChannelChunk16",
        chunk_id_expr: "P3D_LIGHT_ANIM_PARAM_CHANNEL",
        subchunks: SUBCHUNKS_128,
        fields: FIELDS_128,
    },
    ChunkDefinition {
        schema_key: "schema_0044",
        name: "tlLightAnimEnableChannelChunk16",
        chunk_id_expr: "P3D_LIGHT_ANIM_ENABLE_CHANNEL",
        subchunks: SUBCHUNKS_129,
        fields: FIELDS_129,
    },
    ChunkDefinition {
        schema_key: "schema_0045",
        name: "light",
        chunk_id_expr: "Pure3D::Light::LIGHT",
        subchunks: SUBCHUNKS_130,
        fields: FIELDS_130,
    },
    ChunkDefinition {
        schema_key: "schema_0045",
        name: "tlLightDirectionChunk",
        chunk_id_expr: "Pure3D::Light::DIRECTION",
        subchunks: SUBCHUNKS_131,
        fields: FIELDS_131,
    },
    ChunkDefinition {
        schema_key: "schema_0045",
        name: "tlLightPositionChunk",
        chunk_id_expr: "Pure3D::Light::POSITION",
        subchunks: SUBCHUNKS_132,
        fields: FIELDS_132,
    },
    ChunkDefinition {
        schema_key: "schema_0045",
        name: "tlLightConeParamChunk",
        chunk_id_expr: "Pure3D::Light::CONE_PARAM",
        subchunks: SUBCHUNKS_133,
        fields: FIELDS_133,
    },
    ChunkDefinition {
        schema_key: "schema_0045",
        name: "tlLightShadowChunk",
        chunk_id_expr: "Pure3D::Light::SHADOW",
        subchunks: SUBCHUNKS_134,
        fields: FIELDS_134,
    },
    ChunkDefinition {
        schema_key: "schema_0045",
        name: "tlLightDecayRangeRotationYChunk",
        chunk_id_expr: "Pure3D::Light::DECAY_RANGE_ROTATION_Y",
        subchunks: SUBCHUNKS_135,
        fields: FIELDS_135,
    },
    ChunkDefinition {
        schema_key: "schema_0045",
        name: "tlLightDecayRangeChunk",
        chunk_id_expr: "Pure3D::Light::DECAY_RANGE",
        subchunks: SUBCHUNKS_136,
        fields: FIELDS_136,
    },
    ChunkDefinition {
        schema_key: "schema_0045",
        name: "tlLightIlluminationTypeChunk",
        chunk_id_expr: "Pure3D::Light::ILLUMINATION_TYPE",
        subchunks: SUBCHUNKS_137,
        fields: FIELDS_137,
    },
    ChunkDefinition {
        schema_key: "schema_0046",
        name: "tlLightGroupChunk16",
        chunk_id_expr: "P3D_LIGHT_GROUP",
        subchunks: SUBCHUNKS_138,
        fields: FIELDS_138,
    },
    ChunkDefinition {
        schema_key: "schema_0047",
        name: "tlLocatorChunk",
        chunk_id_expr: "Pure3D::Locator::LOCATOR",
        subchunks: SUBCHUNKS_139,
        fields: FIELDS_139,
    },
    ChunkDefinition {
        schema_key: "schema_0048",
        name: "tlMemorySectionChunk",
        chunk_id_expr: "MemorySection::MEMORYSECTION",
        subchunks: SUBCHUNKS_140,
        fields: FIELDS_140,
    },
    ChunkDefinition {
        schema_key: "schema_0049",
        name: "mesh",
        chunk_id_expr: "Pure3D::Mesh::MESH",
        subchunks: SUBCHUNKS_141,
        fields: FIELDS_141,
    },
    ChunkDefinition {
        schema_key: "schema_0049",
        name: "tlRenderStatusChunk",
        chunk_id_expr: "Pure3D::Mesh::RENDERSTATUS",
        subchunks: SUBCHUNKS_142,
        fields: FIELDS_142,
    },
    ChunkDefinition {
        schema_key: "schema_0050",
        name: "tlMultiControllerChunk16",
        chunk_id_expr: "P3D_MULTI_CONTROLLER",
        subchunks: SUBCHUNKS_143,
        fields: FIELDS_143,
    },
    ChunkDefinition {
        schema_key: "schema_0050",
        name: "tlMultiControllerTracksChunk16",
        chunk_id_expr: "P3D_MULTI_CONTROLLER_TRACKS",
        subchunks: SUBCHUNKS_144,
        fields: FIELDS_144,
    },
    ChunkDefinition {
        schema_key: "schema_0050",
        name: "tlMultiControllerTrackChunk16",
        chunk_id_expr: "P3D_MULTI_CONTROLLER_TRACK",
        subchunks: SUBCHUNKS_145,
        fields: FIELDS_145,
    },
    ChunkDefinition {
        schema_key: "schema_0051",
        name: "tlObjectAttributeChunk",
        chunk_id_expr: "SRR2::ChunkID::OBJECT_ATTRIBUTES",
        subchunks: SUBCHUNKS_146,
        fields: FIELDS_146,
    },
    ChunkDefinition {
        schema_key: "schema_0052",
        name: "tlLensFlareChunk",
        chunk_id_expr: "Pure3D::OpticEffect::LENS_FLARE",
        subchunks: SUBCHUNKS_147,
        fields: FIELDS_147,
    },
    ChunkDefinition {
        schema_key: "schema_0052",
        name: "tlLensFlareGroupChunk",
        chunk_id_expr: "Pure3D::OpticEffect::LENS_FLARE_GROUP",
        subchunks: SUBCHUNKS_148,
        fields: FIELDS_148,
    },
    ChunkDefinition {
        schema_key: "schema_0052",
        name: "tlOpticVectorV14Chunk",
        chunk_id_expr: "Pure3D::OpticEffect::VECTOR_V14",
        subchunks: SUBCHUNKS_149,
        fields: FIELDS_149,
    },
    ChunkDefinition {
        schema_key: "schema_0052",
        name: "tlCoronaV14Chunk",
        chunk_id_expr: "Pure3D::OpticEffect::CORONA_V14",
        subchunks: SUBCHUNKS_150,
        fields: FIELDS_150,
    },
    ChunkDefinition {
        schema_key: "schema_0052",
        name: "tlLensFlareParentV14Chunk",
        chunk_id_expr: "Pure3D::OpticEffect::LENS_FLARE_PARENT_V14",
        subchunks: SUBCHUNKS_151,
        fields: FIELDS_151,
    },
    ChunkDefinition {
        schema_key: "schema_0052",
        name: "tlLensFlareV14Chunk",
        chunk_id_expr: "Pure3D::OpticEffect::LENS_FLARE_V14",
        subchunks: SUBCHUNKS_152,
        fields: FIELDS_152,
    },
    ChunkDefinition {
        schema_key: "schema_0053",
        name: "tlParticleInstancingInfoChunk",
        chunk_id_expr: "Pure3D::ParticleSystem::INSTANCING_INFO",
        subchunks: SUBCHUNKS_153,
        fields: FIELDS_153,
    },
    ChunkDefinition {
        schema_key: "schema_0053",
        name: "particle_system_factory",
        chunk_id_expr: "Pure3D::ParticleSystem::SYSTEM_FACTORY",
        subchunks: SUBCHUNKS_154,
        fields: FIELDS_154,
    },
    ChunkDefinition {
        schema_key: "schema_0053",
        name: "particle_system",
        chunk_id_expr: "Pure3D::ParticleSystem::SYSTEM",
        subchunks: SUBCHUNKS_155,
        fields: FIELDS_155,
    },
    ChunkDefinition {
        schema_key: "schema_0053",
        name: "tlParticleAnimationChunk",
        chunk_id_expr: "Pure3D::ParticleSystem::PARTICLE_ANIMATION",
        subchunks: SUBCHUNKS_156,
        fields: FIELDS_156,
    },
    ChunkDefinition {
        schema_key: "schema_0053",
        name: "tlEmitterAnimationChunk",
        chunk_id_expr: "Pure3D::ParticleSystem::EMITTER_ANIMATION",
        subchunks: SUBCHUNKS_157,
        fields: FIELDS_157,
    },
    ChunkDefinition {
        schema_key: "schema_0053",
        name: "tlGeneratorAnimationChunk",
        chunk_id_expr: "Pure3D::ParticleSystem::GENERATOR_ANIMATION",
        subchunks: SUBCHUNKS_158,
        fields: FIELDS_158,
    },
    ChunkDefinition {
        schema_key: "schema_0053",
        name: "tlBaseEmitterFactoryChunk",
        chunk_id_expr: "Pure3D::ParticleSystem::BASE_EMITTER_FACTORY",
        subchunks: SUBCHUNKS_159,
        fields: FIELDS_159,
    },
    ChunkDefinition {
        schema_key: "schema_0053",
        name: "tlSpriteEmitterFactoryChunk",
        chunk_id_expr: "Pure3D::ParticleSystem::SPRITE_EMITTER_FACTORY",
        subchunks: SUBCHUNKS_160,
        fields: FIELDS_160,
    },
    ChunkDefinition {
        schema_key: "schema_0053",
        name: "tlDrawableEmitterFactoryChunk",
        chunk_id_expr: "Pure3D::ParticleSystem::DRAWABLE_EMITTER_FACTORY",
        subchunks: SUBCHUNKS_161,
        fields: FIELDS_161,
    },
    ChunkDefinition {
        schema_key: "schema_0054",
        name: "tlPedpathChunk",
        chunk_id_expr: "SRR2::ChunkID::PED_PATH",
        subchunks: SUBCHUNKS_162,
        fields: FIELDS_162,
    },
    ChunkDefinition {
        schema_key: "schema_0055",
        name: "tlPhotonMapChunk",
        chunk_id_expr: "Pure3D::Light::PHOTON_MAP",
        subchunks: SUBCHUNKS_163,
        fields: FIELDS_163,
    },
    ChunkDefinition {
        schema_key: "schema_0056",
        name: "tlPhysicsObjectChunk",
        chunk_id_expr: "Simulation::Physics::OBJECT",
        subchunks: SUBCHUNKS_164,
        fields: FIELDS_164,
    },
    ChunkDefinition {
        schema_key: "schema_0056",
        name: "tlPhysicsJointChunk",
        chunk_id_expr: "Simulation::Physics::JOINT",
        subchunks: SUBCHUNKS_165,
        fields: FIELDS_165,
    },
    ChunkDefinition {
        schema_key: "schema_0056",
        name: "tlPhysicsVectorChunk",
        chunk_id_expr: "Simulation::Physics::VECTOR",
        subchunks: SUBCHUNKS_166,
        fields: FIELDS_166,
    },
    ChunkDefinition {
        schema_key: "schema_0056",
        name: "tlPhysicsInertiaMatrixChunk",
        chunk_id_expr: "Simulation::Physics::IMAT",
        subchunks: SUBCHUNKS_167,
        fields: FIELDS_167,
    },
    ChunkDefinition {
        schema_key: "schema_0057",
        name: "tlPhysWrapperChunk",
        chunk_id_expr: "SRR2::ChunkID::PHYS_WRAPPER",
        subchunks: SUBCHUNKS_168,
        fields: FIELDS_168,
    },
    ChunkDefinition {
        schema_key: "schema_0058",
        name: "tlPoseAnimChunk16",
        chunk_id_expr: "P3D_POSE_ANIM",
        subchunks: SUBCHUNKS_169,
        fields: FIELDS_169,
    },
    ChunkDefinition {
        schema_key: "schema_0058",
        name: "tlPoseJointListChunk16",
        chunk_id_expr: "P3D_JOINT_LIST",
        subchunks: SUBCHUNKS_170,
        fields: FIELDS_170,
    },
    ChunkDefinition {
        schema_key: "schema_0058",
        name: "tlPoseAnimMirroredChunk16",
        chunk_id_expr: "P3D_POSE_ANIM_MIRRORED",
        subchunks: SUBCHUNKS_171,
        fields: FIELDS_171,
    },
    ChunkDefinition {
        schema_key: "schema_0058",
        name: "tlAnimChannelChunk16",
        chunk_id_expr: "P3D_ANIM_CHANNEL",
        subchunks: SUBCHUNKS_172,
        fields: FIELDS_172,
    },
    ChunkDefinition {
        schema_key: "schema_0058",
        name: "tlChannel1DOFChunk16",
        chunk_id_expr: "P3D_CHANNEL_1DOF",
        subchunks: SUBCHUNKS_173,
        fields: FIELDS_173,
    },
    ChunkDefinition {
        schema_key: "schema_0058",
        name: "tlChannel3DOFChunk16",
        chunk_id_expr: "P3D_CHANNEL_3DOF",
        subchunks: SUBCHUNKS_174,
        fields: FIELDS_174,
    },
    ChunkDefinition {
        schema_key: "schema_0058",
        name: "tlChannel1DOFAngleChunk16",
        chunk_id_expr: "P3D_CHANNEL_1DOF_ANGLE",
        subchunks: SUBCHUNKS_175,
        fields: FIELDS_175,
    },
    ChunkDefinition {
        schema_key: "schema_0058",
        name: "tlChannel3DOFAngleChunk16",
        chunk_id_expr: "P3D_CHANNEL_3DOF_ANGLE",
        subchunks: SUBCHUNKS_176,
        fields: FIELDS_176,
    },
    ChunkDefinition {
        schema_key: "schema_0058",
        name: "tlChannelQuaternionChunk16",
        chunk_id_expr: "P3D_CHANNEL_QUATERNION",
        subchunks: SUBCHUNKS_177,
        fields: FIELDS_177,
    },
    ChunkDefinition {
        schema_key: "schema_0058",
        name: "tlChannelStaticVectorChunk16",
        chunk_id_expr: "P3D_CHANNEL_STATIC",
        subchunks: SUBCHUNKS_178,
        fields: FIELDS_178,
    },
    ChunkDefinition {
        schema_key: "schema_0058",
        name: "tlChannelStaticAngleChunk16",
        chunk_id_expr: "P3D_CHANNEL_STATIC_ANGLE",
        subchunks: SUBCHUNKS_179,
        fields: FIELDS_179,
    },
    ChunkDefinition {
        schema_key: "schema_0058",
        name: "tlChannelStaticQuaternionChunk16",
        chunk_id_expr: "P3D_CHANNEL_STATIC_QUATERNION",
        subchunks: SUBCHUNKS_180,
        fields: FIELDS_180,
    },
    ChunkDefinition {
        schema_key: "schema_0058",
        name: "tlKeyListColourChunk16",
        chunk_id_expr: "P3D_KEYLIST_COLOUR",
        subchunks: SUBCHUNKS_181,
        fields: FIELDS_181,
    },
    ChunkDefinition {
        schema_key: "schema_0059",
        name: "tlPrimGroupChunk",
        chunk_id_expr: "Pure3D::Mesh::PRIMGROUP",
        subchunks: SUBCHUNKS_182,
        fields: FIELDS_182,
    },
    ChunkDefinition {
        schema_key: "schema_0059",
        name: "tlVertexShaderChunk",
        chunk_id_expr: "Pure3D::Mesh::VERTEXSHADER",
        subchunks: SUBCHUNKS_183,
        fields: FIELDS_183,
    },
    ChunkDefinition {
        schema_key: "schema_0059",
        name: "tlPositionListChunk",
        chunk_id_expr: "Pure3D::Mesh::POSITIONLIST",
        subchunks: SUBCHUNKS_184,
        fields: FIELDS_184,
    },
    ChunkDefinition {
        schema_key: "schema_0059",
        name: "tlNormalListChunk",
        chunk_id_expr: "Pure3D::Mesh::NORMALLIST",
        subchunks: SUBCHUNKS_185,
        fields: FIELDS_185,
    },
    ChunkDefinition {
        schema_key: "schema_0059",
        name: "tlTangentListChunk",
        chunk_id_expr: "Pure3D::Mesh::TANGENTLIST",
        subchunks: SUBCHUNKS_186,
        fields: FIELDS_186,
    },
    ChunkDefinition {
        schema_key: "schema_0059",
        name: "tlBinormalListChunk",
        chunk_id_expr: "Pure3D::Mesh::BINORMALLIST",
        subchunks: SUBCHUNKS_187,
        fields: FIELDS_187,
    },
    ChunkDefinition {
        schema_key: "schema_0059",
        name: "tlPackedNormalListChunk",
        chunk_id_expr: "Pure3D::Mesh::PACKEDNORMALLIST",
        subchunks: SUBCHUNKS_188,
        fields: FIELDS_188,
    },
    ChunkDefinition {
        schema_key: "schema_0059",
        name: "tlUVListChunk",
        chunk_id_expr: "Pure3D::Mesh::UVLIST",
        subchunks: SUBCHUNKS_189,
        fields: FIELDS_189,
    },
    ChunkDefinition {
        schema_key: "schema_0059",
        name: "tlColourListChunk",
        chunk_id_expr: "Pure3D::Mesh::COLOURLIST",
        subchunks: SUBCHUNKS_190,
        fields: FIELDS_190,
    },
    ChunkDefinition {
        schema_key: "schema_0059",
        name: "tlMultiColourListChunk",
        chunk_id_expr: "Pure3D::Mesh::MULTICOLOURLIST",
        subchunks: SUBCHUNKS_191,
        fields: FIELDS_191,
    },
    ChunkDefinition {
        schema_key: "schema_0059",
        name: "tlStripListChunk",
        chunk_id_expr: "Pure3D::Mesh::STRIPLIST",
        subchunks: SUBCHUNKS_192,
        fields: FIELDS_192,
    },
    ChunkDefinition {
        schema_key: "schema_0059",
        name: "tlIndexListChunk",
        chunk_id_expr: "Pure3D::Mesh::INDEXLIST",
        subchunks: SUBCHUNKS_193,
        fields: FIELDS_193,
    },
    ChunkDefinition {
        schema_key: "schema_0059",
        name: "tlExpressionOffsetsChunk",
        chunk_id_expr: "Pure3D::Mesh::EXPRESSIONOFFSETS",
        subchunks: SUBCHUNKS_194,
        fields: FIELDS_194,
    },
    ChunkDefinition {
        schema_key: "schema_0059",
        name: "tlOffsetListChunk",
        chunk_id_expr: "Pure3D::Mesh::OFFSETLIST",
        subchunks: SUBCHUNKS_195,
        fields: FIELDS_195,
    },
    ChunkDefinition {
        schema_key: "schema_0059",
        name: "tlMatrixListChunk",
        chunk_id_expr: "Pure3D::Mesh::MATRIXLIST",
        subchunks: SUBCHUNKS_196,
        fields: FIELDS_196,
    },
    ChunkDefinition {
        schema_key: "schema_0059",
        name: "tlWeightListChunk",
        chunk_id_expr: "Pure3D::Mesh::WEIGHTLIST",
        subchunks: SUBCHUNKS_197,
        fields: FIELDS_197,
    },
    ChunkDefinition {
        schema_key: "schema_0059",
        name: "tlMatrixPaletteChunk",
        chunk_id_expr: "Pure3D::Mesh::MATRIXPALETTE",
        subchunks: SUBCHUNKS_198,
        fields: FIELDS_198,
    },
    ChunkDefinition {
        schema_key: "schema_0059",
        name: "tlInstanceInfoChunk",
        chunk_id_expr: "Pure3D::Mesh::INSTANCEINFO",
        subchunks: SUBCHUNKS_199,
        fields: FIELDS_199,
    },
    ChunkDefinition {
        schema_key: "schema_0059",
        name: "tlPrimGroupMemoryImageVertexDescriptionChunk",
        chunk_id_expr: "Pure3D::Mesh::MEMORYIMAGEVERTEXDESCRIPTIONLIST",
        subchunks: SUBCHUNKS_200,
        fields: FIELDS_200,
    },
    ChunkDefinition {
        schema_key: "schema_0059",
        name: "tlPrimGroupMemoryImageVertexChunk",
        chunk_id_expr: "Pure3D::Mesh::MEMORYIMAGEVERTEXLIST",
        subchunks: SUBCHUNKS_201,
        fields: FIELDS_201,
    },
    ChunkDefinition {
        schema_key: "schema_0059",
        name: "tlPrimGroupMemoryImageIndexChunk",
        chunk_id_expr: "Pure3D::Mesh::MEMORYIMAGEINDEXLIST",
        subchunks: SUBCHUNKS_202,
        fields: FIELDS_202,
    },
    ChunkDefinition {
        schema_key: "schema_0060",
        name: "road",
        chunk_id_expr: "SRR2::ChunkID::ROAD",
        subchunks: SUBCHUNKS_203,
        fields: FIELDS_203,
    },
    ChunkDefinition {
        schema_key: "schema_0061",
        name: "tlRoadSegmentChunk",
        chunk_id_expr: "SRR2::ChunkID::ROAD_SEGMENT",
        subchunks: SUBCHUNKS_204,
        fields: FIELDS_204,
    },
    ChunkDefinition {
        schema_key: "schema_0062",
        name: "road_segment_data",
        chunk_id_expr: "SRR2::ChunkID::ROAD_SEGMENT_DATA",
        subchunks: SUBCHUNKS_205,
        fields: FIELDS_205,
    },
    ChunkDefinition {
        schema_key: "schema_0063",
        name: "scenegraph",
        chunk_id_expr: "Pure3D::SceneGraph::SCENEGRAPH",
        subchunks: SUBCHUNKS_206,
        fields: FIELDS_206,
    },
    ChunkDefinition {
        schema_key: "schema_0063",
        name: "tlScenegraphRootChunk",
        chunk_id_expr: "Pure3D::SceneGraph::ROOT",
        subchunks: SUBCHUNKS_207,
        fields: FIELDS_207,
    },
    ChunkDefinition {
        schema_key: "schema_0063",
        name: "tlScenegraphBranchChunk",
        chunk_id_expr: "Pure3D::SceneGraph::BRANCH",
        subchunks: SUBCHUNKS_208,
        fields: FIELDS_208,
    },
    ChunkDefinition {
        schema_key: "schema_0063",
        name: "tlScenegraphTransformChunk",
        chunk_id_expr: "Pure3D::SceneGraph::TRANSFORM",
        subchunks: SUBCHUNKS_209,
        fields: FIELDS_209,
    },
    ChunkDefinition {
        schema_key: "schema_0063",
        name: "tlScenegraphVisibilityChunk",
        chunk_id_expr: "Pure3D::SceneGraph::VISIBILITY",
        subchunks: SUBCHUNKS_210,
        fields: FIELDS_210,
    },
    ChunkDefinition {
        schema_key: "schema_0063",
        name: "tlScenegraphAttachmentChunk",
        chunk_id_expr: "Pure3D::SceneGraph::ATTACHMENT",
        subchunks: SUBCHUNKS_211,
        fields: FIELDS_211,
    },
    ChunkDefinition {
        schema_key: "schema_0063",
        name: "tlScenegraphAttachmentPointChunk",
        chunk_id_expr: "Pure3D::SceneGraph::ATTACHMENTPOINT",
        subchunks: SUBCHUNKS_212,
        fields: FIELDS_212,
    },
    ChunkDefinition {
        schema_key: "schema_0063",
        name: "tlScenegraphDrawableChunk",
        chunk_id_expr: "Pure3D::SceneGraph::DRAWABLE",
        subchunks: SUBCHUNKS_213,
        fields: FIELDS_213,
    },
    ChunkDefinition {
        schema_key: "schema_0063",
        name: "tlScenegraphCameraChunk",
        chunk_id_expr: "Pure3D::SceneGraph::CAMERA",
        subchunks: SUBCHUNKS_214,
        fields: FIELDS_214,
    },
    ChunkDefinition {
        schema_key: "schema_0063",
        name: "tlScenegraphLightGroupChunk",
        chunk_id_expr: "Pure3D::SceneGraph::LIGHTGROUP",
        subchunks: SUBCHUNKS_215,
        fields: FIELDS_215,
    },
    ChunkDefinition {
        schema_key: "schema_0063",
        name: "tlScenegraphSortOrderChunk",
        chunk_id_expr: "Pure3D::SceneGraph::SORTORDER",
        subchunks: SUBCHUNKS_216,
        fields: FIELDS_216,
    },
    ChunkDefinition {
        schema_key: "schema_0064",
        name: "tlScenegraphTransformAnimChunk16",
        chunk_id_expr: "P3D_SG_TRANSFORM_ANIM",
        subchunks: SUBCHUNKS_217,
        fields: FIELDS_217,
    },
    ChunkDefinition {
        schema_key: "schema_0065",
        name: "tlScroobyProjectChunk",
        chunk_id_expr: "Pure3D::Scrooby::PROJECT",
        subchunks: SUBCHUNKS_218,
        fields: FIELDS_218,
    },
    ChunkDefinition {
        schema_key: "schema_0065",
        name: "tlScroobyScreenChunk",
        chunk_id_expr: "Pure3D::Scrooby::SCREEN",
        subchunks: SUBCHUNKS_219,
        fields: FIELDS_219,
    },
    ChunkDefinition {
        schema_key: "schema_0065",
        name: "tlScroobyPageChunk",
        chunk_id_expr: "Pure3D::Scrooby::PAGE",
        subchunks: SUBCHUNKS_220,
        fields: FIELDS_220,
    },
    ChunkDefinition {
        schema_key: "schema_0065",
        name: "tlScroobyLayerChunk",
        chunk_id_expr: "Pure3D::Scrooby::LAYER",
        subchunks: SUBCHUNKS_221,
        fields: FIELDS_221,
    },
    ChunkDefinition {
        schema_key: "schema_0065",
        name: "tlScroobyStringTextBibleChunk",
        chunk_id_expr: "Pure3D::Scrooby::STRINGTEXTBIBLE",
        subchunks: SUBCHUNKS_222,
        fields: FIELDS_222,
    },
    ChunkDefinition {
        schema_key: "schema_0065",
        name: "tlScroobyStringHardCodedChunk",
        chunk_id_expr: "Pure3D::Scrooby::STRINGHARDCODED",
        subchunks: SUBCHUNKS_223,
        fields: FIELDS_223,
    },
    ChunkDefinition {
        schema_key: "schema_0065",
        name: "tlScroobyMultiTextChunk",
        chunk_id_expr: "Pure3D::Scrooby::MULTITEXT",
        subchunks: SUBCHUNKS_224,
        fields: FIELDS_224,
    },
    ChunkDefinition {
        schema_key: "schema_0065",
        name: "tlScroobyMultiSpriteChunk",
        chunk_id_expr: "Pure3D::Scrooby::MULTISPRITE",
        subchunks: SUBCHUNKS_225,
        fields: FIELDS_225,
    },
    ChunkDefinition {
        schema_key: "schema_0065",
        name: "tlScroobyPolygonChunk",
        chunk_id_expr: "Pure3D::Scrooby::POLYGON",
        subchunks: SUBCHUNKS_226,
        fields: FIELDS_226,
    },
    ChunkDefinition {
        schema_key: "schema_0065",
        name: "tlScroobyGroupChunk",
        chunk_id_expr: "Pure3D::Scrooby::GROUP",
        subchunks: SUBCHUNKS_227,
        fields: FIELDS_227,
    },
    ChunkDefinition {
        schema_key: "schema_0065",
        name: "tlScroobyPure3dObjectChunk",
        chunk_id_expr: "Pure3D::Scrooby::P3DOBJECT",
        subchunks: SUBCHUNKS_228,
        fields: FIELDS_228,
    },
    ChunkDefinition {
        schema_key: "schema_0065",
        name: "tlScroobyTextBibleChunk",
        chunk_id_expr: "Pure3D::Scrooby::TEXTBIBLE",
        subchunks: SUBCHUNKS_229,
        fields: FIELDS_229,
    },
    ChunkDefinition {
        schema_key: "schema_0065",
        name: "tlScroobyLanguageChunk",
        chunk_id_expr: "Pure3D::Scrooby::LANGUAGE",
        subchunks: SUBCHUNKS_230,
        fields: FIELDS_230,
    },
    ChunkDefinition {
        schema_key: "schema_0065",
        name: "tlScroobyImageResourceChunk",
        chunk_id_expr: "Pure3D::Scrooby::RESOURCEIMAGE",
        subchunks: SUBCHUNKS_231,
        fields: FIELDS_231,
    },
    ChunkDefinition {
        schema_key: "schema_0065",
        name: "tlScroobyPure3dResourceChunk",
        chunk_id_expr: "Pure3D::Scrooby::RESOURCEPURE3D",
        subchunks: SUBCHUNKS_232,
        fields: FIELDS_232,
    },
    ChunkDefinition {
        schema_key: "schema_0065",
        name: "tlScroobyTextStyleResourceChunk",
        chunk_id_expr: "Pure3D::Scrooby::RESOURCETEXTSTYLE",
        subchunks: SUBCHUNKS_233,
        fields: FIELDS_233,
    },
    ChunkDefinition {
        schema_key: "schema_0065",
        name: "tlScroobyTextBibleResourceChunk",
        chunk_id_expr: "Pure3D::Scrooby::RESOURCETEXTBIBLE",
        subchunks: SUBCHUNKS_234,
        fields: FIELDS_234,
    },
    ChunkDefinition {
        schema_key: "schema_0065",
        name: "tlScroobyTextStyleResourceChunk16",
        chunk_id_expr: "Pure3D::Scrooby::OLDRESOURCETEXTSTYLE",
        subchunks: SUBCHUNKS_235,
        fields: FIELDS_235,
    },
    ChunkDefinition {
        schema_key: "schema_0065",
        name: "tlScroobyTextBibleResourceChunk16",
        chunk_id_expr: "Pure3D::Scrooby::OLDRESOURCETEXTBIBLE",
        subchunks: SUBCHUNKS_236,
        fields: FIELDS_236,
    },
    ChunkDefinition {
        schema_key: "schema_0066",
        name: "tlSetChunk",
        chunk_id_expr: "SRR2::ChunkID::CHUNK_SET",
        subchunks: SUBCHUNKS_237,
        fields: FIELDS_237,
    },
    ChunkDefinition {
        schema_key: "schema_0067",
        name: "shader",
        chunk_id_expr: "Pure3D::Shader::SHADER",
        subchunks: SUBCHUNKS_238,
        fields: FIELDS_238,
    },
    ChunkDefinition {
        schema_key: "schema_0067",
        name: "tlShaderDefinitionChunk",
        chunk_id_expr: "Pure3D::Shader::SHADER_DEFINITION",
        subchunks: SUBCHUNKS_239,
        fields: FIELDS_239,
    },
    ChunkDefinition {
        schema_key: "schema_0067",
        name: "tlShaderTextureParamChunk",
        chunk_id_expr: "Pure3D::Shader::TEXTURE_PARAM",
        subchunks: SUBCHUNKS_240,
        fields: FIELDS_240,
    },
    ChunkDefinition {
        schema_key: "schema_0067",
        name: "tlShaderIntParamChunk",
        chunk_id_expr: "Pure3D::Shader::INT_PARAM",
        subchunks: SUBCHUNKS_241,
        fields: FIELDS_241,
    },
    ChunkDefinition {
        schema_key: "schema_0067",
        name: "tlShaderFloatParamChunk",
        chunk_id_expr: "Pure3D::Shader::FLOAT_PARAM",
        subchunks: SUBCHUNKS_242,
        fields: FIELDS_242,
    },
    ChunkDefinition {
        schema_key: "schema_0067",
        name: "tlShaderColourParamChunk",
        chunk_id_expr: "Pure3D::Shader::COLOUR_PARAM",
        subchunks: SUBCHUNKS_243,
        fields: FIELDS_243,
    },
    ChunkDefinition {
        schema_key: "schema_0067",
        name: "tlShaderVectorParamChunk",
        chunk_id_expr: "Pure3D::Shader::VECTOR_PARAM",
        subchunks: SUBCHUNKS_244,
        fields: FIELDS_244,
    },
    ChunkDefinition {
        schema_key: "schema_0067",
        name: "tlShaderMatrixParamChunk",
        chunk_id_expr: "Pure3D::Shader::MATRIX_PARAM",
        subchunks: SUBCHUNKS_245,
        fields: FIELDS_245,
    },
    ChunkDefinition {
        schema_key: "schema_0068",
        name: "tlShadowSkinChunk",
        chunk_id_expr: "Pure3D::Mesh::SHADOWSKIN",
        subchunks: SUBCHUNKS_246,
        fields: FIELDS_246,
    },
    ChunkDefinition {
        schema_key: "schema_0068",
        name: "tlTopologyChunk",
        chunk_id_expr: "Pure3D::Mesh::TOPOLOGY",
        subchunks: SUBCHUNKS_247,
        fields: FIELDS_247,
    },
    ChunkDefinition {
        schema_key: "schema_0068",
        name: "tlShadowMeshChunk",
        chunk_id_expr: "Pure3D::Mesh::SHADOWMESH",
        subchunks: SUBCHUNKS_248,
        fields: FIELDS_248,
    },
    ChunkDefinition {
        schema_key: "schema_0069",
        name: "tlSkeletonChunk16",
        chunk_id_expr: "P3D_SKELETON",
        subchunks: SUBCHUNKS_249,
        fields: FIELDS_249,
    },
    ChunkDefinition {
        schema_key: "schema_0069",
        name: "tlSkeletonJointChunk16",
        chunk_id_expr: "P3D_SKELETON_JOINT",
        subchunks: SUBCHUNKS_250,
        fields: FIELDS_250,
    },
    ChunkDefinition {
        schema_key: "schema_0069",
        name: "tlSkeletonJointMirrorMapChunk16",
        chunk_id_expr: "P3D_SKELETON_JOINT_MIRROR_MAP",
        subchunks: SUBCHUNKS_251,
        fields: FIELDS_251,
    },
    ChunkDefinition {
        schema_key: "schema_0069",
        name: "tlSkeletonJointBonePreserveChunk16",
        chunk_id_expr: "P3D_SKELETON_JOINT_FIX_FLAG",
        subchunks: SUBCHUNKS_252,
        fields: FIELDS_252,
    },
    ChunkDefinition {
        schema_key: "schema_0070",
        name: "skin",
        chunk_id_expr: "Pure3D::Mesh::SKIN",
        subchunks: SUBCHUNKS_253,
        fields: FIELDS_253,
    },
    ChunkDefinition {
        schema_key: "schema_0071",
        name: "tlSmartPropChunk",
        chunk_id_expr: "SmartProp::SMARTPROP",
        subchunks: SUBCHUNKS_254,
        fields: FIELDS_254,
    },
    ChunkDefinition {
        schema_key: "schema_0071",
        name: "tlSmartPropStateDataChunk",
        chunk_id_expr: "SmartProp::SMARTPROPSTATEDATA",
        subchunks: SUBCHUNKS_255,
        fields: FIELDS_255,
    },
    ChunkDefinition {
        schema_key: "schema_0071",
        name: "tlSmartPropVisibilitiesDataChunk",
        chunk_id_expr: "SmartProp::SMARTPROPVISIBILITYDATA",
        subchunks: SUBCHUNKS_256,
        fields: FIELDS_256,
    },
    ChunkDefinition {
        schema_key: "schema_0071",
        name: "tlSmartPropFrameControllerDataChunk",
        chunk_id_expr: "SmartProp::SMARTPROPFRAMECONTROLLERDATA",
        subchunks: SUBCHUNKS_257,
        fields: FIELDS_257,
    },
    ChunkDefinition {
        schema_key: "schema_0071",
        name: "tlSmartPropEventDataChunk",
        chunk_id_expr: "SmartProp::SMARTPROPEVENTDATA",
        subchunks: SUBCHUNKS_258,
        fields: FIELDS_258,
    },
    ChunkDefinition {
        schema_key: "schema_0071",
        name: "tlSmartPropCallbackDataChunk",
        chunk_id_expr: "SmartProp::SMARTPROPCALLBACKDATA",
        subchunks: SUBCHUNKS_259,
        fields: FIELDS_259,
    },
    ChunkDefinition {
        schema_key: "schema_0071",
        name: "tlSmartPropAppliedForceDataChunk",
        chunk_id_expr: "SmartProp::SMARTPROPAPPLIEDFORCE",
        subchunks: SUBCHUNKS_260,
        fields: FIELDS_260,
    },
    ChunkDefinition {
        schema_key: "schema_0071",
        name: "tlSmartPropBreakableDataChunk",
        chunk_id_expr: "SmartProp::SMARTPROPBREAKABLE",
        subchunks: SUBCHUNKS_261,
        fields: FIELDS_261,
    },
    ChunkDefinition {
        schema_key: "schema_0071",
        name: "tlSmartPropExtraAttributeDataChunk",
        chunk_id_expr: "SmartProp::SMARTPROPEXTRAATTRIBUTE",
        subchunks: SUBCHUNKS_262,
        fields: FIELDS_262,
    },
    ChunkDefinition {
        schema_key: "schema_0072",
        name: "tlSpriteChunk",
        chunk_id_expr: "Pure3D::Texture::SPRITE",
        subchunks: SUBCHUNKS_263,
        fields: FIELDS_263,
    },
    ChunkDefinition {
        schema_key: "schema_0073",
        name: "tlStatePropChunk",
        chunk_id_expr: "StateProp::STATEPROP",
        subchunks: SUBCHUNKS_264,
        fields: FIELDS_264,
    },
    ChunkDefinition {
        schema_key: "schema_0073",
        name: "tlStatePropStateDataChunk",
        chunk_id_expr: "StateProp::STATEPROPSTATEDATA",
        subchunks: SUBCHUNKS_265,
        fields: FIELDS_265,
    },
    ChunkDefinition {
        schema_key: "schema_0073",
        name: "tlStatePropVisibilitiesDataChunk",
        chunk_id_expr: "StateProp::STATEPROPVISIBILITYDATA",
        subchunks: SUBCHUNKS_266,
        fields: FIELDS_266,
    },
    ChunkDefinition {
        schema_key: "schema_0073",
        name: "tlStatePropFrameControllerDataChunk",
        chunk_id_expr: "StateProp::STATEPROPFRAMECONTROLLERDATA",
        subchunks: SUBCHUNKS_267,
        fields: FIELDS_267,
    },
    ChunkDefinition {
        schema_key: "schema_0073",
        name: "tlStatePropEventDataChunk",
        chunk_id_expr: "StateProp::STATEPROPEVENTDATA",
        subchunks: SUBCHUNKS_268,
        fields: FIELDS_268,
    },
    ChunkDefinition {
        schema_key: "schema_0073",
        name: "tlStatePropCallbackDataChunk",
        chunk_id_expr: "StateProp::STATEPROPCALLBACKDATA",
        subchunks: SUBCHUNKS_269,
        fields: FIELDS_269,
    },
    ChunkDefinition {
        schema_key: "schema_0074",
        name: "static_phys_dsg",
        chunk_id_expr: "SRR2::ChunkID::STATIC_PHYS_DSG",
        subchunks: SUBCHUNKS_270,
        fields: FIELDS_270,
    },
    ChunkDefinition {
        schema_key: "schema_0075",
        name: "tlTerrainTypeChunk",
        chunk_id_expr: "SRR2::ChunkID::TERRAIN_TYPE",
        subchunks: SUBCHUNKS_271,
        fields: FIELDS_271,
    },
    ChunkDefinition {
        schema_key: "schema_0076",
        name: "tlTextureAnimChunk16",
        chunk_id_expr: "P3D_TEXTURE_ANIM",
        subchunks: SUBCHUNKS_272,
        fields: FIELDS_272,
    },
    ChunkDefinition {
        schema_key: "schema_0076",
        name: "tlTextureAnimChannelChunk16",
        chunk_id_expr: "P3D_TEXTURE_ANIM_CHANNEL",
        subchunks: SUBCHUNKS_273,
        fields: FIELDS_273,
    },
    ChunkDefinition {
        schema_key: "schema_0077",
        name: "texture",
        chunk_id_expr: "Pure3D::Texture::TEXTURE",
        subchunks: SUBCHUNKS_274,
        fields: FIELDS_274,
    },
    ChunkDefinition {
        schema_key: "schema_0078",
        name: "tree_dsg",
        chunk_id_expr: "SRR2::ChunkID::TREE_DSG",
        subchunks: SUBCHUNKS_275,
        fields: FIELDS_275,
    },
    ChunkDefinition {
        schema_key: "schema_0078",
        name: "tlContiguousBinNodeChunk",
        chunk_id_expr: "SRR2::ChunkID::CONTIGUOUS_BIN_NODE",
        subchunks: SUBCHUNKS_276,
        fields: FIELDS_276,
    },
    ChunkDefinition {
        schema_key: "schema_0078",
        name: "tlSpatialNodeChunk",
        chunk_id_expr: "SRR2::ChunkID::SPATIAL_NODE",
        subchunks: SUBCHUNKS_277,
        fields: FIELDS_277,
    },
    ChunkDefinition {
        schema_key: "schema_0079",
        name: "tlVertexAnimKeyChunk",
        chunk_id_expr: "Pure3D::Animation::VertexAnim::KEY",
        subchunks: SUBCHUNKS_278,
        fields: FIELDS_278,
    },
    ChunkDefinition {
        schema_key: "schema_0079",
        name: "tlColourOffsetListChunk",
        chunk_id_expr: "Pure3D::Animation::VertexAnim::OffsetList::COLOUR",
        subchunks: SUBCHUNKS_279,
        fields: FIELDS_279,
    },
    ChunkDefinition {
        schema_key: "schema_0079",
        name: "tlVectorOffsetListChunk",
        chunk_id_expr: "Pure3D::Animation::VertexAnim::OffsetList::VECTOR",
        subchunks: SUBCHUNKS_280,
        fields: FIELDS_280,
    },
    ChunkDefinition {
        schema_key: "schema_0079",
        name: "tlVector2OffsetListChunk",
        chunk_id_expr: "Pure3D::Animation::VertexAnim::OffsetList::VECTOR2",
        subchunks: SUBCHUNKS_281,
        fields: FIELDS_281,
    },
    ChunkDefinition {
        schema_key: "schema_0079",
        name: "tlOffsetIndexListChunk",
        chunk_id_expr: "Pure3D::Animation::VertexAnim::OffsetList::INDEX",
        subchunks: SUBCHUNKS_282,
        fields: FIELDS_282,
    },
    ChunkDefinition {
        schema_key: "schema_0080",
        name: "tlVertexOffsetExpressionChunk16",
        chunk_id_expr: "P3D_VERTEX_OFFSET_EXPRESSION",
        subchunks: SUBCHUNKS_283,
        fields: FIELDS_283,
    },
    ChunkDefinition {
        schema_key: "schema_0081",
        name: "tlVisibilityAnimChunk16",
        chunk_id_expr: "P3D_VISIBILITY_ANIM",
        subchunks: SUBCHUNKS_284,
        fields: FIELDS_284,
    },
    ChunkDefinition {
        schema_key: "schema_0081",
        name: "tlVisibilityAnimChannelChunk16",
        chunk_id_expr: "P3D_VISIBILITY_ANIM_CHANNEL",
        subchunks: SUBCHUNKS_285,
        fields: FIELDS_285,
    },
    ChunkDefinition {
        schema_key: "schema_0082",
        name: "tlWalkerCamDataChunk",
        chunk_id_expr: "SRR2::ChunkID::WALKERCAM",
        subchunks: SUBCHUNKS_286,
        fields: FIELDS_286,
    },
    ChunkDefinition {
        schema_key: "schema_0083",
        name: "tlWallChunk",
        chunk_id_expr: "SRR2::ChunkID::WALL",
        subchunks: SUBCHUNKS_287,
        fields: FIELDS_287,
    },
    ChunkDefinition {
        schema_key: "schema_0084",
        name: "tlWBLocatorChunk",
        chunk_id_expr: "SRR2::ChunkID::LOCATOR",
        subchunks: SUBCHUNKS_288,
        fields: FIELDS_288,
    },
    ChunkDefinition {
        schema_key: "schema_0085",
        name: "tlWBRailCamChunk",
        chunk_id_expr: "SRR2::ChunkID::RAIL",
        subchunks: SUBCHUNKS_289,
        fields: FIELDS_289,
    },
    ChunkDefinition {
        schema_key: "schema_0086",
        name: "tlWBSplineChunk",
        chunk_id_expr: "SRR2::ChunkID::SPLINE",
        subchunks: SUBCHUNKS_290,
        fields: FIELDS_290,
    },
    ChunkDefinition {
        schema_key: "schema_0087",
        name: "tlWBTriggerVolumeChunk",
        chunk_id_expr: "SRR2::ChunkID::TRIGGER_VOLUME",
        subchunks: SUBCHUNKS_291,
        fields: FIELDS_291,
    },
    ChunkDefinition {
        schema_key: "schema_0088",
        name: "world_sphere_dsg",
        chunk_id_expr: "SRR2::ChunkID::WORLD_SPHERE_DSG",
        subchunks: SUBCHUNKS_292,
        fields: FIELDS_292,
    },
];

/// Chunk id constants.
pub const CHUNK_ID_CONSTANTS: &[ChunkIdConstant] = &[
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D",
        name: "DATA_FILE",
        value: 0xff44_3350,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D",
        name: "DATA_FILE_SWAP",
        value: 0x5033_44ff,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D",
        name: "DATA_FILE_COMPRESSED",
        value: 0x5a44_3350,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D",
        name: "DATA_FILE_COMPRESSED_SWAP",
        value: 0x5033_445a,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Mesh",
        name: "MESH",
        value: 0x0001_0000,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Mesh",
        name: "SKIN",
        value: 0x0001_0001,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Mesh",
        name: "PRIMGROUP",
        value: 0x0001_0002,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Mesh",
        name: "BOX",
        value: 0x0001_0003,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Mesh",
        name: "SPHERE",
        value: 0x0001_0004,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Mesh",
        name: "POSITIONLIST",
        value: 0x0001_0005,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Mesh",
        name: "NORMALLIST",
        value: 0x0001_0006,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Mesh",
        name: "UVLIST",
        value: 0x0001_0007,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Mesh",
        name: "COLOURLIST",
        value: 0x0001_0008,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Mesh",
        name: "STRIPLIST",
        value: 0x0001_0009,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Mesh",
        name: "INDEXLIST",
        value: 0x0001_000a,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Mesh",
        name: "MATRIXLIST",
        value: 0x0001_000b,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Mesh",
        name: "WEIGHTLIST",
        value: 0x0001_000c,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Mesh",
        name: "MATRIXPALETTE",
        value: 0x0001_000d,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Mesh",
        name: "OFFSETLIST",
        value: 0x0001_000e,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Mesh",
        name: "INSTANCEINFO",
        value: 0x0001_000f,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Mesh",
        name: "PACKEDNORMALLIST",
        value: 0x0001_0010,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Mesh",
        name: "VERTEXSHADER",
        value: 0x0001_0011,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Mesh",
        name: "MEMORYIMAGEVERTEXLIST",
        value: 0x0001_0012,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Mesh",
        name: "MEMORYIMAGEINDEXLIST",
        value: 0x0001_0013,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Mesh",
        name: "MEMORYIMAGEVERTEXDESCRIPTIONLIST",
        value: 0x0001_0014,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Mesh",
        name: "TANGENTLIST",
        value: 0x0001_0015,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Mesh",
        name: "BINORMALLIST",
        value: 0x0001_0016,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Mesh",
        name: "RENDERSTATUS",
        value: 0x0001_0017,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Mesh",
        name: "EXPRESSIONOFFSETS",
        value: 0x0001_0018,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Mesh",
        name: "SHADOWSKIN",
        value: 0x0001_0019,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Mesh",
        name: "SHADOWMESH",
        value: 0x0001_001a,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Mesh",
        name: "TOPOLOGY",
        value: 0x0001_001b,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Mesh",
        name: "MULTICOLOURLIST",
        value: 0x0001_001c,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Shader",
        name: "SHADER",
        value: 0x0001_1000,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Shader",
        name: "SHADER_DEFINITION",
        value: 0x0001_1001,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Shader",
        name: "TEXTURE_PARAM",
        value: 0x0001_1002,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Shader",
        name: "INT_PARAM",
        value: 0x0001_1003,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Shader",
        name: "FLOAT_PARAM",
        value: 0x0001_1004,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Shader",
        name: "COLOUR_PARAM",
        value: 0x0001_1005,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Shader",
        name: "VECTOR_PARAM",
        value: 0x0001_1006,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Shader",
        name: "MATRIX_PARAM",
        value: 0x0001_1007,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::GameAttr",
        name: "GAME_ATTR",
        value: 0x0001_2000,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::GameAttr",
        name: "INT_PARAM",
        value: 0x0001_2001,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::GameAttr",
        name: "FLOAT_PARAM",
        value: 0x0001_2002,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::GameAttr",
        name: "COLOUR_PARAM",
        value: 0x0001_2003,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::GameAttr",
        name: "VECTOR_PARAM",
        value: 0x0001_2004,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::GameAttr",
        name: "MATRIX_PARAM",
        value: 0x0001_2005,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Light",
        name: "LIGHT",
        value: 0x0001_3000,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Light",
        name: "DIRECTION",
        value: 0x0001_3001,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Light",
        name: "POSITION",
        value: 0x0001_3002,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Light",
        name: "CONE_PARAM",
        value: 0x0001_3003,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Light",
        name: "SHADOW",
        value: 0x0001_3004,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Light",
        name: "PHOTON_MAP",
        value: 0x0001_3005,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Light",
        name: "DECAY_RANGE",
        value: 0x0001_3006,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Light",
        name: "DECAY_RANGE_ROTATION_Y",
        value: 0x0001_3007,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Light",
        name: "ILLUMINATION_TYPE",
        value: 0x0001_3008,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Locator",
        name: "LOCATOR",
        value: 0x0001_4000,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::ParticleSystem",
        name: "SYSTEM_FACTORY",
        value: 0x0001_5800,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::ParticleSystem",
        name: "SYSTEM",
        value: 0x0001_5801,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::ParticleSystem",
        name: "BASE_PARTICLE_ARRAY",
        value: 0x0001_5802,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::ParticleSystem",
        name: "SPRITE_PARTICLE_ARRAY",
        value: 0x0001_5803,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::ParticleSystem",
        name: "DRAWABLE_PARTICLE_ARRAY",
        value: 0x0001_5804,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::ParticleSystem",
        name: "BASE_EMITTER_FACTORY",
        value: 0x0001_5805,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::ParticleSystem",
        name: "SPRITE_EMITTER_FACTORY",
        value: 0x0001_5806,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::ParticleSystem",
        name: "DRAWABLE_EMITTER_FACTORY",
        value: 0x0001_5807,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::ParticleSystem",
        name: "PARTICLE_ANIMATION",
        value: 0x0001_5808,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::ParticleSystem",
        name: "EMITTER_ANIMATION",
        value: 0x0001_5809,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::ParticleSystem",
        name: "GENERATOR_ANIMATION",
        value: 0x0001_580a,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::ParticleSystem",
        name: "INSTANCING_INFO",
        value: 0x0001_580b,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::OpticEffect",
        name: "CORONA_V14",
        value: 0x0001_6000,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::OpticEffect",
        name: "LENS_FLARE_PARENT_V14",
        value: 0x0001_6001,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::OpticEffect",
        name: "LENS_FLARE_V14",
        value: 0x0001_6002,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::OpticEffect",
        name: "VECTOR_V14",
        value: 0x0001_6f00,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::OpticEffect",
        name: "LENS_FLARE_GROUP",
        value: 0x0001_6006,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::OpticEffect",
        name: "LENS_FLARE",
        value: 0x0001_6007,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::BillboardObject",
        name: "QUAD_V14",
        value: 0x0001_7000,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::BillboardObject",
        name: "QUAD",
        value: 0x0001_7001,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::BillboardObject",
        name: "QUAD_GROUP",
        value: 0x0001_7002,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::BillboardObject",
        name: "DISPLAY_INFO",
        value: 0x0001_7003,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::BillboardObject",
        name: "PERSPECTIVE_INFO",
        value: 0x0001_7004,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Scrooby",
        name: "PROJECT",
        value: 0x0001_8000,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Scrooby",
        name: "SCREEN",
        value: 0x0001_8001,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Scrooby",
        name: "PAGE",
        value: 0x0001_8002,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Scrooby",
        name: "LAYER",
        value: 0x0001_8003,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Scrooby",
        name: "GROUP",
        value: 0x0001_8004,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Scrooby",
        name: "MOVIE",
        value: 0x0001_8005,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Scrooby",
        name: "MULTISPRITE",
        value: 0x0001_8006,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Scrooby",
        name: "MULTITEXT",
        value: 0x0001_8007,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Scrooby",
        name: "P3DOBJECT",
        value: 0x0001_8008,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Scrooby",
        name: "POLYGON",
        value: 0x0001_8009,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Scrooby",
        name: "SPRITE",
        value: 0x0001_800a,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Scrooby",
        name: "STRINGTEXTBIBLE",
        value: 0x0001_800b,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Scrooby",
        name: "STRINGHARDCODED",
        value: 0x0001_800c,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Scrooby",
        name: "TEXTBIBLE",
        value: 0x0001_800d,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Scrooby",
        name: "LANGUAGE",
        value: 0x0001_800e,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Scrooby",
        name: "RESOURCEIMAGE",
        value: 0x0001_8100,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Scrooby",
        name: "RESOURCEPURE3D",
        value: 0x0001_8101,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Scrooby",
        name: "OLDRESOURCETEXTSTYLE",
        value: 0x0001_8102,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Scrooby",
        name: "OLDRESOURCETEXTBIBLE",
        value: 0x0001_8103,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Scrooby",
        name: "RESOURCETEXTSTYLE",
        value: 0x0001_8104,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Scrooby",
        name: "RESOURCETEXTBIBLE",
        value: 0x0001_8105,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Texture",
        name: "TEXTURE",
        value: 0x0001_9000,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Texture",
        name: "IMAGE",
        value: 0x0001_9001,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Texture",
        name: "IMAGE_DATA",
        value: 0x0001_9002,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Texture",
        name: "IMAGE_FILENAME",
        value: 0x0001_9003,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Texture",
        name: "VOLUME_IMAGE",
        value: 0x0001_9004,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Texture",
        name: "SPRITE",
        value: 0x0001_9005,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::AnimatedObject",
        name: "FACTORY",
        value: 0x0002_0000,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::AnimatedObject",
        name: "OBJECT",
        value: 0x0002_0001,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::AnimatedObject",
        name: "ANIMATION",
        value: 0x0002_0002,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Expression",
        name: "VERTEX_EXPRESSION",
        value: 0x0002_1000,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Expression",
        name: "VERTEX_EXPRESSION_GROUP",
        value: 0x0002_1001,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Expression",
        name: "VERTEX_EXPRESSION_MIXER",
        value: 0x0002_1002,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Font",
        name: "TEXTURE_FONT",
        value: 0x0002_2000,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Font",
        name: "TEXTURE_GLYPH_LIST",
        value: 0x0002_2001,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Font",
        name: "IMAGE_FONT",
        value: 0x0002_2002,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Font",
        name: "IMAGE_GLYPH_LIST",
        value: 0x0002_2003,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::SceneGraph",
        name: "SCENEGRAPH",
        value: 0x0012_0100,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::SceneGraph",
        name: "ROOT",
        value: 0x0012_0101,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::SceneGraph",
        name: "BRANCH",
        value: 0x0012_0102,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::SceneGraph",
        name: "TRANSFORM",
        value: 0x0012_0103,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::SceneGraph",
        name: "VISIBILITY",
        value: 0x0012_0104,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::SceneGraph",
        name: "ATTACHMENT",
        value: 0x0012_0105,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::SceneGraph",
        name: "ATTACHMENTPOINT",
        value: 0x0012_0106,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::SceneGraph",
        name: "DRAWABLE",
        value: 0x0012_0107,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::SceneGraph",
        name: "CAMERA",
        value: 0x0012_0108,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::SceneGraph",
        name: "LIGHTGROUP",
        value: 0x0012_0109,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::SceneGraph",
        name: "SORTORDER",
        value: 0x0012_010a,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Animation::AnimationData",
        name: "ANIMATION",
        value: 0x0012_1000,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Animation::AnimationData",
        name: "GROUP",
        value: 0x0012_1001,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Animation::AnimationData",
        name: "GROUP_LIST",
        value: 0x0012_1002,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Animation::AnimationData",
        name: "SIZE",
        value: 0x0012_1004,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Animation::ChannelData",
        name: "FLOAT_1",
        value: 0x0012_1100,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Animation::ChannelData",
        name: "FLOAT_2",
        value: 0x0012_1101,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Animation::ChannelData",
        name: "VECTOR_1DOF",
        value: 0x0012_1102,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Animation::ChannelData",
        name: "VECTOR_2DOF",
        value: 0x0012_1103,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Animation::ChannelData",
        name: "VECTOR_3DOF",
        value: 0x0012_1104,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Animation::ChannelData",
        name: "QUATERNION",
        value: 0x0012_1105,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Animation::ChannelData",
        name: "STRING",
        value: 0x0012_1106,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Animation::ChannelData",
        name: "ENTITY",
        value: 0x0012_1107,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Animation::ChannelData",
        name: "BOOL",
        value: 0x0012_1108,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Animation::ChannelData",
        name: "COLOUR",
        value: 0x0012_1109,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Animation::ChannelData",
        name: "EVENT",
        value: 0x0012_110a,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Animation::ChannelData",
        name: "EVENT_OBJECT",
        value: 0x0012_110b,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Animation::ChannelData",
        name: "EVENT_OBJECT_DATA",
        value: 0x0012_110c,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Animation::ChannelData",
        name: "EVENT_OBJECT_DATA_IMAGE",
        value: 0x0012_110d,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Animation::ChannelData",
        name: "INT",
        value: 0x0012_110e,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Animation::ChannelData",
        name: "QUATERNION_FORMAT",
        value: 0x0012_110f,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Animation::ChannelData",
        name: "INTERPOLATION_MODE",
        value: 0x0012_1110,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Animation::ChannelData",
        name: "COMPRESSED_QUATERNION",
        value: 0x0012_1111,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Animation::FrameControllerData",
        name: "FRAME_CONTROLLER",
        value: 0x0012_1200,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Animation::VertexAnim::OffsetList",
        name: "COLOUR",
        value: 0x0012_1300,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Animation::VertexAnim::OffsetList",
        name: "VECTOR",
        value: 0x0012_1301,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Animation::VertexAnim::OffsetList",
        name: "VECTOR2",
        value: 0x0012_1302,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Animation::VertexAnim::OffsetList",
        name: "INDEX",
        value: 0x0012_1303,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Animation::VertexAnim",
        name: "KEY",
        value: 0x0012_1304,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Animation::VertexSplineAnim::List",
        name: "VECTOR",
        value: 0x0012_1400,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Animation::VertexSplineAnim::List",
        name: "VECTOR2",
        value: 0x0012_1401,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D::Animation::VertexSplineAnim",
        name: "KEY",
        value: 0x0012_1402,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Simulation::Collision",
        name: "OBJECT",
        value: 0x0701_0000,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Simulation::Collision",
        name: "VOLUME",
        value: 0x0701_0001,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Simulation::Collision",
        name: "SPHERE",
        value: 0x0701_0002,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Simulation::Collision",
        name: "CYLINDER",
        value: 0x0701_0003,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Simulation::Collision",
        name: "OBBOX",
        value: 0x0701_0004,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Simulation::Collision",
        name: "WALL",
        value: 0x0701_0005,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Simulation::Collision",
        name: "BBOX",
        value: 0x0701_0006,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Simulation::Collision",
        name: "VECTOR",
        value: 0x0701_0007,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Simulation::Collision",
        name: "SELFCOLLISION",
        value: 0x0701_0020,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Simulation::Collision",
        name: "OWNER",
        value: 0x0701_0021,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Simulation::Collision",
        name: "OWNERNAME",
        value: 0x0701_0022,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Simulation::Collision",
        name: "ATTRIBUTE",
        value: 0x0701_0023,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Simulation::Physics",
        name: "OBJECT",
        value: 0x0701_1000,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Simulation::Physics",
        name: "IMAT",
        value: 0x0701_1001,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Simulation::Physics",
        name: "VECTOR",
        value: 0x0701_1002,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Simulation::Physics",
        name: "JOINT",
        value: 0x0701_1020,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Simulation::Physics",
        name: "JOINT_DOF",
        value: 0x0701_1021,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Simulation::Flexible",
        name: "OBJECT",
        value: 0x0701_2000,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Simulation::Flexible",
        name: "OBJECT_PARAMETERS",
        value: 0x0701_2001,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Simulation::Flexible",
        name: "FIX_PARTICLE",
        value: 0x0701_2002,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Simulation::Flexible",
        name: "MAP_VL",
        value: 0x0701_2003,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Simulation::Flexible",
        name: "TRI_MAP",
        value: 0x0701_2004,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Simulation::Flexible",
        name: "EDGE_MAP",
        value: 0x0701_2005,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Simulation::Flexible",
        name: "EDGE_LEN",
        value: 0x0701_2006,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Simulation::Flexible",
        name: "OBJECT_LAMBDA",
        value: 0x0701_2007,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Simulation::Flexible",
        name: "JOINT",
        value: 0x0701_2020,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Simulation::Flexible",
        name: "JOINT_PARAMETERS",
        value: 0x0701_2021,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Simulation::Flexible",
        name: "JOINT_DEFINITION",
        value: 0x0701_2022,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Simulation::Flexible",
        name: "JOINT_LAMBDA",
        value: 0x0701_2023,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Simulation::Link",
        name: "IK",
        value: 0x0701_4000,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Simulation::Link",
        name: "REACH",
        value: 0x0701_4001,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Simulation::Link",
        name: "TRACKER",
        value: 0x0701_4002,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Simulation::Link",
        name: "TARGET",
        value: 0x0701_4003,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Simulation::Link",
        name: "TARGET_NODE",
        value: 0x0701_4004,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Simulation::Link",
        name: "TARGET_POSE",
        value: 0x0701_4005,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Simulation::Parameters",
        name: "ENVIRONMENT",
        value: 0x0701_5000,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Simulation::Parameters",
        name: "PHYSICS",
        value: 0x0701_5001,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "SmartProp",
        name: "SMARTPROP",
        value: 0x0801_0000,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "SmartProp",
        name: "SMARTPROPSTATEDATA",
        value: 0x0801_0001,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "SmartProp",
        name: "SMARTPROPVISIBILITYDATA",
        value: 0x0801_0002,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "SmartProp",
        name: "SMARTPROPFRAMECONTROLLERDATA",
        value: 0x0801_0003,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "SmartProp",
        name: "SMARTPROPEVENTDATA",
        value: 0x0801_0004,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "SmartProp",
        name: "SMARTPROPCALLBACKDATA",
        value: 0x0801_0005,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "SmartProp",
        name: "SMARTPROPAPPLIEDFORCE",
        value: 0x0801_0006,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "SmartProp",
        name: "SMARTPROPBREAKABLE",
        value: 0x0801_0007,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "SmartProp",
        name: "SMARTPROPEXTRAATTRIBUTE",
        value: 0x0801_0008,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "StateProp",
        name: "STATEPROP",
        value: 0x0802_0000,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "StateProp",
        name: "STATEPROPSTATEDATA",
        value: 0x0802_0001,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "StateProp",
        name: "STATEPROPVISIBILITYDATA",
        value: 0x0802_0002,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "StateProp",
        name: "STATEPROPFRAMECONTROLLERDATA",
        value: 0x0802_0003,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "StateProp",
        name: "STATEPROPEVENTDATA",
        value: 0x0802_0004,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "StateProp",
        name: "STATEPROPCALLBACKDATA",
        value: 0x0802_0005,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "MemorySection",
        name: "MEMORYSECTION",
        value: 0xffff_0000,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D",
        name: "PARTICLE_SYSTEM",
        value: 0x0010_0000,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D",
        name: "EMITTER",
        value: 0x0010_0001,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D",
        name: "SIMPLE_EMITTER",
        value: 0x0010_0002,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D",
        name: "EMITTER_LIFE_CHANNEL",
        value: 0x0010_0003,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D",
        name: "EMITTER_SPEED_CHANNEL",
        value: 0x0010_0004,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D",
        name: "EMITTER_WEIGHT_CHANNEL",
        value: 0x0010_0005,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D",
        name: "EMITTER_LIFE_VAR_CHANNEL",
        value: 0x0010_0006,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D",
        name: "EMITTER_SPEED_VAR_CHANNEL",
        value: 0x0010_0007,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D",
        name: "EMITTER_EMISSION_RATE_CHANNEL",
        value: 0x0010_0008,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D",
        name: "S_EMITTER_SIZE_CHANNEL",
        value: 0x0010_0009,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D",
        name: "S_EMITTER_SPIN_CHANNEL",
        value: 0x0010_000a,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D",
        name: "S_EMITTER_TRANSPARENCY_CHANNEL",
        value: 0x0010_000b,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D",
        name: "S_EMITTER_COLOUR_CHANNEL",
        value: 0x0010_000c,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D",
        name: "S_EMITTER_SIZE_VAR_CHANNEL",
        value: 0x0010_000d,
    },
    ChunkIdConstant {
        authority_key: "p3d_core",
        scope: "Pure3D",
        name: "S_EMITTER_SPIN_VAR_CHANNEL",
        value: 0x0010_000e,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_IMAGE",
        value: 0x0000_3510,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_IMAGE_DATA",
        value: 0x0000_3511,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_IMAGE_FILENAME",
        value: 0x0000_3512,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_SKELETON",
        value: 0x0000_4500,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_SKELETON_JOINT",
        value: 0x0000_4501,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_SKELETON_JOINT_PHYSICS",
        value: 0x0000_4502,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_SKELETON_JOINT_MIRROR_MAP",
        value: 0x0000_4503,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_SKELETON_JOINT_FIX_FLAG",
        value: 0x0000_4504,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_COMPOSITE_DRAWABLE",
        value: 0x0000_4512,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_COMPOSITE_DRAWABLE_SKIN_LIST",
        value: 0x0000_4513,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_COMPOSITE_DRAWABLE_PROP_LIST",
        value: 0x0000_4514,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_COMPOSITE_DRAWABLE_SKIN",
        value: 0x0000_4515,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_COMPOSITE_DRAWABLE_PROP",
        value: 0x0000_4516,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_COMPOSITE_DRAWABLE_EFFECT_LIST",
        value: 0x0000_4517,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_COMPOSITE_DRAWABLE_EFFECT",
        value: 0x0000_4518,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_COMPOSITE_DRAWABLE_SORTORDER",
        value: 0x0000_4519,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_FRAME_CONTROLLER",
        value: 0x0000_4520,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_MULTI_CONTROLLER",
        value: 0x0000_48a0,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_MULTI_CONTROLLER_TRACKS",
        value: 0x0000_48a1,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_MULTI_CONTROLLER_TRACK",
        value: 0x0000_48a2,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_CAMERA",
        value: 0x0000_2200,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_LIGHT_GROUP",
        value: 0x0000_2380,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_HISTORY",
        value: 0x0000_7000,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_ALIGN",
        value: 0x0000_7001,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_EXPORT_INFO",
        value: 0x0000_7030,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_EXPORT_NAMED_STRING",
        value: 0x0000_7031,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_EXPORT_NAMED_INT",
        value: 0x0000_7032,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_EXPRESSION_PRESET",
        value: 0x0000_4a00,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_EXPRESSION_GROUP",
        value: 0x0000_4a01,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_EXPRESSION_ANIM",
        value: 0x0000_4a10,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_EXPRESSION_ANIM_CHANNEL",
        value: 0x0000_4a11,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_EXPRESSION_MIXER",
        value: 0x0000_4a20,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_VERTEXOFFSET",
        value: 0x0000_4a80,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_VERTEXOFFSET_ANIM",
        value: 0x0000_4a81,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_VERTEX_OFFSET_EXPRESSION",
        value: 0x0000_4a82,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_SG_TRANSFORM_ANIM",
        value: 0x0000_9150,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_VISIBILITY_ANIM",
        value: 0x0000_4290,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_VISIBILITY_ANIM_CHANNEL",
        value: 0x0000_4291,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_TEXTURE_ANIM",
        value: 0x0000_3520,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_TEXTURE_ANIM_CHANNEL",
        value: 0x0000_3521,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_POSE_ANIM",
        value: 0x0000_4700,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_JOINT_LIST",
        value: 0x0000_4201,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_ANIM_CHANNEL",
        value: 0x0000_4702,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_POSE_ANIM_MIRRORED",
        value: 0x0000_4703,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_CHANNEL_1DOF",
        value: 0x0000_4800,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_CHANNEL_3DOF",
        value: 0x0000_4801,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_CHANNEL_1DOF_ANGLE",
        value: 0x0000_4802,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_CHANNEL_3DOF_ANGLE",
        value: 0x0000_4803,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_CHANNEL_STATIC",
        value: 0x0000_4804,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_CHANNEL_STATIC_ANGLE",
        value: 0x0000_4805,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_CHANNEL_QUATERNION",
        value: 0x0000_4806,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_CHANNEL_STATIC_QUATERNION",
        value: 0x0000_4807,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_CAMERA_ANIM",
        value: 0x0000_4900,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_CAMERA_ANIM_CHANNEL",
        value: 0x0000_4901,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_CAMERA_ANIM_POS_CHANNEL",
        value: 0x0000_4902,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_CAMERA_ANIM_LOOK_CHANNEL",
        value: 0x0000_4903,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_CAMERA_ANIM_UP_CHANNEL",
        value: 0x0000_4904,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_CAMERA_ANIM_FOV_CHANNEL",
        value: 0x0000_4905,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_CAMERA_ANIM_NEARCLIP_CHANNEL",
        value: 0x0000_4906,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_CAMERA_ANIM_FARCLIP_CHANNEL",
        value: 0x0000_4907,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_LIGHT_ANIM",
        value: 0x0000_4980,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_LIGHT_ANIM_CHANNEL",
        value: 0x0000_4981,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_LIGHT_ANIM_COLOUR_CHANNEL",
        value: 0x0000_4982,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_LIGHT_ANIM_PARAM_CHANNEL",
        value: 0x0000_4983,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_LIGHT_ANIM_ENABLE_CHANNEL",
        value: 0x0000_4985,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_ENTITY_ANIM_CHANNEL",
        value: 0x0000_42a0,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_KEYLIST_COLOUR",
        value: 0x0000_4216,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_VERTEX_ANIM",
        value: 0x0000_4a00,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_VERTEX_ANIM_CHANNEL",
        value: 0x0000_4a01,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_DATA_FILE",
        value: 0xff44_3350,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_DATA_FILE_SWAP",
        value: 0x5033_44ff,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_16BIT_DATA_FILE",
        value: 0x0000_ff04,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "ATTR_VERTEX",
        value: 0x0000_7010,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "ATTR_POLY",
        value: 0x0000_7011,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_EXPORTER_VERSION",
        value: 0x0000_7023,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_MATRIX",
        value: 0x0000_2000,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_POS_ROT",
        value: 0x0000_2001,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_COLOR_RGB",
        value: 0x0000_2002,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_COLOR_RGBA",
        value: 0x0000_2003,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_FOV",
        value: 0x0000_2004,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_DIRECTION",
        value: 0x0000_2005,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_POSITION",
        value: 0x0000_2006,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_ROTATION_AXIS",
        value: 0x0000_2007,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_BOX",
        value: 0x0000_2008,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_SPHERE",
        value: 0x0000_2009,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_PLANE",
        value: 0x0000_200a,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_PARAMETERS",
        value: 0x0000_200b,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_SPHERE_LIST",
        value: 0x0000_200c,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_PARTICLE_SYSTEM",
        value: 0x0000_2100,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_POINT_EMITTER",
        value: 0x0000_2101,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_SPRITE_EMITTER",
        value: 0x0000_2102,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_PARTICLE_LIFE_CHANNEL",
        value: 0x0000_2110,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_PARTICLE_SPEED_CHANNEL",
        value: 0x0000_2111,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_PARTICLE_WEIGHT_CHANNEL",
        value: 0x0000_2112,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_PARTICLE_LIFE_VAR_CHANNEL",
        value: 0x0000_2113,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_PARTICLE_SPEED_VAR_CHANNEL",
        value: 0x0000_2114,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_PARTICLE_WEIGHT_VAR_CHANNEL",
        value: 0x0000_2115,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_PARTICLE_LIFE_OL_CHANNEL",
        value: 0x0000_2116,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_PARTICLE_SPEED_OL_CHANNEL",
        value: 0x0000_2117,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_PARTICLE_WEIGHT_OL_CHANNEL",
        value: 0x0000_2118,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_PARTICLE_NUM_PARTICLES_CHANNEL",
        value: 0x0000_2119,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_PARTICLE_EMISSION_RATE_CHANNEL",
        value: 0x0000_211a,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_PARTICLE_SIZE_CHANNEL",
        value: 0x0000_211b,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_PARTICLE_SPIN_CHANNEL",
        value: 0x0000_211c,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_PARTICLE_TRANSPARENCY_CHANNEL",
        value: 0x0000_211d,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_PARTICLE_COLOUR_CHANNEL",
        value: 0x0000_211e,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_PARTICLE_SIZE_VAR_CHANNEL",
        value: 0x0000_211f,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_PARTICLE_SPIN_VAR_CHANNEL",
        value: 0x0000_2120,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_PARTICLE_TRANSPARENCY_VAR_CHANNEL",
        value: 0x0000_2121,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_PARTICLE_COLOUR_VAR_CHANNEL",
        value: 0x0000_2122,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_PARTICLE_SIZE_OL_CHANNEL",
        value: 0x0000_2123,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_PARTICLE_SPIN_OL_CHANNEL",
        value: 0x0000_2124,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_PARTICLE_TRANSPARENCY_OL_CHANNEL",
        value: 0x0000_2125,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_PARTICLE_COLOUR_OL_CHANNEL",
        value: 0x0000_2126,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_PARTICLE_CHANNEL",
        value: 0x0000_2127,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_PARTICLE_POINT_GENERATOR",
        value: 0x0000_2128,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_PARTICLE_PLANE_GENERATOR",
        value: 0x0000_2129,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_PARTICLE_SPHERE_GENERATOR",
        value: 0x0000_212a,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_PARTICLE_GRAVITY_CHANNEL",
        value: 0x0000_212b,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_PARTICLE_GENERATOR_HORIZ_SPREAD",
        value: 0x0000_212e,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_PARTICLE_GENERATOR_VERT_SPREAD",
        value: 0x0000_212f,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_PARTICLE_POSITION_CHANNEL",
        value: 0x0000_2130,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_PARTICLE_ROTATION_CHANNEL",
        value: 0x0000_2131,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_FONT",
        value: 0x0000_3062,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_FONT_GLYPHS",
        value: 0x0000_3063,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_TEXTURE_FONT",
        value: 0x0000_3064,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_TEXTURE_GLYPH",
        value: 0x0000_3065,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_IMAGE_FONT",
        value: 0x0000_3066,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_IMAGE_GLYPH",
        value: 0x0000_3067,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_MESH",
        value: 0x0000_3100,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_VERTEX_LIST",
        value: 0x0000_3101,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_NORMAL_LIST",
        value: 0x0000_3102,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_UV_LIST",
        value: 0x0000_3103,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_COLOUR_LIST",
        value: 0x0000_3104,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_MATERIAL_LIST",
        value: 0x0000_3105,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_FACE_LIST",
        value: 0x0000_3106,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_PRIM_GROUP",
        value: 0x0000_3107,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_FACE_NORMAL_LIST",
        value: 0x0000_3108,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_EDGE_LIST",
        value: 0x0000_31a9,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_SKIN",
        value: 0x0000_3700,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_BONE_WEIGHTING",
        value: 0x0000_3701,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_BONE_MAPPING",
        value: 0x0000_3701,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_MATERIAL",
        value: 0x0000_3120,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_MATERIAL_PASS",
        value: 0x0000_3125,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_SHADER",
        value: 0x0000_3130,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_SHADER_DEFINITION",
        value: 0x0000_3131,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_SHADER_TEXTURE_PARAM",
        value: 0x0000_3132,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_SHADER_INT_PARAM",
        value: 0x0000_3133,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_SHADER_FLOAT_PARAM",
        value: 0x0000_3134,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_SHADER_COLOUR_PARAM",
        value: 0x0000_3135,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_SHADER_VECTOR_PARAM",
        value: 0x0000_3136,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_SHADER_MATRIX_PARAM",
        value: 0x0000_3137,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "GEO_MESH",
        value: 0x0000_3000,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "GEO_VERTEX_LIST",
        value: 0x0000_3001,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "GEO_FACE_LIST_TEX",
        value: 0x0000_3005,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "GEO_UV_LIST",
        value: 0x0000_3006,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "GEO_NORMAL_LIST",
        value: 0x0000_3007,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "GEO_MATERIAL_GROUP",
        value: 0x0000_3008,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "GEO_HIT",
        value: 0x0000_3009,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "GEO_FLAGS",
        value: 0x0000_300a,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "GEO_ANIM_VERTEX_LIST",
        value: 0x0000_300b,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "GEO_ANIM_NORMAL_LIST",
        value: 0x0000_300c,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "GEO_COLOUR_LIST",
        value: 0x0000_300d,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "GEO_VERTEX_COLOUR_LIST",
        value: 0x0000_300e,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "GEO_PRO_TEXTURE",
        value: 0x0000_3010,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "GEO_PRO_TEX_PAL",
        value: 0x0000_3011,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "GEO_PRO_TEX_PIXELS",
        value: 0x0000_3012,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "GEO_PRO_ALPHA_PIXELS",
        value: 0x0000_3013,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "GEO_PRO_MATERIAL",
        value: 0x0000_3020,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "GEO_PRO_MAT_COLOUR",
        value: 0x0000_3021,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "GEO_PRO_MAT_TEXTURE",
        value: 0x0000_3022,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "GEO_PRO_MAT_TRANSP",
        value: 0x0000_3023,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "GEO_PRO_MAT_BLENDMODE",
        value: 0x0000_3024,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_BMP_IMAGE",
        value: 0x0000_3040,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_SPRITE",
        value: 0x0000_3041,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_TRI_STRIP_MESH",
        value: 0x0000_3200,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_TRI_STRIP",
        value: 0x0000_3201,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_BACKGROUND",
        value: 0x0000_3300,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_BMP_IMAGE_REF",
        value: 0x0000_3400,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_TEXTURE",
        value: 0x0000_3500,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_HSPLINE",
        value: 0x0000_3e00,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_HS_SB_LIST",
        value: 0x0000_3e10,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_HS_STORAGE_BLOCK",
        value: 0x0000_3e11,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_HS_GN_LIST",
        value: 0x0000_3e30,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_HS_GRAFTING_NODE",
        value: 0x0000_3e31,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_HS_CONTRIB_LIST",
        value: 0x0000_3e40,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_HS_CONTRIBUTOR",
        value: 0x0000_3e41,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_HS_EDGE_LIST",
        value: 0x0000_3e50,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_HS_EDGE",
        value: 0x0000_3e51,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_HS_OFFSET_LIST",
        value: 0x0000_3e60,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_HS_OFFSET",
        value: 0x0000_3e61,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_HS_OFFSET_ADD",
        value: 0x0000_3e62,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_HS_OFFSET_TANGENT",
        value: 0x0000_3e63,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_HS_OFFSET_JOINT",
        value: 0x0000_3e64,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_HS_OFFSET_PHANTOM",
        value: 0x0000_3e65,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_HS_OFFSET_FRAME",
        value: 0x0000_3e66,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_HS_COPY_LIST",
        value: 0x0000_3e70,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_HS_COPY_CN",
        value: 0x0000_3e71,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_HS_CONTROL_NODE",
        value: 0x0000_3e81,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_HS_CCPATCH_LIST",
        value: 0x0000_3e90,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_HS_CCPATCH",
        value: 0x0000_3e91,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_HS_REF_FRAME_LIST",
        value: 0x0000_3ea0,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_HS_REF_CN",
        value: 0x0000_3ea1,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_HSTREE",
        value: 0x0000_3ef0,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_HSTREE_JOINT",
        value: 0x0000_3ef1,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_HSTREE_MAPPED_HSTREE",
        value: 0x0000_3ef2,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_HSTREE_MAPPING",
        value: 0x0000_3ef3,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_HSTREE_REST_POSE",
        value: 0x0000_3ef4,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_HSTREE_PARENT_INDEX",
        value: 0x0000_3ef5,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_HS_STITCHER",
        value: 0x0000_3f00,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_HS_STITCH",
        value: 0x0000_3f01,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_HS_STITCH_PATCH",
        value: 0x0000_3f02,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_HS_STITCH_PATCHLIST",
        value: 0x0000_3f03,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_HS_STITCH_TARGETLIST",
        value: 0x0000_3f04,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_HS_STITCH_SKIN",
        value: 0x0000_3f05,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_HS_TESSELLATION",
        value: 0x0000_3f10,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_HS_INDEX_MAPPING",
        value: 0x0000_3f11,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_HS_SKIN",
        value: 0x0000_3f20,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_HS_SKIN_OFFSET_GROUP",
        value: 0x0000_3f21,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_HS_SKIN_CONNECT",
        value: 0x0000_3f22,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_HS_SKIN_VERT_CONNECT",
        value: 0x0000_3f23,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_HS_POLYSKIN",
        value: 0x0000_3f30,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_HS_OFFSET_ANIM",
        value: 0x0000_3f40,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_HS_ANIM_CHANNEL",
        value: 0x0000_3f41,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_HS_CHANNEL_OFFSET_DYNAMIC",
        value: 0x0000_3f42,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_HS_CHANNEL_OFFSET_STATIC",
        value: 0x0000_3f43,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "GEO_ANIM",
        value: 0x0000_4001,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "GEO_ANIM_JOINT",
        value: 0x0000_4002,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "GEO_ANIM_TRANSL_LIST",
        value: 0x0000_4003,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "GEO_ANIM_ROTATE_LIST",
        value: 0x0000_4004,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "GEO_ANIM_QUAT_ROTATE_LIST",
        value: 0x0000_4010,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "GEO_ANIM_SCALE_LIST",
        value: 0x0000_4005,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "GEO_ANIM_CLUT",
        value: 0x0000_4006,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_DEFORM_POLYSKIN",
        value: 0x0000_4a88,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_DEFORM_POLYSKIN_JOINT",
        value: 0x0000_4a89,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_DEFORM_POLYSKIN_STATE",
        value: 0x0000_4a8a,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "GEO_COMPOSITE_ANIM",
        value: 0x0000_4007,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "GEO_ANIM_TEX",
        value: 0x0000_4008,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "GEO_ANIM_ROOT_TRANS",
        value: 0x0000_4009,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "GEO_ANIM_VERT",
        value: 0x0000_400a,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "GEO_ANIM_VERT_SPHERE",
        value: 0x0000_400b,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "GEO_ANIM_VERT_FRAMES",
        value: 0x0000_400c,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "GEO_ANIM_CVERT",
        value: 0x0000_400d,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "GEO_ANIM_CVERT_SPHERE",
        value: 0x0000_400e,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "GEO_ANIM_CVERT_FRAMES",
        value: 0x0000_400f,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "GEO_ANIM_TREETYPE",
        value: 0x0000_4011,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "ANIM_SEQ",
        value: 0x0000_4012,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_VIZ_ANIM",
        value: 0x0000_4020,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_VIZ_ANIM_DATA",
        value: 0x0000_4021,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_UV_ANIM",
        value: 0x0000_4030,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_UV_ANIM_FRAMES",
        value: 0x0000_4031,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_CBV_ANIM",
        value: 0x0000_4040,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_CBV_ANIM_FRAMES",
        value: 0x0000_4041,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_CBV_PARAM_ANIM",
        value: 0x0000_4050,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_CBV_PARAM_ANIM_FRAMES",
        value: 0x0000_4051,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_EVENT_ANIM",
        value: 0x0000_4060,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_EVENT_ANIM_EVENT",
        value: 0x0000_4061,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_EVENT_ANIM_DATA",
        value: 0x0000_4062,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_JOINT_STATE",
        value: 0x0000_4410,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_STATE_VERTEX_MAP",
        value: 0x0000_4411,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_STATE_CHANNEL",
        value: 0x0000_4412,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_JOINT_DEFORMER",
        value: 0x0000_4413,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_DEFORMER_JOINT_STATE_MAP",
        value: 0x0000_4414,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_DEFORM_SKIN",
        value: 0x0000_4416,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_DEFORMER",
        value: 0x0000_4417,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_DEFORMER_CHANNEL",
        value: 0x0000_4418,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_DEFORMER_JOINT_DRIVER",
        value: 0x0000_4419,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_DEFORMER_JOINT_DRIVER_DATA",
        value: 0x0000_441a,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_DEFORMER_DRIVER_MAP",
        value: 0x0000_441b,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_DEFORMER_CHANNEL_GROUP",
        value: 0x0000_441c,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "MTR_MTREE",
        value: 0x0000_4100,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "MTR_MTREE_JOINT",
        value: 0x0000_4101,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "MTR_BILLBOARD",
        value: 0x0000_4110,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "STR_STREE",
        value: 0x0000_4120,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "STR_STREE_JOINT",
        value: 0x0000_4121,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "STR_MAPPED_STREE",
        value: 0x0000_4122,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "STR_STREE_MAPPING",
        value: 0x0000_4123,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "STR_STREE_WEIGHTING",
        value: 0x0000_4124,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "STR_STREE_REST_POSE",
        value: 0x0000_4125,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "STR_STREE_PARENT_INDEX",
        value: 0x0000_4126,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "ETR_ETREE",
        value: 0x0000_4140,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "ETR_ETREE_JOINT",
        value: 0x0000_4141,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_TRAN_ANIM",
        value: 0x0000_4200,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_JOINT",
        value: 0x0000_4202,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_TIME_INDEX",
        value: 0x0000_4203,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_JOINT_NAMES",
        value: 0x0000_4204,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_JOINT_INFO",
        value: 0x0000_4205,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_KEYLIST_1DOF",
        value: 0x0000_4210,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_KEYLIST_2DOF",
        value: 0x0000_4211,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_KEYLIST_3DOF",
        value: 0x0000_4212,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_KEYLIST_1DOF_ANGLE",
        value: 0x0000_4213,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_KEYLIST_2DOF_ANGLE",
        value: 0x0000_4214,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_KEYLIST_3DOF_ANGLE",
        value: 0x0000_4215,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_KEYLIST_QUAT",
        value: 0x0000_4217,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_KEYLIST_ROT",
        value: 0x0000_4218,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_KEYLIST_SCALEMATRIX",
        value: 0x0000_4219,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_STATIC_ROT_KEYLIST",
        value: 0x0000_4220,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_STATIC_TRANS_KEYLIST",
        value: 0x0000_4221,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_STATIC_SCALE_KEYLIST",
        value: 0x0000_4222,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_STATIC_QUAT_KEYLIST",
        value: 0x0000_4223,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_STATIC_SCALEMATRIX",
        value: 0x0000_4224,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_STATIC_ROTATION",
        value: 0x0000_4225,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_STATIC_TRANSLATION",
        value: 0x0000_4226,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_KEYLIST_HS_OFF_3DOF",
        value: 0x0000_4230,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_PARAM_ANIM",
        value: 0x0000_4300,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_PARAM_ANIM_PARAM",
        value: 0x0000_4301,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_HSPLINE_PARAM_ANIM",
        value: 0x0000_4400,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_USER_PARAM_ANIM",
        value: 0x8000_0000,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_PARAM_CAM_ANIM",
        value: 0x0000_0001,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_PARAM_TARGETCAM_ANIM",
        value: 0x0000_0002,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_PARAM_LIGHT_ANIM",
        value: 0x0000_0101,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_PARAM_HS_ABS_OFF_ANIM",
        value: 0x0000_0201,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_PARAM_HS_REL_OFF_ANIM",
        value: 0x0000_0202,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_PARAM_CBV_ANIM",
        value: 0x0000_0300,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "PSX_KEYLIST_1DOF",
        value: 0x0000_4283,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "PSX_KEYLIST_3DOF",
        value: 0x0000_4285,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_PM_MESH",
        value: 0x0000_5000,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_PM_SKIN",
        value: 0x0000_5001,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_PM_PRIM_GROUP",
        value: 0x0000_5002,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_PM_HISTORY",
        value: 0x0000_5005,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_PM_HISTORY_ELEMENT",
        value: 0x0000_5006,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_VDPM_GEO",
        value: 0x0000_5010,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_VDPM_STREE",
        value: 0x0000_5011,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_VDPM_HISTORY",
        value: 0x0000_5012,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_VDPM_JOINT_HISTORY",
        value: 0x0000_5013,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_VDPM_HISTORY_LEVEL",
        value: 0x0000_5014,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "PARAM_CAM_ROT",
        value: 0x0000_0001,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "PARAM_CAM_TRANS",
        value: 0x0000_0002,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "PARAM_CAM_FOV_X",
        value: 0x0000_0011,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "PARAM_CAM_FOV_Y",
        value: 0x0000_0012,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "PARAM_CAM_TARGET",
        value: 0x0000_0021,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "PARAM_CAM_ROLL",
        value: 0x0000_0022,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "PARAM_LIGHT_ROT",
        value: 0x0000_0101,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "PARAM_LIGHT_TRANS",
        value: 0x0000_0102,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "PARAM_LIGHT_COLOUR_RGB",
        value: 0x0000_0111,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "PARAM_LIGHT_COLOUR_HSV",
        value: 0x0000_0112,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "PARAM_LIGHT_MULTIPLIER",
        value: 0x0000_0120,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "PARAM_HS_OFFSET_TRANS",
        value: 0x0000_0202,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_LIGHT",
        value: 0x0000_2300,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_LIGHT_EXCLUSION",
        value: 0x0000_2310,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_LIGHT_ATTENUATION",
        value: 0x0000_2320,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_LIGHT_SHADOW",
        value: 0x0000_2330,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_LIGHT_SHADOW_MAPPED",
        value: 0x0000_2331,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_LIGHT_EXTRA",
        value: 0x0000_2340,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_CORONA",
        value: 0x0000_2400,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_LENSFLARE",
        value: 0x0000_2401,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_LENSFLARE_FLARE",
        value: 0x0000_2402,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "PSX_VERSION",
        value: 0x0000_6000,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "PSX_MATERIALS",
        value: 0x0000_6001,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "PSX_GEOMETRY",
        value: 0x0000_6002,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "PSX_COLLISION_GEOM",
        value: 0x0000_6003,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "PSX_VERT_ANIM",
        value: 0x0000_6004,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "PSX_NORM_ANIM",
        value: 0x0000_6005,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "PSX_CLUT_ANIM",
        value: 0x0000_6006,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "PSX_TEX_ANIM",
        value: 0x0000_6007,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "PSX_TEXTURE",
        value: 0x0000_6008,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "PSX_PRIMS",
        value: 0x0000_6009,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "PSX_TEX_ANIM_FRAMES",
        value: 0x0000_600a,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "PSX_TEX_ANIM_OFFSETS",
        value: 0x0000_600b,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "PSX_CLUT_ANIM_FRAMES",
        value: 0x0000_600c,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "PSX_CLUT_ANIM_OFFSETS",
        value: 0x0000_600d,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "PSX_UV_ANIM",
        value: 0x0000_600e,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "PSX_UV_ANIM_FRAMES",
        value: 0x0000_600f,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "PSX_UV_ANIM_OFFSETS",
        value: 0x0000_6010,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "PSX_CBV_ANIM",
        value: 0x0000_6011,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "PSX_CBV_ANIM_FRAMES",
        value: 0x0000_6012,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "PSX_CBV_ANIM_OFFSETS",
        value: 0x0000_6013,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "PSX_CBV_PARAM_ANIM",
        value: 0x0000_6021,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "PSX_CBV_PARAM_ANIM_FRAMES",
        value: 0x0000_6022,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "PSX_CBV_PARAM_ANIM_OFFSETS",
        value: 0x0000_6023,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "PSX_SEQUENCE_ANIM",
        value: 0x0000_6040,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "PSX_MAIN_RAM_TEX_ANIM",
        value: 0x0000_6050,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "PSX_MAIN_RAM_TEX_ANIM_NAMES",
        value: 0x0000_6051,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "PSX_MAIN_RAM_TEX_ANIM_FRAMES",
        value: 0x0000_6052,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "PSX_STREE",
        value: 0x0000_6120,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "PSX_STREE_JOINT",
        value: 0x0000_6121,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "PSX_MAPPED_STREE",
        value: 0x0000_6122,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "PSX_STREE_WEIGHTING",
        value: 0x0000_6124,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "PSX_STREE_REST_POSE",
        value: 0x0000_6125,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "PSX_MTREE",
        value: 0x0000_6130,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "PSX_MTREE_JOINT",
        value: 0x0000_6131,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "PSX_ETREE",
        value: 0x0000_6140,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "PSX_ETREE_JOINT",
        value: 0x0000_6141,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "PSX_TRAN_ANIM",
        value: 0x0000_6400,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "PSX_TEXTURE_REF",
        value: 0x0000_6500,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "PSX_PRIM_OFFSETS",
        value: 0x0000_6600,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "PSX_MATRIX",
        value: 0x0000_6f00,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "MAT_USELIGHT",
        value: 0x0000_0001,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "MAT_GOURAUD",
        value: 0x0000_0002,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "MAT_COLOURBYVERTEX",
        value: 0x0000_0004,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "MAT_PERSP",
        value: 0x0000_0008,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "MAT_ENVMAP",
        value: 0x0000_0010,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "MAT_TILED",
        value: 0x0000_0020,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "MAT_STIPPLEALPHA",
        value: 0x0000_0040,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "MAT_MONOLIT",
        value: 0x0000_0080,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "MAT_ALPHA",
        value: 0x0000_0100,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "MAT_WIREFRAME",
        value: 0x0000_0200,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "MAT_ALPHATEST",
        value: 0x0000_0400,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "MAT_BLEND_ZERO",
        value: 0x0000_0001,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "MAT_BLEND_ONE",
        value: 0x0000_0002,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "MAT_BLEND_SRC",
        value: 0x0000_0003,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "MAT_BLEND_ONE_MINUS_SRC",
        value: 0x0000_0004,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "MAT_BLEND_DEST",
        value: 0x0000_0005,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "MAT_BLEND_ONE_MINUS_DEST",
        value: 0x0000_0006,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "MAT_BLEND_SRCALPHA",
        value: 0x0000_0007,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "MAT_BLEND_ONE_MINUS_SRCALPHA",
        value: 0x0000_0008,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "MAT_BLEND_DESTALPHA",
        value: 0x0000_0009,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "MAT_BLEND_ONE_MINUS_DESTALPHA",
        value: 0x0000_000a,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "MAT_BLEND_SRCALPHASATURATE",
        value: 0x0000_000b,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "TEX_COLOURKEY",
        value: 0x0000_0001,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "TEX_ONEBITALPHA",
        value: 0x0000_0002,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "BSP_TREE",
        value: 0x0000_9000,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "BSP_NODE_SPLIT",
        value: 0x0000_9001,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "BSP_NODE_LEAF",
        value: 0x0000_9002,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "BSP_NODE_NULL",
        value: 0x0000_9003,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_SG_SCENEGRAPH",
        value: 0x0000_9100,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_SG_ROOT",
        value: 0x0000_9101,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_SG_BRANCH",
        value: 0x0000_9102,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_SG_TRANSFORM",
        value: 0x0000_9103,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_SG_DRAWABLE",
        value: 0x0000_9104,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_SG_CAMERA",
        value: 0x0000_9105,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_SG_LIGHTGROUP",
        value: 0x0000_9106,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_SG_ATTACHMENT",
        value: 0x0000_9107,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_SG_ATTACHMENTPOINT",
        value: 0x0000_9108,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_SG_VISIBILITY",
        value: 0x0000_9109,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "P3D_SG_TRANSFORM_CONTROLLER",
        value: 0x0000_9151,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "PHY_OBJ_OLD",
        value: 0x0000_c000,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "PHY_OBJ",
        value: 0x0000_c111,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "PHY_OBJ_IMAT",
        value: 0x0000_c001,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "PHY_OBJ_COLLEL",
        value: 0x0000_c002,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "PHY_OBJ_COLLEL_SPHERE",
        value: 0x0000_c003,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "PHY_OBJ_COLLEL_CYL",
        value: 0x0000_c004,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "PHY_OBJ_COLLEL_OBBOX",
        value: 0x0000_c005,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "PHY_OBJ_COLLEL_WALL",
        value: 0x0000_c006,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "PHY_OBJ_COLLEL_BBOX",
        value: 0x0000_c007,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "PHY_VECTOR",
        value: 0x0000_c010,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "PHY_OBJ_JOINT",
        value: 0x0000_c011,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "PHY_OBJ_JOINT_DOF",
        value: 0x0000_c012,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "PHY_OBJ_SELFCOLL",
        value: 0x0000_c020,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "PHY_OBJ_SELFCOLL_ITEM",
        value: 0x0000_c021,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "PHY_FOOTSTEPS",
        value: 0x0000_c100,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "PHY_FLEX_GEOM",
        value: 0x0000_c200,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "PHY_FLEX_JOINT",
        value: 0x0000_c201,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "PHY_FLEX_PARAM",
        value: 0x0000_c210,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "PHY_FLEX_FIX_PARTICLE",
        value: 0x0000_c211,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "PHY_FLEX_MAP_VL",
        value: 0x0000_c212,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "PHY_FLEX_TRI_MAP",
        value: 0x0000_c213,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "PHY_FLEX_EDGE_MAP",
        value: 0x0000_c214,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "PHY_FLEX_EDGE_LEN",
        value: 0x0000_c215,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "PHY_FLEX_COLL_JOINT",
        value: 0x0000_c216,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "PHY_FLEX_JOINT_DEF",
        value: 0x0000_c220,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "PHY_LINK",
        value: 0x0000_c320,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "PHY_LINK_IK",
        value: 0x0000_c321,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "PHY_LINK_REACH",
        value: 0x0000_c322,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "PHY_LINK_TRACKER",
        value: 0x0000_c323,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "PHY_LINK_TARGET",
        value: 0x0000_c330,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "PHY_TARGET_NODE",
        value: 0x0000_c331,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "PHY_TARGET_POSE",
        value: 0x0000_c332,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "TW_WORLD",
        value: 0x0000_b000,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "TW_OBJ_POINT",
        value: 0x0000_b001,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "TW_OBJ_LINE",
        value: 0x0000_b002,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "TW_OBJ_MESH",
        value: 0x0000_b003,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "TW_OBJ_ICON",
        value: 0x0000_b004,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "TW_OBJ_VOLUME",
        value: 0x0000_b005,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "TW_OBJ_SPHERE",
        value: 0x0000_b006,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "TW_OBJ_LIGHT",
        value: 0x0000_b007,
    },
    ChunkIdConstant {
        authority_key: "p3d_legacy",
        scope: "P3D",
        name: "TW_BLOCK_VOLUME",
        value: 0x0000_bb00,
    },
    ChunkIdConstant {
        authority_key: "srr",
        scope: "SRR2::ChunkID",
        name: "WALL",
        value: 0x0300_0000,
    },
    ChunkIdConstant {
        authority_key: "srr",
        scope: "SRR2::ChunkID",
        name: "FENCELINE",
        value: 0x0300_0001,
    },
    ChunkIdConstant {
        authority_key: "srr",
        scope: "SRR2::ChunkID",
        name: "ROAD_SEGMENT",
        value: 0x0300_0002,
    },
    ChunkIdConstant {
        authority_key: "srr",
        scope: "SRR2::ChunkID",
        name: "ROAD",
        value: 0x0300_0003,
    },
    ChunkIdConstant {
        authority_key: "srr",
        scope: "SRR2::ChunkID",
        name: "INTERSECTION",
        value: 0x0300_0004,
    },
    ChunkIdConstant {
        authority_key: "srr",
        scope: "SRR2::ChunkID",
        name: "LOCATOR",
        value: 0x0300_0005,
    },
    ChunkIdConstant {
        authority_key: "srr",
        scope: "SRR2::ChunkID",
        name: "TRIGGER_VOLUME",
        value: 0x0300_0006,
    },
    ChunkIdConstant {
        authority_key: "srr",
        scope: "SRR2::ChunkID",
        name: "SPLINE",
        value: 0x0300_0007,
    },
    ChunkIdConstant {
        authority_key: "srr",
        scope: "SRR2::ChunkID",
        name: "INSTANCES",
        value: 0x0300_0008,
    },
    ChunkIdConstant {
        authority_key: "srr",
        scope: "SRR2::ChunkID",
        name: "ROAD_SEGMENT_DATA",
        value: 0x0300_0009,
    },
    ChunkIdConstant {
        authority_key: "srr",
        scope: "SRR2::ChunkID",
        name: "RAIL",
        value: 0x0300_000a,
    },
    ChunkIdConstant {
        authority_key: "srr",
        scope: "SRR2::ChunkID",
        name: "PED_PATH",
        value: 0x0300_000b,
    },
    ChunkIdConstant {
        authority_key: "srr",
        scope: "SRR2::ChunkID",
        name: "EXTRA_MATRIX",
        value: 0x0300_000c,
    },
    ChunkIdConstant {
        authority_key: "srr",
        scope: "SRR2::ChunkID",
        name: "PED_PATH_SEGMENT",
        value: 0x0300_000d,
    },
    ChunkIdConstant {
        authority_key: "srr",
        scope: "SRR2::ChunkID",
        name: "TERRAIN_TYPE",
        value: 0x0300_000e,
    },
    ChunkIdConstant {
        authority_key: "srr",
        scope: "SRR2::ChunkID",
        name: "FOLLOWCAM",
        value: 0x0300_0100,
    },
    ChunkIdConstant {
        authority_key: "srr",
        scope: "SRR2::ChunkID",
        name: "WALKERCAM",
        value: 0x0300_0101,
    },
    ChunkIdConstant {
        authority_key: "srr",
        scope: "SRR2::ChunkID",
        name: "CHUNK_SET",
        value: 0x0300_0110,
    },
    ChunkIdConstant {
        authority_key: "srr",
        scope: "SRR2::ChunkID",
        name: "OBJECT_ATTRIBUTES",
        value: 0x0300_0600,
    },
    ChunkIdConstant {
        authority_key: "srr",
        scope: "SRR2::ChunkID",
        name: "PHYS_WRAPPER",
        value: 0x0300_0601,
    },
    ChunkIdConstant {
        authority_key: "srr",
        scope: "SRR2::ChunkID",
        name: "ATTRIBUTE_TABLE",
        value: 0x0300_0602,
    },
    ChunkIdConstant {
        authority_key: "srr",
        scope: "SRR2::ChunkID",
        name: "BREAKABLE_OBJECT",
        value: 0x0300_1000,
    },
    ChunkIdConstant {
        authority_key: "srr",
        scope: "SRR2::ChunkID",
        name: "INST_PARTICLE_SYSTEM",
        value: 0x0300_1001,
    },
    ChunkIdConstant {
        authority_key: "srr",
        scope: "SRR2::ChunkID",
        name: "ENTITY_DSG",
        value: 0x03f0_0000,
    },
    ChunkIdConstant {
        authority_key: "srr",
        scope: "SRR2::ChunkID",
        name: "STATIC_PHYS_DSG",
        value: 0x03f0_0001,
    },
    ChunkIdConstant {
        authority_key: "srr",
        scope: "SRR2::ChunkID",
        name: "DYNA_PHYS_DSG",
        value: 0x03f0_0002,
    },
    ChunkIdConstant {
        authority_key: "srr",
        scope: "SRR2::ChunkID",
        name: "INTERSECT_DSG",
        value: 0x03f0_0003,
    },
    ChunkIdConstant {
        authority_key: "srr",
        scope: "SRR2::ChunkID",
        name: "TREE_DSG",
        value: 0x03f0_0004,
    },
    ChunkIdConstant {
        authority_key: "srr",
        scope: "SRR2::ChunkID",
        name: "CONTIGUOUS_BIN_NODE",
        value: 0x03f0_0005,
    },
    ChunkIdConstant {
        authority_key: "srr",
        scope: "SRR2::ChunkID",
        name: "SPATIAL_NODE",
        value: 0x03f0_0006,
    },
    ChunkIdConstant {
        authority_key: "srr",
        scope: "SRR2::ChunkID",
        name: "FENCE_DSG",
        value: 0x03f0_0007,
    },
    ChunkIdConstant {
        authority_key: "srr",
        scope: "SRR2::ChunkID",
        name: "ANIM_COLL_DSG",
        value: 0x03f0_0008,
    },
    ChunkIdConstant {
        authority_key: "srr",
        scope: "SRR2::ChunkID",
        name: "INSTA_ENTITY_DSG",
        value: 0x03f0_0009,
    },
    ChunkIdConstant {
        authority_key: "srr",
        scope: "SRR2::ChunkID",
        name: "INSTA_STATIC_PHYS_DSG",
        value: 0x03f0_000a,
    },
    ChunkIdConstant {
        authority_key: "srr",
        scope: "SRR2::ChunkID",
        name: "WORLD_SPHERE_DSG",
        value: 0x03f0_000b,
    },
    ChunkIdConstant {
        authority_key: "srr",
        scope: "SRR2::ChunkID",
        name: "ANIM_DSG",
        value: 0x03f0_000c,
    },
    ChunkIdConstant {
        authority_key: "srr",
        scope: "SRR2::ChunkID",
        name: "LENS_FLARE_DSG",
        value: 0x03f0_000d,
    },
    ChunkIdConstant {
        authority_key: "srr",
        scope: "SRR2::ChunkID",
        name: "INSTA_ANIM_DYNA_PHYS_DSG",
        value: 0x03f0_000e,
    },
    ChunkIdConstant {
        authority_key: "srr",
        scope: "SRR2::ChunkID",
        name: "ANIM_DSG_WRAPPER",
        value: 0x03f0_000f,
    },
    ChunkIdConstant {
        authority_key: "srr",
        scope: "SRR2::ChunkID",
        name: "ANIM_OBJ_DSG_WRAPPER",
        value: 0x03f0_0010,
    },
];

/// Chunk constants by value.
pub fn chunk_constants_by_value(
    value: u32
) -> impl Iterator<Item = &'static ChunkIdConstant> {
    CHUNK_ID_CONSTANTS
        .iter()
        .filter(move |constant| constant.value == value)
}

/// First chunk constant.
#[must_use]
pub fn first_chunk_constant(value: u32) -> Option<&'static ChunkIdConstant> {
    chunk_constants_by_value(value).next()
}

/// Schemas matching one generated chunk name in source order.
pub fn schemas_by_chunk_name(
    name: &str
) -> impl Iterator<Item = &'static ChunkDefinition> {
    SCHEMA_CHUNKS
        .iter()
        .filter(move |schema| schema.name == name)
}

/// Returns the schema for a chunk name only when that name is unique.
// The explicit function name distinguishes chunk-name lookup from schema-key
// lookup. Ambiguous generated names fail closed instead of depending on source
// order.
#[expect(
    clippy::module_name_repetitions,
    reason = "Explicit lookup names keep generated schema query semantics \
              unambiguous."
)]
#[must_use]
pub fn schema_by_chunk_name(name: &str) -> Option<&'static ChunkDefinition> {
    let mut schemas = schemas_by_chunk_name(name);
    let schema = schemas.next()?;
    schemas
        .next()
        .is_none()
        .then_some(schema)
}
/// Schemas by key.
pub fn schemas_by_key(
    schema_key: &str
) -> impl Iterator<Item = &'static ChunkDefinition> {
    SCHEMA_CHUNKS
        .iter()
        .filter(move |schema| schema.schema_key == schema_key)
}

/// Schema ref for kind.
// The explicit function name preserves the stable schema-reference vocabulary.
#[expect(
    clippy::match_same_arms,
    clippy::module_name_repetitions,
    reason = "Generated aliases intentionally share stable schema references \
              for callers."
)]
#[must_use]
pub fn schema_ref_for_kind(kind: &str) -> Option<&'static str> {
    Some(
        match kind {
            "text_bible" => "text_bible",
            "language" => "language",
            "texture" => "texture",
            "image_data" => "image",
            "mesh" => "mesh",
            "skin" => "skin",
            "shader" => "shader",
            "animation" => "animation",
            "skeleton" => "skeleton",
            "composite_drawable" => "composite_drawable",
            "camera" => "camera",
            "light" => "light",
            "light_group" => "light_group",
            "game_attr" => "game_attr",
            "particle_system_factory" => "particle_system_factory",
            "particle_system" => "particle_system",
            "scenegraph" => "scenegraph",
            "srr_road" => "road",
            "srr_intersection" => "intersection",
            "srr_road_segment_data" => "road_segment_data",
            "srr_entity_dsg" => "entity_dsg",
            "srr_static_phys_dsg" => "static_phys_dsg",
            "srr_intersect_dsg" => "intersect_dsg",
            "srr_tree_dsg" => "tree_dsg",
            "srr_fence_dsg" => "fence_dsg",
            "srr_anim_coll_dsg" => "anim_coll_dsg",
            "srr_world_sphere_dsg" => "world_sphere_dsg",
            "srr_anim_dsg" => "anim_dsg",
            "locator" => "locator",
            "sprite" => "sprite",
            "quad_group" => "quad_group",
            "multi_controller" => "multi_controller",
            "history" => "history",
            "export_info" => "export_info",
            "animated_object_factory" => "animated_object_factory",
            "animated_object" => "animated_object",
            "vertex_expression_group" => "vertex_expression_group",
            "vertex_expression_mixer" => "vertex_expression_mixer",
            "texture_font" => "texture_font",
            "scrooby_project" => "scrooby_project",
            "frame_controller" => "frame_controller",
            "frame_controller_variant_a" => "frame_controller_variant_a",
            "frame_controller_variant_b" => "frame_controller_variant_b",
            "vertex_anim_key" => "vertex_anim_key",
            "simulation_collision_object" => "simulation_collision_object",
            "simulation_physics_object" => "simulation_physics_object",
            "state_prop" => "state_prop",
            "srr_locator" => "locator",
            "srr_ped_path" => "ped_path",
            "srr_chunk_set" => "chunk_set",
            "srr_attribute_table" => "attribute_table",
            "srr_breakable_object" => "breakable_object",
            "srr_inst_particle_system" => "inst_particle_system",
            "srr_follow_cam" => "follow_cam",
            "srr_dyna_phys_dsg" => "dyna_phys_dsg",
            "srr_insta_entity_dsg" => "insta_entity_dsg",
            "srr_insta_static_phys_dsg" => "insta_static_phys_dsg",
            "srr_insta_anim_dyna_phys_dsg" => "insta_anim_dyna_phys_dsg",
            "srr_lens_flare_dsg" => "lens_flare_dsg",
            _ => return None,
        },
    )
}

#[cfg(test)]
mod tests {
    use super::{
        CHUNK_ID_CONSTANT_COUNT, CHUNK_ID_CONSTANTS, SCHEMA_CHUNK_COUNT,
        SCHEMA_FILE_COUNT, chunk_constants_by_value, schema_by_chunk_name,
        schema_ref_for_kind, schemas_by_chunk_name,
    };

    #[test]
    fn chunk_constants_have_complete_identity_metadata() {
        for constant in CHUNK_ID_CONSTANTS {
            assert!(
                !constant
                    .authority_key
                    .is_empty()
            );
            assert!(
                !constant
                    .scope
                    .is_empty()
            );
            assert!(
                !constant
                    .name
                    .is_empty()
            );
        }
    }

    #[test]
    fn singular_schema_lookup_rejects_ambiguous_names() {
        assert!(schema_by_chunk_name("tlCompositeSkinProp").is_none());
        assert_eq!(
            schemas_by_chunk_name("tlCompositeSkinProp").count(),
            2
        );
    }

    #[test]
    fn registry_covers_all_schema16_files() {
        assert_eq!(
            SCHEMA_FILE_COUNT,
            88
        );
        assert_eq!(
            SCHEMA_CHUNK_COUNT,
            293
        );
        const { assert!(CHUNK_ID_CONSTANT_COUNT > 200) };
        assert!(schema_by_chunk_name("texture").is_some());
        assert!(schema_by_chunk_name("mesh").is_some());
        assert!(schema_by_chunk_name("fence_dsg").is_some());
        assert_eq!(
            schema_ref_for_kind("mesh"),
            Some("mesh")
        );
        assert!(
            chunk_constants_by_value(0x03f0_0007).any(
                |constant| constant
                    .name
                    .contains("FENCE")
            )
        );
    }
}
