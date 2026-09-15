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
//   - Process-lifetime Unreal game-instance composition boundary.
// - Must-Not:
//   - Own application-mode state or duplicate domain coordinator state.
// - Allows:
//   - Native Unreal game-instance lifecycle and subsystem composition.
// - Split-When:
//   - One composed adapter gains an independent lifecycle.
// - Merge-When:
//   - Another project game instance owns the identical composition boundary.
// - Summary:
//   - Implements the state-free SHAR game-instance shell.
// - Description:
//   - Intentionally contains no startup state machine.
// - Usage:
//   - Unreal constructs it through project game settings.
// - Defaults:
//   - Base UGameInstance behavior only.
//

//! State-free SHAR game-instance composition shell.

#include "Runtime/SharGameInstance.h"

#include "Application/SharApplicationModeCatalogSubsystem.h"
#include "Application/SharApplicationModeCoordinator.h"
#include "Catalog/SharGameplayCatalogSubsystem.h"
#include "Engine/AssetManager.h"

void USharGameInstance::Init()
{
    Super::Init();
    RuntimeBootstrap = NewObject<USharRuntimeBootstrap>(this);
    RuntimeAssetLoader = NewObject<USharRuntimeAssetLoader>(this);
    GameplayTransitionComposer =
        NewObject<USharGameplayTransitionComposer>(this);
    RuntimeAssetLoader->OnTerminal().AddUObject(
        this,
        &USharGameInstance::HandleRuntimeAssetLoadTerminal
    );
    UAssetManager::CallOrRegister_OnCompletedInitialScan(
        FSimpleDelegate::CreateUObject(
            this,
            &USharGameInstance::StartRuntimeAssetLoading
        )
    );
}

USharRuntimeBootstrap* USharGameInstance::GetRuntimeBootstrap() const
{
    return RuntimeBootstrap;
}

USharRuntimeAssetLoader* USharGameInstance::GetRuntimeAssetLoader() const
{
    return RuntimeAssetLoader;
}

USharGameplayTransitionComposer*
USharGameInstance::GetGameplayTransitionComposer() const
{
    return GameplayTransitionComposer;
}

void USharGameInstance::StartRuntimeAssetLoading()
{
    if (RuntimeAssetLoader == nullptr)
    {
        return;
    }
    RuntimeAssetLoader->Start(
        UAssetManager::GetIfInitialized(),
        GetSubsystem<USharGameplayCatalogSubsystem>()
    );
}

void USharGameInstance::HandleRuntimeAssetLoadTerminal(
    const ESharRuntimeAssetLoadStatus Status,
    const ESharRuntimeAssetLoadFailure Failure
)
{
    if (Status != ESharRuntimeAssetLoadStatus::Ready
        || Failure != ESharRuntimeAssetLoadFailure::None
        || RuntimeAssetLoader == nullptr
        || RuntimeBootstrap == nullptr)
    {
        return;
    }
    RuntimeBootstrap->ConfigureApplicationRuntime(
        GetSubsystem<USharGameplayCatalogSubsystem>(),
        GetSubsystem<USharApplicationModeCatalogSubsystem>(),
        GetSubsystem<USharApplicationModeCoordinator>(),
        RuntimeAssetLoader->GetLoadedApplicationModes()
    );
}
