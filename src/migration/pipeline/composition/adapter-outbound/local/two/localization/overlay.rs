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
//   - Overlay outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Overlay outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for pipeline.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Overlay outbound adapter.

use std::collections::{BTreeMap, BTreeSet};

use super::{
    CustomTextEntry, Error, LanguageDocument, Outcome, OverlayEntry,
    OverlayMerge, custom_entry_hash,
};

/// Apply custom text entries to one decoded base language.
///
/// # Errors
///
/// Returns an error when the base modulus is invalid or two custom keys resolve
/// to the same hash.
pub(super) fn apply_overlay(
    base: &LanguageDocument,
    custom: &[CustomTextEntry],
) -> Outcome<OverlayMerge> {
    let mut by_hash = BTreeMap::new();
    for entry in custom {
        let hash = custom_entry_hash(&entry.key, base.modulo)?;
        if let Some(previous) = by_hash.insert(hash, entry) {
            return Err(Error::invalid(format!(
                "custom-text keys '{}' and '{}' resolve to hash {hash}",
                previous.key, entry.key
            )));
        }
    }
    let mut seen_base_hashes = BTreeSet::new();
    for entry in &base.entries {
        if !seen_base_hashes.insert(entry.hash)
            && by_hash.contains_key(&entry.hash)
        {
            return Err(Error::invalid(format!(
                "custom-text hash {} matches multiple base entries",
                entry.hash
            )));
        }
    }
    let mut matched = BTreeSet::new();
    let entries = base
        .entries
        .iter()
        .map(|entry| {
            by_hash.get(&entry.hash).map_or_else(
                || OverlayEntry {
                    hash: entry.hash,
                    offset: entry.offset,
                    value: entry.value.clone(),
                    value_source: "base_language",
                    overlay_key: None,
                },
                |overlay| {
                    let _inserted = matched.insert(entry.hash);
                    OverlayEntry {
                        hash: entry.hash,
                        offset: entry.offset,
                        value: overlay.value.clone(),
                        value_source: "custom_text",
                        overlay_key: Some(overlay.key.clone()),
                    }
                },
            )
        })
        .collect();
    let mut unmatched = Vec::new();
    for entry in custom {
        let hash = custom_entry_hash(&entry.key, base.modulo)?;
        if !matched.contains(&hash) {
            unmatched.push(entry.clone());
        }
    }
    Ok(OverlayMerge { entries, unmatched })
}

#[cfg(test)]
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../../../../../tests/migration/pipeline/unit/adapter-outbound/local/two/localization/overlay/tests.rs"]
mod tests;
