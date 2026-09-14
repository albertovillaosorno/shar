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
//   - Production Asset Manager loading for root runtime definitions.
// - Must-Not:
//   - Parse source formats, invent missing assets, or own application state.
// - Allows:
//   - Discovering and asynchronously loading the root catalog and mode family.
// - Split-When:
//   - Another Primary Asset family requires an independent loading lifecycle.
// - Merge-When:
//   - Another composition object owns the identical startup loading contract.
// - Summary:
//   - Implements root runtime Primary Asset loading.
// - Description:
//   - Resolves one catalog, activates it, then loads its declared mode family.
// - Usage:
//   - Started once by the project game instance after engine initialization.
// - Defaults:
//   - Missing or ambiguous runtime definitions fail closed.
//

//! Root runtime Primary Asset loading through Unreal Asset Manager.

#include "Runtime/SharRuntimeAssetLoader.h"

#include "Application/SharApplicationModeDefinition.h"
#include "Catalog/SharGameplayCatalog.h"
#include "Catalog/SharGameplayCatalogSubsystem.h"
#include "Engine/AssetManager.h"

namespace
{
const FPrimaryAssetType SharCatalogAssetType(TEXT("SharCatalog"));
const FPrimaryAssetType SharApplicationModeAssetType(
    TEXT("SharApplicationMode")
);
} // namespace

ESharRuntimeAssetLoadStartResult USharRuntimeAssetLoader::Start(
    UAssetManager* InAssetManager,
    USharGameplayCatalogSubsystem* InRootCatalog
)
{
    if (Status != ESharRuntimeAssetLoadStatus::Idle)
    {
        return ESharRuntimeAssetLoadStartResult::AlreadyStarted;
    }
    if (InAssetManager == nullptr)
    {
        Fail(ESharRuntimeAssetLoadFailure::AssetManagerUnavailable);
        return ESharRuntimeAssetLoadStartResult::InvalidDependencies;
    }
    if (InRootCatalog == nullptr)
    {
        Fail(ESharRuntimeAssetLoadFailure::RootCatalogUnavailable);
        return ESharRuntimeAssetLoadStartResult::InvalidDependencies;
    }

    AssetManager = InAssetManager;
    RootCatalog = InRootCatalog;
    TArray<FPrimaryAssetId> CatalogIds;
    AssetManager->GetPrimaryAssetIdList(SharCatalogAssetType, CatalogIds);
    if (CatalogIds.Num() != 1)
    {
        Fail(ESharRuntimeAssetLoadFailure::CatalogNotUnique);
        return ESharRuntimeAssetLoadStartResult::CatalogNotUnique;
    }

    CatalogAssetId = CatalogIds[0];
    Status = ESharRuntimeAssetLoadStatus::LoadingCatalog;
    AssetManager->LoadPrimaryAsset(
        CatalogAssetId,
        {},
        FStreamableDelegate::CreateUObject(
            this,
            &USharRuntimeAssetLoader::HandleCatalogLoaded
        )
    );
    return ESharRuntimeAssetLoadStartResult::Started;
}

ESharRuntimeAssetLoadStatus USharRuntimeAssetLoader::GetStatus() const
{
    return Status;
}

ESharRuntimeAssetLoadFailure USharRuntimeAssetLoader::GetFailure() const
{
    return Failure;
}

const USharGameplayCatalog* USharRuntimeAssetLoader::GetLoadedCatalog() const
{
    return LoadedCatalog;
}

TArray<USharApplicationModeDefinition*>
USharRuntimeAssetLoader::GetLoadedApplicationModes() const
{
    TArray<USharApplicationModeDefinition*> Modes;
    Modes.Reserve(LoadedApplicationModes.Num());
    for (USharApplicationModeDefinition* Mode : LoadedApplicationModes)
    {
        Modes.Add(Mode);
    }
    return Modes;
}

FSharRuntimeAssetLoadTerminalDelegate& USharRuntimeAssetLoader::OnTerminal()
{
    return TerminalDelegate;
}

void USharRuntimeAssetLoader::HandleCatalogLoaded()
{
    if (bTerminalPublished
        || Status != ESharRuntimeAssetLoadStatus::LoadingCatalog)
    {
        return;
    }
    if (AssetManager == nullptr || RootCatalog == nullptr)
    {
        Fail(ESharRuntimeAssetLoadFailure::CatalogLoadFailed);
        return;
    }

    LoadedCatalog = AssetManager->GetPrimaryAssetObject<USharGameplayCatalog>(
        CatalogAssetId
    );
    if (LoadedCatalog == nullptr)
    {
        Fail(ESharRuntimeAssetLoadFailure::CatalogLoadFailed);
        return;
    }
    if (RootCatalog->Activate(LoadedCatalog)
        != ESharGameplayCatalogActivationResult::Accepted)
    {
        Fail(ESharRuntimeAssetLoadFailure::CatalogActivationRejected);
        return;
    }

    const FSharGameplayCatalogFamily* ModeFamily =
        LoadedCatalog->FindFamilyByType(SharApplicationModeAssetType);
    if (ModeFamily == nullptr || ModeFamily->DefinitionIds.IsEmpty())
    {
        Fail(ESharRuntimeAssetLoadFailure::ApplicationModeFamilyMissing);
        return;
    }

    ApplicationModeAssetIds = ModeFamily->DefinitionIds;
    Status = ESharRuntimeAssetLoadStatus::LoadingApplicationModes;
    AssetManager->LoadPrimaryAssets(
        ApplicationModeAssetIds,
        {},
        FStreamableDelegate::CreateUObject(
            this,
            &USharRuntimeAssetLoader::HandleApplicationModesLoaded
        )
    );
}

void USharRuntimeAssetLoader::HandleApplicationModesLoaded()
{
    if (bTerminalPublished
        || Status != ESharRuntimeAssetLoadStatus::LoadingApplicationModes)
    {
        return;
    }
    if (AssetManager == nullptr)
    {
        Fail(ESharRuntimeAssetLoadFailure::ApplicationModeLoadFailed);
        return;
    }

    LoadedApplicationModes.Reset(ApplicationModeAssetIds.Num());
    for (const FPrimaryAssetId& ModeId : ApplicationModeAssetIds)
    {
        USharApplicationModeDefinition* Mode =
            AssetManager->GetPrimaryAssetObject<USharApplicationModeDefinition>(
                ModeId
            );
        if (Mode == nullptr || Mode->GetPrimaryAssetId() != ModeId)
        {
            LoadedApplicationModes.Reset();
            Fail(ESharRuntimeAssetLoadFailure::ApplicationModeLoadFailed);
            return;
        }
        LoadedApplicationModes.Add(Mode);
    }
    Failure = ESharRuntimeAssetLoadFailure::None;
    Status = ESharRuntimeAssetLoadStatus::Ready;
    PublishTerminal();
}

void USharRuntimeAssetLoader::PublishTerminal()
{
    if (bTerminalPublished)
    {
        return;
    }
    bTerminalPublished = true;
    TerminalDelegate.Broadcast(Status, Failure);
}

void USharRuntimeAssetLoader::Fail(const ESharRuntimeAssetLoadFailure Reason)
{
    if (bTerminalPublished)
    {
        return;
    }
    Failure = Reason;
    Status = ESharRuntimeAssetLoadStatus::Failed;
    PublishTerminal();
}
