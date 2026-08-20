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
//   - Pure official-language identities, evidence records, and errors.
// - Must-Not:
//   - Own filesystem access, serialization, hashing, or package publication.
// - Allows:
//   - Model source-backed language selection and deterministic export evidence.
// - Split-When:
//   - Language identity and export evidence gain independent lifecycles.
// - Merge-When:
//   - Another domain module owns the identical language data contract.
// - Summary:
//   - Pure official-language domain model.
// - Description:
//   - Keeps localization data policy independent from export composition.
// - Usage:
//   - Used through the owning official-language function facade.
// - Defaults:
//   - Missing or invalid source evidence fails through explicit errors.
//

//! Pure official-language identities and export evidence.

/// One official non-English language carried by the original game source.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    /// French localization.
    French,
    /// German localization.
    German,
    /// Italian localization where a lawful source actually contains it.
    Italian,
    /// Spanish localization.
    Spanish,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct LanguageSpec {
    pub(crate) name: &'static str,
    pub(crate) code: &'static str,
    pub(crate) column: usize,
    pub(crate) dialogue: &'static str,
    pub(crate) readme: Option<&'static str>,
    pub(crate) ui_directory: &'static str,
    pub(crate) movie_audio_track: Option<usize>,
}

impl Language {
    pub(crate) const fn spec(self) -> LanguageSpec {
        match self {
            Self::French => LanguageSpec {
                name: "french",
                code: "F",
                column: 4,
                dialogue: "dialogf.rcf",
                readme: Some("Lisez-moi.rtf"),
                ui_directory: "french",
                movie_audio_track: Some(2),
            },
            Self::German => LanguageSpec {
                name: "german",
                code: "G",
                column: 5,
                dialogue: "dialogg.rcf",
                readme: Some("Liesmich.rtf"),
                ui_directory: "german",
                movie_audio_track: Some(3),
            },
            Self::Italian => LanguageSpec {
                name: "italian",
                code: "I",
                column: 6,
                dialogue: "dialogi.rcf",
                readme: None,
                ui_directory: "italian",
                movie_audio_track: None,
            },
            Self::Spanish => LanguageSpec {
                name: "spanish",
                code: "S",
                column: 7,
                dialogue: "dialogs.rcf",
                readme: Some("Léeme.rtf"),
                ui_directory: "spanish",
                movie_audio_track: Some(4),
            },
        }
    }
}

/// One preserved source artifact.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceEvidence {
    /// Portable source-relative path.
    pub path: String,
    /// Exact byte size.
    pub size: u64,
    /// Exact SHA-256 digest.
    pub sha256: String,
}

/// One normalized cinematic audio artifact selected for the language.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CinematicAudioEvidence {
    /// Movie package name.
    pub movie: String,
    /// Normalized track filename selected from that movie package.
    pub track: String,
    /// Exact byte size.
    pub size: u64,
    /// Exact SHA-256 digest.
    pub sha256: String,
}

/// Deterministic evidence for one canonical language-mod source bundle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LanguageManifest {
    /// Stable schema identifier.
    pub schema: &'static str,
    /// Canonical base language this mod overlays.
    pub base_language: &'static str,
    /// Selected non-English language.
    pub language: &'static str,
    /// Original `TextBible` language code.
    pub language_code: &'static str,
    /// Number of translated `TextBible` records.
    pub records: usize,
    /// Number of authored `???` placeholders retained verbatim.
    pub untranslated_placeholders: usize,
    /// Exact preserved original-language sources.
    pub included_sources: Vec<SourceEvidence>,
    /// Normalized localized movie-audio tracks included in the bundle.
    pub cinematic_audio: Vec<CinematicAudioEvidence>,
    /// Optional original sources not present in this lawful installation.
    pub missing_optional_sources: Vec<String>,
    /// Canonical SHAR package identity generated for this language.
    pub package_id: String,
    /// Current package-contract state.
    pub status: &'static str,
}
