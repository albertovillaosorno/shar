// File:
//   - cli.rs
// Path:
//   - src/fbx/src/adapters/driving/cli.rs
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
//   - The fbx adapter boundary for adapters driving cli.
// - Must-Not:
//   - Change domain semantics or bypass application and port contracts.
// - Allows:
//   - Filesystem, JSON, CLI, Blender, or serialization work behind explicit
//   - ports.
// - Split-When:
//   - Split when cli contains two independently testable contracts.
// - Merge-When:
//   - Another fbx module owns the same adapters boundary with no distinct
//   - invariant.
// - Summary:
//   - CLI export request after string parsing but before application execution.
// - Description:
//   - Defines cli data and behavior for fbx adapters driving.
// - Usage:
//   - Constructed by composition roots or tests and passed behind port traits.
// - Defaults:
//   - Adapter defaults stay local to the adapter constructor or config.
//
// ADRs:
// - docs/adr/pipeline/fbx/hexagonal-scene-export.md
// - docs/adr/pipeline/unreal/unreal-manifest-and-package-taxonomy.md
//
// Large file:
//   - false
//

//! CLI export request after string parsing but before application execution.
// These exact file-local lints preserve explicit domain and binary contracts.
#![expect(
    clippy::module_name_repetitions,
    reason = "Tests verify these intentional explicit file-local contracts \
              remain safe."
)]

use std::path::PathBuf;

/// CLI export selection validation failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CliExportSelectionError {
    /// Package selector was empty or whitespace-only.
    MissingPackageSelector,
    /// Package selector carried surrounding whitespace.
    NonCanonicalPackageSelector,
    /// Output file path was empty.
    MissingOutputFile,
    /// Output file path carried surrounding whitespace.
    NonCanonicalOutputFile,
}

/// CLI export request after string parsing but before application execution.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CliExportSelection {
    /// Stable package selector or package id provided by the caller.
    pub package_selector: String,
    /// Output file selected by the caller.
    pub output_file: PathBuf,
}

impl CliExportSelection {
    /// Build a CLI export selection without interpreting package semantics.
    ///
    /// # Errors
    ///
    /// Returns an error when a required request field is empty.
    pub fn new(
        package_selector: impl Into<String>,
        output_file: impl Into<PathBuf>,
    ) -> Result<Self, CliExportSelectionError> {
        let normalized_selector = package_selector.into();
        if normalized_selector
            .trim()
            .is_empty()
        {
            return Err(CliExportSelectionError::MissingPackageSelector);
        }
        if normalized_selector != normalized_selector.trim()
            || normalized_selector
                .chars()
                .any(char::is_control)
        {
            return Err(CliExportSelectionError::NonCanonicalPackageSelector);
        }
        let normalized_output_file = output_file.into();
        if normalized_output_file
            .as_os_str()
            .is_empty()
        {
            return Err(CliExportSelectionError::MissingOutputFile);
        }
        if normalized_output_file
            .to_str()
            .is_some_and(
                |path| {
                    path != path.trim()
                        || path
                            .chars()
                            .any(char::is_control)
                },
            )
        {
            return Err(CliExportSelectionError::NonCanonicalOutputFile);
        }
        Ok(
            Self {
                package_selector: normalized_selector,
                output_file: normalized_output_file,
            },
        )
    }
}
