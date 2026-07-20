// File: SharVehicleRuntimeState.cpp
// Path: src/uproject/Source/SharVehicles/Private/Vehicles/SharVehicleRuntimeState.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: deterministic damage-band selection only; no actor or Chaos mutation.
// ADR: docs/adr/unreal/architecture/aaa-native-content-and-gameplay-foundation.md
// LARGE-FILE owner=SharVehicles; reason=cohesive runtime damage-state behavior;
// split=extract repair policy if vehicle persistence grows;
// validation=validate.sh SharVehicles plus Unreal automation; review=2027-01.

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
