// File: SharApplicationQueueTests.cpp
// Path: src/uproject/Source/SharApplication/Private/Tests/SharApplicationQueueTests.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: deterministic application-transition priority and stable identity ordering tests only.
// Specification: docs/technical/unreal/application-lifecycle-and-mode-runtime.md

#if WITH_DEV_AUTOMATION_TESTS

#include "SharApplicationTestFixtures.h"

#include "Application/SharApplicationModeCoordinator.h"
#include "Misc/AutomationTest.h"

namespace
{
constexpr int32 FirstQueuePosition = 1;
constexpr int32 SecondQueuePosition = 2;
constexpr int32 ThirdQueuePosition = 3;

IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharApplicationQueueOrderingTest,
    "SHAR.Application.Queue.DeterministicOrdering",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
} // namespace

bool FSharApplicationQueueOrderingTest::RunTest(const FString& Parameters)
{
    (void)Parameters;
    const FSharApplicationRuntimeFixture Runtime = MakeApplicationRuntime();
    const FSharApplicationModeRequest Gameplay = MakeApplicationRequest({
        .RequestId = FName(TEXT("request_gameplay")),
        .Priority = ESharApplicationTransitionPriority::Gameplay,
        .CallerId = FName(TEXT("mission_runtime")),
    });
    const FSharApplicationModeRequest UserB = MakeApplicationRequest({
        .RequestId = FName(TEXT("request_user_b")),
        .Priority = ESharApplicationTransitionPriority::User,
        .CallerId = FName(TEXT("frontend_runtime_b")),
    });
    const FSharApplicationModeRequest UserA = MakeApplicationRequest({
        .RequestId = FName(TEXT("request_user_a")),
        .Priority = ESharApplicationTransitionPriority::User,
        .CallerId = FName(TEXT("frontend_runtime_a")),
    });
    Runtime.Coordinator->Submit(Gameplay);
    Runtime.Coordinator->Submit(UserB);
    Runtime.Coordinator->Submit(UserA);

    TestTrue(
        TEXT("Equal user priority uses stable request identity"),
        Runtime.Coordinator->GetQueuePosition(UserA.RequestId)
            == FirstQueuePosition
    );
    TestTrue(
        TEXT("Second equal-priority request follows lexical identity"),
        Runtime.Coordinator->GetQueuePosition(UserB.RequestId)
            == SecondQueuePosition
    );
    TestTrue(
        TEXT("Gameplay-driven request follows explicit user requests"),
        Runtime.Coordinator->GetQueuePosition(Gameplay.RequestId)
            == ThirdQueuePosition
    );
    TestTrue(
        TEXT("Non-head request cannot begin"),
        Runtime.Coordinator->Begin(Gameplay.RequestId)
            == ESharApplicationOperationResult::NotHead
    );
    return true;
}

#endif
