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
//   - Successful runtime asset-loading-to-application-composition test.
// - Must-Not:
//   - Depend on authored production assets or the global Asset Manager state.
// - Allows:
//   - Transient catalog, mode definitions, and Asset Manager substitution.
// - Split-When:
//   - Another startup domain requires an independent integration transaction.
// - Merge-When:
//   - Another suite proves the same root startup composition contract.
// - Summary:
//   - Verifies root runtime loading publishes one composable terminal.
// - Description:
//   - Loads exactly the declared mode family and composes the entry authority.
// - Usage:
//   - Runs in headless Automation without creating or saving asset packages.
// - Defaults:
//   - Uses a three-mode synthetic lifecycle graph.
//

//! Successful root runtime asset loading and composition Automation.

#if WITH_DEV_AUTOMATION_TESTS

#include "Runtime/SharRuntimeAssetLoader.h"
#include "Runtime/SharRuntimeBootstrap.h"

#include "Application/SharApplicationModeCatalogSubsystem.h"
#include "Application/SharApplicationModeCoordinator.h"
#include "Application/SharApplicationModeDefinition.h"
#include "Catalog/SharGameplayCatalog.h"
#include "Catalog/SharGameplayCatalogSubsystem.h"
#include "Engine/GameInstance.h"
#include "Misc/AutomationTest.h"
#include "Tests/SharRuntimeAssetManagerFixture.h"

namespace
{
USharApplicationModeDefinition* MakeLoadedMode(
    const FName& Id,
    const ESharApplicationModeKind Kind,
    const TArray<FName>& Predecessors,
    const TArray<FName>& Successors
)
{
    auto* Mode = NewObject<USharApplicationModeDefinition>();
    Mode->CanonicalId = Id;
    Mode->DisplayName = FText::FromName(Id);
    Mode->SourcePackageIds = {FName(TEXT("runtime_loading_contract"))};
    Mode->RevisionToken = FString::Printf(TEXT("sha256:%s_v1"), *Id.ToString());
    Mode->ValidationProfile = FName(TEXT("runtime_mode_v1"));
    Mode->OwningFeature = FName(TEXT("base"));
    Mode->ModeKind = Kind;
    Mode->AllowedPredecessorIds = Predecessors;
    Mode->AllowedSuccessorIds = Successors;
    Mode->EntryPlanId = FName(TEXT("entry_plan"));
    Mode->ExitPlanId = FName(TEXT("exit_plan"));
    Mode->ReadinessBarrierId = FName(TEXT("ready_barrier"));
    Mode->bSupportsCancellation = true;
    Mode->bHasBoundedTimeout = true;
    return Mode;
}

IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharRuntimeAssetLoadingCompositionTest,
    "SHAR.Runtime.AssetLoading.ApplicationComposition",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
} // namespace

bool FSharRuntimeAssetLoadingCompositionTest::RunTest(const FString& Parameters)
{
    (void)Parameters;
    auto* GameInstance = NewObject<UGameInstance>();
    auto* RootCatalog =
        NewObject<USharGameplayCatalogSubsystem>(GameInstance);
    auto* ApplicationCatalog =
        NewObject<USharApplicationModeCatalogSubsystem>(GameInstance);
    auto* Coordinator =
        NewObject<USharApplicationModeCoordinator>(GameInstance);
    auto* Bootstrap = NewObject<USharRuntimeBootstrap>(GameInstance);
    auto* Loader = NewObject<USharRuntimeAssetLoader>(GameInstance);

    TArray<USharApplicationModeDefinition*> Modes = {
        MakeLoadedMode(
            FName(TEXT("entry")),
            ESharApplicationModeKind::Entry,
            {},
            {FName(TEXT("gameplay"))}
        ),
        MakeLoadedMode(
            FName(TEXT("gameplay")),
            ESharApplicationModeKind::Active,
            {FName(TEXT("entry"))},
            {FName(TEXT("exit"))}
        ),
        MakeLoadedMode(
            FName(TEXT("exit")),
            ESharApplicationModeKind::Exit,
            {FName(TEXT("gameplay"))},
            {}
        ),
    };

    auto* Catalog = NewObject<USharGameplayCatalog>();
    Catalog->CanonicalId = FName(TEXT("gameplay"));
    Catalog->DisplayName = FText::FromString(TEXT("Gameplay catalog"));
    Catalog->SourcePackageIds = {FName(TEXT("runtime_loading_catalog"))};
    Catalog->RevisionToken = TEXT("sha256:runtime_loading_catalog_v1");
    Catalog->ValidationProfile = FName(TEXT("gameplay_catalog_v1"));
    Catalog->OwningFeature = FName(TEXT("base"));
    FSharGameplayCatalogFamily ModeFamily;
    ModeFamily.FamilyId = FName(TEXT("application_modes"));
    ModeFamily.PrimaryAssetTypeName = FName(TEXT("SharApplicationMode"));
    for (const USharApplicationModeDefinition* Mode : Modes)
    {
        ModeFamily.DefinitionIds.Add(Mode->GetPrimaryAssetId());
    }
    Catalog->Families.Add(ModeFamily);

    auto* AssetManager = NewObject<USharRuntimeTestAssetManager>();
    AssetManager->ConfigureInventory(Catalog, Modes);
    AssetManager->SetAutoCompleteModeFamily(false);

    int32 TerminalCount = 0;
    ESharRuntimeBootstrapResult CompositionResult =
        ESharRuntimeBootstrapResult::InvalidRootCatalog;
    Loader->OnTerminal().AddLambda(
        [&](const ESharRuntimeAssetLoadStatus Status,
            const ESharRuntimeAssetLoadFailure Failure)
        {
            ++TerminalCount;
            if (Status == ESharRuntimeAssetLoadStatus::Ready
                && Failure == ESharRuntimeAssetLoadFailure::None)
            {
                CompositionResult = Bootstrap->ConfigureApplicationRuntime(
                    RootCatalog,
                    ApplicationCatalog,
                    Coordinator,
                    Loader->GetLoadedApplicationModes()
                );
            }
        }
    );

    TestTrue(
        TEXT("Synthetic inventory starts loading"),
        Loader->Start(AssetManager, RootCatalog)
            == ESharRuntimeAssetLoadStartResult::Started
    );
    TestTrue(
        TEXT("Catalog completion advances only to mode loading"),
        Loader->GetStatus()
            == ESharRuntimeAssetLoadStatus::LoadingApplicationModes
            && Loader->GetFailure() == ESharRuntimeAssetLoadFailure::None
    );
    TestEqual(TEXT("Mode loading is not terminal"), TerminalCount, 0);
    TestEqual(
        TEXT("Catalog loads exactly once"),
        AssetManager->GetCatalogLoadCount(),
        1
    );
    TestEqual(
        TEXT("Declared mode family loads exactly once"),
        AssetManager->GetModeFamilyLoadCount(),
        1
    );
    TestTrue(
        TEXT("Loader requests only the catalogued mode identities"),
        AssetManager->GetRequestedModeIds() == ModeFamily.DefinitionIds
    );
    AssetManager->ReplayCatalogCompletion();
    TestTrue(
        TEXT("Stale catalog completion cannot abort mode loading"),
        Loader->GetStatus()
            == ESharRuntimeAssetLoadStatus::LoadingApplicationModes
            && Loader->GetFailure() == ESharRuntimeAssetLoadFailure::None
    );
    TestEqual(
        TEXT("Stale catalog completion is not terminal"),
        TerminalCount,
        0
    );
    AssetManager->ReplayModeFamilyCompletion();
    TestTrue(
        TEXT("Mode completion reaches ready"),
        Loader->GetStatus() == ESharRuntimeAssetLoadStatus::Ready
            && Loader->GetFailure() == ESharRuntimeAssetLoadFailure::None
    );
    TestEqual(TEXT("Ready terminal publishes once"), TerminalCount, 1);
    TestTrue(
        TEXT("Ready terminal composes application runtime"),
        CompositionResult == ESharRuntimeBootstrapResult::Accepted
            && Bootstrap->IsApplicationRuntimeConfigured()
            && ApplicationCatalog->IsActive()
            && ApplicationCatalog->GetModeCount() == Modes.Num()
    );
    const FSharApplicationModeObservation Observation =
        Coordinator->GetObservation();
    TestTrue(
        TEXT("Composed coordinator starts at loaded entry revision"),
        Observation.ActiveModeId == FName(TEXT("entry"))
            && Observation.ActiveModeRevision == Modes[0]->RevisionToken
            && Observation.WorldId.IsNone()
            && Observation.WorldRevision.IsEmpty()
            && Observation.ProfileRevision.IsEmpty()
            && Observation.SessionRevision.IsEmpty()
    );
    AssetManager->ReplayCatalogCompletion();
    AssetManager->ReplayModeFamilyCompletion();
    TestTrue(
        TEXT("Late completion callbacks cannot change the ready terminal"),
        Loader->GetStatus() == ESharRuntimeAssetLoadStatus::Ready
            && Loader->GetFailure() == ESharRuntimeAssetLoadFailure::None
    );
    TestEqual(
        TEXT("Late callbacks cannot republish terminal"),
        TerminalCount,
        1
    );
    TestTrue(
        TEXT("Ready loader cannot restart"),
        Loader->Start(AssetManager, RootCatalog)
            == ESharRuntimeAssetLoadStartResult::AlreadyStarted
    );
    TestEqual(TEXT("Restart cannot republish terminal"), TerminalCount, 1);
    return true;
}

#endif
