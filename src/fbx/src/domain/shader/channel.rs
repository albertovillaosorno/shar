// File:
//   - channel.rs
// Path:
//   - src/fbx/src/domain/shader/channel.rs
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
//   - Pure fbx domain rules for domain shader channel.
// - Must-Not:
//   - Read files, parse generated indexes, invoke CLI code, or call writer
//   - adapters.
// - Allows:
//   - Value objects, invariant checks, and pure evidence-to-domain translation.
// - Split-When:
//   - Split when channel contains two independently testable contracts.
// - Merge-When:
//   - Another fbx module owns the same domain boundary with no distinct
//   - invariant.
// - Summary:
//   - Material channel that the FBX export plan can reason about.
// - Description:
//   - Defines channel data and behavior for fbx domain shader.
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

//! Material channel that the FBX export plan can reason about.
// These exact file-local lints preserve explicit domain and binary contracts.
#![expect(
    clippy::module_name_repetitions,
    reason = "Tests verify these intentional explicit file-local contracts \
              remain safe."
)]

/// Supported shader channel exposed to material translation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MaterialChannel {
    /// Base color or diffuse texture channel.
    Diffuse,
    /// Alpha or transparency channel.
    Alpha,
    /// Emissive/light-map style channel.
    Emissive,
    /// Unknown channel preserved for reporting instead of being discarded.
    Preserved,
}
