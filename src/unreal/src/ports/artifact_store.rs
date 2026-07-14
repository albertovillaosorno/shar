// File:
//   - artifact_store.rs
// Path:
//   - src/unreal/src/ports/artifact_store.rs
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
//   - Persistence of deterministic conversion plans and reports.
// - Must-Not:
//   - Contain filesystem, MCP, Unreal Editor, serialization, or process
//   - implementations.
// - Allows:
//   - A replaceable text-artifact storage contract for pipeline adapters.
// - Split-When:
//   - Binary conversion artifacts require a distinct storage contract.
// - Merge-When:
//   - Another port owns the same conversion-plan persistence boundary.
// - Summary:
//   - Conversion artifact storage port.
// - Description:
//   - Defines the persistence seam for public-safe plans and reports.
// - Usage:
//   - Implemented by pipeline-owned adapters outside this crate.
// - Defaults:
//   - No filesystem or storage implementation is selected implicitly.
//
// ADRs:
// - docs/adr/unreal/architecture.md
//
// Large file:
//   - false
//

//! Persistence port for deterministic Unreal conversion artifacts.
//!
//! The port contains no filesystem or serialization implementation.

/// Text artifact storage used by conversion orchestration.
pub trait PlanStore {
    /// Read one UTF-8 conversion artifact.
    ///
    /// # Errors
    ///
    /// Returns an error when the adapter cannot read or decode the artifact.
    fn read_text(
        &self,
        path: &str,
    ) -> Result<String, String>;

    /// Write one UTF-8 conversion artifact.
    ///
    /// # Errors
    ///
    /// Returns an error when the adapter cannot persist the artifact.
    fn write_text(
        &self,
        path: &str,
        text: &str,
    ) -> Result<(), String>;

    /// Return true when the conversion artifact exists.
    fn exists(
        &self,
        path: &str,
    ) -> bool;
}
