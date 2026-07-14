// File:
//   - extractor.rs
// Path:
//   - src/p3d/src/adapters/driven/extractor.rs
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
//   - The p3d adapter boundary for adapters driven extractor.
// - Must-Not:
//   - Change domain semantics or bypass application and port contracts.
// - Allows:
//   - Filesystem, JSON, CLI, Blender, or serialization work behind explicit
//   - ports.
// - Split-When:
//   - Split when extractor contains two independently testable contracts.
// - Merge-When:
//   - Another p3d module owns the same adapters boundary with no distinct
//   - invariant.
// - Summary:
//   - Losslesspackagewriter.
// - Description:
//   - Defines extractor data and behavior for p3d adapters driven.
// - Usage:
//   - Constructed by composition roots or tests and passed behind port traits.
// - Defaults:
//   - Adapter defaults stay local to the adapter constructor or config.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - true
//   - Reason: src/p3d/src/adapters/driven/extractor.rs has 4085 effective lines
//   - after the required header and remains cohesive until a focused split
//   - lands.
//

//! Losslesspackagewriter.
//!
//! This boundary keeps losslesspackagewriter explicit and returns
//! deterministic results to p3d callers.
// These exact file-local lints preserve explicit domain and binary contracts.
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
    pub fn write(
        input_path: &Path,
        output_dir: &Path,
    ) -> Result<(), P3dError> {
        let input_bytes = local::read_bytes(input_path)
            .map_err(|error| P3dError::invalid_source(error.to_string()))?;
        let prepared = prepare_p3d_bytes(&input_bytes)?;
        let bytes = prepared
            .bytes
            .into_owned();
        let document = analyze_p3d(&bytes)?;
        local::write_bytes(
            &output_dir.join("source.p3d"),
            &bytes,
            true,
        )
        .map_err(|error| P3dError::invalid_source(error.to_string()))?;
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
        let mut kind_counts = BTreeMap::<&'static str, usize>::new();
        let mut outputs = Vec::new();
        for component in document
            .chunks
            .iter()
            .filter(|chunk| chunk.parent_ordinal == Some(0))
        {
            let kind = component
                .kind
                .label();
            let next_index = kind_counts
                .entry(kind)
                .or_insert(0);
            *next_index += 1;
            let recovered = recover_component(
                component,
                &bytes,
                *next_index,
            )?;
            outputs.push(
                publish_recovered_component(
                    component,
                    kind,
                    recovered,
                    &bytes,
                    &components_dir,
                )?,
            );
        }
        let mut lines = String::new();
        lines.push_str(
            &package_header(
                input_path,
                &document,
                outputs.len(),
            ),
        );
        lines.push('\n');
        for output in &outputs {
            lines.push_str(&component_line(output));
            lines.push('\n');
        }
        validate_json_lines(
            &lines,
            "components.jsonl",
        )?;
        local::write_text(
            &output_dir.join("components.jsonl"),
            &lines,
            true,
        )
        .map_err(|error| P3dError::invalid_source(error.to_string()))?;
        Ok(())
    }
}

/// Validate and publish one recovered component plus optional metadata.
fn publish_recovered_component(
    component: &ChunkRecord,
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
            &recovered
                .relative_path
                .to_string_lossy(),
        )?;
    }
    local::write_bytes(
        &path,
        &recovered.bytes,
        true,
    )
    .map_err(|error| P3dError::invalid_source(error.to_string()))?;
    if kind == "texture"
        && let Some(metadata) = texture_metadata_json(
            component, source,
        )
    {
        let metadata_path = path.with_extension("metadata.json");
        validate_document(
            metadata.as_bytes(),
            &metadata_path.to_string_lossy(),
        )?;
        local::write_text(
            &metadata_path,
            &metadata,
            true,
        )
        .map_err(|error| P3dError::invalid_source(error.to_string()))?;
    }
    Ok(
        ComponentOutput {
            chunk: *component,
            name: recovered.name,
            // Record a portable path so provenance is stable on every OS.
            path: recovered
                .relative_path
                .to_string_lossy()
                .replace(
                    char::from(92),
                    "/",
                ),
            payload_format: recovered.payload_format,
            schema_ref: kind_schema(kind).to_owned(),
            recovery_status: recovered.recovery_status,
        },
    )
}

impl crate::ports::PackageExporter for LosslessPackageExporter {
    type Error = P3dError;

    fn export_package(
        &self,
        input_path: &Path,
        output_dir: &Path,
    ) -> Result<(), Self::Error> {
        Self::write(
            input_path, output_dir,
        )
    }
}

/// Recover component.
fn recover_component(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Result<RecoveredComponent, P3dError> {
    if component
        .kind
        .label()
        == "texture"
        && let Some(recovered) = recover_texture(
            component, source,
        )?
    {
        return Ok(recovered);
    }
    if let Some(recovered) = recover_schema_json(
        component, source, kind_index,
    ) {
        return Ok(recovered);
    }
    Err(
        P3dError::invalid_source(
            format!(
                "missing decoder for Pure3D component kind {}",
                component
                    .kind
                    .label()
            ),
        ),
    )
}

/// Recover texture.
fn recover_texture(
    component: &ChunkRecord,
    source: &[u8],
) -> Result<Option<RecoveredComponent>, P3dError> {
    let name = component_name(
        component, source, 0,
    );
    let chunk = raw_component_bytes(
        component, source,
    )?;
    let Some(image_payload) = extract_first_image_payload(chunk) else {
        return Ok(None);
    };
    let Some(extension) = detect_image_extension(image_payload) else {
        return Ok(None);
    };
    let file_stem = strip_known_image_extension(&sanitize(&name));
    Ok(
        Some(
            RecoveredComponent {
                relative_path: PathBuf::from("texture")
                    .join(format!("{file_stem}.{extension}")),
                name,
                bytes: image_payload.to_vec(),
                payload_format: format!("image/{extension}"),
                recovery_status: "recovered_embedded_image_payload".to_owned(),
            },
        ),
    )
}

/// Decode texture metadata sidecar JSON.
fn texture_metadata_json(
    component: &ChunkRecord,
    source: &[u8],
) -> Option<String> {
    let chunk = raw_component_bytes(
        component, source,
    )
    .ok()?;
    let mut cursor = 12;
    let name = read_pascal_at(
        chunk,
        &mut cursor,
    )?;
    let version = read_u32(
        chunk, cursor,
    )?;
    cursor += 4;
    let width = read_u32(
        chunk, cursor,
    )?;
    cursor += 4;
    let height = read_u32(
        chunk, cursor,
    )?;
    cursor += 4;
    let bpp = read_u32(
        chunk, cursor,
    )?;
    cursor += 4;
    let alpha_depth = read_u32(
        chunk, cursor,
    )?;
    cursor += 4;
    let mip_count = read_u32(
        chunk, cursor,
    )?;
    cursor += 4;
    let texture_type = read_u32(
        chunk, cursor,
    )?;
    cursor += 4;
    let usage = read_u32(
        chunk, cursor,
    )?;
    cursor += 4;
    let priority = read_u32(
        chunk, cursor,
    )?;
    let children = child_chunks_json(
        chunk,
        component.header_size,
        component.total_size,
    );
    Some(
        format!(
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
        ),
    )
}

/// Extract first image payload.
fn extract_first_image_payload(texture_chunk: &[u8]) -> Option<&[u8]> {
    let (_, texture_header, texture_total) = read_chunk_header(
        texture_chunk,
        0,
    )?;
    let mut cursor = texture_header;
    while cursor + 12 <= texture_total {
        let (child_id, child_header, child_total) = read_chunk_header(
            texture_chunk,
            cursor,
        )?;
        if child_id == 0x0001_9001 {
            let mut sub = cursor + child_header;
            let child_end = cursor + child_total;
            while sub + 12 <= child_end {
                let (sub_id, _sub_header, sub_total) = read_chunk_header(
                    texture_chunk,
                    sub,
                )?;
                if sub_id == 0x0001_9002 {
                    let size_offset = sub + 12;
                    let payload_size = read_u32(
                        texture_chunk,
                        size_offset,
                    )? as usize;
                    let payload_start = size_offset + 4;
                    let payload_end =
                        payload_start.checked_add(payload_size)?;
                    return texture_chunk.get(payload_start..payload_end);
                }
                sub += sub_total;
            }
        }
        cursor += child_total;
    }
    None
}

/// Raw component bytes.
fn raw_component_bytes<'a>(
    component: &ChunkRecord,
    source: &'a [u8],
) -> Result<&'a [u8], P3dError> {
    let end = component.offset + component.total_size;
    source
        .get(component.offset..end)
        .ok_or_else(
            || P3dError::invalid_source("component slice out of bounds"),
        )
}

/// Component name.
fn component_name(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> String {
    read_pascal_name(
        component, source,
    )
    .unwrap_or_else(
        || {
            format!(
                "{}_{kind_index:04}",
                component
                    .kind
                    .label()
            )
        },
    )
}

/// Read pascal name.
fn read_pascal_name(
    component: &ChunkRecord,
    source: &[u8],
) -> Option<String> {
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
) -> Option<(
    u32,
    usize,
    usize,
)> {
    Some(
        (
            read_u32(
                bytes, offset,
            )?,
            read_u32(
                bytes,
                offset + 4,
            )? as usize,
            read_u32(
                bytes,
                offset + 8,
            )? as usize,
        ),
    )
}

/// Read u32.
fn read_u32(
    bytes: &[u8],
    offset: usize,
) -> Option<u32> {
    let end = offset.checked_add(4)?;
    let slice = bytes.get(offset..end)?;
    Some(
        u32::from_le_bytes(
            [
                slice[0], slice[1], slice[2], slice[3],
            ],
        ),
    )
}

/// Strip known image extension.
fn strip_known_image_extension(value: &str) -> String {
    for extension in [
        ".bmp", ".png", ".tga", ".dds",
    ] {
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
            || matches!(
                character,
                '.' | '-' | '_'
            )
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

/// Recovers one schema through the first matching decoder family.
fn recover_schema_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    recover_world_schema_json(
        component, source, kind_index,
    )
    .or_else(
        || {
            super::schema_recovery::recover_render_schema_json(
                component, source, kind_index,
            )
        },
    )
    .or_else(
        || {
            recover_auxiliary_schema_json(
                component, source, kind_index,
            )
        },
    )
}

/// Routes world, simulation, and foundational schema families.
fn recover_world_schema_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    match component
        .kind
        .label()
    {
        "text_bible" => recover_text_bible_json(
            component, source, kind_index,
        ),
        "shader" => recover_shader_json(
            component, source, kind_index,
        ),
        "game_attr" => recover_game_attr_json(
            component, source, kind_index,
        ),
        "light" => recover_light_json(
            component, source, kind_index,
        ),
        "srr_locator" => recover_srr_locator_json(
            component, source, kind_index,
        ),
        "srr_fence_dsg" => recover_fence_json(
            component, source, kind_index,
        ),
        "srr_entity_dsg" => recover_entity_dsg_json(
            component, source, kind_index,
        ),
        "srr_insta_entity_dsg" => recover_insta_entity_dsg_json(
            component, source, kind_index,
        ),
        "srr_dyna_phys_dsg"
        | "srr_insta_anim_dyna_phys_dsg"
        | "srr_static_phys_dsg"
        | "srr_insta_static_phys_dsg" => recover_physics_dsg_json(
            component, source, kind_index,
        ),
        "srr_anim_dsg" | "srr_anim_coll_dsg" => {
            recover_name_version_alpha_json(
                component, source, kind_index,
            )
        }
        "srr_road_segment_data" => recover_road_segment_json(
            component, source, kind_index,
        ),
        "srr_road" => recover_road_json(
            component, source, kind_index,
        ),
        "srr_intersection" => recover_intersection_json(
            component, source, kind_index,
        ),
        "srr_tree_dsg" => recover_tree_json(
            component, source, kind_index,
        ),
        "srr_intersect_dsg" => recover_intersect_json(
            component, source, kind_index,
        ),
        "simulation_collision_object" => recover_collision_object_json(
            component, source, kind_index,
        ),
        "simulation_physics_object" => recover_physics_object_json(
            component, source, kind_index,
        ),
        "srr_chunk_set" => recover_chunk_set_json(
            component, source, kind_index,
        ),
        _ => None,
    }
}

/// Routes auxiliary, UI, and remaining specialized schema families.
fn recover_auxiliary_schema_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    match component
        .kind
        .label()
    {
        "locator" => recover_locator_json(
            component, source, kind_index,
        ),
        "srr_ped_path" => recover_ped_path_json(
            component, source, kind_index,
        ),
        "srr_follow_cam" => recover_follow_cam_json(
            component, source, kind_index,
        ),
        "export_info" => recover_export_info_json(
            component, source, kind_index,
        ),
        "srr_breakable_object" => recover_breakable_object_json(
            component, source, kind_index,
        ),
        "srr_attribute_table" => recover_attribute_table_json(
            component, source, kind_index,
        ),
        "srr_lens_flare_dsg" => recover_lens_flare_json(
            component, source, kind_index,
        ),
        "animated_object" => recover_animated_object_json(
            component, source, kind_index,
        ),
        "animated_object_factory" => recover_animated_object_factory_json(
            component, source, kind_index,
        ),
        "state_prop" => recover_state_prop_json(
            component, source, kind_index,
        ),
        "vertex_expression_group" | "vertex_expression_mixer" => {
            recover_vertex_expression_json(
                component, source, kind_index,
            )
        }
        "quad_group" => recover_quad_group_json(
            component, source, kind_index,
        ),
        "texture_font" => recover_texture_font_json(
            component, source, kind_index,
        ),
        "scrooby_project" => recover_scrooby_project_json(
            component, source, kind_index,
        ),
        "srr_inst_particle_system" => recover_inst_particle_system_json(
            component, source, kind_index,
        ),
        _ => None,
    }
}

/// Recover shader json.
fn recover_shader_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(
        component, source,
    )
    .ok()?;
    let mut cursor = 12;
    let name = read_pascal_at(
        chunk,
        &mut cursor,
    )?;
    let version = read_u32(
        chunk, cursor,
    )?;
    cursor += 4;
    let pddi_shader_name = read_pascal_at(
        chunk,
        &mut cursor,
    )?;
    let has_translucency = read_u32(
        chunk, cursor,
    )?;
    cursor += 4;
    let vertex_needs = read_u32(
        chunk, cursor,
    )?;
    cursor += 4;
    let vertex_mask = read_u32(
        chunk, cursor,
    )?;
    cursor += 4;
    let num_params = read_u32(
        chunk, cursor,
    )?;
    let fallback = format!("shader_{kind_index:04}");
    let file_name = sanitize(
        if name.is_empty() {
            &fallback
        } else {
            &name
        },
    );
    let params = shader_params_json(
        chunk,
        component.header_size,
        component.total_size,
    );
    let json = format!(
        concat!(
            r#"{{"schema":"shader","#,
            r#""name":"{}","#,
            r#""version":{},"#,
            r#""pddi_shader_name":"{}","#,
            r#""has_translucency":{},"#,
            r#""vertex_needs":{},"#,
            r#""vertex_mask":{},"#,
            r#""num_params":{},"#,
            r#""params":[{}]}}"#,
        ),
        escape_json(&name),
        version,
        escape_json(&pddi_shader_name),
        has_translucency,
        vertex_needs,
        vertex_mask,
        num_params,
        params
    );
    Some(
        RecoveredComponent {
            relative_path: PathBuf::from("shader")
                .join(format!("{file_name}.json")),
            name,
            bytes: json.into_bytes(),
            payload_format: "schema_json".to_owned(),
            recovery_status: "decoded_schema_payload".to_owned(),
        },
    )
}

/// Recover game attr json.
fn recover_game_attr_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(
        component, source,
    )
    .ok()?;
    let mut cursor = 12;
    let name = read_pascal_at(
        chunk,
        &mut cursor,
    )?;
    let version = read_u32(
        chunk, cursor,
    )?;
    cursor += 4;
    let num_params = read_u32(
        chunk, cursor,
    )?;
    let fallback = format!("game_attr_{kind_index:04}");
    let file_name = sanitize(
        if name.is_empty() {
            &fallback
        } else {
            &name
        },
    );
    let params = game_attr_params_json(
        chunk,
        component.header_size,
        component.total_size,
    );
    let json = format!(
        concat!(
            r#"{{"schema":"game_attr","#,
            r#""name":"{}","#,
            r#""version":{},"#,
            r#""num_params":{},"#,
            r#""params":[{}]}}"#,
        ),
        escape_json(&name),
        version,
        num_params,
        params
    );
    Some(
        RecoveredComponent {
            relative_path: PathBuf::from("game_attr")
                .join(format!("{file_name}.json")),
            name,
            bytes: json.into_bytes(),
            payload_format: "schema_json".to_owned(),
            recovery_status: "decoded_schema_payload".to_owned(),
        },
    )
}

/// Recover light json.
fn recover_light_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(
        component, source,
    )
    .ok()?;
    let mut cursor = 12;
    let name = read_pascal_at(
        chunk,
        &mut cursor,
    )?;
    let version = read_u32(
        chunk, cursor,
    )?;
    cursor += 4;
    let light_type = read_u32(
        chunk, cursor,
    )?;
    cursor += 4;
    let colour = read_u32(
        chunk, cursor,
    )?;
    cursor += 4;
    let constant = read_f32(
        chunk, cursor,
    )?;
    cursor += 4;
    let linear = read_f32(
        chunk, cursor,
    )?;
    cursor += 4;
    let squared = read_f32(
        chunk, cursor,
    )?;
    cursor += 4;
    let enabled = read_u32(
        chunk, cursor,
    )?;
    let fallback = format!("light_{kind_index:04}");
    let file_name = sanitize(
        if name.is_empty() {
            &fallback
        } else {
            &name
        },
    );
    let extras = light_children_json(
        chunk,
        component.header_size,
        component.total_size,
    );
    let json = format!(
        concat!(
            r#"{{"schema":"light","#,
            r#""name":"{}","#,
            r#""version":{},"#,
            r#""type":{},"#,
            r#""type_name":"{}","#,
            r#""colour":{},"#,
            r#""attenuation":{{"#,
            r#""constant":{},"#,
            r#""linear":{},"#,
            r#""squared":{}}},"#,
            r#""enabled":{},"#,
            r#""extras":[{}]}}"#,
        ),
        escape_json(&name),
        version,
        light_type,
        light_type_name(light_type),
        colour,
        constant,
        linear,
        squared,
        enabled,
        extras
    );
    Some(
        RecoveredComponent {
            relative_path: PathBuf::from("light")
                .join(format!("{file_name}.json")),
            name,
            bytes: json.into_bytes(),
            payload_format: "schema_json".to_owned(),
            recovery_status: "decoded_schema_payload".to_owned(),
        },
    )
}

/// Recover fence json.
fn recover_fence_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(
        component, source,
    )
    .ok()?;
    let wall_offset = 12;
    let (_, _, wall_total) = read_chunk_header(
        chunk,
        wall_offset,
    )?;
    if wall_offset + wall_total > chunk.len() {
        return None;
    }
    let mut cursor = wall_offset + 12;
    let start = read_point(
        chunk,
        &mut cursor,
    )?;
    let end = read_point(
        chunk,
        &mut cursor,
    )?;
    let normal = read_point(
        chunk,
        &mut cursor,
    )?;
    let name = format!("srr_fence_dsg_{kind_index:04}");
    let json = format!(
        "{{\"schema\":\"tlFenceDSGChunk.sc/tlWallChunk.sc\",\"name\":\"{}\",\"\
         start\":[{},{},{}],\"end\":[{},{},{}],\"normal\":[{},{},{}]}}\n",
        name,
        start[0],
        start[1],
        start[2],
        end[0],
        end[1],
        end[2],
        normal[0],
        normal[1],
        normal[2]
    );
    Some(
        RecoveredComponent {
            relative_path: PathBuf::from("srr_fence_dsg")
                .join(format!("{name}.json")),
            name,
            bytes: json.into_bytes(),
            payload_format: "schema_json".to_owned(),
            recovery_status: "decoded_schema_payload".to_owned(),
        },
    )
}

/// Read point.
fn read_point(
    bytes: &[u8],
    cursor: &mut usize,
) -> Option<[f32; 3]> {
    let x = read_f32(
        bytes, *cursor,
    )?;
    *cursor += 4;
    let y = read_f32(
        bytes, *cursor,
    )?;
    *cursor += 4;
    let z = read_f32(
        bytes, *cursor,
    )?;
    *cursor += 4;
    Some(
        [
            x, y, z,
        ],
    )
}

/// Read pascal at.
fn read_pascal_at(
    bytes: &[u8],
    cursor: &mut usize,
) -> Option<String> {
    let length = usize::from(*bytes.get(*cursor)?);
    let start = (*cursor).checked_add(1)?;
    let end = start.checked_add(length)?;
    let raw = bytes.get(start..end)?;
    let value = std::str::from_utf8(raw)
        .ok()?
        .to_owned();
    *cursor = end;
    Some(value)
}

/// Read f32.
fn read_f32(
    bytes: &[u8],
    offset: usize,
) -> Option<f32> {
    let end = offset.checked_add(4)?;
    let slice = bytes.get(offset..end)?;
    Some(
        f32::from_le_bytes(
            [
                slice[0], slice[1], slice[2], slice[3],
            ],
        ),
    )
}

/// Recover name version alpha json.
fn recover_name_version_alpha_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(
        component, source,
    )
    .ok()?;
    let mut cursor = 12;
    let name = read_pascal_at(
        chunk,
        &mut cursor,
    )?;
    let version = read_u32(
        chunk, cursor,
    )?;
    cursor += 4;
    let has_alpha = read_u32(
        chunk, cursor,
    )?;
    let kind = component
        .kind
        .label();
    let file_name = fallback_name(
        kind, kind_index, &name,
    );
    let json = format!(
        "{{\"schema\":\"{}\",\"name\":\"{}\",\"version\":{},\"has_alpha\":\
         {}}}\n",
        escape_json(kind_schema(kind)),
        escape_json(&name),
        version,
        has_alpha
    );
    Some(
        json_component(
            kind,
            &file_name,
            name,
            json,
            "decoded_schema_payload",
        ),
    )
}

/// Recover road segment json.
fn recover_road_segment_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(
        component, source,
    )
    .ok()?;
    let mut cursor = 12;
    let name = read_pascal_at(
        chunk,
        &mut cursor,
    )?;
    let road_type = read_u32(
        chunk, cursor,
    )?;
    cursor += 4;
    let lanes = read_u32(
        chunk, cursor,
    )?;
    cursor += 4;
    let shoulder = read_u32(
        chunk, cursor,
    )?;
    cursor += 4;
    let direction = read_point(
        chunk,
        &mut cursor,
    )?;
    let top = read_point(
        chunk,
        &mut cursor,
    )?;
    let bottom = read_point(
        chunk,
        &mut cursor,
    )?;
    let kind = component
        .kind
        .label();
    let file_name = fallback_name(
        kind, kind_index, &name,
    );
    let json = format!(
        "{{\"schema\":\"road_segment_data\",\"name\":\"{}\",\"type\":{},\"\
         num_lanes\":{},\"has_shoulder\":{},\"direction\":[{},{},{}],\"top\":\
         [{},{},{}],\"bottom\":[{},{},{}]}}\n",
        escape_json(&name),
        road_type,
        lanes,
        shoulder,
        direction[0],
        direction[1],
        direction[2],
        top[0],
        top[1],
        top[2],
        bottom[0],
        bottom[1],
        bottom[2]
    );
    Some(
        json_component(
            kind,
            &file_name,
            name,
            json,
            "decoded_schema_payload",
        ),
    )
}

/// Recover road json.
fn recover_road_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(
        component, source,
    )
    .ok()?;
    let mut cursor = 12;
    let name = read_pascal_at(
        chunk,
        &mut cursor,
    )?;
    let road_type = read_u32(
        chunk, cursor,
    )?;
    cursor += 4;
    let start = read_pascal_at(
        chunk,
        &mut cursor,
    )?;
    let end = read_pascal_at(
        chunk,
        &mut cursor,
    )?;
    let density = read_u32(
        chunk, cursor,
    )?;
    cursor += 4;
    let speed = read_u32(
        chunk, cursor,
    )?;
    let kind = component
        .kind
        .label();
    let file_name = fallback_name(
        kind, kind_index, &name,
    );
    let json = format!(
        "{{\"schema\":\"road\",\"name\":\"{}\",\"type\":{},\"\
         start_intersection\":\"{}\",\"end_intersection\":\"{}\",\"density\":\
         {},\"speed\":{}}}\n",
        escape_json(&name),
        road_type,
        escape_json(&start),
        escape_json(&end),
        density,
        speed
    );
    Some(
        json_component(
            kind,
            &file_name,
            name,
            json,
            "decoded_schema_payload",
        ),
    )
}

/// Recover intersection json.
fn recover_intersection_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(
        component, source,
    )
    .ok()?;
    let mut cursor = 12;
    let name = read_pascal_at(
        chunk,
        &mut cursor,
    )?;
    let centre = read_point(
        chunk,
        &mut cursor,
    )?;
    let radius = read_f32(
        chunk, cursor,
    )?;
    cursor += 4;
    let intersection_type = read_u32(
        chunk, cursor,
    )?;
    let kind = component
        .kind
        .label();
    let file_name = fallback_name(
        kind, kind_index, &name,
    );
    let json = format!(
        concat!(
            "{{\"schema\":\"intersection\",",
            "\"name\":\"{}\",",
            "\"centre\":[{},{},{}],",
            "\"radius\":{},",
            "\"type\":{}}}\n",
        ),
        escape_json(&name),
        centre[0],
        centre[1],
        centre[2],
        radius,
        intersection_type
    );
    Some(
        json_component(
            kind,
            &file_name,
            name,
            json,
            "decoded_schema_payload",
        ),
    )
}

/// Recover tree json.
fn recover_tree_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(
        component, source,
    )
    .ok()?;
    let mut cursor = 12;
    let nodes = read_u32(
        chunk, cursor,
    )?;
    cursor += 4;
    let bounds_min = read_point(
        chunk,
        &mut cursor,
    )?;
    let bounds_max = read_point(
        chunk,
        &mut cursor,
    )?;
    let kind = component
        .kind
        .label();
    let name = format!("{kind}_{kind_index:04}");
    let json = format!(
        "{{\"schema\":\"tree_dsg\",\"name\":\"{}\",\"num_nodes\":{},\"\
         bounds_min\":[{},{},{}],\"bounds_max\":[{},{},{}]}}\n",
        escape_json(&name),
        nodes,
        bounds_min[0],
        bounds_min[1],
        bounds_min[2],
        bounds_max[0],
        bounds_max[1],
        bounds_max[2]
    );
    Some(
        json_component(
            kind,
            &name,
            name.clone(),
            json,
            "decoded_schema_payload",
        ),
    )
}

/// Recover intersect json.
fn recover_intersect_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(
        component, source,
    )
    .ok()?;
    let kind = component
        .kind
        .label();
    let name = format!("{kind}_{kind_index:04}");
    let json = super::decoders::intersect::dsg_json(chunk)?;
    Some(
        json_component(
            kind,
            &name,
            name.clone(),
            json,
            "decoded_schema_payload",
        ),
    )
}

/// Recover simulation collision object json.
fn recover_collision_object_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    recover_collision_decoded_json(
        component,
        source,
        kind_index,
        super::decoders::collision::object_json,
    )
}

/// Recover simulation physics object json.
fn recover_physics_object_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    recover_collision_decoded_json(
        component,
        source,
        kind_index,
        super::decoders::collision::physics_json,
    )
}

/// Recover physics DSG wrapper json.
fn recover_physics_dsg_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(
        component, source,
    )
    .ok()?;
    let kind = component
        .kind
        .label();
    let name = component_name(
        component, source, kind_index,
    );
    let file_name = fallback_name(
        kind, kind_index, &name,
    );
    let json = super::decoders::collision::dsg_json(
        chunk,
        kind_schema(kind),
    )?;
    Some(
        json_component(
            kind,
            &file_name,
            name,
            json,
            "decoded_schema_payload",
        ),
    )
}

/// Recover chunk-set json.
fn recover_chunk_set_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    recover_collision_decoded_json(
        component,
        source,
        kind_index,
        super::decoders::collision::chunk_set_json,
    )
}

/// Recover collision-family decoded json.
fn recover_collision_decoded_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
    decoder: fn(&[u8]) -> Option<String>,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(
        component, source,
    )
    .ok()?;
    let kind = component
        .kind
        .label();
    let name = component_name(
        component, source, kind_index,
    );
    let file_name = fallback_name(
        kind, kind_index, &name,
    );
    let json = decoder(chunk)?;
    Some(
        json_component(
            kind,
            &file_name,
            name,
            json,
            "decoded_schema_payload",
        ),
    )
}

/// Fallback name.
fn fallback_name(
    kind: &str,
    kind_index: usize,
    name: &str,
) -> String {
    if name.is_empty() {
        format!("{kind}_{kind_index:04}")
    } else {
        sanitize(name)
    }
}

/// Build a fallback component name from the kind and per-kind index.
fn fallback_component_name(
    component: &ChunkRecord,
    kind_index: usize,
) -> String {
    format!(
        "{}_{kind_index:04}",
        component
            .kind
            .label()
    )
}

/// Wrap decoded schema JSON in package component metadata.
fn schema_component(
    component: &ChunkRecord,
    kind_index: usize,
    name: String,
    json: String,
) -> RecoveredComponent {
    let kind = component
        .kind
        .label();
    let file_name = fallback_name(
        kind, kind_index, &name,
    );
    json_component(
        kind,
        &file_name,
        name,
        json,
        "decoded_schema_payload",
    )
}

/// Json component.
fn json_component(
    kind: &str,
    file_name: &str,
    name: String,
    json: String,
    recovery_status: &str,
) -> RecoveredComponent {
    RecoveredComponent {
        relative_path: PathBuf::from(kind).join(
            format!(
                "{}.json",
                sanitize(file_name)
            ),
        ),
        name,
        bytes: json.into_bytes(),
        payload_format: "schema_json".to_owned(),
        recovery_status: recovery_status.to_owned(),
    }
}

/// Recover mesh json.
pub(super) fn recover_mesh_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(
        component, source,
    )
    .ok()?;
    let name = read_pascal_name(
        component, source,
    )
    .unwrap_or_else(
        || {
            format!(
                "{}_{kind_index:04}",
                component
                    .kind
                    .label()
            )
        },
    );
    let json = super::decoders::mesh::mesh_json(chunk)?;
    let kind = component
        .kind
        .label();
    let file_name = fallback_name(
        kind, kind_index, &name,
    );
    Some(
        json_component(
            kind,
            &file_name,
            name,
            json,
            "decoded_schema_payload",
        ),
    )
}

/// Recover skin json.
pub(super) fn recover_skin_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(
        component, source,
    )
    .ok()?;
    let name = read_pascal_name(
        component, source,
    )
    .unwrap_or_else(
        || {
            format!(
                "{}_{kind_index:04}",
                component
                    .kind
                    .label()
            )
        },
    );
    let json = super::decoders::mesh::skin_json(chunk)?;
    let kind = component
        .kind
        .label();
    let file_name = fallback_name(
        kind, kind_index, &name,
    );
    Some(
        json_component(
            kind,
            &file_name,
            name,
            json,
            "decoded_schema_payload",
        ),
    )
}

/// Recover skeleton json.
pub(super) fn recover_skeleton_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    recover_rig_decoded_json(
        component,
        source,
        kind_index,
        super::decoders::rig::skeleton_json,
    )
}

/// Recover camera json.
pub(super) fn recover_camera_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(
        component, source,
    )
    .ok()?;
    let mut cursor = 12;
    let name = read_pascal_at(
        chunk,
        &mut cursor,
    )?;
    let version = read_u32(
        chunk, cursor,
    )?;
    cursor += 4;
    let fov = read_f32(
        chunk, cursor,
    )?;
    cursor += 4;
    let aspect_ratio = read_f32(
        chunk, cursor,
    )?;
    cursor += 4;
    let near_clip = read_f32(
        chunk, cursor,
    )?;
    cursor += 4;
    let far_clip = read_f32(
        chunk, cursor,
    )?;
    let kind = component
        .kind
        .label();
    let file_name = fallback_name(
        kind, kind_index, &name,
    );
    let json = format!(
        "{{\"schema\":\"camera\",\"name\":\"{}\",\"version\":{},\"fov\":{},\"\
         aspect_ratio\":{},\"near_clip\":{},\"far_clip\":{}}}\n",
        escape_json(&name),
        version,
        fov,
        aspect_ratio,
        near_clip,
        far_clip
    );
    Some(
        json_component(
            kind,
            &file_name,
            name,
            json,
            "decoded_schema_payload",
        ),
    )
}

/// Recover composite json.
pub(super) fn recover_composite_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(
        component, source,
    )
    .ok()?;
    let name = read_pascal_name(
        component, source,
    )
    .unwrap_or_else(
        || {
            fallback_component_name(
                component, kind_index,
            )
        },
    );
    let json = super::decoders::scene::composite_drawable_json(chunk)?;
    Some(
        schema_component(
            component, kind_index, name, json,
        ),
    )
}

/// Recover scenegraph json.
pub(super) fn recover_scenegraph_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(
        component, source,
    )
    .ok()?;
    let name = read_pascal_name(
        component, source,
    )
    .unwrap_or_else(
        || {
            fallback_component_name(
                component, kind_index,
            )
        },
    );
    let json = super::decoders::scene::scenegraph_json(chunk)?;
    Some(
        schema_component(
            component, kind_index, name, json,
        ),
    )
}

/// Recover entity DSG json.
fn recover_entity_dsg_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(
        component, source,
    )
    .ok()?;
    let name = read_pascal_name(
        component, source,
    )
    .unwrap_or_else(
        || {
            fallback_component_name(
                component, kind_index,
            )
        },
    );
    let json = super::decoders::scene::entity_dsg_json(chunk)?;
    Some(
        schema_component(
            component, kind_index, name, json,
        ),
    )
}

/// Recover insta entity DSG json.
fn recover_insta_entity_dsg_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(
        component, source,
    )
    .ok()?;
    let name = read_pascal_name(
        component, source,
    )
    .unwrap_or_else(
        || {
            fallback_component_name(
                component, kind_index,
            )
        },
    );
    let json = super::decoders::scene::insta_entity_dsg_json(chunk)?;
    Some(
        schema_component(
            component, kind_index, name, json,
        ),
    )
}

/// Recover animation json.
pub(super) fn recover_animation_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    recover_rig_decoded_json(
        component,
        source,
        kind_index,
        super::decoders::rig::animation_json,
    )
}

/// Recover particle system json.
pub(super) fn recover_particle_system_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(
        component, source,
    )
    .ok()?;
    let mut cursor = 12;
    let version = read_u32(
        chunk, cursor,
    )?;
    cursor += 4;
    let name = read_pascal_at(
        chunk,
        &mut cursor,
    )?;
    let factory = read_pascal_at(
        chunk,
        &mut cursor,
    )?;
    let kind = component
        .kind
        .label();
    let file_name = fallback_name(
        kind, kind_index, &name,
    );
    let json = format!(
        "{{\"schema\":\"particle_system\",\"name\":\"{}\",\"version\":{},\"\
         factory_name\":\"{}\"}}\n",
        escape_json(&name),
        version,
        escape_json(&factory)
    );
    Some(
        json_component(
            kind,
            &file_name,
            name,
            json,
            "decoded_schema_payload",
        ),
    )
}

/// Recover particle factory json.
pub(super) fn recover_particle_factory_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(
        component, source,
    )
    .ok()?;
    let mut cursor = 12;
    let version = read_u32(
        chunk, cursor,
    )?;
    cursor += 4;
    let name = read_pascal_at(
        chunk,
        &mut cursor,
    )?;
    let frame_rate = read_f32(
        chunk, cursor,
    )?;
    cursor += 4;
    let anim_frames = read_u32(
        chunk, cursor,
    )?;
    cursor += 4;
    let ol_frames = read_u32(
        chunk, cursor,
    )?;
    let kind = component
        .kind
        .label();
    let file_name = fallback_name(
        kind, kind_index, &name,
    );
    let json = format!(
        "{{\"schema\":\"particle_system_factory\",\"name\":\"{}\",\"version\":\
         {},\"frame_rate\":{},\"num_anim_frames\":{},\"num_ol_frames\":{}}}\n",
        escape_json(&name),
        version,
        frame_rate,
        anim_frames,
        ol_frames
    );
    Some(
        json_component(
            kind,
            &file_name,
            name,
            json,
            "decoded_schema_payload",
        ),
    )
}

/// Recover light group json.
pub(super) fn recover_light_group_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(
        component, source,
    )
    .ok()?;
    let mut cursor = 12;
    let name = read_pascal_at(
        chunk,
        &mut cursor,
    )?;
    let num_lights = read_u32(
        chunk, cursor,
    )?;
    let kind = component
        .kind
        .label();
    let file_name = fallback_name(
        kind, kind_index, &name,
    );
    let json = format!(
        "{{\"schema\":\"light_group\",\"name\":\"{}\",\"num_lights\":{}}}\n",
        escape_json(&name),
        num_lights
    );
    Some(
        json_component(
            kind,
            &file_name,
            name,
            json,
            "decoded_schema_payload",
        ),
    )
}

/// Recover world sphere json.
pub(super) fn recover_world_sphere_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(
        component, source,
    )
    .ok()?;
    let mut cursor = 12;
    let name = read_pascal_at(
        chunk,
        &mut cursor,
    )?;
    let version = read_u32(
        chunk, cursor,
    )?;
    cursor += 4;
    let num_meshes = read_u32(
        chunk, cursor,
    )?;
    cursor += 4;
    let num_billboards = read_u32(
        chunk, cursor,
    )?;
    let kind = component
        .kind
        .label();
    let file_name = fallback_name(
        kind, kind_index, &name,
    );
    let json = format!(
        "{{\"schema\":\"world_sphere_dsg\",\"name\":\"{}\",\"version\":{},\"\
         num_meshes\":{},\"num_billboard_quads\":{}}}\n",
        escape_json(&name),
        version,
        num_meshes,
        num_billboards
    );
    Some(
        json_component(
            kind,
            &file_name,
            name,
            json,
            "decoded_schema_payload",
        ),
    )
}

/// Read fourcc.
fn read_fourcc(
    bytes: &[u8],
    offset: usize,
) -> Option<String> {
    let end = offset.checked_add(4)?;
    let slice = bytes.get(offset..end)?;
    Some(
        std::str::from_utf8(slice)
            .ok()?
            .trim_matches(char::from(0))
            .to_owned(),
    )
}

/// Recover text bible json.
fn recover_text_bible_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(
        component, source,
    )
    .ok()?;
    let name = component_name(
        component, source, kind_index,
    );
    let kind = component
        .kind
        .label();
    let file_name = fallback_name(
        kind, kind_index, &name,
    );
    let children = child_chunks_json(
        chunk,
        component.header_size,
        component.total_size,
    );
    let json = format!(
        concat!(
            r#"{{"schema":"text_bible","#,
            r#""name":"{}","#,
            r#""payload_size":{},"#,
            r#""child_count":{},"#,
            r#""language_chunks":[{}]}}"#,
        ),
        escape_json(&name),
        component.payload_size,
        component.child_count,
        children
    );
    Some(
        json_component(
            kind,
            &file_name,
            name,
            json,
            "decoded_schema_payload",
        ),
    )
}

/// Recover srr locator json.
fn recover_srr_locator_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(
        component, source,
    )
    .ok()?;
    let mut cursor = 12;
    let name = read_pascal_at(
        chunk,
        &mut cursor,
    )?;
    let locator_type = read_u32(
        chunk, cursor,
    )?;
    cursor += 4;
    let num_data = read_u32(
        chunk, cursor,
    )? as usize;
    cursor += 4;
    let mut data = Vec::with_capacity(num_data);
    for _ in 0..num_data {
        data.push(
            read_u32(
                chunk, cursor,
            )?,
        );
        cursor += 4;
    }
    let position = read_point(
        chunk,
        &mut cursor,
    )?;
    let num_triggers = read_u32(
        chunk, cursor,
    )?;
    let locator_type_name = super::decoders::locator::type_name(locator_type)?;
    let data_interpretation =
        super::decoders::locator::data_interpretation_json(
            locator_type,
            &data,
            num_triggers,
        )?;
    let triggers = trigger_volumes_json(
        chunk,
        component.header_size,
        component.total_size,
    );
    let kind = component
        .kind
        .label();
    let file_name = fallback_name(
        kind, kind_index, &name,
    );
    let json = format!(
        "{{\"schema\":\"locator\",\"name\":\"{}\",\"locator_type\":{},\"\
         locator_type_name\":\"{}\",\"position\":[{},{},{}],\"\
         num_data_elements\":{},\"data_elements_u32\":[{}],\"\
         data_elements_f32\":[{}],\"data_ascii_lossy\":\"{}\",\"\
         data_interpretation\":{},\"num_triggers\":{},\"trigger_volumes\":\
         [{}]}}\n",
        escape_json(&name),
        locator_type,
        locator_type_name,
        position[0],
        position[1],
        position[2],
        num_data,
        u32_list_json(&data),
        f32_list_json(&data),
        escape_json(&ascii_from_u32_words(&data)),
        data_interpretation,
        num_triggers,
        triggers
    );
    Some(
        json_component(
            kind,
            &file_name,
            name,
            json,
            "decoded_schema_payload",
        ),
    )
}

/// U32 list json.
fn u32_list_json(values: &[u32]) -> String {
    values
        .iter()
        .map(u32::to_string)
        .collect::<Vec<_>>()
        .join(",")
}

/// F32 list json.
fn f32_list_json(values: &[u32]) -> String {
    values
        .iter()
        .map(
            |value| {
                let decoded = f32::from_bits(*value);
                render_f32(
                    decoded,
                    decoded.to_string(),
                )
            },
        )
        .collect::<Vec<_>>()
        .join(",")
}

/// Ascii from u32 words.
fn ascii_from_u32_words(values: &[u32]) -> String {
    let mut bytes = Vec::with_capacity(values.len() * 4);
    for value in values {
        bytes.extend_from_slice(&value.to_le_bytes());
    }
    String::from_utf8_lossy(&bytes)
        .trim_matches(char::from(0))
        .to_owned()
}

/// Trigger volumes json.
fn trigger_volumes_json(
    chunk: &[u8],
    mut cursor: usize,
    end: usize,
) -> String {
    let mut triggers = Vec::new();
    while cursor + 12 <= end {
        let Some((id, header_size, total_size)) = read_chunk_header(
            chunk, cursor,
        ) else {
            break;
        };
        let next = cursor.saturating_add(total_size);
        if total_size < header_size || next > end {
            break;
        }
        if id == 0x0300_0006
            && let Some(trigger) = trigger_volume_json(
                chunk,
                cursor,
                header_size,
            )
        {
            triggers.push(trigger);
        }
        cursor = next;
    }
    triggers.join(",")
}

/// Trigger volume json.
fn trigger_volume_json(
    chunk: &[u8],
    offset: usize,
    header_size: usize,
) -> Option<String> {
    let mut cursor = offset + 12;
    let name = read_pascal_at(
        chunk,
        &mut cursor,
    )?;
    let volume_type = read_u32(
        chunk, cursor,
    )?;
    cursor += 4;
    let scale = [
        read_f32(
            chunk, cursor,
        )?,
        read_f32(
            chunk,
            cursor + 4,
        )?,
        read_f32(
            chunk,
            cursor + 8,
        )?,
    ];
    cursor += 12;
    let mut matrix = [[0.0_f32; 4]; 4];
    for row in &mut matrix {
        for value in row {
            *value = read_f32(
                chunk, cursor,
            )?;
            cursor += 4;
        }
    }
    if cursor > offset + header_size {
        return None;
    }
    let position = [
        matrix[3][0],
        matrix[3][1],
        matrix[3][2],
    ];
    Some(
        format!(
            "{{\"name\":\"{}\",\"type\":{},\"type_name\":\"{}\",\"scale\":[{},\
             {},{}],\"position\":[{},{},{}],\"matrix\":{}}}",
            escape_json(&name),
            volume_type,
            trigger_volume_type_name(volume_type),
            scale[0],
            scale[1],
            scale[2],
            position[0],
            position[1],
            position[2],
            matrix_json(&matrix)
        ),
    )
}

/// Trigger volume type name.
fn trigger_volume_type_name(volume_type: u32) -> &'static str {
    match volume_type {
        0 => "sphere",
        1 => "rectangle",
        _ => "unknown_trigger_volume_type",
    }
}

/// Matrix json.
fn matrix_json(matrix: &[[f32; 4]; 4]) -> String {
    format!(
        "[[{},{},{},{}],[{},{},{},{}],[{},{},{},{}],[{},{},{},{}]]",
        matrix[0][0],
        matrix[0][1],
        matrix[0][2],
        matrix[0][3],
        matrix[1][0],
        matrix[1][1],
        matrix[1][2],
        matrix[1][3],
        matrix[2][0],
        matrix[2][1],
        matrix[2][2],
        matrix[2][3],
        matrix[3][0],
        matrix[3][1],
        matrix[3][2],
        matrix[3][3]
    )
}

/// Shader params json.
fn shader_params_json(
    chunk: &[u8],
    cursor: usize,
    end: usize,
) -> String {
    param_chunks_json(
        chunk, cursor, end, true,
    )
}

/// Game attr params json.
fn game_attr_params_json(
    chunk: &[u8],
    cursor: usize,
    end: usize,
) -> String {
    param_chunks_json(
        chunk, cursor, end, false,
    )
}

/// Param chunks json.
fn param_chunks_json(
    chunk: &[u8],
    mut cursor: usize,
    end: usize,
    shader_params: bool,
) -> String {
    let mut params = Vec::new();
    while cursor + 12 <= end {
        let header = read_chunk_header(
            chunk, cursor,
        );
        let Some((id, header_size, total_size)) = header else {
            break;
        };
        let next = cursor.saturating_add(total_size);
        if total_size < header_size || next > end {
            break;
        }
        let parsed = if shader_params {
            shader_param_json(
                chunk, cursor, id,
            )
        } else {
            game_attr_param_json(
                chunk, cursor, id,
            )
        };
        if let Some(value) = parsed {
            params.push(value);
        }
        cursor = next;
    }
    params.join(",")
}

/// Shader param json.
fn shader_param_json(
    chunk: &[u8],
    offset: usize,
    id: u32,
) -> Option<String> {
    let mut cursor = offset + 12;
    match id {
        0x0001_1002 => {
            let param = read_fourcc(
                chunk, cursor,
            )?;
            cursor += 4;
            let value = read_pascal_at(
                chunk,
                &mut cursor,
            )?;
            Some(
                format!(
                    r#"{{"kind":"texture","param":"{}","value":"{}"}}"#,
                    escape_json(&param),
                    escape_json(&value)
                ),
            )
        }
        0x0001_1003 => shader_number_param(
            chunk, cursor, "int",
        ),
        0x0001_1004 => shader_float_param(
            chunk, cursor,
        ),
        0x0001_1005 => shader_colour_param(
            chunk, cursor,
        ),
        0x0001_1006 => shader_vector_param(
            chunk, cursor,
        ),
        0x0001_1007 => shader_matrix_param(
            chunk, cursor,
        ),
        _ => None,
    }
}

/// Game attr param json.
fn game_attr_param_json(
    chunk: &[u8],
    offset: usize,
    id: u32,
) -> Option<String> {
    let mut cursor = offset + 12;
    let name = read_pascal_at(
        chunk,
        &mut cursor,
    )?;
    match id {
        0x0001_2001 => {
            let value = read_u32(
                chunk, cursor,
            )?;
            Some(
                format!(
                    r#"{{"kind":"int","name":"{}","value":{}}}"#,
                    escape_json(&name),
                    value
                ),
            )
        }
        0x0001_2002 => {
            let value = read_f32(
                chunk, cursor,
            )?;
            Some(
                format!(
                    r#"{{"kind":"float","name":"{}","value":{}}}"#,
                    escape_json(&name),
                    value
                ),
            )
        }
        0x0001_2003 => {
            let value = read_u32(
                chunk, cursor,
            )?;
            Some(
                format!(
                    r#"{{"kind":"colour","name":"{}","value":{}}}"#,
                    escape_json(&name),
                    value
                ),
            )
        }
        0x0001_2004 => game_attr_vector_param(
            chunk, cursor, &name,
        ),
        0x0001_2005 => game_attr_matrix_param(
            chunk, cursor, &name,
        ),
        _ => None,
    }
}

/// Shader number param.
fn shader_number_param(
    chunk: &[u8],
    cursor: usize,
    kind: &str,
) -> Option<String> {
    let param = read_fourcc(
        chunk, cursor,
    )?;
    let value = read_u32(
        chunk,
        cursor + 4,
    )?;
    Some(
        format!(
            r#"{{"kind":"{}","param":"{}","value":{}}}"#,
            kind,
            escape_json(&param),
            value
        ),
    )
}

/// Shader float param.
fn shader_float_param(
    chunk: &[u8],
    cursor: usize,
) -> Option<String> {
    let param = read_fourcc(
        chunk, cursor,
    )?;
    let value = read_f32(
        chunk,
        cursor + 4,
    )?;
    Some(
        format!(
            r#"{{"kind":"float","param":"{}","value":{}}}"#,
            escape_json(&param),
            value
        ),
    )
}

/// Shader colour param.
fn shader_colour_param(
    chunk: &[u8],
    cursor: usize,
) -> Option<String> {
    let param = read_fourcc(
        chunk, cursor,
    )?;
    let value = read_u32(
        chunk,
        cursor + 4,
    )?;
    Some(
        format!(
            r#"{{"kind":"colour","param":"{}","value":{}}}"#,
            escape_json(&param),
            value
        ),
    )
}

/// Shader vector param.
fn shader_vector_param(
    chunk: &[u8],
    cursor: usize,
) -> Option<String> {
    let param = read_fourcc(
        chunk, cursor,
    )?;
    let x = read_f32(
        chunk,
        cursor + 4,
    )?;
    let y = read_f32(
        chunk,
        cursor + 8,
    )?;
    let z = read_f32(
        chunk,
        cursor + 12,
    )?;
    Some(
        format!(
            r#"{{"kind":"vector","param":"{}","value":[{},{},{}]}}"#,
            escape_json(&param),
            x,
            y,
            z
        ),
    )
}

/// Shader matrix param.
fn shader_matrix_param(
    chunk: &[u8],
    cursor: usize,
) -> Option<String> {
    let param = read_fourcc(
        chunk, cursor,
    )?;
    let matrix = matrix_values_json(
        chunk,
        cursor + 4,
    )?;
    Some(
        format!(
            r#"{{"kind":"matrix","param":"{}","value":{}}}"#,
            escape_json(&param),
            matrix
        ),
    )
}

/// Game attr vector param.
fn game_attr_vector_param(
    chunk: &[u8],
    cursor: usize,
    name: &str,
) -> Option<String> {
    let x = read_f32(
        chunk, cursor,
    )?;
    let y = read_f32(
        chunk,
        cursor + 4,
    )?;
    let z = read_f32(
        chunk,
        cursor + 8,
    )?;
    Some(
        format!(
            r#"{{"kind":"vector","name":"{}","value":[{},{},{}]}}"#,
            escape_json(name),
            x,
            y,
            z
        ),
    )
}

/// Game attr matrix param.
fn game_attr_matrix_param(
    chunk: &[u8],
    cursor: usize,
    name: &str,
) -> Option<String> {
    let matrix = matrix_values_json(
        chunk, cursor,
    )?;
    Some(
        format!(
            r#"{{"kind":"matrix","name":"{}","value":{}}}"#,
            escape_json(name),
            matrix
        ),
    )
}

/// Matrix values json.
fn matrix_values_json(
    chunk: &[u8],
    cursor: usize,
) -> Option<String> {
    let mut values = Vec::with_capacity(16);
    for index in 0..16 {
        values.push(
            read_f32(
                chunk,
                cursor + index * 4,
            )?,
        );
    }
    Some(
        format!(
            "[[{},{},{},{}],[{},{},{},{}],[{},{},{},{}],[{},{},{},{}]]",
            values[0],
            values[1],
            values[2],
            values[3],
            values[4],
            values[5],
            values[6],
            values[7],
            values[8],
            values[9],
            values[10],
            values[11],
            values[12],
            values[13],
            values[14],
            values[15]
        ),
    )
}

/// Light type name.
fn light_type_name(value: u32) -> &'static str {
    match value {
        0 => "ambient",
        1 => "directional",
        2 => "point",
        3 => "spot",
        _ => "unknown_light_type",
    }
}

/// Light children json.
fn light_children_json(
    chunk: &[u8],
    mut cursor: usize,
    end: usize,
) -> String {
    let mut extras = Vec::new();
    while cursor + 12 <= end {
        let Some((id, header_size, total_size)) = read_chunk_header(
            chunk, cursor,
        ) else {
            break;
        };
        let next = cursor.saturating_add(total_size);
        if total_size < header_size || next > end {
            break;
        }
        if let Some(extra) = light_child_json(
            chunk, cursor, id,
        ) {
            extras.push(extra);
        }
        cursor = next;
    }
    extras.join(",")
}

/// Light child json.
fn light_child_json(
    chunk: &[u8],
    offset: usize,
    id: u32,
) -> Option<String> {
    let mut cursor = offset + 12;
    match id {
        0x0001_3001 => {
            let p = read_point(
                chunk,
                &mut cursor,
            )?;
            Some(
                format!(
                    r#"{{"kind":"direction","value":[{},{},{}]}}"#,
                    p[0], p[1], p[2]
                ),
            )
        }
        0x0001_3002 => {
            let p = read_point(
                chunk,
                &mut cursor,
            )?;
            Some(
                format!(
                    r#"{{"kind":"position","value":[{},{},{}]}}"#,
                    p[0], p[1], p[2]
                ),
            )
        }
        0x0001_3003 => {
            let phi = read_f32(
                chunk, cursor,
            )?;
            let theta = read_f32(
                chunk,
                cursor + 4,
            )?;
            let falloff = read_f32(
                chunk,
                cursor + 8,
            )?;
            let range = read_f32(
                chunk,
                cursor + 12,
            )?;
            Some(
                format!(
                    concat!(
                        r#"{{"kind":"cone","#,
                        r#""phi":{},"#,
                        r#""theta":{},"#,
                        r#""falloff":{},"#,
                        r#""range":{}}}"#,
                    ),
                    phi, theta, falloff, range
                ),
            )
        }
        0x0001_3004 => {
            let value = read_u32(
                chunk, cursor,
            )?;
            Some(format!(r#"{{"kind":"shadow","value":{value}}}"#))
        }
        0x0001_3006 => light_decay_json(
            chunk, cursor,
        ),
        0x0001_3008 => {
            let value = read_u32(
                chunk, cursor,
            )?;
            Some(format!(r#"{{"kind":"illumination_type","value":{value}}}"#))
        }
        _ => None,
    }
}

/// Light decay json.
fn light_decay_json(
    chunk: &[u8],
    mut cursor: usize,
) -> Option<String> {
    let decay_type = read_u32(
        chunk, cursor,
    )?;
    cursor += 4;
    let inner = read_point(
        chunk,
        &mut cursor,
    )?;
    let outer = read_point(
        chunk,
        &mut cursor,
    )?;
    Some(
        format!(
            concat!(
                r#"{{"kind":"decay_range","#,
                r#""type":{},"#,
                r#""inner":[{},{},{}],"#,
                r#""outer":[{},{},{}]}}"#,
            ),
            decay_type,
            inner[0],
            inner[1],
            inner[2],
            outer[0],
            outer[1],
            outer[2]
        ),
    )
}

/// Recover frame controller json.
pub(super) fn recover_frame_controller_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(
        component, source,
    )
    .ok()?;
    let mut cursor = 12;
    let version = read_u32(
        chunk, cursor,
    )?;
    cursor += 4;
    let name = read_pascal_at(
        chunk,
        &mut cursor,
    )?;
    let controller_type = read_fourcc(
        chunk, cursor,
    )?;
    cursor += 4;
    let frame_offset = read_f32(
        chunk, cursor,
    )?;
    cursor += 4;
    let hierarchy_name = read_pascal_at(
        chunk,
        &mut cursor,
    )?;
    let animation_name = read_pascal_at(
        chunk,
        &mut cursor,
    )?;
    let kind = component
        .kind
        .label();
    let file_name = fallback_name(
        kind, kind_index, &name,
    );
    let json = format!(
        concat!(
            r#"{{"schema":"frame_controller","#,
            r#""name":"{}","#,
            r#""version":{},"#,
            r#""type":"{}","#,
            r#""frame_offset":{},"#,
            r#""hierarchy_name":"{}","#,
            r#""animation_name":"{}"}}"#,
        ),
        escape_json(&name),
        version,
        escape_json(&controller_type),
        frame_offset,
        escape_json(&hierarchy_name),
        escape_json(&animation_name)
    );
    Some(
        json_component(
            kind,
            &file_name,
            name,
            json,
            "decoded_schema_payload",
        ),
    )
}

/// Recover sprite json.
pub(super) fn recover_sprite_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(
        component, source,
    )
    .ok()?;
    let mut cursor = 12;
    let name = read_pascal_at(
        chunk,
        &mut cursor,
    )?;
    let native_x = read_u32(
        chunk, cursor,
    )?;
    cursor += 4;
    let native_y = read_u32(
        chunk, cursor,
    )?;
    cursor += 4;
    let shader = read_pascal_at(
        chunk,
        &mut cursor,
    )?;
    let image_width = read_u32(
        chunk, cursor,
    )?;
    cursor += 4;
    let image_height = read_u32(
        chunk, cursor,
    )?;
    cursor += 4;
    let image_count = read_u32(
        chunk, cursor,
    )?;
    cursor += 4;
    let blit_border = read_u32(
        chunk, cursor,
    )?;
    let kind = component
        .kind
        .label();
    let file_name = fallback_name(
        kind, kind_index, &name,
    );
    let json = format!(
        concat!(
            r#"{{"schema":"sprite","#,
            r#""name":"{}","#,
            r#""native_size":[{},{}],"#,
            r#""shader":"{}","#,
            r#""image_size":[{},{}],"#,
            r#""image_count":{},"#,
            r#""blit_border":{}}}"#,
        ),
        escape_json(&name),
        native_x,
        native_y,
        escape_json(&shader),
        image_width,
        image_height,
        image_count,
        blit_border
    );
    Some(
        json_component(
            kind,
            &file_name,
            name,
            json,
            "decoded_schema_payload",
        ),
    )
}

/// Recover billboard quad group json.
fn recover_quad_group_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(
        component, source,
    )
    .ok()?;
    let mut cursor = 12;
    let version = read_u32(
        chunk, cursor,
    )?;
    cursor += 4;
    let name = read_pascal_at(
        chunk,
        &mut cursor,
    )?;
    let shader = read_pascal_at(
        chunk,
        &mut cursor,
    )?;
    let z_test = read_u32(
        chunk, cursor,
    )?;
    cursor += 4;
    let z_write = read_u32(
        chunk, cursor,
    )?;
    cursor += 4;
    let fog = read_u32(
        chunk, cursor,
    )?;
    cursor += 4;
    let num_quads = read_u32(
        chunk, cursor,
    )?;
    let children = child_chunks_json(
        chunk,
        component.header_size,
        component.total_size,
    );
    let kind = component
        .kind
        .label();
    let file_name = fallback_name(
        kind, kind_index, &name,
    );
    let json = format!(
        concat!(
            r#"{{"schema":"quad_group","#,
            r#""version":{},"#,
            r#""name":"{}","#,
            r#""shader":"{}","#,
            r#""z_test":{},"#,
            r#""z_write":{},"#,
            r#""fog":{},"#,
            r#""num_quads":{},"#,
            r#""quads":[{}]}}"#,
        ),
        version,
        escape_json(&name),
        escape_json(&shader),
        z_test,
        z_write,
        fog,
        num_quads,
        children
    );
    Some(
        json_component(
            kind,
            &file_name,
            name,
            json,
            "decoded_schema_payload",
        ),
    )
}

/// Recover texture font json.
fn recover_texture_font_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(
        component, source,
    )
    .ok()?;
    let mut cursor = 12;
    let version = read_u32(
        chunk, cursor,
    )?;
    cursor += 4;
    let name = read_pascal_at(
        chunk,
        &mut cursor,
    )?;
    let shader = read_pascal_at(
        chunk,
        &mut cursor,
    )?;
    let font_size = read_f32(
        chunk, cursor,
    )?;
    cursor += 4;
    let font_width = read_f32(
        chunk, cursor,
    )?;
    cursor += 4;
    let font_height = read_f32(
        chunk, cursor,
    )?;
    cursor += 4;
    let baseline = read_f32(
        chunk, cursor,
    )?;
    cursor += 4;
    let num_textures = read_u32(
        chunk, cursor,
    )?;
    let children = child_chunks_json(
        chunk,
        component.header_size,
        component.total_size,
    );
    let kind = component
        .kind
        .label();
    let file_name = fallback_name(
        kind, kind_index, &name,
    );
    let json = format!(
        concat!(
            r#"{{"schema":"texture_font","version":{},"name":"{}","#,
            r#""shader":"{}","font_size":{},"font_width":{},"#,
            r#""font_height":{},"baseline":{},"num_textures":{},"#,
            r#""children":[{}]}}"#,
        ),
        version,
        escape_json(&name),
        escape_json(&shader),
        font_size,
        font_width,
        font_height,
        baseline,
        num_textures,
        children
    );
    Some(
        json_component(
            kind,
            &file_name,
            name,
            json,
            "decoded_schema_payload",
        ),
    )
}

/// Recover scrooby project json.
fn recover_scrooby_project_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(
        component, source,
    )
    .ok()?;
    let mut cursor = 12;
    let name = read_pascal_at(
        chunk,
        &mut cursor,
    )?;
    let version = read_u32(
        chunk, cursor,
    )?;
    cursor += 4;
    let resolution_x = read_u32(
        chunk, cursor,
    )?;
    cursor += 4;
    let resolution_y = read_u32(
        chunk, cursor,
    )?;
    cursor += 4;
    let platform = read_pascal_at(
        chunk,
        &mut cursor,
    )?;
    let page_path = read_pascal_at(
        chunk,
        &mut cursor,
    )?;
    let resource_path = read_pascal_at(
        chunk,
        &mut cursor,
    )?;
    let screen_path = read_pascal_at(
        chunk,
        &mut cursor,
    )?;
    let children = child_chunks_json(
        chunk,
        component.header_size,
        component.total_size,
    );
    let kind = component
        .kind
        .label();
    let file_name = fallback_name(
        kind, kind_index, &name,
    );
    let json = format!(
        concat!(
            r#"{{"schema":"scrooby_project","name":"{}","version":{},"#,
            r#""resolution":[{},{}],"platform":"{}","#,
            r#""page_path":"{}","resource_path":"{}","screen_path":"{}","#,
            r#""children":[{}]}}"#,
        ),
        escape_json(&name),
        version,
        resolution_x,
        resolution_y,
        escape_json(&platform),
        escape_json(&page_path),
        escape_json(&resource_path),
        escape_json(&screen_path),
        children
    );
    Some(
        json_component(
            kind,
            &file_name,
            name,
            json,
            "decoded_schema_payload",
        ),
    )
}

/// Recover instanced particle system json.
fn recover_inst_particle_system_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(
        component, source,
    )
    .ok()?;
    let particle_type = read_u32(
        chunk, 12,
    )?;
    let max_instances = read_u32(
        chunk, 16,
    )?;
    let children = child_chunks_json(
        chunk,
        component.header_size,
        component.total_size,
    );
    let kind = component
        .kind
        .label();
    let name = format!("{kind}_{kind_index:04}");
    let json = format!(
        concat!(
            r#"{{"schema":"inst_particle_system","#,
            r#""particle_type":{},"#,
            r#""max_instances":{},"#,
            r#""children":[{}]}}"#,
        ),
        particle_type, max_instances, children
    );
    Some(
        json_component(
            kind,
            &name,
            name.clone(),
            json,
            "decoded_schema_payload",
        ),
    )
}

/// Recover multi controller json.
pub(super) fn recover_multi_controller_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    recover_rig_decoded_json(
        component,
        source,
        kind_index,
        super::decoders::rig::multi_controller_json,
    )
}

/// Recover vertex animation key json.
pub(super) fn recover_vertex_anim_key_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    recover_rig_decoded_json(
        component,
        source,
        kind_index,
        super::decoders::rig::vertex_key_json,
    )
}

/// Recover rig-family decoded json.
fn recover_rig_decoded_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
    decoder: fn(&[u8]) -> Option<String>,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(
        component, source,
    )
    .ok()?;
    let kind = component
        .kind
        .label();
    let name = component_name(
        component, source, kind_index,
    );
    let file_name = fallback_name(
        kind, kind_index, &name,
    );
    let json = decoder(chunk)?;
    Some(
        json_component(
            kind,
            &file_name,
            name,
            json,
            "decoded_schema_payload",
        ),
    )
}

/// Recover history json.
pub(super) fn recover_history_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(
        component, source,
    )
    .ok()?;
    let mut cursor = 12;
    let num_lines = read_u16(
        chunk, cursor,
    )? as usize;
    cursor += 2;
    let mut lines = Vec::new();
    for _ in 0..num_lines {
        lines.push(
            format!(
                "\"{}\"",
                escape_json(
                    &read_pascal_at(
                        chunk,
                        &mut cursor
                    )?
                )
            ),
        );
    }
    let kind = component
        .kind
        .label();
    let name = format!("{kind}_{kind_index:04}");
    let json = format!(
        r#"{{"schema":"history","num_lines":{},"history":[{}]}}"#,
        num_lines,
        lines.join(",")
    );
    Some(
        json_component(
            kind,
            &name,
            name.clone(),
            json,
            "decoded_schema_payload",
        ),
    )
}

/// Recover locator json.
fn recover_locator_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(
        component, source,
    )
    .ok()?;
    let mut cursor = 12;
    let name = read_pascal_at(
        chunk,
        &mut cursor,
    )?;
    let version = read_u32(
        chunk, cursor,
    )?;
    cursor += 4;
    let position = read_point(
        chunk,
        &mut cursor,
    )?;
    let kind = component
        .kind
        .label();
    let file_name = fallback_name(
        kind, kind_index, &name,
    );
    let json = format!(
        concat!(
            r#"{{"schema":"locator","#,
            r#""name":"{}","#,
            r#""version":{},"#,
            r#""position":[{},{},{}]}}"#,
        ),
        escape_json(&name),
        version,
        position[0],
        position[1],
        position[2]
    );
    Some(
        json_component(
            kind,
            &file_name,
            name,
            json,
            "decoded_schema_payload",
        ),
    )
}

/// Recover ped path json.
fn recover_ped_path_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(
        component, source,
    )
    .ok()?;
    let mut cursor = 12;
    let count = read_u32(
        chunk, cursor,
    )? as usize;
    cursor += 4;
    let mut points = Vec::new();
    for _ in 0..count {
        let p = read_point(
            chunk,
            &mut cursor,
        )?;
        points.push(
            format!(
                "[{},{},{}]",
                p[0], p[1], p[2]
            ),
        );
    }
    let kind = component
        .kind
        .label();
    let name = format!("{kind}_{kind_index:04}");
    let json = format!(
        r#"{{"schema":"ped_path","num_points":{},"points":[{}]}}"#,
        count,
        points.join(",")
    );
    Some(
        json_component(
            kind,
            &name,
            name.clone(),
            json,
            "decoded_schema_payload",
        ),
    )
}

/// Recover follow cam json.
fn recover_follow_cam_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(
        component, source,
    )
    .ok()?;
    let mut cursor = 12;
    let id = read_u32(
        chunk, cursor,
    )?;
    cursor += 4;
    let rotation = read_f32(
        chunk, cursor,
    )?;
    cursor += 4;
    let elevation = read_f32(
        chunk, cursor,
    )?;
    cursor += 4;
    let magnitude = read_f32(
        chunk, cursor,
    )?;
    cursor += 4;
    let target_offset = read_point(
        chunk,
        &mut cursor,
    )?;
    let kind = component
        .kind
        .label();
    let name = format!("{kind}_{kind_index:04}");
    let json = format!(
        concat!(
            r#"{{"schema":"follow_cam","#,
            r#""id":{},"#,
            r#""rotation":{},"#,
            r#""elevation":{},"#,
            r#""magnitude":{},"#,
            r#""target_offset":[{},{},{}]}}"#,
        ),
        id,
        rotation,
        elevation,
        magnitude,
        target_offset[0],
        target_offset[1],
        target_offset[2]
    );
    Some(
        json_component(
            kind,
            &name,
            name.clone(),
            json,
            "decoded_schema_payload",
        ),
    )
}

/// Recover export info json.
fn recover_export_info_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(
        component, source,
    )
    .ok()?;
    let mut cursor = 12;
    let name = read_pascal_at(
        chunk,
        &mut cursor,
    )?;
    let children = child_chunks_json(
        chunk,
        component.header_size,
        component.total_size,
    );
    let kind = component
        .kind
        .label();
    let file_name = fallback_name(
        kind, kind_index, &name,
    );
    let json = format!(
        r#"{{"schema":"export_info","name":"{}","entries":[{}]}}"#,
        escape_json(&name),
        children
    );
    Some(
        json_component(
            kind,
            &file_name,
            name,
            json,
            "decoded_schema_payload",
        ),
    )
}

/// Recover breakable object json.
fn recover_breakable_object_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(
        component, source,
    )
    .ok()?;
    let breakable_type = read_u32(
        chunk, 12,
    )?;
    let max_instances = read_u32(
        chunk, 16,
    )?;
    let children = child_chunks_json(
        chunk,
        component.header_size,
        component.total_size,
    );
    let kind = component
        .kind
        .label();
    let name = format!("{kind}_{kind_index:04}");
    let json = format!(
        concat!(
            r#"{{"schema":"breakable_object","#,
            r#""breakable_type":{},"#,
            r#""max_instances":{},"#,
            r#""children":[{}]}}"#,
        ),
        breakable_type, max_instances, children
    );
    Some(
        json_component(
            kind,
            &name,
            name.clone(),
            json,
            "decoded_schema_payload",
        ),
    )
}

/// Recover lens flare json.
fn recover_lens_flare_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(
        component, source,
    )
    .ok()?;
    let mut cursor = 12;
    let name = read_pascal_at(
        chunk,
        &mut cursor,
    )?;
    let version = read_u32(
        chunk, cursor,
    )?;
    cursor += 4;
    let num_billboard_quads = read_u32(
        chunk, cursor,
    )?;
    let children = child_chunks_json(
        chunk,
        component.header_size,
        component.total_size,
    );
    let kind = component
        .kind
        .label();
    let file_name = fallback_name(
        kind, kind_index, &name,
    );
    let json = format!(
        concat!(
            r#"{{"schema":"lens_flare_dsg","#,
            r#""name":"{}","#,
            r#""version":{},"#,
            r#""num_billboard_quads":{},"#,
            r#""children":[{}]}}"#,
        ),
        escape_json(&name),
        version,
        num_billboard_quads,
        children
    );
    Some(
        json_component(
            kind,
            &file_name,
            name,
            json,
            "decoded_schema_payload",
        ),
    )
}

/// Recover attribute table json.
fn recover_attribute_table_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(
        component, source,
    )
    .ok()?;
    let mut cursor = 12;
    let num_rows = read_u32(
        chunk, cursor,
    )? as usize;
    cursor += 4;
    let mut rows = Vec::new();
    for _ in 0..num_rows {
        let sound = read_pascal_at(
            chunk,
            &mut cursor,
        )?;
        let particle = read_pascal_at(
            chunk,
            &mut cursor,
        )?;
        let animation = read_pascal_at(
            chunk,
            &mut cursor,
        )?;
        let friction = read_f32(
            chunk, cursor,
        )?;
        cursor += 4;
        let mass = read_f32(
            chunk, cursor,
        )?;
        cursor += 4;
        let elasticity = read_f32(
            chunk, cursor,
        )?;
        cursor += 4;
        rows.push(
            format!(
                concat!(
                    r#"{{"sound":"{}","#,
                    r#""particle":"{}","#,
                    r#""animation":"{}","#,
                    r#""friction":{},"#,
                    r#""mass":{},"#,
                    r#""elasticity":{}}}"#,
                ),
                escape_json(&sound),
                escape_json(&particle),
                escape_json(&animation),
                friction,
                mass,
                elasticity,
            ),
        );
    }
    let kind = component
        .kind
        .label();
    let name = format!("{kind}_{kind_index:04}");
    let json = format!(
        r#"{{"schema":"attribute_table","num_rows":{},"rows":[{}]}}"#,
        num_rows,
        rows.join(",")
    );
    Some(
        json_component(
            kind,
            &name,
            name.clone(),
            json,
            "decoded_schema_payload",
        ),
    )
}

/// Recover animated object json.
fn recover_animated_object_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(
        component, source,
    )
    .ok()?;
    let mut cursor = 12;
    let version = read_u32(
        chunk, cursor,
    )?;
    cursor += 4;
    let name = read_pascal_at(
        chunk,
        &mut cursor,
    )?;
    let factory_name = read_pascal_at(
        chunk,
        &mut cursor,
    )?;
    let starting_animation = read_u32(
        chunk, cursor,
    )?;
    let kind = component
        .kind
        .label();
    let file_name = fallback_name(
        kind, kind_index, &name,
    );
    let json = format!(
        concat!(
            r#"{{"schema":"animated_object","#,
            r#""version":{},"#,
            r#""name":"{}","#,
            r#""factory_name":"{}","#,
            r#""starting_animation":{}}}"#,
        ),
        version,
        escape_json(&name),
        escape_json(&factory_name),
        starting_animation
    );
    Some(
        json_component(
            kind,
            &file_name,
            name,
            json,
            "decoded_schema_payload",
        ),
    )
}

/// Recover animated object factory json.
fn recover_animated_object_factory_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(
        component, source,
    )
    .ok()?;
    let mut cursor = 12;
    let version = read_u32(
        chunk, cursor,
    )?;
    cursor += 4;
    let name = read_pascal_at(
        chunk,
        &mut cursor,
    )?;
    let base_object_name = read_pascal_at(
        chunk,
        &mut cursor,
    )?;
    let num_animations = read_u32(
        chunk, cursor,
    )?;
    let children = child_chunks_json(
        chunk,
        component.header_size,
        component.total_size,
    );
    let kind = component
        .kind
        .label();
    let file_name = fallback_name(
        kind, kind_index, &name,
    );
    let json = format!(
        concat!(
            r#"{{"schema":"animated_object_factory","#,
            r#""version":{},"#,
            r#""name":"{}","#,
            r#""base_object_name":"{}","#,
            r#""num_animations":{},"#,
            r#""children":[{}]}}"#,
        ),
        version,
        escape_json(&name),
        escape_json(&base_object_name),
        num_animations,
        children
    );
    Some(
        json_component(
            kind,
            &file_name,
            name,
            json,
            "decoded_schema_payload",
        ),
    )
}

/// Recover state prop json.
fn recover_state_prop_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(
        component, source,
    )
    .ok()?;
    let mut cursor = 12;
    let version = read_u32(
        chunk, cursor,
    )?;
    cursor += 4;
    let name = read_pascal_at(
        chunk,
        &mut cursor,
    )?;
    let object_factory_name = read_pascal_at(
        chunk,
        &mut cursor,
    )?;
    let num_states = read_u32(
        chunk, cursor,
    )?;
    let children = child_chunks_json(
        chunk,
        component.header_size,
        component.total_size,
    );
    let kind = component
        .kind
        .label();
    let file_name = fallback_name(
        kind, kind_index, &name,
    );
    let json = format!(
        concat!(
            r#"{{"schema":"state_prop","#,
            r#""version":{},"#,
            r#""name":"{}","#,
            r#""object_factory_name":"{}","#,
            r#""num_states":{},"#,
            r#""children":[{}]}}"#,
        ),
        version,
        escape_json(&name),
        escape_json(&object_factory_name),
        num_states,
        children
    );
    Some(
        json_component(
            kind,
            &file_name,
            name,
            json,
            "decoded_schema_payload",
        ),
    )
}

/// Recover vertex expression json.
fn recover_vertex_expression_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(
        component, source,
    )
    .ok()?;
    let kind = component
        .kind
        .label();
    let name = component_name(
        component, source, kind_index,
    );
    let file_name = fallback_name(
        kind, kind_index, &name,
    );
    let json = vertex_expression_json(
        kind, chunk,
    )?;
    Some(
        json_component(
            kind,
            &file_name,
            name,
            json,
            "decoded_schema_payload",
        ),
    )
}

/// Child chunks json.
fn child_chunks_json(
    chunk: &[u8],
    mut cursor: usize,
    end: usize,
) -> String {
    let mut children = Vec::new();
    while cursor + 12 <= end {
        let Some((id, header_size, total_size)) = read_chunk_header(
            chunk, cursor,
        ) else {
            break;
        };
        let next = cursor.saturating_add(total_size);
        if total_size < header_size || next > end {
            break;
        }
        children.push(
            format!(
                concat!(
                    r#"{{"id_hex":"0x{:08x}","#,
                    r#""header_size":{},"#,
                    r#""total_size":{},"#,
                    r#""payload_size":{}}}"#,
                ),
                id,
                header_size,
                total_size,
                total_size.saturating_sub(header_size)
            ),
        );
        cursor = next;
    }
    children.join(",")
}

/// Read u16.
fn read_u16(
    bytes: &[u8],
    offset: usize,
) -> Option<u16> {
    let end = offset.checked_add(2)?;
    let slice = bytes.get(offset..end)?;
    Some(
        u16::from_le_bytes(
            [
                slice[0], slice[1],
            ],
        ),
    )
}

#[cfg(test)]
#[test]
fn extractor_u32_reader_rejects_offset_overflow() -> Result<(), String> {
    if read_u32(
        &[],
        usize::MAX,
    )
    .is_some()
    {
        return Err(
            String::from(
                "extractor u32 reads must reject an offset that cannot \
                 contain four bytes",
            ),
        );
    }
    Ok(())
}
#[cfg(test)]
#[test]
fn extractor_f32_reader_rejects_offset_overflow() -> Result<(), String> {
    if read_f32(
        &[],
        usize::MAX,
    )
    .is_some()
    {
        return Err(
            String::from(
                "extractor f32 reads must reject an offset that cannot \
                 contain four bytes",
            ),
        );
    }
    Ok(())
}
#[cfg(test)]
#[test]
fn extractor_fourcc_reader_rejects_offset_overflow() -> Result<(), String> {
    if read_fourcc(
        &[],
        usize::MAX,
    )
    .is_some()
    {
        return Err(
            String::from(
                "extractor FOURCC reads must reject an offset that cannot \
                 contain four bytes",
            ),
        );
    }
    Ok(())
}
#[cfg(test)]
#[test]
fn extractor_fourcc_reader_rejects_invalid_utf8() -> Result<(), String> {
    let value = read_fourcc(
        &[
            b'A', b'B', b'C', 0xff,
        ],
        0,
    );
    if value.is_some() {
        return Err(
            String::from(
                "FOURCC reads must reject invalid UTF-8 without replacement",
            ),
        );
    }
    Ok(())
}
#[cfg(test)]
#[test]
fn extractor_u16_reader_rejects_offset_overflow() -> Result<(), String> {
    if read_u16(
        &[],
        usize::MAX,
    )
    .is_some()
    {
        return Err(
            String::from(
                "extractor u16 reads must reject an offset that cannot \
                 contain two bytes",
            ),
        );
    }
    Ok(())
}
#[cfg(test)]
#[test]
fn truncated_pascal_read_preserves_cursor() -> Result<(), String> {
    let mut cursor = 0_usize;
    let value = read_pascal_at(
        &[
            4, b'a',
        ],
        &mut cursor,
    );
    if value.is_some() {
        return Err(String::from("truncated Pascal strings must fail"));
    }
    if cursor != 0 {
        return Err(
            String::from(
                "failed Pascal string reads must preserve the caller cursor",
            ),
        );
    }
    Ok(())
}
#[cfg(test)]
#[test]
fn invalid_utf8_pascal_read_preserves_cursor() -> Result<(), String> {
    let mut cursor = 0_usize;
    let value = read_pascal_at(
        &[
            1, 0xff,
        ],
        &mut cursor,
    );
    if value.is_some() {
        return Err(String::from("invalid UTF-8 Pascal strings must fail"));
    }
    if cursor != 0 {
        return Err(
            String::from(
                "invalid UTF-8 Pascal reads must preserve the caller cursor",
            ),
        );
    }
    Ok(())
}
#[cfg(test)]
#[test]
fn pascal_read_preserves_significant_whitespace() -> Result<(), String> {
    let mut cursor = 0_usize;
    let value = read_pascal_at(
        &[
            3, b' ', b'a', b' ',
        ],
        &mut cursor,
    )
    .ok_or_else(|| String::from("valid Pascal string should decode"))?;
    if value != " a " {
        return Err(
            String::from(
                "Pascal reads must preserve significant edge whitespace",
            ),
        );
    }
    Ok(())
}
#[cfg(test)]
#[test]
fn pascal_read_preserves_declared_null_data() -> Result<(), String> {
    let mut cursor = 0_usize;
    let value = read_pascal_at(
        &[
            2, b'a', 0,
        ],
        &mut cursor,
    )
    .ok_or_else(|| String::from("valid Pascal string should decode"))?;
    if value != "a\0" {
        return Err(
            String::from(
                "Pascal reads must preserve declared trailing null data",
            ),
        );
    }
    Ok(())
}
#[cfg(test)]
#[test]
fn pascal_component_name_preserves_edge_spaces() -> Result<(), String> {
    let component = ChunkRecord {
        ordinal: 0,
        depth: 0,
        parent_ordinal: None,
        id: 0,
        kind: crate::ChunkKind::Unknown,
        offset: 0,
        header_size: 16,
        total_size: 16,
        payload_offset: 16,
        payload_size: 0,
        child_count: 0,
    };
    let mut source = vec![0_u8; 12];
    source.extend_from_slice(
        &[
            3, b' ', b'a', b' ',
        ],
    );
    let name = read_pascal_name(
        &component, &source,
    )
    .ok_or_else(|| String::from("valid component name should decode"))?;
    if name != " a " {
        return Err(
            String::from(
                "component names must preserve significant edge spaces",
            ),
        );
    }
    Ok(())
}
