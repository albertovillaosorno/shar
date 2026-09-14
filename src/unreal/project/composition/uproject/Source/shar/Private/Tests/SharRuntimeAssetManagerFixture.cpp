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
//   - Synthetic Asset Manager fixture implementation for runtime tests.
// - Must-Not:
//   - Touch disk, package state, or the process Asset Manager singleton.
// - Allows:
//   - Deterministic in-memory Primary Asset discovery and completion.
// - Split-When:
//   - Fixture loading behavior gains an independent lifecycle.
// - Merge-When:
//   - Another test adapter owns the identical substitution behavior.
// - Summary:
//   - Implements the transient runtime Asset Manager fixture.
// - Description:
//   - Returns configured objects and controls requested load completion.
// - Usage:
//   - Linked only into the project runtime module's Automation surface.
// - Defaults:
//   - Unknown identities resolve to null and no synthetic fallback exists.
//

//! Transient Asset Manager fixture for root runtime Automation.

#include "Tests/SharRuntimeAssetManagerFixture.h"

#include "Application/SharApplicationModeDefinition.h"
#include "Catalog/SharGameplayCatalog.h"

namespace
{
const FPrimaryAssetType SharRuntimeTestCatalogType(TEXT("SharCatalog"));
}

void USharRuntimeTestAssetManager::ConfigureInventory(
    USharGameplayCatalog* InCatalog,
    const TArray<USharApplicationModeDefinition*>& InModes
)
{
    Catalog = InCatalog;
    Modes.Reset(InModes.Num());
    for (USharApplicationModeDefinition* Mode : InModes)
    {
        Modes.Add(Mode);
    }
    RequestedModeIds.Reset();
    CatalogCompletion.Unbind();
    ModeFamilyCompletion.Unbind();
    CatalogLoadCount = 0;
    ModeFamilyLoadCount = 0;
}

bool USharRuntimeTestAssetManager::GetPrimaryAssetIdList(
    const FPrimaryAssetType PrimaryAssetType,
    TArray<FPrimaryAssetId>& PrimaryAssetIdList,
    const EAssetManagerFilter Filter
) const
{
    (void)Filter;
    PrimaryAssetIdList.Reset();
    if (PrimaryAssetType != SharRuntimeTestCatalogType || Catalog == nullptr)
    {
        return false;
    }
    PrimaryAssetIdList.Add(Catalog->GetPrimaryAssetId());
    return true;
}

UObject* USharRuntimeTestAssetManager::GetPrimaryAssetObject(
    const FPrimaryAssetId& PrimaryAssetId
) const
{
    if (Catalog != nullptr && Catalog->GetPrimaryAssetId() == PrimaryAssetId)
    {
        return Catalog;
    }
    for (USharApplicationModeDefinition* Mode : Modes)
    {
        if (Mode != nullptr && Mode->GetPrimaryAssetId() == PrimaryAssetId)
        {
            return Mode;
        }
    }
    return nullptr;
}

TSharedPtr<FStreamableHandle> USharRuntimeTestAssetManager::LoadPrimaryAsset(
    const FPrimaryAssetId& AssetToLoad,
    const TArray<FName>& LoadBundles,
    FStreamableDelegate DelegateToCall,
    const TAsyncLoadPriority Priority,
    UE::FSourceLocation Location
)
{
    (void)LoadBundles;
    (void)Priority;
    (void)Location;
    ++CatalogLoadCount;
    CatalogCompletion = DelegateToCall;
    DelegateToCall.ExecuteIfBound();
    return nullptr;
}

TSharedPtr<FStreamableHandle> USharRuntimeTestAssetManager::LoadPrimaryAssets(
    const TArray<FPrimaryAssetId>& AssetsToLoad,
    const TArray<FName>& LoadBundles,
    FStreamableDelegate DelegateToCall,
    const TAsyncLoadPriority Priority,
    UE::FSourceLocation Location
)
{
    (void)LoadBundles;
    (void)Priority;
    (void)Location;
    ++ModeFamilyLoadCount;
    RequestedModeIds = AssetsToLoad;
    ModeFamilyCompletion = DelegateToCall;
    if (bAutoCompleteModeFamily)
    {
        DelegateToCall.ExecuteIfBound();
    }
    return nullptr;
}

void USharRuntimeTestAssetManager::SetAutoCompleteModeFamily(
    const bool bInAutoCompleteModeFamily
)
{
    bAutoCompleteModeFamily = bInAutoCompleteModeFamily;
}

void USharRuntimeTestAssetManager::ReplayCatalogCompletion()
{
    CatalogCompletion.ExecuteIfBound();
}

void USharRuntimeTestAssetManager::ReplayModeFamilyCompletion()
{
    ModeFamilyCompletion.ExecuteIfBound();
}

int32 USharRuntimeTestAssetManager::GetCatalogLoadCount() const
{
    return CatalogLoadCount;
}

int32 USharRuntimeTestAssetManager::GetModeFamilyLoadCount() const
{
    return ModeFamilyLoadCount;
}

const TArray<FPrimaryAssetId>&
USharRuntimeTestAssetManager::GetRequestedModeIds() const
{
    return RequestedModeIds;
}
