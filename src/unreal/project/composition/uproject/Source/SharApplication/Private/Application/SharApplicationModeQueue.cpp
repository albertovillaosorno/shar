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
//   - Shar application mode queue composition module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Shar application mode queue composition module.
// - Description:
//   - Implements the declared composition module responsibility for project.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Shar application mode queue composition module.

#include "Application/SharApplicationModeCoordinator.h"

#include "Algo/AnyOf.h"
#include "Algo/Find.h"
#include "Application/SharApplicationModeCatalogSubsystem.h"
#include "Application/SharApplicationModeDefinition.h"
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

bool USharApplicationModeCoordinator::RequiresWorldAuthority(
    const USharApplicationModeDefinition& Mode
)
{
    return Mode.WorldPolicy == ESharApplicationWorldPolicy::Prepare
        || Mode.WorldPolicy == ESharApplicationWorldPolicy::Retain
        || Mode.WorldPolicy == ESharApplicationWorldPolicy::Own;
}

bool USharApplicationModeCoordinator::IsValidWorldAuthority(
    const FName& WorldId,
    const FString& WorldRevision,
    const USharApplicationModeDefinition& Mode
)
{
    if (RequiresWorldAuthority(Mode))
    {
        return IsCanonicalApplicationIdentity(WorldId)
            && IsRevisionToken(WorldRevision);
    }
    return WorldId.IsNone() && WorldRevision.IsEmpty();
}

bool USharApplicationModeCoordinator::RequiresSessionAuthority(
    const USharApplicationModeDefinition& Mode
)
{
    return Mode.SessionPolicy == ESharApplicationSessionPolicy::Prepare
        || Mode.SessionPolicy == ESharApplicationSessionPolicy::Retain
        || Mode.SessionPolicy == ESharApplicationSessionPolicy::Own;
}

bool USharApplicationModeCoordinator::IsValidSessionAuthority(
    const FString& SessionRevision,
    const USharApplicationModeDefinition& Mode
)
{
    return RequiresSessionAuthority(Mode)
        ? IsRevisionToken(SessionRevision)
        : SessionRevision.IsEmpty();
}

bool USharApplicationModeCoordinator::IsValidRequest(
    const FSharApplicationModeRequest& Request
)
{
    const bool bHasWorldAuthority = !Request.WorldId.IsNone();
    const bool bWorldPairValid = bHasWorldAuthority
        ? IsCanonicalApplicationIdentity(Request.WorldId)
            && IsRevisionToken(Request.WorldRevision)
        : Request.WorldRevision.IsEmpty();
    const bool bInvalidIdentity =
        !IsCanonicalApplicationIdentity(Request.RequestId)
        || !IsCanonicalApplicationIdentity(Request.SourceModeId)
        || !IsCanonicalApplicationIdentity(Request.TargetModeId)
        || !IsCanonicalApplicationIdentity(Request.ReasonId)
        || !IsCanonicalApplicationIdentity(Request.CallerId)
        || !IsCanonicalOrNone(Request.ReturnModeId)
        || !bWorldPairValid;
    const bool bSessionRevisionValid = Request.SessionRevision.IsEmpty()
        || IsRevisionToken(Request.SessionRevision);
    const bool bInvalidRevision =
        !IsRevisionToken(Request.CatalogRevision)
        || !IsRevisionToken(Request.SourceModeRevision)
        || !IsRevisionToken(Request.TargetModeRevision)
        || !bSessionRevisionValid
        || !IsRevisionToken(Request.ProfileRevision)
        || !IsRevisionToken(Request.RequestRevision);
    const bool bInvalidDeadline =
        !FMath::IsFinite(Request.DeadlineSeconds)
        || Request.DeadlineSeconds <= 0.0;
    return !bInvalidIdentity && !bInvalidRevision && !bInvalidDeadline;
}

bool USharApplicationModeCoordinator::IsValidInitialObservation(
    const FSharApplicationModeObservation& InitialObservation,
    const USharApplicationModeDefinition& ActiveMode
)
{
    const bool bInvalidCommon =
        InitialObservation.ActiveModeId != ActiveMode.CanonicalId
        || InitialObservation.ActiveModeRevision != ActiveMode.RevisionToken
        || !IsCanonicalApplicationIdentity(InitialObservation.ActiveModeId)
        || !IsRevisionToken(InitialObservation.ActiveModeRevision);
    if (bInvalidCommon)
    {
        return false;
    }
    if (!IsValidWorldAuthority(
        InitialObservation.WorldId,
        InitialObservation.WorldRevision,
        ActiveMode
    ) || !IsValidSessionAuthority(
        InitialObservation.SessionRevision,
        ActiveMode
    ))
    {
        return false;
    }
    if (ActiveMode.ModeKind == ESharApplicationModeKind::Entry)
    {
        return InitialObservation.ProfileRevision.IsEmpty();
    }
    return IsRevisionToken(InitialObservation.ProfileRevision);
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
    const USharApplicationModeDefinition* ActiveMode = InCatalog == nullptr
        ? nullptr
        : InCatalog->FindMode(InitialObservation.ActiveModeId);
    if (InCatalog == nullptr
        || !InCatalog->IsActive()
        || ActiveMode == nullptr
        || !IsValidInitialObservation(InitialObservation, *ActiveMode))
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
    const USharApplicationModeDefinition* Source =
        Catalog->FindMode(Request.SourceModeId);
    const USharApplicationModeDefinition* Target =
        Catalog->FindMode(Request.TargetModeId);
    if (Source == nullptr || Target == nullptr)
    {
        return ESharApplicationOperationResult::ModeMissing;
    }
    if (Request.SourceModeRevision != Source->RevisionToken
        || Request.TargetModeRevision != Target->RevisionToken)
    {
        return ESharApplicationOperationResult::StaleRevision;
    }
    if (!IsValidWorldAuthority(
        Request.WorldId,
        Request.WorldRevision,
        *Target
    ) || !IsValidSessionAuthority(Request.SessionRevision, *Target))
    {
        return ESharApplicationOperationResult::InvalidRequest;
    }
    const bool bSourceHasWorld = RequiresWorldAuthority(*Source);
    const bool bTargetKeepsWorld =
        Target->WorldPolicy == ESharApplicationWorldPolicy::Retain
        || Target->WorldPolicy == ESharApplicationWorldPolicy::Own;
    if (bSourceHasWorld && bTargetKeepsWorld
        && (Request.WorldId != Observation.WorldId
            || Request.WorldRevision != Observation.WorldRevision))
    {
        return ESharApplicationOperationResult::StaleRevision;
    }
    const bool bSourceHasSession = RequiresSessionAuthority(*Source);
    const bool bTargetKeepsSession =
        Target->SessionPolicy == ESharApplicationSessionPolicy::Retain
        || Target->SessionPolicy == ESharApplicationSessionPolicy::Own;
    if (bSourceHasSession && bTargetKeepsSession
        && Request.SessionRevision != Observation.SessionRevision)
    {
        return ESharApplicationOperationResult::StaleRevision;
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
