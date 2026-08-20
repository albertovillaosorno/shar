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
//   - Localization outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Localization outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for pipeline.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Localization outbound adapter.

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
    path: &std::path::Path,
) -> Result<Vec<String>, crate::domain::PipelineError> {
    parse_custom_text(path)
        .map(|entries| entries.into_iter().map(|entry| entry.key).collect())
        .map_err(|error| crate::domain::PipelineError::new(error.to_string()))
}
