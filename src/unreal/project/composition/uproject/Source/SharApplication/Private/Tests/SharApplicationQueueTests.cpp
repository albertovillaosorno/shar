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
//   - Shar application queue tests composition module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Shar application queue tests composition module.
// - Description:
//   - Implements the declared composition module responsibility for project.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Shar application queue tests composition module.

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
