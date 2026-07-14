// File:
//   - decoders.rs
// Path:
//   - src/p3d/src/adapters/driven/decoders.rs
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
//   - The p3d adapter boundary for adapters driven decoders.
// - Must-Not:
//   - Change domain semantics or bypass application and port contracts.
// - Allows:
//   - Filesystem, JSON, CLI, Blender, or serialization work behind explicit
//   - ports.
// - Split-When:
//   - Split when decoders contains two independently testable contracts.
// - Merge-When:
//   - Another p3d module owns the same adapters boundary with no distinct
//   - invariant.
// - Summary:
//   - Per-kind `Pure3D` chunk decoders.
// - Description:
//   - Defines decoders data and behavior for p3d adapters driven.
// - Usage:
//   - Constructed by composition roots or tests and passed behind port traits.
// - Defaults:
//   - Adapter defaults stay local to the adapter constructor or config.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Per-kind `Pure3D` chunk decoders.
//!
//! This boundary keeps per-kind `pure3d` chunk decoders explicit and returns
//! deterministic results to p3d callers.
pub mod collision;
pub mod intersect;
pub mod locator;
pub mod mesh;
pub mod reader;
pub mod rig;
pub mod scene;
