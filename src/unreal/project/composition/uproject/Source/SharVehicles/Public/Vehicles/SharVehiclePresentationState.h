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
//   - Transient semantic visibility state for one vehicle presentation.
// - Must-Not:
//   - Mutate materials, infer source shader names, or own gameplay damage.
// - Allows:
//   - Resolve reviewed headlight, brake, and reverse binding visibility.
// - Split-When:
//   - Light intensity, material parameters, or VFX gain independent policy.
// - Merge-When:
//   - Another runtime state object owns identical light presentation state.
// - Summary:
//   - Vehicle dynamic-light presentation state.
// - Description:
//   - Keeps presentation requests separate from material implementation.
// - Usage:
//   - Configure from a validated presentation, then apply semantic requests.
// - Defaults:
//   - All light roles start hidden and unsuppressed at full fade opacity.
//

//! Transient semantic vehicle light presentation state.

#pragma once

#include "CoreMinimal.h"
#include "UObject/ObjectPtr.h"

#include "SharVehiclePresentationState.generated.h"

class USharVehiclePresentationDefinition;

UCLASS(BlueprintType)
class SHARVEHICLES_API USharVehiclePresentationState final : public UObject
{
    GENERATED_BODY()

public:
    UFUNCTION(BlueprintCallable, Category = "SHAR|Vehicle|Presentation")
    bool Configure(USharVehiclePresentationDefinition* InPresentation);

    UFUNCTION(BlueprintCallable, Category = "SHAR|Vehicle|Presentation")
    bool SetHeadlightsEnabled(bool bEnabled);

    UFUNCTION(BlueprintCallable, Category = "SHAR|Vehicle|Presentation")
    bool SetBrakeState(bool bBrakeActive, bool bInReverse);

    UFUNCTION(BlueprintCallable, Category = "SHAR|Vehicle|Presentation")
    bool SetLightsSuppressedByDamage(bool bSuppressed);

    UFUNCTION(BlueprintCallable, Category = "SHAR|Vehicle|Presentation")
    bool SetFadeOpacity(float InOpacity);

    UFUNCTION(BlueprintCallable, Category = "SHAR|Vehicle|Presentation")
    bool GetBindingVisibility(FName BindingId, bool& bOutVisible) const;

    UFUNCTION(BlueprintPure, Category = "SHAR|Vehicle|Presentation")
    [[nodiscard]] bool IsConfigured() const;

private:
    UPROPERTY(Transient)
    TObjectPtr<USharVehiclePresentationDefinition> Presentation;

    UPROPERTY(Transient)
    bool bHeadlightsEnabled = false;

    UPROPERTY(Transient)
    bool bBrakeLightsEnabled = false;

    UPROPERTY(Transient)
    bool bReverseLightsEnabled = false;

    UPROPERTY(Transient)
    bool bLightsSuppressedByDamage = false;

    UPROPERTY(Transient)
    float FadeOpacity = 1.0F;
};
