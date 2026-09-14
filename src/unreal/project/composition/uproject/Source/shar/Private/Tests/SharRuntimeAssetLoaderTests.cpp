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
//   - Runtime Primary Asset loading preflight tests.
// - Must-Not:
//   - Depend on imported game content or mutate project Asset Manager settings.
// - Allows:
//   - Synthetic empty Asset Manager and root catalog collaborators.
// - Split-When:
//   - Loaded-asset completion needs independently generated asset fixtures.
// - Merge-When:
//   - Another suite proves the same startup Asset Manager preflight contract.
// - Summary:
//   - Verifies fail-closed root runtime asset discovery.
// - Description:
//   - Rejects absent Asset Manager dependencies and ambiguous catalog counts.
// - Usage:
//   - Runs through Unreal Automation without final content assets.
// - Defaults:
//   - Uses an empty synthetic Asset Manager inventory.
//

//! Runtime Primary Asset loading preflight tests.

#if WITH_DEV_AUTOMATION_TESTS

#include "Runtime/SharRuntimeAssetLoader.h"

#include "Catalog/SharGameplayCatalogSubsystem.h"
#include "Engine/AssetManager.h"
#include "Engine/GameInstance.h"
#include "Misc/AutomationTest.h"

IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharRuntimeAssetLoaderPreflightTest,
    "SHAR.Runtime.AssetLoading.Preflight",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)

bool FSharRuntimeAssetLoaderPreflightTest::RunTest(const FString& Parameters)
{
    (void)Parameters;

    auto* MissingDependencyLoader = NewObject<USharRuntimeAssetLoader>();
    TestTrue(
        TEXT("Missing Asset Manager is rejected"),
        MissingDependencyLoader->Start(nullptr, nullptr)
            == ESharRuntimeAssetLoadStartResult::InvalidDependencies
    );
    TestTrue(
        TEXT("Missing dependency failure is observable"),
        MissingDependencyLoader->GetStatus()
            == ESharRuntimeAssetLoadStatus::Failed
            && MissingDependencyLoader->GetFailure()
                == ESharRuntimeAssetLoadFailure::AssetManagerUnavailable
    );
    TestTrue(
        TEXT("A failed loader cannot be restarted in place"),
        MissingDependencyLoader->Start(nullptr, nullptr)
            == ESharRuntimeAssetLoadStartResult::AlreadyStarted
    );

    auto* GameInstance = NewObject<UGameInstance>();
    auto* RootCatalog =
        NewObject<USharGameplayCatalogSubsystem>(GameInstance);
    auto* EmptyAssetManager = NewObject<UAssetManager>();
    auto* EmptyInventoryLoader = NewObject<USharRuntimeAssetLoader>();
    int32 EmptyInventoryTerminalCount = 0;
    EmptyInventoryLoader->OnTerminal().AddLambda(
        [&EmptyInventoryTerminalCount](
            const ESharRuntimeAssetLoadStatus Status,
            const ESharRuntimeAssetLoadFailure Failure
        )
        {
            if (Status == ESharRuntimeAssetLoadStatus::Failed
                && Failure == ESharRuntimeAssetLoadFailure::CatalogNotUnique)
            {
                ++EmptyInventoryTerminalCount;
            }
        }
    );
    TestTrue(
        TEXT("Zero root catalogs fail closed"),
        EmptyInventoryLoader->Start(EmptyAssetManager, RootCatalog)
            == ESharRuntimeAssetLoadStartResult::CatalogNotUnique
    );
    TestTrue(
        TEXT("Catalog count failure is typed"),
        EmptyInventoryLoader->GetStatus()
            == ESharRuntimeAssetLoadStatus::Failed
            && EmptyInventoryLoader->GetFailure()
                == ESharRuntimeAssetLoadFailure::CatalogNotUnique
    );
    TestTrue(
        TEXT("Failed discovery exposes no application modes"),
        EmptyInventoryLoader->GetLoadedApplicationModes().IsEmpty()
    );
    TestEqual(
        TEXT("Failed discovery publishes one terminal"),
        EmptyInventoryTerminalCount,
        1
    );
    TestTrue(
        TEXT("Repeated start remains rejected"),
        EmptyInventoryLoader->Start(EmptyAssetManager, RootCatalog)
            == ESharRuntimeAssetLoadStartResult::AlreadyStarted
    );
    TestEqual(
        TEXT("Repeated start cannot republish the terminal"),
        EmptyInventoryTerminalCount,
        1
    );
    return true;
}

#endif
