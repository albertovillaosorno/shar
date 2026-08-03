// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT
//
// Boundary-Contract:
// - Owns:
//   - Shar frontend flow evidence composition module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Shar frontend flow evidence composition module.
// - Description:
//   - Implements the declared composition module responsibility for project.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Shar frontend flow evidence composition module.

#include "UI/SharFrontendFlowSubsystem.h"

#include "Algo/AnyOf.h"
#include "Content/SharPrimaryContentDefinition.h"
#include "UI/SharFrontendCatalogDefinition.h"
#include "UI/SharFrontendFlowContracts.h"

static bool IsRevisionToken(const FString& Value)
{
    return !Value.IsEmpty() && Value.Contains(TEXT(":"));
}


static bool ContainsRequirement(
    const TArray<ESharFrontendReadinessKind>& Requirements,
    const ESharFrontendReadinessKind Kind
)
{
    return Algo::AnyOf(
        Requirements,
        [Kind](const ESharFrontendReadinessKind Candidate)
        {
            return Candidate == Kind;
        }
    );
}

static bool IsTerminalState(const ESharFrontendTransitionState State)
{
    return State == ESharFrontendTransitionState::Success
        || State == ESharFrontendTransitionState::Failed
        || State == ESharFrontendTransitionState::Cancelled
        || State == ESharFrontendTransitionState::Superseded;
}
ESharFrontendOperationResult USharFrontendFlowSubsystem::ValidateEvidence(
    const FSharFrontendTransitionSnapshot& Transition,
    const FSharFrontendReadinessEvidence& Evidence
) const
{
    if (!USharPrimaryContentDefinition::IsCanonicalIdentifier(
            Evidence.EvidenceId
        )
        || !IsRevisionToken(Evidence.CatalogRevision)
        || !IsRevisionToken(Evidence.RequestRevision)
        || !IsRevisionToken(Evidence.DestinationScreenRevision)
        || !IsRevisionToken(Evidence.EvidenceRevision))
    {
        return ESharFrontendOperationResult::InvalidRequest;
    }
    if (Evidence.RequestId != Transition.Request.RequestId
        || Evidence.CatalogRevision != Transition.Request.CatalogRevision
        || Evidence.RequestRevision != Transition.Request.RequestRevision
        || Evidence.DestinationScreenRevision
            != Transition.CandidateScreenRevision)
    {
        return ESharFrontendOperationResult::StaleRevision;
    }
    if (HasEvidence(Transition, Evidence.Kind))
    {
        return ESharFrontendOperationResult::DuplicateEvidence;
    }

    const FSharFrontendScreenDefinition* Destination =
        GetDestinationDefinition(Transition);
    if (Destination == nullptr)
    {
        return ESharFrontendOperationResult::DestinationMissing;
    }
    const TArray<ESharFrontendReadinessKind>* Requirements = nullptr;
    if (Transition.State
        == ESharFrontendTransitionState::VerifyingReadiness)
    {
        Requirements = &Destination->PreCommitRequirements;
    }
    else if (Transition.State
             == ESharFrontendTransitionState::VerifyingTarget)
    {
        Requirements = &Destination->PostCommitRequirements;
    }
    else
    {
        return ESharFrontendOperationResult::InvalidState;
    }
    return ContainsRequirement(*Requirements, Evidence.Kind)
        ? ESharFrontendOperationResult::Accepted
        : ESharFrontendOperationResult::EvidenceNotRequired;
}

ESharFrontendOperationResult
USharFrontendFlowSubsystem::ValidateEvidenceSubmission(
    const FSharFrontendTransitionSnapshot* Transition,
    const FSharFrontendReadinessEvidence& Evidence
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
    if (!IsHead(Evidence.RequestId))
    {
        return ESharFrontendOperationResult::NotHead;
    }
    return ValidateEvidence(*Transition, Evidence);
}

ESharFrontendOperationResult USharFrontendFlowSubsystem::ApplyAcceptedEvidence(
    FSharFrontendTransitionSnapshot& Transition,
    const FSharFrontendScreenDefinition& Destination,
    const FSharFrontendReadinessEvidence& Evidence
)
{
    const bool bUnacceptableDegraded =
        Evidence.Status == ESharFrontendEvidenceStatus::Degraded
        && !Destination.bAllowDegradedReadiness;
    if (Evidence.Status == ESharFrontendEvidenceStatus::Failed
        || bUnacceptableDegraded)
    {
        if (Transition.bCommitted)
        {
            RestorePriorObservation(Transition);
        }
        CompleteTerminal(
            Transition,
            ESharFrontendTerminalResult::Failed,
            ESharFrontendTransitionState::Failed
        );
        return ESharFrontendOperationResult::EvidenceFailed;
    }

    Transition.Evidence.Add(Evidence);
    if (Transition.State
        == ESharFrontendTransitionState::VerifyingReadiness)
    {
        if (RequirementsSatisfied(
                Transition,
                Destination.PreCommitRequirements
            ))
        {
            Transition.State = ESharFrontendTransitionState::ReadyToCommit;
        }
        return ESharFrontendOperationResult::Accepted;
    }
    if (Evidence.Kind == ESharFrontendReadinessKind::Focus)
    {
        Observation.StableFocusTargetId = Evidence.EvidenceId;
    }
    if (RequirementsSatisfied(
            Transition,
            Destination.PostCommitRequirements
        ))
    {
        CompleteSuccess(Transition);
    }
    return ESharFrontendOperationResult::Accepted;
}

ESharFrontendOperationResult USharFrontendFlowSubsystem::AcceptEvidence(
    const FSharFrontendReadinessEvidence& Evidence
)
{
    FSharFrontendTransitionSnapshot* Transition =
        FindMutableTransition(Evidence.RequestId);
    const ESharFrontendOperationResult Submission =
        ValidateEvidenceSubmission(Transition, Evidence);
    if (Submission != ESharFrontendOperationResult::Accepted)
    {
        return Submission;
    }
    const FSharFrontendScreenDefinition* Destination =
        GetDestinationDefinition(*Transition);
    if (Destination == nullptr)
    {
        return ESharFrontendOperationResult::DestinationMissing;
    }
    return ApplyAcceptedEvidence(*Transition, *Destination, Evidence);
}
