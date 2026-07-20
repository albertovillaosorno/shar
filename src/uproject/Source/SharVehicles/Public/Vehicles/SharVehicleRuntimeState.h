// File: SharVehicleRuntimeState.h
// Path: src/uproject/Source/SharVehicles/Public/Vehicles/SharVehicleRuntimeState.h
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: transient typed vehicle damage projection only; no actor or physics authority.
// ADR: docs/adr/unreal/architecture/aaa-native-content-and-gameplay-foundation.md

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
