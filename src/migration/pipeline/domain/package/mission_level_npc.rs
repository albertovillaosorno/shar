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
//   - Source-backed ambient and bonus-mission NPC declarations and waypoints.
// - Must-Not:
//   - Resolve level locators without level inventory context.
//   - Infer navigation, dialogue, reward, or progression behavior.
// - Allows:
//   - Bind authored NPC models to canonical character package evidence.
//   - Preserve source-derived runtime names and exact level locator identities.
// - Split-When:
//   - Level-locator binding or NPC navigation gains an independent schema.
// - Merge-When:
//   - Final level mission compilation owns this exact NPC setup boundary.
// - Summary:
//   - Level NPC declaration and waypoint preflight.
// - Description:
//   - Preserves source NPC setup and prior-declaration waypoint relationships.
// - Usage:
//   - Runs after mission scope projection and character catalog creation.
// - Defaults:
//   - Malformed, unresolved, reordered, or ambiguous NPC setup fails closed.
//

//! Source-backed ambient and bonus-mission NPC setup.

use super::{
    MissionCharacterCatalogReference, MissionReferenceCatalog,
    MissionScopeReport,
};

/// Source-backed level NPC family.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MissionLevelNpcKind {
    /// Ambient free-roam NPC.
    Ambient,
    /// Bonus-mission or street-race NPC.
    BonusMission,
}

/// One reviewed level NPC declaration.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionLevelNpcBinding {
    source_ordinal: usize,
    kind: MissionLevelNpcKind,
    source_character_id: String,
    runtime_character_id: String,
    character: MissionCharacterCatalogReference,
    choreo_id: String,
    locator_id: String,
    ambient_radius_source: Option<String>,
    bonus_mission_id: Option<String>,
    bonus_icon_id: Option<String>,
    bonus_dialogue_id: Option<String>,
    bonus_is_race: Option<bool>,
    bonus_alternate_icon_id: Option<String>,
}

impl MissionLevelNpcBinding {
    /// Return the declaration source ordinal.
    #[must_use]
    pub const fn source_ordinal(&self) -> usize { self.source_ordinal }
    /// Return the NPC family.
    #[must_use]
    pub const fn kind(&self) -> MissionLevelNpcKind { self.kind }
    /// Return the authored character/model identity.
    #[must_use]
    pub fn source_character_id(&self) -> &str { &self.source_character_id }
    /// Return the runtime character identity derived by the source loader.
    #[must_use]
    pub fn runtime_character_id(&self) -> &str { &self.runtime_character_id }
    /// Return canonical character package evidence.
    #[must_use]
    pub const fn character(&self) -> &MissionCharacterCatalogReference {
        &self.character
    }
    /// Return the exact choreo identity.
    #[must_use]
    pub fn choreo_id(&self) -> &str { &self.choreo_id }
    /// Return the exact source spawn locator identity.
    #[must_use]
    pub fn locator_id(&self) -> &str { &self.locator_id }
    /// Return the optional exact ambient radius lexeme.
    #[must_use]
    pub fn ambient_radius_source(&self) -> Option<&str> {
        self.ambient_radius_source.as_deref()
    }
    /// Return the bonus mission identity, when applicable.
    #[must_use]
    pub fn bonus_mission_id(&self) -> Option<&str> {
        self.bonus_mission_id.as_deref()
    }
    /// Return the bonus icon identity, when applicable.
    #[must_use]
    pub fn bonus_icon_id(&self) -> Option<&str> {
        self.bonus_icon_id.as_deref()
    }
    /// Return the bonus dialogue identity, when applicable.
    #[must_use]
    pub fn bonus_dialogue_id(&self) -> Option<&str> {
        self.bonus_dialogue_id.as_deref()
    }
    /// Return the reviewed source race flag, when applicable.
    #[must_use]
    pub const fn bonus_is_race(&self) -> Option<bool> { self.bonus_is_race }
    /// Return the optional alternate bonus icon identity.
    #[must_use]
    pub fn bonus_alternate_icon_id(&self) -> Option<&str> {
        self.bonus_alternate_icon_id.as_deref()
    }
}

/// One waypoint attached to a previously declared level NPC.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionLevelNpcWaypointBinding {
    source_ordinal: usize,
    declaration_source_ordinal: usize,
    kind: MissionLevelNpcKind,
    source_character_id: String,
    runtime_character_id: String,
    locator_id: String,
}

impl MissionLevelNpcWaypointBinding {
    /// Return the waypoint source ordinal.
    #[must_use]
    pub const fn source_ordinal(&self) -> usize { self.source_ordinal }
    /// Return the matched declaration source ordinal.
    #[must_use]
    pub const fn declaration_source_ordinal(&self) -> usize {
        self.declaration_source_ordinal
    }
    /// Return the NPC family.
    #[must_use]
    pub const fn kind(&self) -> MissionLevelNpcKind { self.kind }
    /// Return the authored character identity.
    #[must_use]
    pub fn source_character_id(&self) -> &str { &self.source_character_id }
    /// Return the source-derived runtime character identity.
    #[must_use]
    pub fn runtime_character_id(&self) -> &str { &self.runtime_character_id }
    /// Return the exact level waypoint locator identity.
    #[must_use]
    pub fn locator_id(&self) -> &str { &self.locator_id }
}

/// All reviewed level NPC setup for one normalized source.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionLevelNpcReport {
    declarations: Vec<MissionLevelNpcBinding>,
    waypoints: Vec<MissionLevelNpcWaypointBinding>,
}

impl MissionLevelNpcReport {
    /// Return declarations in source order.
    #[must_use]
    pub fn declarations(&self) -> &[MissionLevelNpcBinding] {
        &self.declarations
    }
    /// Return waypoints in source order.
    #[must_use]
    pub fn waypoints(&self) -> &[MissionLevelNpcWaypointBinding] {
        &self.waypoints
    }
}

/// Compile ambient and bonus-mission NPC setup from unscoped level commands.
///
/// # Errors
///
/// Returns an error for role/shape drift, malformed values, unresolved
/// character models, or a waypoint without one unique prior matching NPC.
pub fn preflight_mission_level_npcs(
    catalog: &MissionReferenceCatalog,
    scopes: &MissionScopeReport,
) -> Result<MissionLevelNpcReport, String> {
    let mut declarations = Vec::new();
    for command in scopes.unscoped_commands() {
        if command.semantic_role() != "mission-script" {
            continue;
        }
        match command.name() {
            "addambientcharacter" => push_ambient(
                &mut declarations,
                catalog,
                command.source_ordinal(),
                command.arguments(),
            )?,
            "addnpccharacterbonusmission" => push_bonus(
                &mut declarations,
                catalog,
                command.source_ordinal(),
                command.arguments(),
            )?,
            _ => {},
        }
    }

    let mut waypoints = Vec::new();
    for command in scopes.unscoped_commands() {
        let kind = match command.name() {
            "addambientnpcwaypoint" => MissionLevelNpcKind::Ambient,
            "addbonusmissionnpcwaypoint" => MissionLevelNpcKind::BonusMission,
            _ => continue,
        };
        if command.semantic_role() != "mission-script" {
            return Err("level NPC waypoint semantic role changed".to_owned());
        }
        push_waypoint(
            &mut waypoints,
            &declarations,
            kind,
            command.source_ordinal(),
            command.arguments(),
        )?;
    }
    Ok(MissionLevelNpcReport { declarations, waypoints })
}

fn push_ambient(
    out: &mut Vec<MissionLevelNpcBinding>,
    catalog: &MissionReferenceCatalog,
    source_ordinal: usize,
    arguments: &[String],
) -> Result<(), String> {
    let (character, locator, radius) = match arguments {
        [character, locator] => (character, locator, None),
        [character, locator, radius] => (character, locator, Some(radius)),
        _ => {
            return Err(
                "AddAmbientCharacter must have two or three arguments"
                    .to_owned(),
            );
        },
    };
    let source_character_id = token(character, "ambient character")?;
    let character = catalog.resolve_character(&source_character_id)?;
    let locator_id = token(locator, "ambient spawn locator")?;
    let ambient_radius_source = radius
        .map(|value| finite_decimal(value, "ambient radius"))
        .transpose()?;
    out.push(MissionLevelNpcBinding {
        source_ordinal,
        kind: MissionLevelNpcKind::Ambient,
        runtime_character_id: source_character_id.clone(),
        source_character_id,
        character,
        choreo_id: "npd".to_owned(),
        locator_id,
        ambient_radius_source,
        bonus_mission_id: None,
        bonus_icon_id: None,
        bonus_dialogue_id: None,
        bonus_is_race: None,
        bonus_alternate_icon_id: None,
    });
    Ok(())
}

fn push_bonus(
    out: &mut Vec<MissionLevelNpcBinding>,
    catalog: &MissionReferenceCatalog,
    source_ordinal: usize,
    arguments: &[String],
) -> Result<(), String> {
    let (
        character, choreo, locator, mission, icon, dialogue, is_race, alternate,
    ) =
        match arguments {
            [a, b, c, d, e, f, g] => (a, b, c, d, e, f, g, None),
            [a, b, c, d, e, f, g, h] => (a, b, c, d, e, f, g, Some(h)),
            _ => {
                return Err(
                    concat!(
                        "AddNPCCharacterBonusMission must have seven or ",
                        "eight arguments",
                    )
                    .to_owned(),
                );
            },
        };
    let source_character_id = token(character, "bonus character")?;
    let character = catalog.resolve_character(&source_character_id)?;
    let bonus_is_race = match is_race.as_str() {
        "0" => false,
        "1" => true,
        _ => return Err("bonus mission race flag is not reviewed".to_owned()),
    };
    out.push(MissionLevelNpcBinding {
        source_ordinal,
        kind: MissionLevelNpcKind::BonusMission,
        runtime_character_id: format!("b_{source_character_id}"),
        source_character_id,
        character,
        choreo_id: token(choreo, "bonus choreo")?,
        locator_id: token(locator, "bonus spawn locator")?,
        ambient_radius_source: None,
        bonus_mission_id: Some(token(mission, "bonus mission")?),
        bonus_icon_id: Some(token(icon, "bonus icon")?),
        bonus_dialogue_id: Some(token(dialogue, "bonus dialogue")?),
        bonus_is_race: Some(bonus_is_race),
        bonus_alternate_icon_id: alternate
            .map(|value| token(value, "bonus alternate icon"))
            .transpose()?,
    });
    Ok(())
}

fn push_waypoint(
    out: &mut Vec<MissionLevelNpcWaypointBinding>,
    declarations: &[MissionLevelNpcBinding],
    kind: MissionLevelNpcKind,
    source_ordinal: usize,
    arguments: &[String],
) -> Result<(), String> {
    let [character, locator] = arguments else {
        return Err("level NPC waypoint must have two arguments".to_owned());
    };
    let source_character_id = token(character, "waypoint character")?;
    let locator_id = token(locator, "waypoint locator")?;
    let matching = declarations
        .iter()
        .filter(|item| {
            item.kind() == kind
                && item.source_ordinal() < source_ordinal
                && item.source_character_id() == source_character_id
        })
        .collect::<Vec<_>>();
    let [declaration] = matching.as_slice() else {
        return Err(
            "level NPC waypoint has no unique prior declaration".to_owned(),
        );
    };
    out.push(MissionLevelNpcWaypointBinding {
        source_ordinal,
        declaration_source_ordinal: declaration.source_ordinal(),
        kind,
        source_character_id,
        runtime_character_id: declaration.runtime_character_id().to_owned(),
        locator_id,
    });
    Ok(())
}

fn token(value: &str, role: &str) -> Result<String, String> {
    if value.is_empty()
        || value != value.trim()
        || value.chars().any(char::is_control)
    {
        return Err(format!("level NPC {role} is malformed"));
    }
    Ok(value.to_owned())
}

fn finite_decimal(value: &str, role: &str) -> Result<String, String> {
    let source = token(value, role)?;
    let parsed = source
        .parse::<f32>()
        .map_err(|_error| format!("level NPC {role} is not a decimal"))?;
    if !parsed.is_finite() || parsed < 0.0 {
        return Err(format!(
            "level NPC {role} must be finite and non-negative"
        ));
    }
    Ok(source)
}

#[cfg(test)]
// jig-ignore-next-line: exact Rust test-module path is indivisible.
#[path = "../../../../../tests/migration/pipeline/unit/domain/package/mission_level_npc/tests.rs"]
mod tests;
