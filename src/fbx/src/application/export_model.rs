// File:
//   - export_model.rs
// Path:
//   - src/fbx/src/application/export_model.rs
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
//   - fbx use-case orchestration for application export model.
// - Must-Not:
//   - Depend on driven adapters, parse local routes, or encode writer-specific
//   - syntax.
// - Allows:
//   - Use-case orchestration, planning, reporting, and calls through declared
//   - ports.
// - Split-When:
//   - Split when export model contains two independently testable contracts.
// - Merge-When:
//   - Another fbx module owns the same application boundary with no distinct
//   - invariant.
// - Summary:
//   - fbx application behavior for application export model.
// - Description:
//   - Defines export model data and behavior for fbx application.
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

//! This application module coordinates application export model without
//! choosing adapters.
