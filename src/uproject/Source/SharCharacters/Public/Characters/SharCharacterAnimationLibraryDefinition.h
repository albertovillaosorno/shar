#pragma once
// File:
//   - SharCharacterAnimationLibraryDefinition.h
// Path:
//   - src/uproject/Source/SharCharacters/Public/Characters/SharCharacterAnimationLibraryDefinition.h
//
// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE
// Path-Rule:
//   - All paths in this header are repository-root relative.
//
// Boundary-Contract:
// - Owns:
//   - One shared native character-animation library for a compatible rig family.
// - Must-Not:
//   - Own character identity, execute animation, or perform runtime retargeting.
// - Allows:
//   - Typed semantic clips, Skeleton compatibility, animation class, and policy.
// - Split-When:
//   - A clip family needs an independently loaded Primary Asset boundary.
// - Merge-When:
//   - Another definition owns the identical rig-family animation contract.
// - Summary:
//   - Defines the central deduplicated character-animation library.
// - Description:
//   - Stores compatible clips once and exposes them through stable semantic ids.
// - Usage:
//   - Referenced by character presentations, animation adapters, and mods.
// - Defaults:
//   - Uses one Skeleton, one animation class, and explicit clip definitions.
//
// ADRs:
// - docs/adr/unreal/runtime/shared-rig-family-animation-libraries.md
// - docs/adr/unreal/architecture/aaa-native-content-and-gameplay-foundation.md
//
// Large file:
//   - false
//

#include "Content/SharPrimaryContentDefinition.h"

#include "SharCharacterAnimationLibraryDefinition.generated.h"

class UAnimationAsset;
class UAnimInstance;
class USkeleton;

/** One semantic native animation clip in a shared rig-family library. */
USTRUCT(BlueprintType)
struct SHARCHARACTERS_API FSharCharacterAnimationClipDefinition
{
    GENERATED_BODY()

    /** Default native sample rate used before import read-back replaces it. */
    static constexpr double DefaultSampleRateFramesPerSecond = 30.0;

    /** Stable lowercase clip identity within this rig-family library. */
    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Identity")
    FName ClipId;

    /** Registered semantic role such as locomotion_walk_forward. */
    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Semantics")
    FName SemanticRoleId;

    /** Native Sequence, Montage, Blend Space, Pose Asset, or compatible asset. */
    UPROPERTY(
        EditDefaultsOnly,
        BlueprintReadOnly,
        Category = "Animation",
        meta = (AssetBundles = "Presentation")
    )
    TSoftObjectPtr<UAnimationAsset> AnimationAsset;

    /** Optional eligibility tags for character, costume, mission, or mode. */
    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Eligibility")
    FGameplayTagContainer EligibilityTags;

    /** Expected native duration used for deterministic import read-back. */
    UPROPERTY(
        EditDefaultsOnly,
        BlueprintReadOnly,
        Category = "Validation",
        meta = (ClampMin = "0.0", UIMin = "0.0")
    )
    double ExpectedDurationSeconds = 0.0;

    /** Expected sample rate for native timing validation. */
    UPROPERTY(
        EditDefaultsOnly,
        BlueprintReadOnly,
        Category = "Validation",
        meta = (ClampMin = "1.0", UIMin = "1.0")
    )
    double SampleRateFramesPerSecond = DefaultSampleRateFramesPerSecond;

    /** Whether playback may loop without issuing a terminal observation. */
    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Playback")
    bool bLooping = false;

    /** Whether authoritative root motion is expected from this clip. */
    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Playback")
    bool bUsesRootMotion = false;
};

/** Shared, deduplicated animation library for one compatible character rig. */
UCLASS(BlueprintType)
class SHARCHARACTERS_API USharCharacterAnimationLibraryDefinition
    : public USharPrimaryContentDefinition
{
    GENERATED_BODY()

public:
    /** Canonical rig-family identity used for compatibility and asset paths. */
    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Rig")
    FName RigFamilyId;

    /** Semantic rig profile that maps gameplay roles to bones and sockets. */
    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Rig")
    FName RigProfileId;

    /** Native Skeleton accepted by every directly compatible clip. */
    UPROPERTY(
        EditDefaultsOnly,
        BlueprintReadOnly,
        Category = "Rig",
        meta = (AssetBundles = "Presentation")
    )
    TSoftObjectPtr<USkeleton> Skeleton;

    /** Animation Blueprint class that evaluates this rig-family library. */
    UPROPERTY(
        EditDefaultsOnly,
        BlueprintReadOnly,
        Category = "Animation",
        meta = (AssetBundles = "Presentation")
    )
    TSoftClassPtr<UAnimInstance> AnimationClass;

    /** Deterministically ordered semantic clips stored once for the rig family. */
    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Animation")
    TArray<FSharCharacterAnimationClipDefinition> Clips;

    /** Add library-specific errors without loading referenced assets. */
    void GatherValidationErrors(TArray<FText>& OutErrors) const override;

protected:
    /** Return the fixed `SharCharacterAnimationLibrary` Asset Manager type. */
    [[nodiscard]] FPrimaryAssetType GetDefinitionAssetType() const override;
};
