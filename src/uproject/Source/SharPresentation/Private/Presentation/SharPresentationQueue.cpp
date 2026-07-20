// File: SharPresentationQueue.cpp
// Path: src/uproject/Source/SharPresentation/Private/Presentation/SharPresentationQueue.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: channel registration, request validation, deterministic queue ordering, and duplicate policy only.
// Specification: docs/technical/unreal/presentation-playback-runtime.md
// LARGE-FILE owner=SharPresentation; reason=cohesive bounded queue and arbitration implementation;
// split=extract channel diagnostics if starvation evidence becomes persistent;
// validation=validate.sh SharPresentation plus Unreal automation; review=2027-01.

#include "Presentation/SharPresentationPlaybackSubsystem.h"

#include "Algo/AnyOf.h"
#include "Algo/Find.h"
#include "Content/SharPrimaryContentDefinition.h"
#include "Engine/DataAsset.h"

static bool IsCanonicalPlaybackId(const FName& Candidate)
{
    return USharPrimaryContentDefinition::IsCanonicalIdentifier(Candidate);
}

static bool IsCanonicalOrNone(const FName& Candidate)
{
    return Candidate.IsNone() || IsCanonicalPlaybackId(Candidate);
}

bool USharPresentationPlaybackSubsystem::IsValidRevision(
    const FString& Revision
)
{
    return Revision.StartsWith(TEXT("sha256:"));
}

bool USharPresentationPlaybackSubsystem::IsValidRequest(
    const FSharPresentationRequest& Request
)
{
    const bool bInvalidIdentity =
        !IsCanonicalPlaybackId(Request.RequestId)
        || !Request.PresentationId.IsValid()
        || !IsCanonicalPlaybackId(Request.OwnerId)
        || !IsCanonicalPlaybackId(Request.ChannelId)
        || !IsCanonicalOrNone(Request.ParticipantId)
        || !IsCanonicalOrNone(Request.TargetId);
    const bool bInvalidRevision =
        !IsValidRevision(Request.OwnerRevision)
        || !IsValidRevision(Request.WorldRevision)
        || !IsValidRevision(Request.RequestRevision);
    const bool bInvalidDeadline =
        !FMath::IsFinite(Request.CompletionDeadlineSeconds)
        || Request.CompletionDeadlineSeconds < 0.0;
    return !bInvalidIdentity && !bInvalidRevision && !bInvalidDeadline;
}

const FSharPresentationChannelPolicy*
USharPresentationPlaybackSubsystem::FindChannel(const FName& ChannelId) const
{
    return Algo::FindByPredicate(
        Channels,
        [&ChannelId](const FSharPresentationChannelPolicy& Policy)
        {
            return Policy.ChannelId == ChannelId;
        }
    );
}

FSharPresentationPlaybackSnapshot*
USharPresentationPlaybackSubsystem::FindRequest(const FName& RequestId)
{
    return Algo::FindByPredicate(
        Requests,
        [&RequestId](const FSharPresentationPlaybackSnapshot& Snapshot)
        {
            return Snapshot.Request.RequestId == RequestId;
        }
    );
}

const FSharPresentationPlaybackSnapshot*
USharPresentationPlaybackSubsystem::FindRequest(const FName& RequestId) const
{
    return Algo::FindByPredicate(
        Requests,
        [&RequestId](const FSharPresentationPlaybackSnapshot& Snapshot)
        {
            return Snapshot.Request.RequestId == RequestId;
        }
    );
}

bool USharPresentationPlaybackSubsystem::IsPendingState(
    const ESharPresentationPlaybackState State
)
{
    return State == ESharPresentationPlaybackState::Queued
        || State == ESharPresentationPlaybackState::Loading
        || State == ESharPresentationPlaybackState::Ready;
}

bool USharPresentationPlaybackSubsystem::IsActiveState(
    const ESharPresentationPlaybackState State
)
{
    return State == ESharPresentationPlaybackState::Starting
        || State == ESharPresentationPlaybackState::Playing
        || State == ESharPresentationPlaybackState::Paused
        || State == ESharPresentationPlaybackState::Stopping;
}

bool USharPresentationPlaybackSubsystem::IsTerminalState(
    const ESharPresentationPlaybackState State
)
{
    return State == ESharPresentationPlaybackState::Completed
        || State == ESharPresentationPlaybackState::Skipped
        || State == ESharPresentationPlaybackState::Cancelled
        || State == ESharPresentationPlaybackState::Failed;
}

bool USharPresentationPlaybackSubsystem::Outranks(
    const FSharPresentationPlaybackSnapshot& Left,
    const FSharPresentationPlaybackSnapshot& Right
)
{
    if (Left.Request.Priority != Right.Request.Priority)
    {
        return Left.Request.Priority > Right.Request.Priority;
    }
    if (Left.InsertionSequence != Right.InsertionSequence)
    {
        return Left.InsertionSequence < Right.InsertionSequence;
    }
    return Left.Request.RequestId.LexicalLess(Right.Request.RequestId);
}

bool USharPresentationPlaybackSubsystem::IsHead(
    const FSharPresentationPlaybackSnapshot& QueueEntry
) const
{
    if (QueueEntry.bReleased || !IsPendingState(QueueEntry.State))
    {
        return false;
    }
    const bool bHasOutrankingEntry = Algo::AnyOf(
        Requests,
        [&QueueEntry](const FSharPresentationPlaybackSnapshot& Other)
        {
            const bool bComparable =
                !Other.bReleased
                && IsPendingState(Other.State)
                && Other.Request.ChannelId == QueueEntry.Request.ChannelId
                && Other.Request.RequestId != QueueEntry.Request.RequestId;
            return bComparable && Outranks(Other, QueueEntry);
        }
    );
    return !bHasOutrankingEntry;
}

int32 USharPresentationPlaybackSubsystem::CountPending(
    const FName& ChannelId
) const
{
    int32 Count = 0;
    for (const FSharPresentationPlaybackSnapshot& Snapshot : Requests)
    {
        Count += !Snapshot.bReleased
                && Snapshot.Request.ChannelId == ChannelId
                && IsPendingState(Snapshot.State)
            ? 1
            : 0;
    }
    return Count;
}

int32 USharPresentationPlaybackSubsystem::CountActive(
    const FName& ChannelId
) const
{
    int32 Count = 0;
    for (const FSharPresentationPlaybackSnapshot& Snapshot : Requests)
    {
        Count += !Snapshot.bReleased
                && Snapshot.Request.ChannelId == ChannelId
                && IsActiveState(Snapshot.State)
            ? 1
            : 0;
    }
    return Count;
}

bool USharPresentationPlaybackSubsystem::ConfigureWorld(
    const FName& InWorldId,
    const FString& InWorldRevision
)
{
    if (!IsCanonicalPlaybackId(InWorldId)
        || !IsValidRevision(InWorldRevision))
    {
        return false;
    }
    WorldId = InWorldId;
    WorldRevision = InWorldRevision;
    Channels.Reset();
    Requests.Reset();
    NextInsertionSequence = 0;
    return true;
}

bool USharPresentationPlaybackSubsystem::RegisterChannel(
    const FSharPresentationChannelPolicy& Policy
)
{
    const bool bInvalid =
        WorldId.IsNone()
        || !IsCanonicalPlaybackId(Policy.ChannelId)
        || !IsCanonicalPlaybackId(Policy.StarvationPolicyId)
        || !IsCanonicalPlaybackId(Policy.TeardownPolicyId)
        || Policy.MaximumPending <= 0
        || Policy.MaximumActive <= 0
        || FindChannel(Policy.ChannelId) != nullptr;
    if (bInvalid)
    {
        return false;
    }
    Channels.Add(Policy);
    return true;
}

bool USharPresentationPlaybackSubsystem::ReplacePendingDuplicate(
    const FSharPresentationRequest& Request,
    const FSharPresentationChannelPolicy& Policy
)
{
    if (Policy.DuplicatePolicy
        != ESharPresentationDuplicatePolicy::ReplacePending)
    {
        return false;
    }
    for (FSharPresentationPlaybackSnapshot& Snapshot : Requests)
    {
        const bool bMatches =
            !Snapshot.bReleased
            && IsPendingState(Snapshot.State)
            && Snapshot.Request.ChannelId == Request.ChannelId
            && Snapshot.Request.OwnerId == Request.OwnerId
            && Snapshot.Request.PresentationId == Request.PresentationId;
        if (!bMatches)
        {
            continue;
        }
        Snapshot.State = ESharPresentationPlaybackState::Cancelled;
        Snapshot.TerminalResult = ESharPresentationTerminalResult::Cancelled;
        Snapshot.bReleased = true;
        Snapshot.State = ESharPresentationPlaybackState::Released;
        return true;
    }
    return false;
}

ESharPresentationOperationResult USharPresentationPlaybackSubsystem::Enqueue(
    const FSharPresentationRequest& Request
)
{
    if (WorldId.IsNone() || !IsValidRequest(Request))
    {
        return ESharPresentationOperationResult::InvalidRequest;
    }
    if (Request.WorldRevision != WorldRevision)
    {
        return ESharPresentationOperationResult::StaleRevision;
    }
    const FSharPresentationChannelPolicy* Policy =
        FindChannel(Request.ChannelId);
    if (Policy == nullptr)
    {
        return ESharPresentationOperationResult::ChannelMissing;
    }
    if (FindRequest(Request.RequestId) != nullptr)
    {
        return ESharPresentationOperationResult::DuplicateRequest;
    }

    const bool bHasDuplicate = Algo::AnyOf(
        Requests,
        [&Request](const FSharPresentationPlaybackSnapshot& Snapshot)
        {
            return !Snapshot.bReleased
                && Snapshot.Request.ChannelId == Request.ChannelId
                && Snapshot.Request.OwnerId == Request.OwnerId
                && Snapshot.Request.PresentationId == Request.PresentationId;
        }
    );
    if (bHasDuplicate && !ReplacePendingDuplicate(Request, *Policy))
    {
        return ESharPresentationOperationResult::DuplicateRequest;
    }
    if (CountPending(Request.ChannelId) >= Policy->MaximumPending)
    {
        return ESharPresentationOperationResult::QueueFull;
    }

    FSharPresentationPlaybackSnapshot Snapshot;
    Snapshot.Request = Request;
    Snapshot.InsertionSequence = NextInsertionSequence;
    ++NextInsertionSequence;
    Requests.Add(Snapshot);
    return ESharPresentationOperationResult::Accepted;
}

int32 USharPresentationPlaybackSubsystem::GetQueuePosition(
    const FName& RequestId
) const
{
    const FSharPresentationPlaybackSnapshot* Candidate = FindRequest(RequestId);
    if (Candidate == nullptr || Candidate->bReleased
        || !IsPendingState(Candidate->State))
    {
        return 0;
    }
    int32 Position = 1;
    for (const FSharPresentationPlaybackSnapshot& Other : Requests)
    {
        const bool bComparable =
            !Other.bReleased
            && IsPendingState(Other.State)
            && Other.Request.ChannelId == Candidate->Request.ChannelId
            && Other.Request.RequestId != Candidate->Request.RequestId;
        Position += bComparable && Outranks(Other, *Candidate) ? 1 : 0;
    }
    return Position;
}
