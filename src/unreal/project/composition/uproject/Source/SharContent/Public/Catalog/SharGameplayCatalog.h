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
//   - Root gameplay catalog identity and family membership.
// - Must-Not:
//   - Load assets, mutate gameplay state, or infer identity from paths.
// - Allows:
//   - Stable Primary Asset identities grouped by registered family.
// - Split-When:
//   - Alias resolution or loading gains an independent lifecycle.
// - Merge-When:
//   - Another root catalog owns the identical registry contract.
// - Summary:
//   - Defines the immutable root gameplay catalog.
// - Description:
//   - Publishes one revisioned catalog of runtime Primary Asset identities.
// - Usage:
//   - Loaded through Asset Manager before domain catalogs activate.
// - Defaults:
//   - Contains no families until generated catalog data is supplied.
//

//! Immutable root gameplay catalog definition.

#pragma once

#include "Content/SharPrimaryContentDefinition.h"
#include "CoreMinimal.h"

#include "SharGameplayCatalog.generated.h"

USTRUCT(BlueprintType)
struct SHARCONTENT_API FSharGameplayCatalogFamily
{
    GENERATED_BODY()

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Identity")
    FName FamilyId;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Identity")
    FName PrimaryAssetTypeName;

    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Definitions")
    TArray<FPrimaryAssetId> DefinitionIds;
};

UCLASS()
class SHARCONTENT_API USharGameplayCatalog final
    : public USharPrimaryContentDefinition
{
    GENERATED_BODY()

public:
    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Catalog")
    TArray<FSharGameplayCatalogFamily> Families;

    void GatherValidationErrors(TArray<FText>& OutErrors) const override;

    [[nodiscard]] const FSharGameplayCatalogFamily* FindFamilyByType(
        const FPrimaryAssetType& PrimaryAssetType
    ) const;

    [[nodiscard]] bool ContainsDefinition(
        const FPrimaryAssetId& DefinitionId
    ) const;

protected:
    [[nodiscard]] FPrimaryAssetType GetDefinitionAssetType() const override;
};
