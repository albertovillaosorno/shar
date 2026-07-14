// File:
//   - driving.rs
// Path:
//   - src/p3d/src/adapters/driving.rs
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
//   - Inbound adapters translating operator requests into Pure3D use cases.
// - Must-Not:
//   - Decode chunks or publish package artifacts directly.
// - Allows:
//   - CLI decoding, dependency composition, batching, and result presentation.
// - Split-When:
//   - Split when another inbound protocol needs an independent adapter family.
// - Merge-When:
//   - Another facade owns the same inbound request contracts.
// - Summary:
//   - Driving adapter facade for Pure3D commands.
// - Description:
//   - Exposes single and batch CLI composition roots.
// - Usage:
//   - Called by thin executable entrypoints or adapter-level tests.
// - Defaults:
//   - Invalid requests fail with deterministic usage diagnostics.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Driving adapters for Pure3D operator requests.
//!
//! Process, batching, and presentation concerns stay outside the core layers.
pub mod batch_cli;
pub mod single_cli;
