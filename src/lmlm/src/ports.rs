// File:
//   - ports.rs
// Path:
//   - src/lmlm/src/ports.rs
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
//   - LMLM outbound port declarations.
// - Must-Not:
//   - Implement storage behavior or command-line policy.
// - Allows:
//   - Traits isolating application commands from external mechanisms.
// - Split-When:
//   - Split when one port family becomes an independent context.
// - Merge-When:
//   - Another facade owns the same port declarations.
// - Summary:
//   - Hexagonal ports for LMLM extraction.
// - Description:
//   - Exposes replaceable archive source and entry sink boundaries.
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

//! Hexagonal ports for LMLM extraction workflows.
//!
//! Application code depends on source and sink contracts instead of concrete
//! filesystem operations.
mod archive_source;
mod entry_sink;

pub use archive_source::ArchiveSource;
pub use entry_sink::EntrySink;
