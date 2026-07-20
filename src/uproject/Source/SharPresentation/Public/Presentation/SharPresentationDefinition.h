// File: SharPresentationDefinition.h
// Path: src/uproject/Source/SharPresentation/Public/Presentation/SharPresentationDefinition.h
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: immutable playback policy only; animation, camera, media, UI, and audio remain external adapters.
// Specification: docs/technical/unreal/presentation-playback-runtime.md
// LARGE-FILE owner=SharPresentation; reason=cohesive reflected presentation policy schema;
// split=extract adapter-specific policy identities if they become independently versioned assets;
// validation=validate.sh SharPresentation plus Unreal automation; review=2027-01.

#pragma once

#include "Content/SharPrimaryContentDefinition.h"
#include "CoreMinimal.h"

#include "SharPresentationDefinition.generated.h"

UENUM(BlueprintType)
enum class ESharPresentationKind : uint8
{
    Animation,
    Camera,
    CosmeticLayer,
    Sequence,
    Media,
    Composite,
};

UENUM(BlueprintType)
enum class ESharPresentationSkipPolicy : uint8
{
    NotSkippable,
    Immediate,
    Hold,
    Vote,
    Accessibility,
    OwnerControlled,
};

UENUM(BlueprintType)
enum class ESharPresentationTimePolicy : uint8
{
    Simulation,
    Sequence,
    Audio,
    Media,
};

UCLASS(BlueprintType)
class SHARPRESENTATION_API USharPresentationDefinition final
    : public USharPrimaryContentDefinition
{
    GENERATED_BODY()

public:
    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Presentation")
    ESharPresentationKind PresentationKind = ESharPresentationKind::Animation;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Assets")
    FName AssetSetId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Ownership")
    FName OwnerPolicyId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Playback")
    FName PlaybackPolicyId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Exclusivity")
    FName ExclusivityPolicyId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Skip")
    ESharPresentationSkipPolicy SkipPolicy =
        ESharPresentationSkipPolicy::NotSkippable;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Time")
    ESharPresentationTimePolicy TimePolicy =
        ESharPresentationTimePolicy::Simulation;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Camera")
    FName CameraPolicyId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Character")
    FName CharacterLayerPolicyId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Fallback")
    FName FallbackPolicyId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Result")
    FName ResultPolicyId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Teardown")
    FName TeardownPolicyId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Exclusivity")
    bool bRequiresScopedLeases = false;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Teardown")
    bool bHasCompleteReleasePath = true;

    void GatherValidationErrors(TArray<FText>& OutErrors) const override;

protected:
    [[nodiscard]] FPrimaryAssetType GetDefinitionAssetType() const override;
};
