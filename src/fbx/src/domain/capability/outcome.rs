// File:
//   - outcome.rs
// Path:
//   - src/fbx/src/domain/capability/outcome.rs
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
//   - Pure fbx domain rules for domain capability outcome.
// - Must-Not:
//   - Read files, parse generated indexes, invoke CLI code, or call writer
//   - adapters.
// - Allows:
//   - Value objects, invariant checks, and pure evidence-to-domain translation.
// - Split-When:
//   - Split when outcome contains two independently testable contracts.
// - Merge-When:
//   - Another fbx module owns the same domain boundary with no distinct
//   - invariant.
// - Summary:
//   - Export outcome for each input concept in the package.
// - Description:
//   - Defines outcome data and behavior for fbx domain capability.
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

//! Export outcome for each input concept in the package.
// These exact file-local lints preserve explicit domain and binary contracts.
#![expect(
    clippy::module_name_repetitions,
    reason = "Tests verify these intentional explicit file-local contracts \
              remain safe."
)]

/// Outcome of evaluating one export capability.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CapabilityOutcome {
    /// Concept is converted into the FBX artifact.
    Converted,
    /// Concept is retained in a companion report because FBX cannot represent
    /// it.
    PreservedAsMetadata,
    /// Concept is known but intentionally deferred for a later capability pass.
    Deferred,
    /// Concept is required and must fail the package export.
    UnsupportedFailClosed,
}
