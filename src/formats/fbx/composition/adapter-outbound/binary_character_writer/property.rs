// Copyright:
//   - Copyright © 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT
//
// Boundary-Contract:
// - Owns:
//   - Binary character FBX property construction and serialization failures.
// - Must-Not:
//   - Own scene assembly, filesystem persistence, or domain translation.
// - Allows:
//   - Checked binary node factories and stable serialization error values.
// - Split-When:
//   - Split when one property family gains an independent format lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical property construction.
// - Summary:
//   - Binary character FBX property and error support.
// - Description:
//   - Builds scalar FBX nodes and owns checked writer failure evidence.
// - Usage:
//   - Used by the owning deterministic binary character writer.
// - Defaults:
//   - Numeric narrowing and invalid identities fail explicitly.
//

//! Binary character FBX property and error support.

use super::{
    BinaryIdentityError, BinaryNode, BinaryProperty, CharacterInputError,
    InverseError,
};

/// Build one vector property entry.
pub(super) fn vector_property(name: &str, value: [f64; 3]) -> BinaryNode {
    BinaryNode::leaf("P", vec![
        string(name),
        string(name),
        string(""),
        string("A"),
        BinaryProperty::F64(value[0]),
        BinaryProperty::F64(value[1]),
        BinaryProperty::F64(value[2]),
    ])
}

/// Build one RGB color property entry.
pub(super) fn color_property(name: &str, value: [f64; 3]) -> BinaryNode {
    BinaryNode::leaf("P", vec![
        string(name),
        string("ColorRGB"),
        string("Color"),
        string(""),
        BinaryProperty::F64(value[0]),
        BinaryProperty::F64(value[1]),
        BinaryProperty::F64(value[2]),
    ])
}

/// Build one integer property entry.
pub(super) fn integer_property(name: &str, value: i32) -> BinaryNode {
    BinaryNode::leaf("P", vec![
        string(name),
        string("int"),
        string("Integer"),
        string(""),
        BinaryProperty::I32(value),
    ])
}

/// Build one enum property entry.
pub(super) fn enum_property(name: &str, value: i32) -> BinaryNode {
    BinaryNode::leaf("P", vec![
        string(name),
        string("enum"),
        string(""),
        string(""),
        BinaryProperty::I32(value),
    ])
}

/// Build one FBX visibility property entry.
pub(super) fn visibility_property(value: f64) -> BinaryNode {
    BinaryNode::leaf("P", vec![
        string("Visibility"),
        string("Visibility"),
        string(""),
        string("A"),
        BinaryProperty::F64(value),
    ])
}

/// Build one double property entry.
pub(super) fn double_property(name: &str, value: f64) -> BinaryNode {
    BinaryNode::leaf("P", vec![
        string(name),
        string("double"),
        string("Number"),
        string(""),
        BinaryProperty::F64(value),
    ])
}

/// Build one time property entry.
pub(super) fn time_property(name: &str, value: i64) -> BinaryNode {
    BinaryNode::leaf("P", vec![
        string(name),
        string("KTime"),
        string("Time"),
        string(""),
        BinaryProperty::I64(value),
    ])
}

/// Build one external-reference string property entry.
pub(super) fn xref_string_property(name: &str, value: &str) -> BinaryNode {
    BinaryNode::leaf("P", vec![
        string(name),
        string("KString"),
        string("XRefUrl"),
        string(""),
        string(value),
    ])
}

/// Build one user-defined string property entry.
pub(super) fn user_string_property(name: &str, value: &str) -> BinaryNode {
    BinaryNode::leaf("P", vec![
        string(name),
        string("KString"),
        string(""),
        string("U"),
        string(value),
    ])
}

/// Build one string property entry.
pub(super) fn string_property(name: &str, value: &str) -> BinaryNode {
    BinaryNode::leaf("P", vec![
        string(name),
        string("KString"),
        string(""),
        string(""),
        string(value),
    ])
}

/// Build one scalar i32 node.
pub(super) fn i32_node(name: &str, value: i32) -> BinaryNode {
    BinaryNode::leaf(name, vec![BinaryProperty::I32(value)])
}

/// Build one scalar string node.
pub(super) fn string_node(name: &str, value: &str) -> BinaryNode {
    BinaryNode::leaf(name, vec![string(value)])
}

/// Build one owned string property.
pub(super) fn string(value: &str) -> BinaryProperty {
    BinaryProperty::String(value.to_owned())
}

/// Build one binary FBX object name with its class separator.
pub(super) fn name_class(name: &str, class: &str) -> BinaryProperty {
    BinaryProperty::String(format!("{name}\0\x01{class}"))
}

/// Build one scalar binary FBX name-and-class node.
pub(super) fn name_class_node(
    name: &str,
    value: &str,
    class: &str,
) -> BinaryNode {
    BinaryNode::leaf(name, vec![name_class(value, class)])
}

/// Convert one deterministic unsigned object id to an FBX signed id.
pub(super) fn id_property(
    id: u64,
) -> Result<BinaryProperty, CharacterBinaryFbxError> {
    let narrowed = i64::try_from(id).map_err(|_conversion_error| {
        CharacterBinaryFbxError::IdExceedsI64 { id }
    })?;
    Ok(BinaryProperty::I64(narrowed))
}

/// Convert one object count to an FBX signed 32-bit count.
pub(super) fn count_i32(
    count: usize,
    context: &'static str,
) -> Result<i32, CharacterBinaryFbxError> {
    i32::try_from(count).map_err(|_conversion_error| {
        CharacterBinaryFbxError::CountExceedsI32 { context, count }
    })
}

/// Binary character FBX serialization failure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CharacterBinaryFbxError {
    /// Static model identity was empty, padded, or contained control data.
    InvalidModelName,
    /// Static model aggregate did not contain any mesh geometry.
    MissingModelMeshes,
    /// Two static model meshes shared one scene identity.
    DuplicateModelMeshName {
        /// Repeated mesh identity.
        mesh: String,
    },
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
    /// Target-only surface-frame derivation reached a triangle with no usable
    /// normal.
    DegenerateTargetSurfaceFrame {
        /// Deterministic geometry object identity.
        object: String,
        /// Triangle ordinal in the emitted primitive group.
        triangle: usize,
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
