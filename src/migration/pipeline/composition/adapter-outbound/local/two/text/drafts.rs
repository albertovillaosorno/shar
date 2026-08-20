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
//   - Drafts outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Drafts outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for pipeline.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Drafts outbound adapter.

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
