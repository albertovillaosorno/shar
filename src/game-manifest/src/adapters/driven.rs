// File:
//   - driven.rs
// Path:
//   - src/game-manifest/src/adapters/driven.rs
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
//   - Replaceable outbound adapters for tree evidence and manifest artifacts.
// - Must-Not:
//   - Parse CLI requests or change manifest semantics.
// - Allows:
//   - Filesystem traversal and complete text artifact storage behind ports.
// - Split-When:
//   - Split when another provider gains an independent adapter family.
// - Merge-When:
//   - Another facade owns the same outbound implementations.
// - Summary:
//   - Driven adapter facade for game manifests.
// - Description:
//   - Exposes concrete filesystem implementations of manifest ports.
// - Usage:
//   - Constructed by driving adapters and integration tests.
// - Defaults:
//   - No root or artifact path is selected implicitly.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Driven adapters implementing game-manifest outbound ports.
//!
//! Filesystem traversal and storage remain outside domain and application
//! layers.
mod filesystem_game_tree;
mod filesystem_text_store;

pub use filesystem_game_tree::FilesystemGameTree;
pub use filesystem_text_store::FilesystemTextStore;
