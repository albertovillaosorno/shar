// File:
//   - application.rs
// Path:
//   - src/fbx/src/application.rs
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
//   - fbx module behavior for application.
// - Must-Not:
//   - Violate repository architecture, path, provenance, or output rules.
// - Allows:
//   - Operations required to validate and execute application.
// - Split-When:
//   - Split when application contains two independently testable contracts.
// - Merge-When:
//   - Another fbx module owns the same module boundary with no distinct
//   - invariant.
// - Summary:
//   - Defines build_scene for this module boundary.
// - Description:
//   - Defines application data and behavior for fbx root.
// - Usage:
//   - Used by fbx root code that needs application.
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

//! Defines build_scene for this module boundary.
//!
//! This boundary keeps defines build_scene for this module boundary explicit
//! and returns deterministic results to fbx callers.
pub mod build_scene;
pub mod export_model;
pub mod package_profile;
pub mod planning;
pub mod report;
