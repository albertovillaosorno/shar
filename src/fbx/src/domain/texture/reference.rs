// File:
//   - reference.rs
// Path:
//   - src/fbx/src/domain/texture/reference.rs
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
//   - Pure fbx domain rules for domain texture reference.
// - Must-Not:
//   - Read files, parse generated indexes, invoke CLI code, or call writer
//   - adapters.
// - Allows:
//   - Value objects, invariant checks, and pure evidence-to-domain translation.
// - Split-When:
//   - Split when reference contains two independently testable contracts.
// - Merge-When:
//   - Another fbx module owns the same domain boundary with no distinct
//   - invariant.
// - Summary:
//   - Texture artifact referenced by a scene material.
// - Description:
//   - Defines reference data and behavior for fbx domain texture.
// - Usage:
//   - Imported through crate domain facades or sibling domain modules.
// - Defaults:
//   - No filesystem paths, no external process calls, and no implicit IO
//   - defaults.
//
// ADRs:
// - docs/adr/pipeline/fbx/hexagonal-scene-export.md
// - docs/adr/pipeline/unreal/unreal-manifest-and-package-taxonomy.md
//
// Large file:
//   - false
//

//! Texture artifact referenced by a scene material.
//!
//! This boundary keeps texture artifact referenced by a scene material
//! explicit and returns deterministic results to fbx callers.
// These exact file-local lints preserve explicit domain and binary contracts.
#![expect(
    clippy::module_name_repetitions,
    reason = "Tests verify these intentional explicit file-local contracts \
              remain safe."
)]

/// Stable reference to a decoded texture artifact.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TextureReference {
    /// Stable texture id from package evidence.
    pub id: String,
    /// Human-readable label retained for reports.
    pub label: String,
}

/// Convert one stable texture id into a domain texture reference.
#[must_use]
pub fn texture_reference(
    id: impl Into<String>,
    label: impl Into<String>,
) -> TextureReference {
    TextureReference {
        id: id.into(),
        label: label.into(),
    }
}
