// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT
//
// Boundary-Contract:
// - Owns:
//   - Shar load execution tests composition module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Shar load execution tests composition module.
// - Description:
//   - Implements the declared composition module responsibility for project.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Shar load execution tests composition module.

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
                // jig-ignore-next-line: exact syntax is indivisible
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
