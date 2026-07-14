// File:
//   - overlay.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/two/localization/overlay.rs
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
//   - Deterministic custom-text overlay application.
// - Must-Not:
//   - Overwrite colliding identities silently or reorder base language
//   - entries.
// - Allows:
//   - Hash-based matching with explicit unmatched-record preservation.
// - Split-When:
//   - Overlay precedence gains another independent policy layer.
// - Merge-When:
//   - Another localization use case owns identical merge and provenance
//   - rules.
// - Summary:
//   - Deterministic localization overlay merge.
// - Description:
//   - Preserves base order and rejects ambiguous custom hash collisions.
// - Usage:
//   - Called after strict source decoding and before package derivation.
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

//! Overlay merges preserve base order and expose unmatched source records.

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
        let hash = custom_entry_hash(
            &entry.key,
            base.modulo,
        )?;
        if let Some(previous) = by_hash.insert(
            hash, entry,
        ) {
            return Err(
                Error::invalid(
                    format!(
                        "custom-text keys '{}' and '{}' resolve to hash {hash}",
                        previous.key, entry.key
                    ),
                ),
            );
        }
    }
    let mut seen_base_hashes = BTreeSet::new();
    for entry in &base.entries {
        if !seen_base_hashes.insert(entry.hash)
            && by_hash.contains_key(&entry.hash)
        {
            return Err(
                Error::invalid(
                    format!(
                        "custom-text hash {} matches multiple base entries",
                        entry.hash
                    ),
                ),
            );
        }
    }
    let mut matched = BTreeSet::new();
    let entries = base
        .entries
        .iter()
        .map(
            |entry| {
                by_hash
                    .get(&entry.hash)
                    .map_or_else(
                        || OverlayEntry {
                            hash: entry.hash,
                            offset: entry.offset,
                            value: entry
                                .value
                                .clone(),
                            value_source: "base_language",
                            overlay_key: None,
                        },
                        |overlay| {
                            let _inserted = matched.insert(entry.hash);
                            OverlayEntry {
                                hash: entry.hash,
                                offset: entry.offset,
                                value: overlay
                                    .value
                                    .clone(),
                                value_source: "custom_text",
                                overlay_key: Some(
                                    overlay
                                        .key
                                        .clone(),
                                ),
                            }
                        },
                    )
            },
        )
        .collect();
    let mut unmatched = Vec::new();
    for entry in custom {
        let hash = custom_entry_hash(
            &entry.key,
            base.modulo,
        )?;
        if !matched.contains(&hash) {
            unmatched.push(entry.clone());
        }
    }
    Ok(
        OverlayMerge {
            entries,
            unmatched,
        },
    )
}

#[cfg(test)]
mod tests {
    use super::apply_overlay;
    use crate::adapters::driven::local::two::localization::{
        CustomTextEntry, LanguageDocument, LanguageEntry,
    };

    #[test]
    fn rejects_overlay_targeting_duplicate_base_hash() -> Result<(), String> {
        let base = LanguageDocument {
            id: 'S',
            language: "spanish_spain",
            source_name: "base".to_owned(),
            modulo: 1009,
            entries: vec![
                LanguageEntry {
                    hash: 16,
                    offset: 0,
                    value: "one".to_owned(),
                },
                LanguageEntry {
                    hash: 16,
                    offset: 2,
                    value: "two".to_owned(),
                },
            ],
        };
        let custom = [
            CustomTextEntry {
                key: "0x10".to_owned(),
                value: "replacement".to_owned(),
                line: 1,
            },
        ];
        if apply_overlay(
            &base, &custom,
        )
        .is_err()
        {
            Ok(())
        } else {
            Err("overlay silently replaced duplicate base hashes".to_owned())
        }
    }

    #[test]
    fn preserves_untargeted_duplicate_base_hashes() -> Result<(), String> {
        let base = LanguageDocument {
            id: 'S',
            language: "spanish_spain",
            source_name: "base".to_owned(),
            modulo: 1009,
            entries: vec![
                LanguageEntry {
                    hash: 16,
                    offset: 0,
                    value: "one".to_owned(),
                },
                LanguageEntry {
                    hash: 16,
                    offset: 2,
                    value: "two".to_owned(),
                },
            ],
        };
        let merge = apply_overlay(
            &base,
            &[],
        )
        .map_err(|error| error.to_string())?;
        if merge
            .entries
            .iter()
            .map(
                |entry| {
                    entry
                        .value
                        .as_str()
                },
            )
            .eq(
                [
                    "one", "two",
                ],
            )
        {
            Ok(())
        } else {
            Err(
                format!(
                    "duplicate base values changed: {:?}",
                    merge.entries
                ),
            )
        }
    }

    #[test]
    fn rejects_colliding_custom_hashes() -> Result<(), String> {
        let base = LanguageDocument {
            id: 'S',
            language: "spanish_spain",
            source_name: "base".to_owned(),
            modulo: 1,
            entries: vec![
                LanguageEntry {
                    hash: 0,
                    offset: 0,
                    value: "base".to_owned(),
                },
            ],
        };
        let custom = vec![
            CustomTextEntry {
                key: "A".to_owned(),
                value: "first".to_owned(),
                line: 1,
            },
            CustomTextEntry {
                key: "B".to_owned(),
                value: "second".to_owned(),
                line: 2,
            },
        ];
        if apply_overlay(
            &base, &custom,
        )
        .is_err()
        {
            Ok(())
        } else {
            Err("colliding overlay identities were accepted".to_owned())
        }
    }
}
