// File:
//   - generate_classification_resources.rs
// Path:
//   - src/game-manifest/tests/generate_classification_resources.rs
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
//   - Resource and metadata classification regressions.
// - Must-Not:
//   - Depend on path names beyond approved obfuscated coordinates.
// - Allows:
//   - Pure extension-to-kind assertions.
// - Split-When:
//   - Split when resource families require filesystem evidence.
// - Merge-When:
//   - Another test owns the same resource classification boundary.
// - Summary:
//   - Protects deterministic resource-kind classification.
// - Description:
//   - Verifies known metadata and resource formats never fall through.
// - Usage:
//   - Executed through cargo test for the game-manifest crate.
// - Defaults:
//   - Tests use synthetic extension tokens only.
//
// ADRs:
// - docs/adr/pipeline/game-manifest-ledger.md
//
// Large file:
//   - false
//

//! Resource and metadata classification regression coverage.
//!
//! Pure extension evidence keeps these tests deterministic and independent of
//! licensed or generated trees.

use game_manifest::classify_manifest_bucket;
use schoenwald_cli as _;
use schoenwald_filesystem as _;

#[test]
fn typ_buckets_are_sound_types() {
    assert_eq!(
        classify_manifest_bucket(
            "aa", "typ"
        ),
        "sound-type"
    );
}

#[test]
fn textbible_variants_are_language_text() {
    for extension in [
        "txt", "e", "f", "g", "i", "s", "x",
    ] {
        assert_eq!(
            classify_manifest_bucket(
                "aa", extension
            ),
            "language_textbible"
        );
    }
}

#[test]
fn ui_resource_buckets_are_classified() {
    for extension in [
        "prj", "pag", "scr",
    ] {
        assert_eq!(
            classify_manifest_bucket(
                "aa", extension
            ),
            "ui-resource"
        );
    }
}

#[test]
fn rtf_buckets_are_documents() {
    assert_eq!(
        classify_manifest_bucket(
            "aa", "rtf"
        ),
        "document"
    );
}

#[test]
fn err_buckets_are_build_logs() {
    assert_eq!(
        classify_manifest_bucket(
            "aa", "err"
        ),
        "build-log"
    );
}

#[test]
fn lmlm_buckets_are_language_mods_at_any_depth() {
    assert_eq!(
        classify_manifest_bucket(
            "aa", "lmlm"
        ),
        "language_mod"
    );
}

#[test]
fn unclassified_buckets_use_error_sentinel() {
    assert_eq!(
        classify_manifest_bucket(
            "aa", "mystery"
        ),
        "error"
    );
}

#[test]
fn bink_buckets_are_movies() {
    for extension in [
        "bik", "bk2",
    ] {
        assert_eq!(
            classify_manifest_bucket(
                "aa", extension
            ),
            "movie"
        );
    }
}

#[test]
fn jsonl_buckets_are_ledgers() {
    assert_eq!(
        classify_manifest_bucket(
            "aa", "jsonl"
        ),
        "json-ledger"
    );
}

#[test]
fn json_buckets_are_metadata() {
    assert_eq!(
        classify_manifest_bucket(
            "aa", "json"
        ),
        "metadata"
    );
}

#[test]
fn rsm_buckets_are_music_arrangements() {
    assert_eq!(
        classify_manifest_bucket(
            "aa", "rsm"
        ),
        "music_arrangement"
    );
}
