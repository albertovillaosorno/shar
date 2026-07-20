// File: SharPresentationLifecycleTests.cpp
// Path: src/uproject/Source/SharPresentation/Private/Tests/SharPresentationLifecycleTests.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: readiness, start, terminal result, skip, cancellation, and release tests only.
// Specification: docs/technical/unreal/presentation-playback-runtime.md
// LARGE-FILE owner=SharPresentation; reason=three cohesive lifecycle and teardown scenarios;
// split=separate terminal-result tests if adapter evidence expands;
// validation=validate.sh SharPresentation plus Unreal automation; review=2027-01.

#if WITH_DEV_AUTOMATION_TESTS

#include "SharPresentationTestFixtures.h"

#include "Misc/AutomationTest.h"
#include "Presentation/SharPresentationPlaybackSubsystem.h"

static void StartPresentation(
    USharPresentationPlaybackSubsystem& Subsystem,
    const FSharPresentationRequest& Request,
    const FSharPresentationCallbackRevision& Revision
)
{
    Subsystem.Enqueue(Request);
    Subsystem.BeginLoading(Request.RequestId);
    Subsystem.MarkReady(Request.RequestId, Revision);
    Subsystem.BeginStart(Request.RequestId, Revision);
    Subsystem.MarkPlaying(Request.RequestId, Revision);
}

namespace
{
constexpr int32 ExpectedOwnerRequestCount = 2;

IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharPresentationReadinessLifecycleTest,
    "SHAR.Presentation.Lifecycle.ReadinessAndCompletion",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharPresentationSkipResultTest,
    "SHAR.Presentation.Lifecycle.SkipResult",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharPresentationOwnerTeardownTest,
    "SHAR.Presentation.Lifecycle.OwnerTeardown",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
} // namespace

bool FSharPresentationReadinessLifecycleTest::RunTest(
    const FString& Parameters
)
{
    (void)Parameters;
    USharPresentationPlaybackSubsystem* Subsystem =
        MakePresentationSubsystem(ESharPresentationDuplicatePolicy::Reject);
    const FSharPresentationRequest Request = MakePresentationRequest({
        .RequestId = FName(TEXT("intro_request")),
        .PresentationId = FName(TEXT("kwik_e_mart_intro")),
        .OwnerId = FName(TEXT("mission_owner")),
        .ChannelId = FName(TEXT("cinematic")),
    });
    const FSharPresentationCallbackRevision Revision =
        MakePresentationRevision();
    Subsystem->Enqueue(Request);

    TestTrue(
        TEXT("Playback cannot start before readiness"),
        Subsystem->BeginStart(Request.RequestId, Revision)
            == ESharPresentationOperationResult::InvalidState
    );
    TestTrue(
        TEXT("Queue head begins loading"),
        Subsystem->BeginLoading(Request.RequestId)
            == ESharPresentationOperationResult::Accepted
    );
    FSharPresentationCallbackRevision StaleRevision = Revision;
    StaleRevision.RequestRevision = TEXT("sha256:request_old");
    TestTrue(
        TEXT("Late readiness callback is rejected"),
        Subsystem->MarkReady(Request.RequestId, StaleRevision)
            == ESharPresentationOperationResult::StaleRevision
    );
    TestTrue(
        TEXT("Correlated readiness callback is accepted"),
        Subsystem->MarkReady(Request.RequestId, Revision)
            == ESharPresentationOperationResult::Accepted
    );
    TestTrue(
        TEXT("Ready request enters starting state"),
        Subsystem->BeginStart(Request.RequestId, Revision)
            == ESharPresentationOperationResult::Accepted
    );
    TestTrue(
        TEXT("Correlated adapter start enters playing state"),
        Subsystem->MarkPlaying(Request.RequestId, Revision)
            == ESharPresentationOperationResult::Accepted
    );
    TestTrue(
        TEXT("Playing request pauses"),
        Subsystem->Pause(Request.RequestId)
            == ESharPresentationOperationResult::Accepted
    );
    TestTrue(
        TEXT("Paused request resumes"),
        Subsystem->Resume(Request.RequestId)
            == ESharPresentationOperationResult::Accepted
    );
    TestTrue(
        TEXT("Playing request publishes completion once"),
        Subsystem->Complete(Request.RequestId)
            == ESharPresentationOperationResult::Accepted
    );
    TestTrue(
        TEXT("Duplicate terminal publication is rejected"),
        Subsystem->Complete(Request.RequestId)
            == ESharPresentationOperationResult::AlreadyTerminal
    );
    TestTrue(
        TEXT("Terminal request releases separately"),
        Subsystem->Release(Request.RequestId)
            == ESharPresentationOperationResult::Accepted
    );
    TestTrue(
        TEXT("Released request preserves completed result"),
        Subsystem->GetTerminalResult(Request.RequestId)
            == ESharPresentationTerminalResult::Completed
    );
    return true;
}

bool FSharPresentationSkipResultTest::RunTest(const FString& Parameters)
{
    (void)Parameters;
    USharPresentationPlaybackSubsystem* Subsystem =
        MakePresentationSubsystem(ESharPresentationDuplicatePolicy::Reject);
    const FSharPresentationRequest Request = MakePresentationRequest({
        .RequestId = FName(TEXT("skippable_request")),
        .PresentationId = FName(TEXT("mission_recap")),
        .OwnerId = FName(TEXT("mission_owner")),
        .ChannelId = FName(TEXT("cinematic")),
        .bSkipAllowed = true,
    });
    const FSharPresentationCallbackRevision Revision =
        MakePresentationRevision();
    StartPresentation(*Subsystem, Request, Revision);
    Subsystem->Pause(Request.RequestId);

    TestTrue(
        TEXT("Paused skippable playback publishes skipped result"),
        Subsystem->Skip(Request.RequestId)
            == ESharPresentationOperationResult::Accepted
    );
    TestTrue(
        TEXT("Skip remains distinct from completion"),
        Subsystem->GetTerminalResult(Request.RequestId)
            == ESharPresentationTerminalResult::Skipped
    );
    TestTrue(
        TEXT("Skipped request releases"),
        Subsystem->Release(Request.RequestId)
            == ESharPresentationOperationResult::Accepted
    );
    return true;
}

bool FSharPresentationOwnerTeardownTest::RunTest(
    const FString& Parameters
)
{
    (void)Parameters;
    USharPresentationPlaybackSubsystem* Subsystem =
        MakePresentationSubsystem(ESharPresentationDuplicatePolicy::Reject);
    const FSharPresentationRequest Active = MakePresentationRequest({
        .RequestId = FName(TEXT("owner_active")),
        .PresentationId = FName(TEXT("active_scene")),
        .OwnerId = FName(TEXT("interaction_owner")),
        .ChannelId = FName(TEXT("cinematic")),
    });
    const FSharPresentationRequest Pending = MakePresentationRequest({
        .RequestId = FName(TEXT("owner_pending")),
        .PresentationId = FName(TEXT("pending_scene")),
        .OwnerId = FName(TEXT("interaction_owner")),
        .ChannelId = FName(TEXT("cinematic")),
    });
    const FSharPresentationCallbackRevision Revision =
        MakePresentationRevision();
    StartPresentation(*Subsystem, Active, Revision);
    Subsystem->Enqueue(Pending);

    TestTrue(
        TEXT("Owner teardown releases active and pending requests"),
        Subsystem->CancelOwner(FName(TEXT("interaction_owner")))
            == ExpectedOwnerRequestCount
    );
    TestTrue(
        TEXT("Active request preserves cancelled terminal result"),
        Subsystem->GetTerminalResult(Active.RequestId)
            == ESharPresentationTerminalResult::Cancelled
    );
    TestTrue(
        TEXT("Pending request preserves cancelled terminal result"),
        Subsystem->GetTerminalResult(Pending.RequestId)
            == ESharPresentationTerminalResult::Cancelled
    );
    TestTrue(
        TEXT("Owner teardown leaves no active request"),
        Subsystem->GetActiveRequestCount() == 0
    );
    TestTrue(
        TEXT("Owner teardown leaves no unreleased request"),
        Subsystem->GetUnreleasedRequestCount() == 0
    );
    return true;
}

#endif
