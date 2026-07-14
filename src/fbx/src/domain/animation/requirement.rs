// File:
//   - requirement.rs
// Path:
//   - src/fbx/src/domain/animation/requirement.rs
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
//   - Pure fbx domain rules for domain animation requirement.
// - Must-Not:
//   - Read files, parse generated indexes, invoke CLI code, or call writer
//   - adapters.
// - Allows:
//   - Value objects, invariant checks, and pure evidence-to-domain translation.
// - Split-When:
//   - Split when requirement contains two independently testable contracts.
// - Merge-When:
//   - Another fbx module owns the same domain boundary with no distinct
//   - invariant.
// - Summary:
//   - Animation requirement for one export package.
// - Description:
//   - Defines requirement data and behavior for fbx domain animation.
// - Usage:
//   - Imported through crate domain facades or sibling domain modules.
// - Defaults:
//   - No filesystem paths, no external process calls, and no implicit IO
//   - defaults.
//
// ADRs:
// - docs/adr/pipeline/fbx/hexagonal-scene-export.md
// - docs/adr/pipeline/unreal/unreal-manifest-and-package-taxonomy.md
//
// Large file:
//   - false
//

//! Animation requirement for one export package.
//!
//! This boundary keeps animation requirement for one export package explicit
//! and returns deterministic results to fbx callers.
// These exact file-local lints preserve explicit domain and binary contracts.
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
            .any(
                |member_id| {
                    member_id
                        .trim()
                        .is_empty()
                },
            )
        {
            return Err(AnimationRequirementError::BlankMemberId);
        }
        if member_ids
            .iter()
            .any(
                |member_id| {
                    member_id != member_id.trim()
                        || member_id
                            .chars()
                            .any(char::is_control)
                },
            )
        {
            return Err(AnimationRequirementError::NonCanonicalMemberId);
        }
        let mut unique_member_ids = BTreeSet::new();
        if member_ids
            .iter()
            .any(
                |member_id| {
                    !unique_member_ids.insert(member_id.to_ascii_lowercase())
                },
            )
        {
            return Err(AnimationRequirementError::DuplicateMemberId);
        }
        member_ids.sort();
        Ok(
            Self {
                member_ids,
                capability,
            },
        )
    }

    /// Returns true when animation data must be preserved in the export report.
    #[must_use]
    pub fn requires_report(&self) -> bool {
        !self
            .member_ids
            .is_empty()
            && self.capability != AnimationCapability::BoundClip
    }
}
