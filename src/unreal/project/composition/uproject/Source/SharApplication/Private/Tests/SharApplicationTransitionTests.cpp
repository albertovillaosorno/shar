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
    FSharApplicationEntryAuthorityAbsenceTest,
    "SHAR.Application.Configuration.EntryAuthorityAbsence",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
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
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharApplicationTransitionThirdModeRecoveryRevisionTest,
    "SHAR.Application.Transition.ThirdModeRecoveryRevision",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharApplicationGameplayPauseResumeLifecycleTest,
    "SHAR.Application.Transition.GameplayPauseResumeLifecycle",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)

FString ModeRevision(const FName& ModeId)
{
    return FString::Printf(TEXT("sha256:%s_v1"), *ModeId.ToString());
}

FSharApplicationModeRequest MakeLifecycleRequest(
    const FName& RequestId,
    const FName& SourceModeId,
    const FName& TargetModeId
)
{
    FSharApplicationModeRequest Request;
    Request.RequestId = RequestId;
    Request.SourceModeId = SourceModeId;
    Request.TargetModeId = TargetModeId;
    Request.ReasonId = FName(TEXT("lifecycle_step"));
    Request.CallerId = FName(TEXT("lifecycle_test"));
    Request.Priority = ESharApplicationTransitionPriority::Gameplay;
    Request.CatalogRevision = TEXT("sha256:application_catalog_v1");
    Request.SourceModeRevision = ModeRevision(SourceModeId);
    Request.TargetModeRevision = ModeRevision(TargetModeId);
    Request.SessionRevision = TEXT("sha256:gameplay_session_v1");
    Request.ProfileRevision = TEXT("sha256:profile_v1");
    Request.WorldRevision = TEXT("sha256:springfield_world_v1");
    Request.RequestRevision = FString::Printf(
        TEXT("sha256:%s_v1"),
        *RequestId.ToString()
    );
    Request.ReturnModeId = TargetModeId == FName(TEXT("pause"))
        ? SourceModeId
        : FName();
    Request.DeadlineSeconds = DefaultApplicationDeadlineSeconds;
    return Request;
}

void PrepareLifecycleRequest(
    USharApplicationModeCoordinator& Coordinator,
    const FSharApplicationModeRequest& Request,
    const TArray<FName>& RequiredServices
)
{
    Coordinator.Begin(Request.RequestId);
    for (const FName& ServiceId : RequiredServices)
    {
        FSharApplicationServiceEvidence Evidence;
        Evidence.RequestId = Request.RequestId;
        Evidence.ServiceId = ServiceId;
        Evidence.Status = ESharApplicationServiceStatus::Ready;
        Evidence.CatalogRevision = Request.CatalogRevision;
        Evidence.RequestRevision = Request.RequestRevision;
        Evidence.ServiceRevision = TEXT("sha256:service_v1");
        Coordinator.RecordServiceEvidence(Evidence);
    }
    Coordinator.BeginReadinessVerification(Request.RequestId);
    FSharApplicationBarrierEvidence Barrier;
    Barrier.RequestId = Request.RequestId;
    Barrier.BarrierId = FName(TEXT("mode_ready_barrier_v1"));
    Barrier.CatalogRevision = Request.CatalogRevision;
    Barrier.RequestRevision = Request.RequestRevision;
    Barrier.TargetModeRevision = Request.TargetModeRevision;
    Coordinator.AcceptBarrier(Barrier);
}
} // namespace

bool FSharApplicationEntryAuthorityAbsenceTest::RunTest(
    const FString& Parameters
)
{
    (void)Parameters;
    auto* GameInstance = NewObject<UGameInstance>();
    USharApplicationModeCatalogSubsystem* Catalog = MakeApplicationCatalog(
        *GameInstance,
        ESharApplicationCatalogShape::Valid,
        true
    );

    FSharApplicationModeObservation EntryObservation;
    EntryObservation.ActiveModeId = FName(TEXT("entry"));
    EntryObservation.ActiveModeRevision = TEXT("sha256:entry_v1");
    auto* EntryCoordinator = NewObject<USharApplicationModeCoordinator>(
        GameInstance
    );
    TestTrue(
        TEXT("Entry accepts absent world profile and session authority"),
        EntryCoordinator->Configure(Catalog, EntryObservation)
    );
    const FSharApplicationModeObservation Accepted =
        EntryCoordinator->GetObservation();
    TestTrue(TEXT("Entry has no world identity"), Accepted.WorldId.IsNone());
    TestTrue(
        TEXT("Entry has no fabricated world revision"),
        Accepted.WorldRevision.IsEmpty()
    );
    TestTrue(
        TEXT("Entry has no fabricated profile revision"),
        Accepted.ProfileRevision.IsEmpty()
    );
    TestTrue(
        TEXT("Entry has no fabricated session revision"),
        Accepted.SessionRevision.IsEmpty()
    );

    FSharApplicationModeObservation StaleEntry = EntryObservation;
    StaleEntry.ActiveModeRevision = TEXT("sha256:stale_entry_v1");
    auto* StaleCoordinator = NewObject<USharApplicationModeCoordinator>(
        GameInstance
    );
    TestFalse(
        TEXT("Entry rejects a revision not owned by its definition"),
        StaleCoordinator->Configure(Catalog, StaleEntry)
    );

    FSharApplicationModeObservation SyntheticEntry = EntryObservation;
    SyntheticEntry.WorldId = FName(TEXT("no_world"));
    SyntheticEntry.WorldRevision = TEXT("sha256:no_world_v1");
    SyntheticEntry.ProfileRevision = TEXT("sha256:profile_none_v1");
    SyntheticEntry.SessionRevision = TEXT("sha256:session_none_v1");
    auto* SyntheticCoordinator = NewObject<USharApplicationModeCoordinator>(
        GameInstance
    );
    TestFalse(
        TEXT("Entry rejects fabricated absent-authority revisions"),
        SyntheticCoordinator->Configure(Catalog, SyntheticEntry)
    );

    FSharApplicationModeObservation FrontEndObservation;
    FrontEndObservation.ActiveModeId = FName(TEXT("front_end"));
    FrontEndObservation.ActiveModeRevision = TEXT("sha256:front_end_v1");
    auto* FrontEndCoordinator = NewObject<USharApplicationModeCoordinator>(
        GameInstance
    );
    TestFalse(
        TEXT("Non-entry mode still requires concrete revisions"),
        FrontEndCoordinator->Configure(Catalog, FrontEndObservation)
    );
    return true;
}

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

bool FSharApplicationTransitionThirdModeRecoveryRevisionTest::RunTest(
    const FString& Parameters
)
{
    (void)Parameters;
    FSharApplicationRuntimeFixture Runtime;
    Runtime.GameInstance = NewObject<UGameInstance>();
    Runtime.Catalog = MakeApplicationCatalog(
        *Runtime.GameInstance,
        ESharApplicationCatalogShape::RecoveryToExit,
        true
    );
    Runtime.Coordinator = NewObject<USharApplicationModeCoordinator>(
        Runtime.GameInstance
    );
    TestTrue(
        TEXT("Coordinator accepts third-mode recovery fixture"),
        Runtime.Coordinator->Configure(
            Runtime.Catalog,
            MakeInitialApplicationObservation()
        )
    );

    const FSharApplicationModeRequest Request = MakeApplicationRequest({
        .RequestId = FName(TEXT("third_mode_recovery_transition")),
        .Priority = ESharApplicationTransitionPriority::Recovery,
        .CallerId = FName(TEXT("loading_runtime")),
    });
    Runtime.Coordinator->Submit(Request);
    PrepareApplicationTransition(*Runtime.Coordinator, Request);
    Runtime.Coordinator->Commit(Request.RequestId);

    TestTrue(
        TEXT("Committed failure enters declared third recovery mode"),
        Runtime.Coordinator->Resolve(MakeApplicationResolution(
            Request.RequestId,
            ESharApplicationTransitionCommand::Fail
        )) == ESharApplicationOperationResult::Accepted
    );
    const FSharApplicationModeObservation Recovered =
        Runtime.Coordinator->GetObservation();
    TestTrue(
        TEXT("Third recovery selects exit mode"),
        Recovered.ActiveModeId == FName(TEXT("exit"))
    );
    TestTrue(
        TEXT("Third recovery publishes recovery mode revision"),
        Recovered.ActiveModeRevision == TEXT("sha256:exit_v1")
    );
    TestTrue(
        TEXT("Third recovery does not reuse failed target revision"),
        Recovered.ActiveModeRevision != Request.TargetModeRevision
    );
    return true;
}

bool FSharApplicationGameplayPauseResumeLifecycleTest::RunTest(
    const FString& Parameters
)
{
    (void)Parameters;
    const FSharApplicationRuntimeFixture Runtime = MakeApplicationRuntime();

    struct FStep
    {
        FName RequestId;
        FName SourceModeId;
        FName TargetModeId;
        TArray<FName> RequiredServices;
    };
    const TArray<FStep> Steps = {
        {
            FName(TEXT("enter_loading_gameplay")),
            FName(TEXT("front_end")),
            FName(TEXT("loading_gameplay")),
            {
                FName(TEXT("catalog_service")),
                FName(TEXT("world_service")),
            },
        },
        {
            FName(TEXT("commit_gameplay")),
            FName(TEXT("loading_gameplay")),
            FName(TEXT("gameplay")),
            {},
        },
        {
            FName(TEXT("enter_pause")),
            FName(TEXT("gameplay")),
            FName(TEXT("pause")),
            {},
        },
        {
            FName(TEXT("resume_gameplay")),
            FName(TEXT("pause")),
            FName(TEXT("gameplay")),
            {},
        },
    };

    for (const FStep& Step : Steps)
    {
        const FSharApplicationModeRequest Request = MakeLifecycleRequest(
            Step.RequestId,
            Step.SourceModeId,
            Step.TargetModeId
        );
        TestTrue(
            TEXT("Lifecycle request is accepted"),
            Runtime.Coordinator->Submit(Request)
                == ESharApplicationOperationResult::Accepted
        );
        PrepareLifecycleRequest(
            *Runtime.Coordinator,
            Request,
            Step.RequiredServices
        );
        TestTrue(
            TEXT("Source remains active until commit"),
            Runtime.Coordinator->GetObservation().ActiveModeId
                == Step.SourceModeId
        );
        TestTrue(
            TEXT("Prepared lifecycle step commits"),
            Runtime.Coordinator->Commit(Request.RequestId)
                == ESharApplicationOperationResult::Accepted
        );
        TestTrue(
            TEXT("Commit publishes target mode"),
            Runtime.Coordinator->GetObservation().ActiveModeId
                == Step.TargetModeId
        );
        TestTrue(
            TEXT("Committed lifecycle step completes"),
            Runtime.Coordinator->Complete(Request.RequestId)
                == ESharApplicationOperationResult::Accepted
        );
        TestTrue(
            TEXT("Completed lifecycle step releases"),
            Runtime.Coordinator->Release(Request.RequestId)
                == ESharApplicationOperationResult::Accepted
        );
    }

    TestTrue(
        TEXT("Pause resume returns to gameplay"),
        Runtime.Coordinator->GetObservation().ActiveModeId
            == FName(TEXT("gameplay"))
    );
    return true;
}

#endif
