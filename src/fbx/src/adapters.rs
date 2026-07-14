// File:
//   - adapters.rs
// Path:
//   - src/fbx/src/adapters.rs
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
//   - fbx module behavior for adapters.
// - Must-Not:
//   - Violate repository architecture, path, provenance, or output rules.
// - Allows:
//   - Operations required to validate and execute adapters.
// - Split-When:
//   - Split when adapters contains two independently testable contracts.
// - Merge-When:
//   - Another fbx module owns the same module boundary with no distinct
//   - invariant.
// - Summary:
//   - Driven adapters called through outbound ports.
// - Description:
//   - Defines adapters data and behavior for fbx root.
// - Usage:
//   - Used by fbx root code that needs adapters.
// - Defaults:
//   - No implicit output outside the repository is allowed.
//
// ADRs:
// - docs/adr/pipeline/fbx/hexagonal-scene-export.md
// - docs/adr/pipeline/unreal/unreal-manifest-and-package-taxonomy.md
//
// Large file:
//   - false
//

//! Driven adapters called through outbound ports.
//!
//! This boundary keeps driven adapters called through outbound ports explicit
//! and returns deterministic results to fbx callers.
pub mod driven;
/// Inbound adapters such as CLI request parsing.
pub mod driving;
