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
//   - Extractor outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Extractor outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for p3d.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Extractor outbound adapter.

#![expect(
    clippy::doc_markdown,
    reason = "Tests verify these intentional explicit file-local contracts \
              remain safe."
)]
// Binary recovery keeps exact offsets and bounded legacy conversion behavior
// local.
#![expect(
    clippy::arithmetic_side_effects,
    clippy::indexing_slicing,
    clippy::as_conversions,
    clippy::missing_const_for_fn,
    reason = "P3D binary parser code mirrors fixed on-disk offsets and \
              generated chunk taxonomy; follow-up tranche work replaces stubs \
              with typed decoders."
)]
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use schoenwald_filesystem::PathKind;
use schoenwald_filesystem::adapters::driving::local;
use shar_sha256::digest_hex;

use super::expression::vertex_expression_json;
use super::image::detect_image_extension;
use super::json::{
    escape_json, render_f32, validate_document, validate_json_lines,
};
use super::package::{
    ComponentOutput, component_line, kind_schema, package_header,
};
use crate::domain::prepare_p3d_bytes;
use crate::{ChunkRecord, P3dError, analyze_p3d};

#[derive(Debug, Clone, Copy)]
/// Exports one validated Pure3D package without dropping decoded evidence.
pub struct LosslessPackageExporter;

/// Recoveredcomponent.
pub(super) struct RecoveredComponent {
    /// Name.
    name: String,
    /// Relative path.
    relative_path: PathBuf,
    /// Bytes.
    bytes: Vec<u8>,
    /// Payload format.
    payload_format: String,
    /// Recovery status.
    recovery_status: String,
}

impl LosslessPackageExporter {
    /// Write.
    ///
    /// # Errors
    ///
    /// Returns an error when source parsing or filesystem output fails.
    pub fn write(input_path: &Path, output_dir: &Path) -> Result<(), P3dError> {
        let input_bytes = local::read_bytes(input_path)
            .map_err(|error| P3dError::invalid_source(error.to_string()))?;
        let source_sha256 = digest_hex(&input_bytes);
        let prepared = prepare_p3d_bytes(&input_bytes)?;
        let bytes = prepared.bytes.into_owned();
        let normalized_sha256 = digest_hex(&bytes);
        let document = analyze_p3d(&bytes)?;
        local::write_bytes(&output_dir.join("source.p3d"), &bytes, true)
            .map_err(|error| P3dError::invalid_source(error.to_string()))?;
        let components_dir = reset_components_directory(output_dir)?;
        let mut kind_counts = BTreeMap::<&'static str, usize>::new();
        let mut published_paths = BTreeMap::<String, (PathBuf, Vec<u8>)>::new();
        let mut outputs = Vec::new();
        for component in document
            .chunks
            .iter()
            .filter(|chunk| should_publish_component(chunk, &document.chunks))
        {
            let kind = component.kind.label();
            let next_index = kind_counts.entry(kind).or_insert(0);
            *next_index += 1;
            let mut recovered =
                recover_component(component, &bytes, *next_index)?;
            if !register_recovered_path(
                &mut published_paths,
                component,
                &mut recovered,
            )? {
                continue;
            }
            outputs.push(publish_recovered_component(
                component,
                top_level_ancestor_ordinal(component, &document.chunks)?,
                kind,
                recovered,
                &bytes,
                &components_dir,
            )?);
        }
        let mut lines = String::new();
        lines.push_str(&package_header(
            &document,
            outputs.len(),
            &source_sha256,
            &normalized_sha256,
        ));
        lines.push('\n');
        for output in &outputs {
            lines.push_str(&component_line(output));
            lines.push('\n');
        }
        validate_json_lines(&lines, "components.jsonl")?;
        local::write_text(&output_dir.join("components.jsonl"), &lines, true)
            .map_err(|error| P3dError::invalid_source(error.to_string()))?;
        Ok(())
    }
}

/// Register one recovered component under a portable output-path identity.
///
/// Byte-identical nested repeats of the exact same path are references to the
/// already-published component. Other byte-identical duplicate identities
/// retain their source ordinal under a deterministic qualified physical path.
/// Semantic consumers reject ambiguous same-name payloads when selecting one.
fn register_recovered_path(
    published_paths: &mut BTreeMap<String, (PathBuf, Vec<u8>)>,
    component: &ChunkRecord,
    recovered: &mut RecoveredComponent,
) -> Result<bool, P3dError> {
    let identity = portable_path_identity(&recovered.relative_path)?;
    if let Some((existing_path, existing_bytes)) =
        published_paths.get(&identity)
    {
        if existing_path == &recovered.relative_path
            && existing_bytes == &recovered.bytes
            && component.parent_ordinal != Some(0)
        {
            return Ok(false);
        }
        let alias = ordinal_qualified_path(
            &recovered.relative_path,
            component.ordinal,
        )?;
        let alias_identity = portable_path_identity(&alias)?;
        if published_paths.contains_key(&alias_identity) {
            return Err(P3dError::invalid_source(format!(
                "recovered component alias path already exists: {}",
                alias.display()
            )));
        }
        recovered.relative_path = alias;
        drop(published_paths.insert(
            alias_identity,
            (recovered.relative_path.clone(), recovered.bytes.clone()),
        ));
        return Ok(true);
    }
    drop(published_paths.insert(
        identity,
        (recovered.relative_path.clone(), recovered.bytes.clone()),
    ));
    Ok(true)
}

/// Return one case-insensitive portable identity for a generated path.
fn portable_path_identity(path: &Path) -> Result<String, P3dError> {
    let relative = path.to_str().ok_or_else(|| {
        P3dError::invalid_source(
            "recovered component path is not valid Unicode",
        )
    })?;
    Ok(relative
        .chars()
        .flat_map(char::to_uppercase)
        .collect::<String>())
}

/// Add a source-ordinal suffix before the generated file extension.
fn ordinal_qualified_path(
    path: &Path,
    ordinal: usize,
) -> Result<PathBuf, P3dError> {
    let parent = path.parent().unwrap_or_else(|| Path::new(""));
    let stem = path
        .file_stem()
        .and_then(|value| value.to_str())
        .ok_or_else(|| {
            P3dError::invalid_source(
                "recovered component alias path has no portable file stem",
            )
        })?;
    let extension = path
        .extension()
        .and_then(|value| value.to_str())
        .ok_or_else(|| {
            P3dError::invalid_source(
                "recovered component alias path has no portable extension",
            )
        })?;
    Ok(parent.join(format!("{stem}__ordinal_{ordinal:04}.{extension}")))
}

/// Recreate the normalized component directory for one package export.
fn reset_components_directory(output_dir: &Path) -> Result<PathBuf, P3dError> {
    let components_dir = output_dir.join("components");
    if local::path_kind(&components_dir)
        .map_err(|error| P3dError::invalid_source(error.to_string()))?
        != PathKind::Missing
    {
        fs::remove_dir_all(&components_dir)
            .map_err(|error| P3dError::invalid_source(error.to_string()))?;
    }
    local::create_dir_all(&components_dir)
        .map_err(|error| P3dError::invalid_source(error.to_string()))?;
    Ok(components_dir)
}

/// Return whether one parsed chunk belongs in the normalized component set.
fn should_publish_component(
    component: &ChunkRecord,
    chunks: &[ChunkRecord],
) -> bool {
    if component.parent_ordinal == Some(0) {
        return true;
    }
    let kind = component.kind.label();
    if is_nested_model_support(kind) {
        return has_ancestor(component, chunks, is_model_container);
    }
    if kind == "texture" {
        return has_ancestor(component, chunks, |ancestor| {
            matches!(ancestor, "srr_chunk_set" | "texture_font")
        });
    }
    if kind == "image" {
        return component
            .parent_ordinal
            .and_then(|ordinal| chunks.get(ordinal))
            .is_some_and(|parent| parent.kind.label() == "sprite");
    }
    false
}

/// Return whether one component has an ancestor accepted by one predicate.
fn has_ancestor(
    component: &ChunkRecord,
    chunks: &[ChunkRecord],
    accepts: impl Fn(&str) -> bool,
) -> bool {
    let mut parent = component.parent_ordinal;
    while let Some(ordinal) = parent {
        let Some(ancestor) = chunks.get(ordinal) else {
            return false;
        };
        if accepts(ancestor.kind.label()) {
            return true;
        }
        parent = ancestor.parent_ordinal;
    }
    false
}

/// Return whether one nested family carries model or model-animation evidence.
fn is_nested_model_support(kind: &str) -> bool {
    matches!(
        kind,
        "mesh"
            | "skin"
            | "animation"
            | "skeleton"
            | "composite_drawable"
            | "quad_group"
            | "multi_controller"
            | "frame_controller"
            | "frame_controller_variant_a"
            | "frame_controller_variant_b"
    )
}

/// Return whether one ancestor owns an embedded world or prop presentation.
fn is_model_container(kind: &str) -> bool {
    matches!(
        kind,
        "srr_entity_dsg"
            | "srr_insta_entity_dsg"
            | "srr_dyna_phys_dsg"
            | "srr_insta_anim_dyna_phys_dsg"
            | "srr_static_phys_dsg"
            | "srr_insta_static_phys_dsg"
            | "srr_anim_dsg"
            | "srr_anim_coll_dsg"
            | "srr_breakable_object"
            | "state_prop"
            | "animated_object_factory"
    )
}

/// Resolve the direct root child that owns one recovered component.
fn top_level_ancestor_ordinal(
    component: &ChunkRecord,
    chunks: &[ChunkRecord],
) -> Result<usize, P3dError> {
    let mut current = component;
    loop {
        match current.parent_ordinal {
            Some(0) | None => return Ok(current.ordinal),
            Some(parent) => {
                current = chunks.get(parent).ok_or_else(|| {
                    P3dError::invalid_source(
                        "component ancestry references an invalid \
                                 ordinal",
                    )
                })?;
            },
        }
    }
}

/// Validate and publish one recovered component plus optional metadata.
fn publish_recovered_component(
    component: &ChunkRecord,
    container_ordinal: usize,
    kind: &str,
    recovered: RecoveredComponent,
    source: &[u8],
    components_dir: &Path,
) -> Result<ComponentOutput, P3dError> {
    let path = components_dir.join(&recovered.relative_path);
    if recovered
        .relative_path
        .extension()
        .and_then(|value| value.to_str())
        .is_some_and(|extension| extension.eq_ignore_ascii_case("json"))
    {
        validate_document(
            &recovered.bytes,
            &recovered.relative_path.to_string_lossy(),
        )?;
    }
    local::write_bytes(&path, &recovered.bytes, true)
        .map_err(|error| P3dError::invalid_source(error.to_string()))?;
    let artifact_sha256 = digest_hex(&recovered.bytes);
    if kind == "texture"
        && let Some(metadata) = texture_metadata_json(component, source)
    {
        let metadata_path = path.with_extension("metadata.json");
        validate_document(
            metadata.as_bytes(),
            &metadata_path.to_string_lossy(),
        )?;
        local::write_text(&metadata_path, &metadata, true)
            .map_err(|error| P3dError::invalid_source(error.to_string()))?;
    }
    Ok(ComponentOutput {
        chunk: *component,
        container_ordinal,
        name: recovered.name,
        // Record a portable path so provenance is stable on every OS.
        path: recovered
            .relative_path
            .to_string_lossy()
            .replace(char::from(92), "/"),
        payload_format: recovered.payload_format,
        schema_ref: kind_schema(kind).to_owned(),
        recovery_status: recovered.recovery_status,
        sha256: artifact_sha256,
    })
}

impl crate::ports::PackageExporter for LosslessPackageExporter {
    type Error = P3dError;

    fn export_package(
        &self,
        input_path: &Path,
        output_dir: &Path,
    ) -> Result<(), Self::Error> {
        Self::write(input_path, output_dir)
    }
}

/// Recover component.
fn recover_component(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Result<RecoveredComponent, P3dError> {
    if component.kind.label() == "texture"
        && let Some(recovered) = recover_texture(component, source)?
    {
        return Ok(recovered);
    }
    if component.kind.label() == "image"
        && let Some(recovered) = recover_image(component, source)?
    {
        return Ok(recovered);
    }
    if let Some(recovered) =
        schema::recover_schema_json(component, source, kind_index)
    {
        return Ok(recovered);
    }
    Err(P3dError::invalid_source(format!(
        "missing decoder for Pure3D component kind {}",
        component.kind.label()
    )))
}

/// Recover texture.
fn recover_texture(
    component: &ChunkRecord,
    source: &[u8],
) -> Result<Option<RecoveredComponent>, P3dError> {
    let name = component_name(component, source, 0);
    let chunk = raw_component_bytes(component, source)?;
    let Some(image_payload) = extract_first_image_payload(chunk) else {
        return Ok(None);
    };
    let Some(extension) = detect_image_extension(image_payload) else {
        return Ok(None);
    };
    let file_stem = strip_known_image_extension(&sanitize(&name));
    Ok(Some(RecoveredComponent {
        relative_path: PathBuf::from("texture")
            .join(format!("{file_stem}.{extension}")),
        name,
        bytes: image_payload.to_vec(),
        payload_format: format!("image/{extension}"),
        recovery_status: "recovered_embedded_image_payload".to_owned(),
    }))
}

/// Recover one embedded image child as its exact physical payload.
fn recover_image(
    component: &ChunkRecord,
    source: &[u8],
) -> Result<Option<RecoveredComponent>, P3dError> {
    let name = component_name(component, source, 0);
    let chunk = raw_component_bytes(component, source)?;
    let Some(image_payload) = extract_image_payload(chunk) else {
        return Ok(None);
    };
    let Some(extension) = detect_image_extension(image_payload) else {
        return Ok(None);
    };
    let file_stem = strip_known_image_extension(&sanitize(&name));
    Ok(Some(RecoveredComponent {
        relative_path: PathBuf::from("image")
            .join(format!("{file_stem}.{extension}")),
        name,
        bytes: image_payload.to_vec(),
        payload_format: format!("image/{extension}"),
        recovery_status: "recovered_embedded_image_payload".to_owned(),
    }))
}

/// Decode texture metadata sidecar JSON.
fn texture_metadata_json(
    component: &ChunkRecord,
    source: &[u8],
) -> Option<String> {
    let chunk = raw_component_bytes(component, source).ok()?;
    let mut cursor = 12;
    let name = schema::read_pascal_at(chunk, &mut cursor)?;
    let version = read_u32(chunk, cursor)?;
    cursor += 4;
    let width = read_u32(chunk, cursor)?;
    cursor += 4;
    let height = read_u32(chunk, cursor)?;
    cursor += 4;
    let bpp = read_u32(chunk, cursor)?;
    cursor += 4;
    let alpha_depth = read_u32(chunk, cursor)?;
    cursor += 4;
    let mip_count = read_u32(chunk, cursor)?;
    cursor += 4;
    let texture_type = read_u32(chunk, cursor)?;
    cursor += 4;
    let usage = read_u32(chunk, cursor)?;
    cursor += 4;
    let priority = read_u32(chunk, cursor)?;
    let children = auxiliary::child_chunks_json(
        chunk,
        component.header_size,
        component.total_size,
    );
    Some(format!(
        concat!(
            r#"{{"schema":"texture_metadata","#,
            r#""name":"{}","#,
            r#""version":{},"#,
            r#""width":{},"#,
            r#""height":{},"#,
            r#""bpp":{},"#,
            r#""alpha_depth":{},"#,
            r#""mip_count":{},"#,
            r#""texture_type":{},"#,
            r#""usage":{},"#,
            r#""priority":{},"#,
            r#""children":[{}]}}"#,
        ),
        escape_json(&name),
        version,
        width,
        height,
        bpp,
        alpha_depth,
        mip_count,
        texture_type,
        usage,
        priority,
        children
    ))
}

/// Extract first image payload.
fn extract_first_image_payload(texture_chunk: &[u8]) -> Option<&[u8]> {
    let (_, texture_header, texture_total) =
        read_chunk_header(texture_chunk, 0)?;
    let mut cursor = texture_header;
    while cursor + 12 <= texture_total {
        let (child_id, _child_header, child_total) =
            read_chunk_header(texture_chunk, cursor)?;
        if child_id == 0x0001_9001 {
            let child_end = cursor.checked_add(child_total)?;
            let child = texture_chunk.get(cursor..child_end)?;
            return extract_image_payload(child);
        }
        cursor += child_total;
    }
    None
}

/// Extract the exact payload from one `IMAGE` chunk's `IMAGE_DATA` child.
fn extract_image_payload(image_chunk: &[u8]) -> Option<&[u8]> {
    let (image_id, image_header, image_total) =
        read_chunk_header(image_chunk, 0)?;
    if image_id != 0x0001_9001 || image_total != image_chunk.len() {
        return None;
    }
    let mut cursor = image_header;
    while cursor + 12 <= image_total {
        let (child_id, child_header, child_total) =
            read_chunk_header(image_chunk, cursor)?;
        let child_end = cursor.checked_add(child_total)?;
        if child_end > image_total {
            return None;
        }
        if child_id == 0x0001_9002 {
            if child_header != child_total || child_header < 16 {
                return None;
            }
            let size_offset = cursor.checked_add(12)?;
            let payload_size =
                usize::try_from(read_u32(image_chunk, size_offset)?).ok()?;
            let payload_start = size_offset.checked_add(4)?;
            let payload_end = payload_start.checked_add(payload_size)?;
            if payload_end != child_end {
                return None;
            }
            return image_chunk.get(payload_start..payload_end);
        }
        cursor = child_end;
    }
    None
}

/// Raw component bytes.
fn raw_component_bytes<'a>(
    component: &ChunkRecord,
    source: &'a [u8],
) -> Result<&'a [u8], P3dError> {
    let end = component.offset + component.total_size;
    source.get(component.offset..end).ok_or_else(|| {
        P3dError::invalid_source("component slice out of bounds")
    })
}

/// Component name.
fn component_name(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> String {
    read_pascal_name(component, source).unwrap_or_else(|| {
        format!("{}_{kind_index:04}", component.kind.label())
    })
}

/// Read pascal name.
fn read_pascal_name(component: &ChunkRecord, source: &[u8]) -> Option<String> {
    let start = component.offset + 12;
    let end = component.offset + component.header_size;
    let payload = source.get(start..end)?;
    let length = usize::from(*payload.first()?);
    if length == 0 || length > 96 || payload.len() < 1 + length {
        return None;
    }
    let raw = payload.get(1..1 + length)?;
    if !raw
        .iter()
        .all(|byte| *byte == 0 || (32..=126).contains(byte))
    {
        return None;
    }
    let name = String::from_utf8_lossy(raw)
        .trim_matches(char::from(0))
        .to_owned();
    if name.is_empty() {
        None
    } else {
        Some(name)
    }
}

/// Read chunk header.
fn read_chunk_header(
    bytes: &[u8],
    offset: usize,
) -> Option<(u32, usize, usize)> {
    Some((
        read_u32(bytes, offset)?,
        read_u32(bytes, offset + 4)? as usize,
        read_u32(bytes, offset + 8)? as usize,
    ))
}

/// Read u32.
fn read_u32(bytes: &[u8], offset: usize) -> Option<u32> {
    let end = offset.checked_add(4)?;
    let slice = bytes.get(offset..end)?;
    Some(u32::from_le_bytes([slice[0], slice[1], slice[2], slice[3]]))
}

/// Strip known image extension.
fn strip_known_image_extension(value: &str) -> String {
    for extension in [".bmp", ".png", ".tga", ".dds"] {
        if let Some(stripped) = value.strip_suffix(extension) {
            return stripped.to_owned();
        }
    }
    value.to_owned()
}

/// Sanitize.
fn sanitize(value: &str) -> String {
    let mut output = String::new();
    for character in value.chars() {
        if character.is_ascii_alphanumeric()
            || matches!(character, '.' | '-' | '_')
        {
            output.push(character);
        } else {
            output.push('_');
        }
    }
    if output.is_empty() {
        "component".to_owned()
    } else {
        output
    }
}

mod auxiliary;
mod render;
mod schema;
mod schema_recovery;

#[cfg(test)]
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../../tests/formats/p3d/unit/adapter-outbound/extractor/loose_tests.rs"]
mod loose_tests;
#[cfg(test)]
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../../tests/formats/p3d/unit/adapter-outbound/extractor/nested_model_component_tests.rs"]
mod nested_model_component_tests;
