// File:
//   - artifact_sink.rs
// Path:
//   - src/fbx/src/ports/artifact_sink.rs
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
//   - The fbx port contract for ports artifact sink.
// - Must-Not:
//   - Contain concrete filesystem, JSON, Blender, or serialization
//   - implementations.
// - Allows:
//   - Trait and DTO definitions that keep adapters replaceable.
// - Split-When:
//   - Split when artifact sink contains two independently testable contracts.
// - Merge-When:
//   - Another fbx module owns the same ports boundary with no distinct
//   - invariant.
// - Summary:
//   - fbx ports behavior for ports artifact sink.
// - Description:
//   - Defines artifact sink data and behavior for fbx ports.
// - Usage:
//   - Implemented by adapters and consumed by application use cases.
// - Defaults:
//   - No default implementation is provided by the port contract.
//
// ADRs:
// - docs/adr/pipeline/fbx/hexagonal-scene-export.md
// - docs/adr/pipeline/unreal/unreal-manifest-and-package-taxonomy.md
//
// Large file:
//   - false
//

//! This port module exposes ports artifact sink contracts without
//! implementations.
pub use crate::ports::scene_writer::{
    SceneArtifactReceipt as ArtifactReceipt,
    SceneArtifactTarget as ArtifactTarget,
};
