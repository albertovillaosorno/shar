// File:
//   - capability.rs
// Path:
//   - src/fbx/src/domain/animation/capability.rs
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
//   - Pure fbx domain rules for domain animation capability.
// - Must-Not:
//   - Read files, parse generated indexes, invoke CLI code, or call writer
//   - adapters.
// - Allows:
//   - Value objects, invariant checks, and pure evidence-to-domain translation.
// - Split-When:
//   - Split when capability contains two independently testable contracts.
// - Merge-When:
//   - Another fbx module owns the same domain boundary with no distinct
//   - invariant.
// - Summary:
//   - Animation capability observed in a package.
// - Description:
//   - Defines capability data and behavior for fbx domain animation.
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

//! Animation capability observed in a package.
//!
//! This boundary keeps animation capability observed in a package explicit and
//! returns deterministic results to fbx callers.
// These exact file-local lints preserve explicit domain and binary contracts.
#![expect(
    clippy::module_name_repetitions,
    reason = "Tests verify these intentional explicit file-local contracts \
              remain safe."
)]

/// Animation capability available to one package.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AnimationCapability {
    /// No animation evidence is required for this package.
    NotPresent,
    /// Animation evidence exists but still needs target binding.
    PreservedOnly,
    /// Animation can be exported as a bound FBX clip.
    BoundClip,
}
