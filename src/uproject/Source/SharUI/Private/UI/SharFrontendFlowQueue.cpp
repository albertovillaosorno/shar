// File: SharFrontendFlowQueue.cpp
// Path: src/uproject/Source/SharUI/Private/UI/SharFrontendFlowQueue.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: frontend request validation, deterministic queue ordering, and head preparation only.
// Specification: docs/technical/unreal/frontend-screen-flow-and-settings-runtime.md

#include "UI/SharFrontendFlowSubsystem.h"

#include "Content/SharPrimaryContentDefinition.h"
#include "Templates/UnrealTemplate.h"
#include "UI/SharFrontendCatalogDefinition.h"
#include "UI/SharFrontendFlowContracts.h"

static bool IsRevisionToken(const FString& Value)
{
    return !Value.IsEmpty() && Value.Contains(TEXT(":"));
}

static bool IsCanonicalIdentity(const FName& Value)
{
    return USharPrimaryContentDefinition::IsCanonicalIdentifier(Value);
}

static bool IsTerminalState(const ESharFrontendTransitionState State)
{
    return State == ESharFrontendTransitionState::Success
        || State == ESharFrontendTransitionState::Failed
        || State == ESharFrontendTransitionState::Cancelled
        || State == ESharFrontendTransitionState::Superseded;
}
ESharFrontendOperationResult
USharFrontendFlowSubsystem::ValidateRequestEnvelope(
    const FSharFrontendNavigationRequest& Request
) const
{
    if (!bConfigured || Catalog == nullptr)
    {
        return ESharFrontendOperationResult::NotConfigured;
    }
    if (!Catalog->IsActive())
    {
        return ESharFrontendOperationResult::CatalogInactive;
    }
    const bool bInvalidIdentity =
        !IsCanonicalIdentity(Request.RequestId)
        || !IsCanonicalIdentity(Request.SourceScreenId)
        || !IsCanonicalIdentity(Request.DestinationScreenId)
        || !IsCanonicalIdentity(Request.CallerId)
        || !IsCanonicalIdentity(Request.LocalPlayerId);
    const bool bInvalidRevision =
        !IsRevisionToken(Request.CatalogRevision)
        || !IsRevisionToken(Request.FlowRevision)
        || !IsRevisionToken(Request.SourceScreenRevision)
        || !IsRevisionToken(Request.RequestRevision);
    if (bInvalidIdentity || bInvalidRevision)
    {
        return ESharFrontendOperationResult::InvalidRequest;
    }
    if (Request.CatalogRevision != Catalog->GetCatalogRevision()
        || Request.FlowRevision != Observation.FlowRevision
        || Request.SourceScreenRevision != Observation.ActiveScreenRevision)
    {
        return ESharFrontendOperationResult::StaleRevision;
    }
    return Request.SourceScreenId == GetEffectiveScreenId()
        ? ESharFrontendOperationResult::Accepted
        : ESharFrontendOperationResult::SourceMismatch;
}

ESharFrontendOperationResult USharFrontendFlowSubsystem::ValidateRequestRoute(
    const FSharFrontendNavigationRequest& Request
) const
{
    const FSharFrontendScreenDefinition* Source =
        Catalog->FindScreen(Request.SourceScreenId);
    const FSharFrontendScreenDefinition* Destination =
        Catalog->FindScreen(Request.DestinationScreenId);
    if (Destination == nullptr)
    {
        return ESharFrontendOperationResult::DestinationMissing;
    }
    if (Source == nullptr
        || !Source->AllowedDestinationScreenIds.Contains(
            Request.DestinationScreenId
        ))
    {
        return ESharFrontendOperationResult::TransitionNotAllowed;
    }
    return Destination->Layer == ESharFrontendLayer::Notification
        ? ESharFrontendOperationResult::TransitionNotAllowed
        : ESharFrontendOperationResult::Accepted;
}

ESharFrontendOperationResult
USharFrontendFlowSubsystem::ValidateModalHistory(
    const FSharFrontendNavigationRequest& Request,
    const FSharFrontendScreenDefinition& Destination
) const
{
    if (!Observation.ActiveModalScreenId.IsNone())
    {
        const bool bDismissesModal =
            Request.DestinationScreenId == Observation.ActivePrimaryScreenId
            && Request.HistoryPolicy == ESharFrontendHistoryPolicy::Pop;
        return bDismissesModal
            ? ESharFrontendOperationResult::Accepted
            : ESharFrontendOperationResult::TransitionNotAllowed;
    }
    if (Destination.Layer != ESharFrontendLayer::Modal)
    {
        return ESharFrontendOperationResult::Accepted;
    }
    return Request.HistoryPolicy == ESharFrontendHistoryPolicy::Preserve
        ? ESharFrontendOperationResult::Accepted
        : ESharFrontendOperationResult::TransitionNotAllowed;
}

ESharFrontendOperationResult
USharFrontendFlowSubsystem::ValidatePrimaryHistory(
    const FSharFrontendNavigationRequest& Request
) const
{
    if (Request.HistoryPolicy == ESharFrontendHistoryPolicy::Preserve)
    {
        return ESharFrontendOperationResult::TransitionNotAllowed;
    }
    if (Request.HistoryPolicy != ESharFrontendHistoryPolicy::Pop)
    {
        return ESharFrontendOperationResult::Accepted;
    }
    if (Observation.PrimaryHistory.IsEmpty())
    {
        return ESharFrontendOperationResult::HistoryEmpty;
    }
    return Observation.PrimaryHistory.Last() == Request.DestinationScreenId
        ? ESharFrontendOperationResult::Accepted
        : ESharFrontendOperationResult::TransitionNotAllowed;
}

ESharFrontendOperationResult
USharFrontendFlowSubsystem::ValidateRequestHistory(
    const FSharFrontendNavigationRequest& Request
) const
{
    const FSharFrontendScreenDefinition* Destination =
        Catalog->FindScreen(Request.DestinationScreenId);
    if (Destination == nullptr)
    {
        return ESharFrontendOperationResult::DestinationMissing;
    }
    const ESharFrontendOperationResult Modal =
        ValidateModalHistory(Request, *Destination);
    if (Modal != ESharFrontendOperationResult::Accepted)
    {
        return Modal;
    }
    const bool bModalOpenOrDismiss =
        Destination->Layer == ESharFrontendLayer::Modal
        || !Observation.ActiveModalScreenId.IsNone();
    return bModalOpenOrDismiss
        ? ESharFrontendOperationResult::Accepted
        : ValidatePrimaryHistory(Request);
}

ESharFrontendOperationResult USharFrontendFlowSubsystem::ValidateRequest(
    const FSharFrontendNavigationRequest& Request
) const
{
    const ESharFrontendOperationResult Envelope =
        ValidateRequestEnvelope(Request);
    if (Envelope != ESharFrontendOperationResult::Accepted)
    {
        return Envelope;
    }
    const ESharFrontendOperationResult Route = ValidateRequestRoute(Request);
    return Route == ESharFrontendOperationResult::Accepted
        ? ValidateRequestHistory(Request)
        : Route;
}

ESharFrontendOperationResult USharFrontendFlowSubsystem::Submit(
    const FSharFrontendNavigationRequest& Request
)
{
    const ESharFrontendOperationResult Validation = ValidateRequest(Request);
    if (Validation != ESharFrontendOperationResult::Accepted)
    {
        return Validation;
    }
    if (FindTransition(Request.RequestId) != nullptr)
    {
        return ESharFrontendOperationResult::DuplicateRequest;
    }
    for (const FSharFrontendTransitionSnapshot& Transition : Transitions)
    {
        if (Transition.State != ESharFrontendTransitionState::Pending
            && Transition.State != ESharFrontendTransitionState::Released
            && !IsTerminalState(Transition.State))
        {
            return ESharFrontendOperationResult::ConflictingTransition;
        }
    }

    FSharFrontendTransitionSnapshot Snapshot;
    Snapshot.Request = Request;
    Snapshot.State = ESharFrontendTransitionState::Pending;
    Snapshot.InsertionSequence = NextInsertionSequence++;
    Transitions.Add(MoveTemp(Snapshot));
    SortQueue();
    return ESharFrontendOperationResult::Accepted;
}

ESharFrontendOperationResult USharFrontendFlowSubsystem::Begin(
    const FName& RequestId
)
{
    FSharFrontendTransitionSnapshot* Transition =
        FindMutableTransition(RequestId);
    if (Transition == nullptr)
    {
        return ESharFrontendOperationResult::NotFound;
    }
    if (Transition->State == ESharFrontendTransitionState::Released)
    {
        return ESharFrontendOperationResult::Released;
    }
    if (IsTerminalState(Transition->State))
    {
        return ESharFrontendOperationResult::AlreadyTerminal;
    }
    if (!IsHead(RequestId))
    {
        return ESharFrontendOperationResult::NotHead;
    }
    if (Transition->State != ESharFrontendTransitionState::Pending)
    {
        return ESharFrontendOperationResult::InvalidState;
    }

    const ESharFrontendOperationResult Validation =
        ValidateRequest(Transition->Request);
    if (Validation != ESharFrontendOperationResult::Accepted)
    {
        return Validation;
    }
    const FSharFrontendScreenDefinition* Destination =
        GetDestinationDefinition(*Transition);
    if (Destination == nullptr)
    {
        return ESharFrontendOperationResult::DestinationMissing;
    }

    Transition->PriorObservation = Observation;
    Transition->CandidateScreenRevision = MakeNextScreenRevision();
    Transition->State = ESharFrontendTransitionState::Preparing;
    Observation.ActiveTransitionId = RequestId;
    Transition->State = ESharFrontendTransitionState::VerifyingReadiness;
    if (Destination->PreCommitRequirements.IsEmpty())
    {
        Transition->State = ESharFrontendTransitionState::ReadyToCommit;
    }
    SortQueue();
    return ESharFrontendOperationResult::Accepted;
}
