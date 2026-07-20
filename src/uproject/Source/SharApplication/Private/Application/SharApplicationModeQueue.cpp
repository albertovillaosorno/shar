// File: SharApplicationModeQueue.cpp
// Path: src/uproject/Source/SharApplication/Private/Application/SharApplicationModeQueue.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: coordinator configuration, request validation, deterministic ordering, submission, and transition start only.
// Specification: docs/technical/unreal/application-lifecycle-and-mode-runtime.md
// LARGE-FILE owner=SharApplication; reason=cohesive application-transition admission and arbitration;
// split=extract capacity policy if transition budgets become configurable;
// validation=validate.sh SharApplication plus Unreal automation; review=2027-01.

#include "Application/SharApplicationModeCoordinator.h"

#include "Algo/AnyOf.h"
#include "Algo/Find.h"
#include "Application/SharApplicationModeCatalogSubsystem.h"
#include "Content/SharPrimaryContentDefinition.h"

static constexpr int32 MaximumPendingTransitions = 32;

static bool IsCanonicalApplicationIdentity(const FName& Candidate)
{
    return USharPrimaryContentDefinition::IsCanonicalIdentifier(Candidate);
}

static bool IsCanonicalOrNone(const FName& Candidate)
{
    return Candidate.IsNone() || IsCanonicalApplicationIdentity(Candidate);
}

bool USharApplicationModeCoordinator::IsRevisionToken(
    const FString& Revision
)
{
    return Revision.StartsWith(TEXT("sha256:"));
}

bool USharApplicationModeCoordinator::IsValidRequest(
    const FSharApplicationModeRequest& Request
)
{
    const bool bInvalidIdentity =
        !IsCanonicalApplicationIdentity(Request.RequestId)
        || !IsCanonicalApplicationIdentity(Request.SourceModeId)
        || !IsCanonicalApplicationIdentity(Request.TargetModeId)
        || !IsCanonicalApplicationIdentity(Request.ReasonId)
        || !IsCanonicalApplicationIdentity(Request.CallerId)
        || !IsCanonicalOrNone(Request.ReturnModeId);
    const bool bInvalidRevision =
        !IsRevisionToken(Request.CatalogRevision)
        || !IsRevisionToken(Request.SourceModeRevision)
        || !IsRevisionToken(Request.TargetModeRevision)
        || !IsRevisionToken(Request.SessionRevision)
        || !IsRevisionToken(Request.ProfileRevision)
        || !IsRevisionToken(Request.WorldRevision)
        || !IsRevisionToken(Request.RequestRevision);
    const bool bInvalidDeadline =
        !FMath::IsFinite(Request.DeadlineSeconds)
        || Request.DeadlineSeconds <= 0.0;
    return !bInvalidIdentity && !bInvalidRevision && !bInvalidDeadline;
}

bool USharApplicationModeCoordinator::IsTerminalState(
    const ESharApplicationTransitionState State
)
{
    return State == ESharApplicationTransitionState::Success
        || State == ESharApplicationTransitionState::Failed
        || State == ESharApplicationTransitionState::Cancelled
        || State == ESharApplicationTransitionState::Superseded
        || State == ESharApplicationTransitionState::Recovered;
}

FSharApplicationTransitionSnapshot*
USharApplicationModeCoordinator::FindTransition(const FName& RequestId)
{
    return Algo::FindByPredicate(
        Transitions,
        [&RequestId](const FSharApplicationTransitionSnapshot& Snapshot)
        {
            return Snapshot.Request.RequestId == RequestId;
        }
    );
}

const FSharApplicationTransitionSnapshot*
USharApplicationModeCoordinator::FindTransition(const FName& RequestId) const
{
    return Algo::FindByPredicate(
        Transitions,
        [&RequestId](const FSharApplicationTransitionSnapshot& Snapshot)
        {
            return Snapshot.Request.RequestId == RequestId;
        }
    );
}

bool USharApplicationModeCoordinator::Outranks(
    const FSharApplicationTransitionSnapshot& Left,
    const FSharApplicationTransitionSnapshot& Right
)
{
    const auto LeftPriority = static_cast<uint8>(Left.Request.Priority);
    const auto RightPriority = static_cast<uint8>(Right.Request.Priority);
    if (LeftPriority != RightPriority)
    {
        return LeftPriority > RightPriority;
    }
    return Left.Request.RequestId.LexicalLess(Right.Request.RequestId);
}

bool USharApplicationModeCoordinator::IsHead(
    const FSharApplicationTransitionSnapshot& Snapshot
) const
{
    if (Snapshot.bReleased
        || Snapshot.State != ESharApplicationTransitionState::Pending)
    {
        return false;
    }
    const bool bHasOutrankingTransition = Algo::AnyOf(
        Transitions,
        [&Snapshot](const FSharApplicationTransitionSnapshot& Other)
        {
            const bool bComparable =
                !Other.bReleased
                && Other.State == ESharApplicationTransitionState::Pending
                && Other.Request.RequestId != Snapshot.Request.RequestId;
            return bComparable && Outranks(Other, Snapshot);
        }
    );
    return !bHasOutrankingTransition;
}

bool USharApplicationModeCoordinator::HasConflictingActiveTransition(
    const FName& RequestId
) const
{
    return Algo::AnyOf(
        Transitions,
        [&RequestId](const FSharApplicationTransitionSnapshot& Snapshot)
        {
            return !Snapshot.bReleased
                && Snapshot.Request.RequestId != RequestId
                && Snapshot.State != ESharApplicationTransitionState::Pending
                && !IsTerminalState(Snapshot.State);
        }
    );
}

bool USharApplicationModeCoordinator::Configure(
    USharApplicationModeCatalogSubsystem* InCatalog,
    const FSharApplicationModeObservation& InitialObservation
)
{
    const bool bInvalid =
        InCatalog == nullptr
        || !InCatalog->IsActive()
        || !IsCanonicalApplicationIdentity(InitialObservation.ActiveModeId)
        || !IsRevisionToken(InitialObservation.ActiveModeRevision)
        || !IsRevisionToken(InitialObservation.WorldRevision)
        || !IsRevisionToken(InitialObservation.ProfileRevision)
        || !IsRevisionToken(InitialObservation.SessionRevision)
        || InCatalog->FindMode(InitialObservation.ActiveModeId) == nullptr;
    if (bInvalid)
    {
        return false;
    }
    Catalog = InCatalog;
    Observation = InitialObservation;
    Observation.ActiveTransitionId = FName();
    Transitions.Reset();
    NextInsertionSequence = 0;
    return true;
}

ESharApplicationOperationResult
USharApplicationModeCoordinator::ClassifySubmission(
    const FSharApplicationModeRequest& Request
) const
{
    if (Catalog == nullptr)
    {
        return ESharApplicationOperationResult::CatalogMissing;
    }
    if (!Catalog->IsActive())
    {
        return ESharApplicationOperationResult::CatalogInactive;
    }
    if (!IsValidRequest(Request))
    {
        return ESharApplicationOperationResult::InvalidRequest;
    }
    if (Request.CatalogRevision != Catalog->GetCatalogRevision()
        || Request.SourceModeRevision != Observation.ActiveModeRevision)
    {
        return ESharApplicationOperationResult::StaleRevision;
    }
    if (Request.SourceModeId != Observation.ActiveModeId)
    {
        return ESharApplicationOperationResult::InvalidRequest;
    }
    if (Catalog->FindMode(Request.TargetModeId) == nullptr)
    {
        return ESharApplicationOperationResult::ModeMissing;
    }
    if (!Catalog->IsTransitionAllowed(
        Request.SourceModeId,
        Request.TargetModeId
    ))
    {
        return ESharApplicationOperationResult::TransitionNotAllowed;
    }
    return FindTransition(Request.RequestId) == nullptr
        ? ESharApplicationOperationResult::Accepted
        : ESharApplicationOperationResult::DuplicateRequest;
}

int32 USharApplicationModeCoordinator::CountPendingTransitions() const
{
    int32 PendingCount = 0;
    for (const FSharApplicationTransitionSnapshot& Snapshot : Transitions)
    {
        PendingCount += !Snapshot.bReleased
                && Snapshot.State == ESharApplicationTransitionState::Pending
            ? 1
            : 0;
    }
    return PendingCount;
}

ESharApplicationOperationResult USharApplicationModeCoordinator::Submit(
    const FSharApplicationModeRequest& Request
)
{
    const ESharApplicationOperationResult Classification =
        ClassifySubmission(Request);
    if (Classification != ESharApplicationOperationResult::Accepted)
    {
        return Classification;
    }
    if (CountPendingTransitions() >= MaximumPendingTransitions)
    {
        return ESharApplicationOperationResult::ConflictingTransition;
    }
    FSharApplicationTransitionSnapshot Snapshot;
    Snapshot.Request = Request;
    Snapshot.InsertionSequence = NextInsertionSequence;
    ++NextInsertionSequence;
    Transitions.Add(Snapshot);
    return ESharApplicationOperationResult::Accepted;
}

ESharApplicationOperationResult USharApplicationModeCoordinator::Begin(
    const FName& RequestId
)
{
    FSharApplicationTransitionSnapshot* Snapshot = FindTransition(RequestId);
    if (Snapshot == nullptr)
    {
        return ESharApplicationOperationResult::NotFound;
    }
    if (Snapshot->bReleased)
    {
        return ESharApplicationOperationResult::Released;
    }
    if (Snapshot->State != ESharApplicationTransitionState::Pending)
    {
        return ESharApplicationOperationResult::InvalidState;
    }
    if (!IsHead(*Snapshot))
    {
        return ESharApplicationOperationResult::NotHead;
    }
    if (HasConflictingActiveTransition(RequestId))
    {
        return ESharApplicationOperationResult::ConflictingTransition;
    }
    const bool bStaleSource =
        Snapshot->Request.SourceModeId != Observation.ActiveModeId
        || Snapshot->Request.SourceModeRevision
            != Observation.ActiveModeRevision
        || Snapshot->Request.CatalogRevision
            != Catalog->GetCatalogRevision();
    if (bStaleSource)
    {
        return ESharApplicationOperationResult::StaleRevision;
    }
    Snapshot->State = ESharApplicationTransitionState::Validating;
    Snapshot->State = ESharApplicationTransitionState::Preparing;
    Observation.ActiveTransitionId = Snapshot->Request.RequestId;
    return ESharApplicationOperationResult::Accepted;
}

int32 USharApplicationModeCoordinator::GetQueuePosition(
    const FName& RequestId
) const
{
    const FSharApplicationTransitionSnapshot* Snapshot =
        FindTransition(RequestId);
    if (Snapshot == nullptr || Snapshot->bReleased
        || Snapshot->State != ESharApplicationTransitionState::Pending)
    {
        return 0;
    }
    int32 Position = 1;
    for (const FSharApplicationTransitionSnapshot& Other : Transitions)
    {
        const bool bComparable =
            !Other.bReleased
            && Other.State == ESharApplicationTransitionState::Pending
            && Other.Request.RequestId != Snapshot->Request.RequestId;
        Position += bComparable && Outranks(Other, *Snapshot) ? 1 : 0;
    }
    return Position;
}
