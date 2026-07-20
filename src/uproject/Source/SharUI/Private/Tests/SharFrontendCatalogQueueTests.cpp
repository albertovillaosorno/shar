// File: SharFrontendCatalogQueueTests.cpp
// Path: src/uproject/Source/SharUI/Private/Tests/SharFrontendCatalogQueueTests.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: frontend catalog validation, cross-catalog uniqueness, and deterministic queue tests only.
// Specification: docs/technical/unreal/frontend-screen-flow-and-settings-runtime.md

#if WITH_DEV_AUTOMATION_TESTS

#include "SharFrontendTestFixtures.h"

#include "Engine/GameInstance.h"
#include "Misc/AutomationTest.h"
#include "UI/SharFrontendCatalogDefinition.h"
#include "UI/SharFrontendCatalogSubsystem.h"
#include "UI/SharFrontendFlowContracts.h"
#include "UI/SharFrontendFlowSubsystem.h"

namespace
{
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharFrontendCatalogValidationTest,
    "SHAR.Frontend.Catalog.Validation",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)

IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharFrontendCrossCatalogDuplicateTest,
    "SHAR.Frontend.Catalog.CrossCatalogDuplicate",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)

IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharFrontendDeterministicQueueTest,
    "SHAR.Frontend.Flow.DeterministicQueue",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
} // namespace

bool FSharFrontendCatalogValidationTest::RunTest(const FString& Parameters)
{
    (void)Parameters;
    USharFrontendCatalogDefinition* Definition =
        MakeFrontendCatalogDefinition();
    TArray<FText> Errors;
    Definition->GatherValidationErrors(Errors);
    TestTrue(TEXT("Canonical frontend catalog validates"), Errors.IsEmpty());

    Definition->Screens.Add(MakeFrontendScreen(
        FName(TEXT("main_menu")),
        ESharFrontendLayer::Primary,
        {FName(TEXT("options"))}
    ));
    Errors.Reset();
    Definition->GatherValidationErrors(Errors);
    TestTrue(TEXT("Duplicate screen identity is rejected"), !Errors.IsEmpty());
    return true;
}

bool FSharFrontendCrossCatalogDuplicateTest::RunTest(
    const FString& Parameters
)
{
    (void)Parameters;
    auto* GameInstance = NewObject<UGameInstance>();
    auto* Catalog =
        NewObject<USharFrontendCatalogSubsystem>(GameInstance);
    TestTrue(
        TEXT("Catalog revision and root configure"),
        Catalog->Configure(
            TEXT("sha256:frontend_catalog_v1"),
            FName(TEXT("base_frontend"))
        )
    );
    TestTrue(
        TEXT("Root catalog registers"),
        Catalog->RegisterCatalog(MakeFrontendCatalogDefinition())
            == ESharFrontendCatalogResult::Accepted
    );

    auto* Extension = NewObject<USharFrontendCatalogDefinition>();
    Extension->CanonicalId = FName(TEXT("extension_frontend"));
    Extension->DisplayName = FText::FromString(TEXT("Extension Frontend"));
    Extension->SourcePackageIds = {FName(TEXT("extension_contract"))};
    Extension->RevisionToken = TEXT("sha256:extension_frontend_v1");
    Extension->ValidationProfile = FName(TEXT("extension_frontend_v1"));
    Extension->OwningFeature = FName(TEXT("extension"));
    Extension->InitialScreenId = FName(TEXT("main_menu"));
    Extension->Screens = {
        MakeFrontendScreen(
            FName(TEXT("main_menu")),
            ESharFrontendLayer::Primary,
            {}
        ),
    };
    TestTrue(
        TEXT("Individually valid extension registers"),
        Catalog->RegisterCatalog(Extension)
            == ESharFrontendCatalogResult::Accepted
    );
    TestTrue(
        TEXT("Cross-catalog duplicate screen blocks activation"),
        Catalog->Activate() == ESharFrontendCatalogResult::DuplicateScreen
    );
    return true;
}

bool FSharFrontendDeterministicQueueTest::RunTest(
    const FString& Parameters
)
{
    (void)Parameters;
    const FSharFrontendRuntimeFixture Runtime = MakeFrontendRuntime();
    const FSharFrontendNavigationRequest UserRequest = MakeFrontendRequest(
        Runtime,
        FName(TEXT("zeta_user_navigation")),
        FName(TEXT("options")),
        ESharFrontendNavigationPriority::User,
        ESharFrontendHistoryPolicy::Push
    );
    const FSharFrontendNavigationRequest RecoveryBeta = MakeFrontendRequest(
        Runtime,
        FName(TEXT("beta_recovery_navigation")),
        FName(TEXT("gallery")),
        ESharFrontendNavigationPriority::Recovery,
        ESharFrontendHistoryPolicy::Push
    );
    const FSharFrontendNavigationRequest RecoveryAlpha = MakeFrontendRequest(
        Runtime,
        FName(TEXT("alpha_recovery_navigation")),
        FName(TEXT("confirm_quit")),
        ESharFrontendNavigationPriority::Recovery,
        ESharFrontendHistoryPolicy::Preserve
    );
    TestTrue(
        TEXT("User navigation queues"),
        Runtime.FlowSubsystem->Submit(UserRequest)
            == ESharFrontendOperationResult::Accepted
    );
    TestTrue(
        TEXT("Recovery beta navigation queues"),
        Runtime.FlowSubsystem->Submit(RecoveryBeta)
            == ESharFrontendOperationResult::Accepted
    );
    TestTrue(
        TEXT("Recovery alpha navigation queues"),
        Runtime.FlowSubsystem->Submit(RecoveryAlpha)
            == ESharFrontendOperationResult::Accepted
    );
    TestEqual(
        TEXT("Recovery alpha is deterministic head"),
        Runtime.FlowSubsystem->GetQueuePosition(RecoveryAlpha.RequestId),
        1
    );
    TestEqual(
        TEXT("Recovery beta is second"),
        Runtime.FlowSubsystem->GetQueuePosition(RecoveryBeta.RequestId),
        FrontendQueuePositionSecond
    );
    TestEqual(
        TEXT("User request is third"),
        Runtime.FlowSubsystem->GetQueuePosition(UserRequest.RequestId),
        FrontendQueuePositionThird
    );
    TestTrue(
        TEXT("Lower-ranked request cannot begin"),
        Runtime.FlowSubsystem->Begin(UserRequest.RequestId)
            == ESharFrontendOperationResult::NotHead
    );
    TestTrue(
        TEXT("Deterministic head begins"),
        Runtime.FlowSubsystem->Begin(RecoveryAlpha.RequestId)
            == ESharFrontendOperationResult::Accepted
    );
    return true;
}

#endif
