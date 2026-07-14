// File:
//   - mod.rs
// Path:
//   - src/p3d/src/decompiler/mod.rs
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
//   - p3d module behavior for decompiler mod.
// - Must-Not:
//   - Violate repository architecture, path, provenance, or output rules.
// - Allows:
//   - Operations required to validate and execute decompiler module facade.
// - Split-When:
//   - Split when decompiler module facade contains two independently testable
//   - contracts.
// - Merge-When:
//   - Another p3d module owns the same module boundary with no distinct
//   - invariant.
// - Summary:
//   - Production recovery lives in `adapters::driven::decoders`; this module
//   - stays.
// - Description:
//   - Defines decompiler module facade data and behavior for p3d decompiler.
// - Usage:
//   - Used by p3d decompiler code that needs decompiler module facade.
// - Defaults:
//   - No implicit output outside the repository is allowed.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Decoder compatibility facade for legacy decompiler call sites.
//!
//! Production recovery remains in `adapters::driven::decoders`, while this
//! facade preserves deterministic legacy imports for `p3d` callers.
