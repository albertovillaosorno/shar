// File:
//   - blender_scene_writer.rs
// Path:
//   - src/fbx/src/adapters/driven/blender_scene_writer.rs
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
//   - The fbx adapter boundary for adapters driven blender scene writer.
// - Must-Not:
//   - Change domain semantics or bypass application and port contracts.
// - Allows:
//   - Filesystem, JSON, CLI, Blender, or serialization work behind explicit
//   - ports.
// - Split-When:
//   - Split when blender scene writer contains two independently testable
//   - contracts.
// - Merge-When:
//   - Another fbx module owns the same adapters boundary with no distinct
//   - invariant.
// - Summary:
//   - Blender invocation intent selected by an adapter configuration.
// - Description:
//   - Defines blender scene writer data and behavior for fbx adapters driven.
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

//! Blender invocation intent selected by an adapter configuration.
/// Blender command-plan validation failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BlenderCommandPlanError {
    /// Blender executable identity was empty or whitespace-only.
    MissingExecutable,
    /// Blender executable identity carried surrounding whitespace.
    NonCanonicalExecutable,
    /// Blender script identity was empty or whitespace-only.
    MissingScript,
    /// Blender script identity carried surrounding whitespace.
    NonCanonicalScript,
    /// Output file identity was empty or whitespace-only.
    MissingOutputFile,
    /// Output file identity carried surrounding whitespace.
    NonCanonicalOutputFile,
}

/// Blender-backed scene writer adapter.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BlenderCommandPlan {
    /// Caller-supplied executable path or command name.
    pub executable: String,
    /// Script or module argument used by the adapter.
    pub script: String,
    /// Output file requested by the export operation.
    pub output_file: String,
}

impl BlenderCommandPlan {
    /// Build a command plan without assuming any local installation path.
    ///
    /// # Errors
    ///
    /// Returns an error when a required command field is blank.
    pub fn new(
        executable: impl Into<String>,
        script: impl Into<String>,
        output_file: impl Into<String>,
    ) -> Result<Self, BlenderCommandPlanError> {
        let normalized_executable = executable.into();
        if normalized_executable
            .trim()
            .is_empty()
        {
            return Err(BlenderCommandPlanError::MissingExecutable);
        }
        if normalized_executable != normalized_executable.trim()
            || normalized_executable
                .chars()
                .any(char::is_control)
        {
            return Err(BlenderCommandPlanError::NonCanonicalExecutable);
        }
        let normalized_script = script.into();
        if normalized_script
            .trim()
            .is_empty()
        {
            return Err(BlenderCommandPlanError::MissingScript);
        }
        if normalized_script != normalized_script.trim()
            || normalized_script
                .chars()
                .any(char::is_control)
        {
            return Err(BlenderCommandPlanError::NonCanonicalScript);
        }
        let normalized_output_file = output_file.into();
        if normalized_output_file
            .trim()
            .is_empty()
        {
            return Err(BlenderCommandPlanError::MissingOutputFile);
        }
        if normalized_output_file != normalized_output_file.trim()
            || normalized_output_file
                .chars()
                .any(char::is_control)
        {
            return Err(BlenderCommandPlanError::NonCanonicalOutputFile);
        }
        Ok(
            Self {
                executable: normalized_executable,
                script: normalized_script,
                output_file: normalized_output_file,
            },
        )
    }
}
