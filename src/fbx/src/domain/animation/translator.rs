// File:
//   - translator.rs
// Path:
//   - src/fbx/src/domain/animation/translator.rs
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
//   - Pure fbx domain rules for domain animation translator.
// - Must-Not:
//   - Read files, parse generated indexes, invoke CLI code, or call writer
//   - adapters.
// - Allows:
//   - Value objects, invariant checks, and pure evidence-to-domain translation.
// - Split-When:
//   - Split when translator contains two independently testable contracts.
// - Merge-When:
//   - Another fbx module owns the same domain boundary with no distinct
//   - invariant.
// - Summary:
//   - Preserve animation ids as an explicit requirement until clip binding
//   - exists.
// - Description:
//   - Defines translator data and behavior for fbx domain animation.
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

//! Preserve animation ids as an explicit requirement until clip binding exists.
use super::capability::AnimationCapability;
use super::requirement::{AnimationRequirement, AnimationRequirementError};

/// Preserve animation ids as an explicit requirement until clip binding exists.
///
/// # Errors
///
/// Returns an error when one animation member identity is blank or duplicated.
pub fn preserve_animation_ids(
    member_ids: Vec<String>
) -> Result<AnimationRequirement, AnimationRequirementError> {
    AnimationRequirement::new(
        member_ids,
        AnimationCapability::PreservedOnly,
    )
}
