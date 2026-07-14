// File:
//   - drafts.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/two/text/drafts.rs
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
//   - The drafts contract for pipeline phase two minor units language text.
// - Must-Not:
//   - Violate repository architecture, path, provenance, or output rules.
// - Allows:
//   - Operations required to validate and execute drafts.
// - Split-When:
//   - Split when drafts contains two independently testable contracts.
// - Merge-When:
//   - Another pipeline module owns the same module boundary with no distinct
//   - invariant.
// - Summary:
//   - Drafts for pipeline phase two minor units language text.
// - Description:
//   - Defines drafts data and behavior for pipeline phase two minor units
//   - language text.
// - Usage:
//   - Used by pipeline phase two minor units language text code that needs
//   - drafts.
// - Defaults:
//   - No implicit output outside the repository is allowed.
//
// ADRs:
// - docs/adr/pipeline/minor-unit-taxonomy-value-case.md
//
// Large file:
//   - false
//

//! This module keeps phase two minor units language text drafts behavior inside
//! its Rust boundary.
#[derive(Clone, Debug, Eq, PartialEq)]
/// Groups `TextKeyDraft` evidence for deterministic package classification.
pub(in crate::adapters::driven::local::two) struct TextKeyDraft {
    /// Stores `id` evidence required by this deterministic record.
    pub(in crate::adapters::driven::local::two) id: String,
    /// Stores `key` evidence required by this deterministic record.
    pub(in crate::adapters::driven::local::two) key: String,
    /// Stores `source_unit_id` evidence required by this deterministic record.
    pub(in crate::adapters::driven::local::two) source_unit_id: String,
    /// Stores `subcategory` evidence required by this deterministic record.
    pub(in crate::adapters::driven::local::two) subcategory: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Groups `TextPackageDraft` evidence for deterministic package classification.
pub(in crate::adapters::driven::local::two) struct TextPackageDraft {
    /// Stores `package_root` evidence required by this deterministic record.
    pub(in crate::adapters::driven::local::two) package_root: String,
    /// Stores `subcategory` evidence required by this deterministic record.
    pub(in crate::adapters::driven::local::two) subcategory: String,
    /// Stores `source_unit_ids` evidence required by this deterministic
    /// record.
    pub(in crate::adapters::driven::local::two) source_unit_ids: Vec<String>,
    /// Stores `keys` evidence required by this deterministic record.
    pub(in crate::adapters::driven::local::two) keys: Vec<TextKeyDraft>,
}
