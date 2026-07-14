// File:
//   - generate_classification.rs
// Path:
//   - src/game-manifest/tests/generate_classification.rs
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
//   - Generated manifest classification regressions.
// - Must-Not:
//   - Read licensed inputs or mutable repository output trees.
// - Allows:
//   - Synthetic temporary files and the compiled generator executable.
// - Split-When:
//   - Split when one classification family needs independent fixtures.
// - Merge-When:
//   - Another test owns generator classification output end to end.
// - Summary:
//   - Protects deterministic bucket classification in generated ledgers.
// - Description:
//   - Executes the real generator against synthetic extension buckets.
// - Usage:
//   - Executed through cargo test for the game-manifest crate.
// - Defaults:
//   - Every temporary fixture is removed after command execution.
//
// ADRs:
// - docs/adr/pipeline/game-manifest-ledger.md
//
// Large file:
//   - false
//

//! End-to-end generator classification regression coverage.
//!
//! Synthetic files prove the emitted ledger classification without exposing
//! source names or depending on repository-local outputs.

use std::fs;
use std::io::{self, ErrorKind};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

use game_manifest::{MANIFEST_FILE_NAME, classify_manifest_bucket};
use schoenwald_cli as _;
use schoenwald_filesystem as _;

static NEXT_FIXTURE: AtomicU64 = AtomicU64::new(0);

fn generated_manifest(extension: &str) -> io::Result<String> {
    let sequence = NEXT_FIXTURE.fetch_add(
        1,
        Ordering::Relaxed,
    );
    let root = std::env::temp_dir().join(
        format!(
            "game-manifest-classify-{}-{sequence}",
            std::process::id()
        ),
    );
    match fs::remove_dir_all(&root) {
        Ok(()) => {}
        Err(error) if error.kind() == ErrorKind::NotFound => {}
        Err(error) => return Err(error),
    }
    fs::create_dir_all(&root)?;
    let result = (|| {
        fs::write(
            root.join(format!("asset.{extension}")),
            b"fixture",
        )?;
        let output = Command::new(env!("CARGO_BIN_EXE_generate-manifest"))
            .arg(&root)
            .output()?;
        if !output
            .status
            .success()
        {
            return Err(
                io::Error::other(String::from_utf8_lossy(&output.stderr)),
            );
        }
        fs::read_to_string(root.join(MANIFEST_FILE_NAME))
    })();
    drop(fs::remove_dir_all(&root));
    result
}

#[test]
fn p3d_buckets_are_classified_as_containers() {
    let result = generated_manifest("p3d");
    assert!(result.is_ok());
    let Some(manifest) = result.ok() else {
        return;
    };
    assert!(
        manifest
            .contains("\"ext\":\"p3d\",\"min\":1,\"kind\":\"p3d_container\"")
    );
}

#[test]
fn rmv_buckets_are_movies_in_any_directory() {
    assert_eq!(
        classify_manifest_bucket(
            "aa", "rmv"
        ),
        "movie"
    );
}

#[test]
fn rcf_buckets_are_archive_containers() {
    assert_eq!(
        classify_manifest_bucket(
            "aa", "rcf"
        ),
        "rcf_container"
    );
}

#[test]
fn decoded_audio_buckets_are_audio() {
    for extension in [
        "rsd", "wav",
    ] {
        assert_eq!(
            classify_manifest_bucket(
                "aa", extension
            ),
            "audio"
        );
    }
}

#[test]
fn rms_buckets_are_music_arrangements() {
    assert_eq!(
        classify_manifest_bucket(
            "aa", "rms"
        ),
        "music_arrangement"
    );
}

#[test]
fn script_buckets_are_scripts() {
    for extension in [
        "mfk", "con", "lua",
    ] {
        assert_eq!(
            classify_manifest_bucket(
                "aa", extension
            ),
            "script"
        );
    }
}

#[test]
fn image_buckets_are_images() {
    for extension in [
        "ico", "bmp", "tga", "jpg", "jpeg",
    ] {
        assert_eq!(
            classify_manifest_bucket(
                "aa", extension
            ),
            "image"
        );
    }
}

#[test]
fn nested_png_buckets_are_images() {
    assert_eq!(
        classify_manifest_bucket(
            "aa", "png"
        ),
        "image"
    );
    assert_eq!(
        classify_manifest_bucket(
            "", "png"
        ),
        "generated_artifact"
    );
}

#[test]
fn cho_buckets_are_character_outfits() {
    assert_eq!(
        classify_manifest_bucket(
            "aa", "cho"
        ),
        "character_outfit"
    );
}
