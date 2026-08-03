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
//   - Shar vehicle definition composition module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Shar vehicle definition composition module.
// - Description:
//   - Implements the declared composition module responsibility for project.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Shar vehicle definition composition module.

#pragma once

#include "Content/SharPrimaryContentDefinition.h"
#include "CoreMinimal.h"

#include "SharVehicleDefinition.generated.h"

UENUM(BlueprintType)
enum class ESharVehicleDamageState : uint8
{
    Operational,
    Damaged,
    Critical,
    Disabled,
};

USTRUCT(BlueprintType)
struct SHARVEHICLES_API FSharVehiclePhysicsDefinition
{
    GENERATED_BODY()

    static constexpr float DefaultMassKilograms = 1200.0F;
    static constexpr float DefaultMaximumSpeedKilometersPerHour = 160.0F;
    static constexpr float DefaultEngineTorqueNewtonMeters = 300.0F;
    static constexpr float DefaultWheelRadiusCentimeters = 32.0F;
    static constexpr float DefaultWheelbaseCentimeters = 250.0F;
    static constexpr float DefaultTrackWidthCentimeters = 160.0F;
    static constexpr float DefaultSuspensionTravelCentimeters = 20.0F;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Physics")
    float MassKilograms = DefaultMassKilograms;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Physics")
    float MaximumSpeedKilometersPerHour =
        DefaultMaximumSpeedKilometersPerHour;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Physics")
    float EngineTorqueNewtonMeters = DefaultEngineTorqueNewtonMeters;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Physics")
    float WheelRadiusCentimeters = DefaultWheelRadiusCentimeters;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Physics")
    float WheelbaseCentimeters = DefaultWheelbaseCentimeters;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Physics")
    float TrackWidthCentimeters = DefaultTrackWidthCentimeters;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Physics")
    float SuspensionTravelCentimeters =
        DefaultSuspensionTravelCentimeters;
};

USTRUCT(BlueprintType)
struct SHARVEHICLES_API FSharVehicleSeatDefinition
{
    GENERATED_BODY()

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Seat")
    FName SeatId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Seat")
    FName OccupancyRoleId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Seat")
    FName EntryTransformId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Seat")
    FName ExitTransformId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Seat")
    FName CameraProfileId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Seat")
    bool bDriver = false;
};

USTRUCT(BlueprintType)
struct SHARVEHICLES_API FSharVehicleDamageBandDefinition
{
    GENERATED_BODY()

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Damage")
    ESharVehicleDamageState State = ESharVehicleDamageState::Operational;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Damage")
    float MinimumNormalizedDamage = 0.0F;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Damage")
    float HandlingMultiplier = 1.0F;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Damage")
    FName PresentationProfileId;
};

UCLASS(BlueprintType)
class SHARVEHICLES_API USharVehicleDefinition final
    : public USharPrimaryContentDefinition
{
    GENERATED_BODY()

public:
    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Vehicle")
    FName VehicleFamilyId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Presentation")
    FName DefaultPresentationId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Vehicle")
    bool bDrivable = true;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Physics")
    FSharVehiclePhysicsDefinition Physics;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Seat")
    TArray<FSharVehicleSeatDefinition> Seats;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Damage")
    TArray<FSharVehicleDamageBandDefinition> DamageBands;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "AI")
    FName AiProfileId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Network")
    FName NetworkPredictionProfileId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Recovery")
    FName RecoveryPolicyId;

    void GatherValidationErrors(TArray<FText>& OutErrors) const override;

protected:
    [[nodiscard]] FPrimaryAssetType GetDefinitionAssetType() const override;
};
