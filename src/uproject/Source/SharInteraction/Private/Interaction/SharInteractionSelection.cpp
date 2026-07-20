// File: SharInteractionSelection.cpp
// Path: src/uproject/Source/SharInteraction/Private/Interaction/SharInteractionSelection.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: source registration, query validation, and deterministic candidate selection only.
// ADR: docs/adr/unreal/runtime/contextual-interaction-query-and-transaction.md
// LARGE-FILE owner=SharInteraction; reason=cohesive interaction discovery and ordering implementation;
// split=extract source lifecycle if source diagnostics become persistent;
// validation=validate.sh SharInteraction plus Unreal automation; review=2027-01.

#include "Interaction/SharInteractionSubsystem.h"

#include "Algo/Find.h"
#include "Content/SharPrimaryContentDefinition.h"
#include "Engine/DataAsset.h"

static bool IsCanonicalRuntimeId(const FName& Candidate)
{
    return USharPrimaryContentDefinition::IsCanonicalIdentifier(Candidate);
}

static bool IsRuntimeRevision(const FString& Candidate)
{
    return Candidate.StartsWith(TEXT("sha256:"));
}

const FSharInteractionSourceState* USharInteractionSubsystem::FindSource(
    const FName& SourceId
) const
{
    return Algo::FindByPredicate(
        Sources,
        [&SourceId](const FSharInteractionSourceState& Source)
        {
            return Source.SourceId == SourceId;
        }
    );
}

FSharInteractionSourceState* USharInteractionSubsystem::FindSource(
    const FName& SourceId
)
{
    return Algo::FindByPredicate(
        Sources,
        [&SourceId](const FSharInteractionSourceState& Source)
        {
            return Source.SourceId == SourceId;
        }
    );
}

const FSharInteractionTransactionState*
USharInteractionSubsystem::FindTransaction(
    const FName& TransactionId
) const
{
    return Algo::FindByPredicate(
        Transactions,
        [&TransactionId](const FSharInteractionTransactionState& Transaction)
        {
            return Transaction.TransactionId == TransactionId;
        }
    );
}

FSharInteractionTransactionState* USharInteractionSubsystem::FindTransaction(
    const FName& TransactionId
)
{
    return Algo::FindByPredicate(
        Transactions,
        [&TransactionId](const FSharInteractionTransactionState& Transaction)
        {
            return Transaction.TransactionId == TransactionId;
        }
    );
}

bool USharInteractionSubsystem::ConfigureWorld(
    const FName& InWorldId,
    const FString& InWorldRevision
)
{
    if (!IsCanonicalRuntimeId(InWorldId)
        || !IsRuntimeRevision(InWorldRevision))
    {
        return false;
    }
    WorldId = InWorldId;
    WorldRevision = InWorldRevision;
    Sources.Reset();
    Transactions.Reset();
    return true;
}

bool USharInteractionSubsystem::RegisterSource(
    const FSharInteractionSourceState& Source
)
{
    const bool bInvalid =
        WorldId.IsNone()
        || !IsCanonicalRuntimeId(Source.SourceId)
        || !Source.InteractionId.IsValid()
        || !IsRuntimeRevision(Source.SourceRevision)
        || FindSource(Source.SourceId) != nullptr;
    if (bInvalid)
    {
        return false;
    }
    Sources.Add(Source);
    return true;
}

bool USharInteractionSubsystem::UnregisterSource(const FName& SourceId)
{
    FSharInteractionSourceState* Source = FindSource(SourceId);
    if (Source == nullptr || !Source->bEnabled)
    {
        return false;
    }
    Source->bEnabled = false;
    FailTransactionsForSource(SourceId);
    return true;
}

bool USharInteractionSubsystem::CandidateOutranks(
    const FSharInteractionCandidate& Candidate,
    const FSharInteractionCandidate& Current
)
{
    if (Candidate.Priority != Current.Priority)
    {
        return Candidate.Priority > Current.Priority;
    }
    if (Candidate.DistanceSquared < Current.DistanceSquared)
    {
        return true;
    }
    if (Candidate.DistanceSquared > Current.DistanceSquared)
    {
        return false;
    }
    if (Candidate.InteractionId != Current.InteractionId)
    {
        return Candidate.InteractionId.ToString()
            < Current.InteractionId.ToString();
    }
    return Candidate.SourceId.LexicalLess(Current.SourceId);
}

ESharInteractionResultCode USharInteractionSubsystem::ValidateQuery(
    const FSharInteractionQuery& Query
) const
{
    const bool bInvalidIdentity =
        !IsCanonicalRuntimeId(Query.QueryId)
        || !IsCanonicalRuntimeId(Query.InteractorId)
        || !IsRuntimeRevision(Query.InteractorRevision);
    if (bInvalidIdentity || Query.Candidates.IsEmpty())
    {
        return ESharInteractionResultCode::InvalidRequest;
    }
    if (Query.WorldRevision != WorldRevision)
    {
        return ESharInteractionResultCode::SourceStale;
    }
    return ESharInteractionResultCode::Accepted;
}

ESharInteractionResultCode USharInteractionSubsystem::ValidateCandidate(
    const FSharInteractionQuery& Query,
    const FSharInteractionCandidate& Candidate
) const
{
    const bool bInvalidCandidate =
        !IsCanonicalRuntimeId(Candidate.SourceId)
        || !Candidate.InteractionId.IsValid()
        || !IsRuntimeRevision(Candidate.SourceRevision)
        || !FMath::IsFinite(Candidate.DistanceSquared)
        || Candidate.DistanceSquared < 0.0;
    if (bInvalidCandidate)
    {
        return ESharInteractionResultCode::InvalidRequest;
    }
    const FSharInteractionSourceState* Source = FindSource(Candidate.SourceId);
    if (Source == nullptr || !Source->bEnabled)
    {
        return ESharInteractionResultCode::NotFound;
    }
    if (Source->InteractionId != Candidate.InteractionId
        || Source->SourceRevision != Candidate.SourceRevision)
    {
        return ESharInteractionResultCode::SourceStale;
    }
    if (!Candidate.bEligible)
    {
        return ESharInteractionResultCode::NotEligible;
    }
    if (Query.InteractorRevision.IsEmpty())
    {
        return ESharInteractionResultCode::InteractorStale;
    }
    return ESharInteractionResultCode::Accepted;
}

ESharInteractionResultCode USharInteractionSubsystem::SelectCandidate(
    const FSharInteractionQuery& Query,
    FSharInteractionCandidate& OutCandidate
) const
{
    const ESharInteractionResultCode QueryResult = ValidateQuery(Query);
    if (QueryResult != ESharInteractionResultCode::Accepted)
    {
        return QueryResult;
    }

    const FSharInteractionCandidate* BestCandidate = nullptr;
    ESharInteractionResultCode LastFailure =
        ESharInteractionResultCode::NotFound;
    for (const FSharInteractionCandidate& Candidate : Query.Candidates)
    {
        const ESharInteractionResultCode CandidateResult =
            ValidateCandidate(Query, Candidate);
        if (CandidateResult != ESharInteractionResultCode::Accepted)
        {
            LastFailure = CandidateResult;
            continue;
        }
        if (BestCandidate == nullptr
            || CandidateOutranks(Candidate, *BestCandidate))
        {
            BestCandidate = &Candidate;
        }
    }
    if (BestCandidate == nullptr)
    {
        return LastFailure;
    }
    OutCandidate = *BestCandidate;
    return ESharInteractionResultCode::Accepted;
}
