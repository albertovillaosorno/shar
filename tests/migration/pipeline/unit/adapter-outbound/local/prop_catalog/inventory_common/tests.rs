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
//   - Tests unit tests.
// - Must-Not:
//   - Own production behavior or broaden the tested API surface.
// - Allows:
//   - Private test fixtures and assertions for the owning source module.
// - Split-When:
//   - Split when an independent fixture family gains separate ownership.
// - Merge-When:
//   - Merge when another test module owns the identical evidence.
// - Summary:
//   - Tests unit tests.
// - Description:
//   - Preserves unit-test access through a test-only path module.
// - Usage:
//   - Included only by the owning source module under cfg(test).
// - Defaults:
//   - Test setup and assertions fail explicitly.
//

//! Tests unit tests.

use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use super::{clean_identity, read_composite};

static NEXT_FIXTURE: AtomicU64 = AtomicU64::new(0);

fn composite_fixture(
    label: &str,
    props: &str,
    effects: &str,
) -> Result<PathBuf, String> {
    let sequence = NEXT_FIXTURE.fetch_add(1, Ordering::Relaxed);
    let root = std::env::temp_dir().join(format!(
        "shar-prop-composite-{label}-{}-{sequence}",
        std::process::id()
    ));
    fs::create_dir_all(&root).map_err(|error| error.to_string())?;
    let path = root.join("composite.json");
    fs::write(
        &path,
        format!(
            concat!(
                r#"{{"name":"owner","skeleton_name":"rig","#,
                r#""props":{props},"effects":{effects}}}"#
            ),
            props = props,
            effects = effects,
        ),
    )
    .map_err(|error| error.to_string())?;
    Ok(path)
}

#[test]
fn decoded_identity_padding_is_removed() {
    assert_eq!(
        clean_identity("PTRN_flag\x00\x00").ok().as_deref(),
        Some("PTRN_flag")
    );
    assert_eq!(clean_identity("flag\0\0").ok().as_deref(), Some("flag"));
}

#[test]
fn composite_prop_order_is_preserved() -> Result<(), String> {
    let path = composite_fixture(
        "source-order",
        concat!(
            r#"[{"name":"zebra","skeleton_joint_id":3,"is_translucent":0,"#,
            r#""sort_order":0.4},{"name":"alpha","skeleton_joint_id":1,"#,
            r#""is_translucent":1,"sort_order":0.49},{"name":"middle","#,
            r#""skeleton_joint_id":2,"is_translucent":0}]"#,
        ),
        "[]",
    )?;
    let result = read_composite(&path).map_err(|error| error.to_string());
    drop(fs::remove_dir_all(path.parent().ok_or("fixture has no parent")?));
    let composite = result?;
    assert_eq!(composite.prop_names, ["zebra", "alpha", "middle"]);
    let first = composite
        .prop_bindings
        .first()
        .ok_or_else(|| "composite lost first prop binding".to_owned())?;
    let second = composite
        .prop_bindings
        .get(1)
        .ok_or_else(|| "composite lost second prop binding".to_owned())?;
    assert_eq!(first.skeleton_joint_id, 3);
    assert!(!first.is_translucent);
    assert_eq!(first.sort_order_bits, Some(0.4_f32.to_bits()));
    assert_eq!(second.skeleton_joint_id, 1);
    assert!(second.is_translucent);
    assert_eq!(second.sort_order_bits, Some(0.49_f32.to_bits()));
    assert_eq!(
        composite
            .prop_bindings
            .get(2)
            .and_then(|binding| binding.sort_order_bits),
        None
    );
    Ok(())
}

#[test]
fn composite_effect_order_is_preserved() -> Result<(), String> {
    let path = composite_fixture(
        "effect-order",
        "[]",
        concat!(
            r#"[{"kind":"effect","name":"spark","skeleton_joint_id":7,"#,
            r#""is_translucent":1,"sort_order":0.1},{"kind":"effect","#,
            r#""name":"smoke","skeleton_joint_id":2,"is_translucent":0}]"#,
        ),
    )?;
    let result = read_composite(&path).map_err(|error| error.to_string());
    drop(fs::remove_dir_all(path.parent().ok_or("fixture has no parent")?));
    let composite = result?;
    let [first, second] = composite.effect_bindings.as_slice() else {
        return Err("composite effect order was not retained".to_owned());
    };
    assert_eq!(first.name, "spark");
    assert_eq!(first.skeleton_joint_id, 7);
    assert!(first.is_translucent);
    assert_eq!(first.sort_order_bits, Some(0.1_f32.to_bits()));
    assert_eq!(second.name, "smoke");
    assert_eq!(second.skeleton_joint_id, 2);
    assert!(!second.is_translucent);
    assert_eq!(second.sort_order_bits, None);
    Ok(())
}

#[test]
fn composite_effect_kind_fails_closed() -> Result<(), String> {
    let path = composite_fixture(
        "effect-kind",
        "[]",
        concat!(
            r#"[{"kind":"particle","name":"spark","skeleton_joint_id":7,"#,
            r#""is_translucent":1,"sort_order":0.1}]"#,
        ),
    )?;
    let result = read_composite(&path);
    drop(fs::remove_dir_all(path.parent().ok_or("fixture has no parent")?));
    let Err(error) = result else {
        return Err("non-effect composite row was accepted".to_owned());
    };
    if !error
        .to_string()
        .contains("prop composite has unsupported effect kind")
    {
        return Err(format!("unexpected effect-kind error: {error}"));
    }
    Ok(())
}

#[test]
fn composite_space_padded_prop_identity_fails_closed() -> Result<(), String> {
    let path = composite_fixture(
        "space-padded-prop",
        r#"[{"name":" shared","skeleton_joint_id":0,"is_translucent":0}]"#,
        "[]",
    )?;
    let result = read_composite(&path);
    let parent = path.parent().ok_or("fixture has no parent")?;
    drop(fs::remove_dir_all(parent));
    if result.is_ok() {
        return Err(
            "space-padded composite prop identity was repaired".to_owned(),
        );
    }
    Ok(())
}

#[test]
fn composite_duplicate_prop_identity_fails_closed() -> Result<(), String> {
    let path = composite_fixture(
        "duplicate-prop",
        concat!(
            r#"[{"name":"shared","skeleton_joint_id":0,"is_translucent":0},"#,
            r#"{"name":"shared\u0000","skeleton_joint_id":1,"#,
            r#""is_translucent":1}]"#,
        ),
        "[]",
    )?;
    let result = read_composite(&path);
    drop(fs::remove_dir_all(path.parent().ok_or("fixture has no parent")?));
    let Err(error) = result else {
        return Err("duplicate composite prop identity was accepted".to_owned());
    };
    if !error
        .to_string()
        .contains("prop composite repeats prop identity shared")
    {
        return Err(format!("unexpected duplicate prop error: {error}"));
    }
    Ok(())
}
