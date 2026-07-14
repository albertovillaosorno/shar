// File:
//   - item.rs
// Path:
//   - src/fbx/src/domain/capability/item.rs
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
//   - Pure fbx domain rules for domain capability item.
// - Must-Not:
//   - Read files, parse generated indexes, invoke CLI code, or call writer
//   - adapters.
// - Allows:
//   - Value objects, invariant checks, and pure evidence-to-domain translation.
// - Split-When:
//   - Split when item contains two independently testable contracts.
// - Merge-When:
//   - Another fbx module owns the same domain boundary with no distinct
//   - invariant.
// - Summary:
//   - One explicit capability decision for package evidence.
// - Description:
//   - Defines item data and behavior for fbx domain capability.
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

//! One explicit capability decision for package evidence.
//!
//! This boundary keeps one explicit capability decision for package evidence
//! explicit and returns deterministic results to fbx callers.
// These exact file-local lints preserve explicit domain and binary contracts.
#![expect(
    clippy::module_name_repetitions,
    reason = "Tests verify these intentional explicit file-local contracts \
              remain safe."
)]

use super::outcome::CapabilityOutcome;

/// One explicit capability decision for package evidence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CapabilityReportItem {
    /// Stable evidence id or capability name.
    pub id: String,
    /// Outcome selected for the evidence.
    pub outcome: CapabilityOutcome,
    /// Deterministic review reason.
    pub reason: String,
}
