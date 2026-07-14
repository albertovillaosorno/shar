// File:
//   - translator.rs
// Path:
//   - src/fbx/src/domain/skeleton/translator.rs
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
//   - Pure fbx domain rules for domain skeleton translator.
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
//   - Return an empty skeleton until bone evidence is resolved by adapters.
// - Description:
//   - Defines translator data and behavior for fbx domain skeleton.
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

//! Return an empty skeleton until bone evidence is resolved by adapters.
// These exact file-local lints preserve explicit domain and binary contracts.
#![expect(
    clippy::missing_const_for_fn,
    reason = "Tests verify these intentional explicit file-local contracts \
              remain safe."
)]

use super::bone::Bone;

/// Return an empty skeleton until bone evidence is resolved by adapters.
#[must_use]
pub fn unresolved_skeleton() -> Vec<Bone> {
    Vec::new()
}
