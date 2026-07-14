// File:
//   - local_original.rs
// Path:
//   - src/rmv/src/domain/local_original.rs
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
//   - Pure rmv domain rules for domain local original.
// - Must-Not:
//   - Read files, parse generated indexes, invoke CLI code, or call writer
//   - adapters.
// - Allows:
//   - Value objects, invariant checks, and pure evidence-to-domain translation.
// - Split-When:
//   - Split when local original contains two independently testable contracts.
// - Merge-When:
//   - Another rmv module owns the same domain boundary with no distinct
//   - invariant.
// - Summary:
//   - Runtime completion is represented by `runtime_source` so the movie
//   - pipeline.
// - Description:
//   - Defines local original data and behavior for rmv domain.
// - Usage:
//   - Imported through crate domain facades or sibling domain modules.
// - Defaults:
//   - No filesystem paths, no external process calls, and no implicit IO
//   - defaults.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Historical local-original terminology intentionally has no separate domain
//! model.
//!
//! Runtime completion is represented by `runtime_source` so the movie pipeline
//! has one validation policy for user-supplied movie bytes instead of two
//! drifting copies of the same prerequisite checks.
