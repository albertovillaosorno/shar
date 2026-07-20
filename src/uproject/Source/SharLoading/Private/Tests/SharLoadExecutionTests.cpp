// File: SharLoadExecutionTests.cpp
// Path: src/uproject/Source/SharLoading/Private/Tests/SharLoadExecutionTests.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: dependency ordering and callback revision-fence tests only.
// Specification: docs/technical/unreal/native-asset-load-request-and-streaming-runtime.md

#if WITH_DEV_AUTOMATION_TESTS

#include "SharLoadingTestFixtures.h"

#include "Loading/SharLoadCoordinatorSubsystem.h"
#include "Misc/AutomationTest.h"

namespace
{
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharLoadExecutionDependencyTest,
    "SHAR.Loading.Execution.DependencyAndRevisionFences",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
} // namespace

bool FSharLoadExecutionDependencyTest::RunTest(const FString& Parameters)
{
    (void)Parameters;
    USharLoadCoordinatorSubsystem* Coordinator =
        MakeCoordinator(MakeRequiredPlan());
    const FSharLoadRequest Request = MakeRequest({
        .RequestId = FName(TEXT("dependency_request")),
        .PlanId = FName(TEXT("springfield_world_plan")),
        .ScopeId = FName(TEXT("springfield_scope")),
        .CallerId = FName(TEXT("world_runtime")),
        .Priority = 20,
        .CancellationPolicy = ESharLoadCancellationPolicy::RejectDuplicate,
        .ResultPolicy = ESharLoadResultPolicy::Required,
    });
    Coordinator->Submit(Request);
    Coordinator->BeginRequest(Request.RequestId);

    TestTrue(
        TEXT("Dependent node cannot begin before root completion"),
        Coordinator->BeginNode({
                .RequestId = Request.RequestId,
                .NodeId = FName(TEXT("world_ready")),
                .AttemptId = FName(TEXT("attempt_world_01")),
            }) == ESharLoadOperationResult::DependencyBlocked
    );
    TestTrue(
        TEXT("Root node begins"),
        Coordinator->BeginNode({
                .RequestId = Request.RequestId,
                .NodeId = FName(TEXT("package_ready")),
                .AttemptId = FName(TEXT("attempt_package_01")),
            }) == ESharLoadOperationResult::Accepted
    );
    FSharLoadCallbackRevision Stale = MakeCallbackRevision(
        FName(TEXT("attempt_package_01"))
    );
    Stale.RequestRevision = TEXT("sha256:request_old");
    TestTrue(
        TEXT("Stale node completion is rejected"),
        Coordinator->CompleteNode({
                .RequestId = Request.RequestId,
                .NodeId = FName(TEXT("package_ready")),
                .Revision = Stale,
            }) == ESharLoadOperationResult::StaleRevision
    );
    TestTrue(
        TEXT("Correlated root completion is accepted"),
        Coordinator->CompleteNode({
                .RequestId = Request.RequestId,
                .NodeId = FName(TEXT("package_ready")),
                .Revision = MakeCallbackRevision(FName(TEXT("attempt_package_01"))),
            }) == ESharLoadOperationResult::Accepted
    );
    TestTrue(
        TEXT("Dependent node begins after root completion"),
        Coordinator->BeginNode({
                .RequestId = Request.RequestId,
                .NodeId = FName(TEXT("world_ready")),
                .AttemptId = FName(TEXT("attempt_world_01")),
            }) == ESharLoadOperationResult::Accepted
    );
    return true;
}

#endif
