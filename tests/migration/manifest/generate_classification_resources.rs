// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT
//
// Boundary-Contract:
// - Owns:
//   - Generate classification resources test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Generate classification resources test module.
// - Description:
//   - Implements the declared test module responsibility for manifest.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Generate classification resources test module.

use game_manifest::classify_manifest_bucket;
use schoenwald_cli as _;
use schoenwald_filesystem as _;

#[test]
fn typ_buckets_are_sound_types() {
    assert_eq!(classify_manifest_bucket("aa", "typ"), "sound-type");
}

#[test]
fn textbible_variants_are_language_text() {
    for extension in ["txt", "e", "f", "g", "i", "s", "x"] {
        assert_eq!(
            classify_manifest_bucket("aa", extension),
            "language_textbible"
        );
    }
}

#[test]
fn ui_resource_buckets_are_classified() {
    for extension in ["prj", "pag", "scr"] {
        assert_eq!(classify_manifest_bucket("aa", extension), "ui-resource");
    }
}

#[test]
fn rtf_buckets_are_documents() {
    assert_eq!(classify_manifest_bucket("aa", "rtf"), "document");
}

#[test]
fn err_buckets_are_build_logs() {
    assert_eq!(classify_manifest_bucket("aa", "err"), "build-log");
}

#[test]
fn lmlm_buckets_are_language_mods_at_any_depth() {
    assert_eq!(classify_manifest_bucket("aa", "lmlm"), "language_mod");
}

#[test]
fn unclassified_buckets_use_error_sentinel() {
    assert_eq!(classify_manifest_bucket("aa", "mystery"), "error");
}

#[test]
fn bink_buckets_are_movies() {
    for extension in ["bik", "bk2"] {
        assert_eq!(classify_manifest_bucket("aa", extension), "movie");
    }
}

#[test]
fn jsonl_buckets_are_ledgers() {
    assert_eq!(classify_manifest_bucket("aa", "jsonl"), "json-ledger");
}

#[test]
fn json_buckets_are_metadata() {
    assert_eq!(classify_manifest_bucket("aa", "json"), "metadata");
}

#[test]
fn rsm_buckets_are_music_arrangements() {
    assert_eq!(classify_manifest_bucket("aa", "rsm"), "music_arrangement");
}
