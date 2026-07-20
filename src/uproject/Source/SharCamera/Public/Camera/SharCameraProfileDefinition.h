// File: SharCameraProfileDefinition.h
// Path: src/uproject/Source/SharCamera/Public/Camera/SharCameraProfileDefinition.h
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: immutable camera intent, target, policy, and framing metadata only.
// ADR: docs/adr/unreal/architecture/aaa-native-content-and-gameplay-foundation.md
// LARGE-FILE owner=SharCamera; reason=cohesive reflected camera profile contract;
// split=extract framing data if independent preset assets are introduced;
// validation=validate.sh SharCamera plus Unreal automation; review=2027-01.

#pragma once

#include "Content/SharPrimaryContentDefinition.h"
#include "CoreMinimal.h"

#include "SharCameraProfileDefinition.generated.h"

UENUM(BlueprintType)
enum class ESharCameraModeKind : uint8
{
    Follow,
    Chase,
    Bumper,
    FirstPerson,
    Conversation,
    Animated,
    Debug,
};

UENUM(BlueprintType)
enum class ESharCameraTargetKind : uint8
{
    Character,
    Vehicle,
    CameraActor,
    Sequence,
    Multiple,
};

UENUM(BlueprintType)
enum class ESharCameraPriorityClass : uint8
{
    Debug,
    Default,
    ContextualGameplay,
    PlayerSelected,
    Conversation,
    Cinematic,
    Safety,
};

UENUM(BlueprintType)
enum class ESharCameraCancellationPolicy : uint8
{
    Immediate,
    BlendOut,
    SafePoint,
    Uninterruptible,
};

UCLASS(BlueprintType)
class SHARCAMERA_API USharCameraProfileDefinition final
    : public USharPrimaryContentDefinition
{
    GENERATED_BODY()

public:
    static constexpr float MinimumAllowedFovDegrees = 1.0F;
    static constexpr float MaximumAllowedFovDegrees = 179.0F;
    static constexpr float DefaultMinimumFovDegrees = 60.0F;
    static constexpr float DefaultMaximumFovDegrees = 100.0F;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Camera")
    ESharCameraModeKind ModeKind = ESharCameraModeKind::Follow;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Camera")
    ESharCameraTargetKind TargetKind = ESharCameraTargetKind::Character;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Camera")
    ESharCameraPriorityClass PriorityClass =
        ESharCameraPriorityClass::Default;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Camera")
    ESharCameraCancellationPolicy CancellationPolicy =
        ESharCameraCancellationPolicy::BlendOut;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Policy")
    FName PresetId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Policy")
    FName TransitionPolicyId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Policy")
    FName CollisionPolicyId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Policy")
    FName InputPolicyId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Policy")
    FName VerificationPolicyId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Framing")
    float MinimumFovDegrees = DefaultMinimumFovDegrees;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Framing")
    float MaximumFovDegrees = DefaultMaximumFovDegrees;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Input")
    bool bAllowsLookInput = true;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Input")
    bool bAllowsZoomInput = false;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Input")
    bool bAllowsReverseView = false;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Input")
    bool bAllowsSkipInput = false;

    void GatherValidationErrors(TArray<FText>& OutErrors) const override;

protected:
    [[nodiscard]] FPrimaryAssetType GetDefinitionAssetType() const override;
};
