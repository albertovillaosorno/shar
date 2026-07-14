// File:
//   - ports.rs
// Path:
//   - src/rsd/src/ports.rs
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
//   - RSD outbound port declarations.
// - Must-Not:
//   - Implement IO or command-line policy.
// - Allows:
//   - Traits isolating use cases from external mechanisms.
// - Split-When:
//   - Split when one port family becomes an independent context.
// - Merge-When:
//   - Another facade owns the same port declarations.
// - Summary:
//   - Hexagonal ports for RSD export workflows.
// - Description:
//   - Exposes replaceable boundaries used by application commands.
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

//! Hexagonal ports for RSD export workflows.
//!
//! Application code depends on these contracts rather than concrete
//! filesystems.
mod exporter;

pub use exporter::Exporter;
