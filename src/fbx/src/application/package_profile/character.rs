// File:
//   - character.rs
// Path:
//   - src/fbx/src/application/package_profile/character.rs
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
//   - fbx use-case orchestration for application package profile character.
// - Must-Not:
//   - Depend on driven adapters, parse local routes, or encode writer-specific
//   - syntax.
// - Allows:
//   - Use-case orchestration, planning, reporting, and calls through declared
//   - ports.
// - Split-When:
//   - Split when character contains two independently testable contracts.
// - Merge-When:
//   - Another fbx module owns the same application boundary with no distinct
//   - invariant.
// - Summary:
//   - Character package export requirements.
// - Description:
//   - Defines character data and behavior for fbx application package profile.
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

//! Character package export requirements.
//!
//! This boundary keeps character package export requirements explicit and
//! returns deterministic results to fbx callers.
// These exact file-local lints preserve explicit domain and binary contracts.
#![expect(
    clippy::module_name_repetitions,
    reason = "Tests verify these intentional explicit file-local contracts \
              remain safe."
)]

/// Character package export profile.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CharacterProfile {
    /// Characters require visible model geometry.
    pub requires_mesh: bool,
    /// Characters may require skeleton and skin capability checks.
    pub may_require_rig: bool,
}
