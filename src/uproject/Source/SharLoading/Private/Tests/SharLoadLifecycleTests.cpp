// File: SharLoadLifecycleTests.cpp
// Path: src/uproject/Source/SharLoading/Private/Tests/SharLoadLifecycleTests.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: progress, readiness commit, shared cancellation, timeout, terminal uniqueness, and release tests only.
// Specification: docs/technical/unreal/native-asset-load-request-and-streaming-runtime.md
// LARGE-FILE owner=SharLoading; reason=three cohesive request-lifecycle scenarios;
// split=separate progress tests if byte projections are introduced;
// validation=validate.sh SharLoading plus Unreal automation; review=2027-01.

#if WITH_DEV_AUTOMATION_TESTS

#include "SharLoadingTestFixtures.h"

#include "Loading/SharLoadCoordinatorSubsystem.h"
#include "Misc/AutomationTest.h"

namespace
{
constexpr int32 ExpectedRequiredNodeCount = 2;
constexpr int32 ExpectedSharedConsumerCount = 2;
constexpr int32 ExpectedSingleConsumerCount = 1;

IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharLoadSuccessAndProgressTest,
    "SHAR.Loading.Lifecycle.SuccessAndProgress",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharLoadSharedCancellationTest,
    "SHAR.Loading.Lifecycle.SharedCancellation",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharLoadTerminalGuardsTest,
    "SHAR.Loading.Lifecycle.TerminalGuards",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
} // namespace

static void CompleteRequiredNodes(
    USharLoadCoordinatorSubsystem& Coordinator,
    const FSharLoadRequest& Request
)
{
    Coordinator.BeginRequest(Request.RequestId);
    Coordinator.BeginNode({
        .RequestId = Request.RequestId,
        .NodeId = FName(TEXT("package_ready")),
        .AttemptId = FName(TEXT("attempt_package_01")),
    });
    Coordinator.CompleteNode({
        .RequestId = Request.RequestId,
        .NodeId = FName(TEXT("package_ready")),
        .Revision = MakeCallbackRevision(FName(TEXT("attempt_package_01"))),
    });
    Coordinator.BeginNode({
        .RequestId = Request.RequestId,
        .NodeId = FName(TEXT("world_ready")),
        .AttemptId = FName(TEXT("attempt_world_01")),
    });
    Coordinator.CompleteNode({
        .RequestId = Request.RequestId,
        .NodeId = FName(TEXT("world_ready")),
        .Revision = MakeCallbackRevision(FName(TEXT("attempt_world_01"))),
    });
}

bool FSharLoadSuccessAndProgressTest::RunTest(const FString& Parameters)
{
    (void)Parameters;
    USharLoadCoordinatorSubsystem* Coordinator =
        MakeCoordinator(MakeRequiredPlan());
    const FSharLoadRequest Request = MakeRequest({
        .RequestId = FName(TEXT("success_request")),
        .PlanId = FName(TEXT("springfield_world_plan")),
        .ScopeId = FName(TEXT("springfield_scope")),
        .CallerId = FName(TEXT("application_mode_runtime")),
        .Priority = 30,
        .CancellationPolicy = ESharLoadCancellationPolicy::RejectDuplicate,
        .ResultPolicy = ESharLoadResultPolicy::Required,
    });
    Coordinator->Submit(Request);
    const FSharLoadProgress InitialProgress =
        Coordinator->GetProgress(Request.RequestId);
    CompleteRequiredNodes(*Coordinator, Request);
    const FSharLoadProgress CompletedProgress =
        Coordinator->GetProgress(Request.RequestId);

    TestTrue(
        TEXT("All required nodes complete"),
        CompletedProgress.CompletedNodeCount == ExpectedRequiredNodeCount
    );
    TestTrue(
        TEXT("Progress revision advances monotonically"),
        CompletedProgress.Revision > InitialProgress.Revision
    );
    TestTrue(
        TEXT("Verification begins after required nodes complete"),
        Coordinator->BeginVerification(Request.RequestId)
            == ESharLoadOperationResult::Accepted
    );
    FSharLoadCallbackRevision StaleBarrier = MakeCallbackRevision(
        FName(TEXT("barrier_attempt_01"))
    );
    StaleBarrier.ScopeRevision = TEXT("sha256:scope_old");
    TestTrue(
        TEXT("Stale readiness barrier is rejected"),
        Coordinator->AcceptBarrier({
                .RequestId = Request.RequestId,
                .BarrierId = Request.ReadinessBarrierId,
                .Revision = StaleBarrier,
            }) == ESharLoadOperationResult::StaleRevision
    );
    TestTrue(
        TEXT("Correlated readiness barrier is accepted"),
        Coordinator->AcceptBarrier({
                .RequestId = Request.RequestId,
                .BarrierId = Request.ReadinessBarrierId,
                .Revision = MakeCallbackRevision(FName(TEXT("barrier_attempt_01"))),
            }) == ESharLoadOperationResult::Accepted
    );
    TestTrue(
        TEXT("Verified request publishes success"),
        Coordinator->CommitSuccess(Request.RequestId)
            == ESharLoadOperationResult::Accepted
    );
    TestTrue(
        TEXT("Terminal result is success"),
        Coordinator->GetTerminalResult(Request.RequestId)
            == ESharLoadTerminalResult::Success
    );
    TestTrue(
        TEXT("Duplicate terminal publication is rejected"),
        Coordinator->CommitSuccess(Request.RequestId)
            == ESharLoadOperationResult::AlreadyTerminal
    );
    TestTrue(
        TEXT("Successful retained work releases explicitly"),
        Coordinator->Release(Request.RequestId)
            == ESharLoadOperationResult::Accepted
    );
    return true;
}

bool FSharLoadSharedCancellationTest::RunTest(const FString& Parameters)
{
    (void)Parameters;
    USharLoadCoordinatorSubsystem* Coordinator =
        MakeCoordinator(MakeRequiredPlan());
    const FSharLoadRequest First = MakeRequest({
        .RequestId = FName(TEXT("shared_request_a")),
        .PlanId = FName(TEXT("springfield_world_plan")),
        .ScopeId = FName(TEXT("springfield_scope")),
        .CallerId = FName(TEXT("presentation_runtime")),
        .Priority = 20,
        .CancellationPolicy = ESharLoadCancellationPolicy::RetainSharedWork,
        .ResultPolicy = ESharLoadResultPolicy::Required,
    });
    const FSharLoadRequest Second = MakeRequest({
        .RequestId = FName(TEXT("shared_request_b")),
        .PlanId = FName(TEXT("springfield_world_plan")),
        .ScopeId = FName(TEXT("springfield_scope")),
        .CallerId = FName(TEXT("presentation_runtime")),
        .Priority = 20,
        .CancellationPolicy = ESharLoadCancellationPolicy::RetainSharedWork,
        .ResultPolicy = ESharLoadResultPolicy::Required,
    });
    Coordinator->Submit(First);
    Coordinator->Submit(Second);
    Coordinator->BeginRequest(First.RequestId);
    Coordinator->BeginRequest(Second.RequestId);

    TestTrue(
        TEXT("Two requests share package dependency ownership"),
        Coordinator->GetSharedConsumerCount(
            FName(TEXT("springfield_package")),
            First.ScopeId
        ) == ExpectedSharedConsumerCount
    );
    TestTrue(
        TEXT("Cancelling one consumer retains shared work"),
        Coordinator->ResolveTerminal({
            .RequestId = First.RequestId,
            .Command = ESharLoadTerminalCommand::Cancel,
        })
            == ESharLoadOperationResult::SharedWorkRetained
    );
    TestTrue(
        TEXT("One shared consumer remains"),
        Coordinator->GetSharedConsumerCount(
            FName(TEXT("springfield_package")),
            First.ScopeId
        ) == ExpectedSingleConsumerCount
    );
    TestTrue(
        TEXT("Final consumer cancellation releases shared work"),
        Coordinator->ResolveTerminal({
            .RequestId = Second.RequestId,
            .Command = ESharLoadTerminalCommand::Cancel,
        })
            == ESharLoadOperationResult::Accepted
    );
    TestTrue(
        TEXT("No shared consumers remain"),
        Coordinator->GetSharedConsumerCount(
            FName(TEXT("springfield_package")),
            First.ScopeId
        ) == 0
    );
    return true;
}

bool FSharLoadTerminalGuardsTest::RunTest(const FString& Parameters)
{
    (void)Parameters;
    USharLoadCoordinatorSubsystem* Coordinator =
        MakeCoordinator(MakeRequiredPlan());
    const FSharLoadRequest Request = MakeRequest({
        .RequestId = FName(TEXT("timeout_request")),
        .PlanId = FName(TEXT("springfield_world_plan")),
        .ScopeId = FName(TEXT("springfield_scope")),
        .CallerId = FName(TEXT("world_runtime")),
        .Priority = 20,
        .CancellationPolicy = ESharLoadCancellationPolicy::RejectDuplicate,
        .ResultPolicy = ESharLoadResultPolicy::Required,
    });
    Coordinator->Submit(Request);
    Coordinator->BeginRequest(Request.RequestId);
    Coordinator->BeginNode({
            .RequestId = Request.RequestId,
            .NodeId = FName(TEXT("package_ready")),
            .AttemptId = FName(TEXT("attempt_package_timeout")),
        });

    TestTrue(
        TEXT("Timeout publishes one terminal result"),
        Coordinator->ResolveTerminal({
            .RequestId = Request.RequestId,
            .Command = ESharLoadTerminalCommand::Timeout,
        })
            == ESharLoadOperationResult::Accepted
    );
    TestTrue(
        TEXT("Late completion cannot revive timed-out request"),
        Coordinator->CompleteNode({
                .RequestId = Request.RequestId,
                .NodeId = FName(TEXT("package_ready")),
                .Revision = MakeCallbackRevision(FName(TEXT("attempt_package_timeout"))),
            }) == ESharLoadOperationResult::AlreadyTerminal
    );
    TestTrue(
        TEXT("Duplicate timeout is rejected"),
        Coordinator->ResolveTerminal({
            .RequestId = Request.RequestId,
            .Command = ESharLoadTerminalCommand::Timeout,
        })
            == ESharLoadOperationResult::AlreadyTerminal
    );
    TestTrue(
        TEXT("Timed-out request releases"),
        Coordinator->Release(Request.RequestId)
            == ESharLoadOperationResult::Accepted
    );
    return true;
}

#endif
