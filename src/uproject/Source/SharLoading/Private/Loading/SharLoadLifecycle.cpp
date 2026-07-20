// File: SharLoadLifecycle.cpp
// Path: src/uproject/Source/SharLoading/Private/Loading/SharLoadLifecycle.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: terminal results, cancellation, timeout, supersession, release, and immutable query projections only.
// Specification: docs/technical/unreal/native-asset-load-request-and-streaming-runtime.md
// LARGE-FILE owner=SharLoading; reason=cohesive load-request terminal and residency lifecycle;
// split=extract terminal diagnostics if typed failure reasons become persistent;
// validation=validate.sh SharLoading plus Unreal automation; review=2027-01.

#include "Loading/SharLoadCoordinatorSubsystem.h"

static void CancelOutstandingNodes(FSharLoadRequestSnapshot& Snapshot)
{
    for (FSharLoadNodeSnapshot& Node : Snapshot.Nodes)
    {
        if (Node.State == ESharLoadNodeState::Pending
            || Node.State == ESharLoadNodeState::Active)
        {
            Node.State = ESharLoadNodeState::Cancelled;
        }
    }
}

ESharLoadOperationResult USharLoadCoordinatorSubsystem::PublishTerminal(
    FSharLoadRequestSnapshot& Snapshot,
    const ESharLoadRequestState State,
    const ESharLoadTerminalResult Result
)
{
    if (Snapshot.bReleased)
    {
        return ESharLoadOperationResult::Released;
    }
    if (IsTerminalState(Snapshot.State))
    {
        return ESharLoadOperationResult::AlreadyTerminal;
    }
    if (Result != ESharLoadTerminalResult::Success
        && Result != ESharLoadTerminalResult::Degraded)
    {
        CancelOutstandingNodes(Snapshot);
    }
    Snapshot.State = State;
    Snapshot.TerminalResult = Result;
    RefreshProgress(Snapshot);
    return ESharLoadOperationResult::Accepted;
}

static ESharLoadRequestState ResolveTerminalState(
    const ESharLoadTerminalCommand Command
)
{
    switch (Command)
    {
    case ESharLoadTerminalCommand::MarkUnavailable:
        return ESharLoadRequestState::Unavailable;
    case ESharLoadTerminalCommand::Timeout:
        return ESharLoadRequestState::TimedOut;
    case ESharLoadTerminalCommand::Cancel:
        return ESharLoadRequestState::Cancelled;
    case ESharLoadTerminalCommand::Supersede:
        return ESharLoadRequestState::Superseded;
    default:
        return ESharLoadRequestState::Rejected;
    }
}

static ESharLoadTerminalResult ResolveTerminalResult(
    const ESharLoadTerminalCommand Command
)
{
    switch (Command)
    {
    case ESharLoadTerminalCommand::MarkUnavailable:
        return ESharLoadTerminalResult::Unavailable;
    case ESharLoadTerminalCommand::Timeout:
        return ESharLoadTerminalResult::TimedOut;
    case ESharLoadTerminalCommand::Cancel:
        return ESharLoadTerminalResult::Cancelled;
    case ESharLoadTerminalCommand::Supersede:
        return ESharLoadTerminalResult::Superseded;
    default:
        return ESharLoadTerminalResult::Rejected;
    }
}

ESharLoadOperationResult USharLoadCoordinatorSubsystem::ResolveTerminal(
    const FSharLoadTerminalRequest& TerminalRequest
)
{
    FSharLoadRequestSnapshot* Snapshot = FindRequest(TerminalRequest.RequestId);
    if (Snapshot == nullptr)
    {
        return ESharLoadOperationResult::NotFound;
    }
    const ESharLoadOperationResult Result = PublishTerminal(
        *Snapshot,
        ResolveTerminalState(TerminalRequest.Command),
        ResolveTerminalResult(TerminalRequest.Command)
    );
    if (Result != ESharLoadOperationResult::Accepted)
    {
        return Result;
    }
    const FSharLoadPlan* Plan = FindPlan(Snapshot->Request.PlanId);
    const bool bRetained = Plan != nullptr
        && ReleaseSharedDependencies(*Plan, *Snapshot);
    return bRetained
        ? ESharLoadOperationResult::SharedWorkRetained
        : ESharLoadOperationResult::Accepted;
}

ESharLoadOperationResult USharLoadCoordinatorSubsystem::Release(
    const FName& RequestId
)
{
    FSharLoadRequestSnapshot* Snapshot = FindRequest(RequestId);
    if (Snapshot == nullptr)
    {
        return ESharLoadOperationResult::NotFound;
    }
    if (Snapshot->bReleased)
    {
        return ESharLoadOperationResult::Released;
    }
    if (!IsTerminalState(Snapshot->State))
    {
        return ESharLoadOperationResult::InvalidState;
    }
    const FSharLoadPlan* Plan = FindPlan(Snapshot->Request.PlanId);
    const bool bRetained = Plan != nullptr
        && ReleaseSharedDependencies(*Plan, *Snapshot);
    Snapshot->bReleased = true;
    Snapshot->State = ESharLoadRequestState::Released;
    return bRetained
        ? ESharLoadOperationResult::SharedWorkRetained
        : ESharLoadOperationResult::Accepted;
}

ESharLoadRequestState USharLoadCoordinatorSubsystem::GetState(
    const FName& RequestId
) const
{
    const FSharLoadRequestSnapshot* Snapshot = FindRequest(RequestId);
    return Snapshot == nullptr
        ? ESharLoadRequestState::Rejected
        : Snapshot->State;
}

ESharLoadTerminalResult USharLoadCoordinatorSubsystem::GetTerminalResult(
    const FName& RequestId
) const
{
    const FSharLoadRequestSnapshot* Snapshot = FindRequest(RequestId);
    return Snapshot == nullptr
        ? ESharLoadTerminalResult::None
        : Snapshot->TerminalResult;
}

FSharLoadProgress USharLoadCoordinatorSubsystem::GetProgress(
    const FName& RequestId
) const
{
    const FSharLoadRequestSnapshot* Snapshot = FindRequest(RequestId);
    return Snapshot == nullptr ? FSharLoadProgress() : Snapshot->Progress;
}

int32 USharLoadCoordinatorSubsystem::GetUnreleasedRequestCount() const
{
    int32 Count = 0;
    for (const FSharLoadRequestSnapshot& Snapshot : Requests)
    {
        Count += Snapshot.bReleased ? 0 : 1;
    }
    return Count;
}
