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
//   - Requirement domain module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Requirement domain module.
// - Description:
//   - Implements the declared domain module responsibility for fbx.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Requirement domain module.

#![expect(
    clippy::module_name_repetitions,
    reason = "Tests verify these intentional explicit file-local contracts \
              remain safe."
)]

use std::collections::BTreeSet;

use super::capability::AnimationCapability;

/// Animation-requirement validation failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AnimationRequirementError {
    /// One animation member id was empty or whitespace-only.
    BlankMemberId,
    /// One animation member id carried surrounding whitespace.
    NonCanonicalMemberId,
    /// One animation member id appeared more than once.
    DuplicateMemberId,
}

/// Animation requirement for one export package.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AnimationRequirement {
    /// Animation member ids referenced by package evidence.
    pub member_ids: Vec<String>,
    /// Capability state selected by the planner.
    pub capability: AnimationCapability,
}

impl AnimationRequirement {
    /// Create an explicit animation requirement from member evidence.
    ///
    /// # Errors
    ///
    /// Returns an error when member identities are blank or duplicated.
    pub fn new(
        mut member_ids: Vec<String>,
        capability: AnimationCapability,
    ) -> Result<Self, AnimationRequirementError> {
        if member_ids
            .iter()
            .any(|member_id| member_id.trim().is_empty())
        {
            return Err(AnimationRequirementError::BlankMemberId);
        }
        if member_ids.iter().any(|member_id| {
            member_id != member_id.trim()
                || member_id.chars().any(char::is_control)
        }) {
            return Err(AnimationRequirementError::NonCanonicalMemberId);
        }
        let mut unique_member_ids = BTreeSet::new();
        if member_ids.iter().any(|member_id| {
            !unique_member_ids.insert(member_id.to_ascii_lowercase())
        }) {
            return Err(AnimationRequirementError::DuplicateMemberId);
        }
        member_ids.sort();
        Ok(Self { member_ids, capability })
    }

    /// Returns true when animation data must be preserved in the export report.
    #[must_use]
    pub fn requires_report(&self) -> bool {
        !self.member_ids.is_empty()
            && self.capability != AnimationCapability::BoundClip
    }
}
