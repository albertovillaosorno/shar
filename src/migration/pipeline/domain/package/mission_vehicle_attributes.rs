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
//   - Package-backed source `SetCarAttributes` tuple evidence.
// - Must-Not:
//   - Name or infer the runtime meaning of the four positional scalar fields.
// - Allows:
//   - Validate reviewed scalar shape and bind the exact physical vehicle.
// - Split-When:
//   - Runtime vehicle-stat semantics gain independent source authority.
// - Merge-When:
//   - Final vehicle tuning compilation owns these exact source tuples.
// - Summary:
//   - Vehicle attribute tuple preflight.
// - Description:
//   - Preserves four reviewed numeric lexemes without inventing field meaning.
// - Usage:
//   - Runs after mission scope projection and vehicle catalog creation.
// - Defaults:
//   - Duplicate ids, symbolic vehicles, or scalar drift fail closed.
//

//! Package-backed opaque `SetCarAttributes` tuple evidence.

use std::collections::BTreeSet;

use super::{
    MissionReferenceCatalog, MissionScopeReport, MissionVehicleCatalogReference,
    MissionVehicleReference,
};

/// One reviewed `SetCarAttributes` tuple bound to a physical vehicle package.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionVehicleAttributeBinding {
    source_ordinal: usize,
    vehicle_id: String,
    vehicle: MissionVehicleCatalogReference,
    source_values: [String; 4],
}

impl MissionVehicleAttributeBinding {
    /// Return source statement ordinal.
    #[must_use]
    pub const fn source_ordinal(&self) -> usize {
        self.source_ordinal
    }

    /// Return exact authored vehicle identity.
    #[must_use]
    pub fn vehicle_id(&self) -> &str {
        &self.vehicle_id
    }

    /// Return canonical physical vehicle package reference.
    #[must_use]
    pub const fn vehicle(&self) -> &MissionVehicleCatalogReference {
        &self.vehicle
    }

    /// Return the four exact positional source lexemes.
    #[must_use]
    pub fn source_values(&self) -> &[String; 4] {
        &self.source_values
    }
}

/// Reviewed package-backed vehicle attribute tuples in source order.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct MissionVehicleAttributeReport {
    bindings: Vec<MissionVehicleAttributeBinding>,
}

impl MissionVehicleAttributeReport {
    /// Return bindings in authored source order.
    #[must_use]
    pub fn bindings(&self) -> &[MissionVehicleAttributeBinding] {
        &self.bindings
    }
}

/// Compile every unscoped source `SetCarAttributes` command.
///
/// # Errors
///
/// Returns an error for role/arity drift, duplicate vehicle ids, symbolic or
/// missing vehicle packages, or scalar values outside reviewed base evidence.
pub fn preflight_mission_vehicle_attributes(
    catalog: &MissionReferenceCatalog,
    scopes: &MissionScopeReport,
) -> Result<MissionVehicleAttributeReport, String> {
    let mut bindings = Vec::new();
    let mut ids = BTreeSet::new();
    for command in scopes
        .unscoped_commands()
        .iter()
        .filter(|command| command.name() == "setcarattributes")
    {
        if command.semantic_role() != "mission-script" {
            return Err("SetCarAttributes semantic role changed".to_owned());
        }
        push_binding(
            &mut bindings,
            &mut ids,
            catalog,
            command.source_ordinal(),
            command.arguments(),
        )?;
    }
    Ok(MissionVehicleAttributeReport { bindings })
}

fn push_binding(
    bindings: &mut Vec<MissionVehicleAttributeBinding>,
    ids: &mut BTreeSet<String>,
    catalog: &MissionReferenceCatalog,
    source_ordinal: usize,
    arguments: &[String],
) -> Result<(), String> {
    let [vehicle_id, first, second, third, fourth] = arguments else {
        return Err("SetCarAttributes must have five arguments".to_owned());
    };
    let vehicle_id = required_token(vehicle_id, "vehicle identity")?;
    if !ids.insert(vehicle_id.to_ascii_lowercase()) {
        return Err(
            "SetCarAttributes vehicle identity is duplicated".to_owned(),
        );
    }
    let vehicle = match catalog.resolve_vehicle(&vehicle_id)? {
        MissionVehicleReference::Catalog(reference) => reference,
        MissionVehicleReference::Current => {
            return Err(
                "SetCarAttributes cannot target current vehicle".to_owned(),
            );
        },
    };
    let source_values = [
        required_scalar(first, 1)?,
        required_scalar(second, 2)?,
        required_scalar(third, 3)?,
        required_scalar(fourth, 4)?,
    ];
    bindings.push(MissionVehicleAttributeBinding {
        source_ordinal,
        vehicle_id,
        vehicle,
        source_values,
    });
    Ok(())
}

fn required_token(value: &str, role: &str) -> Result<String, String> {
    if value.is_empty()
        || value != value.trim()
        || value.chars().any(char::is_control)
    {
        return Err(format!("SetCarAttributes {role} is malformed"));
    }
    Ok(value.to_owned())
}

fn required_scalar(value: &str, position: usize) -> Result<String, String> {
    let source = required_token(value, "scalar")?;
    let parsed = source.parse::<f32>().map_err(|_error| {
        format!("SetCarAttributes scalar {position} is not decimal")
    })?;
    if !parsed.is_finite() || !(0.5..=5.0).contains(&parsed) {
        return Err(format!(
            "SetCarAttributes scalar {position} is outside reviewed range"
        ));
    }
    Ok(source)
}

#[cfg(test)]
// jig-ignore-next-line: exact Rust test-module path is indivisible.
#[path = "../../../../../tests/migration/pipeline/unit/domain/package/mission_vehicle_attributes/tests.rs"]
mod tests;
