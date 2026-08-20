// Copyright:
//   - Copyright © 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT
//
// Boundary-Contract:
// - Owns:
//   - Domain domain module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Domain domain module.
// - Description:
//   - Implements the declared domain module responsibility for fbx.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Domain domain module.

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
