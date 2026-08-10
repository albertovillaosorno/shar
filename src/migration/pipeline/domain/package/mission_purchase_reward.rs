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
//   - Source-backed setup for `AddPurchaseCarReward` level storefront actions.
// - Must-Not:
//   - Infer merchandise, price, ownership, unlock, or save-game behavior.
//   - Resolve level-scoped locator names without level inventory context.
// - Allows:
//   - Bind the reward NPC to canonical character package evidence.
//   - Preserve exact action, choreo, locator, and radius source evidence.
// - Split-When:
//   - Level-store locator binding or purchase transactions gain own schemas.
// - Merge-When:
//   - Final reward compilation owns this exact storefront setup boundary.
// - Summary:
//   - Purchase-car reward storefront preflight.
//

//! Source-backed `AddPurchaseCarReward` storefront setup.

use super::{
    MissionCharacterCatalogReference, MissionReferenceCatalog,
    MissionScopeReport,
};

/// Source-observed seller selected by the purchase action locator identity.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MissionPurchaseRewardSeller {
    /// The source `gil` action resolves the level vendor.
    GilVendor,
    /// The source `simpson` action resolves the level playable character.
    LevelPlayableCharacter,
}

/// One reviewed `AddPurchaseCarReward` storefront setup.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionPurchaseRewardBinding {
    source_ordinal: usize,
    action_locator_id: String,
    seller: MissionPurchaseRewardSeller,
    reward_character_id: String,
    character: MissionCharacterCatalogReference,
    choreo_id: String,
    position_locator_id: String,
    trigger_radius_source: String,
    car_start_locator_id: String,
}

impl MissionPurchaseRewardBinding {
    /// Return the source statement ordinal.
    #[must_use]
    pub const fn source_ordinal(&self) -> usize {
        self.source_ordinal
    }

    /// Return the exact authored action-locator identity.
    #[must_use]
    pub fn action_locator_id(&self) -> &str {
        &self.action_locator_id
    }

    /// Return the source-backed seller selection.
    #[must_use]
    pub const fn seller(&self) -> MissionPurchaseRewardSeller {
        self.seller
    }

    /// Return the runtime reward-NPC identity derived by the source loader.
    #[must_use]
    pub fn reward_character_id(&self) -> &str {
        &self.reward_character_id
    }

    /// Return the canonical source character package reference.
    #[must_use]
    pub const fn character(&self) -> &MissionCharacterCatalogReference {
        &self.character
    }

    /// Return the exact authored reward-NPC choreo token.
    #[must_use]
    pub fn choreo_id(&self) -> &str {
        &self.choreo_id
    }

    /// Return the exact position locator used by the reward NPC.
    #[must_use]
    pub fn position_locator_id(&self) -> &str {
        &self.position_locator_id
    }

    /// Return the exact positive trigger-radius source lexeme.
    #[must_use]
    pub fn trigger_radius_source(&self) -> &str {
        &self.trigger_radius_source
    }

    /// Return the exact authored car-start locator identity.
    #[must_use]
    pub fn car_start_locator_id(&self) -> &str {
        &self.car_start_locator_id
    }
}

/// All reviewed purchase-car storefront bindings for one normalized source.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionPurchaseRewardReport {
    bindings: Vec<MissionPurchaseRewardBinding>,
}

impl MissionPurchaseRewardReport {
    /// Return bindings in source statement order.
    #[must_use]
    pub fn bindings(&self) -> &[MissionPurchaseRewardBinding] {
        &self.bindings
    }
}

/// Compile every unscoped source `AddPurchaseCarReward` command.
///
/// The original loader creates one `PurchaseCar` action locator, a reward NPC,
/// one spherical trigger, and stores a second locator for the car-start target.
/// This preflight preserves that source-backed setup without assigning a price,
/// merchandise entry, ownership state, or save-game behavior.
///
/// # Errors
///
/// Returns an error for role/arity drift, unreviewed action identities,
/// malformed tokens or trigger radius, or a missing/ambiguous character package.
pub fn preflight_mission_purchase_rewards(
    catalog: &MissionReferenceCatalog,
    scopes: &MissionScopeReport,
) -> Result<MissionPurchaseRewardReport, String> {
    let mut bindings = Vec::new();
    for command in scopes
        .unscoped_commands()
        .iter()
        .filter(|command| command.name() == "addpurchasecarreward")
    {
        if command.semantic_role() != "mission-reward" {
            return Err(
                "AddPurchaseCarReward semantic role changed".to_owned()
            );
        }
        push_binding(
            &mut bindings,
            catalog,
            command.source_ordinal(),
            command.arguments(),
        )?;
    }
    Ok(MissionPurchaseRewardReport { bindings })
}

fn push_binding(
    bindings: &mut Vec<MissionPurchaseRewardBinding>,
    catalog: &MissionReferenceCatalog,
    source_ordinal: usize,
    arguments: &[String],
) -> Result<(), String> {
    let [action, character, choreo, position, radius, car_start] = arguments
    else {
        return Err(
            "AddPurchaseCarReward must have six arguments".to_owned()
        );
    };
    let action_locator_id = required_token(action, "action locator")?;
    let seller = match action_locator_id.as_str() {
        "gil" => MissionPurchaseRewardSeller::GilVendor,
        "simpson" => MissionPurchaseRewardSeller::LevelPlayableCharacter,
        _ => {
            return Err(
                "AddPurchaseCarReward action locator is not reviewed".to_owned()
            );
        },
    };
    let character_id = required_token(character, "reward character")?;
    let character = catalog.resolve_character(&character_id)?;
    let choreo_id = required_token(choreo, "reward choreo")?;
    let position_locator_id = required_token(position, "position locator")?;
    let trigger_radius_source = required_radius(radius)?;
    let car_start_locator_id = required_token(car_start, "car-start locator")?;
    let reward_character_id = format!("reward_{character_id}");
    bindings.push(MissionPurchaseRewardBinding {
        source_ordinal,
        action_locator_id,
        seller,
        reward_character_id,
        character,
        choreo_id,
        position_locator_id,
        trigger_radius_source,
        car_start_locator_id,
    });
    Ok(())
}

fn required_token(value: &str, role: &str) -> Result<String, String> {
    if value.is_empty()
        || value != value.trim()
        || value.chars().any(char::is_control)
    {
        return Err(format!(
            "AddPurchaseCarReward {role} is malformed"
        ));
    }
    Ok(value.to_owned())
}

fn required_radius(value: &str) -> Result<String, String> {
    let source = required_token(value, "trigger radius")?;
    let parsed = source.parse::<f32>().map_err(|_error| {
        "AddPurchaseCarReward trigger radius is not a decimal".to_owned()
    })?;
    if !parsed.is_finite() || parsed <= 0.0 {
        return Err(
            "AddPurchaseCarReward trigger radius must be finite and positive"
                .to_owned(),
        );
    }
    Ok(source)
}

#[cfg(test)]
#[path = "../../../../../tests/migration/pipeline/unit/domain/package/mission_purchase_reward/tests.rs"]
mod tests;
