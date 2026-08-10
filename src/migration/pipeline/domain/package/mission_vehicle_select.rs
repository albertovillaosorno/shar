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
//   - Canonical package binding for authored vehicle-select registrations.
// - Must-Not:
//   - Infer menu availability, ownership, unlock, or selection behavior.
// - Allows:
//   - Bind exact P3D, vehicle, and character identities to package evidence.
// - Split-When:
//   - Vehicle-selection runtime policy gains an authoritative model.
// - Merge-When:
//   - Final level registry compilation owns these exact registrations.
// - Summary:
//   - Vehicle-select registration semantic preflight.
// - Description:
//   - Closes three authored source identities without inventing UI policy.
// - Usage:
//   - Runs on every normalized mission-script source.
// - Defaults:
//   - Missing, ambiguous, symbolic, or malformed references fail closed.
//

//! Source-backed vehicle-select registration bindings.

use super::{
    MissionCharacterCatalogReference, MissionP3dPackageReference,
    MissionP3dReferenceCatalog, MissionReferenceCatalog, MissionScriptEvidence,
    MissionVehicleCatalogReference, MissionVehicleReference,
};

/// One canonical `AddVehicleSelectInfo` registration.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionVehicleSelectBinding {
    source_ordinal: usize,
    p3d: MissionP3dPackageReference,
    vehicle: MissionVehicleCatalogReference,
    character: MissionCharacterCatalogReference,
}

impl MissionVehicleSelectBinding {
    /// Return the source command ordinal.
    #[must_use]
    pub const fn source_ordinal(&self) -> usize {
        self.source_ordinal
    }

    /// Return the canonical authored P3D binding.
    #[must_use]
    pub const fn p3d(&self) -> &MissionP3dPackageReference {
        &self.p3d
    }

    /// Return the canonical vehicle-package binding.
    #[must_use]
    pub const fn vehicle(&self) -> &MissionVehicleCatalogReference {
        &self.vehicle
    }

    /// Return the canonical character-package binding.
    #[must_use]
    pub const fn character(&self) -> &MissionCharacterCatalogReference {
        &self.character
    }
}

/// All vehicle-select registrations in one source.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct MissionVehicleSelectReport {
    bindings: Vec<MissionVehicleSelectBinding>,
}

impl MissionVehicleSelectReport {
    /// Return registrations in authored order.
    #[must_use]
    pub fn bindings(&self) -> &[MissionVehicleSelectBinding] {
        &self.bindings
    }
}

/// Resolve all authored vehicle-select source identities canonically.
///
/// # Errors
///
/// Fails on malformed shape/role, missing or ambiguous packages, or symbolic
/// vehicle references where a physical vehicle registration is required.
pub fn preflight_mission_vehicle_selects(
    evidence: &MissionScriptEvidence,
    references: &MissionReferenceCatalog,
    p3d_references: &MissionP3dReferenceCatalog,
) -> Result<MissionVehicleSelectReport, String> {
    let mut bindings = Vec::new();
    for invocation in evidence
        .invocations()
        .iter()
        .filter(|invocation| invocation.name() == "addvehicleselectinfo")
    {
        if invocation.semantic_role() != "mission-script" {
            return Err("vehicle-select semantic role changed".to_owned());
        }
        let [p3d_path, vehicle_id, character_id] = invocation.arguments() else {
            return Err(
                "AddVehicleSelectInfo must have three arguments".to_owned(),
            );
        };
        let package_reference = p3d_references.resolve(p3d_path)?;
        let vehicle = match references.resolve_vehicle(vehicle_id)? {
            MissionVehicleReference::Catalog(vehicle) => vehicle,
            MissionVehicleReference::Current => {
                return Err(
                    "vehicle-select vehicle cannot be current".to_owned(),
                );
            }
        };
        let character = references.resolve_character(character_id)?;
        bindings.push(MissionVehicleSelectBinding {
            source_ordinal: invocation.ordinal(),
            p3d: package_reference,
            vehicle,
            character,
        });
    }
    Ok(MissionVehicleSelectReport { bindings })
}

#[cfg(test)]
// jig-ignore-next-line: exact Rust test-module path is indivisible.
#[path = "../../../../../tests/migration/pipeline/unit/domain/package/mission_vehicle_select/tests.rs"]
mod tests;
