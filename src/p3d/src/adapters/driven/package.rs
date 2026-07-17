// File:
//   - package.rs
// Path:
//   - src/p3d/src/adapters/driven/package.rs
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
//   - The p3d adapter boundary for adapters driven package.
// - Must-Not:
//   - Change domain semantics or bypass application and port contracts.
// - Allows:
//   - Filesystem, JSON, CLI, Blender, or serialization work behind explicit
//   - ports.
// - Split-When:
//   - Split when package contains two independently testable contracts.
// - Merge-When:
//   - Another p3d module owns the same adapters boundary with no distinct
//   - invariant.
// - Summary:
//   - Write lossless package.
// - Description:
//   - Defines package data and behavior for p3d adapters driven.
// - Usage:
//   - Constructed by composition roots or tests and passed behind port traits.
// - Defaults:
//   - Adapter defaults stay local to the adapter constructor or config.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Write lossless package.
//!
//! This boundary keeps write lossless package explicit and returns
//! deterministic results to p3d callers.
// The explicit package type names distinguish manifest, component, and chunk
// records throughout public P3D call sites.
#![expect(
    clippy::module_name_repetitions,
    reason = "Package helper names keep the public P3D package manifest \
              vocabulary stable."
)]
use std::path::Path;

use crate::{ChunkRecord, P3dDocument, P3dError};

#[derive(Debug, Clone)]
/// Componentoutput.
pub struct ComponentOutput {
    /// Chunk.
    pub chunk: ChunkRecord,
    /// Direct child of the root that owns this recovered component.
    pub container_ordinal: usize,
    /// Name.
    pub name: String,
    /// Path.
    pub path: String,
    /// Payload format.
    pub payload_format: String,
    /// Schema ref.
    pub schema_ref: String,
    /// Recovery status.
    pub recovery_status: String,
}

/// Write lossless package.
///
/// # Errors
///
/// Returns an error when source parsing or filesystem output fails.
pub fn write_lossless_package(
    input_path: &Path,
    output_dir: &Path,
) -> Result<(), P3dError> {
    super::extractor::LosslessPackageExporter::write(
        input_path, output_dir,
    )
}

/// Package header.
#[must_use]
pub fn package_header(
    _input_path: &Path,
    document: &P3dDocument,
    component_count: usize,
) -> String {
    let mut json = String::from("{\"schema\":\"p3d.package.v1\",");
    json.push_str("\"byte_len\":");
    json.push_str(
        &document
            .byte_len
            .to_string(),
    );
    json.push_str(",\"chunk_count\":");
    json.push_str(
        &document
            .chunks
            .len()
            .to_string(),
    );
    json.push_str(",\"component_count\":");
    json.push_str(&component_count.to_string());
    json.push('}');
    json
}

/// Component line.
#[must_use]
pub fn component_line(component: &ComponentOutput) -> String {
    let chunk = &component.chunk;
    let mut json = String::from("{\"ordinal\":");
    json.push_str(
        &chunk
            .ordinal
            .to_string(),
    );
    json.push_str(",\"depth\":");
    json.push_str(
        &chunk
            .depth
            .to_string(),
    );
    json.push_str(",\"parent_ordinal\":");
    match chunk.parent_ordinal {
        Some(parent) => json.push_str(&parent.to_string()),
        None => json.push_str("null"),
    }
    json.push_str(",\"container_ordinal\":");
    json.push_str(
        &component
            .container_ordinal
            .to_string(),
    );
    json.push_str(",\"name\":\"");
    json.push_str(&escape(&component.name));
    json.push_str("\",\"path\":\"");
    json.push_str(&escape(&component.path));
    json.push_str("\",\"kind\":\"");
    json.push_str(
        chunk
            .kind
            .label(),
    );
    json.push_str("\",\"payload_format\":\"");
    json.push_str(&escape(&component.payload_format));
    json.push_str("\",\"schema_ref\":\"");
    json.push_str(&escape(&component.schema_ref));
    json.push_str("\",\"recovery_status\":\"");
    json.push_str(&escape(&component.recovery_status));
    json.push_str("\"}");
    json
}

/// Kind schema.
#[must_use]
pub fn kind_schema(kind: &str) -> &'static str {
    crate::schema::schema_ref_for_kind(kind).unwrap_or("unknown")
}

/// Escape one string for JSON without changing its decoded value.
fn escape(value: &str) -> String {
    let mut output = String::new();
    for character in value.chars() {
        if character == char::from(34) || character == char::from(92) {
            push_short_escape(
                &mut output,
                character,
            );
        } else if character == char::from(8) {
            push_short_escape(
                &mut output,
                'b',
            );
        } else if character == char::from(9) {
            push_short_escape(
                &mut output,
                't',
            );
        } else if character == char::from(10) {
            push_short_escape(
                &mut output,
                'n',
            );
        } else if character == char::from(12) {
            push_short_escape(
                &mut output,
                'f',
            );
        } else if character == char::from(13) {
            push_short_escape(
                &mut output,
                'r',
            );
        } else if character.is_control() {
            push_control_escape(
                &mut output,
                character,
            );
        } else {
            output.push(character);
        }
    }
    output
}

/// Append a two-character JSON escape.
fn push_short_escape(
    output: &mut String,
    escaped: char,
) {
    output.push(char::from(92));
    output.push(escaped);
}

/// Append the canonical four-digit JSON escape for a control character.
fn push_control_escape(
    output: &mut String,
    character: char,
) {
    let code = u32::from(character);
    output.push(char::from(92));
    output.push('u');
    output.push('0');
    output.push('0');
    push_hex_digit(
        output,
        (code >> 4) & 15_u32,
    );
    push_hex_digit(
        output,
        code & 15_u32,
    );
}

/// Append one lowercase hexadecimal digit.
fn push_hex_digit(
    output: &mut String,
    value: u32,
) {
    let digit = match value {
        0 => '0',
        1 => '1',
        2 => '2',
        3 => '3',
        4 => '4',
        5 => '5',
        6 => '6',
        7 => '7',
        8 => '8',
        9 => '9',
        10 => 'a',
        11 => 'b',
        12 => 'c',
        13 => 'd',
        14 => 'e',
        15 => 'f',
        _ => return,
    };
    output.push(digit);
}
