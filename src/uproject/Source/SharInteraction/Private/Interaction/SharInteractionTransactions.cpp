// File: SharInteractionTransactions.cpp
// Path: src/uproject/Source/SharInteraction/Private/Interaction/SharInteractionTransactions.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: interaction reservation, phase transitions, completion, cancellation, and source teardown only.
// ADR: docs/adr/unreal/runtime/contextual-interaction-query-and-transaction.md
// LARGE-FILE owner=SharInteraction; reason=cohesive transaction lifecycle implementation;
// split=extract compensation observations if durable diagnostics are introduced;
// validation=validate.sh SharInteraction plus Unreal automation; review=2027-01.

#include "Interaction/SharInteractionSubsystem.h"

#include "Algo/AnyOf.h"
#include "Content/SharPrimaryContentDefinition.h"

static bool IsTerminalPhase(
    const ESharInteractionTransactionPhase Phase
)
{
    return Phase == ESharInteractionTransactionPhase::Released
        || Phase == ESharInteractionTransactionPhase::Cancelled
        || Phase == ESharInteractionTransactionPhase::Failed;
}

static bool IsCanonicalTransactionId(const FName& TransactionId)
{
    return USharPrimaryContentDefinition::IsCanonicalIdentifier(TransactionId);
}

static ESharInteractionTransactionPhase GetAllowedNextPhase(
    const ESharInteractionTransactionPhase Current
)
{
    switch (Current)
    {
    case ESharInteractionTransactionPhase::Reserved:
        return ESharInteractionTransactionPhase::Revalidated;
    case ESharInteractionTransactionPhase::Revalidated:
        return ESharInteractionTransactionPhase::PresentationPrepared;
    case ESharInteractionTransactionPhase::PresentationPrepared:
        return ESharInteractionTransactionPhase::EffectsCommitted;
    case ESharInteractionTransactionPhase::Query:
    case ESharInteractionTransactionPhase::EffectsCommitted:
    case ESharInteractionTransactionPhase::ResultPublished:
    case ESharInteractionTransactionPhase::Released:
    case ESharInteractionTransactionPhase::Cancelled:
    case ESharInteractionTransactionPhase::Failed:
    default:
        return ESharInteractionTransactionPhase::Failed;
    }
}

bool USharInteractionSubsystem::HasActiveInteractorTransaction(
    const FName& InteractorId
) const
{
    return Algo::AnyOf(
        Transactions,
        [&InteractorId](const FSharInteractionTransactionState& Transaction)
        {
            return Transaction.InteractorId == InteractorId
                && !IsTerminalPhase(Transaction.Phase);
        }
    );
}

bool USharInteractionSubsystem::IsSourceReserved(
    const FName& SourceId
) const
{
    return Algo::AnyOf(
        Transactions,
        [&SourceId](const FSharInteractionTransactionState& Transaction)
        {
            return Transaction.SourceId == SourceId
                && Transaction.bReservationHeld;
        }
    );
}

void USharInteractionSubsystem::ReleaseReservation(
    FSharInteractionTransactionState& Transaction
)
{
    Transaction.bReservationHeld = false;
}

void USharInteractionSubsystem::FailTransactionsForSource(
    const FName& SourceId
)
{
    for (FSharInteractionTransactionState& Transaction : Transactions)
    {
        if (Transaction.SourceId != SourceId || IsTerminalPhase(Transaction.Phase))
        {
            continue;
        }
        Transaction.Result = ESharInteractionResultCode::DownstreamRejected;
        Transaction.Phase = ESharInteractionTransactionPhase::Failed;
        ReleaseReservation(Transaction);
    }
}

ESharInteractionResultCode USharInteractionSubsystem::BeginTransaction(
    const FSharInteractionQuery& Query,
    const FSharInteractionCandidate& Candidate,
    const FName& TransactionId
)
{
    if (!IsCanonicalTransactionId(TransactionId)
        || FindTransaction(TransactionId) != nullptr)
    {
        return ESharInteractionResultCode::InvalidRequest;
    }
    if (HasActiveInteractorTransaction(Query.InteractorId))
    {
        return ESharInteractionResultCode::AlreadyExecuting;
    }

    FSharInteractionCandidate SelectedCandidate;
    const ESharInteractionResultCode SelectionResult = SelectCandidate(
        Query,
        SelectedCandidate
    );
    if (SelectionResult != ESharInteractionResultCode::Accepted)
    {
        return SelectionResult;
    }
    const bool bCandidateMismatch =
        SelectedCandidate.SourceId != Candidate.SourceId
        || SelectedCandidate.InteractionId != Candidate.InteractionId
        || SelectedCandidate.SourceRevision != Candidate.SourceRevision;
    if (bCandidateMismatch)
    {
        return ESharInteractionResultCode::NotEligible;
    }

    const FSharInteractionSourceState* Source = FindSource(Candidate.SourceId);
    if (Source == nullptr || !Source->bEnabled)
    {
        return ESharInteractionResultCode::NotFound;
    }
    if (Source->bExclusive && IsSourceReserved(Source->SourceId))
    {
        return ESharInteractionResultCode::SlotUnavailable;
    }

    FSharInteractionTransactionState Transaction;
    Transaction.TransactionId = TransactionId;
    Transaction.InteractorId = Query.InteractorId;
    Transaction.SourceId = Candidate.SourceId;
    Transaction.InteractionId = Candidate.InteractionId;
    Transaction.WorldRevision = Query.WorldRevision;
    Transaction.InteractorRevision = Query.InteractorRevision;
    Transaction.SourceRevision = Candidate.SourceRevision;
    Transaction.Phase = ESharInteractionTransactionPhase::Reserved;
    Transaction.Result = ESharInteractionResultCode::Accepted;
    Transaction.bReservationHeld = true;
    Transactions.Add(Transaction);
    return ESharInteractionResultCode::Accepted;
}

ESharInteractionResultCode USharInteractionSubsystem::AdvanceTransaction(
    const FName& TransactionId,
    const ESharInteractionTransactionPhase ExpectedPhase,
    const ESharInteractionTransactionPhase NextPhase
)
{
    FSharInteractionTransactionState* Transaction = FindTransaction(TransactionId);
    if (Transaction == nullptr)
    {
        return ESharInteractionResultCode::NotFound;
    }
    if (Transaction->Phase != ExpectedPhase
        || GetAllowedNextPhase(ExpectedPhase) != NextPhase)
    {
        return ESharInteractionResultCode::InvalidPhase;
    }
    const FSharInteractionSourceState* Source = FindSource(Transaction->SourceId);
    if (Source == nullptr || !Source->bEnabled)
    {
        Transaction->Result = ESharInteractionResultCode::SourceStale;
        Transaction->Phase = ESharInteractionTransactionPhase::Failed;
        ReleaseReservation(*Transaction);
        return Transaction->Result;
    }
    if (Source->SourceRevision != Transaction->SourceRevision)
    {
        Transaction->Result = ESharInteractionResultCode::SourceStale;
        Transaction->Phase = ESharInteractionTransactionPhase::Failed;
        ReleaseReservation(*Transaction);
        return Transaction->Result;
    }
    Transaction->Phase = NextPhase;
    return ESharInteractionResultCode::Accepted;
}

ESharInteractionResultCode USharInteractionSubsystem::CompleteTransaction(
    const FName& TransactionId,
    const bool bVerificationSucceeded
)
{
    FSharInteractionTransactionState* Transaction = FindTransaction(TransactionId);
    if (Transaction == nullptr)
    {
        return ESharInteractionResultCode::NotFound;
    }
    if (Transaction->Phase != ESharInteractionTransactionPhase::EffectsCommitted)
    {
        return ESharInteractionResultCode::InvalidPhase;
    }
    if (!bVerificationSucceeded)
    {
        Transaction->Result = ESharInteractionResultCode::VerificationFailed;
        Transaction->Phase = ESharInteractionTransactionPhase::Failed;
        ReleaseReservation(*Transaction);
        return Transaction->Result;
    }
    Transaction->Phase = ESharInteractionTransactionPhase::ResultPublished;
    Transaction->Result = ESharInteractionResultCode::Completed;
    ReleaseReservation(*Transaction);
    Transaction->Phase = ESharInteractionTransactionPhase::Released;
    return Transaction->Result;
}

ESharInteractionResultCode USharInteractionSubsystem::CancelTransaction(
    const FName& TransactionId,
    const bool bCompensationSucceeded
)
{
    FSharInteractionTransactionState* Transaction = FindTransaction(TransactionId);
    if (Transaction == nullptr)
    {
        return ESharInteractionResultCode::NotFound;
    }
    if (IsTerminalPhase(Transaction->Phase))
    {
        return ESharInteractionResultCode::InvalidPhase;
    }
    const bool bNeedsCompensation =
        Transaction->Phase == ESharInteractionTransactionPhase::EffectsCommitted
        || Transaction->Phase
            == ESharInteractionTransactionPhase::ResultPublished;
    if (bNeedsCompensation && !bCompensationSucceeded)
    {
        Transaction->Result = ESharInteractionResultCode::CompensationFailed;
        Transaction->Phase = ESharInteractionTransactionPhase::Failed;
        ReleaseReservation(*Transaction);
        return Transaction->Result;
    }
    Transaction->Result = ESharInteractionResultCode::Cancelled;
    Transaction->Phase = ESharInteractionTransactionPhase::Cancelled;
    ReleaseReservation(*Transaction);
    return Transaction->Result;
}

ESharInteractionTransactionPhase USharInteractionSubsystem::GetTransactionPhase(
    const FName& TransactionId
) const
{
    const FSharInteractionTransactionState* Transaction =
        FindTransaction(TransactionId);
    return Transaction == nullptr
        ? ESharInteractionTransactionPhase::Failed
        : Transaction->Phase;
}

ESharInteractionResultCode USharInteractionSubsystem::GetTransactionResult(
    const FName& TransactionId
) const
{
    const FSharInteractionTransactionState* Transaction =
        FindTransaction(TransactionId);
    return Transaction == nullptr
        ? ESharInteractionResultCode::NotFound
        : Transaction->Result;
}

int32 USharInteractionSubsystem::GetActiveTransactionCount() const
{
    int32 ActiveCount = 0;
    for (const FSharInteractionTransactionState& Transaction : Transactions)
    {
        ActiveCount += IsTerminalPhase(Transaction.Phase) ? 0 : 1;
    }
    return ActiveCount;
}
