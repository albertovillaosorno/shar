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
//   - Shar presentation definition composition module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Shar presentation definition composition module.
// - Description:
//   - Implements the declared composition module responsibility for project.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Shar presentation definition composition module.

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
