// File:
//   - ports.rs
// Path:
//   - src/game-manifest/src/ports.rs
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
//   - Game-manifest outbound port declarations.
// - Must-Not:
//   - Implement filesystem behavior or CLI policy.
// - Allows:
//   - Traits and DTOs isolating application commands from external mechanisms.
// - Split-When:
//   - Split when one port family becomes an independent context.
// - Merge-When:
//   - Another facade owns the same port declarations.
// - Summary:
//   - Hexagonal ports for manifest workflows.
// - Description:
//   - Exposes replaceable tree evidence and text artifact boundaries.
// - Usage:
//   - Implemented by driven adapters and supplied by driving adapters.
// - Defaults:
//   - Ports infer no roots or artifact paths.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Hexagonal ports for game-manifest workflows.
//!
//! Application commands depend on these contracts instead of concrete
//! filesystems.
mod game_tree;
mod text_artifact_store;

pub use game_tree::{GameTree, PathKind};
pub use text_artifact_store::TextArtifactStore;
