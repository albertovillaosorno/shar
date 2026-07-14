// File:
//   - editor.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/two/units/editor.rs
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
//   - The editor contract for pipeline phase two minor units.
// - Must-Not:
//   - Violate repository architecture, path, provenance, or output rules.
// - Allows:
//   - Operations required to validate and execute editor.
// - Split-When:
//   - Split when editor contains two independently testable contracts.
// - Merge-When:
//   - Another pipeline module owns the same module boundary with no distinct
//   - invariant.
// - Summary:
//   - Edit minor unit metadata.
// - Description:
//   - Defines editor data and behavior for pipeline phase two minor units.
// - Usage:
//   - Used by pipeline phase two minor units code that needs editor.
// - Defaults:
//   - No implicit output outside the repository is allowed.
//
// ADRs:
// - docs/adr/pipeline/minor-unit-taxonomy-value-case.md
//
// Large file:
//   - true
//   - Reason: Edit minor unit metadata keeps tightly coupled validation,
//   - ordering, and deterministic transformation invariants together; split
//   - when a stable independently testable sub-boundary is identified.
//

//! Edit minor unit metadata.
//!
//! This boundary keeps edit minor unit metadata explicit and returns
//! deterministic results to pipeline callers.
use std::path::Path;

use schoenwald_filesystem::adapters::driving::local::{
    read_utf8 as local_read_utf8, write_text as local_write_text,
};

use super::metadata_fill::{compute_id, read_string_field};
use super::taxonomy;
use crate::domain::{PipelineError, StageReport};

/// Result.
type PipelineOutcome<T> = Result<T, PipelineError>;

/// Edit minor unit metadata.
///
/// # Errors
///
/// Returns an error when validation, filesystem access, or output writing
/// fails.
pub(in crate::adapters::driven::local) fn edit_minor_unit_metadata(
    extracted_root: &Path
) -> PipelineOutcome<StageReport> {
    let manifest_path = taxonomy::manifest_path(extracted_root);
    let input =
        local_read_utf8(&manifest_path).map_err(io_error(&manifest_path))?;
    let mut output = String::new();
    let mut rows = 0usize;
    let mut edited = 0usize;

    for line in input.lines() {
        if line
            .trim()
            .is_empty()
        {
            continue;
        }
        let edited_line = edit_row(line);
        if edited_line != line {
            edited = edited.saturating_add(1);
        }
        output.push_str(&edited_line);
        output.push('\n');
        rows = rows.saturating_add(1);
    }

    local_write_text(
        &manifest_path,
        &output,
        true,
    )
    .map_err(io_error(&manifest_path))?;
    Ok(
        StageReport {
            name: "minor-unit-metadata-editor",
            files: rows,
            bytes: u64::try_from(edited).unwrap_or(u64::MAX),
            note: format!(
                "batch metadata editor refined {edited}/{rows} minor-unit rows"
            ),
        },
    )
}

/// Edit row.
fn edit_row(line: &str) -> String {
    let type_ = read_string_field(
        line, "type",
    )
    .unwrap_or_default();
    let kind = read_string_field(
        line, "kind",
    )
    .unwrap_or_default();
    let origin = read_string_field(
        line, "origin",
    )
    .unwrap_or_default();
    let future = read_string_field(
        line,
        "future_normalization",
    )
    .unwrap_or_default();
    let path = read_string_field(
        line, "path",
    )
    .unwrap_or_default();

    if origin == "p3d-package"
        && (kind == "derived-component"
            || type_ == "package-component"
            || future == "p3d-component-to-mesh-material-texture")
    {
        return rewrite_p3d_component_row(
            line, &path,
        );
    }

    line.to_owned()
}

/// Rewrite p3d component row.
fn rewrite_p3d_component_row(
    line: &str,
    path: &str,
) -> String {
    let segment = component_segment(path);
    let bucket = component_bucket(segment);
    let subtype = format!("p3d-{segment}-component");
    let function = format!(
        "Pure3D {segment} component for {}",
        bucket.function
    );
    let note =
        format!("batch-editor-classified-p3d-component-by-segment:{segment}");
    // The segment rewrite changes the final type, so the identity (which leads
    // with the type and hashes it) must be recomputed from the name-free route
    // and recovery status this row already carries.
    let refreshed_id = compute_id(
        &read_string_field(
            line,
            "obfuscated_route",
        )
        .unwrap_or_default(),
        bucket.type_,
        &read_string_field(
            line,
            "recovery_status",
        )
        .unwrap_or_default(),
    );
    replace_many_owned(
        line,
        &[
            (
                "id",
                refreshed_id,
            ),
            (
                "type",
                bucket
                    .type_
                    .to_owned(),
            ),
            (
                "subtype", subtype,
            ),
            (
                "kind",
                bucket
                    .kind
                    .to_owned(),
            ),
            (
                "function", function,
            ),
            (
                "unreal_import_relation",
                bucket
                    .relation
                    .to_owned(),
            ),
            (
                "future_normalization",
                bucket
                    .future
                    .to_owned(),
            ),
            (
                "classification_notes",
                note,
            ),
        ],
    )
}

/// Component segment.
fn component_segment(path: &str) -> &str {
    let parts = path
        .split('/')
        .collect::<Vec<_>>();
    if let Some(index) = parts
        .iter()
        .position(|part| *part == "components")
        && let Some(segment) = parts.get(index.saturating_add(1))
    {
        return segment;
    }
    "raw"
}

/// Componentbucket.
struct ComponentBucket {
    /// Str.
    type_: &'static str,
    /// Str.
    kind: &'static str,
    /// Str.
    relation: &'static str,
    /// Str.
    future: &'static str,
    /// Str.
    function: &'static str,
}

/// Component bucket.
// This scoped expectation preserves a documented boundary with explicit
// validation.
#[expect(
    clippy::too_many_lines,
    reason = "The function preserves a deterministic extraction/reporting \
              sequence; splitting it now would hide ordering guarantees \
              tracked by ADRs."
)]
// This scoped expectation preserves a documented boundary with explicit
// validation.
#[expect(
    clippy::match_same_arms,
    reason = "Equivalent component variants intentionally share one Unreal \
              import bucket."
)]
fn component_bucket(segment: &str) -> ComponentBucket {
    match segment {
        "texture" => bucket(
            "image",
            "p3d-texture",
            "compose-into-asset",
            "p3d-texture-to-texture2d",
            "texture asset reconstruction",
        ),
        "texture_font" => bucket(
            "ui",
            "p3d-texture-font",
            "compose-into-asset",
            "p3d-texture-font-to-font-asset",
            "font atlas reconstruction",
        ),
        "text_bible" => bucket(
            "localization",
            "p3d-text-bible",
            "import-as-data-asset",
            "p3d-text-bible-to-string-table",
            "Pure3D embedded text bible reconstruction",
        ),
        "export_info" => bucket(
            "metadata",
            "p3d-export-info",
            "editor-only-metadata",
            "p3d-export-info-to-editor-metadata",
            "export provenance metadata",
        ),
        "scrooby_project" => bucket(
            "ui",
            "p3d-scrooby-project",
            "compose-into-asset",
            "p3d-scrooby-project-to-ui-project",
            "Scrooby UI project reconstruction",
        ),
        "vertex_anim_key"
        | "vertex_expression_group"
        | "vertex_expression_mixer" => bucket(
            "animation",
            "p3d-vertex-animation",
            "compose-into-asset",
            "p3d-vertex-animation-to-morph-targets",
            "vertex animation and expression reconstruction",
        ),
        "animated_object" | "animated_object_factory" | "state_prop" => bucket(
            "world",
            "p3d-animated-prop",
            "compose-into-asset",
            "p3d-animated-prop-to-actor-blueprint",
            "animated world prop reconstruction",
        ),
        "light_group" => bucket(
            "light",
            "p3d-light-group",
            "compose-into-asset",
            "p3d-light-group-to-light-rig",
            "light rig reconstruction",
        ),
        "frame_controller_variant_a" | "frame_controller_variant_b" => bucket(
            "controller",
            "p3d-controller",
            "compose-into-asset",
            "p3d-controller-to-animation",
            "animation/controller binding reconstruction",
        ),
        "shader" => bucket(
            "material",
            "p3d-shader",
            "compose-into-asset",
            "p3d-shader-to-material",
            "material/shader reconstruction",
        ),
        "game_attr" => bucket(
            "material",
            "p3d-attribute",
            "compose-into-asset",
            "p3d-attribute-to-physical-material",
            "surface and gameplay material attributes",
        ),
        "mesh" | "quad_group" => bucket(
            "model",
            "p3d-mesh",
            "compose-into-asset",
            "p3d-mesh-to-static-mesh",
            "static mesh geometry reconstruction",
        ),
        "skin" => bucket(
            "model",
            "p3d-skin",
            "compose-into-asset",
            "p3d-skin-to-skeletal-mesh",
            "skinned mesh reconstruction",
        ),
        "composite_drawable" => bucket(
            "model",
            "p3d-composite-drawable",
            "compose-into-asset",
            "p3d-composite-drawable-to-blueprint",
            "composed drawable asset assembly",
        ),
        "sprite" => bucket(
            "image",
            "p3d-sprite",
            "compose-into-asset",
            "p3d-sprite-to-texture2d",
            "sprite texture reconstruction",
        ),
        "animation" => bucket(
            "animation",
            "p3d-animation",
            "compose-into-asset",
            "p3d-animation-to-animation-asset",
            "animation clip reconstruction",
        ),
        "skeleton" => bucket(
            "animation",
            "p3d-skeleton",
            "compose-into-asset",
            "p3d-skeleton-to-skeleton-asset",
            "skeleton hierarchy reconstruction",
        ),
        "frame_controller" | "multi_controller" => bucket(
            "controller",
            "p3d-controller",
            "compose-into-asset",
            "p3d-controller-to-animation",
            "animation/controller binding reconstruction",
        ),
        "camera" => bucket(
            "camera",
            "p3d-camera",
            "compose-into-asset",
            "p3d-camera-to-camera-actor",
            "camera actor reconstruction",
        ),
        "light" => bucket(
            "light",
            "p3d-light",
            "compose-into-asset",
            "p3d-light-to-light-component",
            "light component reconstruction",
        ),
        "particle_system" | "particle_system_factory" => bucket(
            "particle",
            "p3d-particle",
            "compose-into-asset",
            "p3d-particle-to-niagara",
            "particle effect reconstruction",
        ),
        "scenegraph" | "history" => bucket(
            "scene",
            "p3d-scenegraph",
            "compose-into-asset",
            "p3d-scenegraph-to-level-hierarchy",
            "scene hierarchy reconstruction",
        ),
        "locator" | "srr_locator" => bucket(
            "locator",
            "p3d-locator",
            "compose-into-asset",
            "p3d-locator-to-scene-component",
            "locator and marker reconstruction",
        ),
        "srr_road" | "srr_intersection" | "srr_road_segment_data" => bucket(
            "world",
            "p3d-road-network",
            "compose-into-asset",
            "p3d-road-to-spline-network",
            "road and traffic graph reconstruction",
        ),
        "srr_ped_path" => bucket(
            "world",
            "p3d-ped-path",
            "compose-into-asset",
            "p3d-ped-path-to-spline",
            "pedestrian path reconstruction",
        ),
        "simulation_collision_object" => bucket(
            "physics",
            "p3d-collision",
            "compose-into-asset",
            "p3d-physics-to-collision",
            "collision body reconstruction",
        ),
        "simulation_physics_object"
        | "srr_static_phys_dsg"
        | "srr_dyna_phys_dsg"
        | "srr_insta_anim_dyna_phys_dsg"
        | "srr_insta_static_phys_dsg" => bucket(
            "physics",
            "p3d-physics",
            "compose-into-asset",
            "p3d-physics-to-collision",
            "physics object reconstruction",
        ),
        "srr_fence_dsg"
        | "srr_intersect_dsg"
        | "srr_entity_dsg"
        | "srr_anim_dsg"
        | "srr_breakable_object"
        | "srr_follow_cam" => bucket(
            "world",
            "p3d-world-dsg",
            "compose-into-asset",
            "p3d-world-dsg-to-actor-data",
            "world DSG actor reconstruction",
        ),
        _ if segment.starts_with("srr_") => bucket(
            "world",
            "p3d-world-dsg",
            "compose-into-asset",
            "p3d-world-dsg-to-actor-data",
            "world DSG actor reconstruction",
        ),
        _ => error_bucket("unmapped component classification failure"),
    }
}

/// Error bucket.
const fn error_bucket(reason: &'static str) -> ComponentBucket {
    bucket(
        "error", "error", "error", "error", reason,
    )
}

/// Bucket.
const fn bucket(
    type_: &'static str,
    kind: &'static str,
    relation: &'static str,
    future: &'static str,
    function: &'static str,
) -> ComponentBucket {
    ComponentBucket {
        type_,
        kind,
        relation,
        future,
        function,
    }
}

/// Replace many owned.
fn replace_many_owned(
    line: &str,
    replacements: &[(
        &str,
        String,
    )],
) -> String {
    let mut output = line.to_owned();
    for (field, value) in replacements {
        output = replace_field(
            &output, field, value,
        );
    }
    output
}

/// Replace field.
fn replace_field(
    line: &str,
    field: &str,
    value: &str,
) -> String {
    let needle = format!("\"{field}\":\"");
    let Some(start) = line.find(&needle) else {
        return line.to_owned();
    };
    let value_start = start.saturating_add(needle.len());
    let Some(value_end_offset) = line
        .get(value_start..)
        .unwrap_or_default()
        .find('"')
    else {
        return line.to_owned();
    };
    let value_end = value_start.saturating_add(value_end_offset);
    let mut output = String::new();
    output.push_str(
        line.get(..value_start)
            .unwrap_or_default(),
    );
    output.push_str(value);
    output.push_str(
        line.get(value_end..)
            .unwrap_or_default(),
    );
    output
}

/// Io error.
fn io_error(path: &Path) -> impl FnOnce(std::io::Error) -> PipelineError + '_ {
    move |error| {
        PipelineError::new(
            format!(
                "{}: {error}",
                path.display()
            ),
        )
    }
}
