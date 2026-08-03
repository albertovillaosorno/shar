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

#pragma once

#include "CoreMinimal.h"
#include "UObject/ObjectPtr.h"
#include "Vehicles/SharVehicleDefinition.h"

#include "SharVehicleRuntimeState.generated.h"

UCLASS(BlueprintType)
class SHARVEHICLES_API USharVehicleRuntimeState final : public UObject
{
    GENERATED_BODY()

public:
    UFUNCTION(BlueprintCallable, Category = "SHAR|Vehicle")
    bool Configure(USharVehicleDefinition* InDefinition);

    UFUNCTION(BlueprintCallable, Category = "SHAR|Vehicle")
    bool ApplyNormalizedDamage(float DamageAmount);

    UFUNCTION(BlueprintCallable, Category = "SHAR|Vehicle")
    bool RepairToNormalizedDamage(float TargetDamage);

    UFUNCTION(BlueprintPure, Category = "SHAR|Vehicle")
    [[nodiscard]] float GetNormalizedDamage() const;

    UFUNCTION(BlueprintPure, Category = "SHAR|Vehicle")
    [[nodiscard]] ESharVehicleDamageState GetDamageState() const;

    UFUNCTION(BlueprintPure, Category = "SHAR|Vehicle")
    [[nodiscard]] float GetHandlingMultiplier() const;

    UFUNCTION(BlueprintPure, Category = "SHAR|Vehicle")
    [[nodiscard]] bool IsConfigured() const;

private:
    UPROPERTY(Transient)
    TObjectPtr<USharVehicleDefinition> Definition;

    UPROPERTY(Transient)
    float NormalizedDamage = 0.0F;

    UPROPERTY(Transient)
    ESharVehicleDamageState DamageState =
        ESharVehicleDamageState::Operational;

    UPROPERTY(Transient)
    float HandlingMultiplier = 1.0F;

    void RefreshDamageBand();
};
