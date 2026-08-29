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
//   - Scrooby runtime-layout semantic unit regressions.
// - Must-Not:
//   - Depend on the lawful source installation or authored UI content.
// - Allows:
//   - Synthetic signed-coordinate and justification assertions.
// - Split-When:
//   - Layout publication gains independent fixture ownership.
// - Merge-When:
//   - Another test module owns the identical layout semantics.
// - Summary:
//   - Pin source-runtime Scrooby layout conversions.
// - Description:
//   - Verifies bit-preserving signed coordinates, axis justification, and
//     polygon screen-coordinate truncation.
// - Usage:
//   - Included only by the Scrooby layout adapter under cfg(test).
// - Defaults:
//   - Unsupported enum values fail closed.
//

//! Scrooby runtime-layout unit regressions.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

use super::{
    Component, add_bounded_alignment_policy, add_multi_text,
    add_owner_color_policy, add_polygon, add_runtime_field_consumption,
    add_semantics, collect_package_layout,
    horizontal_justification,
    packed_rgba_u8, publish_rendered, screen_i32, semantic_i32,
    vertical_justification,
};

static CASE_ID: AtomicUsize = AtomicUsize::new(0);

type TestResult = Result<(), String>;

type TestPathResult = Result<PathBuf, String>;
type TestRowResult = Result<String, String>;

fn case_dir(label: &str) -> TestPathResult {
    let path = std::env::temp_dir().join(format!(
        "shar-scrooby-layout-{label}-{}-{}",
        std::process::id(),
        CASE_ID.fetch_add(1, Ordering::Relaxed),
    ));
    if path.exists() {
        fs::remove_dir_all(&path).map_err(|error| error.to_string())?;
    }
    fs::create_dir_all(path.join("components"))
        .map_err(|error| error.to_string())?;
    Ok(path)
}

fn write_component(
    root: &Path,
    ordinal: usize,
    parent_ordinal: usize,
    kind: &str,
    name: &str,
    payload: &str,
) -> TestRowResult {
    let relative = format!("{kind}/{name}.json");
    let path = root.join("components").join(&relative);
    let parent = path
        .parent()
        .ok_or_else(|| "layout component path has no parent".to_owned())?;
    fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    fs::write(&path, payload).map_err(|error| error.to_string())?;
    Ok(format!(
        concat!(
            r#"{{"ordinal":{},"parent_ordinal":{},"#,
            r#""kind":"{}","path":"{}"}}"#,
        ),
        ordinal,
        parent_ordinal,
        kind,
        relative,
    ))
}


#[test]
fn layout_reuse_rejects_transaction_debris() -> TestResult {
    let root = case_dir("reuse-debris")?;
    let output = root.join("layout-output");
    let rendered = concat!(
        r#"{"layout_count":0,"record_type":"header","#,
        r#""schema":"shar-schoenwald.scrooby-layout-catalog.v14","#,
        r#""status":"complete"}"#,
        "\n",
    );
    publish_rendered(&output, rendered).map_err(|error| error.to_string())?;
    let accepted = fs::read_to_string(output.join("layout.jsonl"))
        .map_err(|error| error.to_string())?;
    let parent = output
        .parent()
        .ok_or_else(|| "layout output has no parent".to_owned())?;
    let name = output
        .file_name()
        .and_then(|value| value.to_str())
        .ok_or_else(|| "layout output has no file name".to_owned())?;
    for (suffix, expected) in [
        ("complete-staging", "Scrooby layout staging already exists"),
        ("complete-backup", "Scrooby layout backup already exists"),
    ] {
        let debris = parent.join(format!(".{name}.{suffix}"));
        fs::create_dir_all(&debris).map_err(|error| error.to_string())?;
        let result = publish_rendered(&output, rendered);
        fs::remove_dir_all(&debris).map_err(|error| error.to_string())?;
        let Err(error) = result else {
            return Err(format!("layout reuse accepted {suffix} debris"));
        };
        if !error.to_string().contains(expected) {
            return Err(format!("unexpected {suffix} debris error: {error}"));
        }
        let unchanged = fs::read_to_string(output.join("layout.jsonl"))
            .map_err(|error| error.to_string())?;
        if unchanged != accepted {
            return Err(format!("{suffix} debris changed accepted layout"));
        }
    }
    fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    Ok(())
}

#[test]
fn layout_publisher_rejects_non_directory_output() -> TestResult {
    let root = case_dir("non-directory")?;
    let output = root.join("layout-output");
    fs::write(&output, b"not a directory").map_err(|error| error.to_string())?;
    let result = publish_rendered(&output, "{}\n");
    fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    let Err(error) = result else {
        return Err("layout publisher accepted a file output root".to_owned());
    };
    if !error
        .to_string()
        .contains("Scrooby layout output is not a regular directory")
    {
        return Err(format!("unexpected non-directory error: {error}"));
    }
    Ok(())
}

#[test]
fn runtime_indices_follow_source_parent_child_semantics() -> TestResult {
    let root = case_dir("runtime-indices")?;
    let rows = [
        write_component(
            &root,
            1,
            0,
            "scrooby_project",
            "project",
            concat!(
                r#"{"schema":"scrooby_project","version":10,"#,
                r#""resolution":[640,480]}"#,
            ),
        )?,
        write_component(
            &root,
            2,
            1,
            "scrooby_page",
            "page",
            r#"{"schema":"scrooby_page","version":11,"resolution":[640,480]}"#,
        )?,
        write_component(
            &root,
            3,
            2,
            "scrooby_image_resource",
            "resource",
            r#"{"schema":"scrooby_image_resource"}"#,
        )?,
        write_component(
            &root,
            4,
            2,
            "scrooby_layer",
            "layer",
            concat!(
                r#"{"schema":"scrooby_layer","version":12,"visible":1,"#,
                r#""editable":1,"alpha":255}"#,
            ),
        )?,
        write_component(
            &root,
            5,
            4,
            "scrooby_multi_text",
            "text",
            concat!(
                r#"{"schema":"scrooby_multi_text","position":[0,0],"#,
                r#""dimensions":[100,20],"justification":[0,2],"#,
                r#""color":4294967295,"translucency":0,"rotation":0,"#,
                r#""version":17,"shadow_enabled":0,"shadow_color":0,"#,
                r#""shadow_offset":[0,0],"current_text":0}"#,
            ),
        )?,
        write_component(
            &root,
            6,
            5,
            "scrooby_string_hardcoded",
            "string",
            r#"{"schema":"scrooby_string_hardcoded"}"#,
        )?,
        write_component(
            &root,
            7,
            1,
            "scrooby_screen",
            "screen",
            r#"{"schema":"scrooby_screen","version":13}"#,
        )?,
    ];
    let mut ledger = String::from(
        "{\"schema\":\"p3d.package.v1\",\"component_count\":7}\n",
    );
    for row in rows {
        ledger.push_str(&row);
        ledger.push('\n');
    }
    fs::write(root.join("components.jsonl"), ledger)
        .map_err(|error| error.to_string())?;

    let layout = collect_package_layout(&root)
        .map_err(|error| error.to_string())?;
    let row = |ordinal| {
        layout
            .iter()
            .find(|row| row.get("ordinal").and_then(serde_json::Value::as_u64)
                == Some(ordinal))
            .ok_or_else(|| format!("layout row {ordinal} is missing"))
    };
    let project = row(1)?;
    let page = row(2)?;
    let layer = row(4)?;
    let text = row(5)?;
    let string = row(6)?;
    let screen = row(7)?;
    let index = |value: &serde_json::Value, field: &str| {
        value.get(field).and_then(serde_json::Value::as_u64)
    };
    if index(project, "raw_version_u32") != Some(10)
        || project.get("runtime_version_consumed")
            != Some(&serde_json::json!(false))
        || index(page, "raw_version_u32") != Some(11)
        || page.get("runtime_version_consumed")
            != Some(&serde_json::json!(false))
        || index(layer, "raw_version_u32") != Some(12)
        || index(screen, "raw_version_u32") != Some(13)
        || screen.get("runtime_version_consumed")
            != Some(&serde_json::json!(false))
        || page.get("runtime_index") != Some(&serde_json::Value::Null)
        || page.get("canvas").is_some()
        || page.get("raw_resolution_u32")
            != Some(&serde_json::json!([640, 480]))
        || page.get("runtime_resolution_consumed")
            != Some(&serde_json::json!(false))
        || index(layer, "source_sibling_index") != Some(1)
        || index(layer, "runtime_index") != Some(0)
        || layer.get("runtime_editable_consumed")
            != Some(&serde_json::json!(false))
        || layer.get("runtime_alpha_consumed")
            != Some(&serde_json::json!(false))
        || text.get("runtime_translucency_consumed")
            != Some(&serde_json::json!(false))
        || text.get("runtime_rotation_consumed")
            != Some(&serde_json::json!(false))
        || index(string, "runtime_index") != Some(0)
        || index(screen, "source_sibling_index") != Some(1)
        || index(screen, "runtime_index") != Some(0)
        || screen.get("z_buffer_enabled") != Some(&serde_json::json!(false))
        || screen.get("projection_mode")
            != Some(&serde_json::json!("perspective"))
        || screen.get("cull_mode") != Some(&serde_json::json!("none"))
        || text.get("color_rgba_u8")
            != Some(&serde_json::json!([255, 255, 255, 255]))
        || text.get("shadow_color_rgba_u8")
            != Some(&serde_json::json!([0, 0, 0, 0]))
        || text.get("authored_shadow_enabled")
            != Some(&serde_json::json!(false))
        || text.get("runtime_shadow_enabled")
            != Some(&serde_json::json!(false))
        || text.get("runtime_outline_enabled")
            != Some(&serde_json::json!(false))
        || text.get("runtime_frame_normalization")
            != Some(&serde_json::json!(
                "position-and-bounds-both-axes-divide-by-project-width"
            ))
        || index(text, "runtime_frame_denominator_u32") != Some(640)
        || text.get("runtime_text_mode")
            != Some(&serde_json::json!("overlap"))
        || text.get("runtime_alignment_box_width_scale_f64")
            != Some(&serde_json::json!(2.0))
        || text.get("runtime_glyph_scale_policy")
            != Some(&serde_json::json!(
                "project-height-reciprocal-then-half"
            ))
        || index(text, "runtime_glyph_scale_denominator_u32") != Some(480)
        || text.get("runtime_alignment_metrics")
            != Some(&serde_json::json!("resolved-font-current-string"))
        || text.get("runtime_horizontal_alignment_policy")
            != Some(&serde_json::json!(
                "left-zero-right-box-minus-half-glyph-centre-half-box"
            ))
        || text.get("runtime_vertical_alignment_policy")
            != Some(&serde_json::json!(
                "centre-half-box-minus-glyph-otherwise-zero"
            ))
        || page.get("owner_color_modulation").is_some()
        || layer.get("owner_color_restore")
            != Some(&serde_json::json!("original-after-display"))
        || text.get("owner_color_modulation")
            != Some(&serde_json::json!(
                "rgba-component-multiply-u8-floor-before-display"
            ))
        || screen.get("owner_color_modulation").is_some()
        || index(text, "raw_version_u32") != Some(17)
        || text.get("runtime_version_consumed")
            != Some(&serde_json::json!(true))
        || index(text, "initial_index_i32") != Some(0)
    {
        return Err(format!(
            "unexpected runtime/layout semantics: page={page} layer={layer} \
             text={text} string={string} screen={screen}"
        ));
    }
    fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    Ok(())
}

#[test]
fn layout_rejects_zero_project_resolution() -> TestResult {
    for (label, resolution) in [
        ("zero-project-width", [0, 480]),
        ("zero-project-height", [640, 0]),
    ] {
        let root = case_dir(label)?;
        let project = write_component(
            &root,
            1,
            0,
            "scrooby_project",
            "project",
            &format!(
                concat!(
                    r#"{{"schema":"scrooby_project","version":10,"#,
                    r#""resolution":[{},{}]}}"#,
                ),
                resolution[0], resolution[1],
            ),
        )?;
        let ledger = format!(
            r#"{{"schema":"p3d.package.v1","component_count":1}}
{project}
"#
        );
        fs::write(root.join("components.jsonl"), ledger)
            .map_err(|error| error.to_string())?;
        let result = collect_package_layout(&root);
        fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
        let Err(error) = result else {
            return Err(format!("{label} was accepted"));
        };
        if error.to_string()
            != "Scrooby layout project resolution is non-positive"
        {
            return Err(format!("unexpected {label} error: {error}"));
        }
    }
    Ok(())
}

#[test]
fn signed_screen_values_preserve_source_bits() {
    assert_eq!(semantic_i32(0xffff_ffff), -1);
    assert_eq!(semantic_i32(0xffff_ffec), -20);
    assert_eq!(semantic_i32(0xffff_ffcd), -51);
    assert_eq!(semantic_i32(640), 640);
}

#[test]
fn packed_colors_follow_pddi_channel_order() {
    assert_eq!(packed_rgba_u8(0xc011_2233), [0x11, 0x22, 0x33, 0xc0]);
    assert_eq!(packed_rgba_u8(0xffff_ffff), [255, 255, 255, 255]);
    assert_eq!(packed_rgba_u8(0x0000_0000), [0, 0, 0, 0]);
}

#[test]
fn justification_matches_scrooby_runtime_axes() -> Result<(), String> {
    for (raw, expected) in [(0, "left"), (1, "right"), (4, "centre")] {
        assert_eq!(
            horizontal_justification(raw).map_err(|e| e.to_string())?,
            expected,
        );
    }
    for (raw, expected) in [(2, "top"), (3, "bottom"), (4, "centre")] {
        assert_eq!(
            vertical_justification(raw).map_err(|e| e.to_string())?,
            expected,
        );
    }
    assert!(horizontal_justification(2).is_err());
    assert!(vertical_justification(0).is_err());
    Ok(())
}

#[test]
fn text_zero_shadow_offset_promotes_runtime_outline() -> Result<(), String> {
    let payload = serde_json::json!({
        "position": [0, 0],
        "dimensions": [100, 20],
        "justification": [0, 2],
        "color": 4_294_967_295_u32,
        "translucency": 0,
        "rotation": 0,
        "version": 17,
        "shadow_enabled": 1,
        "shadow_color": 0xc011_2233_u32,
        "shadow_offset": [0, 0],
        "current_text": 2,
    });
    let mut row = serde_json::Map::new();
    add_multi_text(&payload, [640, 480], &mut row)
        .map_err(|error| error.to_string())?;
    assert_eq!(
        row.get("authored_shadow_enabled"),
        Some(&serde_json::json!(true)),
    );
    assert_eq!(
        row.get("runtime_shadow_enabled"),
        Some(&serde_json::json!(false)),
    );
    assert_eq!(
        row.get("runtime_outline_enabled"),
        Some(&serde_json::json!(true)),
    );
    assert_eq!(
        row.get("runtime_outline_color_rgba_u8"),
        Some(&serde_json::json!([0x11, 0x22, 0x33, 0xc0])),
    );
    assert_eq!(
        row.get("runtime_outline_pass_offsets_i32"),
        Some(&serde_json::json!([[-1, -1], [-1, 1], [1, -1], [1, 1]])),
    );
    assert_eq!(
        row.get("runtime_outline_thickness_font_fraction_f64"),
        Some(&serde_json::json!(0.05)),
    );
    assert!(row.get("runtime_shadow_offset_i32").is_none());
    Ok(())
}

#[test]
fn text_nonzero_shadow_offset_remains_runtime_shadow() -> Result<(), String> {
    let payload = serde_json::json!({
        "position": [0, 0],
        "dimensions": [100, 20],
        "justification": [0, 2],
        "color": 4_294_967_295_u32,
        "translucency": 0,
        "rotation": 0,
        "version": 17,
        "shadow_enabled": 1,
        "shadow_color": 0x8044_5566_u32,
        "shadow_offset": [3, 4_294_967_294_u32],
        "current_text": 1,
    });
    let mut row = serde_json::Map::new();
    add_multi_text(&payload, [640, 480], &mut row)
        .map_err(|error| error.to_string())?;
    assert_eq!(
        row.get("authored_shadow_offset_i32"),
        Some(&serde_json::json!([3, -2])),
    );
    assert_eq!(
        row.get("runtime_shadow_enabled"),
        Some(&serde_json::json!(true)),
    );
    assert_eq!(
        row.get("runtime_outline_enabled"),
        Some(&serde_json::json!(false)),
    );
    assert_eq!(
        row.get("runtime_shadow_offset_i32"),
        Some(&serde_json::json!([3, -2])),
    );
    assert!(row.get("runtime_outline_color_rgba_u8").is_none());
    Ok(())
}

#[test]
fn bounded_alignment_policy_matches_runtime_dispatch() {
    let mut sprite = serde_json::Map::new();
    add_bounded_alignment_policy("scrooby_multi_sprite", &mut sprite);
    assert_eq!(
        sprite.get("runtime_alignment_metrics"),
        Some(&serde_json::json!("resolved-current-sprite-bounds")),
    );
    assert_eq!(
        sprite.get("runtime_horizontal_alignment_policy"),
        Some(&serde_json::json!(
            "left-zero-right-difference-centre-half-difference"
        )),
    );
    assert_eq!(
        sprite.get("runtime_vertical_alignment_policy"),
        Some(&serde_json::json!(
            "bottom-zero-top-difference-centre-half-difference"
        )),
    );

    let mut text = serde_json::Map::new();
    add_bounded_alignment_policy("scrooby_multi_text", &mut text);
    assert_eq!(
        text.get("runtime_horizontal_alignment_policy"),
        Some(&serde_json::json!(
            "left-zero-right-box-minus-half-glyph-centre-half-box"
        )),
    );
    assert_eq!(
        text.get("runtime_vertical_alignment_policy"),
        Some(&serde_json::json!(
            "centre-half-box-minus-glyph-otherwise-zero"
        )),
    );

    let mut pure3d = serde_json::Map::new();
    add_bounded_alignment_policy("scrooby_pure3d_object", &mut pure3d);
    assert_eq!(
        pure3d.get("runtime_justification_consumed"),
        Some(&serde_json::json!(false)),
    );
    assert_eq!(
        pure3d.get("runtime_alignment_policy"),
        Some(&serde_json::json!("none-render-uses-base-drawable-matrix")),
    );
}

#[test]
fn owner_color_policy_matches_runtime_render_consumers() {
    for kind in [
        "scrooby_layer",
        "scrooby_group",
        "scrooby_multi_sprite",
        "scrooby_multi_text",
        "scrooby_polygon",
    ] {
        let mut row = serde_json::Map::new();
        add_owner_color_policy(kind, &mut row);
        assert_eq!(
            row.get("owner_color_modulation"),
            Some(&serde_json::json!(
                "rgba-component-multiply-u8-floor-before-display"
            )),
        );
    }
    for kind in [
        "scrooby_project",
        "scrooby_screen",
        "scrooby_page",
        "scrooby_pure3d_object",
    ] {
        let mut row = serde_json::Map::new();
        add_owner_color_policy(kind, &mut row);
        assert!(row.is_empty(), "unexpected color policy for {kind}");
    }
}

#[test]
fn ignored_authored_fields_are_not_promoted_to_runtime_state() {
    for (kind, expected) in [
        (
            "scrooby_layer",
            &["runtime_editable_consumed", "runtime_alpha_consumed"][..],
        ),
        ("scrooby_group", &["runtime_alpha_consumed"][..]),
        (
            "scrooby_multi_sprite",
            &[
                "runtime_translucency_consumed",
                "runtime_rotation_consumed",
            ][..],
        ),
        (
            "scrooby_multi_text",
            &[
                "runtime_translucency_consumed",
                "runtime_rotation_consumed",
            ][..],
        ),
        (
            "scrooby_pure3d_object",
            &[
                "runtime_translucency_consumed",
                "runtime_rotation_consumed",
            ][..],
        ),
        (
            "scrooby_polygon",
            &["runtime_translucency_consumed"][..],
        ),
    ] {
        let mut row = serde_json::Map::new();
        add_runtime_field_consumption(kind, &mut row);
        assert_eq!(
            row.len(),
            expected.len() + 1,
            "unexpected fields for {kind}",
        );
        assert_eq!(
            row.get("runtime_version_consumed"),
            Some(&serde_json::json!(kind == "scrooby_multi_text")),
        );
        for field in expected {
            assert_eq!(row.get(*field), Some(&serde_json::json!(false)));
        }
    }

    for kind in ["scrooby_project", "scrooby_page", "scrooby_screen"] {
        let mut row = serde_json::Map::new();
        add_runtime_field_consumption(kind, &mut row);
        assert_eq!(
            row.get("runtime_version_consumed"),
            Some(&serde_json::json!(false)),
        );
        assert_eq!(row.len(), 1, "unexpected fields for {kind}");
    }

    for kind in [
        "scrooby_string_text_bible",
        "scrooby_string_hardcoded",
    ] {
        let mut row = serde_json::Map::new();
        add_runtime_field_consumption(kind, &mut row);
        assert!(row.is_empty(), "unexpected ignored-field policy for {kind}");
    }
}

#[test]
fn layout_rows_do_not_copy_authored_names_or_text() -> Result<(), String> {
    for component in [
        Component {
            ordinal: 1,
            parent_ordinal: None,
            kind: "scrooby_page".to_owned(),
            payload: serde_json::json!({
                "name": "Title\\x00",
                "version": 0,
                "resolution": [640, 480],
            }),
        },
        Component {
            ordinal: 2,
            parent_ordinal: Some(1),
            kind: "scrooby_string_hardcoded".to_owned(),
            payload: serde_json::json!({"value": "Apply\\x00"}),
        },
        Component {
            ordinal: 3,
            parent_ordinal: Some(1),
            kind: "scrooby_string_text_bible".to_owned(),
            payload: serde_json::json!({
                "bible_name": "Body\\x00",
                "string_id": "Title\\x00",
            }),
        },
    ] {
        let mut row = serde_json::Map::new();
        add_semantics(&component, [640, 480], &mut row)
            .map_err(|error| error.to_string())?;
        let copied = row.values().filter_map(serde_json::Value::as_str).any(
            |value| matches!(
                value,
                "Title\\x00" | "Apply\\x00" | "Body\\x00"
            ),
        );
        if copied {
            return Err(format!(
                "{} copied authored identity into layout evidence",
                component.kind
            ));
        }
    }
    Ok(())
}

#[test]
fn polygon_screen_coordinates_truncate_toward_zero() -> Result<(), String> {
    assert_eq!(screen_i32(12.9).map_err(|e| e.to_string())?, 12);
    assert_eq!(screen_i32(-12.9).map_err(|e| e.to_string())?, -12);
    Ok(())
}

#[test]
fn polygon_layout_publishes_runtime_triangle_fan() -> Result<(), String> {
    let payload = serde_json::json!({
        "translucency": 0,
        "point_count": 4,
        "points": [[0, 0, 0], [10, 0, 0], [10, 10, 0], [0, 10, 0]],
        "colors": [
            4_294_967_295_u32,
            4_294_967_295_u32,
            4_294_967_295_u32,
            4_294_967_295_u32,
        ],
    });
    let mut row = serde_json::Map::new();
    add_polygon(&payload, &mut row).map_err(|error| error.to_string())?;
    assert_eq!(
        row.get("render_topology"),
        Some(&serde_json::json!("triangle-fan")),
    );
    assert_eq!(
        row.get("triangle_indices"),
        Some(&serde_json::json!([[0, 1, 2], [0, 2, 3]])),
    );
    assert_eq!(row.get("blend_mode"), Some(&serde_json::json!("alpha")));
    assert_eq!(
        row.get("shade_mode"),
        Some(&serde_json::json!("gouraud")),
    );
    assert_eq!(
        row.get("vertex_rgb_modulation"),
        Some(&serde_json::json!(
            "multiply-by-current-drawable-rgb-u8-floor"
        )),
    );
    assert_eq!(
        row.get("stream_alpha_policy"),
        Some(&serde_json::json!(
            "uniform-min-vertex0-alpha-current-drawable-color-alpha"
        )),
    );
    Ok(())
}

#[test]
fn polygon_layout_rejects_declared_point_count_drift() -> Result<(), String> {
    let payload = serde_json::json!({
        "translucency": 0,
        "point_count": 4,
        "points": [[0, 0, 0], [10, 0, 0], [0, 10, 0]],
        "colors": [
            4_294_967_295_u32,
            4_294_967_295_u32,
            4_294_967_295_u32,
        ],
    });
    let mut row = serde_json::Map::new();
    let Err(error) = add_polygon(&payload, &mut row) else {
        return Err("mismatched polygon point count was accepted".to_owned());
    };
    let expected = "Scrooby polygon point count disagrees with points";
    if error.to_string() != expected {
        return Err(format!("unexpected polygon point-count error: {error}"));
    }
    Ok(())
}

#[test]
fn polygon_layout_publishes_runtime_rgba_channels() -> Result<(), String> {
    let payload = serde_json::json!({
        "translucency": 0,
        "point_count": 3,
        "points": [[0, 0, 0], [10, 0, 0], [0, 10, 0]],
        "colors": [0xc011_2233_u32, 0xff44_5566_u32, 0x0077_8899_u32],
    });
    let mut row = serde_json::Map::new();
    add_polygon(&payload, &mut row).map_err(|error| error.to_string())?;
    assert_eq!(
        row.get("colors_rgba_u8"),
        Some(&serde_json::json!([
            [0x11, 0x22, 0x33, 0xc0],
            [0x44, 0x55, 0x66, 0xff],
            [0x77, 0x88, 0x99, 0x00],
        ])),
    );
    Ok(())
}
