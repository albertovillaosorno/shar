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
//   - Canonical repository-relative generated workspace locations.
// - Must-Not:
//   - Define portable artifact identities stored inside manifests.
// - Allows:
//   - Shared physical defaults for pipeline adapters and CLI composition.
// - Split-When:
//   - One generated workspace gains an independent lifecycle contract.
// - Merge-When:
//   - Another composition module owns the identical physical defaults.
// - Summary:
//   - Canonical generated workspace paths.
// - Description:
//   - Keeps regenerable pipeline output below one ignored cache hierarchy while
//     logical manifest identities remain stable.
// - Usage:
//   - Used by command defaults and local generated-output adapters.
// - Defaults:
//   - Generated pipeline output lives below `.cache/pipeline`.
//

//! Canonical repository-relative generated workspace paths.

/// Default physical extraction workspace.
pub(crate) const EXTRACTED_WORKSPACE_ROOT: &str = ".cache/pipeline/extracted";
/// Default physical complete FBX catalog workspace.
pub(crate) const FBX_WORKSPACE_ROOT: &str = ".cache/pipeline/fbx-assets";
/// Default physical Unreal staging workspace.
pub(crate) const UNREAL_STAGING_WORKSPACE_ROOT: &str =
    ".cache/pipeline/unreal-staging";
