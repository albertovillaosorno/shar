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
//   - Definition domain module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Definition domain module.
// - Description:
//   - Implements the declared domain module responsibility for fbx.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Definition domain module.

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
