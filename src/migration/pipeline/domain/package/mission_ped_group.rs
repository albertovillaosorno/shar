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
//   - Source-backed pedestrian group declarations and canonical models.
// - Must-Not:
//   - Infer spawn frequency, navigation, or zone switching policy.
// - Allows:
//   - Validate Create/Add/Close structure and runtime capacity bounds.
//   - Bind each authored pedestrian model to character-package evidence.
// - Split-When:
//   - Pedestrian selection gains independent runtime lifecycle semantics.
// - Merge-When:
//   - Final level population compilation owns these exact declarations.
// - Summary:
//   - Pedestrian group semantic preflight.
// - Description:
//   - Compiles authored pedestrian groups without inventing spawn policy.
// - Usage:
//   - Runs on each normalized level script after scope compilation.
// - Defaults:
//   - Malformed, unsupported, or unresolved declarations fail closed.
//

//! Source-backed pedestrian group declarations.

use std::collections::BTreeSet;

use super::{
    MissionCharacterCatalogReference, MissionReferenceCatalog,
    MissionScopeReport,
};

const MAX_PED_GROUPS: u8 = 10;
const MAX_PED_MODELS: usize = 10;

/// One canonical pedestrian model member.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionPedGroupMemberBinding {
    source_ordinal: usize,
    source_model: String,
    max_instances: u32,
    character: MissionCharacterCatalogReference,
}

impl MissionPedGroupMemberBinding {
    /// Return the source AddPed command ordinal.
    #[must_use]
    pub const fn source_ordinal(&self) -> usize {
        self.source_ordinal
    }

    /// Return the exact authored pedestrian model identity.
    #[must_use]
    pub fn source_model(&self) -> &str {
        &self.source_model
    }

    /// Return the authored positive maximum instance count.
    #[must_use]
    pub const fn max_instances(&self) -> u32 {
        self.max_instances
    }

    /// Return the canonical character-package reference.
    #[must_use]
    pub const fn character(&self) -> &MissionCharacterCatalogReference {
        &self.character
    }
}

/// One complete authored pedestrian group.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionPedGroupBinding {
    group_index: u8,
    create_source_ordinal: usize,
    close_source_ordinal: usize,
    members: Vec<MissionPedGroupMemberBinding>,
}

impl MissionPedGroupBinding {
    /// Return the runtime pedestrian group index.
    #[must_use]
    pub const fn group_index(&self) -> u8 {
        self.group_index
    }

    /// Return the CreatePedGroup source ordinal.
    #[must_use]
    pub const fn create_source_ordinal(&self) -> usize {
        self.create_source_ordinal
    }

    /// Return the ClosePedGroup source ordinal.
    #[must_use]
    pub const fn close_source_ordinal(&self) -> usize {
        self.close_source_ordinal
    }

    /// Return canonical group members in authored order.
    #[must_use]
    pub fn members(&self) -> &[MissionPedGroupMemberBinding] {
        &self.members
    }
}

/// All pedestrian groups declared by one normalized source.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct MissionPedGroupReport {
    groups: Vec<MissionPedGroupBinding>,
}

impl MissionPedGroupReport {
    /// Return complete pedestrian groups in authored order.
    #[must_use]
    pub fn groups(&self) -> &[MissionPedGroupBinding] {
        &self.groups
    }
}

struct OpenPedGroup {
    group_index: u8,
    create_source_ordinal: usize,
    members: Vec<MissionPedGroupMemberBinding>,
}

/// Compile pedestrian group declarations and bind their character models.
///
/// # Errors
///
/// Fails on malformed command shape/order, runtime capacity overflow,
/// duplicate group definitions, or unresolved character-package evidence.
pub fn preflight_mission_ped_groups(
    catalog: &MissionReferenceCatalog,
    scopes: &MissionScopeReport,
) -> Result<MissionPedGroupReport, String> {
    let mut groups = Vec::new();
    let mut open = None;
    let mut defined = BTreeSet::new();

    for command in scopes.unscoped_commands() {
        match command.name() {
            "createpedgroup" => {
                if open.is_some() {
                    return Err(
                        "ped group create nested before close".to_owned(),
                    );
                }
                require_role(command.semantic_role(), "ped group create")?;
                let [index] = command.arguments() else {
                    return Err(
                        "CreatePedGroup must have one argument".to_owned(),
                    );
                };
                let group_index = parse_group_index(index)?;
                if !defined.insert(group_index) {
                    return Err(
                        "ped group index is defined more than once".to_owned(),
                    );
                }
                open = Some(OpenPedGroup {
                    group_index,
                    create_source_ordinal: command.source_ordinal(),
                    members: Vec::new(),
                });
            }
            "addped" => {
                require_role(command.semantic_role(), "ped group member")?;
                let Some(group) = open.as_mut() else {
                    return Err("AddPed appears outside a ped group".to_owned());
                };
                let [model, max_instances] = command.arguments() else {
                    return Err("AddPed must have two arguments".to_owned());
                };
                if group.members.len() >= MAX_PED_MODELS {
                    return Err(
                        "ped group exceeds runtime model capacity".to_owned(),
                    );
                }
                let source_model = source_token(model, "ped model")?;
                let max_instances =
                    parse_positive(max_instances, "ped max instances")?;
                let character = catalog.resolve_character(&source_model)?;
                group.members.push(MissionPedGroupMemberBinding {
                    source_ordinal: command.source_ordinal(),
                    source_model,
                    max_instances,
                    character,
                });
            }
            "closepedgroup" => {
                require_role(command.semantic_role(), "ped group close")?;
                if !command.arguments().is_empty() {
                    return Err(
                        "ClosePedGroup must have no arguments".to_owned(),
                    );
                }
                let Some(group) = open.take() else {
                    return Err(
                        "ClosePedGroup appears without create".to_owned(),
                    );
                };
                if group.members.is_empty() {
                    return Err("ped group cannot be empty".to_owned());
                }
                groups.push(MissionPedGroupBinding {
                    group_index: group.group_index,
                    create_source_ordinal: group.create_source_ordinal,
                    close_source_ordinal: command.source_ordinal(),
                    members: group.members,
                });
            }
            _ => {}
        }
    }
    if open.is_some() {
        return Err("ped group remains open at end of source".to_owned());
    }
    Ok(MissionPedGroupReport { groups })
}

fn require_role(actual: &str, label: &str) -> Result<(), String> {
    if actual != "mission-script" {
        return Err(format!("{label} semantic role changed"));
    }
    Ok(())
}

fn parse_group_index(value: &str) -> Result<u8, String> {
    let index = value
        .parse::<u8>()
        .map_err(|_| "ped group index is not unsigned".to_owned())?;
    if index >= MAX_PED_GROUPS {
        return Err("ped group index exceeds runtime capacity".to_owned());
    }
    Ok(index)
}

fn parse_positive(value: &str, label: &str) -> Result<u32, String> {
    let parsed = value
        .parse::<u32>()
        .map_err(|_| format!("{label} is not unsigned"))?;
    if parsed == 0 {
        return Err(format!("{label} must be positive"));
    }
    Ok(parsed)
}

fn source_token(value: &str, label: &str) -> Result<String, String> {
    let token = value.trim();
    if token.is_empty() || token.chars().any(char::is_control) {
        return Err(format!("{label} is malformed"));
    }
    Ok(token.to_owned())
}

#[cfg(test)]
// jig-ignore-next-line: exact Rust test-module path is indivisible.
#[path = "../../../../../tests/migration/pipeline/unit/domain/package/mission_ped_group/tests.rs"]
mod tests;
