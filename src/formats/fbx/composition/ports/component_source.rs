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
//   - Component source outbound port.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Component source outbound port.
// - Description:
//   - Implements the declared outbound port responsibility for fbx.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Component source outbound port.

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
    fn load_mesh(&self, mesh_member_id: &str)
    -> Result<MeshAsset, Self::Error>;

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
