// File:
//   - requirement.rs
// Path:
//   - src/fbx/src/domain/shader/requirement.rs
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
//   - Pure fbx domain rules for domain shader requirement.
// - Must-Not:
//   - Read files, parse generated indexes, invoke CLI code, or call writer
//   - adapters.
// - Allows:
//   - Value objects, invariant checks, and pure evidence-to-domain translation.
// - Split-When:
//   - Split when requirement contains two independently testable contracts.
// - Merge-When:
//   - Another fbx module owns the same domain boundary with no distinct
//   - invariant.
// - Summary:
//   - One decoded shader input normalized for material planning.
// - Description:
//   - Defines requirement data and behavior for fbx domain shader.
// - Usage:
//   - Imported through crate domain facades or sibling domain modules.
// - Defaults:
//   - No filesystem paths, no external process calls, and no implicit IO
//   - defaults.
//
// ADRs:
// - docs/adr/pipeline/fbx/hexagonal-scene-export.md
// - docs/adr/pipeline/unreal/unreal-manifest-and-package-taxonomy.md
//
// Large file:
//   - false
//

//! One decoded shader input normalized for material planning.
// These exact file-local lints preserve explicit domain and binary contracts.
#![expect(
    clippy::module_name_repetitions,
    reason = "Tests verify these intentional explicit file-local contracts \
              remain safe."
)]

use super::channel::MaterialChannel;

/// Shader-requirement validation failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ShaderRequirementError {
    /// Shader identity was empty or whitespace-only.
    MissingShaderId,
    /// Shader identity carried surrounding whitespace.
    NonCanonicalShaderId,
    /// Texture member identity was empty or whitespace-only.
    BlankTextureMemberId,
    /// Texture member identity carried surrounding whitespace.
    NonCanonicalTextureMemberId,
}

/// One decoded shader input normalized for material planning.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ShaderRequirement {
    /// Stable shader id or name from member evidence.
    pub shader_id: String,
    /// Required material channel.
    pub channel: MaterialChannel,
    /// Optional texture member id bound to the channel.
    pub texture_member_id: Option<String>,
}

impl ShaderRequirement {
    /// Build one material requirement.
    ///
    /// # Errors
    ///
    /// Returns an error when a required identity is blank.
    pub fn new(
        shader_id: impl Into<String>,
        channel: MaterialChannel,
        texture_member_id: Option<String>,
    ) -> Result<Self, ShaderRequirementError> {
        let normalized_shader_id = shader_id.into();
        if normalized_shader_id
            .trim()
            .is_empty()
        {
            return Err(ShaderRequirementError::MissingShaderId);
        }
        if normalized_shader_id != normalized_shader_id.trim()
            || normalized_shader_id
                .chars()
                .any(char::is_control)
        {
            return Err(ShaderRequirementError::NonCanonicalShaderId);
        }
        if texture_member_id
            .as_ref()
            .is_some_and(
                |member_id| {
                    member_id
                        .trim()
                        .is_empty()
                },
            )
        {
            return Err(ShaderRequirementError::BlankTextureMemberId);
        }
        if texture_member_id
            .as_ref()
            .is_some_and(
                |member_id| {
                    member_id != member_id.trim()
                        || member_id
                            .chars()
                            .any(char::is_control)
                },
            )
        {
            return Err(ShaderRequirementError::NonCanonicalTextureMemberId);
        }
        Ok(
            Self {
                shader_id: normalized_shader_id,
                channel,
                texture_member_id,
            },
        )
    }
}
