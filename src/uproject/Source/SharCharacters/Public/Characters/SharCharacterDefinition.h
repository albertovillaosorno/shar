#pragma once
// File:
//   - SharCharacterDefinition.h
// Path:
//   - src/uproject/Source/SharCharacters/Public/Characters/SharCharacterDefinition.h
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
//   - Stable character identity and replaceable gameplay or presentation links.
// - Must-Not:
//   - Spawn pawns, grant progression, run missions, or synchronously load assets.
// - Allows:
//   - Soft character class, presentation, profile, selector, and collision data.
// - Split-When:
//   - A character subsystem needs its own independently versioned definition.
// - Merge-When:
//   - Another asset owns the same canonical character identity and invariants.
// - Summary:
//   - Defines the native character catalog contract.
// - Description:
//   - Separates stable character identity from replaceable implementation assets.
// - Usage:
//   - Resolved by catalog, selector, mission, spawning, save, and mod services.
// - Defaults:
//   - Supports free-roam selection and AI eligibility with explicit profiles.
//
// ADRs:
// - docs/adr/unreal/architecture/aaa-native-content-and-gameplay-foundation.md
// - docs/adr/unreal/runtime/data-driven-gameplay-content-catalog.md
//
// Large file:
//   - false
//

#include "Content/SharPrimaryContentDefinition.h"

#include "SharCharacterDefinition.generated.h"

class ACharacter;
class USharCharacterPresentationDefinition;

/** Stable gameplay and catalog definition for one canonical character. */
UCLASS(BlueprintType)
class SHARCHARACTERS_API USharCharacterDefinition
    : public USharPrimaryContentDefinition
{
    GENERATED_BODY()

public:
    /** Default capsule radius before a validated profile replaces it. */
    static constexpr double DefaultCapsuleRadiusCentimeters = 42.0;

    /** Default capsule half-height before a validated profile replaces it. */
    static constexpr double DefaultCapsuleHalfHeightCentimeters = 96.0;

    /** Native pawn class constructed after all required bundles are ready. */
    UPROPERTY(
        EditDefaultsOnly,
        BlueprintReadOnly,
        Category = "Gameplay",
        meta = (AssetBundles = "Gameplay")
    )
    TSoftClassPtr<ACharacter> CharacterClass;

    /** Complete default appearance loaded through the Presentation bundle. */
    UPROPERTY(
        EditDefaultsOnly,
        BlueprintReadOnly,
        Category = "Presentation",
        meta = (AssetBundles = "Presentation")
    )
    TSoftObjectPtr<USharCharacterPresentationDefinition> DefaultPresentation;

    /** Additional complete costume or mod-selectable presentations. */
    UPROPERTY(
        EditDefaultsOnly,
        BlueprintReadOnly,
        Category = "Presentation",
        meta = (AssetBundles = "Presentation")
    )
    TArray<TSoftObjectPtr<USharCharacterPresentationDefinition>>
        PresentationVariants;

    /** Character Movement tuning and capability profile. */
    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Gameplay")
    FName MovementProfileId;

    /** Gameplay Ability System ability and attribute profile. */
    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Gameplay")
    FName AbilitySetId;

    /** Modern camera profile used for ordinary third-person play. */
    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Camera")
    FName CameraProfileId;

    /** Dialogue, effort, impact, and character-audio profile. */
    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Audio")
    FName VoiceProfileId;

    /** Footprint, dirt, wetness, and ground-contact presentation profile. */
    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Presentation")
    FName FootprintProfileId;

    /** Progression policy that controls permanent and temporary availability. */
    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Progression")
    FName UnlockPolicyId;

    /** Whether an unlocked character can be selected outside a mission. */
    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Selection")
    bool bSelectableOutsideMissions = true;

    /** Whether native AI or ambient systems may control this character. */
    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "AI")
    bool bEligibleForAI = true;

    /** Authoritative Character capsule radius in centimeters. */
    UPROPERTY(
        EditDefaultsOnly,
        BlueprintReadOnly,
        Category = "Collision",
        meta = (ClampMin = "1.0", UIMin = "1.0")
    )
    double CapsuleRadiusCentimeters = DefaultCapsuleRadiusCentimeters;

    /** Authoritative Character capsule half-height in centimeters. */
    UPROPERTY(
        EditDefaultsOnly,
        BlueprintReadOnly,
        Category = "Collision",
        meta = (ClampMin = "1.0", UIMin = "1.0")
    )
    double CapsuleHalfHeightCentimeters =
        DefaultCapsuleHalfHeightCentimeters;

    /** Add character-specific errors without loading referenced assets. */
    void GatherValidationErrors(TArray<FText>& OutErrors) const override;

protected:
    /** Return the fixed `SharCharacter` Asset Manager type. */
    [[nodiscard]] FPrimaryAssetType GetDefinitionAssetType() const override;
};
