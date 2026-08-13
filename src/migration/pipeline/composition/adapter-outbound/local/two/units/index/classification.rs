// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT
//
// Boundary-Contract:
// - Owns:
//   - Minor-unit package category and subcategory classification.
// - Must-Not:
//   - Own filesystem intake, index publication, or role decoding.
// - Allows:
//   - Deterministic classification from normalized package-root evidence.
// - Split-When:
//   - Split when one category family gains independent evidence rules.
// - Merge-When:
//   - Merge when another module owns the identical classification policy.
// - Summary:
//   - Minor-unit package classification.
// - Description:
//   - Maps normalized package roots into stable category identities.
// - Usage:
//   - Used by the owning phase-two minor-unit index writer.
// - Defaults:
//   - Unknown roots remain explicitly unmapped.
//

//! Minor-unit package classification.

/// High-level package category for browsing and exporter routing.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in super::super) enum PackageCategory {
    /// Playable and non-playable character packages.
    Characters,
    /// Vehicle packages.
    Cars,
    /// Terrain, roads, interiors, and connected world chunks.
    TerrainWorld,
    /// Mission art and mission-specific assets.
    Missions,
    /// Collectible card art.
    Cards,
    /// UI screen/page packages.
    UiScreens,
    /// UI image packages loaded by frontend dynaload.
    UiImages,
    /// UI resource packages from Scrooby resource folders.
    UiResources,
    /// UI vehicle preview packages.
    UiVehiclePreviews,
    /// UI component packages.
    /// Language and localization packages.
    Language,
    /// Non-interactive sequence or cinematic art packages.
    Cinematics,
    /// Music packages.
    Music,
    /// Dialog voice packages.
    Dialog,
    /// Sound-effect and ambience packages.
    SoundEffects,
    /// Movie packages.
    Movies,
    /// Mission script packages.
    MissionScripts,
    /// Vehicle tuning script packages.
    VehicleTuning,
    /// Sound script packages.
    SoundScripts,
    /// Props, buildings, tools, effects, or miscellaneous art packages.
    Props,
    /// Extraction report packages.
    ExtractionReports,
    /// Game icon packages.
    GameIcons,
    /// Unmapped package category that must be fixed before export.
    Error,
}

impl PackageCategory {
    /// Stable category label used in index output.
    #[must_use]
    pub(in super::super) const fn as_str(self) -> &'static str {
        match self {
            Self::Characters => "characters",
            Self::Cars => "cars",
            Self::TerrainWorld => "terrain-world",
            Self::Missions => "missions",
            Self::Cards => "cards",
            Self::UiScreens => "ui-screens",
            Self::UiImages => "ui-images",
            Self::UiResources => "ui-resources",
            Self::UiVehiclePreviews => "ui-vehicle-previews",
            Self::Language => "language",
            Self::Cinematics => "cinematics",
            Self::Music => "music",
            Self::Dialog => "dialog",
            Self::SoundEffects => "sound-effects",
            Self::Movies => "movies",
            Self::MissionScripts => "mission-scripts",
            Self::VehicleTuning => "vehicle-tuning",
            Self::SoundScripts => "sound-scripts",
            Self::Props => "props",
            Self::ExtractionReports => "extraction-reports",
            Self::GameIcons => "game-icons",
            Self::Error => "error",
        }
    }
}

/// Supports the `subcategory_from_root` operation within this deterministic
/// classification boundary.
pub(super) fn subcategory_from_root(package_root: &str) -> String {
    if let Some(dialog) = dialog_subcategory(package_root) {
        return dialog;
    }
    if package_root == "extracted/art/cards" {
        return "cards/pickup-effects".to_owned();
    }
    if let Some(character) = character_subcategory(package_root) {
        return character;
    }
    if let Some(mission) = mission_subcategory(package_root) {
        return mission;
    }
    if let Some(script) = script_subcategory(package_root) {
        return script;
    }
    if let Some(language) = language_subcategory(package_root) {
        return language;
    }
    if let Some(prop) = prop_subcategory(package_root) {
        return prop;
    }
    category_from_root(package_root).as_str().to_owned()
}

/// Supports the `character_subcategory` operation within this deterministic
/// classification boundary.
fn character_subcategory(package_root: &str) -> Option<String> {
    let root = package_root.to_ascii_lowercase();
    let asset_part = root
        .strip_prefix("extracted/art/chars/")
        .or_else(|| root.strip_prefix("extracted/game/art/chars/"));
    let Some(asset) = asset_part else {
        if root == "extracted/game/art/chars" {
            return Some("characters/registry/package".to_owned());
        }
        return None;
    };
    if asset == "global" {
        return Some("characters/rig/common".to_owned());
    }
    if let Some(base) = asset.strip_suffix("_a") {
        return Some(format!(
            "characters/{}/animation-set",
            character_name(base)
        ));
    }
    if let Some(base) = asset.strip_suffix("_electrocuted") {
        return Some(format!(
            "characters/{}/effect/electrocuted",
            character_name(base)
        ));
    }
    if let Some(base) = asset.strip_suffix("_kickwave") {
        return Some(format!(
            "characters/{}/effect/kickwave",
            character_name(base)
        ));
    }
    if let Some((character, costume)) = character_costume(asset) {
        return Some(format!("characters/{character}/costume/{costume}"));
    }
    if asset == "ndr_m" {
        return Some("characters/ned/base-model/ndr".to_owned());
    }
    let base = asset.strip_suffix("_m").unwrap_or(asset);
    Some(format!(
        "characters/{}/{}",
        character_name(base),
        character_model_group(base)
    ))
}

/// Supports the `character_costume` operation within this deterministic
/// classification boundary.
fn character_costume(asset: &str) -> Option<(&'static str, &'static str)> {
    apu_costume(asset)
        .or_else(|| bart_costume(asset))
        .or_else(|| homer_costume(asset))
        .or_else(|| lisa_costume(asset))
        .or_else(|| marge_costume(asset))
        .or_else(|| barney_costume(asset))
}

/// Supports the `apu_costume` operation within this deterministic
/// classification boundary.
fn apu_costume(asset: &str) -> Option<(&'static str, &'static str)> {
    match asset {
        "a_amer_m" => Some(("apu", "american")),
        "a_army_m" => Some(("apu", "army")),
        "a_besh_m" => Some(("apu", "be-sharps")),
        _ => None,
    }
}

/// Supports the `bart_costume` operation within this deterministic
/// classification boundary.
fn bart_costume(asset: &str) -> Option<(&'static str, &'static str)> {
    match asset {
        "b_foot_m" => Some(("bart", "football")),
        "b_hugo_m" => Some(("bart", "hugo")),
        "b_man_m" => Some(("bart", "bartman")),
        "b_mili_m" => Some(("bart", "military")),
        "b_ninj_m" => Some(("bart", "ninja")),
        "b_tall_m" => Some(("bart", "tall")),
        _ => None,
    }
}

/// Supports the `homer_costume` operation within this deterministic
/// classification boundary.
fn homer_costume(asset: &str) -> Option<(&'static str, &'static str)> {
    match asset {
        "h_donu_m" => Some(("homer", "donut")),
        "h_evil_m" => Some(("homer", "evil")),
        "h_fat_m" => Some(("homer", "muumuu")),
        "h_scuz_m" => Some(("homer", "scuzzy")),
        "h_stcr_m" => Some(("homer", "stonecutter")),
        "h_undr_m" => Some(("homer", "underwear")),
        _ => None,
    }
}

/// Supports the `lisa_costume` operation within this deterministic
/// classification boundary.
fn lisa_costume(asset: &str) -> Option<(&'static str, &'static str)> {
    match asset {
        "l_cool_m" => Some(("lisa", "cool")),
        "l_flor_m" => Some(("lisa", "florida")),
        "l_jers_m" => Some(("lisa", "jersey")),
        _ => None,
    }
}

/// Supports the `marge_costume` operation within this deterministic
/// classification boundary.
fn marge_costume(asset: &str) -> Option<(&'static str, &'static str)> {
    match asset {
        "m_pink_m" => Some(("marge", "pink")),
        "m_poli_m" => Some(("marge", "police")),
        "m_pris_m" => Some(("marge", "prisoner")),
        _ => None,
    }
}

/// Supports the `barney_costume` operation within this deterministic
/// classification boundary.
fn barney_costume(asset: &str) -> Option<(&'static str, &'static str)> {
    match asset {
        "brn_un_m" => Some(("barney", "underwear")),
        _ => None,
    }
}

/// Supports the `character_model_group` operation within this deterministic
/// classification boundary.
fn character_model_group(base: &str) -> &'static str {
    const CROWD_PREFIXES: &[&str] = &[
        "boy", "girl", "male", "fem", "olady", "busm", "busw", "joger", "sail",
        "const", "rednk", "zfem", "zmale",
    ];

    if CROWD_PREFIXES.iter().any(|prefix| base.starts_with(prefix)) {
        "crowd-model"
    } else if base.starts_with('z') || base == "witch" || base == "franke" {
        "halloween-model"
    } else {
        "base-model"
    }
}

/// Supports the `character_name` operation within this deterministic
/// classification boundary.
fn character_name(code: &str) -> String {
    match code {
        "askinn" => "agnes-skinner".to_owned(),
        "apu" => "apu".to_owned(),
        "barney" | "brn" => "barney".to_owned(),
        "bart" => "bart".to_owned(),
        "beeman" => "bumblebee-man".to_owned(),
        "burns" => "burns".to_owned(),
        "captai" => "sea-captain".to_owned(),
        "carl" => "carl".to_owned(),
        "cbg" => "comic-book-guy".to_owned(),
        "cletus" => "cletus".to_owned(),
        "dolph" => "dolph".to_owned(),
        "eddie" => "eddie".to_owned(),
        "franke" => "frankenstein".to_owned(),
        "frink" => "frink".to_owned(),
        "gil" => "gil".to_owned(),
        "grandp" => "grampa".to_owned(),
        "hibber" => "hibbert".to_owned(),
        "homer" => "homer".to_owned(),
        "hooker" => "hooker".to_owned(),
        "jasper" => "jasper".to_owned(),
        "jimbo" => "jimbo".to_owned(),
        "kearne" => "kearney".to_owned(),
        "krusty" => "krusty".to_owned(),
        "lenny" => "lenny".to_owned(),
        "lisa" => "lisa".to_owned(),
        "lou" => "lou".to_owned(),
        "louie" => "louie".to_owned(),
        "marge" => "marge".to_owned(),
        "milhou" => "milhouse".to_owned(),
        "mobstr" => "mobster".to_owned(),
        "moe" => "moe".to_owned(),
        "molema" => "moleman".to_owned(),
        "ndr" | "ned" => "ned".to_owned(),
        "nelson" => "nelson".to_owned(),
        "npd" => "apu-driver".to_owned(),
        "nps" => "school-bus-driver".to_owned(),
        "nrivie" => "riviera".to_owned(),
        "nuclea" => "nuclear-worker".to_owned(),
        "otto" => "otto".to_owned(),
        "patty" => "patty".to_owned(),
        "ralph" => "ralph".to_owned(),
        "selma" => "selma".to_owned(),
        "skinne" => "skinner".to_owned(),
        "smithe" => "smithers".to_owned(),
        "snake" => "snake".to_owned(),
        "teen" => "squeaky-voiced-teen".to_owned(),
        "wiggum" => "wiggum".to_owned(),
        "willie" => "willie".to_owned(),
        other => other.replace('_', "-"),
    }
}

/// Supports the `dialog_subcategory` operation within this deterministic
/// classification boundary.
fn dialog_subcategory(package_root: &str) -> Option<String> {
    let root = package_root.to_ascii_lowercase();
    if let Some(name) = root.strip_prefix("extracted/dialog/conversations/") {
        let parts = name.split('/').collect::<Vec<_>>();
        if let Some(speaker) = parts.first()
            && let Some(kind) = parts.get(1)
        {
            return Some(format!(
                "dialog/{}/conversation/{}/{}",
                speaker_name(speaker),
                kind,
                parts.get(2).copied().unwrap_or("global")
            ));
        }
    }
    if let Some(character) = root.strip_prefix("extracted/dialog/") {
        return Some(format!("dialog/{}/ad-lib", speaker_name(character)));
    }
    None
}

/// Supports the `mission_subcategory` operation within this deterministic
/// classification boundary.
fn mission_subcategory(package_root: &str) -> Option<String> {
    let root = package_root.to_ascii_lowercase();
    if let Some(rest) = root.strip_prefix("extracted/art/missions/") {
        let parts = rest.split('/').collect::<Vec<_>>();
        return Some(match parts.as_slice() {
            ["generic", tail @ ..] => format!(
                "missions/generic/{}",
                tail.first().copied().unwrap_or("root")
            ),
            ["h2h", tail @ ..] => format!(
                "missions/head-to-head/{}",
                tail.first().copied().unwrap_or("root")
            ),
            [level, asset, ..] if level.starts_with("level") => {
                let normalized = normalize_level(level);
                if *level == "level01" && is_tutorial_mission_asset(asset) {
                    format!("missions/tutorial/{asset}")
                } else {
                    format!("missions/{normalized}/{asset}")
                }
            },
            [only] => format!("missions/uncategorized/{only}"),
            [head, tail @ ..] => format!(
                "missions/uncategorized/{}/{}",
                head,
                tail.first().copied().unwrap_or("root")
            ),
            [] => "missions/root".to_owned(),
        });
    }
    None
}

/// Supports the `script_subcategory` operation within this deterministic
/// classification boundary.
fn script_subcategory(package_root: &str) -> Option<String> {
    let root = package_root.to_ascii_lowercase();
    if root == "extracted/game/scripts" {
        return Some("missions/bootstrap/scripts/root".to_owned());
    }
    if root == "extracted/game/scripts/missions" {
        return Some("missions/bootstrap/scripts/missions".to_owned());
    }
    if let Some(rest) = root.strip_prefix("extracted/game/scripts/missions/") {
        return Some(format!("missions/{}/scripts", normalize_level(rest)));
    }
    if let Some(rest) =
        root.strip_prefix("extracted/game/scripts/cars/missions/")
    {
        return Some(format!(
            "missions/{}/vehicle-tuning",
            normalize_level(rest)
        ));
    }
    if let Some(rest) = root.strip_prefix("extracted/game/scripts/cars/") {
        return Some(format!("vehicle-tuning/{rest}"));
    }
    if root == "extracted/game/scripts/cars" {
        return Some("vehicle-tuning/free-roam".to_owned());
    }
    if root.starts_with("extracted/scripts/sound/scripts") {
        return Some("sound-scripts/vehicle-dialog-routing".to_owned());
    }
    None
}

/// Supports the `language_subcategory` operation within this deterministic
/// classification boundary.
fn language_subcategory(package_root: &str) -> Option<String> {
    let root = package_root.to_ascii_lowercase();
    if root.ends_with("/language") || root.contains("/language/") {
        if root.contains("/scrooby2/") {
            return Some("language/ui-text/scene-layouts".to_owned());
        }
        if root.contains("/scrooby/") {
            return Some("language/ui-text/sprite-layouts".to_owned());
        }
        return Some("language/ui-text".to_owned());
    }
    None
}

/// Supports the `prop_subcategory` operation within this deterministic
/// classification boundary.
fn prop_subcategory(package_root: &str) -> Option<String> {
    let root = package_root.to_ascii_lowercase();
    let asset = root
        .strip_prefix("extracted/art/")
        .or_else(|| root.strip_prefix("extracted/game/art/"))?;
    match asset {
        "atc/atc" | "phonecamera" | "wrench" => {
            Some(format!("props/{}", asset.replace('_', "-")))
        },
        _ => None,
    }
}

/// Supports the `normalize_level` operation within this deterministic
/// classification boundary.
fn normalize_level(value: &str) -> String {
    let lower = value.to_ascii_lowercase();
    if let Some(number) = lower.strip_prefix("level")
        && let Ok(parsed) = number.parse::<u8>()
    {
        return format!("level-{parsed:02}");
    }
    lower
}

/// Supports the `is_tutorial_mission_asset` operation within this deterministic
/// classification boundary.
pub(super) fn is_tutorial_mission_asset(value: &str) -> bool {
    matches!(
        value,
        "m0" | "demo" | "democams" | "mission0cam" | "tutorial"
    )
}

/// Supports the `pedestrian_speaker_name` operation within this deterministic
/// classification boundary.
fn pedestrian_speaker_name(code_or_name: &str) -> Option<&'static str> {
    match code_or_name {
        "generic-boy-1" => Some("pedestrian-boy-1"),
        "generic-boy-2" => Some("pedestrian-boy-2"),
        "generic-female-1" => Some("pedestrian-woman-1"),
        "generic-female-2" => Some("pedestrian-woman-2"),
        "generic-girl-1" => Some("pedestrian-girl-1"),
        "generic-girl-2" => Some("pedestrian-girl-2"),
        "generic-male-1" => Some("pedestrian-man-1"),
        "generic-male-2" => Some("pedestrian-man-2"),
        _ => None,
    }
}

/// Supports the `speaker_name` operation within this deterministic
/// classification boundary.
pub(super) fn speaker_name(code_or_name: &str) -> &'static str {
    if let Some(pedestrian) = pedestrian_speaker_name(code_or_name) {
        return pedestrian;
    }
    match code_or_name {
        "agn" | "agnes" => "agnes",
        "apu" => "apu",
        "brn" | "barney" => "barney",
        "brt" | "bart" => "bart",
        "bur" | "burns" => "burns",
        "cbg" | "comic_book_guy" => "comic-book-guy",
        "clt" | "cletus" => "cletus",
        "crl" | "carl" => "carl",
        "fla" | "flanders" => "flanders",
        "frk" | "dr.frink" | "frink" => "frink",
        "grp" | "grampa" | "grandpa" => "grampa",
        "hib" | "dr.hibbert" => "hibbert",
        "hom" | "homer" => "homer",
        "kea" | "kearney" => "kearney",
        "kru" | "krusty" => "krusty",
        "len" | "lenny" => "lenny",
        "lis" | "lisa" => "lisa",
        "mil" | "milhouse" => "milhouse",
        "moe" => "moe",
        "mrg" | "marge" => "marge",
        "nel" | "nelson" => "nelson",
        "nic" | "dr.nick" => "dr-nick",
        "oto" | "otto" => "otto",
        "pat" | "patty" => "patty",
        "ral" | "ralph" => "ralph",
        "sea" | "captain" => "sea-captain",
        "skn" | "skinner" => "skinner",
        "smi" | "smithers" => "smithers",
        "snk" | "snake" => "snake",
        "svt" | "squeaky_voiced_teen" => "squeaky-voiced-teen",
        "wig" | "wiggum" => "wiggum",
        "zom" | "zm1" | "zm2" | "zm3" => "zombie",
        other => Box::leak(other.replace('_', "-").into_boxed_str()),
    }
}

/// Supports the `category_from_root` operation within this deterministic
/// classification boundary.
pub(in super::super) fn category_from_root(
    package_root: &str,
) -> PackageCategory {
    let root = package_root.to_ascii_lowercase();
    if root == "extracted" {
        PackageCategory::ExtractionReports
    } else if root == "game" {
        PackageCategory::GameIcons
    } else if root.starts_with("extracted/art/chars/")
        || root.starts_with("extracted/game/art/chars")
    {
        PackageCategory::Characters
    } else if root.starts_with("extracted/art/cars/") {
        PackageCategory::Cars
    } else if root.starts_with("extracted/art/frontend/dynaload/cars/")
        || root.starts_with("extracted/game/art/frontend/dynaload/cars")
    {
        PackageCategory::UiVehiclePreviews
    } else if root.starts_with("extracted/art/frontend/dynaload/images")
        || root.starts_with("extracted/game/art/frontend/dynaload/images")
    {
        PackageCategory::UiImages
    } else if root.starts_with("extracted/art/frontend/scrooby/resource/")
        || root.starts_with("extracted/art/frontend/scrooby2/resource/")
    {
        PackageCategory::UiResources
    } else if root.ends_with("/language")
        || root.contains("/language/")
    {
        PackageCategory::Language
    } else if root.starts_with("extracted/art/frontend/scrooby/")
        || root.starts_with("extracted/art/frontend/scrooby2/")
        || root.starts_with("extracted/game/art/frontend/scrooby/")
        || root.starts_with("extracted/game/art/frontend/scrooby2")
    {
        PackageCategory::UiScreens
    } else if root.starts_with("extracted/art/missions/") {
        PackageCategory::Missions
    } else if root == "extracted/art/cards"
        || root.starts_with("extracted/art/cards/")
    {
        PackageCategory::Cards
    } else if root.starts_with("extracted/art/nis/")
        || root.starts_with("extracted/nis/")
    {
        PackageCategory::Cinematics
    } else if root.starts_with("extracted/music") {
        PackageCategory::Music
    } else if root.starts_with("extracted/dialog") {
        PackageCategory::Dialog
    } else if root.starts_with("extracted/movies/") {
        PackageCategory::Movies
    } else if root == "extracted/game/scripts/cars"
        || root.starts_with("extracted/game/scripts/cars/")
    {
        PackageCategory::VehicleTuning
    } else if root == "extracted/game/scripts"
        || root.starts_with("extracted/game/scripts/missions")
    {
        PackageCategory::MissionScripts
    } else if root.starts_with("extracted/scripts/sound/scripts") {
        PackageCategory::SoundScripts
    } else if root.starts_with("extracted/soundfx/")
        || root.starts_with("extracted/ambience/")
        || root.starts_with("extracted/carsound/")
        || root.starts_with("extracted/game/sound/")
        || root == "extracted/sound"
    {
        PackageCategory::SoundEffects
    } else if is_world_art_root(&root) {
        PackageCategory::TerrainWorld
    } else if root.starts_with("extracted/art/") {
        PackageCategory::Props
    } else {
        PackageCategory::Error
    }
}

/// Supports the `is_world_art_root` operation within this deterministic
/// classification boundary.
fn is_world_art_root(root: &str) -> bool {
    let Some(name) = root.strip_prefix("extracted/art/") else {
        return false;
    };
    let lower = name.to_ascii_lowercase();
    lower.contains("terra")
        || lower.starts_with('l')
            && lower
                .chars()
                .nth(1)
                .is_some_and(|value| value.is_ascii_digit())
        || lower.starts_with('b')
            && lower
                .chars()
                .nth(1)
                .is_some_and(|value| value.is_ascii_digit())
}
