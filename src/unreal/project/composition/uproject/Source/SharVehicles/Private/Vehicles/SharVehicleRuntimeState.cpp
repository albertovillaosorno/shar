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
//   - Shar vehicle runtime state composition module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Shar vehicle runtime state composition module.
// - Description:
//   - Implements the declared composition module responsibility for project.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Shar vehicle runtime state composition module.

#include "Vehicles/SharVehicleRuntimeState.h"

#include "Vehicles/SharVehicleDefinition.h"

void USharVehicleRuntimeState::RefreshDamageBand()
{
    if (Definition == nullptr)
    {
        return;
    }
    for (const FSharVehicleDamageBandDefinition& Band : Definition->DamageBands)
    {
        if (Band.MinimumNormalizedDamage > NormalizedDamage)
        {
            break;
        }
        DamageState = Band.State;
        HandlingMultiplier = Band.HandlingMultiplier;
    }
}

bool USharVehicleRuntimeState::Configure(
    USharVehicleDefinition* InDefinition
)
{
    if (InDefinition == nullptr)
    {
        return false;
    }
    TArray<FText> ValidationErrors;
    InDefinition->GatherValidationErrors(ValidationErrors);
    if (!ValidationErrors.IsEmpty())
    {
        return false;
    }
    Definition = InDefinition;
    NormalizedDamage = 0.0F;
    RefreshDamageBand();
    return true;
}

bool USharVehicleRuntimeState::ApplyNormalizedDamage(
    const float DamageAmount
)
{
    if (Definition == nullptr
        || !FMath::IsFinite(DamageAmount)
        || DamageAmount < 0.0F)
    {
        return false;
    }
    NormalizedDamage = FMath::Clamp(
        NormalizedDamage + DamageAmount,
        0.0F,
        1.0F
    );
    RefreshDamageBand();
    return true;
}

bool USharVehicleRuntimeState::RepairToNormalizedDamage(
    const float TargetDamage
)
{
    const bool bInvalid =
        Definition == nullptr
        || !FMath::IsFinite(TargetDamage)
        || TargetDamage < 0.0F
        || TargetDamage > NormalizedDamage;
    if (bInvalid)
    {
        return false;
    }
    NormalizedDamage = TargetDamage;
    DamageState = ESharVehicleDamageState::Operational;
    HandlingMultiplier = 1.0F;
    RefreshDamageBand();
    return true;
}

float USharVehicleRuntimeState::GetNormalizedDamage() const
{
    return NormalizedDamage;
}

ESharVehicleDamageState USharVehicleRuntimeState::GetDamageState() const
{
    return DamageState;
}

float USharVehicleRuntimeState::GetHandlingMultiplier() const
{
    return HandlingMultiplier;
}

bool USharVehicleRuntimeState::IsConfigured() const
{
    return Definition != nullptr;
}
