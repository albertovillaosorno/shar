// File:
//   - definition.rs
// Path:
//   - src/fbx/src/domain/material/definition.rs
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
//   - Pure fbx domain rules for domain material definition.
// - Must-Not:
//   - Read files, parse generated indexes, invoke CLI code, or call writer
//   - adapters.
// - Allows:
//   - Value objects, invariant checks, and pure evidence-to-domain translation.
// - Split-When:
//   - Split when definition contains two independently testable contracts.
// - Merge-When:
//   - Another fbx module owns the same domain boundary with no distinct
//   - invariant.
// - Summary:
//   - Material required by a normalized scene.
// - Description:
//   - Defines definition data and behavior for fbx domain material.
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

//! Material required by a normalized scene.
//!
//! This boundary keeps material required by a normalized scene explicit and
//! returns deterministic results to fbx callers.
use crate::domain::texture::TextureReference;

/// Material required by a normalized scene.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Material {
    /// Stable material id.
    pub id: String,
    /// Optional diffuse texture reference.
    pub diffuse_texture: Option<TextureReference>,
    /// Additional unsupported or deferred shader channels.
    pub preserved_channels: Vec<String>,
}
