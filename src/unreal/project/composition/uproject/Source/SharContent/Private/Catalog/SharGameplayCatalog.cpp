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
//   - Root gameplay catalog validation and read-only lookup.
// - Must-Not:
//   - Load assets, mutate gameplay state, or infer identity from paths.
// - Allows:
//   - Deterministic validation of declared Primary Asset identities.
// - Split-When:
//   - Alias resolution or loading gains an independent lifecycle.
// - Merge-When:
//   - Another root catalog owns the identical registry contract.
// - Summary:
//   - Implements immutable root gameplay catalog behavior.
// - Description:
//   - Rejects malformed, duplicate, or cross-family Primary Asset identities.
// - Usage:
//   - Called before the runtime catalog subsystem accepts a revision.
// - Defaults:
//   - Invalid catalog data fails closed.
//

//! Root gameplay catalog validation and lookup.

#include "Catalog/SharGameplayCatalog.h"

#include "Content/SharPrimaryContentDefinition.h"

namespace
{
void AddCatalogError(TArray<FText>& OutErrors, const TCHAR* Message)
{
    OutErrors.Add(FText::FromString(Message));
}

bool IsMatchingFamilyType(
    const FSharGameplayCatalogFamily& Family,
    const FPrimaryAssetId& DefinitionId
)
{
    return DefinitionId.PrimaryAssetType.ToString()
        == Family.PrimaryAssetTypeName.ToString();
}
} // namespace

void USharGameplayCatalog::GatherValidationErrors(
    TArray<FText>& OutErrors
) const
{
    USharPrimaryContentDefinition::GatherValidationErrors(OutErrors);
    if (Families.IsEmpty())
    {
        AddCatalogError(
            OutErrors,
            TEXT("Gameplay catalog requires at least one definition family.")
        );
        return;
    }

    TSet<FName> SeenFamilyIds;
    TSet<FName> SeenAssetTypes;
    TSet<FPrimaryAssetId> SeenDefinitions;
    for (const FSharGameplayCatalogFamily& Family : Families)
    {
        if (!IsCanonicalIdentifier(Family.FamilyId))
        {
            AddCatalogError(
                OutErrors,
                TEXT("Gameplay catalog family identity must be canonical.")
            );
        }
        if (Family.PrimaryAssetTypeName.IsNone())
        {
            AddCatalogError(
                OutErrors,
                TEXT("Gameplay catalog family requires a Primary Asset type.")
            );
        }
        if (SeenFamilyIds.Contains(Family.FamilyId))
        {
            AddCatalogError(
                OutErrors,
                TEXT("Gameplay catalog family identities must be unique.")
            );
        }
        if (SeenAssetTypes.Contains(Family.PrimaryAssetTypeName))
        {
            AddCatalogError(
                OutErrors,
                TEXT("Gameplay catalog Primary Asset types must be unique.")
            );
        }
        if (Family.DefinitionIds.IsEmpty())
        {
            AddCatalogError(
                OutErrors,
                TEXT("Gameplay catalog family cannot be empty.")
            );
        }
        SeenFamilyIds.Add(Family.FamilyId);
        SeenAssetTypes.Add(Family.PrimaryAssetTypeName);

        for (const FPrimaryAssetId& DefinitionId : Family.DefinitionIds)
        {
            if (!DefinitionId.IsValid())
            {
                AddCatalogError(
                    OutErrors,
                    TEXT("Gameplay catalog contains an invalid definition id.")
                );
                continue;
            }
            if (!IsMatchingFamilyType(Family, DefinitionId))
            {
                AddCatalogError(
                    OutErrors,
                    TEXT(
                        "Gameplay catalog definition has the wrong family type."
                    )
                );
            }
            if (!IsCanonicalIdentifier(DefinitionId.PrimaryAssetName))
            {
                AddCatalogError(
                    OutErrors,
                    TEXT(
                        "Gameplay catalog definition identity must be "
                        "canonical."
                    )
                );
            }
            if (DefinitionId.PrimaryAssetType
                == FPrimaryAssetType(TEXT("SharCatalog")))
            {
                AddCatalogError(
                    OutErrors,
                    TEXT("Gameplay catalog cannot recursively catalog itself.")
                );
            }
            if (SeenDefinitions.Contains(DefinitionId))
            {
                AddCatalogError(
                    OutErrors,
                    TEXT(
                        "Gameplay catalog definition identities must be unique."
                    )
                );
            }
            SeenDefinitions.Add(DefinitionId);
        }
    }
}

const FSharGameplayCatalogFamily* USharGameplayCatalog::FindFamilyByType(
    const FPrimaryAssetType& PrimaryAssetType
) const
{
    return Families.FindByPredicate(
        [&PrimaryAssetType](const FSharGameplayCatalogFamily& Family)
        {
            const FName TypeName(*PrimaryAssetType.ToString());
            return Family.PrimaryAssetTypeName == TypeName;
        }
    );
}

bool USharGameplayCatalog::ContainsDefinition(
    const FPrimaryAssetId& DefinitionId
) const
{
    const FSharGameplayCatalogFamily* Family =
        FindFamilyByType(DefinitionId.PrimaryAssetType);
    return Family != nullptr && Family->DefinitionIds.Contains(DefinitionId);
}

FPrimaryAssetType USharGameplayCatalog::GetDefinitionAssetType() const
{
    return {TEXT("SharCatalog")};
}
