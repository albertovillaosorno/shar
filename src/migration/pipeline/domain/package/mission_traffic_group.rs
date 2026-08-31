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
//   - Source-backed traffic model-group declarations and canonical vehicles.
// - Must-Not:
//   - Infer traffic spawn frequency, parked-car policy, or zone switching.
// - Allows:
//   - Validate Create/Add/Close structure and runtime capacity bounds.
//   - Preserve the optional big-vehicle flag and canonical vehicle package.
// - Split-When:
//   - Active traffic-group selection gains independent lifecycle semantics.
// - Merge-When:
//   - Final level population compilation owns these exact declarations.
// - Summary:
//   - Traffic group semantic preflight.
// - Description:
//   - Compiles authored traffic groups without inventing spawn policy.
// - Usage:
//   - Runs on each normalized level script after scope compilation.
// - Defaults:
//   - Malformed, unsupported, or unresolved declarations fail closed.
//

//! Source-backed traffic model-group declarations.

use std::collections::BTreeSet;

use super::{
    MissionReferenceCatalog, MissionScopeReport, MissionVehicleCatalogReference,
    MissionVehicleReference,
};

const MAX_TRAFFIC_GROUPS: u8 = 10;
const MAX_TRAFFIC_MODELS: usize = 5;

/// One canonical traffic vehicle member.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionTrafficGroupMemberBinding {
    source_ordinal: usize,
    source_model: String,
    max_instances: u32,
    big_flag: Option<i32>,
    vehicle: MissionVehicleCatalogReference,
}

impl MissionTrafficGroupMemberBinding {
    /// Return the source `AddTrafficModel` command ordinal.
    #[must_use]
    pub const fn source_ordinal(&self) -> usize {
        self.source_ordinal
    }

    /// Return the exact authored traffic vehicle identity.
    #[must_use]
    pub fn source_model(&self) -> &str {
        &self.source_model
    }

    /// Return the authored positive maximum instance count.
    #[must_use]
    pub const fn max_instances(&self) -> u32 {
        self.max_instances
    }

    /// Return the optional exact numeric big-vehicle flag.
    #[must_use]
    pub const fn big_flag(&self) -> Option<i32> {
        self.big_flag
    }

    /// Return the runtime big-vehicle interpretation of the optional flag.
    #[must_use]
    pub fn is_big(&self) -> bool {
        self.big_flag == Some(1)
    }

    /// Return the canonical vehicle-package reference.
    #[must_use]
    pub const fn vehicle(&self) -> &MissionVehicleCatalogReference {
        &self.vehicle
    }
}

/// One complete authored traffic model group.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionTrafficGroupBinding {
    group_index: u8,
    create_source_ordinal: usize,
    close_source_ordinal: usize,
    members: Vec<MissionTrafficGroupMemberBinding>,
}

impl MissionTrafficGroupBinding {
    /// Return the runtime traffic model-group index.
    #[must_use]
    pub const fn group_index(&self) -> u8 {
        self.group_index
    }

    /// Return the `CreateTrafficGroup` source ordinal.
    #[must_use]
    pub const fn create_source_ordinal(&self) -> usize {
        self.create_source_ordinal
    }

    /// Return the `CloseTrafficGroup` source ordinal.
    #[must_use]
    pub const fn close_source_ordinal(&self) -> usize {
        self.close_source_ordinal
    }

    /// Return canonical traffic members in authored order.
    #[must_use]
    pub fn members(&self) -> &[MissionTrafficGroupMemberBinding] {
        &self.members
    }
}

/// All traffic model groups declared by one normalized source.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct MissionTrafficGroupReport {
    groups: Vec<MissionTrafficGroupBinding>,
}

impl MissionTrafficGroupReport {
    /// Return complete traffic groups in authored order.
    #[must_use]
    pub fn groups(&self) -> &[MissionTrafficGroupBinding] {
        &self.groups
    }
}

struct OpenTrafficGroup {
    group_index: u8,
    create_source_ordinal: usize,
    members: Vec<MissionTrafficGroupMemberBinding>,
}

/// Compile traffic model-group declarations and bind their vehicle packages.
///
/// # Errors
///
/// Fails on malformed command shape/order, runtime capacity overflow,
/// duplicate group definitions, or unresolved vehicle-package evidence.
pub fn preflight_mission_traffic_groups(
    catalog: &MissionReferenceCatalog,
    scopes: &MissionScopeReport,
) -> Result<MissionTrafficGroupReport, String> {
    let mut groups = Vec::new();
    let mut open = None;
    let mut defined = BTreeSet::new();

    for command in scopes.unscoped_commands() {
        match command.name() {
            "createtrafficgroup" => {
                if open.is_some() {
                    return Err(
                        "traffic group create nested before close".to_owned(),
                    );
                }
                require_role(command.semantic_role(), "traffic group create")?;
                let [index] = command.arguments() else {
                    return Err(
                        "CreateTrafficGroup must have one argument".to_owned(),
                    );
                };
                let group_index = parse_group_index(index)?;
                if !defined.insert(group_index) {
                    return Err(
                        "traffic group index is defined more than once"
                            .to_owned(),
                    );
                }
                open = Some(OpenTrafficGroup {
                    group_index,
                    create_source_ordinal: command.source_ordinal(),
                    members: Vec::new(),
                });
            }
            "addtrafficmodel" => {
                require_role(command.semantic_role(), "traffic group member")?;
                let Some(group) = open.as_mut() else {
                    return Err(
                        "AddTrafficModel appears outside a traffic group"
                            .to_owned(),
                    );
                };
                if group.members.len() >= MAX_TRAFFIC_MODELS {
                    return Err(
                        "traffic group exceeds runtime model capacity"
                            .to_owned(),
                    );
                }
                let (model, max_instances, big_flag) =
                    parse_member_arguments(command.arguments())?;
                let source_model = source_token(model, "traffic model")?;
                let max_instances =
                    parse_positive(max_instances, "traffic max instances")?;
                let big_flag = big_flag
                    .map(|value| {
                        value
                            .parse::<i32>()
                            .map_err(|_error| {
                                "traffic big flag is not numeric".to_owned()
                            })
                    })
                    .transpose()?;
                let vehicle = match catalog.resolve_vehicle(&source_model)? {
                    MissionVehicleReference::Catalog(reference) => reference,
                    MissionVehicleReference::Current => {
                        return Err(
                            "traffic model cannot use current vehicle"
                                .to_owned(),
                        );
                    }
                };
                group.members.push(MissionTrafficGroupMemberBinding {
                    source_ordinal: command.source_ordinal(),
                    source_model,
                    max_instances,
                    big_flag,
                    vehicle,
                });
            }
            "closetrafficgroup" => {
                require_role(command.semantic_role(), "traffic group close")?;
                if !command.arguments().is_empty() {
                    return Err(
                        "CloseTrafficGroup must have no arguments".to_owned(),
                    );
                }
                let Some(group) = open.take() else {
                    return Err(
                        "CloseTrafficGroup appears without create".to_owned(),
                    );
                };
                if group.members.is_empty() {
                    return Err("traffic group cannot be empty".to_owned());
                }
                groups.push(MissionTrafficGroupBinding {
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
        return Err("traffic group remains open at end of source".to_owned());
    }
    Ok(MissionTrafficGroupReport { groups })
}

fn parse_member_arguments(
    arguments: &[String],
) -> Result<(&str, &str, Option<&str>), String> {
    match arguments {
        [model, max_instances] => Ok((model, max_instances, None)),
        [model, max_instances, big_flag] => {
            Ok((model, max_instances, Some(big_flag)))
        }
        _ => Err("AddTrafficModel must have two or three arguments".to_owned()),
    }
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
        .map_err(|_error| "traffic group index is not unsigned".to_owned())?;
    if index >= MAX_TRAFFIC_GROUPS {
        return Err("traffic group index exceeds runtime capacity".to_owned());
    }
    Ok(index)
}

fn parse_positive(value: &str, label: &str) -> Result<u32, String> {
    let parsed = value
        .parse::<u32>()
        .map_err(|_error| format!("{label} is not unsigned"))?;
    if parsed == 0 {
        return Err(format!("{label} must be positive"));
    }
    Ok(parsed)
}

fn source_token(value: &str, label: &str) -> Result<String, String> {
    if value.is_empty()
        || value != value.trim()
        || value.chars().any(char::is_control)
    {
        return Err(format!("{label} is malformed"));
    }
    Ok(value.to_owned())
}

#[cfg(test)]
// jig-ignore-next-line: exact Rust test-module path is indivisible.
#[path = "../../../../../tests/migration/pipeline/unit/domain/package/mission_traffic_group/tests.rs"]
mod tests;
