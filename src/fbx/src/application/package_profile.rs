// File:
//   - package_profile.rs
// Path:
//   - src/fbx/src/application/package_profile.rs
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
//   - fbx use-case orchestration for application package profile.
// - Must-Not:
//   - Depend on driven adapters, parse local routes, or encode writer-specific
//   - syntax.
// - Allows:
//   - Use-case orchestration, planning, reporting, and calls through declared
//   - ports.
// - Split-When:
//   - Split when package profile contains two independently testable
//   - contracts.
// - Merge-When:
//   - Another fbx module owns the same application boundary with no distinct
//   - invariant.
// - Summary:
//   - Package family selected by the phase-three package-index adapter.
// - Description:
//   - Defines package profile data and behavior for fbx application.
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

//! Package family selected by the phase-three package-index adapter.
pub mod character;
pub mod prop;
pub mod terrain;
pub mod vehicle;

/// Package family selected by the phase-three package-index adapter.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ModelPackageFamily {
    /// Static or animated prop package.
    Prop,
    /// Vehicle model package.
    Vehicle,
    /// Character or costume package.
    Character,
    /// Terrain or world-piece package represented as mesh geometry.
    Terrain,
}
