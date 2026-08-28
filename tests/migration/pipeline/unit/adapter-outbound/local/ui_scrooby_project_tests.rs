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

use super::{preflight_scrooby_package, trim_padding};

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
                r#"{"schema":"scrooby_project","children":["#,
                r#"{"id_hex":"0x00018002"},"#,
                r#"{"id_hex":"0x00018001"}]}"#,
            ),
        )?,
        write_component(
            root, 2, 1, "scrooby_page", "page",
            concat!(
                r#"{"schema":"scrooby_page","name":"Main\\x00","#,
                r#""children":[{"id_hex":"0x00018003"}]}"#,
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
            root, 4, 1, "scrooby_image_resource", "image",
            r#"{"schema":"scrooby_image_resource","name":"Icon\\x00"}"#,
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
            root, 7, 1, "scrooby_text_style_resource", "style",
            r#"{"schema":"scrooby_text_style_resource","name":"Body"}"#,
        )?,
        write_component(
            root, 8, 5, "scrooby_multi_text", "text",
            concat!(
                r#"{"schema":"scrooby_multi_text","text_style":"Body","#,
                r#""children":[{"id_hex":"0x0001800b"}]}"#,
            ),
        )?,
        write_component(
            root, 9, 1, "scrooby_text_bible_resource", "bible",
            r#"{"schema":"scrooby_text_bible_resource","name":"srr2"}"#,
        )?,
        write_component(
            root, 10, 8, "scrooby_string_text_bible", "string",
            r#"{"schema":"scrooby_string_text_bible","bible_name":"srr2"}"#,
        )?,
        write_component(
            root, 11, 1, "scrooby_pure3d_resource", "pure",
            concat!(
                r#"{"schema":"scrooby_pure3d_resource","name":"dummy", "#,
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
fn accepts_complete_package_local_bindings() -> TestResult {
    let root = case_dir("complete")?;
    write_valid_package(&root)?;
    let result = preflight_scrooby_package(&root)
        .map_err(|error| error.to_string());
    fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    result
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
fn accepts_opaque_page_resource_references() -> TestResult {
    let root = case_dir("opaque-page-resources")?;
    write_valid_package(&root)?;
    fs::write(
        root.join("components/scrooby_page/page.json"),
        concat!(
            r#"{"schema":"scrooby_page","name":"Main\\x00","#,
            r#""children":[{"id_hex":"0x00018003"},"#,
            r#"{"id_hex":"0x00018100"},{"id_hex":"0x00018101"},"#,
            r#"{"id_hex":"0x00018104"},{"id_hex":"0x00018105"}]}"#,
        ),
    )
    .map_err(|error| error.to_string())?;
    let result = preflight_scrooby_package(&root)
        .map_err(|error| error.to_string());
    fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    result
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
        1,
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
    if !error.to_string().contains("resource name is ambiguous") {
        return Err(format!("unexpected Pure3D ambiguity error: {error}"));
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
        1,
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
        .map_err(|error| error.to_string());
    fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    result
}

#[test]
fn visible_nul_padding_is_removed_only_from_the_end() {
    assert_eq!(trim_padding("name\\x00\\x00"), "name");
    assert_eq!(trim_padding("na\\x00me"), "na\\x00me");
}
