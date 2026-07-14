// File:
//   - ui_resources.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/two/units/ui_resources.rs
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
//   - The ui resources contract for pipeline phase two minor units.
// - Must-Not:
//   - Violate repository architecture, path, provenance, or output rules.
// - Allows:
//   - Operations required to validate and execute ui resources.
// - Split-When:
//   - Split when ui resources contains two independently testable contracts.
// - Merge-When:
//   - Another pipeline module owns the same module boundary with no distinct
//   - invariant.
// - Summary:
//   - UI resource package classifier.
// - Description:
//   - Defines ui resources data and behavior for pipeline phase two minor
//   - units.
// - Usage:
//   - Used by pipeline phase two minor units code that needs ui resources.
// - Defaults:
//   - No implicit output outside the repository is allowed.
//
// ADRs:
// - docs/adr/pipeline/minor-unit-taxonomy-value-case.md
//
// Large file:
//   - true
//   - Reason: UI resource package classifier keeps tightly coupled
//   - validation, ordering, and deterministic transformation invariants
//   - together; split when a stable independently testable sub-boundary is
//   - identified.
//

//! UI resource package classifier.
//! UI resource package classifier.

use super::index::{
    MinorUnitPackage, PackageCategory, category_from_root, package_id_tokens,
};

/// Supports the `classification_from_package` operation within this
/// deterministic classification boundary.
pub(super) fn classification_from_package(
    package: &MinorUnitPackage
) -> Option<(
    PackageCategory,
    String,
)> {
    let tokens = package_id_tokens(package);
    if category_from_root(&package.package_root) != PackageCategory::UiResources
        && !tokens.contains(&"resource")
    {
        return None;
    }
    let resource_index = tokens
        .iter()
        .position(|token| *token == "resource")?;
    let tail = tokens.get(resource_index.saturating_add(1)..)?;
    if let Some(subcategory) = scene_resource_subcategory(
        &tokens, tail,
    ) {
        return Some(
            (
                PackageCategory::UiResources,
                subcategory,
            ),
        );
    }
    let scope = ui_resource_scope(tail)?;
    let role = ui_resource_role(tail);
    let detail = ui_resource_detail(
        &tokens, tail, role,
    );
    Some(
        (
            PackageCategory::UiResources,
            format!("ui-resources/{scope}/{role}{detail}"),
        ),
    )
}

/// Supports the `ui_resource_detail` operation within this deterministic
/// classification boundary.
fn ui_resource_detail(
    tokens: &[&str],
    tail: &[&str],
    role: &str,
) -> String {
    if !ui_resource_role_allows_detail(role) {
        return String::new();
    }
    if role == "art-assets" {
        return ui_resource_art_detail(
            tokens, tail,
        );
    }
    tail.get(1..)
        .filter(|rest| !rest.is_empty())
        .map_or_else(
            String::new,
            |rest| {
                format!(
                    "/{}",
                    rest.join("-")
                )
            },
        )
}

/// Supports the `ui_resource_art_detail` operation within this deterministic
/// classification boundary.
fn ui_resource_art_detail(
    tokens: &[&str],
    tail: &[&str],
) -> String {
    let family = if tokens.contains(&"scrooby2") {
        "scene-layouts"
    } else if tokens.contains(&"scrooby") {
        "sprite-layouts"
    } else {
        "resource-layouts"
    };
    if tail.is_empty() {
        format!("/{family}/source-metadata")
    } else {
        format!(
            "/{family}/{}",
            tail.join("-")
        )
    }
}

/// Supports the `ui_resource_role_allows_detail` operation within this
/// deterministic classification boundary.
fn ui_resource_role_allows_detail(role: &str) -> bool {
    matches!(
        role,
        "art-assets"
            | "cards"
            | "character-icons"
            | "controls"
            | "fonts"
            | "frames"
            | "glyphs"
            | "hazards"
            | "hit-and-run-meter"
            | "audio-icons"
            | "branding"
            | "communication-icons"
            | "economy-icons"
            | "effects"
            | "loading"
            | "radar-icons"
            | "hud-maps"
            | "hud-panels"
            | "mission-icons"
            | "speaker-icons"
            | "status-icons"
            | "vehicles"
    )
}

/// Supports the `scene_resource_subcategory` operation within this
/// deterministic classification boundary.
fn scene_resource_subcategory(
    tokens: &[&str],
    tail: &[&str],
) -> Option<String> {
    if tail
        .first()
        .is_none_or(|head| *head != "pure3d")
    {
        return None;
    }
    let scene_tokens = tail
        .get(1..)
        .unwrap_or(&[]);
    let role = scene_resource_role(scene_tokens);
    let family = scene_resource_family(tokens);
    let detail = scene_resource_detail(scene_tokens);
    Some(format!("ui-resources/frontend-scenes/{role}/{family}/{detail}"))
}

/// Supports the `scene_resource_family` operation within this deterministic
/// classification boundary.
fn scene_resource_family(tokens: &[&str]) -> &'static str {
    if tokens.contains(&"scrooby2") {
        "scene-layouts"
    } else {
        "sprite-layouts"
    }
}

/// Supports the `scene_resource_detail` operation within this deterministic
/// classification boundary.
fn scene_resource_detail(scene_tokens: &[&str]) -> String {
    if scene_tokens.is_empty() {
        "source-metadata".to_owned()
    } else {
        scene_tokens.join("-")
    }
}

/// Supports the `scene_resource_role` operation within this deterministic
/// classification boundary.
fn scene_resource_role(scene_tokens: &[&str]) -> &'static str {
    if scene_tokens.is_empty()
        || scene_tokens
            .iter()
            .any(|token| *token == "stubs" || *token == "dummy")
    {
        "source-metadata"
    } else if scene_tokens
        .iter()
        .any(|token| token.starts_with("camset"))
    {
        "camera-sets"
    } else if scene_tokens.contains(&"camlight") {
        "camera-lighting"
    } else if scene_tokens
        .iter()
        .any(|token| token.ends_with("hudmap"))
    {
        "hud-maps"
    } else if scene_tokens.contains(&"hudhole") {
        "hud-overlays"
    } else if scene_tokens
        .iter()
        .any(|token| token.starts_with("gag"))
    {
        "gag-scenes"
    } else if scene_tokens
        .iter()
        .any(|token| token.starts_with("glow"))
    {
        "interactive-glows"
    } else if scene_tokens
        .iter()
        .any(scene_transition_token)
    {
        "screen-transitions"
    } else if scene_tokens
        .iter()
        .any(scene_reward_token)
    {
        "reward-presentation"
    } else if scene_tokens
        .iter()
        .any(scene_loading_token)
    {
        "loading-presentation"
    } else if scene_tokens
        .iter()
        .any(scene_character_token)
    {
        "character-scenes"
    } else if scene_tokens
        .iter()
        .any(scene_effect_token)
    {
        "effects"
    } else if scene_tokens
        .iter()
        .any(scene_vehicle_token)
    {
        "vehicle-scenes"
    } else if scene_tokens
        .iter()
        .any(scene_glyph_token)
    {
        "glyph-scenes"
    } else {
        "art-assets"
    }
}

/// Supports the `scene_transition_token` operation within this deterministic
/// classification boundary.
fn scene_transition_token(token: &&str) -> bool {
    matches!(
        *token,
        // cspell:disable-next-line -- curtainl curtainr
        "curtainl" | "curtainr" | "iris"
    )
}

/// Supports the `scene_reward_token` operation within this deterministic
/// classification boundary.
fn scene_reward_token(token: &&str) -> bool {
    token.starts_with("reward")
        || matches!(
            *token,
            "pedestal" | "podium"
        )
}

/// Supports the `scene_loading_token` operation within this deterministic
/// classification boundary.
fn scene_loading_token(token: &&str) -> bool {
    token.starts_with("loading") || token.starts_with("logo")
}

/// Supports the `scene_character_token` operation within this deterministic
/// classification boundary.
fn scene_character_token(token: &&str) -> bool {
    matches!(
        *token,
        "homer" | "maggie"
    )
}

/// Supports the `scene_effect_token` operation within this deterministic
/// classification boundary.
fn scene_effect_token(token: &&str) -> bool {
    matches!(
        *token,
        "cardsfx" | "sparkles" | "star" | "tvscreen"
    )
}

/// Supports the `scene_vehicle_token` operation within this deterministic
/// classification boundary.
fn scene_vehicle_token(token: &&str) -> bool {
    matches!(
        *token, "racecar"
    )
}

/// Supports the `scene_glyph_token` operation within this deterministic
/// classification boundary.
fn scene_glyph_token(token: &&str) -> bool {
    matches!(
        *token,
        "a" | "x"
    )
}

/// Supports the `ui_resource_scope` operation within this deterministic
/// classification boundary.
fn ui_resource_scope(tail: &[&str]) -> Option<String> {
    let head = tail
        .first()
        .copied()?;
    if let Some(level) = head
        .strip_prefix("ingamel")
        .and_then(
            |raw| {
                raw.parse::<u8>()
                    .ok()
            },
        )
        .filter(|level| (1..=7).contains(level))
    {
        return Some(format!("in-game/level-{level:02}"));
    }
    match head {
        "backend" => Some("backend".to_owned()),
        "frontend" => Some("frontend".to_owned()),
        "fonts" => Some("fonts".to_owned()),
        "ingame" => Some("in-game".to_owned()),
        "language" | "txtbible" => Some("language".to_owned()),
        "minigame" => Some("mini-game".to_owned()),
        _ => None,
    }
}

/// Supports the `ui_resource_role` operation within this deterministic
/// classification boundary.
fn ui_resource_role(tail: &[&str]) -> &'static str {
    ui_inventory_role(tail)
        .or_else(|| ui_hud_role(tail))
        .or_else(|| ui_feedback_role(tail))
        .or_else(|| ui_presentation_role(tail))
        .unwrap_or("art-assets")
}

/// Supports the `ui_inventory_role` operation within this deterministic
/// classification boundary.
fn ui_inventory_role(tail: &[&str]) -> Option<&'static str> {
    if ui_tail_has(
        tail,
        |token| {
            token.starts_with("card") || token == "cards" || token == "cardsfx"
        },
    ) {
        Some("cards")
    } else if ui_tail_has(
        tail,
        |token| {
            token.starts_with("car") || token == "racecar" || token == "aicar"
        },
    ) {
        Some("vehicles")
    } else if ui_tail_has(
        tail,
        |token| token.starts_with("font"),
    ) {
        Some("fonts")
    } else if ui_tail_has(
        tail,
        |token| token.starts_with("but") || ui_control_token(token),
    ) {
        Some("controls")
    } else {
        None
    }
}

/// Supports the `ui_hud_role` operation within this deterministic
/// classification boundary.
fn ui_hud_role(tail: &[&str]) -> Option<&'static str> {
    if ui_tail_has(
        tail,
        |token| token.starts_with("frame"),
    ) {
        Some("frames")
    } else if ui_tail_has(
        tail,
        ui_hud_panel_token,
    ) {
        Some("hud-panels")
    } else if ui_tail_has(
        tail,
        ui_hud_map_token,
    ) {
        Some("hud-maps")
    } else if ui_tail_has(
        tail,
        ui_speaker_token,
    ) {
        Some("speaker-icons")
    } else if ui_tail_has(
        tail,
        ui_mission_token,
    ) {
        Some("mission-icons")
    } else if ui_tail_has(
        tail,
        ui_economy_token,
    ) {
        Some("economy-icons")
    } else if ui_tail_has(
        tail,
        ui_radar_token,
    ) {
        Some("radar-icons")
    } else if ui_tail_has(
        tail,
        ui_hit_and_run_token,
    ) {
        Some("hit-and-run-meter")
    } else {
        None
    }
}

/// Supports the `ui_feedback_role` operation within this deterministic
/// classification boundary.
fn ui_feedback_role(tail: &[&str]) -> Option<&'static str> {
    if ui_tail_has(
        tail,
        ui_audio_token,
    ) {
        Some("audio-icons")
    } else if ui_tail_has(
        tail,
        ui_communication_token,
    ) {
        Some("communication-icons")
    } else if ui_tail_has(
        tail,
        ui_branding_token,
    ) {
        Some("branding")
    } else if ui_tail_has(
        tail,
        ui_effect_token,
    ) {
        Some("effects")
    } else if ui_tail_has(
        tail,
        |token| token.starts_with("cam"),
    ) {
        Some("camera-sets")
    } else if ui_tail_has(
        tail,
        |token| token.starts_with("gag"),
    ) {
        Some("gag-icons")
    } else if ui_tail_has(
        tail,
        |token| token.starts_with("glow"),
    ) {
        Some("interactive-glows")
    } else {
        None
    }
}

/// Supports the `ui_presentation_role` operation within this deterministic
/// classification boundary.
fn ui_presentation_role(tail: &[&str]) -> Option<&'static str> {
    if ui_tail_has(
        tail,
        ui_loading_token,
    ) {
        Some("loading")
    } else if ui_tail_has(
        tail,
        |token| token.starts_with("reward"),
    ) {
        Some("rewards")
    } else if ui_tail_has(
        tail,
        ui_hazard_token,
    ) {
        Some("hazards")
    } else if ui_tail_has(
        tail,
        ui_character_token,
    ) {
        Some("character-icons")
    } else if ui_tail_has(
        tail,
        ui_status_token,
    ) {
        Some("status-icons")
    } else if ui_tail_has(
        tail,
        ui_glyph_token,
    ) {
        Some("glyphs")
    } else {
        None
    }
}

/// Supports the `ui_tail_has` operation within this deterministic
/// classification boundary.
fn ui_tail_has(
    tail: &[&str],
    predicate: fn(&str) -> bool,
) -> bool {
    tail.iter()
        .copied()
        .any(predicate)
}

/// Supports the `ui_hud_panel_token` operation within this deterministic
/// classification boundary.
fn ui_hud_panel_token(token: &str) -> bool {
    token.starts_with("corner")
        || matches!(
            token,
            "greybar" | "greybgd" | "levelbar" | "redbgd"
        )
}

/// Supports the `ui_hud_map_token` operation within this deterministic
/// classification boundary.
fn ui_hud_map_token(token: &str) -> bool {
    token.ends_with("hudmap")
}

/// Supports the `ui_speaker_token` operation within this deterministic
/// classification boundary.
fn ui_speaker_token(token: &str) -> bool {
    token.starts_with('q') && token.len() > 1
}

/// Supports the `ui_mission_token` operation within this deterministic
/// classification boundary.
fn ui_mission_token(token: &str) -> bool {
    token.starts_with("msn") || token == "mission" || token.starts_with("movie")
}

/// Supports the `ui_economy_token` operation within this deterministic
/// classification boundary.
fn ui_economy_token(token: &str) -> bool {
    matches!(
        token,
        "coins" | "dollar" | "money"
    )
}

/// Supports the `ui_radar_token` operation within this deterministic
/// classification boundary.
fn ui_radar_token(token: &str) -> bool {
    token.starts_with("radar")
}

/// Supports the `ui_hit_and_run_token` operation within this deterministic
/// classification boundary.
fn ui_hit_and_run_token(token: &str) -> bool {
    token.starts_with("hitnrun")
        || token.starts_with("hr")
        || matches!(
            token,
            "damage" | "ticket"
        )
}

/// Supports the `ui_audio_token` operation within this deterministic
/// classification boundary.
fn ui_audio_token(token: &str) -> bool {
    matches!(
        token,
        "music" | "nosound" | "voice"
    )
}

/// Supports the `ui_communication_token` operation within this deterministic
/// classification boundary.
fn ui_communication_token(token: &str) -> bool {
    matches!(
        token,
        "phone" | "walkie"
    )
}

/// Supports the `ui_branding_token` operation within this deterministic
/// classification boundary.
fn ui_branding_token(token: &str) -> bool {
    matches!(
        token,
        // cspell:disable-next-line -- ksticker
        "gamelogo" | "ksticker"
    )
}

/// Supports the `ui_effect_token` operation within this deterministic
/// classification boundary.
fn ui_effect_token(token: &str) -> bool {
    matches!(
        token,
        // cspell:disable-next-line -- pbglow
        "effects" | "pbglow"
    )
}

/// Supports the `ui_control_token` operation within this deterministic
/// classification boundary.
fn ui_control_token(token: &str) -> bool {
    const TOKENS: &[&str] = &[
        "accept",
        "action",
        // cspell:disable-next-line -- auxx
        "auxx",
        "back",
        "controller",
        // cspell:disable-next-line -- larrow
        "larrow",
        // cspell:disable-next-line -- larrowg
        "larrowg",
        // cspell:disable-next-line -- rarrow
        "rarrow",
        // cspell:disable-next-line -- rarrowg
        "rarrowg",
    ];
    TOKENS.contains(&token)
}

/// Supports the `ui_loading_token` operation within this deterministic
/// classification boundary.
fn ui_loading_token(token: &str) -> bool {
    token.starts_with("load")
        || matches!(
            token,
            // cspell:disable-next-line -- curtainl curtainr
            "curtainl" | "curtainr" | "iris" | "logo" | "anim"
        )
}

/// Supports the `ui_hazard_token` operation within this deterministic
/// classification boundary.
fn ui_hazard_token(token: &str) -> bool {
    matches!(
        token,
        "bomb" | "dynamite" | "fire" | "fire2"
    )
}

/// Supports the `ui_character_token` operation within this deterministic
/// classification boundary.
fn ui_character_token(token: &str) -> bool {
    #[rustfmt::skip]
    const TOKENS: &[&str] = &[
        "apu", "bart", "barney",
        "bbman", // cspell:disable-line -- bbman
        "bookb", // cspell:disable-line -- bookb
        "burns",
        "cletu", // cspell:disable-line -- cletu
        "comic",
        "famil", // cspell:disable-line -- famil
        "frink", "gramp", "homer",
        "krust", // cspell:disable-line -- krust
        "lisa", "maggie", "marge",
        "milhouse", "moe", "otto", "ralph", "skinner", "snake", "willie",
    ];
    TOKENS.contains(&token)
}

/// Supports the `ui_status_token` operation within this deterministic
/// classification boundary.
fn ui_status_token(token: &str) -> bool {
    const TOKENS: &[&str] = &[
        "bonus",
        // cspell:disable-next-line -- blueflag
        "blueflag",
        "error",
        "exclamation",
        "star",
        "sparkles",
        "timer",
        "wasp",
        "wager",
    ];
    TOKENS.contains(&token)
}

/// Supports the `ui_glyph_token` operation within this deterministic
/// classification boundary.
fn ui_glyph_token(token: &str) -> bool {
    token
        .chars()
        .all(|character| character.is_ascii_digit())
        || token.starts_with("letter")
        || matches!(
            token,
            "colon"
                | "comma"
                | "dash"
                | "dot"
                | "hyphen"
                | "period"
                | "slash"
                | "space"
        )
        || token.len() == 1
            && token
                .chars()
                .all(|character| character.is_ascii_alphabetic())
}

#[cfg(test)]
mod tests {
    use super::{scene_resource_role, scene_resource_subcategory};

    #[test]
    fn classifies_frontend_scene_roles() {
        assert_eq!(
            scene_resource_role(&["camset"]),
            "camera-sets"
        );
        assert_eq!(
            scene_resource_role(&["l4hudmap"]),
            "hud-maps"
        );
        assert_eq!(
            // cspell:disable-next-line -- gaghomer
            scene_resource_role(&["gaghomer"]),
            "gag-scenes"
        );
        assert_eq!(
            // cspell:disable-next-line -- glowtv
            scene_resource_role(&["glowtv"]),
            "interactive-glows"
        );
        assert_eq!(
            // cspell:disable-next-line -- curtainl
            scene_resource_role(&["curtainl"]),
            "screen-transitions"
        );
        assert_eq!(
            // cspell:disable-next-line -- rewardbg
            scene_resource_role(&["rewardbg"]),
            "reward-presentation"
        );
    }

    #[test]
    fn classifies_scene_resources_as_frontend_scenes() {
        assert_eq!(
            scene_resource_subcategory(
                &[
                    "extracted",
                    "art",
                    "frontend",
                    "scrooby",
                    "resource"
                ],
                &[
                    "pure3d", "camset"
                ]
            ),
            Some(
                "ui-resources/frontend-scenes/camera-sets/sprite-layouts/\
                 camset"
                    .to_owned()
            )
        );
    }

    #[test]
    fn appends_exact_details_for_tokenized_resource_roles() {
        assert_eq!(
            super::ui_resource_detail(
                &[],
                &[
                    "frontend", "card12"
                ],
                "cards",
            ),
            "/card12".to_owned()
        );
        assert_eq!(
            super::ui_resource_detail(
                &[],
                &[
                    // cspell:disable-next-line -- qapu
                    "ingame", "qapu"
                ],
                "speaker-icons",
            ),
            // cspell:disable-next-line -- qapu
            "/qapu".to_owned()
        );
        assert_eq!(
            super::ui_resource_detail(
                &[],
                &[
                    "backend", "loading0"
                ],
                "loading",
            ),
            "/loading0".to_owned()
        );
        assert_eq!(
            super::ui_resource_detail(
                &["scrooby2"],
                &[
                    "txtbible", "srr2"
                ],
                "art-assets",
            ),
            "/scene-layouts/txtbible-srr2".to_owned()
        );
    }
}
