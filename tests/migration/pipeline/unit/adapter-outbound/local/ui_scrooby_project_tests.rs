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
//   - Scrooby semantic-preflight unit regressions.
// - Must-Not:
//   - Depend on the lawful source installation or Unreal editor state.
// - Allows:
//   - Synthetic normalized component ledgers and package-local references.
// - Split-When:
//   - One reference family gains independent fixture ownership.
// - Merge-When:
//   - Another test module owns the identical preflight behavior.
// - Summary:
//   - Pins exact Scrooby package-local reference resolution.
// - Description:
//   - Proves valid bindings pass while missing or ambiguous identities fail
//     closed.
// - Usage:
//   - Included only by the Scrooby semantic-preflight adapter under cfg(test).
// - Defaults:
//   - Fixtures are deleted after every assertion.
//

//! Scrooby semantic-preflight unit regressions.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

use super::{
    preflight_scrooby_package, publish_scrooby_binding_catalog,
    PhaseThreePackageIndex, bind_scrooby_joined_image_entities,
    resolve_exact_image_source, resolve_exact_joined_sprite_ordinal,
    resolve_owner_text_style_inventory, scrooby_resource_package_root,
    trim_padding, validate_scrooby_resource_inventory_kinds,
};

static CASE_ID: AtomicUsize = AtomicUsize::new(0);

type TestResult = Result<(), String>;

fn case_dir(label: &str) -> Result<PathBuf, String> {
    let path = std::env::temp_dir().join(format!(
        "shar-scrooby-preflight-{label}-{}-{}",
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
    json: &str,
) -> Result<String, String> {
    let relative = format!("{kind}/{name}.json");
    let path = root.join("components").join(&relative);
    let parent = path.parent().ok_or("component path has no parent")?;
    fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    fs::write(&path, json).map_err(|error| error.to_string())?;
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

fn write_valid_package(root: &Path) -> TestResult {
    let rows = vec![
        write_component(
            root, 1, 0, "scrooby_project", "project",
            concat!(
                r#"{"schema":"scrooby_project","resource_path":"resource\\","#,
                r#""children":[{"id_hex":"0x00018002"},"#,
                r#"{"id_hex":"0x00018001"}]}"#,
            ),
        )?,
        write_component(
            root, 2, 1, "scrooby_page", "page",
            concat!(
                r#"{"schema":"scrooby_page","name":"Main\\x00","#,
                r#""children":[{"id_hex":"0x00018003"},"#,
                r#"{"id_hex":"0x00018100","name":"Icon\\x00"},"#,
                r#"{"id_hex":"0x00018101","name":"dummy"},"#,
                r#"{"id_hex":"0x00018104","name":"Body"},"#,
                r#"{"id_hex":"0x00018105","name":"srr2"}]}"#,
            ),
        )?,
        write_component(
            root, 3, 1, "scrooby_screen", "screen",
            concat!(
                r#"{"schema":"scrooby_screen","#,
                r#""page_names":["Main"]}"#,
            ),
        )?,
        write_component(
            root, 4, 2, "scrooby_image_resource", "image",
            concat!(
                r#"{"schema":"scrooby_image_resource","#,
                r#""name":"Icon\\x00","filename":"Icon.png"}"#,
            ),
        )?,
        write_component(
            root, 5, 2, "scrooby_layer", "layer",
            concat!(
                r#"{"schema":"scrooby_layer","children":["#,
                r#"{"id_hex":"0x00018006"},{"id_hex":"0x00018007"},"#,
                r#"{"id_hex":"0x00018008"}]}"#,
            ),
        )?,
        write_component(
            root, 6, 5, "scrooby_multi_sprite", "sprite",
            concat!(
                r#"{"schema":"scrooby_multi_sprite","#,
                r#""image_names":["Icon"]}"#,
            ),
        )?,
        write_component(
            root, 7, 2, "scrooby_text_style_resource", "style",
            concat!(
                r#"{"schema":"scrooby_text_style_resource","#,
                r#""name":"Body","filename":"fonts\\body.p3d","#,
                r#""inventory_name":"BodyFont"}"#,
            ),
        )?,
        write_component(
            root, 8, 5, "scrooby_multi_text", "text",
            concat!(
                r#"{"schema":"scrooby_multi_text","text_style":"Body","#,
                r#""children":[{"id_hex":"0x0001800b"}]}"#,
            ),
        )?,
        write_component(
            root, 9, 2, "scrooby_text_bible_resource", "bible",
            concat!(
                r#"{"schema":"scrooby_text_bible_resource","#,
                r#""name":"srr2","filename":"text\\srr2.p3d","#,
                r#""inventory_name":"srr2"}"#,
            ),
        )?,
        write_component(
            root, 10, 8, "scrooby_string_text_bible", "string",
            r#"{"schema":"scrooby_string_text_bible","bible_name":"srr2"}"#,
        )?,
        write_component(
            root, 11, 2, "scrooby_pure3d_resource", "pure",
            concat!(
                r#"{"schema":"scrooby_pure3d_resource","name":"dummy", "#,
                r#""filename":"pure3d\\dummy.p3d","#,
                r#""inventory_name":"DummyDrawable"}"#,
            ),
        )?,
        write_component(
            root, 12, 5, "scrooby_pure3d_object", "object",
            r#"{"schema":"scrooby_pure3d_object","filename":"dummy"}"#,
        )?,
    ];
    let mut ledger = format!(
        "{}\n",
        r#"{"schema":"p3d.package.v1","component_count":12}"#,
    );
    for row in rows {
        ledger.push_str(&row);
        ledger.push('\n');
    }
    fs::write(root.join("components.jsonl"), ledger)
        .map_err(|error| error.to_string())
}

#[test]
fn validates_resource_target_inventory_cardinality() -> TestResult {
    for (kind, matches) in [
        ("scrooby_pure3d_resource", vec!["scenegraph"]),
        (
            "scrooby_pure3d_resource",
            vec!["mesh", "composite_drawable"],
        ),
        ("scrooby_text_style_resource", vec!["texture_font"]),
        ("scrooby_text_bible_resource", vec!["text_bible"]),
    ] {
        validate_scrooby_resource_inventory_kinds(kind, &matches)
            .map_err(|error| error.to_string())?;
    }
    Ok(())
}

#[test]
fn rejects_invalid_resource_target_inventory_cardinality() -> TestResult {
    for (kind, matches) in [
        ("scrooby_pure3d_resource", Vec::new()),
        ("scrooby_text_style_resource", Vec::new()),
        ("scrooby_text_style_resource", vec!["mesh"]),
        (
            "scrooby_text_style_resource",
            vec!["texture_font", "texture_font"],
        ),
        ("scrooby_text_bible_resource", Vec::new()),
        ("scrooby_text_bible_resource", vec!["language"]),
        ("scrooby_image_resource", vec!["sprite"]),
        (
            "scrooby_text_bible_resource",
            vec!["text_bible", "text_bible"],
        ),
    ] {
        if validate_scrooby_resource_inventory_kinds(kind, &matches).is_ok() {
            return Err(format!(
                "invalid inventory cardinality passed: {kind} {matches:?}"
            ));
        }
    }
    Ok(())
}

fn owner_inventory_index(
    root: &Path,
    font_names: &[&str],
) -> Result<PhaseThreePackageIndex, String> {
    let root_name = root
        .file_name()
        .and_then(|value| value.to_str())
        .ok_or_else(|| "fixture root has no portable name".to_owned())?;
    let package_root = format!("{root_name}/ui/project");
    let package_id = format!("{root_name}-ui-project");
    let physical_root = root.join("ui/project/components/texture_font");
    fs::create_dir_all(&physical_root).map_err(|error| error.to_string())?;
    let mut ids = Vec::new();
    let mut members = Vec::new();
    for (index, name) in font_names.iter().enumerate() {
        let id = format!("font-{index}");
        let relative = format!("font-{index}.json");
        fs::write(
            physical_root.join(&relative),
            serde_json::to_vec(&serde_json::json!({
                "schema": "texture_font",
                "name": name,
            }))
            .map_err(|error| error.to_string())?,
        )
        .map_err(|error| error.to_string())?;
        let member_path =
            format!("{package_root}/components/texture_font/{relative}");
        let id_json =
            serde_json::to_string(&id).map_err(|error| error.to_string())?;
        let path_json = serde_json::to_string(&member_path)
            .map_err(|error| error.to_string())?;
        members.push(format!(
            concat!(
                r#"{{"id":{},"role":"ui","path":{},"#,
                r#""type":"ui","kind":"p3d-texture-font","#,
                r#""source_chunk_kind":"texture_font"}}"#,
            ),
            id_json,
            path_json,
        ));
        ids.push(id);
    }
    let ids_json =
        serde_json::to_string(&ids).map_err(|error| error.to_string())?;
    let members_json = format!("[{}]", members.join(","));
    let row = format!(
        concat!(
            r#"{{"package_id":"{}","package_root":"{}","#,
            r#""package_category":"ui-screens","#,
            r#""package_subcategory":"ui-screens/test","unit_count":{},"#,
            r#""text_key_count":0,"unit_ids":{},"world_ids":[],"#,
            r#""texture_ids":[],"material_ids":[],"model_ids":[],"#,
            r#""physics_ids":[],"animation_ids":[],"scene_ids":[],"#,
            r#""locator_ids":[],"camera_ids":[],"light_ids":[],"#,
            r#""particle_ids":[],"controller_ids":[],"audio_ids":[],"#,
            r#""movie_ids":[],"script_ids":[],"text_ids":[],"#,
            r#""ui_ids":{},"metadata_ids":[],"error_ids":[],"#,
            r#""source_unit_ids":[],"text_key_ids":[],"members":{},"#,
            r#""text_keys":[]}}"#,
        ),
        package_id,
        package_root,
        ids.len(),
        ids_json,
        ids_json,
        members_json,
    );
    PhaseThreePackageIndex::from_jsonl(&format!("{row}\n"))
        .map_err(|error| error.to_string())
}

fn text_style_resource(inventory_name: &str) -> super::Component {
    super::Component {
        row: super::LedgerRow {
            ordinal: 1,
            parent_ordinal: Some(0),
            kind: "scrooby_text_style_resource".to_owned(),
            path: "style.json".to_owned(),
        },
        payload: serde_json::json!({
            "schema": "scrooby_text_style_resource",
            "inventory_name": inventory_name,
        }),
    }
}

#[test]
fn owner_text_style_inventory_requires_one_texture_font() -> TestResult {
    for (label, names, expected) in [
        ("unique", vec!["Body"], true),
        ("missing", vec!["Other"], false),
        ("duplicate", vec!["Body", "Body"], false),
    ] {
        let root = case_dir(&format!("owner-inventory-{label}"))?;
        let index = owner_inventory_index(&root, &names)?;
        let package = index
            .packages()
            .first()
            .ok_or_else(|| "owner package fixture is empty".to_owned())?;
        let resource = text_style_resource("Body");
        let resolved = resolve_owner_text_style_inventory(
            &root,
            package,
            &resource,
        )
        .map_err(|error| error.to_string())?;
        fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
        if resolved.is_some() != expected {
            return Err(format!(
                "unexpected owner inventory result for {label}: {resolved:?}"
            ));
        }
    }
    Ok(())
}

#[test]
fn resource_package_root_uses_owner_path_and_strips_p3d() -> TestResult {
    let observed = scrooby_resource_package_root(
        "extracted/art/frontend/scrooby/project",
        "resource\\",
        "pure3d\\model.P3D\\x00",
    )
    .map_err(|error| error.to_string())?;
    if observed.as_deref()
        != Some("extracted/art/frontend/scrooby/resource/pure3d/model")
    {
        return Err(format!("unexpected resource package root: {observed:?}"));
    }
    Ok(())
}

#[test]
fn resource_package_root_ignores_non_p3d_files() -> TestResult {
    let observed = scrooby_resource_package_root(
        "extracted/art/frontend/scrooby/project",
        "resource\\",
        "images\\icon.png",
    )
    .map_err(|error| error.to_string())?;
    if observed.is_some() {
        return Err(format!(
            "non-P3D resource produced package root: {observed:?}"
        ));
    }
    Ok(())
}

#[test]
fn resource_package_root_rejects_unsafe_relative_segments() -> TestResult {
    for (resource_path, filename) in [
        ("../resource", "pure3d/model.p3d"),
        ("/resource", "pure3d/model.p3d"),
        ("C:\\resource", "pure3d/model.p3d"),
        ("resource", "../model.p3d"),
        ("resource", "pure3d//model.p3d"),
    ] {
        if scrooby_resource_package_root(
            "extracted/art/frontend/scrooby/project",
            resource_path,
            filename,
        )
        .is_ok()
        {
            return Err(format!(
                "unsafe path accepted: {resource_path:?} {filename:?}"
            ));
        }
    }
    Ok(())
}

#[test]
fn publishes_deterministic_package_scoped_binding_catalog() -> TestResult {
    let root = case_dir("binding-catalog-input")?;
    write_valid_package(&root)?;
    let bindings = preflight_scrooby_package(&root)
        .map_err(|error| error.to_string())?;
    let preflight = super::ScroobyUiPreflight {
        packages: vec![super::ScroobyPackageBindings {
            package_id: "fixture-package".to_owned(),
            bindings,
            image_resources: Vec::new(),
        }],
    };
    let output = case_dir("binding-catalog-output")?;
    fs::remove_dir_all(&output).map_err(|error| error.to_string())?;
    let count = publish_scrooby_binding_catalog(&preflight, &output)
        .map_err(|error| error.to_string())?;
    if count != 9 {
        return Err(format!("unexpected binding catalog count: {count}"));
    }
    let catalog = output.join("catalog.jsonl");
    let first = fs::read_to_string(&catalog)
        .map_err(|error| error.to_string())?;
    let mut lines = first.lines();
    let header = lines
        .next()
        .ok_or_else(|| "binding catalog header is missing".to_owned())?;
    let header = serde_json::from_str::<serde_json::Value>(header)
        .map_err(|error| error.to_string())?;
    if header
        .get("package_count")
        .and_then(serde_json::Value::as_u64)
        != Some(1)
        || header.get("binding_count").and_then(serde_json::Value::as_u64)
            != Some(9)
    {
        return Err(format!("unexpected binding catalog header: {header}"));
    }
    let rows = lines
        .map(|line| {
            serde_json::from_str::<serde_json::Value>(line)
                .map_err(|error| error.to_string())
        })
        .collect::<Result<Vec<_>, _>>()?;
    let first_binding = rows
        .first()
        .ok_or_else(|| "binding catalog rows are missing".to_owned())?;
    if first_binding.get("package_id").and_then(serde_json::Value::as_str)
        != Some("fixture-package")
        || first_binding
            .get("source_ordinal")
            .and_then(serde_json::Value::as_u64)
            != Some(2)
        || first_binding
            .get("source_index")
            .and_then(serde_json::Value::as_u64)
            != Some(1)
        || first_binding
            .get("target_ordinal")
            .and_then(serde_json::Value::as_u64)
            != Some(4)
    {
        return Err(format!("unexpected first binding row: {first_binding}"));
    }
    let second_count = publish_scrooby_binding_catalog(&preflight, &output)
        .map_err(|error| error.to_string())?;
    let second = fs::read_to_string(&catalog)
        .map_err(|error| error.to_string())?;
    fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    fs::remove_dir_all(&output).map_err(|error| error.to_string())?;
    if second_count != count || second != first {
        return Err(
            "binding catalog reuse changed deterministic output".to_owned(),
        );
    }
    Ok(())
}

#[test]
fn binding_catalog_publishes_public_source_identity_only() -> TestResult {
    let root = case_dir("binding-source-identity")?;
    write_valid_package(&root)?;
    let mut bindings = preflight_scrooby_package(&root)
        .map_err(|error| error.to_string())?;
    for binding in &mut bindings {
        if matches!(
            binding.relation,
            "page-image-resource" | "sprite-image-resource"
        ) {
            binding.target_source_unit_id = Some("texture-source".to_owned());
            binding.target_source_match_basis =
                Some("filename-basename-exact");
        }
    }
    let package_binding = bindings
        .iter_mut()
        .find(|binding| binding.relation == "page-pure3d-resource")
        .ok_or_else(|| "page Pure3D binding is missing".to_owned())?;
    package_binding.target_package_id = Some("resource-package".to_owned());
    package_binding.target_package_match_basis =
        Some("project-resource-path-exact");
    let preflight = super::ScroobyUiPreflight {
        packages: vec![super::ScroobyPackageBindings {
            package_id: "fixture-package".to_owned(),
            bindings,
            image_resources: vec![super::ScroobyImageResourceSource {
                ordinal: 4,
                filename: "private/authored/Icon.png".to_owned(),
                joined_sprite_ordinal: None,
            }],
        }],
    };
    let rendered = preflight
        .to_catalog_jsonl()
        .map_err(|error| error.to_string())?;
    fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    if rendered.contains("private/authored") || rendered.contains("Icon.png") {
        return Err("binding catalog leaked private image filename".to_owned());
    }
    let rows = rendered
        .lines()
        .map(serde_json::from_str::<serde_json::Value>)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| error.to_string())?;
    if rows
        .first()
        .and_then(|row| row.get("direct_import_binding_count"))
        .and_then(serde_json::Value::as_u64)
        != Some(2)
        || rows
            .first()
            .and_then(|row| row.get("normalized_package_binding_count"))
            .and_then(serde_json::Value::as_u64)
            != Some(1)
    {
        return Err(format!(
            "unexpected source binding header: {:?}",
            rows.first(),
        ));
    }
    let resolved = rows
        .iter()
        .skip(1)
        .filter(|row| row.get("target_source_unit_id").is_some())
        .collect::<Vec<_>>();
    if resolved.len() != 2
        || resolved.iter().any(|row| {
            row.get("target_source_unit_id")
                .and_then(serde_json::Value::as_str)
                != Some("texture-source")
                || row
                    .get("target_source_match_basis")
                    .and_then(serde_json::Value::as_str)
                    != Some("filename-basename-exact")
        })
    {
        return Err(format!("unexpected public source bindings: {resolved:?}"));
    }
    let package_resolved = rows
        .iter()
        .skip(1)
        .filter(|row| row.get("target_package_id").is_some())
        .collect::<Vec<_>>();
    if package_resolved.len() != 1
        || package_resolved.first().and_then(|row| {
            row.get("target_package_id")
                .and_then(serde_json::Value::as_str)
        }) != Some("resource-package")
    {
        return Err(format!(
            "unexpected public package binding: {package_resolved:?}"
        ));
    }
    Ok(())
}

#[test]
fn page_resource_lifecycle_preserves_authored_occurrences() -> TestResult {
    let root = case_dir("page-resource-lifecycle")?;
    write_valid_package(&root)?;
    let mut bindings = preflight_scrooby_package(&root)
        .map_err(|error| error.to_string())?;
    let image = bindings
        .iter_mut()
        .find(|binding| binding.relation == "page-image-resource")
        .ok_or_else(|| "page image binding is missing".to_owned())?;
    image.target_source_unit_id = Some("texture-source".to_owned());
    image.target_source_match_basis = Some("filename-basename-exact");
    let preflight = super::ScroobyUiPreflight {
        packages: vec![super::ScroobyPackageBindings {
            package_id: "fixture-package".to_owned(),
            bindings,
            image_resources: Vec::new(),
        }],
    };
    let rows = preflight.page_resource_lifecycle();
    fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    let observed = rows
        .iter()
        .map(|row| {
            (
                row.page_ordinal,
                row.source_index,
                row.resource_kind,
                row.target_ordinal,
                row.target_source_unit_id.as_deref(),
            )
        })
        .collect::<Vec<_>>();
    let expected = vec![
        (2, 1, "image", 4, Some("texture-source")),
        (2, 2, "pure3d", 11, None),
        (2, 3, "text-style", 7, None),
        (2, 4, "text-bible", 9, None),
    ];
    if observed != expected {
        return Err(format!("unexpected page resource lifecycle: {observed:?}"));
    }
    Ok(())
}

#[test]
fn binding_catalog_rejects_incomplete_source_identity() -> TestResult {
    let root = case_dir("binding-source-incomplete")?;
    write_valid_package(&root)?;
    let mut bindings = preflight_scrooby_package(&root)
        .map_err(|error| error.to_string())?;
    let binding = bindings
        .iter_mut()
        .find(|binding| binding.relation == "page-image-resource")
        .ok_or_else(|| "page image binding is missing".to_owned())?;
    binding.target_source_unit_id = Some("texture-source".to_owned());
    let preflight = super::ScroobyUiPreflight {
        packages: vec![super::ScroobyPackageBindings {
            package_id: "fixture-package".to_owned(),
            bindings,
            image_resources: Vec::new(),
        }],
    };
    let result = preflight.to_catalog_jsonl();
    fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    let Err(error) = result else {
        return Err("incomplete source identity was published".to_owned());
    };
    if !error.to_string().contains("direct-import binding is incomplete") {
        return Err(format!("unexpected incomplete binding error: {error}"));
    }
    Ok(())
}

#[test]
fn binding_catalog_rejects_incomplete_entity_identity() -> TestResult {
    let root = case_dir("binding-entity-incomplete")?;
    write_valid_package(&root)?;
    let mut bindings = preflight_scrooby_package(&root)
        .map_err(|error| error.to_string())?;
    let binding = bindings
        .iter_mut()
        .find(|binding| binding.relation == "page-image-resource")
        .ok_or_else(|| "page image binding is missing".to_owned())?;
    binding.target_package_id = Some("fixture-package".to_owned());
    binding.target_package_match_basis = Some("owner-joined-sprite-exact");
    binding.target_entity_ordinal = Some(29);
    let preflight = super::ScroobyUiPreflight {
        packages: vec![super::ScroobyPackageBindings {
            package_id: "fixture-package".to_owned(),
            bindings,
            image_resources: Vec::new(),
        }],
    };
    let result = preflight.to_catalog_jsonl();
    fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    let Err(error) = result else {
        return Err("incomplete entity binding was accepted".to_owned());
    };
    if !error.to_string().contains("normalized-entity binding is incomplete") {
        return Err(format!("unexpected entity binding error: {error}"));
    }
    Ok(())
}

#[test]
fn binding_catalog_rejects_incomplete_package_identity() -> TestResult {
    let root = case_dir("binding-package-incomplete")?;
    write_valid_package(&root)?;
    let mut bindings = preflight_scrooby_package(&root)
        .map_err(|error| error.to_string())?;
    let binding = bindings
        .iter_mut()
        .find(|binding| binding.relation == "page-pure3d-resource")
        .ok_or_else(|| "page Pure3D binding is missing".to_owned())?;
    binding.target_package_id = Some("resource-package".to_owned());
    let preflight = super::ScroobyUiPreflight {
        packages: vec![super::ScroobyPackageBindings {
            package_id: "fixture-package".to_owned(),
            bindings,
            image_resources: Vec::new(),
        }],
    };
    let result = preflight.to_catalog_jsonl();
    fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    let Err(error) = result else {
        return Err("incomplete package identity was published".to_owned());
    };
    if !error
        .to_string()
        .contains("normalized-package binding is incomplete")
    {
        return Err(format!("unexpected package binding error: {error}"));
    }
    Ok(())
}

#[test]
fn binding_catalog_reuse_rejects_transaction_debris() -> TestResult {
    let root = case_dir("binding-debris-input")?;
    write_valid_package(&root)?;
    let bindings = preflight_scrooby_package(&root)
        .map_err(|error| error.to_string())?;
    let preflight = super::ScroobyUiPreflight {
        packages: vec![super::ScroobyPackageBindings {
            package_id: "fixture-package".to_owned(),
            bindings,
            image_resources: Vec::new(),
        }],
    };
    let output = case_dir("binding-debris-output")?;
    fs::remove_dir_all(&output).map_err(|error| error.to_string())?;
    let _count = publish_scrooby_binding_catalog(&preflight, &output)
        .map_err(|error| error.to_string())?;
    let catalog = output.join("catalog.jsonl");
    let accepted = fs::read_to_string(&catalog)
        .map_err(|error| error.to_string())?;
    let name = output
        .file_name()
        .and_then(|value| value.to_str())
        .ok_or_else(|| "binding debris output has no file name".to_owned())?;
    let parent = output
        .parent()
        .ok_or_else(|| "binding debris output has no parent".to_owned())?;
    for (suffix, expected) in [
        ("complete-staging", "Scrooby binding staging already exists"),
        ("complete-backup", "Scrooby binding backup already exists"),
    ] {
        let debris = parent.join(format!(".{name}.{suffix}"));
        fs::create_dir_all(&debris).map_err(|error| error.to_string())?;
        let result = publish_scrooby_binding_catalog(&preflight, &output);
        fs::remove_dir_all(&debris).map_err(|error| error.to_string())?;
        let Err(error) = result else {
            return Err(format!("binding reuse accepted {suffix} debris"));
        };
        if !error.to_string().contains(expected) {
            return Err(format!("unexpected {suffix} debris error: {error}"));
        }
        let unchanged = fs::read_to_string(&catalog)
            .map_err(|error| error.to_string())?;
        if unchanged != accepted {
            return Err(format!("{suffix} debris changed accepted catalog"));
        }
    }
    fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    fs::remove_dir_all(&output).map_err(|error| error.to_string())?;
    Ok(())
}

#[test]
fn binding_catalog_preserves_authored_source_order() -> TestResult {
    let root = case_dir("binding-order")?;
    write_valid_package(&root)?;
    let sprite = root.join("components/scrooby_multi_sprite/sprite.json");
    fs::write(
        &sprite,
        concat!(
            r#"{"schema":"scrooby_multi_sprite","name":"sprite","#,
            r#""position":[0,0],"dimensions":[10,10],"#,
            r#""justification":[0,2],"color":4294967295,"#,
            r#""translucency":0,"rotation":0,"image_count":2,"#,
            r#""image_names":["Icon","Icon"]}"#,
        ),
    )
    .map_err(|error| error.to_string())?;
    let bindings = preflight_scrooby_package(&root)
        .map_err(|error| error.to_string())?;
    let indices = bindings
        .iter()
        .filter(|binding| binding.relation == "sprite-image-resource")
        .map(|binding| binding.source_index)
        .collect::<Vec<_>>();
    fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    if indices != [0, 1] {
        return Err(format!("unexpected sprite binding order: {indices:?}"));
    }
    Ok(())
}

#[test]
fn accepts_complete_package_local_bindings() -> TestResult {
    let root = case_dir("complete")?;
    write_valid_package(&root)?;
    let result = preflight_scrooby_package(&root)
        .map_err(|error| error.to_string())?;
    let observed = result
        .iter()
        .map(|binding| {
            (
                binding.source_ordinal,
                binding.source_index,
                binding.target_ordinal,
                binding.relation,
                binding.match_basis,
            )
        })
        .collect::<Vec<_>>();
    let expected = vec![
        (2, 1, 4, "page-image-resource", "name"),
        (2, 2, 11, "page-pure3d-resource", "name"),
        (2, 3, 7, "page-text-style", "name"),
        (2, 4, 9, "page-text-bible", "name"),
        (3, 0, 2, "screen-page", "name"),
        (6, 0, 4, "sprite-image-resource", "name"),
        (8, 0, 7, "text-style-resource", "name"),
        (10, 0, 9, "string-text-bible", "name"),
        (12, 0, 11, "pure3d-object-resource", "name"),
    ];
    fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    if observed != expected {
        return Err(format!("unexpected Scrooby bindings: {observed:?}"));
    }
    Ok(())
}

#[test]
fn rejects_missing_declared_project_child() -> TestResult {
    let root = case_dir("missing-project-child")?;
    write_valid_package(&root)?;
    fs::write(
        root.join("components/scrooby_project/project.json"),
        concat!(
            r#"{"schema":"scrooby_project","children":["#,
            r#"{"id_hex":"0x00018002"}]}"#,
        ),
    )
    .map_err(|error| error.to_string())?;
    let result = preflight_scrooby_package(&root);
    fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    let Err(error) = result else {
        return Err(
            "missing declared screen passed Scrooby preflight".to_owned(),
        );
    };
    if !error.to_string().contains("project child inventory disagrees") {
        return Err(format!("unexpected project-child error: {error}"));
    }
    Ok(())
}

#[test]
fn accepts_named_page_resource_references() -> TestResult {
    let root = case_dir("named-page-resources")?;
    write_valid_package(&root)?;
    let result = preflight_scrooby_package(&root)
        .map(|_bindings| ())
        .map_err(|error| error.to_string());
    fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    result
}

#[test]
fn rejects_unnamed_page_resource_reference() -> TestResult {
    let root = case_dir("unnamed-page-resource")?;
    write_valid_package(&root)?;
    fs::write(
        root.join("components/scrooby_page/page.json"),
        concat!(
            r#"{"schema":"scrooby_page","name":"Main\\x00","#,
            r#""children":[{"id_hex":"0x00018003"},"#,
            r#"{"id_hex":"0x00018100"}]}"#,
        ),
    )
    .map_err(|error| error.to_string())?;
    let result = preflight_scrooby_package(&root);
    fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    let Err(error) = result else {
        return Err("unnamed page resource passed Scrooby preflight".to_owned());
    };
    if !error.to_string().contains("name is not a string") {
        return Err(format!("unexpected unnamed-resource error: {error}"));
    }
    Ok(())
}

#[test]
fn rejects_missing_page_resource_declaration() -> TestResult {
    let root = case_dir("missing-page-resource")?;
    write_valid_package(&root)?;
    let page = root.join("components/scrooby_page/page.json");
    let text = fs::read_to_string(&page).map_err(|error| error.to_string())?;
    let changed = text.replace(
        r#"{"id_hex":"0x00018100","name":"Icon\\x00"}"#,
        r#"{"id_hex":"0x00018100","name":"Missing"}"#,
    );
    if changed == text {
        return Err("page image reference fixture was not found".to_owned());
    }
    fs::write(&page, changed).map_err(|error| error.to_string())?;
    let result = preflight_scrooby_package(&root);
    fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    let Err(error) = result else {
        return Err("missing page resource declaration was accepted".to_owned());
    };
    if !error.to_string().contains("page image resource is missing") {
        return Err(format!("unexpected page-resource error: {error}"));
    }
    Ok(())
}

#[test]
fn rejects_resource_outside_page_ancestry() -> TestResult {
    let root = case_dir("resource-parent")?;
    write_valid_package(&root)?;
    let ledger = root.join("components.jsonl");
    let text = fs::read_to_string(&ledger).map_err(|error| error.to_string())?;
    let changed = text.replacen(
        r#"{"ordinal":4,"parent_ordinal":2,"kind":"scrooby_image_resource""#,
        r#"{"ordinal":4,"parent_ordinal":1,"kind":"scrooby_image_resource""#,
        1,
    );
    if changed == text {
        return Err("resource fixture row was not found".to_owned());
    }
    fs::write(&ledger, changed).map_err(|error| error.to_string())?;
    let result = preflight_scrooby_package(&root);
    fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    let Err(error) = result else {
        return Err("resource outside page ancestry was accepted".to_owned());
    };
    if !error.to_string().contains("unsupported owning parent") {
        return Err(format!("unexpected resource-parent error: {error}"));
    }
    Ok(())
}

#[test]
fn rejects_page_pure3d_inventory_alias() -> TestResult {
    let root = case_dir("page-pure-inventory-alias")?;
    write_valid_package(&root)?;
    let page = root.join("components/scrooby_page/page.json");
    let text = fs::read_to_string(&page).map_err(|error| error.to_string())?;
    let changed = text.replace(
        r#"{"id_hex":"0x00018101","name":"dummy"}"#,
        r#"{"id_hex":"0x00018101","name":"DummyDrawable"}"#,
    );
    if changed == text {
        return Err("page Pure3D reference fixture was not found".to_owned());
    }
    fs::write(&page, changed).map_err(|error| error.to_string())?;
    let result = preflight_scrooby_package(&root);
    fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    let Err(error) = result else {
        return Err("page Pure3D inventory alias was accepted".to_owned());
    };
    if !error
        .to_string()
        .contains("page Pure3D resource is missing")
    {
        return Err(format!("unexpected page Pure3D alias error: {error}"));
    }
    Ok(())
}

#[test]
fn rejects_unknown_page_child_kind() -> TestResult {
    let root = case_dir("unknown-page-child")?;
    write_valid_package(&root)?;
    fs::write(
        root.join("components/scrooby_page/page.json"),
        concat!(
            r#"{"schema":"scrooby_page","name":"Main\\x00","#,
            r#""children":[{"id_hex":"0x00018003"},"#,
            r#"{"id_hex":"0x00018102"}]}"#,
        ),
    )
    .map_err(|error| error.to_string())?;
    let result = preflight_scrooby_package(&root);
    fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    let Err(error) = result else {
        return Err(
            "unknown page child kind passed Scrooby preflight".to_owned(),
        );
    };
    if !error.to_string().contains("unsupported child kind") {
        return Err(format!("unexpected page-child error: {error}"));
    }
    Ok(())
}

#[test]
fn rejects_layer_outside_page_ancestry() -> TestResult {
    let root = case_dir("wrong-layer-parent")?;
    write_valid_package(&root)?;
    let ledger = root.join("components.jsonl");
    let text = fs::read_to_string(&ledger).map_err(|error| error.to_string())?;
    let changed = text.replacen(
        r#"{"ordinal":5,"parent_ordinal":2,"kind":"scrooby_layer""#,
        r#"{"ordinal":5,"parent_ordinal":1,"kind":"scrooby_layer""#,
        1,
    );
    if changed == text {
        return Err("layer fixture row was not found".to_owned());
    }
    fs::write(&ledger, changed).map_err(|error| error.to_string())?;
    let result = preflight_scrooby_package(&root);
    fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    let Err(error) = result else {
        return Err("layer outside page ancestry passed preflight".to_owned());
    };
    if !error.to_string().contains("unsupported owning parent") {
        return Err(format!("unexpected layer-parent error: {error}"));
    }
    Ok(())
}

#[test]
fn rejects_declared_widget_without_published_child() -> TestResult {
    let root = case_dir("missing-widget")?;
    write_valid_package(&root)?;
    let layer = root.join("components/scrooby_layer/layer.json");
    fs::write(
        layer,
        concat!(
            r#"{"schema":"scrooby_layer","children":["#,
            r#"{"id_hex":"0x00018006"},{"id_hex":"0x00018007"},"#,
            r#"{"id_hex":"0x00018008"},{"id_hex":"0x00018009"}]}"#,
        ),
    )
    .map_err(|error| error.to_string())?;
    let result = preflight_scrooby_package(&root);
    fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    let Err(error) = result else {
        return Err(
            "missing published widget passed Scrooby preflight".to_owned(),
        );
    };
    if !error.to_string().contains("child inventory disagrees") {
        return Err(format!("unexpected missing-widget error: {error}"));
    }
    Ok(())
}

#[test]
fn rejects_missing_image_resource() -> TestResult {
    let root = case_dir("missing-image")?;
    write_valid_package(&root)?;
    fs::write(
        root.join("components/scrooby_multi_sprite/sprite.json"),
        concat!(
            r#"{"schema":"scrooby_multi_sprite","#,
            r#""image_names":["Missing"]}"#,
        ),
    )
    .map_err(|error| error.to_string())?;
    let result = preflight_scrooby_package(&root);
    fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    let Err(error) = result else {
        return Err(
            "missing image resource passed Scrooby preflight".to_owned(),
        );
    };
    if !error.to_string().contains("image resource is missing") {
        return Err(format!("unexpected missing-image error: {error}"));
    }
    Ok(())
}

#[test]
fn rejects_ambiguous_pure3d_resource_name() -> TestResult {
    let root = case_dir("pure-name-ambiguous")?;
    write_valid_package(&root)?;
    let row = write_component(
        &root,
        13,
        2,
        "scrooby_pure3d_resource",
        "duplicate",
        concat!(
            r#"{"schema":"scrooby_pure3d_resource","name":"dummy", "#,
            r#""inventory_name":"OtherDrawable"}"#,
        ),
    )?;
    let ledger_path = root.join("components.jsonl");
    let mut ledger = fs::read_to_string(&ledger_path)
        .map_err(|error| error.to_string())?;
    ledger = ledger.replace("\"component_count\":12", "\"component_count\":13");
    ledger.push_str(&row);
    ledger.push('\n');
    fs::write(&ledger_path, ledger).map_err(|error| error.to_string())?;
    let result = preflight_scrooby_package(&root);
    fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    let Err(error) = result else {
        return Err("ambiguous Pure3D resource name was accepted".to_owned());
    };
    if !error.to_string().contains("Pure3D resource is ambiguous") {
        return Err(format!("unexpected Pure3D ambiguity error: {error}"));
    }
    Ok(())
}

#[test]
fn pure3d_object_uses_inventory_fallback() -> TestResult {
    let root = case_dir("pure-inventory-fallback")?;
    write_valid_package(&root)?;
    fs::write(
        root.join("components/scrooby_pure3d_object/object.json"),
        concat!(
            r#"{"schema":"scrooby_pure3d_object","#,
            r#""filename":"DummyDrawable"}"#,
        ),
    )
    .map_err(|error| error.to_string())?;
    let bindings = preflight_scrooby_package(&root)
        .map_err(|error| error.to_string())?;
    let observed = bindings
        .iter()
        .find(|binding| binding.relation == "pure3d-object-resource")
        .map(|binding| (binding.target_ordinal, binding.match_basis));
    fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    if observed != Some((11, "inventory_name")) {
        return Err(format!(
            "unexpected Pure3D inventory binding: {observed:?}"
        ));
    }
    Ok(())
}

#[test]
fn pure3d_name_precedes_inventory_aliases() -> TestResult {
    let root = case_dir("pure-name-first")?;
    write_valid_package(&root)?;
    let row = write_component(
        &root,
        13,
        2,
        "scrooby_pure3d_resource",
        "alias",
        concat!(
            r#"{"schema":"scrooby_pure3d_resource","name":"other", "#,
            r#""inventory_name":"dummy"}"#,
        ),
    )?;
    let ledger_path = root.join("components.jsonl");
    let mut ledger = fs::read_to_string(&ledger_path)
        .map_err(|error| error.to_string())?;
    ledger = ledger.replace("\"component_count\":12", "\"component_count\":13");
    ledger.push_str(&row);
    ledger.push('\n');
    fs::write(&ledger_path, ledger).map_err(|error| error.to_string())?;
    let result = preflight_scrooby_package(&root)
        .map(|_bindings| ())
        .map_err(|error| error.to_string());
    fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    result
}

#[test]
fn joined_sprite_identity_is_exact_and_unambiguous() -> TestResult {
    if resolve_exact_joined_sprite_ordinal("Icon.png", Some(&[29]))
        .map_err(|error| error.to_string())?
        != Some(29)
    {
        return Err("exact joined sprite ordinal was not resolved".to_owned());
    }
    if resolve_exact_joined_sprite_ordinal("icon.png", None)
        .map_err(|error| error.to_string())?
        .is_some()
    {
        return Err(
            "case-folded joined sprite identity was accepted".to_owned(),
        );
    }
    if resolve_exact_joined_sprite_ordinal(
        "Icon.png",
        Some(&[29, 30]),
    )
    .is_ok()
    {
        return Err("ambiguous joined sprite identity was accepted".to_owned());
    }
    Ok(())
}

#[test]
fn joined_image_binding_targets_owner_sprite_entity() -> TestResult {
    let root = case_dir("joined-image-entity")?;
    write_valid_package(&root)?;
    let mut bindings = preflight_scrooby_package(&root)
        .map_err(|error| error.to_string())?;
    let resources = [super::ScroobyImageResourceSource {
        ordinal: 4,
        filename: "Icon.png".to_owned(),
        joined_sprite_ordinal: Some(29),
    }];
    bind_scrooby_joined_image_entities(
        "fixture-package",
        &resources,
        &mut bindings,
    )
    .map_err(|error| error.to_string())?;
    let resolved = bindings
        .iter()
        .filter(|binding| {
            matches!(
                binding.relation,
                "page-image-resource" | "sprite-image-resource"
            ) && binding.target_ordinal == 4
        })
        .collect::<Vec<_>>();
    if resolved.is_empty()
        || resolved.iter().any(|binding| {
            binding.target_package_id.as_deref() != Some("fixture-package")
                || binding.target_package_match_basis
                    != Some("owner-joined-sprite-exact")
                || binding.target_entity_ordinal != Some(29)
                || binding.target_entity_match_basis
                    != Some("full-filename-exact")
        })
    {
        return Err(format!("unexpected joined image bindings: {resolved:?}"));
    }
    fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    Ok(())
}

#[test]
fn image_source_binding_requires_exact_case_sensitive_basename() -> TestResult {
    let candidates = [
        ("texture-a", "extracted/ui/Icon.png"),
        ("texture-b", "extracted/ui/Other.png"),
    ];
    let matched = resolve_exact_image_source(
        "folder\\Icon.png",
        candidates.into_iter(),
    )
    .map_err(|error| error.to_string())?;
    if matched != Some("texture-a") {
        return Err(format!("unexpected image source binding: {matched:?}"));
    }
    let folded = resolve_exact_image_source(
        "folder/icon.png",
        candidates.into_iter(),
    )
    .map_err(|error| error.to_string())?;
    if folded.is_some() {
        return Err("case-folded image basename was accepted".to_owned());
    }
    Ok(())
}

#[test]
fn image_source_binding_rejects_ambiguous_direct_imports() -> TestResult {
    let candidates = [
        ("texture-a", "first/Icon.png"),
        ("texture-b", "second/Icon.png"),
    ];
    let result = resolve_exact_image_source("Icon.png", candidates.into_iter());
    let Err(error) = result else {
        return Err("ambiguous image source binding was accepted".to_owned());
    };
    if !error.to_string().contains("direct import is ambiguous") {
        return Err(format!("unexpected image source ambiguity: {error}"));
    }
    Ok(())
}

#[test]
fn visible_nul_padding_is_removed_only_from_the_end() {
    assert_eq!(trim_padding("name\\x00\\x00"), "name");
    assert_eq!(trim_padding("name\0\0"), "name");
    assert_eq!(trim_padding("na\\x00me"), "na\\x00me");
    assert_eq!(trim_padding("na\0me"), "na\0me");
}
