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
//   - Text outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Text outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for pipeline.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Text outbound adapter.

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
        by_subcategory.entry(subcategory.clone()).or_default().push(
            TextKeyDraft {
                id,
                key,
                source_unit_id: source_unit_id.to_owned(),
                subcategory,
            },
        );
    }
    Ok(by_subcategory
        .into_iter()
        .map(|(subcategory, mut keys)| {
            keys.sort_by(|left, right| left.key.cmp(&right.key));
            TextPackageDraft {
                package_root: format!("derived/{subcategory}"),
                subcategory,
                source_unit_ids: vec![source_unit_id.to_owned()],
                keys,
            }
        })
        .collect())
}

#[cfg(test)]
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../../../../tests/migration/pipeline/unit/adapter-outbound/local/two/text/tests.rs"]
mod tests;
