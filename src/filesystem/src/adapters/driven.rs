// File:
//   - driven.rs
// Path:
//   - src/filesystem/src/adapters/driven.rs
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
//   - Concrete outbound adapters implementing filesystem ports.
// - Must-Not:
//   - Own caller policy or expose unbounded utility behavior.
// - Allows:
//   - Expose the standard-library local provider.
// - Split-When:
//   - Split when another provider gains a distinct adapter family.
// - Merge-When:
//   - Another facade owns the same outbound implementations.
// - Summary:
//   - Filesystem driven-adapter facade.
// - Description:
//   - Keeps concrete storage mechanisms outside core layers.
// - Usage:
//   - Imported by driving composition and integration tests.
// - Defaults:
//   - The standard provider is stateless.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Driven adapters for shared filesystem ports.
//!
//! Concrete storage mechanisms remain replaceable.
mod io_context;
mod std_filesystem;

pub use std_filesystem::StdFilesystem;
