// File:
//   - character.rs
// Path:
//   - src/fbx/src/domain/character/character.rs
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
//   - Pure fbx domain rules for domain character character.
// - Must-Not:
//   - Read files, parse generated indexes, invoke CLI code, or call writer
//   - adapters.
// - Allows:
//   - Value objects, invariant checks, and pure evidence-to-domain translation.
// - Split-When:
//   - Split when character contains two independently testable contracts.
// - Merge-When:
//   - Another fbx module owns the same domain boundary with no distinct
//   - invariant.
// - Summary:
//   - Facade for the skinned character aggregate.
// - Description:
//   - Exposes the character asset aggregate for fbx domain character.
// - Usage:
//   - Imported through crate domain facades or sibling domain modules.
// - Defaults:
//   - No filesystem paths, no external process calls, and no implicit IO
//   - defaults.
//
// ADRs:
// - docs/adr/pipeline/fbx/hexagonal-scene-export.md
//
// Large file:
//   - false
//

//! This code exposes asset as pure domain behavior for domain character.
pub mod asset;

// Explicit facade names keep downstream imports unambiguous across domain
// modules.
#[expect(
    clippy::module_name_repetitions,
    reason = "Tests verify these explicit facade names preserve stable domain \
              imports."
)]
pub use asset::{CharacterAsset, CharacterError, SkinnedPart};
