#pragma once
// File:
//   - SharPrimaryContentDefinition.h
// Path:
//   - src/uproject/Source/SharContent/Public/Content/SharPrimaryContentDefinition.h
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
//   - Shared identity, provenance, dependency, and validation fields for SHAR
//   - Primary Asset definitions.
// - Must-Not:
//   - Own gameplay-family state, editor import behavior, or mutable discovery.
// - Allows:
//   - Asset Manager identity, soft dependencies, tags, and deterministic
//   - definition validation.
// - Split-When:
//   - A field or invariant belongs to one concrete content family only.
// - Merge-When:
//   - Another base asset owns the identical cross-family definition contract.
// - Summary:
//   - Defines the common native content asset contract.
// - Description:
//   - Makes stable domain identity independent from object names and paths.
// - Usage:
//   - Inherited by character, vehicle, mission, world, and mod definitions.
// - Defaults:
//   - Uses schema revision one and the base feature namespace.
//
// ADRs:
// - docs/adr/unreal/architecture/aaa-native-content-and-gameplay-foundation.md
// - docs/adr/unreal/runtime/data-driven-gameplay-content-catalog.md
//
// Large file:
//   - false
//

#include "CoreMinimal.h"
#include "Engine/DataAsset.h"
#include "GameplayTagContainer.h"

#include "SharPrimaryContentDefinition.generated.h"

/**
 * Stable cross-family definition consumed through Unreal's Asset Manager.
 *
 * Object names and package paths are presentation details. Canonical identity,
 * revision, feature ownership, dependencies, and provenance remain explicit so
 * imported content and mod overlays can be rebuilt without changing save or
 * gameplay meaning.
 */
UCLASS(Abstract, BlueprintType)
class SHARCONTENT_API USharPrimaryContentDefinition : public UPrimaryDataAsset
{
    GENERATED_BODY()

public:
    /** Stable lowercase snake-case identity within the concrete asset type. */
    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Identity")
    FName CanonicalId;

    /** Localizable player-facing name; never used as identity. */
    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Identity")
    FText DisplayName;

    /** Alternate lowercase snake-case lookup names for migration and search. */
    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Identity")
    TArray<FName> Aliases;

    /** Public-safe deterministic package identities that produced this asset. */
    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Provenance")
    TArray<FName> SourcePackageIds;

    /** Semantic classifications and capabilities; never canonical identity. */
    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Classification")
    FGameplayTagContainer ContentTags;

    /** Definitions that must resolve before this definition can activate. */
    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Dependencies")
    TArray<FPrimaryAssetId> RequiredDefinitions;

    /** Deterministic source-data revision used for rebuild and compatibility. */
    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Revision")
    FString RevisionToken;

    /** Version of the native definition schema, independent from content data. */
    UPROPERTY(
        EditDefaultsOnly,
        BlueprintReadOnly,
        Category = "Revision",
        meta = (ClampMin = "1", UIMin = "1")
    )
    int32 DefinitionSchemaVersion = 1;

    /** Exact validator profile required before publication or activation. */
    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Validation")
    FName ValidationProfile;

    /** Base game or namespaced Game Feature that owns this definition layer. */
    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Modding")
    FName OwningFeature = FName(TEXT("base"));

    /** Return domain identity rather than deriving identity from object name. */
    [[nodiscard]] FPrimaryAssetId GetPrimaryAssetId() const override;

    /** Collect deterministic, load-free validation errors for this definition. */
    virtual void GatherValidationErrors(TArray<FText>& OutErrors) const;

    /** Return whether a name satisfies the canonical lowercase identifier rule. */
    [[nodiscard]] static bool IsCanonicalIdentifier(const FName& Candidate);

#if WITH_EDITOR
    /** Integrate the load-free contract with Unreal Data Validation. */
    [[nodiscard]] EDataValidationResult IsDataValid(
        FDataValidationContext& Context
    ) const override;
#endif

protected:
    /** Concrete families provide their fixed Asset Manager type. */
    [[nodiscard]] virtual FPrimaryAssetType GetDefinitionAssetType() const;
};
