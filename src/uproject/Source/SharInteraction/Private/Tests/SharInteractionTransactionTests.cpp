// File: SharInteractionTransactionTests.cpp
// Path: src/uproject/Source/SharInteraction/Private/Tests/SharInteractionTransactionTests.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: transient reservation, phase, compensation, and unload tests only.
// ADR: docs/adr/unreal/runtime/contextual-interaction-query-and-transaction.md
// LARGE-FILE owner=SharInteraction; reason=four cohesive transaction scenarios;
// split=separate compensation tests if typed executors expand;
// validation=validate.sh SharInteraction plus Unreal automation; review=2027-01.

#if WITH_DEV_AUTOMATION_TESTS

#include "SharInteractionTestFixtures.h"

#include "Interaction/SharInteractionSubsystem.h"
#include "Misc/AutomationTest.h"

static constexpr int32 TransactionPriority = 25;
static constexpr double TransactionDistanceSquared = 100.0;

static void RegisterExclusiveVehicleSource(
    USharInteractionSubsystem& Subsystem
)
{
    Subsystem.RegisterSource(MakeInteractionSource({
        .SourceId = FName(TEXT("vehicle_door_source")),
        .InteractionId = MakeInteractionId(TEXT("enter_vehicle")),
        .bExclusive = true,
    }));
}

static FSharInteractionQuery MakeVehicleEntryQuery(const TCHAR* InteractorId)
{
    FSharInteractionQuery Query = MakeInteractionQuery({
        .QueryId = FName(TEXT("vehicle_entry_query")),
        .InteractorId = FName(InteractorId),
    });
    Query.Candidates.Add(MakeInteractionCandidate({
        .SourceId = FName(TEXT("vehicle_door_source")),
        .InteractionId = MakeInteractionId(TEXT("enter_vehicle")),
        .Priority = TransactionPriority,
        .DistanceSquared = TransactionDistanceSquared,
    }));
    return Query;
}

static FSharInteractionCandidate SelectVehicleEntryCandidate(
    USharInteractionSubsystem& Subsystem,
    const FSharInteractionQuery& Query
)
{
    FSharInteractionCandidate Candidate;
    Subsystem.SelectCandidate(Query, Candidate);
    return Candidate;
}

static void AdvanceToEffectsCommitted(
    USharInteractionSubsystem& Subsystem,
    const FName& TransactionId
)
{
    Subsystem.AdvanceTransaction(
        TransactionId,
        ESharInteractionTransactionPhase::Reserved,
        ESharInteractionTransactionPhase::Revalidated
    );
    Subsystem.AdvanceTransaction(
        TransactionId,
        ESharInteractionTransactionPhase::Revalidated,
        ESharInteractionTransactionPhase::PresentationPrepared
    );
    Subsystem.AdvanceTransaction(
        TransactionId,
        ESharInteractionTransactionPhase::PresentationPrepared,
        ESharInteractionTransactionPhase::EffectsCommitted
    );
}

namespace
{
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharInteractionReservationGuardsTest,
    "SHAR.Interaction.Transaction.ReservationGuards",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharInteractionSuccessLifecycleTest,
    "SHAR.Interaction.Transaction.SuccessLifecycle",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharInteractionCompensationFailureTest,
    "SHAR.Interaction.Transaction.CompensationFailure",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharInteractionSourceUnloadTest,
    "SHAR.Interaction.Transaction.SourceUnload",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
} // namespace

bool FSharInteractionReservationGuardsTest::RunTest(
    const FString& Parameters
)
{
    (void)Parameters;
    USharInteractionSubsystem* Subsystem =
        MakeConfiguredInteractionSubsystem();
    RegisterExclusiveVehicleSource(*Subsystem);
    const FSharInteractionQuery FirstQuery = MakeVehicleEntryQuery(
        TEXT("player_01")
    );
    const FSharInteractionCandidate Candidate =
        SelectVehicleEntryCandidate(*Subsystem, FirstQuery);

    TestTrue(
        TEXT("First interaction reserves the source"),
        Subsystem->BeginTransaction(
            FirstQuery,
            Candidate,
            FName(TEXT("vehicle_entry_first"))
        ) == ESharInteractionResultCode::Accepted
    );
    TestTrue(
        TEXT("Duplicate input from one interactor is suppressed"),
        Subsystem->BeginTransaction(
            FirstQuery,
            Candidate,
            FName(TEXT("vehicle_entry_duplicate"))
        ) == ESharInteractionResultCode::AlreadyExecuting
    );

    const FSharInteractionQuery SecondQuery = MakeVehicleEntryQuery(
        TEXT("player_02")
    );
    TestTrue(
        TEXT("Exclusive slot rejects another interactor"),
        Subsystem->BeginTransaction(
            SecondQuery,
            Candidate,
            FName(TEXT("vehicle_entry_second"))
        ) == ESharInteractionResultCode::SlotUnavailable
    );
    TestTrue(
        TEXT("Cancellation releases the reservation"),
        Subsystem->CancelTransaction(
            FName(TEXT("vehicle_entry_first")),
            true
        ) == ESharInteractionResultCode::Cancelled
    );
    TestFalse(
        TEXT("Cancelled interaction no longer reserves the source"),
        Subsystem->IsSourceReserved(Candidate.SourceId)
    );
    return true;
}

bool FSharInteractionSuccessLifecycleTest::RunTest(
    const FString& Parameters
)
{
    (void)Parameters;
    USharInteractionSubsystem* Subsystem =
        MakeConfiguredInteractionSubsystem();
    RegisterExclusiveVehicleSource(*Subsystem);
    const FSharInteractionQuery Query = MakeVehicleEntryQuery(
        TEXT("player_01")
    );
    const FSharInteractionCandidate Candidate =
        SelectVehicleEntryCandidate(*Subsystem, Query);
    const FName TransactionId(TEXT("vehicle_entry_success"));
    Subsystem->BeginTransaction(Query, Candidate, TransactionId);

    TestTrue(
        TEXT("Skipping a transaction phase is rejected"),
        Subsystem->AdvanceTransaction(
            TransactionId,
            ESharInteractionTransactionPhase::Reserved,
            ESharInteractionTransactionPhase::EffectsCommitted
        ) == ESharInteractionResultCode::InvalidPhase
    );
    AdvanceToEffectsCommitted(*Subsystem, TransactionId);
    TestTrue(
        TEXT("Verified commit publishes one completed result"),
        Subsystem->CompleteTransaction(TransactionId, true)
            == ESharInteractionResultCode::Completed
    );
    TestTrue(
        TEXT("Completed transaction is released"),
        Subsystem->GetTransactionPhase(TransactionId)
            == ESharInteractionTransactionPhase::Released
    );
    TestTrue(
        TEXT("No active transaction remains"),
        Subsystem->GetActiveTransactionCount() == 0
    );
    return true;
}

bool FSharInteractionCompensationFailureTest::RunTest(
    const FString& Parameters
)
{
    (void)Parameters;
    USharInteractionSubsystem* Subsystem =
        MakeConfiguredInteractionSubsystem();
    RegisterExclusiveVehicleSource(*Subsystem);
    const FSharInteractionQuery Query = MakeVehicleEntryQuery(
        TEXT("player_01")
    );
    const FSharInteractionCandidate Candidate =
        SelectVehicleEntryCandidate(*Subsystem, Query);
    const FName TransactionId(TEXT("vehicle_entry_compensation"));
    Subsystem->BeginTransaction(Query, Candidate, TransactionId);
    AdvanceToEffectsCommitted(*Subsystem, TransactionId);

    TestTrue(
        TEXT("Failed post-commit compensation returns typed failure"),
        Subsystem->CancelTransaction(TransactionId, false)
            == ESharInteractionResultCode::CompensationFailed
    );
    TestTrue(
        TEXT("Compensation failure disables the transaction"),
        Subsystem->GetTransactionPhase(TransactionId)
            == ESharInteractionTransactionPhase::Failed
    );
    TestFalse(
        TEXT("Compensation failure releases the reservation"),
        Subsystem->IsSourceReserved(Candidate.SourceId)
    );
    return true;
}

bool FSharInteractionSourceUnloadTest::RunTest(const FString& Parameters)
{
    (void)Parameters;
    USharInteractionSubsystem* Subsystem =
        MakeConfiguredInteractionSubsystem();
    RegisterExclusiveVehicleSource(*Subsystem);
    const FSharInteractionQuery Query = MakeVehicleEntryQuery(
        TEXT("player_01")
    );
    const FSharInteractionCandidate Candidate =
        SelectVehicleEntryCandidate(*Subsystem, Query);
    const FName TransactionId(TEXT("vehicle_entry_unload"));
    Subsystem->BeginTransaction(Query, Candidate, TransactionId);

    TestTrue(
        TEXT("Source unregisters during an active transaction"),
        Subsystem->UnregisterSource(Candidate.SourceId)
    );
    TestTrue(
        TEXT("Source unload publishes a typed downstream failure"),
        Subsystem->GetTransactionResult(TransactionId)
            == ESharInteractionResultCode::DownstreamRejected
    );
    TestTrue(
        TEXT("Source unload terminates the transaction"),
        Subsystem->GetTransactionPhase(TransactionId)
            == ESharInteractionTransactionPhase::Failed
    );
    TestFalse(
        TEXT("Source unload releases every reservation"),
        Subsystem->IsSourceReserved(Candidate.SourceId)
    );
    return true;
}

#endif
