// File:
//   - domain.rs
// Path:
//   - src/fbx/src/domain/domain.rs
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
//   - Pure fbx domain rules for domain domain.
// - Must-Not:
//   - Read files, parse generated indexes, invoke CLI code, or call writer
//   - adapters.
// - Allows:
//   - Value objects, invariant checks, and pure evidence-to-domain translation.
// - Split-When:
//   - Split when domain contains two independently testable contracts.
// - Merge-When:
//   - Another fbx module owns the same domain boundary with no distinct
//   - invariant.
// - Summary:
//   - Defines animation for this module boundary.
// - Description:
//   - Defines domain data and behavior for fbx domain.
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

//! This code exposes animation as pure domain behavior for domain domain.
#[path = "animation/animation.rs"]
pub mod animation;
#[path = "camera/camera.rs"]
pub mod camera;
#[path = "capability/capability.rs"]
pub mod capability;
#[path = "character/character.rs"]
pub mod character;
#[path = "coordinate/coordinate.rs"]
pub mod coordinate;
#[path = "geometry/geometry.rs"]
pub mod geometry;
#[path = "material/material.rs"]
pub mod material;
#[path = "mesh/mesh.rs"]
pub mod mesh;
#[path = "scene/scene.rs"]
pub mod scene;
#[path = "shader/shader.rs"]
pub mod shader;
#[path = "skeleton/skeleton.rs"]
pub mod skeleton;
#[path = "skin/skin.rs"]
pub mod skin;
#[path = "surface/surface.rs"]
pub mod surface;
#[path = "texture/texture.rs"]
pub mod texture;
#[path = "timing/timing.rs"]
pub mod timing;
#[path = "transform/transform.rs"]
pub mod transform;
