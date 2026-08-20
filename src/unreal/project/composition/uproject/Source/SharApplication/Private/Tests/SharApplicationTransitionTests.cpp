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
//   - Shar application transition tests composition module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Shar application transition tests composition module.
// - Description:
//   - Implements the declared composition module responsibility for project.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Shar application transition tests composition module.

#if WITH_DEV_AUTOMATION_TESTS

#include "SharApplicationTestFixtures.h"

#include "Application/SharApplicationModeCoordinator.h"
#include "Misc/AutomationTest.h"

namespace
{
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharApplicationTransitionSuccessTest,
    "SHAR.Application.Transition.SuccessLifecycle",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharApplicationTransitionStaleEvidenceTest,
    "SHAR.Application.Transition.StaleEvidence",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharApplicationTransitionPreCommitFailureTest,
    "SHAR.Application.Transition.PreCommitFailure",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharApplicationTransitionPostCommitRecoveryTest,
    "SHAR.Application.Transition.PostCommitRecovery",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
} // namespace

bool FSharApplicationTransitionSuccessTest::RunTest(
    const FString& Parameters
)
{
    (void)Parameters;
    const FSharApplicationRuntimeFixture Runtime = MakeApplicationRuntime();
    const FSharApplicationModeRequest Request = MakeApplicationRequest({
        .RequestId = FName(TEXT("success_transition")),
        .Priority = ESharApplicationTransitionPriority::User,
        .CallerId = FName(TEXT("frontend_runtime")),
    });
    Runtime.Coordinator->Submit(Request);
    PrepareApplicationTransition(*Runtime.Coordinator, Request);

    TestTrue(
        TEXT("Prepared transition commits atomically"),
        Runtime.Coordinator->Commit(Request.RequestId)
            == ESharApplicationOperationResult::Accepted
    );
    const FSharApplicationModeObservation CommittedObservation =
        Runtime.Coordinator->GetObservation();
    TestTrue(
        TEXT("Committed observation exposes target mode"),
        CommittedObservation.ActiveModeId == Request.TargetModeId
    );
    TestTrue(
        TEXT("Committed observation exposes target revision"),
        CommittedObservation.ActiveModeRevision
            == Request.TargetModeRevision
    );
    TestTrue(
        TEXT("Postcondition verification publishes success"),
        Runtime.Coordinator->Complete(Request.RequestId)
            == ESharApplicationOperationResult::Accepted
    );
    TestTrue(
        TEXT("Successful transition has one terminal result"),
        Runtime.Coordinator->GetTerminalResult(Request.RequestId)
            == ESharApplicationTerminalResult::Success
    );
    TestTrue(
        TEXT("Duplicate completion is rejected"),
        Runtime.Coordinator->Complete(Request.RequestId)
            == ESharApplicationOperationResult::AlreadyTerminal
    );
    TestTrue(
        TEXT("Terminal transition releases explicitly"),
        Runtime.Coordinator->Release(Request.RequestId)
            == ESharApplicationOperationResult::Accepted
    );
    return true;
}

bool FSharApplicationTransitionStaleEvidenceTest::RunTest(
    const FString& Parameters
)
{
    (void)Parameters;
    const FSharApplicationRuntimeFixture Runtime = MakeApplicationRuntime();
    const FSharApplicationModeRequest Request = MakeApplicationRequest({
        .RequestId = FName(TEXT("stale_evidence_transition")),
        .Priority = ESharApplicationTransitionPriority::User,
        .CallerId = FName(TEXT("frontend_runtime")),
    });
    Runtime.Coordinator->Submit(Request);
    Runtime.Coordinator->Begin(Request.RequestId);
    FSharApplicationServiceEvidence Evidence =
        MakeApplicationServiceEvidence({
            .RequestId = Request.RequestId,
            .ServiceId = FName(TEXT("catalog_service")),
            .Status = ESharApplicationServiceStatus::Ready,
        });
    Evidence.RequestRevision = TEXT("sha256:transition_old");

    TestTrue(
        TEXT("Stale service evidence is rejected"),
        Runtime.Coordinator->RecordServiceEvidence(Evidence)
            == ESharApplicationOperationResult::StaleRevision
    );
    TestTrue(
        TEXT("Missing correlated evidence blocks readiness"),
        Runtime.Coordinator->BeginReadinessVerification(Request.RequestId)
            == ESharApplicationOperationResult::DependencyBlocked
    );
    TestTrue(
        TEXT("Source mode remains authoritative"),
        Runtime.Coordinator->GetObservation().ActiveModeId
            == Request.SourceModeId
    );
    return true;
}

bool FSharApplicationTransitionPreCommitFailureTest::RunTest(
    const FString& Parameters
)
{
    (void)Parameters;
    const FSharApplicationRuntimeFixture Runtime = MakeApplicationRuntime();
    const FSharApplicationModeRequest Request = MakeApplicationRequest({
        .RequestId = FName(TEXT("precommit_failure_transition")),
        .Priority = ESharApplicationTransitionPriority::User,
        .CallerId = FName(TEXT("frontend_runtime")),
    });
    Runtime.Coordinator->Submit(Request);
    Runtime.Coordinator->Begin(Request.RequestId);

    TestTrue(
        TEXT("Pre-commit failure publishes failure"),
        Runtime.Coordinator->Resolve(MakeApplicationResolution(
            Request.RequestId,
            ESharApplicationTransitionCommand::Fail
        )) == ESharApplicationOperationResult::Accepted
    );
    TestTrue(
        TEXT("Pre-commit failure preserves source mode"),
        Runtime.Coordinator->GetObservation().ActiveModeId
            == Request.SourceModeId
    );
    TestTrue(
        TEXT("Pre-commit failure preserves source revision"),
        Runtime.Coordinator->GetObservation().ActiveModeRevision
            == Request.SourceModeRevision
    );
    TestTrue(
        TEXT("Pre-commit failure has failed terminal result"),
        Runtime.Coordinator->GetTerminalResult(Request.RequestId)
            == ESharApplicationTerminalResult::Failed
    );
    return true;
}

bool FSharApplicationTransitionPostCommitRecoveryTest::RunTest(
    const FString& Parameters
)
{
    (void)Parameters;
    const FSharApplicationRuntimeFixture Runtime = MakeApplicationRuntime();
    const FSharApplicationModeRequest Request = MakeApplicationRequest({
        .RequestId = FName(TEXT("postcommit_recovery_transition")),
        .Priority = ESharApplicationTransitionPriority::Recovery,
        .CallerId = FName(TEXT("loading_runtime")),
    });
    Runtime.Coordinator->Submit(Request);
    PrepareApplicationTransition(*Runtime.Coordinator, Request);
    Runtime.Coordinator->Commit(Request.RequestId);

    TestTrue(
        TEXT("Post-commit failure enters declared recovery mode"),
        Runtime.Coordinator->Resolve(MakeApplicationResolution(
            Request.RequestId,
            ESharApplicationTransitionCommand::Fail
        )) == ESharApplicationOperationResult::Accepted
    );
    TestTrue(
        TEXT("Recovery restores front-end mode"),
        Runtime.Coordinator->GetObservation().ActiveModeId
            == FName(TEXT("front_end"))
    );
    TestTrue(
        TEXT("Recovery restores source revision"),
        Runtime.Coordinator->GetObservation().ActiveModeRevision
            == Request.SourceModeRevision
    );
    TestTrue(
        TEXT("Post-commit failure publishes recovered terminal result"),
        Runtime.Coordinator->GetTerminalResult(Request.RequestId)
            == ESharApplicationTerminalResult::Recovered
    );
    TestTrue(
        TEXT("Duplicate post-commit resolution is rejected"),
        Runtime.Coordinator->Resolve(MakeApplicationResolution(
            Request.RequestId,
            ESharApplicationTransitionCommand::Fail
        )) == ESharApplicationOperationResult::AlreadyTerminal
    );
    return true;
}

#endif
