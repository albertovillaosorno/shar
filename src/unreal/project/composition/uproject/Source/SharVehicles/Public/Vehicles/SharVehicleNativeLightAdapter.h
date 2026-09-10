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
//   - Native world-light components for semantic vehicle headlight hardpoints.
// - Must-Not:
//   - Invent photometric tuning or replace camera-facing glow presentation.
// - Allows:
//   - Attach native spot lights to reviewed bones and refresh their visibility.
// - Split-When:
//   - Brake, reverse, or camera-facing glow output gains its own native policy.
// - Merge-When:
//   - Another vehicle presentation adapter owns identical native light output.
// - Summary:
//   - Adapts semantic headlight state to native Unreal spot lights.
// - Description:
//   - Deduplicates part bindings by rig hardpoint and follows semantic state.
// - Usage:
//   - Configure after the vehicle Skeletal Mesh is committed, then refresh.
// - Defaults:
//   - Light fields are invalid until explicitly supplied by reviewed data.
//

//! Native world-light adapter for semantic vehicle headlight hardpoints.

#pragma once

#include "CoreMinimal.h"
#include "UObject/Object.h"
#include "UObject/ObjectPtr.h"

#include "SharVehicleNativeLightAdapter.generated.h"

class ASharVehiclePawn;
class USharVehiclePresentationDefinition;
class USharVehiclePresentationState;
class USpotLightComponent;

/** Explicit transient photometric configuration for native headlight output. */
USTRUCT(BlueprintType)
struct SHARVEHICLES_API FSharVehicleNativeHeadlightConfiguration
{
    GENERATED_BODY()

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Headlight")
    float IntensityLumens = 0.0F;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Headlight")
    float AttenuationRadiusCentimeters = 0.0F;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Headlight")
    float InnerConeAngleDegrees = 0.0F;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Headlight")
    float OuterConeAngleDegrees = 0.0F;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Headlight")
    FLinearColor LightColor = FLinearColor::Transparent;

    /** Explicit direction in the attached bone's local frame. */
    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Headlight")
    FVector BoneLocalDirection = FVector::ZeroVector;
};

/** One native emitter plus all semantic bindings sharing its hardpoint. */
USTRUCT()
struct FSharVehicleNativeHeadlightEmitter
{
    GENERATED_BODY()

    UPROPERTY(Transient)
    FName BoneName;

    UPROPERTY(Transient)
    TArray<FName> BindingIds;

    UPROPERTY(Transient)
    TObjectPtr<USpotLightComponent> Component;
};

UCLASS(BlueprintType)
class SHARVEHICLES_API USharVehicleNativeLightAdapter final : public UObject
{
    GENERATED_BODY()

public:
    UFUNCTION(BlueprintCallable, Category = "SHAR|Vehicle|Presentation")
    bool ConfigureHeadlights(
        ASharVehiclePawn* InTargetPawn,
        USharVehiclePresentationDefinition* InPresentation,
        USharVehiclePresentationState* InState,
        const FSharVehicleNativeHeadlightConfiguration& InConfiguration
    );

    UFUNCTION(BlueprintCallable, Category = "SHAR|Vehicle|Presentation")
    bool RefreshHeadlights();

    UFUNCTION(BlueprintCallable, Category = "SHAR|Vehicle|Presentation")
    void ResetHeadlights();

    UFUNCTION(BlueprintPure, Category = "SHAR|Vehicle|Presentation")
    [[nodiscard]] int32 GetHeadlightEmitterCount() const;

    UFUNCTION(BlueprintPure, Category = "SHAR|Vehicle|Presentation")
    [[nodiscard]] FName GetHeadlightEmitterBone(int32 Index) const;

    UFUNCTION(BlueprintPure, Category = "SHAR|Vehicle|Presentation")
    [[nodiscard]] USpotLightComponent* GetHeadlightEmitter(int32 Index) const;

private:
    UPROPERTY(Transient)
    TObjectPtr<ASharVehiclePawn> TargetPawn;

    UPROPERTY(Transient)
    TObjectPtr<USharVehiclePresentationState> State;

    UPROPERTY(Transient)
    TArray<FSharVehicleNativeHeadlightEmitter> HeadlightEmitters;
};
