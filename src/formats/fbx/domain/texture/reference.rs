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
//   - Reference domain module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Reference domain module.
// - Description:
//   - Implements the declared domain module responsibility for fbx.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Reference domain module.

#![expect(
    clippy::module_name_repetitions,
    reason = "Tests verify these intentional explicit file-local contracts \
              remain safe."
)]

/// Stable reference to a decoded texture artifact.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TextureReference {
    /// Stable texture id from package evidence.
    pub id: String,
    /// Human-readable label retained for reports.
    pub label: String,
}

/// Convert one stable texture id into a domain texture reference.
#[must_use]
pub fn texture_reference(
    id: impl Into<String>,
    label: impl Into<String>,
) -> TextureReference {
    TextureReference {
        id: id.into(),
        label: label.into(),
    }
}
