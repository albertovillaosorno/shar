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
//   - One complete native drivable-vehicle presentation definition.
// - Must-Not:
//   - Own vehicle identity, tuning policy, spawning, or physics construction.
// - Allows:
//   - Soft native asset references and wheel-to-rig bindings.
// - Split-When:
//   - Damage or animation catalogs gain independent Primary Asset identity.
// - Merge-When:
//   - Another definition owns the identical complete vehicle presentation.
// - Summary:
//   - Defines native assets required to construct one vehicle presentation.
// - Description:
//   - Binds validated mesh, physics, animation, material, and wheel assets.
// - Usage:
//   - Resolved by the future vehicle construction transaction.
// - Defaults:
//   - Missing native assets or wheel bindings fail validation.
//

//! Complete native presentation definition for one drivable vehicle variant.

#pragma once

#include "Content/SharPrimaryContentDefinition.h"
#include "CoreMinimal.h"

#include "SharVehiclePresentationDefinition.generated.h"

class UAnimInstance;
class UChaosVehicleWheel;
class UMaterialInterface;
class UPhysicsAsset;
class USkeletalMesh;
class USkeleton;

/** One native wheel class bound to one semantic wheel and rig location. */
USTRUCT(BlueprintType)
struct SHARVEHICLES_API FSharVehicleWheelPresentationBinding
{
    GENERATED_BODY()

    /** Canonical semantic wheel identity. */
    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Wheel")
    FName WheelId;

    /** Exact validated bone or socket used by the native wheel controller. */
    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Wheel")
    FName BoneOrSocketName;

    /** Native Chaos wheel definition class for this wheel. */
    UPROPERTY(
        EditDefaultsOnly,
        BlueprintReadOnly,
        Category = "Wheel",
        meta = (AssetBundles = "Gameplay")
    )
    TSoftClassPtr<UChaosVehicleWheel> WheelClass;
};

/** Complete final native presentation for one drivable vehicle variant. */
UCLASS(BlueprintType)
class SHARVEHICLES_API USharVehiclePresentationDefinition final
    : public USharPrimaryContentDefinition
{
    GENERATED_BODY()

public:
    /** Canonical presentation variant such as default or damaged body
     * family. */
    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Presentation")
    FName PresentationVariant = FName(TEXT("default"));

    /** Final native Skeletal Mesh used by the vehicle Pawn. */
    UPROPERTY(
        EditDefaultsOnly,
        BlueprintReadOnly,
        Category = "Presentation",
        meta = (AssetBundles = "Presentation")
    )
    TSoftObjectPtr<USkeletalMesh> SkeletalMesh;

    /** Skeleton expected by the Skeletal Mesh and animation class. */
    UPROPERTY(
        EditDefaultsOnly,
        BlueprintReadOnly,
        Category = "Presentation",
        meta = (AssetBundles = "Presentation")
    )
    TSoftObjectPtr<USkeleton> Skeleton;

    /** Physics Asset validated before native vehicle physics state is
     * created. */
    UPROPERTY(
        EditDefaultsOnly,
        BlueprintReadOnly,
        Category = "Gameplay",
        meta = (AssetBundles = "Gameplay")
    )
    TSoftObjectPtr<UPhysicsAsset> PhysicsAsset;

    /** Native Animation Blueprint class that drives wheel and moving-part
     * pose. */
    UPROPERTY(
        EditDefaultsOnly,
        BlueprintReadOnly,
        Category = "Presentation",
        meta = (AssetBundles = "Presentation")
    )
    TSoftClassPtr<UAnimInstance> AnimationClass;

    /** Ordered final Material Instances matching semantic Skeletal Mesh
     * slots. */
    UPROPERTY(
        EditDefaultsOnly,
        BlueprintReadOnly,
        Category = "Presentation",
        meta = (AssetBundles = "Presentation")
    )
    TArray<TSoftObjectPtr<UMaterialInterface>> MaterialInstances;

    /** Native wheel classes and exact rig bindings in authored wheel order. */
    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Gameplay")
    TArray<FSharVehicleWheelPresentationBinding> Wheels;

    /** Semantic rig profile that owns moving-part and hardpoint mappings. */
    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Rig")
    FName RigProfileId;

    /** Deterministic vehicle preparation revision consumed by construction. */
    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Provenance")
    FString SemanticPreparationRevision;

    /** Add presentation-specific errors without loading referenced assets. */
    void GatherValidationErrors(TArray<FText>& OutErrors) const override;

protected:
    /** Return the fixed `SharVehiclePresentation` Asset Manager type. */
    [[nodiscard]] FPrimaryAssetType GetDefinitionAssetType() const override;
};
