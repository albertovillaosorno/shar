// File:
//   - driving.rs
// Path:
//   - src/fbx/src/adapters/driving.rs
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
//   - The fbx adapter boundary for adapters driving.
// - Must-Not:
//   - Change domain semantics or bypass application and port contracts.
// - Allows:
//   - Filesystem, JSON, CLI, Blender, or serialization work behind explicit
//   - ports.
// - Split-When:
//   - Split when driving contains two independently testable contracts.
// - Merge-When:
//   - Another fbx module owns the same adapters boundary with no distinct
//   - invariant.
// - Summary:
//   - Command-line adapter boundary.
// - Description:
//   - Defines driving data and behavior for fbx adapters.
// - Usage:
//   - Constructed by composition roots or tests and passed behind port traits.
// - Defaults:
//   - Adapter defaults stay local to the adapter constructor or config.
//
// ADRs:
// - docs/adr/pipeline/fbx/hexagonal-scene-export.md
// - docs/adr/pipeline/unreal/unreal-manifest-and-package-taxonomy.md
//
// Large file:
//   - false
//

//! Command-line adapter boundary.
//!
//! This boundary keeps command-line adapter boundary explicit and returns
//! deterministic results to fbx callers.
pub mod cli;
