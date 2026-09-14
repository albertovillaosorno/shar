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
//   - Synthetic Asset Manager fixture for root runtime loading tests.
// - Must-Not:
//   - Register production assets, save packages, or mutate Asset Manager
//   - configuration.
// - Allows:
//   - Serving transient catalog and application-mode objects deterministically.
// - Split-When:
//   - Another runtime asset family needs independent fixture behavior.
// - Merge-When:
//   - Another test fixture owns the identical Asset Manager substitution.
// - Summary:
//   - Provides a transient Asset Manager inventory for Automation.
// - Description:
//   - Records exact root-load requests and completes delegates in memory.
// - Usage:
//   - Constructed only by root runtime Automation tests.
// - Defaults:
//   - Contains no assets until explicitly configured by a test.
//

//! Synthetic Asset Manager inventory for root runtime Automation.

#pragma once

#include "CoreMinimal.h"
#include "Engine/AssetManager.h"

#include "SharRuntimeAssetManagerFixture.generated.h"

class USharApplicationModeDefinition;
class USharGameplayCatalog;

UCLASS(Transient)
class USharRuntimeTestAssetManager final : public UAssetManager
{
    GENERATED_BODY()

public:
    void ConfigureInventory(
        USharGameplayCatalog* InCatalog,
        const TArray<USharApplicationModeDefinition*>& InModes
    );

    bool GetPrimaryAssetIdList(
        FPrimaryAssetType PrimaryAssetType,
        TArray<FPrimaryAssetId>& PrimaryAssetIdList,
        EAssetManagerFilter Filter
    ) const override;

    UObject* GetPrimaryAssetObject(
        const FPrimaryAssetId& PrimaryAssetId
    ) const override;

    TSharedPtr<FStreamableHandle> LoadPrimaryAsset(
        const FPrimaryAssetId& AssetToLoad,
        const TArray<FName>& LoadBundles,
        FStreamableDelegate DelegateToCall,
        TAsyncLoadPriority Priority,
        UE::FSourceLocation Location
    ) override;

    TSharedPtr<FStreamableHandle> LoadPrimaryAssets(
        const TArray<FPrimaryAssetId>& AssetsToLoad,
        const TArray<FName>& LoadBundles,
        FStreamableDelegate DelegateToCall,
        TAsyncLoadPriority Priority,
        UE::FSourceLocation Location
    ) override;

    void SetAutoCompleteModeFamily(bool bInAutoCompleteModeFamily);
    void ReplayCatalogCompletion();
    void ReplayModeFamilyCompletion();

    [[nodiscard]] int32 GetCatalogLoadCount() const;
    [[nodiscard]] int32 GetModeFamilyLoadCount() const;
    [[nodiscard]] const TArray<FPrimaryAssetId>&
    GetRequestedModeIds() const;

private:
    UPROPERTY(Transient)
    TObjectPtr<USharGameplayCatalog> Catalog;

    UPROPERTY(Transient)
    TArray<TObjectPtr<USharApplicationModeDefinition>> Modes;

    TArray<FPrimaryAssetId> RequestedModeIds;
    FStreamableDelegate CatalogCompletion;
    FStreamableDelegate ModeFamilyCompletion;
    int32 CatalogLoadCount = 0;
    int32 ModeFamilyLoadCount = 0;
    bool bAutoCompleteModeFamily = true;
};
