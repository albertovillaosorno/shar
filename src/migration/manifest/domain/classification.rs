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
//   - Classification domain module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Classification domain module.
// - Description:
//   - Implements the declared domain module responsibility for manifest.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Classification domain module.

#[must_use]
/// Classify one manifest directory and extension into a stable bucket.
pub fn classify_manifest_bucket(dir: &str, extension: &str) -> String {
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
    if matches!(extension, "rsd" | "wav") {
        return "audio".to_owned();
    }
    if extension == "rms" {
        return "music_arrangement".to_owned();
    }
    if matches!(extension, "mfk" | "con" | "lua") {
        return "script".to_owned();
    }
    if matches!(extension, "ico" | "bmp" | "tga" | "jpg" | "jpeg") {
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
    if matches!(extension, "txt" | "e" | "f" | "g" | "i" | "s" | "x") {
        return "language_textbible".to_owned();
    }
    if matches!(extension, "prj" | "pag" | "scr") {
        return "ui-resource".to_owned();
    }
    if extension == "rtf" {
        return "document".to_owned();
    }
    if extension == "err" {
        return "build-log".to_owned();
    }
    if matches!(extension, "bik" | "bk2") {
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
