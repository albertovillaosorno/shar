// File:
//   - driven.rs
// Path:
//   - src/pipeline/src/adapters/driven.rs
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
//   - Concrete outbound adapters for pipeline application ports.
// - Must-Not:
//   - Parse process commands or own domain invariants.
// - Allows:
//   - Expose local filesystem evidence providers.
// - Split-When:
//   - Split when one provider family becomes independently versioned.
// - Merge-When:
//   - Another facade owns the same driven adapters.
// - Summary:
//   - Pipeline driven-adapter facade.
// - Description:
//   - Keeps concrete provider mechanisms outside pipeline core layers.
// - Usage:
//   - Imported by driving composition and integration tests.
// - Defaults:
//   - Providers are explicit and stateless where possible.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Concrete outbound adapters for pipeline application ports.
//!
//! Shared mechanisms are composed into pipeline-specific providers here.
mod local;
mod output_inventory;

pub use local::LocalPipeline;
pub(in crate::adapters) use local::{ProgressVerbosity, install_progress};
pub use output_inventory::FilesystemOutputInventory;
