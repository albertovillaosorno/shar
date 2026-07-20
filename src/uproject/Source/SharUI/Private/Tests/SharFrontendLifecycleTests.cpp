// File: SharFrontendLifecycleTests.cpp
// Path: src/uproject/Source/SharUI/Private/Tests/SharFrontendLifecycleTests.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: successful frontend navigation, history projection, focus acceptance, and back-navigation tests only.
// Specification: docs/technical/unreal/frontend-screen-flow-and-settings-runtime.md

#if WITH_DEV_AUTOMATION_TESTS

#include "SharFrontendTestFixtures.h"

#include "Misc/AutomationTest.h"
#include "UI/SharFrontendCatalogDefinition.h"
#include "UI/SharFrontendFlowContracts.h"
#include "UI/SharFrontendFlowSubsystem.h"

namespace
{
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharFrontendSuccessfulLifecycleTest,
    "SHAR.Frontend.Flow.SuccessLifecycle",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)

IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharFrontendHistoryPopTest,
    "SHAR.Frontend.Flow.HistoryPop",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
} // namespace

bool FSharFrontendSuccessfulLifecycleTest::RunTest(
    const FString& Parameters
)
{
    (void)Parameters;
    const FSharFrontendRuntimeFixture Runtime = MakeFrontendRuntime();
    const FSharFrontendNavigationRequest Request = MakeFrontendRequest(
        Runtime,
        FName(TEXT("open_options")),
        FName(TEXT("options")),
        ESharFrontendNavigationPriority::User,
        ESharFrontendHistoryPolicy::Push
    );
    TestTrue(
        TEXT("Options request queues"),
        Runtime.FlowSubsystem->Submit(Request)
            == ESharFrontendOperationResult::Accepted
    );
    TestTrue(
        TEXT("Options request begins"),
        Runtime.FlowSubsystem->Begin(Request.RequestId)
            == ESharFrontendOperationResult::Accepted
    );
    TestTrue(
        TEXT("All pre-commit evidence is accepted"),
        AcceptFrontendPreCommit(Runtime, Request.RequestId)
    );
    TestTrue(
        TEXT("Candidate screen commits"),
        Runtime.FlowSubsystem->Commit(Request.RequestId)
            == ESharFrontendOperationResult::Accepted
    );
    TestTrue(
        TEXT("All post-commit evidence is accepted"),
        AcceptFrontendPostCommit(Runtime, Request.RequestId)
    );
    TestTrue(
        TEXT("Navigation publishes successful terminal result"),
        Runtime.FlowSubsystem->GetTerminalResult(Request.RequestId)
            == ESharFrontendTerminalResult::Success
    );

    const FSharFrontendFlowObservation& Observation =
        Runtime.FlowSubsystem->GetObservation();
    TestTrue(
        TEXT("Options becomes active primary screen"),
        Observation.ActivePrimaryScreenId == FName(TEXT("options"))
    );
    TestTrue(
        TEXT("Main menu is retained in history"),
        Observation.PrimaryHistory.Num() == 1
            && Observation.PrimaryHistory.Last()
                == FName(TEXT("main_menu"))
    );
    TestTrue(
        TEXT("Correlated focus evidence becomes stable focus"),
        Observation.StableFocusTargetId
            == FName(TEXT("default_focus_target"))
    );
    TestTrue(
        TEXT("Completed navigation releases explicitly"),
        Runtime.FlowSubsystem->Release(Request.RequestId)
            == ESharFrontendOperationResult::Accepted
    );
    return true;
}

bool FSharFrontendHistoryPopTest::RunTest(const FString& Parameters)
{
    (void)Parameters;
    const FSharFrontendRuntimeFixture Runtime = MakeFrontendRuntime();
    const FSharFrontendNavigationRequest OpenOptions = MakeFrontendRequest(
        Runtime,
        FName(TEXT("open_options_for_back")),
        FName(TEXT("options")),
        ESharFrontendNavigationPriority::User,
        ESharFrontendHistoryPolicy::Push
    );
    Runtime.FlowSubsystem->Submit(OpenOptions);
    Runtime.FlowSubsystem->Begin(OpenOptions.RequestId);
    AcceptFrontendPreCommit(Runtime, OpenOptions.RequestId);
    Runtime.FlowSubsystem->Commit(OpenOptions.RequestId);
    AcceptFrontendPostCommit(Runtime, OpenOptions.RequestId);
    Runtime.FlowSubsystem->Release(OpenOptions.RequestId);

    const FSharFrontendNavigationRequest ReturnToMenu = MakeFrontendRequest(
        Runtime,
        FName(TEXT("return_to_main_menu")),
        FName(TEXT("main_menu")),
        ESharFrontendNavigationPriority::User,
        ESharFrontendHistoryPolicy::Pop
    );
    TestTrue(
        TEXT("Back request queues against accepted revision"),
        Runtime.FlowSubsystem->Submit(ReturnToMenu)
            == ESharFrontendOperationResult::Accepted
    );
    TestTrue(
        TEXT("Back request begins"),
        Runtime.FlowSubsystem->Begin(ReturnToMenu.RequestId)
            == ESharFrontendOperationResult::Accepted
    );
    TestTrue(
        TEXT("Back pre-commit evidence is accepted"),
        AcceptFrontendPreCommit(Runtime, ReturnToMenu.RequestId)
    );
    TestTrue(
        TEXT("Back candidate commits"),
        Runtime.FlowSubsystem->Commit(ReturnToMenu.RequestId)
            == ESharFrontendOperationResult::Accepted
    );
    TestTrue(
        TEXT("Back post-commit evidence is accepted"),
        AcceptFrontendPostCommit(Runtime, ReturnToMenu.RequestId)
    );
    const FSharFrontendFlowObservation& Observation =
        Runtime.FlowSubsystem->GetObservation();
    TestTrue(
        TEXT("Back restores main menu"),
        Observation.ActivePrimaryScreenId == FName(TEXT("main_menu"))
    );
    TestTrue(
        TEXT("Back consumes the history entry"),
        Observation.PrimaryHistory.IsEmpty()
    );
    return true;
}

#endif
