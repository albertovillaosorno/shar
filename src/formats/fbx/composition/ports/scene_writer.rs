// Copyright:
//   - Copyright © 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT
//
// Boundary-Contract:
// - Owns:
//   - Scene writer outbound port.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Scene writer outbound port.
// - Description:
//   - Implements the declared outbound port responsibility for fbx.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Scene writer outbound port.

use crate::domain::scene::Scene;

/// Scene-artifact identity validation failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SceneArtifactError {
    /// Artifact target identity was empty or whitespace-only.
    MissingArtifactId,
    /// Artifact target identity carried surrounding whitespace.
    NonCanonicalArtifactId,
    /// Writer receipt location was empty or whitespace-only.
    MissingReceiptLocation,
    /// Writer receipt location carried surrounding whitespace.
    NonCanonicalReceiptLocation,
}

/// Semantic artifact target selected by an application use case.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SceneArtifactTarget {
    /// Stable artifact id, independent of local filesystem layout.
    pub artifact_id: String,
}

impl SceneArtifactTarget {
    /// Create an artifact target from a deterministic id.
    ///
    /// # Errors
    ///
    /// Returns an error when the artifact identity is blank.
    pub fn new(
        artifact_id: impl Into<String>,
    ) -> Result<Self, SceneArtifactError> {
        let normalized_artifact_id = artifact_id.into();
        if normalized_artifact_id.trim().is_empty() {
            return Err(SceneArtifactError::MissingArtifactId);
        }
        if normalized_artifact_id != normalized_artifact_id.trim()
            || normalized_artifact_id.chars().any(char::is_control)
        {
            return Err(SceneArtifactError::NonCanonicalArtifactId);
        }
        Ok(Self {
            artifact_id: normalized_artifact_id,
        })
    }
}

/// Receipt returned by a scene-writer adapter after writing an artifact.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SceneArtifactReceipt {
    /// Adapter-local location or handle for the written artifact.
    pub location: String,
}

impl SceneArtifactReceipt {
    /// Create a writer receipt without exposing filesystem semantics upstream.
    ///
    /// # Errors
    ///
    /// Returns an error when the receipt location is blank.
    pub fn new(
        location: impl Into<String>,
    ) -> Result<Self, SceneArtifactError> {
        let normalized_location = location.into();
        if normalized_location.trim().is_empty() {
            return Err(SceneArtifactError::MissingReceiptLocation);
        }
        if normalized_location != normalized_location.trim()
            || normalized_location.chars().any(char::is_control)
        {
            return Err(SceneArtifactError::NonCanonicalReceiptLocation);
        }
        Ok(Self {
            location: normalized_location,
        })
    }
}

/// Writes one normalized scene package to an interchange artifact.
pub trait SceneWriter {
    /// Write one normalized scene.
    ///
    /// # Errors
    ///
    /// Returns a stable error message when the adapter cannot serialize or
    /// persist the artifact.
    fn write_scene(
        &self,
        scene: &Scene,
        target: &SceneArtifactTarget,
    ) -> Result<SceneArtifactReceipt, String>;
}
