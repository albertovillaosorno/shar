// File: SharPresentationQueueTests.cpp
// Path: src/uproject/Source/SharPresentation/Private/Tests/SharPresentationQueueTests.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: deterministic ordering, head selection, and pending replacement tests only.
// Specification: docs/technical/unreal/presentation-playback-runtime.md
// LARGE-FILE owner=SharPresentation; reason=two cohesive queue arbitration scenarios;
// split=separate capacity tests if channel policy expands;
// validation=validate.sh SharPresentation plus Unreal automation; review=2027-01.

#if WITH_DEV_AUTOMATION_TESTS

#include "SharPresentationTestFixtures.h"

#include "Misc/AutomationTest.h"
#include "Presentation/SharPresentationPlaybackSubsystem.h"

namespace
{
constexpr int32 LowPriority = 10;
constexpr int32 HighPriority = 50;
constexpr int32 FirstQueuePosition = 1;
constexpr int32 SecondQueuePosition = 2;
constexpr int32 ThirdQueuePosition = 3;
constexpr int32 ExpectedReplacementCount = 1;

IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharPresentationQueueOrderingTest,
    "SHAR.Presentation.Queue.DeterministicOrdering",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharPresentationPendingReplacementTest,
    "SHAR.Presentation.Queue.PendingReplacement",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
} // namespace

bool FSharPresentationQueueOrderingTest::RunTest(
    const FString& Parameters
)
{
    (void)Parameters;
    USharPresentationPlaybackSubsystem* Subsystem =
        MakePresentationSubsystem(ESharPresentationDuplicatePolicy::Reject);
    const FSharPresentationRequest Low = MakePresentationRequest({
        .RequestId = FName(TEXT("request_low")),
        .PresentationId = FName(TEXT("ambient_low")),
        .OwnerId = FName(TEXT("owner_low")),
        .ChannelId = FName(TEXT("cinematic")),
        .Priority = LowPriority,
    });
    const FSharPresentationRequest HighFirst = MakePresentationRequest({
        .RequestId = FName(TEXT("request_high_first")),
        .PresentationId = FName(TEXT("mission_intro")),
        .OwnerId = FName(TEXT("owner_high_first")),
        .ChannelId = FName(TEXT("cinematic")),
        .Priority = HighPriority,
    });
    const FSharPresentationRequest HighSecond = MakePresentationRequest({
        .RequestId = FName(TEXT("request_high_second")),
        .PresentationId = FName(TEXT("mission_outro")),
        .OwnerId = FName(TEXT("owner_high_second")),
        .ChannelId = FName(TEXT("cinematic")),
        .Priority = HighPriority,
    });
    Subsystem->Enqueue(Low);
    Subsystem->Enqueue(HighFirst);
    Subsystem->Enqueue(HighSecond);

    TestTrue(
        TEXT("Higher priority request becomes queue head"),
        Subsystem->GetQueuePosition(HighFirst.RequestId) == FirstQueuePosition
    );
    TestTrue(
        TEXT("Equal priority preserves insertion order"),
        Subsystem->GetQueuePosition(HighSecond.RequestId) == SecondQueuePosition
    );
    TestTrue(
        TEXT("Lower priority remains after higher priority work"),
        Subsystem->GetQueuePosition(Low.RequestId) == ThirdQueuePosition
    );
    TestTrue(
        TEXT("Non-head request cannot begin loading"),
        Subsystem->BeginLoading(Low.RequestId)
            == ESharPresentationOperationResult::NotHead
    );
    TestTrue(
        TEXT("Accepted head begins loading"),
        Subsystem->BeginLoading(HighFirst.RequestId)
            == ESharPresentationOperationResult::Accepted
    );
    return true;
}

bool FSharPresentationPendingReplacementTest::RunTest(
    const FString& Parameters
)
{
    (void)Parameters;
    USharPresentationPlaybackSubsystem* Subsystem =
        MakePresentationSubsystem(
            ESharPresentationDuplicatePolicy::ReplacePending
        );
    const FSharPresentationRequest OldRequest = MakePresentationRequest({
        .RequestId = FName(TEXT("request_old")),
        .PresentationId = FName(TEXT("frontend_loop")),
        .OwnerId = FName(TEXT("frontend_owner")),
        .ChannelId = FName(TEXT("cinematic")),
    });
    const FSharPresentationRequest Replacement = MakePresentationRequest({
        .RequestId = FName(TEXT("request_replacement")),
        .PresentationId = FName(TEXT("frontend_loop")),
        .OwnerId = FName(TEXT("frontend_owner")),
        .ChannelId = FName(TEXT("cinematic")),
    });
    Subsystem->Enqueue(OldRequest);

    TestTrue(
        TEXT("Replacement request is accepted"),
        Subsystem->Enqueue(Replacement)
            == ESharPresentationOperationResult::Accepted
    );
    TestTrue(
        TEXT("Replaced pending request is released"),
        Subsystem->GetState(OldRequest.RequestId)
            == ESharPresentationPlaybackState::Released
    );
    TestTrue(
        TEXT("Replaced request preserves cancelled terminal result"),
        Subsystem->GetTerminalResult(OldRequest.RequestId)
            == ESharPresentationTerminalResult::Cancelled
    );
    TestTrue(
        TEXT("Only replacement remains unreleased"),
        Subsystem->GetUnreleasedRequestCount() == ExpectedReplacementCount
    );
    return true;
}

#endif
