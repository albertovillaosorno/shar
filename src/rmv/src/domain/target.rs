// File:
//   - target.rs
// Path:
//   - src/rmv/src/domain/target.rs
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
//   - Pure rmv domain rules for domain target.
// - Must-Not:
//   - Read files, parse generated indexes, invoke CLI code, or call writer
//   - adapters.
// - Allows:
//   - Value objects, invariant checks, and pure evidence-to-domain translation.
// - Split-When:
//   - Split when target contains two independently testable contracts.
// - Merge-When:
//   - Another rmv module owns the same domain boundary with no distinct
//   - invariant.
// - Summary:
//   - Target strategy for migrated cinematics.
// - Description:
//   - Defines target data and behavior for rmv domain.
// - Usage:
//   - Imported through crate domain facades or sibling domain modules.
// - Defaults:
//   - No filesystem paths, no external process calls, and no implicit IO
//   - defaults.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Target strategy for migrated cinematics.
//!
//! Bink 2 is an optional official-tooling target, not the only professional
//! target. The portable fallback is a HAP movie stream plus WAV package so
//! every developer can rebuild cinematics without private encoder binaries,
//! global installs, or PATH mutation.
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
        matches!(
            self,
            Self::OfficialBink2
        )
    }

    #[must_use]
    /// Is default without official encoder.
    pub const fn is_default_without_official_encoder(self) -> bool {
        matches!(
            self,
            Self::UnrealHapMovie
        )
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
mod tests {
    use super::{CinematicTarget, TargetDecision};

    #[test]
    fn unreal_hap_movie_is_default_without_private_encoder() {
        let decision = TargetDecision::without_official_bink2_encoder();
        assert_eq!(
            decision.primary_target,
            CinematicTarget::UnrealHapMovie
        );
        assert_eq!(
            decision.optional_target,
            Some(CinematicTarget::OfficialBink2)
        );
    }

    #[test]
    fn official_bink2_is_marked_as_private_encoder_dependent() {
        assert!(CinematicTarget::OfficialBink2.requires_private_encoder());
        assert!(!CinematicTarget::UnrealHapMovie.requires_private_encoder());
    }
}
