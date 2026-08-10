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
//   - Static level-init locator references backed by explicit package context.
// - Must-Not:
//   - Resolve purchase car-start locators consumed after level-init execution.
//   - Infer generic Locator precedence across decoded locator subtypes.
// - Allows:
//   - Resolve immediate generic Locator lookups conservatively.
//   - Apply authored package order to exact CarStart dialogue lookups.
// - Split-When:
//   - Deferred level locator lifecycle gains authoritative runtime history.
// - Merge-When:
//   - Final level setup compilation owns these exact locator references.
// - Summary:
//   - Level-init locator reference preflight.
// - Description:
//   - Binds immediate NPC, storefront, waypoint, and dialogue locators while
//     keeping deferred purchase car-start lookup outside static context.
// - Usage:
//   - Runs after level NPC/storefront preflight and sibling-load composition.
// - Defaults:
//   - Missing and ambiguous locator identities remain typed evidence.
//

//! Static level-init locator reference binding.

use super::{
    MissionLevelNpcKind, MissionLevelNpcReport, MissionLocatorCatalog,
    MissionLocatorResolution, MissionLocatorTypeConstraint,
    MissionPurchaseRewardReport, MissionScopeReport,
};

const CAR_START_LOCATOR_TYPE: u32 = 3;

/// Source-backed level locator lookup role.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MissionLevelLocatorRole {
    /// Initial level player-vehicle placement.
    LevelPlayerVehicle,
    /// Ambient NPC initial position.
    AmbientSpawn,
    /// Bonus/street-race NPC initial position.
    BonusSpawn,
    /// Ambient NPC waypoint.
    AmbientWaypoint,
    /// Bonus/street-race NPC waypoint.
    BonusWaypoint,
    /// Purchase reward NPC/storefront position.
    PurchasePosition,
    /// Purchase reward NPC waypoint.
    PurchaseWaypoint,
    /// Player dialogue position.
    BonusDialoguePlayer,
    /// NPC dialogue position.
    BonusDialogueNpc,
    /// Vehicle dialogue position.
    BonusDialogueVehicle,
}

impl MissionLevelLocatorRole {
    const fn exact_type(self) -> Option<u32> {
        match self {
            Self::LevelPlayerVehicle
            | Self::BonusDialoguePlayer
            | Self::BonusDialogueNpc
            | Self::BonusDialogueVehicle => Some(CAR_START_LOCATOR_TYPE),
            Self::AmbientSpawn
            | Self::BonusSpawn
            | Self::AmbientWaypoint
            | Self::BonusWaypoint
            | Self::PurchasePosition
            | Self::PurchaseWaypoint => None,
        }
    }
}

/// One immediate level-init locator reference and its canonical outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionLevelLocatorReferenceBinding {
    source_ordinal: usize,
    role: MissionLevelLocatorRole,
    source_name: String,
    resolution: MissionLocatorResolution,
}

impl MissionLevelLocatorReferenceBinding {
    /// Return the source command ordinal.
    #[must_use]
    pub const fn source_ordinal(&self) -> usize {
        self.source_ordinal
    }

    /// Return the source-backed runtime lookup role.
    #[must_use]
    pub const fn role(&self) -> MissionLevelLocatorRole {
        self.role
    }

    /// Return the exact authored locator identity.
    #[must_use]
    pub fn source_name(&self) -> &str {
        &self.source_name
    }

    /// Return the package-backed canonical resolution outcome.
    #[must_use]
    pub const fn resolution(&self) -> &MissionLocatorResolution {
        &self.resolution
    }
}

/// All immediate level-init locator references for one setup source.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionLevelLocatorReferenceReport {
    bindings: Vec<MissionLevelLocatorReferenceBinding>,
}

impl MissionLevelLocatorReferenceReport {
    /// Return locator references in source-derived traversal order.
    #[must_use]
    pub fn bindings(&self) -> &[MissionLevelLocatorReferenceBinding] {
        &self.bindings
    }
}

/// Bind immediate level-init locator references against static package roots.
///
/// `AddPurchaseCarReward` car-start identities are intentionally excluded:
/// runtime consumes those only after a later vehicle load callback.
///
/// # Errors
///
/// Returns an error when a source identity or package root is malformed.
pub fn preflight_mission_level_locator_references(
    catalog: &MissionLocatorCatalog,
    ordered_package_roots: &[String],
    scopes: &MissionScopeReport,
    npcs: &MissionLevelNpcReport,
    purchases: &MissionPurchaseRewardReport,
) -> Result<MissionLevelLocatorReferenceReport, String> {
    let mut bindings = Vec::new();
    push_level_vehicle_references(
        &mut bindings,
        catalog,
        ordered_package_roots,
        scopes,
    )?;
    for declaration in npcs.declarations() {
        let role = match declaration.kind() {
            MissionLevelNpcKind::Ambient => {
                MissionLevelLocatorRole::AmbientSpawn
            },
            MissionLevelNpcKind::BonusMission => {
                MissionLevelLocatorRole::BonusSpawn
            },
        };
        push_reference(
            &mut bindings,
            catalog,
            ordered_package_roots,
            declaration.source_ordinal(),
            role,
            declaration.locator_id(),
        )?;
    }
    for waypoint in npcs.waypoints() {
        let role = match waypoint.kind() {
            MissionLevelNpcKind::Ambient => {
                MissionLevelLocatorRole::AmbientWaypoint
            },
            MissionLevelNpcKind::BonusMission => {
                MissionLevelLocatorRole::BonusWaypoint
            },
        };
        push_reference(
            &mut bindings,
            catalog,
            ordered_package_roots,
            waypoint.source_ordinal(),
            role,
            waypoint.locator_id(),
        )?;
    }
    for dialogue in npcs.dialogue_locators() {
        for (role, source_name) in [
            (
                MissionLevelLocatorRole::BonusDialoguePlayer,
                dialogue.player_locator_id(),
            ),
            (
                MissionLevelLocatorRole::BonusDialogueNpc,
                dialogue.npc_locator_id(),
            ),
            (
                MissionLevelLocatorRole::BonusDialogueVehicle,
                dialogue.vehicle_locator_id(),
            ),
        ] {
            push_reference(
                &mut bindings,
                catalog,
                ordered_package_roots,
                dialogue.source_ordinal(),
                role,
                source_name,
            )?;
        }
    }
    for purchase in purchases.bindings() {
        push_reference(
            &mut bindings,
            catalog,
            ordered_package_roots,
            purchase.source_ordinal(),
            MissionLevelLocatorRole::PurchasePosition,
            purchase.position_locator_id(),
        )?;
    }
    for waypoint in purchases.waypoints() {
        push_reference(
            &mut bindings,
            catalog,
            ordered_package_roots,
            waypoint.source_ordinal(),
            MissionLevelLocatorRole::PurchaseWaypoint,
            waypoint.waypoint_locator_id(),
        )?;
    }
    bindings.sort_by_key(|binding| binding.source_ordinal);
    Ok(MissionLevelLocatorReferenceReport { bindings })
}

fn push_level_vehicle_references(
    out: &mut Vec<MissionLevelLocatorReferenceBinding>,
    catalog: &MissionLocatorCatalog,
    ordered_package_roots: &[String],
    scopes: &MissionScopeReport,
) -> Result<(), String> {
    for command in scopes.unscoped_commands() {
        if command.name() != "initlevelplayervehicle" {
            continue;
        }
        if command.semantic_role() != "mission-script" {
            return Err(
                "level player vehicle semantic role changed".to_owned(),
            );
        }
        let arguments = command.arguments();
        if arguments.len() != 3 && arguments.len() != 4 {
            return Err(
                "InitLevelPlayerVehicle argument shape changed".to_owned(),
            );
        }
        let locator = source_token(
            &arguments[1],
            "level player vehicle locator",
        )?;
        push_reference(
            out,
            catalog,
            ordered_package_roots,
            command.source_ordinal(),
            MissionLevelLocatorRole::LevelPlayerVehicle,
            &locator,
        )?;
    }
    Ok(())
}

fn source_token(value: &str, label: &str) -> Result<String, String> {
    let token = value.trim();
    if token.is_empty() || token.chars().any(char::is_control) {
        return Err(format!("{label} is malformed"));
    }
    Ok(token.to_owned())
}

fn push_reference(
    out: &mut Vec<MissionLevelLocatorReferenceBinding>,
    catalog: &MissionLocatorCatalog,
    ordered_package_roots: &[String],
    source_ordinal: usize,
    role: MissionLevelLocatorRole,
    source_name: &str,
) -> Result<(), String> {
    let resolution = if let Some(locator_type) = role.exact_type() {
        catalog.resolve_in_package_order(
            source_name,
            ordered_package_roots,
            MissionLocatorTypeConstraint::Exact(locator_type),
        )?
    } else {
        catalog.resolve(
            source_name,
            ordered_package_roots,
            MissionLocatorTypeConstraint::Any,
        )?
    };
    out.push(MissionLevelLocatorReferenceBinding {
        source_ordinal,
        role,
        source_name: source_name.to_owned(),
        resolution,
    });
    Ok(())
}


#[cfg(test)]
// jig-ignore-next-line: exact Rust test-module path is indivisible.
#[path = "../../../../../tests/migration/pipeline/unit/domain/package/mission_level_locator_reference/tests.rs"]
mod tests;
