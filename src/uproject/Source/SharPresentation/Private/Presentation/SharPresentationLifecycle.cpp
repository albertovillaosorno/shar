// File: SharPresentationLifecycle.cpp
// Path: src/uproject/Source/SharPresentation/Private/Presentation/SharPresentationLifecycle.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: readiness, start, pause, terminal result, cancellation, and release transitions only.
// Specification: docs/technical/unreal/presentation-playback-runtime.md
// LARGE-FILE owner=SharPresentation; reason=cohesive playback lifecycle and teardown implementation;
// split=extract terminal publication if adapter evidence becomes persistent;
// validation=validate.sh SharPresentation plus Unreal automation; review=2027-01.

#include "Presentation/SharPresentationPlaybackSubsystem.h"

#include "Content/SharPrimaryContentDefinition.h"

bool USharPresentationPlaybackSubsystem::MatchesRevision(
    const FSharPresentationPlaybackSnapshot& Snapshot,
    const FSharPresentationCallbackRevision& Revision
)
{
    return Snapshot.Request.OwnerRevision == Revision.OwnerRevision
        && Snapshot.Request.WorldRevision == Revision.WorldRevision
        && Snapshot.Request.RequestRevision == Revision.RequestRevision;
}

ESharPresentationOperationResult
USharPresentationPlaybackSubsystem::BeginLoading(const FName& RequestId)
{
    FSharPresentationPlaybackSnapshot* Snapshot = FindRequest(RequestId);
    if (Snapshot == nullptr)
    {
        return ESharPresentationOperationResult::NotFound;
    }
    if (Snapshot->bReleased)
    {
        return ESharPresentationOperationResult::Released;
    }
    if (Snapshot->State != ESharPresentationPlaybackState::Queued)
    {
        return ESharPresentationOperationResult::InvalidState;
    }
    if (!IsHead(*Snapshot))
    {
        return ESharPresentationOperationResult::NotHead;
    }
    Snapshot->State = ESharPresentationPlaybackState::Loading;
    return ESharPresentationOperationResult::Accepted;
}

ESharPresentationOperationResult USharPresentationPlaybackSubsystem::MarkReady(
    const FName& RequestId,
    const FSharPresentationCallbackRevision& Revision
)
{
    FSharPresentationPlaybackSnapshot* Snapshot = FindRequest(RequestId);
    if (Snapshot == nullptr)
    {
        return ESharPresentationOperationResult::NotFound;
    }
    if (Snapshot->bReleased)
    {
        return ESharPresentationOperationResult::Released;
    }
    if (Snapshot->State != ESharPresentationPlaybackState::Loading)
    {
        return ESharPresentationOperationResult::InvalidState;
    }
    if (!MatchesRevision(*Snapshot, Revision))
    {
        return ESharPresentationOperationResult::StaleRevision;
    }
    Snapshot->State = ESharPresentationPlaybackState::Ready;
    return ESharPresentationOperationResult::Accepted;
}

ESharPresentationOperationResult USharPresentationPlaybackSubsystem::BeginStart(
    const FName& RequestId,
    const FSharPresentationCallbackRevision& Revision
)
{
    FSharPresentationPlaybackSnapshot* Snapshot = FindRequest(RequestId);
    if (Snapshot == nullptr)
    {
        return ESharPresentationOperationResult::NotFound;
    }
    if (Snapshot->bReleased)
    {
        return ESharPresentationOperationResult::Released;
    }
    if (Snapshot->State != ESharPresentationPlaybackState::Ready)
    {
        return ESharPresentationOperationResult::InvalidState;
    }
    if (!MatchesRevision(*Snapshot, Revision))
    {
        return ESharPresentationOperationResult::StaleRevision;
    }
    if (!IsHead(*Snapshot))
    {
        return ESharPresentationOperationResult::NotHead;
    }
    const FSharPresentationChannelPolicy* Policy =
        FindChannel(Snapshot->Request.ChannelId);
    if (Policy == nullptr)
    {
        return ESharPresentationOperationResult::ChannelMissing;
    }
    if (CountActive(Snapshot->Request.ChannelId) >= Policy->MaximumActive)
    {
        return ESharPresentationOperationResult::QueueFull;
    }
    Snapshot->State = ESharPresentationPlaybackState::Starting;
    return ESharPresentationOperationResult::Accepted;
}

ESharPresentationOperationResult
USharPresentationPlaybackSubsystem::MarkPlaying(
    const FName& RequestId,
    const FSharPresentationCallbackRevision& Revision
)
{
    FSharPresentationPlaybackSnapshot* Snapshot = FindRequest(RequestId);
    if (Snapshot == nullptr)
    {
        return ESharPresentationOperationResult::NotFound;
    }
    if (Snapshot->bReleased)
    {
        return ESharPresentationOperationResult::Released;
    }
    if (Snapshot->State != ESharPresentationPlaybackState::Starting)
    {
        return ESharPresentationOperationResult::InvalidState;
    }
    if (!MatchesRevision(*Snapshot, Revision))
    {
        return ESharPresentationOperationResult::StaleRevision;
    }
    Snapshot->State = ESharPresentationPlaybackState::Playing;
    return ESharPresentationOperationResult::Accepted;
}

ESharPresentationOperationResult USharPresentationPlaybackSubsystem::Pause(
    const FName& RequestId
)
{
    FSharPresentationPlaybackSnapshot* Snapshot = FindRequest(RequestId);
    if (Snapshot == nullptr)
    {
        return ESharPresentationOperationResult::NotFound;
    }
    if (Snapshot->bReleased)
    {
        return ESharPresentationOperationResult::Released;
    }
    if (Snapshot->State != ESharPresentationPlaybackState::Playing)
    {
        return ESharPresentationOperationResult::InvalidState;
    }
    Snapshot->State = ESharPresentationPlaybackState::Paused;
    return ESharPresentationOperationResult::Accepted;
}

ESharPresentationOperationResult USharPresentationPlaybackSubsystem::Resume(
    const FName& RequestId
)
{
    FSharPresentationPlaybackSnapshot* Snapshot = FindRequest(RequestId);
    if (Snapshot == nullptr)
    {
        return ESharPresentationOperationResult::NotFound;
    }
    if (Snapshot->bReleased)
    {
        return ESharPresentationOperationResult::Released;
    }
    if (Snapshot->State != ESharPresentationPlaybackState::Paused)
    {
        return ESharPresentationOperationResult::InvalidState;
    }
    Snapshot->State = ESharPresentationPlaybackState::Playing;
    return ESharPresentationOperationResult::Accepted;
}

ESharPresentationOperationResult
USharPresentationPlaybackSubsystem::PublishTerminal(
    FSharPresentationPlaybackSnapshot& Snapshot,
    const ESharPresentationPlaybackState State,
    const ESharPresentationTerminalResult Result
)
{
    if (Snapshot.bReleased)
    {
        return ESharPresentationOperationResult::Released;
    }
    if (IsTerminalState(Snapshot.State))
    {
        return ESharPresentationOperationResult::AlreadyTerminal;
    }
    Snapshot.State = State;
    Snapshot.TerminalResult = Result;
    return ESharPresentationOperationResult::Accepted;
}

ESharPresentationOperationResult USharPresentationPlaybackSubsystem::Complete(
    const FName& RequestId
)
{
    FSharPresentationPlaybackSnapshot* Snapshot = FindRequest(RequestId);
    if (Snapshot == nullptr)
    {
        return ESharPresentationOperationResult::NotFound;
    }
    if (Snapshot->State != ESharPresentationPlaybackState::Playing)
    {
        return IsTerminalState(Snapshot->State)
            ? ESharPresentationOperationResult::AlreadyTerminal
            : ESharPresentationOperationResult::InvalidState;
    }
    return PublishTerminal(
        *Snapshot,
        ESharPresentationPlaybackState::Completed,
        ESharPresentationTerminalResult::Completed
    );
}

ESharPresentationOperationResult USharPresentationPlaybackSubsystem::Skip(
    const FName& RequestId
)
{
    FSharPresentationPlaybackSnapshot* Snapshot = FindRequest(RequestId);
    if (Snapshot == nullptr)
    {
        return ESharPresentationOperationResult::NotFound;
    }
    const bool bPlayable =
        Snapshot->State == ESharPresentationPlaybackState::Playing
        || Snapshot->State == ESharPresentationPlaybackState::Paused;
    if (!bPlayable || !Snapshot->Request.bSkipAllowed)
    {
        return IsTerminalState(Snapshot->State)
            ? ESharPresentationOperationResult::AlreadyTerminal
            : ESharPresentationOperationResult::InvalidState;
    }
    return PublishTerminal(
        *Snapshot,
        ESharPresentationPlaybackState::Skipped,
        ESharPresentationTerminalResult::Skipped
    );
}

ESharPresentationOperationResult USharPresentationPlaybackSubsystem::Cancel(
    const FName& RequestId
)
{
    FSharPresentationPlaybackSnapshot* Snapshot = FindRequest(RequestId);
    if (Snapshot == nullptr)
    {
        return ESharPresentationOperationResult::NotFound;
    }
    return PublishTerminal(
        *Snapshot,
        ESharPresentationPlaybackState::Cancelled,
        ESharPresentationTerminalResult::Cancelled
    );
}

ESharPresentationOperationResult USharPresentationPlaybackSubsystem::Fail(
    const FName& RequestId
)
{
    FSharPresentationPlaybackSnapshot* Snapshot = FindRequest(RequestId);
    if (Snapshot == nullptr)
    {
        return ESharPresentationOperationResult::NotFound;
    }
    return PublishTerminal(
        *Snapshot,
        ESharPresentationPlaybackState::Failed,
        ESharPresentationTerminalResult::Failed
    );
}

ESharPresentationOperationResult USharPresentationPlaybackSubsystem::Release(
    const FName& RequestId
)
{
    FSharPresentationPlaybackSnapshot* Snapshot = FindRequest(RequestId);
    if (Snapshot == nullptr)
    {
        return ESharPresentationOperationResult::NotFound;
    }
    if (Snapshot->bReleased)
    {
        return ESharPresentationOperationResult::Released;
    }
    if (!IsTerminalState(Snapshot->State))
    {
        return ESharPresentationOperationResult::InvalidState;
    }
    Snapshot->bReleased = true;
    Snapshot->State = ESharPresentationPlaybackState::Released;
    return ESharPresentationOperationResult::Accepted;
}

int32 USharPresentationPlaybackSubsystem::CancelOwner(const FName& OwnerId)
{
    if (!USharPrimaryContentDefinition::IsCanonicalIdentifier(OwnerId))
    {
        return 0;
    }
    int32 ReleasedCount = 0;
    for (FSharPresentationPlaybackSnapshot& Snapshot : Requests)
    {
        if (Snapshot.bReleased || Snapshot.Request.OwnerId != OwnerId)
        {
            continue;
        }
        if (!IsTerminalState(Snapshot.State))
        {
            const ESharPresentationOperationResult PublishResult =
                PublishTerminal(
                    Snapshot,
                    ESharPresentationPlaybackState::Cancelled,
                    ESharPresentationTerminalResult::Cancelled
                );
            if (PublishResult != ESharPresentationOperationResult::Accepted)
            {
                continue;
            }
        }
        Snapshot.bReleased = true;
        Snapshot.State = ESharPresentationPlaybackState::Released;
        ++ReleasedCount;
    }
    return ReleasedCount;
}

int32 USharPresentationPlaybackSubsystem::ClearChannel(
    const FName& ChannelId
)
{
    if (FindChannel(ChannelId) == nullptr)
    {
        return 0;
    }
    int32 ReleasedCount = 0;
    for (FSharPresentationPlaybackSnapshot& Snapshot : Requests)
    {
        if (Snapshot.bReleased || Snapshot.Request.ChannelId != ChannelId)
        {
            continue;
        }
        if (!IsTerminalState(Snapshot.State))
        {
            const ESharPresentationOperationResult PublishResult =
                PublishTerminal(
                    Snapshot,
                    ESharPresentationPlaybackState::Cancelled,
                    ESharPresentationTerminalResult::Cancelled
                );
            if (PublishResult != ESharPresentationOperationResult::Accepted)
            {
                continue;
            }
        }
        Snapshot.bReleased = true;
        Snapshot.State = ESharPresentationPlaybackState::Released;
        ++ReleasedCount;
    }
    return ReleasedCount;
}

ESharPresentationPlaybackState USharPresentationPlaybackSubsystem::GetState(
    const FName& RequestId
) const
{
    const FSharPresentationPlaybackSnapshot* Snapshot = FindRequest(RequestId);
    return Snapshot == nullptr
        ? ESharPresentationPlaybackState::Failed
        : Snapshot->State;
}

ESharPresentationTerminalResult
USharPresentationPlaybackSubsystem::GetTerminalResult(
    const FName& RequestId
) const
{
    const FSharPresentationPlaybackSnapshot* Snapshot = FindRequest(RequestId);
    return Snapshot == nullptr
        ? ESharPresentationTerminalResult::None
        : Snapshot->TerminalResult;
}

int32 USharPresentationPlaybackSubsystem::GetActiveRequestCount() const
{
    int32 ActiveCount = 0;
    for (const FSharPresentationPlaybackSnapshot& Snapshot : Requests)
    {
        ActiveCount += !Snapshot.bReleased && IsActiveState(Snapshot.State)
            ? 1
            : 0;
    }
    return ActiveCount;
}

int32 USharPresentationPlaybackSubsystem::GetUnreleasedRequestCount() const
{
    int32 UnreleasedCount = 0;
    for (const FSharPresentationPlaybackSnapshot& Snapshot : Requests)
    {
        UnreleasedCount += Snapshot.bReleased ? 0 : 1;
    }
    return UnreleasedCount;
}
