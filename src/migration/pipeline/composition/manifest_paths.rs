// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT
//
// Boundary-Contract:
// - Owns:
//   - Canonical repository-relative generated manifest locations.
// - Must-Not:
//   - Define artifact workspace roots or portable identities stored in plans.
// - Allows:
//   - Shared physical manifest defaults for pipeline composition.
// - Split-When:
//   - One manifest gains an independent storage or publication lifecycle.
// - Merge-When:
//   - Another composition module owns the identical physical manifest paths.
// - Summary:
//   - Canonical generated manifest paths.
// - Description:
//   - Centralizes generated pipeline ledgers beneath `game/manifest` without
//     changing logical artifact identities.
// - Usage:
//   - Used by publishers and command defaults.
// - Defaults:
//   - Generated global manifests live beneath `game/manifest`.
//

//! Canonical repository-relative generated manifest paths.

/// Canonical complete FBX catalog manifest.
pub(crate) const FBX_MANIFEST_PATH: &str = "game/manifest/fbx.jsonl";
