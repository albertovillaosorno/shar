// File:
//   - terrain.rs
// Path:
//   - src/fbx/src/application/package_profile/terrain.rs
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
//   - fbx use-case orchestration for application package profile terrain.
// - Must-Not:
//   - Depend on driven adapters, parse local routes, or encode writer-specific
//   - syntax.
// - Allows:
//   - Use-case orchestration, planning, reporting, and calls through declared
//   - ports.
// - Split-When:
//   - Split when terrain contains two independently testable contracts.
// - Merge-When:
//   - Another fbx module owns the same application boundary with no distinct
//   - invariant.
// - Summary:
//   - Terrain package export requirements.
// - Description:
//   - Defines terrain data and behavior for fbx application package profile.
// - Usage:
//   - Called by entrypoints after ports and adapters are selected by the
//   - caller.
// - Defaults:
//   - No concrete adapter is selected unless the caller supplies one through a
//   - port.
//
// ADRs:
// - docs/adr/pipeline/fbx/hexagonal-scene-export.md
// - docs/adr/pipeline/unreal/unreal-manifest-and-package-taxonomy.md
//
// Large file:
//   - false
//

//! Terrain package export requirements.
//!
//! This boundary keeps terrain package export requirements explicit and
//! returns deterministic results to fbx callers.
// These exact file-local lints preserve explicit domain and binary contracts.
#![expect(
    clippy::module_name_repetitions,
    reason = "Tests verify these intentional explicit file-local contracts \
              remain safe."
)]

/// Terrain package export profile.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TerrainProfile {
    /// Terrain packages export mesh geometry through the same scene engine.
    pub requires_mesh: bool,
    /// World streaming and consolidation remain outside FBX.
    pub preserves_streaming_sidecar: bool,
}
