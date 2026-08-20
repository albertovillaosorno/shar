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
//   - Target domain module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Target domain module.
// - Description:
//   - Implements the declared domain module responsibility for rmv.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Target domain module.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Cinematictarget.
pub enum CinematicTarget {
    /// Item.
    OfficialBink2,
    /// Item.
    UnrealHapMovie,
}

impl CinematicTarget {
    #[must_use]
    /// Label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::OfficialBink2 => "official-bink2",
            Self::UnrealHapMovie => "unreal-hap-movie-wav",
        }
    }

    #[must_use]
    /// Requires private encoder.
    pub const fn requires_private_encoder(self) -> bool {
        matches!(self, Self::OfficialBink2)
    }

    #[must_use]
    /// Is default without official encoder.
    pub const fn is_default_without_official_encoder(self) -> bool {
        matches!(self, Self::UnrealHapMovie)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Targetdecision.
pub struct TargetDecision {
    /// Primary target.
    pub primary_target: CinematicTarget,
    /// Optional target.
    pub optional_target: Option<CinematicTarget>,
    /// Reason.
    pub reason: &'static str,
}

impl TargetDecision {
    #[must_use]
    /// Without official bink2 encoder.
    pub const fn without_official_bink2_encoder() -> Self {
        Self {
            primary_target: CinematicTarget::UnrealHapMovie,
            optional_target: Some(CinematicTarget::OfficialBink2),
            reason: "Official Bink 2 encoding requires official Epic/RAD \
                     tooling. Use a HAP movie plus WAV as the reproducible \
                     default; BK2 remains an optional licensed/tool-available \
                     output.",
        }
    }
}

#[cfg(test)]
#[path = "../../../../tests/formats/rmv/unit/domain/target/tests.rs"]
mod tests;
