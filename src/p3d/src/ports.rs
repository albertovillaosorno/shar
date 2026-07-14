// File:
//   - ports.rs
// Path:
//   - src/p3d/src/ports.rs
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
//   - Pure3D outbound port declarations.
// - Must-Not:
//   - Implement decoding, storage, or command-line policy.
// - Allows:
//   - Traits isolating application commands from external mechanisms.
// - Split-When:
//   - Split when one port family becomes an independent context.
// - Merge-When:
//   - Another facade owns the same port declarations.
// - Summary:
//   - Hexagonal ports for Pure3D workflows.
// - Description:
//   - Exposes replaceable package export boundaries.
// - Usage:
//   - Implemented by driven adapters and supplied by driving adapters.
// - Defaults:
//   - Ports infer no paths.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Hexagonal ports for Pure3D package workflows.
//!
//! Application code invokes these contracts rather than concrete extractors.
mod package_batch_exporter;
mod package_exporter;

pub use package_batch_exporter::PackageBatchExporter;
pub use package_exporter::PackageExporter;
