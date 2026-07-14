// File:
//   - localization.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/two/localization.rs
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
//   - The normalized localization source boundary for phase two.
// - Must-Not:
//   - Classify packages, render final Unreal assets, or own output paths.
// - Allows:
//   - Strict TextBible decoding, custom-text parsing, hashing, and overlays.
// - Split-When:
//   - A source family needs an independently versioned normalization
//   - contract.
// - Merge-When:
//   - Another phase-two module owns the same normalized localization
//   - contract.
// - Summary:
//   - Normalized localization sources consumed by package derivation.
// - Description:
//   - Keeps source decoding separate from package classification and import
//   - IO.
// - Usage:
//   - Called by phase-two language package derivation and future phase-three
//   - plans.
// - Defaults:
//   - Malformed source data fails closed without implicit output.
//
// ADRs:
// - docs/adr/pipeline/minor-unit-taxonomy-value-case.md
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Normalized localization source contracts for phase two.
//!
//! Text decoding stays independent from package classification so every source
//! can fail closed before stable text-key packages are derived.

mod binary;
mod custom_text;
pub(super) mod encoding;
mod error;
mod hash;
mod model;
mod overlay;
mod text_bible;

use custom_text::parse_custom_text;
use error::{Error, Outcome};
use hash::{custom_entry_hash, hash_key};
use model::{
    CustomTextEntry, LanguageDocument, LanguageEntry, OverlayEntry,
    OverlayMerge, TextBibleDocument,
};
use overlay::apply_overlay;
use text_bible::parse_text_bible;

/// Reads validated custom-text keys for sibling phase-two adapters.
///
/// # Errors
///
/// Returns a pipeline failure when source IO or text validation fails.
pub(super) fn read_custom_text_keys(
    path: &std::path::Path
) -> Result<Vec<String>, crate::domain::PipelineError> {
    parse_custom_text(path)
        .map(
            |entries| {
                entries
                    .into_iter()
                    .map(|entry| entry.key)
                    .collect()
            },
        )
        .map_err(|error| crate::domain::PipelineError::new(error.to_string()))
}
