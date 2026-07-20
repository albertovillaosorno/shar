// File: SharLoadQueueTests.cpp
// Path: src/uproject/Source/SharLoading/Private/Tests/SharLoadQueueTests.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: deterministic priority and stable identity ordering tests only.
// Specification: docs/technical/unreal/native-asset-load-request-and-streaming-runtime.md

#if WITH_DEV_AUTOMATION_TESTS

#include "SharLoadingTestFixtures.h"

#include "Loading/SharLoadCoordinatorSubsystem.h"
#include "Misc/AutomationTest.h"

namespace
{
constexpr int32 LowPriority = 10;
constexpr int32 HighPriority = 50;
constexpr int32 FirstQueuePosition = 1;
constexpr int32 SecondQueuePosition = 2;
constexpr int32 ThirdQueuePosition = 3;

IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharLoadQueueOrderingTest,
    "SHAR.Loading.Queue.DeterministicOrdering",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
} // namespace

bool FSharLoadQueueOrderingTest::RunTest(const FString& Parameters)
{
    (void)Parameters;
    USharLoadCoordinatorSubsystem* Coordinator =
        MakeCoordinator(MakeRequiredPlan());
    const FSharLoadRequest Low = MakeRequest({
        .RequestId = FName(TEXT("request_low")),
        .PlanId = FName(TEXT("springfield_world_plan")),
        .ScopeId = FName(TEXT("springfield_scope")),
        .CallerId = FName(TEXT("caller_low")),
        .Priority = LowPriority,
        .CancellationPolicy = ESharLoadCancellationPolicy::RejectDuplicate,
        .ResultPolicy = ESharLoadResultPolicy::Required,
    });
    const FSharLoadRequest HighA = MakeRequest({
        .RequestId = FName(TEXT("request_high_a")),
        .PlanId = FName(TEXT("springfield_world_plan")),
        .ScopeId = FName(TEXT("springfield_scope")),
        .CallerId = FName(TEXT("caller_high_a")),
        .Priority = HighPriority,
        .CancellationPolicy = ESharLoadCancellationPolicy::RejectDuplicate,
        .ResultPolicy = ESharLoadResultPolicy::Required,
    });
    const FSharLoadRequest HighB = MakeRequest({
        .RequestId = FName(TEXT("request_high_b")),
        .PlanId = FName(TEXT("springfield_world_plan")),
        .ScopeId = FName(TEXT("springfield_scope")),
        .CallerId = FName(TEXT("caller_high_b")),
        .Priority = HighPriority,
        .CancellationPolicy = ESharLoadCancellationPolicy::RejectDuplicate,
        .ResultPolicy = ESharLoadResultPolicy::Required,
    });
    Coordinator->Submit(Low);
    Coordinator->Submit(HighB);
    Coordinator->Submit(HighA);

    TestTrue(
        TEXT("Equal high priority uses stable request identity"),
        Coordinator->GetQueuePosition(HighA.RequestId) == FirstQueuePosition
    );
    TestTrue(
        TEXT("Second high priority follows lexical identity"),
        Coordinator->GetQueuePosition(HighB.RequestId) == SecondQueuePosition
    );
    TestTrue(
        TEXT("Lower priority remains last"),
        Coordinator->GetQueuePosition(Low.RequestId) == ThirdQueuePosition
    );
    TestTrue(
        TEXT("Non-head request cannot begin"),
        Coordinator->BeginRequest(Low.RequestId)
            == ESharLoadOperationResult::NotHead
    );
    return true;
}

#endif
