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
//   - Runtime composition-root contract tests.
// - Must-Not:
//   - Depend on imported art, maps, or editor mutations.
// - Allows:
//   - Synthetic root catalogs and application-mode definitions.
// - Split-When:
//   - Asset Manager loading requires a separate integration fixture.
// - Merge-When:
//   - Another suite proves the identical composition invariant.
// - Summary:
//   - Verifies root-catalog-to-application composition.
// - Description:
//   - Proves one complete mode family configures exactly one coordinator.
// - Usage:
//   - Runs through Unreal Automation without final content assets.
// - Defaults:
//   - Uses a three-mode synthetic lifecycle graph.
//

//! Runtime composition-root contract tests.

#if WITH_DEV_AUTOMATION_TESTS

#include "Runtime/SharRuntimeBootstrap.h"

#include "Application/SharApplicationModeCatalogSubsystem.h"
#include "Application/SharApplicationModeCoordinator.h"
#include "Catalog/SharGameplayCatalog.h"
#include "Catalog/SharGameplayCatalogSubsystem.h"
#include "Engine/GameInstance.h"
#include "Misc/AutomationTest.h"

namespace
{
USharApplicationModeDefinition* MakeMode(
    const FName& Id,
    const ESharApplicationModeKind Kind,
    const TArray<FName>& Predecessors,
    const TArray<FName>& Successors
)
{
    auto* Mode = NewObject<USharApplicationModeDefinition>();
    Mode->CanonicalId = Id;
    Mode->DisplayName = FText::FromName(Id);
    Mode->SourcePackageIds = {FName(TEXT("runtime_bootstrap_contract"))};
    Mode->RevisionToken = TEXT("sha256:runtime_mode_v1");
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
    FSharRuntimeBootstrapApplicationTest,
    "SHAR.Runtime.Bootstrap.ApplicationComposition",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
} // namespace

bool FSharRuntimeBootstrapApplicationTest::RunTest(const FString& Parameters)
{
    (void)Parameters;
    auto* GameInstance = NewObject<UGameInstance>();
    auto* RootDefinition = NewObject<USharGameplayCatalog>();
    RootDefinition->CanonicalId = FName(TEXT("gameplay"));
    RootDefinition->DisplayName = FText::FromString(TEXT("Gameplay catalog"));
    RootDefinition->SourcePackageIds = {FName(TEXT("runtime_catalog"))};
    RootDefinition->RevisionToken = TEXT("sha256:runtime_catalog_v1");
    RootDefinition->ValidationProfile = FName(TEXT("gameplay_catalog_v1"));
    RootDefinition->OwningFeature = FName(TEXT("base"));

    TArray<USharApplicationModeDefinition*> Modes = {
        MakeMode(
            FName(TEXT("entry")),
            ESharApplicationModeKind::Entry,
            {},
            {FName(TEXT("gameplay"))}
        ),
        MakeMode(
            FName(TEXT("gameplay")),
            ESharApplicationModeKind::Active,
            {FName(TEXT("entry"))},
            {FName(TEXT("exit"))}
        ),
        MakeMode(
            FName(TEXT("exit")),
            ESharApplicationModeKind::Exit,
            {FName(TEXT("gameplay"))},
            {}
        ),
    };
    FSharGameplayCatalogFamily ModeFamily;
    ModeFamily.FamilyId = FName(TEXT("application_modes"));
    ModeFamily.PrimaryAssetTypeName = FName(TEXT("SharApplicationMode"));
    for (const USharApplicationModeDefinition* Mode : Modes)
    {
        ModeFamily.DefinitionIds.Add(Mode->GetPrimaryAssetId());
    }
    RootDefinition->Families.Add(ModeFamily);

    auto* RootCatalog =
        NewObject<USharGameplayCatalogSubsystem>(GameInstance);
    auto* ApplicationCatalog =
        NewObject<USharApplicationModeCatalogSubsystem>(GameInstance);
    auto* Coordinator =
        NewObject<USharApplicationModeCoordinator>(GameInstance);
    auto* Bootstrap = NewObject<USharRuntimeBootstrap>(GameInstance);
    TestTrue(
        TEXT("Root catalog activates"),
        RootCatalog->Activate(RootDefinition)
            == ESharGameplayCatalogActivationResult::Accepted
    );

    Modes[0]->ModeKind = ESharApplicationModeKind::Boot;
    TestTrue(
        TEXT("Mode family without entry fails before composition"),
        Bootstrap->ConfigureApplicationRuntime(
            RootCatalog,
            ApplicationCatalog,
            Coordinator,
            Modes
        ) == ESharRuntimeBootstrapResult::CoordinatorRejected
    );
    TestFalse(
        TEXT("Missing entry leaves bootstrap unconfigured"),
        Bootstrap->IsApplicationRuntimeConfigured()
    );
    Modes[0]->ModeKind = ESharApplicationModeKind::Entry;

    TArray<USharApplicationModeDefinition*> IncompleteModes = Modes;
    IncompleteModes.Pop();
    TestTrue(
        TEXT("Incomplete declared mode set fails without configuration"),
        Bootstrap->ConfigureApplicationRuntime(
            RootCatalog,
            ApplicationCatalog,
            Coordinator,
            IncompleteModes
        ) == ESharRuntimeBootstrapResult::IncompleteApplicationModes
    );
    TestFalse(
        TEXT("Rejected preflight leaves bootstrap unconfigured"),
        Bootstrap->IsApplicationRuntimeConfigured()
    );
    TestTrue(
        TEXT("Root composition configures application runtime"),
        Bootstrap->ConfigureApplicationRuntime(
            RootCatalog,
            ApplicationCatalog,
            Coordinator,
            Modes
        ) == ESharRuntimeBootstrapResult::Accepted
    );
    const FSharApplicationModeObservation Initial =
        Coordinator->GetObservation();
    TestTrue(
        TEXT("Coordinator derives the declared entry mode"),
        Initial.ActiveModeId == FName(TEXT("entry"))
    );
    TestTrue(
        TEXT("Coordinator derives the entry definition revision"),
        Initial.ActiveModeRevision == TEXT("sha256:runtime_mode_v1")
    );
    TestTrue(TEXT("Initial entry has no world"), Initial.WorldId.IsNone());
    TestTrue(
        TEXT("Initial entry has no world revision"),
        Initial.WorldRevision.IsEmpty()
    );
    TestTrue(
        TEXT("Initial entry has no profile revision"),
        Initial.ProfileRevision.IsEmpty()
    );
    TestTrue(
        TEXT("Initial entry has no session revision"),
        Initial.SessionRevision.IsEmpty()
    );
    TestTrue(
        TEXT("Application runtime configures exactly once"),
        Bootstrap->ConfigureApplicationRuntime(
            RootCatalog,
            ApplicationCatalog,
            Coordinator,
            Modes
        ) == ESharRuntimeBootstrapResult::AlreadyConfigured
    );
    return true;
}

#endif
