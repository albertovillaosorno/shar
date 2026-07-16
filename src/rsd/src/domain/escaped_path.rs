// File:
//   - escaped_path.rs
// Path:
//   - src/rsd/src/domain/escaped_path.rs
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
//   - The RSD-local compatibility name for diagnostic path rendering.
// - Must-Not:
//   - Reimplement escaping, inspect storage, or normalize path identity.
// - Allows:
//   - Re-export the shared filesystem domain renderer within the RSD crate.
// - Split-When:
//   - RSD requires a genuinely distinct diagnostic rendering grammar.
// - Merge-When:
//   - RSD no longer needs a compatibility name for existing error code.
// - Summary:
//   - Shared diagnostic path compatibility alias.
// - Description:
//   - Keeps RSD error call sites stable while the filesystem crate owns the
//   - platform-aware, reversible rendering mechanism.
// - Usage:
//   - Imported by the RSD domain facade as `EscapedPath`.
// - Defaults:
//   - Rendering behavior is exactly the shared filesystem contract.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! RSD-local compatibility name for shared diagnostic path rendering.

#[expect(
    clippy::redundant_pub_crate,
    reason = "Widening this compatibility alias would expose an internal \
              detail."
)]
pub(crate) use schoenwald_filesystem::DiagnosticPath as EscapedPath;
