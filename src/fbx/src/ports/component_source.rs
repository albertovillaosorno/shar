// File:
//   - component_source.rs
// Path:
//   - src/fbx/src/ports/component_source.rs
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
//   - The fbx port contract for ports component source.
// - Must-Not:
//   - Contain concrete filesystem, JSON, Blender, or serialization
//   - implementations.
// - Allows:
//   - Trait and DTO definitions that keep adapters replaceable.
// - Split-When:
//   - Split when component source contains two independently testable
//   - contracts.
// - Merge-When:
//   - Another fbx module owns the same ports boundary with no distinct
//   - invariant.
// - Summary:
//   - Source for normalized model package components.
// - Description:
//   - Defines component source data and behavior for fbx ports.
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

//! Port for loading validated mesh and material components.
//!
//! Export use cases depend on this trait so adapters can supply package data
//! without leaking filesystem or serialization details into the `fbx` domain.
use crate::domain::mesh::MeshAsset;
use crate::domain::texture::MaterialBinding;

/// Source for normalized model package components.
pub trait ComponentSource {
    /// Stable source error type selected by the adapter.
    type Error;

    /// Load the primary mesh for an export package.
    ///
    /// # Errors
    ///
    /// Returns adapter-specific errors when the component source cannot provide
    /// a valid mesh for the request.
    fn load_mesh(
        &self,
        mesh_member_id: &str,
    ) -> Result<MeshAsset, Self::Error>;

    /// Resolve the material binding for one shader or material member.
    ///
    /// # Errors
    ///
    /// Returns adapter-specific errors when material evidence is missing or
    /// cannot be normalized safely.
    fn resolve_material(
        &self,
        shader_id: &str,
    ) -> Result<MaterialBinding, Self::Error>;
}
