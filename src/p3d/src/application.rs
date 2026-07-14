// File:
//   - application.rs
// Path:
//   - src/p3d/src/application.rs
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
//   - Pure3D application use cases.
// - Must-Not:
//   - Import driven adapters or implement decoder and storage mechanics.
// - Allows:
//   - Coordinate domain behavior through explicit ports.
// - Split-When:
//   - Split when a use-case family becomes independently deployable.
// - Merge-When:
//   - Another facade owns the same application commands.
// - Summary:
//   - Application facade for Pure3D export.
// - Description:
//   - Exposes package-level commands without selecting concrete adapters.
// - Usage:
//   - Called by driving adapters and library clients.
// - Defaults:
//   - No concrete provider is selected.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Pure3D application use cases.
//!
//! Commands depend on ports and leave decoding and IO to driven adapters.
mod export_batch;
mod export_package;

pub use export_batch::ExportPackageBatch;
pub use export_package::ExportPackage;
