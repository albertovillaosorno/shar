// File:
//   - classification.rs
// Path:
//   - src/game-manifest/src/domain/classification.rs
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
//   - Deterministic kind classification for minimum-manifest buckets.
// - Must-Not:
//   - Inspect private names beyond obfuscated coordinates.
// - Allows:
//   - Extension and approved obfuscated-coordinate classification.
// - Split-When:
//   - Split when classification requires independent evidence providers.
// - Merge-When:
//   - Another module owns the same minimum-manifest classification contract.
// - Summary:
//   - Maps manifest bucket evidence to controlled taxonomy values.
// - Description:
//   - Centralizes generator classification so tests and callers share one rule.
// - Usage:
//   - Called by the minimum manifest generator for every counted bucket.
// - Defaults:
//   - Unrecognized evidence remains explicit as error.
//
// ADRs:
// - docs/adr/pipeline/game-manifest-ledger.md
//
// Large file:
//   - false
//

//! Minimum-manifest bucket classification.
//!
//! Rules use only extension evidence and approved obfuscated coordinates so the
//! generated ledger remains deterministic and confidentiality-safe.

/// Classifies one minimum-manifest bucket into the controlled taxonomy.
#[must_use]
pub fn classify_manifest_bucket(
    dir: &str,
    extension: &str,
) -> String {
    if extension == "lmlm" {
        return "language_mod".to_owned();
    }
    if extension == "png" && dir.is_empty() {
        return "generated_artifact".to_owned();
    }
    if extension == "rmv" {
        return "movie".to_owned();
    }
    if extension == "p3d"
        && (dir == "at/fd/sy/re/te" || dir == "at/fd/s2/re/te")
    {
        return "language_textbible".to_owned();
    }
    if extension == "p3d" {
        return "p3d_container".to_owned();
    }
    if extension == "rcf" {
        return "rcf_container".to_owned();
    }
    if matches!(
        extension,
        "rsd" | "wav"
    ) {
        return "audio".to_owned();
    }
    if extension == "rms" {
        return "music_arrangement".to_owned();
    }
    if matches!(
        extension,
        "mfk" | "con" | "lua"
    ) {
        return "script".to_owned();
    }
    if matches!(
        extension,
        "ico" | "bmp" | "tga" | "jpg" | "jpeg"
    ) {
        return "image".to_owned();
    }
    if extension == "png" {
        return "image".to_owned();
    }
    if extension == "cho" {
        return "character_outfit".to_owned();
    }
    if extension == "typ" {
        return "sound-type".to_owned();
    }
    if matches!(
        extension,
        "txt" | "e" | "f" | "g" | "i" | "s" | "x"
    ) {
        return "language_textbible".to_owned();
    }
    if matches!(
        extension,
        "prj" | "pag" | "scr"
    ) {
        return "ui-resource".to_owned();
    }
    if extension == "rtf" {
        return "document".to_owned();
    }
    if extension == "err" {
        return "build-log".to_owned();
    }
    if matches!(
        extension,
        "bik" | "bk2"
    ) {
        return "movie".to_owned();
    }
    if extension == "jsonl" {
        return "json-ledger".to_owned();
    }
    if extension == "json" {
        return "metadata".to_owned();
    }
    if extension == "rsm" {
        return "music_arrangement".to_owned();
    }
    "error".to_owned()
}
