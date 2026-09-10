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
//   - Native world-light components for semantic vehicle brake hardpoints.
// - Must-Not:
//   - Invent photometric tuning or replace authored billboard presentation.
// - Allows:
//   - Attach native spot lights to reviewed brake bones and refresh visibility.
// - Split-When:
//   - Reverse or camera-facing rear-light glow gains native output policy.
// - Merge-When:
//   - Another adapter owns identical brake-light component lifecycle.
// - Summary:
//   - Adapts semantic brake state to native Unreal spot lights.
// - Description:
//   - Preserves every authored brake hardpoint without spatial regrouping.
// - Usage:
//   - Configure after vehicle presentation construction, then refresh.
// - Defaults:
//   - Light fields are invalid until explicitly supplied by reviewed data.
//

//! Native world-light adapter for semantic vehicle brake hardpoints.

#pragma once

#include "CoreMinimal.h"
#include "UObject/Object.h"
#include "UObject/ObjectPtr.h"

#include "SharVehicleNativeBrakeLightAdapter.generated.h"

class ASharVehiclePawn;
class USharVehiclePresentationDefinition;
class USharVehiclePresentationState;
class USpotLightComponent;

/** Explicit transient photometric configuration for native brake output. */
USTRUCT(BlueprintType)
struct SHARVEHICLES_API FSharVehicleNativeBrakeLightConfiguration
{
    GENERATED_BODY()

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "BrakeLight")
    float IntensityLumens = 0.0F;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "BrakeLight")
    float AttenuationRadiusCentimeters = 0.0F;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "BrakeLight")
    float InnerConeAngleDegrees = 0.0F;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "BrakeLight")
    float OuterConeAngleDegrees = 0.0F;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "BrakeLight")
    FLinearColor LightColor = FLinearColor::Transparent;

    /** Explicit direction in the attached bone's local frame. */
    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "BrakeLight")
    FVector BoneLocalDirection = FVector::ZeroVector;
};

/** One native brake emitter for one authored semantic hardpoint. */
USTRUCT()
struct FSharVehicleNativeBrakeLightEmitter
{
    GENERATED_BODY()

    UPROPERTY(Transient)
    FName BoneName;

    UPROPERTY(Transient)
    FName BindingId;

    UPROPERTY(Transient)
    TObjectPtr<USpotLightComponent> Component;
};

UCLASS(BlueprintType)
class SHARVEHICLES_API USharVehicleNativeBrakeLightAdapter final
    : public UObject
{
    GENERATED_BODY()

public:
    UFUNCTION(BlueprintCallable, Category = "SHAR|Vehicle|Presentation")
    bool ConfigureBrakeLights(
        ASharVehiclePawn* InTargetPawn,
        USharVehiclePresentationDefinition* InPresentation,
        USharVehiclePresentationState* InState,
        const FSharVehicleNativeBrakeLightConfiguration& InConfiguration
    );

    UFUNCTION(BlueprintCallable, Category = "SHAR|Vehicle|Presentation")
    bool RefreshBrakeLights();

    UFUNCTION(BlueprintCallable, Category = "SHAR|Vehicle|Presentation")
    void ResetBrakeLights();

    UFUNCTION(BlueprintPure, Category = "SHAR|Vehicle|Presentation")
    [[nodiscard]] int32 GetBrakeLightEmitterCount() const;

    UFUNCTION(BlueprintPure, Category = "SHAR|Vehicle|Presentation")
    [[nodiscard]] FName GetBrakeLightEmitterBone(int32 Index) const;

    UFUNCTION(BlueprintPure, Category = "SHAR|Vehicle|Presentation")
    [[nodiscard]] USpotLightComponent* GetBrakeLightEmitter(int32 Index) const;

private:
    UPROPERTY(Transient)
    TObjectPtr<ASharVehiclePawn> TargetPawn;

    UPROPERTY(Transient)
    TObjectPtr<USharVehiclePresentationState> State;

    UPROPERTY(Transient)
    TArray<FSharVehicleNativeBrakeLightEmitter> BrakeLightEmitters;
};
