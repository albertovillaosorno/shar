// File:
//   - binary_fbx.rs
// Path:
//   - src/fbx/src/adapters/driven/binary_fbx.rs
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
//   - Deterministic FBX 7.7 binary node and property encoding.
// - Must-Not:
//   - Interpret scene semantics, read assets, or choose object identities.
// - Allows:
//   - Little-endian node records, raw arrays, sentinels, and standard footer
//   - bytes for one complete FBX binary document.
// - Split-When:
//   - A later FBX version requires a distinct record-width implementation.
// - Merge-When:
//   - Another adapter owns the identical binary node encoding contract.
// - Summary:
//   - Encodes a typed FBX node tree as a binary 7.7 document.
// - Description:
//   - Computes checked absolute node offsets before writing uncompressed
//   - property arrays and the deterministic FBX footer.
// - Usage:
//   - Called by concrete binary scene or character writer adapters.
// - Defaults:
//   - Uses FBX version 7700, 64-bit node metadata, and no compression.
//
// ADRs:
// - docs/adr/pipeline/fbx/hexagonal-scene-export.md
//
// Large file:
//   - true
//   - Reason: The module keeps FBX 7.7 node sizing, typed property encoding,
//   - absolute offsets, raw arrays, and footer construction together because
//   - they form one binary container invariant; split when another FBX record
//   - width or compression format requires a separate encoder.
//

//! Deterministic FBX 7.7 binary node and property encoding.
//!
//! Scene-specific writers build a typed node tree. This module owns only the
//! binary container: checked absolute offsets, uncompressed property arrays,
//! null records, and the standard deterministic footer.

/// FBX 7.7 binary file version.
pub(super) const FBX_VERSION: u32 = 7_700;
/// FBX binary file signature including the binary marker bytes.
pub(super) const BINARY_MAGIC: &[u8; 23] = b"Kaydara FBX Binary  \x00\x1a\x00";
/// Fixed file identity used to keep repeated writes byte-identical.
const FILE_ID: [u8; 16] = [
    0x28, 0xb3, 0x2a, 0xeb, 0xb6, 0x24, 0xcc, 0xc2, 0xbf, 0xc8, 0xb0, 0x2a,
    0xa9, 0x2b, 0xfc, 0xf1,
];
/// Footer identity used by common FBX 7.7 writers.
const FOOTER_ID: [u8; 16] = [
    0xfa, 0xbc, 0xab, 0x09, 0xd0, 0xc8, 0xd4, 0x66, 0xb1, 0x76, 0xfb, 0x83,
    0x1c, 0xf7, 0x26, 0x7e,
];
/// Final footer marker used by common FBX 7.7 writers.
const FINAL_MAGIC: [u8; 16] = [
    0xf8, 0x5a, 0x8c, 0x6a, 0xde, 0xf5, 0xd9, 0x7e, 0xec, 0xe9, 0x0c, 0xe3,
    0x75, 0x8f, 0x29, 0x0b,
];
/// One 7.7 null node record: three zero u64 values plus a zero name length.
const NULL_RECORD: [u8; 25] = [0; 25];
/// Fixed creation timestamp used for deterministic output.
pub(super) const CREATION_TIME: &str = "1970-01-01 10:00:00:000";
/// Fixed file identifier exposed to the character document builder.
pub(super) const DETERMINISTIC_FILE_ID: [u8; 16] = FILE_ID;

/// One typed FBX binary property.
#[derive(Clone, Debug, PartialEq)]
pub(super) enum BinaryProperty {
    /// One FBX boolean property.
    Bool(bool),
    /// One signed 32-bit integer property.
    I32(i32),
    /// One signed 64-bit integer property.
    I64(i64),
    /// One 64-bit floating-point property.
    F64(f64),
    /// One raw byte property.
    Bytes(Vec<u8>),
    /// One UTF-8 string property.
    String(String),
    /// One uncompressed signed 32-bit integer array.
    I32Array(Vec<i32>),
    /// One uncompressed signed 64-bit integer array.
    I64Array(Vec<i64>),
    /// One uncompressed 32-bit floating-point array.
    F32Array(Vec<f32>),
    /// One uncompressed 64-bit floating-point array.
    F64Array(Vec<f64>),
}

/// One FBX node with ordered properties and children.
#[derive(Clone, Debug, PartialEq)]
pub(super) struct BinaryNode {
    /// Node identifier encoded as one-byte characters.
    name: String,
    /// Ordered typed properties.
    properties: Vec<BinaryProperty>,
    /// Ordered child nodes.
    children: Vec<Self>,
    /// Whether this node owns a child scope even when that scope is empty.
    has_child_scope: bool,
}

impl BinaryNode {
    /// Create one node from an id, properties, and children.
    pub(super) fn new(
        name: impl Into<String>,
        properties: Vec<BinaryProperty>,
        children: Vec<Self>,
    ) -> Self {
        Self {
            name: name.into(),
            properties,
            children,
            has_child_scope: true,
        }
    }

    /// Create one property-free node with children.
    pub(super) fn branch(
        name: impl Into<String>,
        children: Vec<Self>,
    ) -> Self {
        Self::new(
            name,
            Vec::new(),
            children,
        )
    }

    /// Create one child-free node with properties.
    pub(super) fn leaf(
        name: impl Into<String>,
        properties: Vec<BinaryProperty>,
    ) -> Self {
        Self {
            name: name.into(),
            properties,
            children: Vec::new(),
            has_child_scope: false,
        }
    }

    /// Return the complete encoded node size.
    fn encoded_len(&self) -> Result<usize, BinaryFbxError> {
        let name_len = self
            .name
            .len();
        if u8::try_from(name_len).is_err() {
            return Err(
                BinaryFbxError::NodeNameTooLong {
                    name: self
                        .name
                        .clone(),
                    length: name_len,
                },
            );
        }
        let properties_len = self.properties_len()?;
        let children_len = self
            .children
            .iter()
            .try_fold(
                0_usize,
                |total, child| {
                    checked_add(
                        total,
                        child.encoded_len()?,
                        "children",
                    )
                },
            )?;
        let sentinel_len = if self.has_child_scope {
            NULL_RECORD.len()
        } else {
            0
        };
        [
            NULL_RECORD.len(),
            name_len,
            properties_len,
            children_len,
            sentinel_len,
        ]
        .into_iter()
        .try_fold(
            0_usize,
            |total, value| {
                checked_add(
                    total,
                    value,
                    "node size",
                )
            },
        )
    }

    /// Return the complete encoded property-list size.
    fn properties_len(&self) -> Result<usize, BinaryFbxError> {
        self.properties
            .iter()
            .try_fold(
                0_usize,
                |total, property| {
                    checked_add(
                        total,
                        property.encoded_len()?,
                        "property list",
                    )
                },
            )
    }

    /// Append this node using absolute file offsets.
    fn append_to(
        &self,
        output: &mut Vec<u8>,
    ) -> Result<(), BinaryFbxError> {
        let start = output.len();
        let node_len = self.encoded_len()?;
        let end = checked_add(
            start,
            node_len,
            "node end offset",
        )?;
        append_u64(
            output,
            end,
            "node end offset",
        )?;
        append_u64(
            output,
            self.properties
                .len(),
            "property count",
        )?;
        append_u64(
            output,
            self.properties_len()?,
            "property list length",
        )?;
        let name_len = u8::try_from(
            self.name
                .len(),
        )
        .map_err(
            |_conversion_error| BinaryFbxError::NodeNameTooLong {
                name: self
                    .name
                    .clone(),
                length: self
                    .name
                    .len(),
            },
        )?;
        output.push(name_len);
        output.extend_from_slice(
            self.name
                .as_bytes(),
        );
        for property in &self.properties {
            property.append_to(output)?;
        }
        for child in &self.children {
            child.append_to(output)?;
        }
        if self.has_child_scope {
            output.extend_from_slice(&NULL_RECORD);
        }
        if output.len() != end {
            return Err(
                BinaryFbxError::LengthMismatch {
                    node: self
                        .name
                        .clone(),
                    expected: end,
                    actual: output.len(),
                },
            );
        }
        Ok(())
    }
}

impl BinaryProperty {
    /// Return the encoded property size including its one-byte type tag.
    fn encoded_len(&self) -> Result<usize, BinaryFbxError> {
        let payload = match self {
            Self::Bool(_) => 1,
            Self::I32(_) => 4,
            Self::I64(_) | Self::F64(_) => 8,
            Self::Bytes(value) => checked_add(
                4,
                value.len(),
                "raw property",
            )?,
            Self::String(value) => checked_add(
                4,
                value.len(),
                "string property",
            )?,
            Self::I32Array(values) => array_property_len(
                values.len(),
                4,
            )?,
            Self::I64Array(values) => array_property_len(
                values.len(),
                8,
            )?,
            Self::F32Array(values) => array_property_len(
                values.len(),
                4,
            )?,
            Self::F64Array(values) => array_property_len(
                values.len(),
                8,
            )?,
        };
        checked_add(
            1,
            payload,
            "typed property",
        )
    }

    /// Append this property in little-endian FBX encoding.
    fn append_to(
        &self,
        output: &mut Vec<u8>,
    ) -> Result<(), BinaryFbxError> {
        match self {
            Self::Bool(value) => {
                output.push(b'C');
                output.push(
                    if *value {
                        b'T'
                    } else {
                        b'F'
                    },
                );
            }
            Self::I32(value) => {
                output.push(b'I');
                output.extend_from_slice(&value.to_le_bytes());
            }
            Self::I64(value) => {
                output.push(b'L');
                output.extend_from_slice(&value.to_le_bytes());
            }
            Self::F64(value) => {
                output.push(b'D');
                output.extend_from_slice(&value.to_le_bytes());
            }
            Self::Bytes(value) => {
                output.push(b'R');
                append_u32(
                    output,
                    value.len(),
                    "raw property length",
                )?;
                output.extend_from_slice(value);
            }
            Self::String(value) => {
                output.push(b'S');
                append_u32(
                    output,
                    value.len(),
                    "string property length",
                )?;
                output.extend_from_slice(value.as_bytes());
            }
            Self::I32Array(values) => {
                output.push(b'i');
                append_array_header(
                    output,
                    values.len(),
                    4,
                )?;
                for value in values {
                    output.extend_from_slice(&value.to_le_bytes());
                }
            }
            Self::I64Array(values) => {
                output.push(b'l');
                append_array_header(
                    output,
                    values.len(),
                    8,
                )?;
                for value in values {
                    output.extend_from_slice(&value.to_le_bytes());
                }
            }
            Self::F32Array(values) => {
                output.push(b'f');
                append_array_header(
                    output,
                    values.len(),
                    4,
                )?;
                for value in values {
                    output.extend_from_slice(&value.to_le_bytes());
                }
            }
            Self::F64Array(values) => {
                output.push(b'd');
                append_array_header(
                    output,
                    values.len(),
                    8,
                )?;
                for value in values {
                    output.extend_from_slice(&value.to_le_bytes());
                }
            }
        }
        Ok(())
    }
}

/// Encode one complete FBX 7.7 binary document.
pub(super) fn encode_binary_document(
    nodes: &[BinaryNode]
) -> Result<Vec<u8>, BinaryFbxError> {
    let body_len = nodes
        .iter()
        .try_fold(
            0_usize,
            |total, node| {
                checked_add(
                    total,
                    node.encoded_len()?,
                    "document body",
                )
            },
        )?;
    let initial_capacity = [
        BINARY_MAGIC.len(),
        4,
        body_len,
        NULL_RECORD.len(),
        FOOTER_ID.len(),
        4,
        16,
        4,
        120,
        FINAL_MAGIC.len(),
    ]
    .into_iter()
    .try_fold(
        0_usize,
        |total, value| {
            checked_add(
                total,
                value,
                "document capacity",
            )
        },
    )?;
    let mut output = Vec::with_capacity(initial_capacity);
    output.extend_from_slice(BINARY_MAGIC);
    output.extend_from_slice(&FBX_VERSION.to_le_bytes());
    for node in nodes {
        node.append_to(&mut output)?;
    }
    output.extend_from_slice(&NULL_RECORD);
    output.extend_from_slice(&FOOTER_ID);
    output.extend_from_slice(&[0; 4]);
    let remainder = output.len() % 16;
    let padding = if remainder == 0 {
        16
    } else {
        16_usize
            .checked_sub(remainder)
            .ok_or(
                BinaryFbxError::LengthOverflow {
                    context: "footer padding",
                },
            )?
    };
    output.resize(
        checked_add(
            output.len(),
            padding,
            "footer alignment",
        )?,
        0,
    );
    output.extend_from_slice(&FBX_VERSION.to_le_bytes());
    output.extend_from_slice(&[0; 120]);
    output.extend_from_slice(&FINAL_MAGIC);
    Ok(output)
}

/// Return one complete uncompressed array property size.
fn array_property_len(
    element_count: usize,
    element_width: usize,
) -> Result<usize, BinaryFbxError> {
    let payload = element_count
        .checked_mul(element_width)
        .ok_or(
            BinaryFbxError::LengthOverflow {
                context: "array payload",
            },
        )?;
    checked_add(
        12,
        payload,
        "array property",
    )
}

/// Append one raw-array header using encoding zero.
fn append_array_header(
    output: &mut Vec<u8>,
    element_count: usize,
    element_width: usize,
) -> Result<(), BinaryFbxError> {
    let payload = element_count
        .checked_mul(element_width)
        .ok_or(
            BinaryFbxError::LengthOverflow {
                context: "array payload",
            },
        )?;
    append_u32(
        output,
        element_count,
        "array element count",
    )?;
    output.extend_from_slice(&0_u32.to_le_bytes());
    append_u32(
        output,
        payload,
        "array payload length",
    )
}

/// Append one checked u32 metadata value.
fn append_u32(
    output: &mut Vec<u8>,
    value: usize,
    context: &'static str,
) -> Result<(), BinaryFbxError> {
    let narrowed = u32::try_from(value).map_err(
        |_conversion_error| BinaryFbxError::ValueExceedsU32 {
            context,
            value,
        },
    )?;
    output.extend_from_slice(&narrowed.to_le_bytes());
    Ok(())
}

/// Append one checked u64 node metadata value.
fn append_u64(
    output: &mut Vec<u8>,
    value: usize,
    context: &'static str,
) -> Result<(), BinaryFbxError> {
    let narrowed = u64::try_from(value).map_err(
        |_conversion_error| BinaryFbxError::ValueExceedsU64 {
            context,
            value,
        },
    )?;
    output.extend_from_slice(&narrowed.to_le_bytes());
    Ok(())
}

/// Checked addition with a stable error context.
fn checked_add(
    left: usize,
    right: usize,
    context: &'static str,
) -> Result<usize, BinaryFbxError> {
    left.checked_add(right)
        .ok_or(
            BinaryFbxError::LengthOverflow {
                context,
            },
        )
}

/// Binary FBX container encoding failure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) enum BinaryFbxError {
    /// One node identifier did not fit in the one-byte FBX name length.
    NodeNameTooLong {
        /// Rejected node name.
        name: String,
        /// UTF-8 byte length of the rejected name.
        length: usize,
    },
    /// Checked size arithmetic overflowed.
    LengthOverflow {
        /// Operation whose length overflowed.
        context: &'static str,
    },
    /// One metadata value exceeded the FBX 7.7 u32 field width.
    ValueExceedsU32 {
        /// Metadata field being narrowed.
        context: &'static str,
        /// Rejected platform-sized value.
        value: usize,
    },
    /// One node metadata value exceeded the FBX 7.7 u64 field width.
    ValueExceedsU64 {
        /// Metadata field being narrowed.
        context: &'static str,
        /// Rejected platform-sized value.
        value: usize,
    },
    /// The final write cursor did not match the precomputed absolute offset.
    LengthMismatch {
        /// Node whose encoded extent mismatched.
        node: String,
        /// Expected absolute end offset.
        expected: usize,
        /// Actual absolute end offset.
        actual: usize,
    },
}
