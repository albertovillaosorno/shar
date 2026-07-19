#pragma once
// File:
//   - SharCharacterPresentationDefinition.h
// Path:
//   - src/uproject/Source/SharCharacters/Public/Characters/SharCharacterPresentationDefinition.h
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
//   - One complete, publishable native character presentation definition.
// - Must-Not:
//   - Own character identity, unlock progression, gameplay behavior, or import.
// - Allows:
//   - Soft mesh, skeleton, physics, material, and animation-class references.
// - Split-When:
//   - Animation or facial catalogs require independent Primary Asset identity.
// - Merge-When:
//   - Another definition owns the identical complete-model presentation.
// - Summary:
//   - Defines a complete character model presentation asset.
// - Description:
//   - Binds final native assets without embedding source-path assumptions.
// - Usage:
//   - Referenced by character definitions and costume or mod selection layers.
// - Defaults:
//   - Requires mesh, skeleton, physics, materials, and preparation evidence.
//
// ADRs:
// - docs/adr/unreal/architecture/aaa-native-content-and-gameplay-foundation.md
// - docs/adr/unreal/import-adapters/staged-mesh-import-and-world-assembly.md
//
// Large file:
//   - false
//

#include "Content/SharPrimaryContentDefinition.h"

#include "SharCharacterPresentationDefinition.generated.h"

class UMaterialInterface;
class USharCharacterAnimationLibraryDefinition;
class UPhysicsAsset;
class USkeletalMesh;
class USkeleton;

/** Complete final native presentation for one character appearance. */
UCLASS(BlueprintType)
class SHARCHARACTERS_API USharCharacterPresentationDefinition
    : public USharPrimaryContentDefinition
{
    GENERATED_BODY()

public:
    /** Default native standing height before import read-back replaces it. */
    static constexpr double DefaultExpectedHeightCentimeters = 180.0;

    /** Default horizontal bounds extent before native read-back. */
    static constexpr double DefaultBoundsHorizontalCentimeters = 50.0;

    /** Default vertical bounds extent before native read-back. */
    static constexpr double DefaultBoundsVerticalCentimeters = 90.0;

    /** Canonical presentation variant such as default or a costume identity. */
    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Presentation")
    FName PresentationVariant = FName(TEXT("default"));

    /** Final native Skeletal Mesh loaded with the Presentation bundle. */
    UPROPERTY(
        EditDefaultsOnly,
        BlueprintReadOnly,
        Category = "Presentation",
        meta = (AssetBundles = "Presentation")
    )
    TSoftObjectPtr<USkeletalMesh> SkeletalMesh;

    /** Skeleton expected by the mesh and shared animation library. */
    UPROPERTY(
        EditDefaultsOnly,
        BlueprintReadOnly,
        Category = "Presentation",
        meta = (AssetBundles = "Presentation")
    )
    TSoftObjectPtr<USkeleton> Skeleton;

    /** Physics Asset used for ragdoll, traces, and cosmetic physical response. */
    UPROPERTY(
        EditDefaultsOnly,
        BlueprintReadOnly,
        Category = "Gameplay",
        meta = (AssetBundles = "Gameplay")
    )
    TSoftObjectPtr<UPhysicsAsset> PhysicsAsset;

    /** Ordered Material Instances matching validated semantic mesh sections. */
    UPROPERTY(
        EditDefaultsOnly,
        BlueprintReadOnly,
        Category = "Presentation",
        meta = (AssetBundles = "Presentation")
    )
    TArray<TSoftObjectPtr<UMaterialInterface>> MaterialInstances;

    /** Semantic rig profile that maps gameplay roles to real bones and sockets. */
    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Rig")
    FName RigProfileId;

    /** Shared rig-family animation library used by this presentation. */
    UPROPERTY(
        EditDefaultsOnly,
        BlueprintReadOnly,
        Category = "Animation",
        meta = (AssetBundles = "Presentation")
    )
    TSoftObjectPtr<USharCharacterAnimationLibraryDefinition> AnimationLibrary;

    /** Texture and material budget profile used during import and validation. */
    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Presentation")
    FName TextureProfileId;

    /** Deterministic semantic mesh, UV, eye, and texture preparation revision. */
    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Provenance")
    FString SemanticPreparationRevision;

    /** Expected native standing height used for scale and bounds validation. */
    UPROPERTY(
        EditDefaultsOnly,
        BlueprintReadOnly,
        Category = "Validation",
        meta = (ClampMin = "1.0", UIMin = "1.0")
    )
    double ExpectedHeightCentimeters = DefaultExpectedHeightCentimeters;

    /** Expected positive native bounds extent in centimeters. */
    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Validation")
    FVector ExpectedBoundsExtentCentimeters = FVector(
        DefaultBoundsHorizontalCentimeters,
        DefaultBoundsHorizontalCentimeters,
        DefaultBoundsVerticalCentimeters
    );

    /** Add presentation-specific errors without loading referenced assets. */
    void GatherValidationErrors(TArray<FText>& OutErrors) const override;

protected:
    /** Return the fixed `SharCharacterPresentation` Asset Manager type. */
    [[nodiscard]] FPrimaryAssetType GetDefinitionAssetType() const override;
};
