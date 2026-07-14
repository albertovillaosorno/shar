// File:
//   - geometry.rs
// Path:
//   - src/fbx/src/domain/geometry/geometry.rs
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
//   - Pure fbx domain rules for domain geometry geometry.
// - Must-Not:
//   - Read files, parse generated indexes, invoke CLI code, or call writer
//   - adapters.
// - Allows:
//   - Value objects, invariant checks, and pure evidence-to-domain translation.
// - Split-When:
//   - Split when geometry contains two independently testable contracts.
// - Merge-When:
//   - Another fbx module owns the same domain boundary with no distinct
//   - invariant.
// - Summary:
//   - Defines bounds for this module boundary.
// - Description:
//   - Defines geometry data and behavior for fbx domain geometry.
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

//! This code exposes bounds as pure domain behavior for domain geometry
//! geometry.
pub mod bounds;
pub mod payload;
pub mod polygon;

// Explicit facade names keep downstream imports unambiguous across domain
// modules.
#[expect(
    clippy::module_name_repetitions,
    reason = "Tests verify these explicit facade names preserve stable domain \
              imports."
)]
pub use bounds::GeometryBounds;
pub use payload::Geometry;
pub use polygon::Polygon;
