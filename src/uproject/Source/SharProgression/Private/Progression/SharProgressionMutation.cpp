// File: SharProgressionMutation.cpp
// Path: src/uproject/Source/SharProgression/Private/Progression/SharProgressionMutation.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: isolated candidate construction, correlated commit acceptance, failure, cancellation, and release only.
// Specification: docs/technical/unreal/progression-collectibles-and-cheats.md
// LARGE-FILE owner=SharProgression; reason=cohesive progression mutation and terminal lifecycle;
// split=extract save-port evidence adapter when SharSave integration becomes concrete;
// validation=validate.sh SharProgression plus Unreal automation; review=2027-01.

#include "Progression/SharProgressionSubsystem.h"

#include "Progression/SharProgressionCatalogDefinition.h"
#include "Progression/SharProgressionCatalogSubsystem.h"
#include "Progression/SharProgressionContracts.h"
#include "Progression/SharProgressionState.h"

static ESharProgressionMutationResult MapRewardApplyResult(
    const ESharRewardApplyResult Result
)
{
    switch (Result)
    {
    case ESharRewardApplyResult::Applied:
        return ESharProgressionMutationResult::Accepted;
    case ESharRewardApplyResult::AlreadyApplied:
        return ESharProgressionMutationResult::AlreadyApplied;
    case ESharRewardApplyResult::UnsupportedOperation:
        return ESharProgressionMutationResult::UnsupportedOperation;
    case ESharRewardApplyResult::InvalidRequest:
    default:
        return ESharProgressionMutationResult::InvalidRequest;
    }
}

static bool WouldOverflowPolicyBound(
    const USharProgressionState& State,
    const FSharRewardRequest& Operation,
    const FSharProgressionOperationDefinition& Definition
)
{
    if (Definition.ValuePolicy == ESharProgressionValuePolicy::SetOnce)
    {
        return false;
    }
    const int32 CurrentQuantity = State.GetQuantity(
        Operation.OperationId,
        Operation.TargetId
    );
    return CurrentQuantity > Definition.MaximumQuantity - Operation.Quantity;
}

ESharProgressionMutationResult USharProgressionSubsystem::BuildCandidate(
    FSharProgressionMutationSnapshot& Mutation
) const
{
    const USharProgressionCatalogDefinition* Catalog =
        CatalogSubsystem->FindCatalog(Mutation.Request.CatalogId);
    if (Catalog == nullptr)
    {
        return ESharProgressionMutationResult::CatalogDefinitionMissing;
    }
    auto* CandidateState = NewObject<USharProgressionState>();
    if (!CandidateState->InitializeSnapshot(
        ActiveSnapshot.Values,
        ActiveSnapshot.AppliedPermanentTransactions
    ))
    {
        return ESharProgressionMutationResult::InvalidState;
    }
    for (const FSharRewardRequest& Operation : Mutation.Request.Operations)
    {
        const FSharProgressionOperationDefinition* Definition =
            Catalog->FindOperation(Operation.OperationId);
        if (Definition == nullptr)
        {
            return ESharProgressionMutationResult::UnsupportedOperation;
        }
        if (WouldOverflowPolicyBound(*CandidateState, Operation, *Definition))
        {
            return ESharProgressionMutationResult::QuantityOverflow;
        }
        const ESharProgressionMutationResult ApplyResult =
            MapRewardApplyResult(CandidateState->ApplyReward(Operation));
        if (ApplyResult != ESharProgressionMutationResult::Accepted)
        {
            return ApplyResult;
        }
    }
    Mutation.CandidateSnapshot = ActiveSnapshot;
    Mutation.CandidateSnapshot.SnapshotRevision =
        Mutation.Request.MutationRevision;
    Mutation.CandidateSnapshot.Values = CandidateState->GetValues();
    Mutation.CandidateSnapshot.AppliedPermanentTransactions =
        CandidateState->GetAppliedTransactions();
    return ValidateSnapshot(Mutation.CandidateSnapshot)
        ? ESharProgressionMutationResult::Accepted
        : ESharProgressionMutationResult::InvalidState;
}

ESharProgressionMutationResult USharProgressionSubsystem::Prepare(
    const FName& MutationId
)
{
    FSharProgressionMutationSnapshot* Mutation = FindMutation(MutationId);
    if (Mutation == nullptr)
    {
        return ESharProgressionMutationResult::NotFound;
    }
    if (Mutation->bReleased)
    {
        return ESharProgressionMutationResult::Released;
    }
    if (Mutation->State != ESharProgressionMutationState::Preparing)
    {
        return IsTerminalState(Mutation->State)
            ? ESharProgressionMutationResult::AlreadyTerminal
            : ESharProgressionMutationResult::InvalidState;
    }
    const ESharProgressionMutationResult Result = BuildCandidate(*Mutation);
    if (Result != ESharProgressionMutationResult::Accepted)
    {
        return Result;
    }
    Mutation->State = ESharProgressionMutationState::AwaitingCommit;
    return ESharProgressionMutationResult::Accepted;
}

bool USharProgressionSubsystem::CommitEvidenceMatches(
    const FSharProgressionMutationSnapshot& Mutation,
    const FSharProgressionCommitEvidence& Evidence
)
{
    return Mutation.Request.MutationId == Evidence.MutationId
        && ProfileIdentitiesMatch(Mutation.Request.Profile, Evidence.Profile)
        && Mutation.Request.ExpectedCatalogRevision
            == Evidence.CatalogRevision
        && Mutation.Request.ExpectedSaveRevision
            == Evidence.ExpectedSaveRevision
        && Mutation.Request.ExpectedSnapshotRevision
            == Evidence.ExpectedSnapshotRevision
        && Mutation.Request.MutationRevision == Evidence.MutationRevision;
}

ESharProgressionMutationResult USharProgressionSubsystem::PublishTerminal(
    FSharProgressionMutationSnapshot& Mutation,
    const ESharProgressionMutationState State,
    const ESharProgressionTerminalResult Result
)
{
    if (Mutation.bReleased)
    {
        return ESharProgressionMutationResult::Released;
    }
    if (IsTerminalState(Mutation.State))
    {
        return ESharProgressionMutationResult::AlreadyTerminal;
    }
    Mutation.State = State;
    Mutation.TerminalResult = Result;
    return ESharProgressionMutationResult::Accepted;
}

ESharProgressionMutationResult
USharProgressionSubsystem::AcceptCommitEvidence(
    const FSharProgressionCommitEvidence& Evidence
)
{
    FSharProgressionMutationSnapshot* Mutation =
        FindMutation(Evidence.MutationId);
    if (Mutation == nullptr)
    {
        return ESharProgressionMutationResult::NotFound;
    }
    if (Mutation->bReleased)
    {
        return ESharProgressionMutationResult::Released;
    }
    if (IsTerminalState(Mutation->State))
    {
        return ESharProgressionMutationResult::AlreadyTerminal;
    }
    if (Mutation->State != ESharProgressionMutationState::AwaitingCommit)
    {
        return ESharProgressionMutationResult::InvalidState;
    }
    const bool bStale =
        !CommitEvidenceMatches(*Mutation, Evidence)
        || !IsRevisionToken(Evidence.ResultingSaveRevision)
        || !IsRevisionToken(Evidence.ResultingSnapshotRevision)
        || Evidence.ResultingSnapshotRevision
            != Mutation->CandidateSnapshot.SnapshotRevision
        || Evidence.ResultingSnapshotRevision
            != Mutation->Request.MutationRevision;
    if (bStale)
    {
        return ESharProgressionMutationResult::StaleRevision;
    }
    Mutation->CandidateSnapshot.SaveRevision = Evidence.ResultingSaveRevision;
    ActiveSnapshot = Mutation->CandidateSnapshot;
    return PublishTerminal(
        *Mutation,
        ESharProgressionMutationState::Completed,
        ESharProgressionTerminalResult::Success
    );
}

ESharProgressionMutationResult USharProgressionSubsystem::Resolve(
    const FSharProgressionMutationResolution& Resolution
)
{
    FSharProgressionMutationSnapshot* Mutation =
        FindMutation(Resolution.MutationId);
    if (Mutation == nullptr)
    {
        return ESharProgressionMutationResult::NotFound;
    }
    if (Mutation->bReleased)
    {
        return ESharProgressionMutationResult::Released;
    }
    if (IsTerminalState(Mutation->State))
    {
        return ESharProgressionMutationResult::AlreadyTerminal;
    }
    if (Resolution.ProfileRevision
            != Mutation->Request.Profile.ProfileRevision
        || Resolution.MutationRevision != Mutation->Request.MutationRevision)
    {
        return ESharProgressionMutationResult::StaleRevision;
    }
    switch (Resolution.Command)
    {
    case ESharProgressionResolutionCommand::Fail:
        return PublishTerminal(
            *Mutation,
            ESharProgressionMutationState::Failed,
            ESharProgressionTerminalResult::Failed
        );
    case ESharProgressionResolutionCommand::Cancel:
        return PublishTerminal(
            *Mutation,
            ESharProgressionMutationState::Cancelled,
            ESharProgressionTerminalResult::Cancelled
        );
    default:
        return ESharProgressionMutationResult::InvalidRequest;
    }
}

ESharProgressionMutationResult USharProgressionSubsystem::Release(
    const FName& MutationId
)
{
    FSharProgressionMutationSnapshot* Mutation = FindMutation(MutationId);
    if (Mutation == nullptr)
    {
        return ESharProgressionMutationResult::NotFound;
    }
    if (Mutation->bReleased)
    {
        return ESharProgressionMutationResult::Released;
    }
    if (!IsTerminalState(Mutation->State))
    {
        return ESharProgressionMutationResult::InvalidState;
    }
    Mutation->bReleased = true;
    Mutation->State = ESharProgressionMutationState::Released;
    return ESharProgressionMutationResult::Accepted;
}
