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
//   - Shar load queue tests composition module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Shar load queue tests composition module.
// - Description:
//   - Implements the declared composition module responsibility for project.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Shar load queue tests composition module.

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
