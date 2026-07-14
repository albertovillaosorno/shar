// File:
//   - text.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/two/text.rs
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
//   - The language text contract for pipeline phase two minor units.
// - Must-Not:
//   - Violate repository architecture, path, provenance, or output rules.
// - Allows:
//   - Operations required to validate and execute language text.
// - Split-When:
//   - Split when language text contains two independently testable
//   - contracts.
// - Merge-When:
//   - Another pipeline module owns the same module boundary with no distinct
//   - invariant.
// - Summary:
//   - Logical text keys stay derived from physical source units so
//   - importable.
// - Description:
//   - Defines language text data and behavior for pipeline phase two minor
//   - units.
// - Usage:
//   - Used by pipeline phase two minor units code that needs language text.
// - Defaults:
//   - No implicit output outside the repository is allowed.
//
// ADRs:
// - docs/adr/pipeline/minor-unit-taxonomy-value-case.md
//
// Large file:
//   - false
//

//! Logical text package derivation from physical source units.
//!
//! Importable groups retain one manifest owner per source unit so language
//! variants never duplicate coverage or lose deterministic package identity.

use std::collections::BTreeMap;
use std::path::Path;

use super::units::metadata_fill::compute_id;
use crate::domain::PipelineError;

pub(super) mod classification;
pub(super) mod drafts;
pub(super) mod entities;
pub(super) mod interface;
pub(super) mod matching;
pub(super) mod missions;
pub(super) mod settings;
pub(super) mod source;

use classification::classify_text_key;
pub(super) use drafts::{TextKeyDraft, TextPackageDraft};
use source::read_custom_text_keys;

/// Shares the `PipelineOutcome` result shape across this module boundary.
type PipelineOutcome<T> = Result<T, PipelineError>;

/// Supports the `derive_text_packages` operation within this deterministic
/// classification boundary.
pub(super) fn derive_text_packages(
    extracted_root: &Path,
    source_unit_id: &str,
    source_path: &str,
    kind: &str,
) -> PipelineOutcome<Vec<TextPackageDraft>> {
    if kind != "localization-override" {
        return Ok(Vec::new());
    }
    let Some(relative) = source_path.strip_prefix("extracted/") else {
        return Ok(Vec::new());
    };
    let path = extracted_root.join(relative);
    let mut by_subcategory = BTreeMap::<String, Vec<TextKeyDraft>>::new();
    for key in read_custom_text_keys(&path)? {
        let subcategory = classify_text_key(&key);
        let id = compute_id(
            &format!("{source_unit_id}|{key}|{subcategory}"),
            "text-key",
            "derived",
        );
        by_subcategory
            .entry(subcategory.clone())
            .or_default()
            .push(
                TextKeyDraft {
                    id,
                    key,
                    source_unit_id: source_unit_id.to_owned(),
                    subcategory,
                },
            );
    }
    Ok(
        by_subcategory
            .into_iter()
            .map(
                |(subcategory, mut keys)| {
                    keys.sort_by(
                        |left, right| {
                            left.key
                                .cmp(&right.key)
                        },
                    );
                    TextPackageDraft {
                        package_root: format!("derived/{subcategory}"),
                        subcategory,
                        source_unit_ids: vec![source_unit_id.to_owned()],
                        keys,
                    }
                },
            )
            .collect(),
    )
}

#[cfg(test)]
mod tests;
