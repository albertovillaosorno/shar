// File: SharFrontendFlowLifecycle.cpp
// Path: src/uproject/Source/SharUI/Private/UI/SharFrontendFlowLifecycle.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: frontend candidate commit, history and modal mutation, rollback, terminal publication, and explicit release only.
// Specification: docs/technical/unreal/frontend-screen-flow-and-settings-runtime.md

#include "UI/SharFrontendFlowSubsystem.h"

#include "UI/SharFrontendCatalogDefinition.h"
#include "UI/SharFrontendFlowContracts.h"

static bool IsTerminalState(const ESharFrontendTransitionState State)
{
    return State == ESharFrontendTransitionState::Success
        || State == ESharFrontendTransitionState::Failed
        || State == ESharFrontendTransitionState::Cancelled
        || State == ESharFrontendTransitionState::Superseded;
}
ESharFrontendOperationResult
USharFrontendFlowSubsystem::ValidateCommitSubmission(
    const FSharFrontendTransitionSnapshot* Transition,
    const FSharFrontendScreenDefinition* Destination,
    const FName& RequestId
) const
{
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
    if (Transition->State != ESharFrontendTransitionState::ReadyToCommit)
    {
        return ESharFrontendOperationResult::EvidenceMissing;
    }
    if (Destination == nullptr)
    {
        return ESharFrontendOperationResult::DestinationMissing;
    }
    return RequirementsSatisfied(
        *Transition,
        Destination->PreCommitRequirements
    )
        ? ESharFrontendOperationResult::Accepted
        : ESharFrontendOperationResult::EvidenceMissing;
}

ESharFrontendOperationResult USharFrontendFlowSubsystem::ApplyPrimaryHistory(
    const FSharFrontendNavigationRequest& Request,
    const FName& DestinationScreenId
)
{
    switch (Request.HistoryPolicy)
    {
        case ESharFrontendHistoryPolicy::Push:
            Observation.PrimaryHistory.Add(Observation.ActivePrimaryScreenId);
            break;
        case ESharFrontendHistoryPolicy::Replace:
            break;
        case ESharFrontendHistoryPolicy::Pop:
            if (Observation.PrimaryHistory.IsEmpty()
                || Observation.PrimaryHistory.Last() != DestinationScreenId)
            {
                return ESharFrontendOperationResult::HistoryEmpty;
            }
            Observation.PrimaryHistory.Pop(EAllowShrinking::No);
            break;
        case ESharFrontendHistoryPolicy::Preserve:
        default:
            return ESharFrontendOperationResult::InvalidRequest;
    }
    Observation.ActivePrimaryScreenId = DestinationScreenId;
    return ESharFrontendOperationResult::Accepted;
}

ESharFrontendOperationResult USharFrontendFlowSubsystem::ApplyNavigationCommit(
    FSharFrontendTransitionSnapshot& Transition,
    const FSharFrontendScreenDefinition& Destination
)
{
    if (Destination.Layer == ESharFrontendLayer::Modal)
    {
        Observation.ActiveModalScreenId = Destination.ScreenId;
        return ESharFrontendOperationResult::Accepted;
    }
    if (!Observation.ActiveModalScreenId.IsNone())
    {
        Observation.ActiveModalScreenId = NAME_None;
        return ESharFrontendOperationResult::Accepted;
    }
    return ApplyPrimaryHistory(Transition.Request, Destination.ScreenId);
}

ESharFrontendOperationResult USharFrontendFlowSubsystem::Commit(
    const FName& RequestId
)
{
    FSharFrontendTransitionSnapshot* Transition =
        FindMutableTransition(RequestId);
    const FSharFrontendScreenDefinition* Destination =
        Transition == nullptr ? nullptr : GetDestinationDefinition(*Transition);
    const ESharFrontendOperationResult Validation =
        ValidateCommitSubmission(Transition, Destination, RequestId);
    if (Validation != ESharFrontendOperationResult::Accepted)
    {
        return Validation;
    }
    const ESharFrontendOperationResult Mutation =
        ApplyNavigationCommit(*Transition, *Destination);
    if (Mutation != ESharFrontendOperationResult::Accepted)
    {
        return Mutation;
    }

    Observation.FlowRevision = MakeNextFlowRevision();
    Observation.ActiveScreenRevision = Transition->CandidateScreenRevision;
    Observation.ActiveTransitionId = RequestId;
    if (Destination->PostCommitRequirements.Contains(
            ESharFrontendReadinessKind::Focus
        ))
    {
        Observation.StableFocusTargetId = NAME_None;
    }
    Transition->bCommitted = true;
    Transition->State = ESharFrontendTransitionState::Committed;
    if (Destination->PostCommitRequirements.IsEmpty())
    {
        CompleteSuccess(*Transition);
    }
    else
    {
        Transition->State = ESharFrontendTransitionState::VerifyingTarget;
    }
    return ESharFrontendOperationResult::Accepted;
}

ESharFrontendOperationResult USharFrontendFlowSubsystem::Resolve(
    const FSharFrontendTransitionResolution& Resolution
)
{
    FSharFrontendTransitionSnapshot* Transition =
        FindMutableTransition(Resolution.RequestId);
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
    if (!IsHead(Resolution.RequestId))
    {
        return ESharFrontendOperationResult::NotHead;
    }
    if (Resolution.CatalogRevision
            != Transition->Request.CatalogRevision
        || Resolution.RequestRevision
            != Transition->Request.RequestRevision)
    {
        return ESharFrontendOperationResult::StaleRevision;
    }
    if (Transition->bCommitted)
    {
        RestorePriorObservation(*Transition);
    }

    switch (Resolution.Command)
    {
        case ESharFrontendResolutionCommand::Cancel:
            CompleteTerminal(
                *Transition,
                ESharFrontendTerminalResult::Cancelled,
                ESharFrontendTransitionState::Cancelled
            );
            break;
        case ESharFrontendResolutionCommand::Supersede:
            CompleteTerminal(
                *Transition,
                ESharFrontendTerminalResult::Superseded,
                ESharFrontendTransitionState::Superseded
            );
            break;
        case ESharFrontendResolutionCommand::Fail:
        default:
            CompleteTerminal(
                *Transition,
                ESharFrontendTerminalResult::Failed,
                ESharFrontendTransitionState::Failed
            );
            break;
    }
    return ESharFrontendOperationResult::Accepted;
}

ESharFrontendOperationResult USharFrontendFlowSubsystem::Release(
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
    if (!IsTerminalState(Transition->State))
    {
        return ESharFrontendOperationResult::InvalidState;
    }
    Transition->bReleased = true;
    Transition->State = ESharFrontendTransitionState::Released;
    SortQueue();
    return ESharFrontendOperationResult::Accepted;
}
