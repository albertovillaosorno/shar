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

fn composite_fixture(label: &str, props: &str) -> Result<PathBuf, String> {
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
            r#"{{"name":"owner","skeleton_name":"rig","props":{props}}}"#
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
        r#"[{"name":"zebra"},{"name":"alpha"},{"name":"middle"}]"#,
    )?;
    let result = read_composite(&path).map_err(|error| error.to_string());
    drop(fs::remove_dir_all(path.parent().ok_or("fixture has no parent")?));
    let composite = result?;
    assert_eq!(composite.prop_names, ["zebra", "alpha", "middle"]);
    Ok(())
}

#[test]
fn composite_space_padded_prop_identity_fails_closed() -> Result<(), String> {
    let path = composite_fixture(
        "space-padded-prop",
        r#"[{"name":" shared"}]"#,
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
        r#"[{"name":"shared"},{"name":"shared\u0000"}]"#,
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
