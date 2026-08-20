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
//   - RMV SHA-256 composition binding.
// - Must-Not:
//   - Alter SHA-256 semantics or expose storage policy.
// - Allows:
//   - Bind the RMV digest value to the repository SHA-256 implementation.
// - Split-When:
//   - Split when another digest provider gains an independent lifecycle.
// - Merge-When:
//   - Merge when another composition module owns the same binding.
// - Summary:
//   - RMV SHA-256 composition binding.
// - Description:
//   - Preserves the public `Sha256::digest` constructor outside pure domain.
// - Usage:
//   - Used by RMV adapters, applications, and tests through the public type.
// - Defaults:
//   - Exact input bytes are hashed without normalization.
//

//! RMV SHA-256 composition binding.

use crate::domain::Sha256;

impl Sha256 {
    /// Hash exact movie bytes.
    #[must_use]
    pub fn digest(data: &[u8]) -> Self {
        Self(shar_sha256::digest(data))
    }
}
