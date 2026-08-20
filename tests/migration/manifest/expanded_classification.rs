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
//   - Expanded classification test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Expanded classification test module.
// - Description:
//   - Implements the declared test module responsibility for manifest.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Expanded classification test module.

use std::fs;
use std::io::{self, ErrorKind};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

use game_manifest as _;
use schoenwald_cli as _;
use schoenwald_filesystem as _;

static NEXT_FIXTURE: AtomicU64 = AtomicU64::new(0);

fn generate_for(extension: &str) -> io::Result<String> {
    generate_named(&format!("asset.{extension}"))
}

fn generate_named(file_name: &str) -> io::Result<String> {
    let sequence = NEXT_FIXTURE.fetch_add(1, Ordering::Relaxed);
    let root = std::env::temp_dir().join(format!(
        "game-manifest-expanded-kind-{}-{sequence}",
        std::process::id()
    ));
    match fs::remove_dir_all(&root) {
        Ok(()) => {},
        Err(error) if error.kind() == ErrorKind::NotFound => {},
        Err(error) => return Err(error),
    }
    let game = root.join("input");
    let extracted = root.join("decoded");
    let output_path = root.join("result.jsonl");
    fs::create_dir_all(&game)?;
    fs::create_dir_all(&extracted)?;
    fs::write(game.join(file_name), b"fixture")?;
    let result = (|| {
        let output =
            Command::new(env!("CARGO_BIN_EXE_generate-expanded-manifest"))
                .arg(&game)
                .arg(&extracted)
                .arg(&output_path)
                .output()?;
        if !output.status.success() {
            return Err(io::Error::other(String::from_utf8_lossy(
                &output.stderr,
            )));
        }
        fs::read_to_string(output_path)
    })();
    drop(fs::remove_dir_all(&root));
    result
}

#[test]
fn expanded_classifier_matches_controlled_taxonomy() {
    for (extension, kind) in [
        ("typ", "sound-type"),
        ("json", "metadata"),
        ("bik", "movie"),
    ] {
        let result = generate_for(extension);
        assert!(result.is_ok());
        let Some(manifest) = result.ok() else {
            continue;
        };
        assert!(manifest.contains(&format!("\"kind\":\"{kind}\"")));
    }
}

#[test]
fn expanded_generator_rejects_unclassified_files() {
    let result = generate_for("mystery");
    assert!(result.is_err());
}

#[test]
fn expanded_p3d_names_do_not_claim_chunk_inspection() {
    let result = generate_named("txtbible-lookalike.p3d");
    assert!(result.is_ok());
    let Some(manifest) = result.ok() else {
        return;
    };

    assert!(manifest.contains("\"kind\":\"p3d_container\""));
    assert!(!manifest.contains("\"kind\":\"language_textbible\""));
    assert!(!manifest.contains("\"textbible_only_chunks\""));
    assert!(!manifest.contains("\"normalized_by\":\"lang\""));
}

#[test]
fn expanded_car_tag_requires_a_path_token() {
    let result = generate_named("scarcity.p3d");
    assert!(result.is_ok());
    let Some(manifest) = result.ok() else {
        return;
    };

    assert!(!manifest.contains("\"vehicle\""));
}

#[test]
fn expanded_mission_tag_requires_a_path_token() {
    let result = generate_named("permission.p3d");
    assert!(result.is_ok());
    let Some(manifest) = result.ok() else {
        return;
    };

    assert!(!manifest.contains("\"mission\""));
}

#[test]
fn expanded_vehicle_tag_requires_a_path_token() {
    let result = generate_named("nonvehicle.p3d");
    assert!(result.is_ok());
    let Some(manifest) = result.ok() else {
        return;
    };

    assert!(!manifest.contains("\"vehicle\""));
}

#[test]
fn expanded_world_tag_requires_a_path_token() {
    for file_name in ["cleveland.p3d", "terrace.p3d", "worldview.p3d"] {
        let result = generate_named(file_name);
        assert!(result.is_ok());
        let Some(manifest) = result.ok() else {
            continue;
        };

        assert!(!manifest.contains("\"world\""));
    }
}

#[test]
fn expanded_ui_tag_requires_a_path_token() {
    for file_name in ["backendfrontend.p3d", "miscrooby.p3d"] {
        let result = generate_named(file_name);
        assert!(result.is_ok());
        let Some(manifest) = result.ok() else {
            continue;
        };

        assert!(!manifest.contains("\"ui\""));
    }
}
