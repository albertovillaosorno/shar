// File: SharFrontendFlowSubsystem.cpp
// Path: src/uproject/Source/SharUI/Private/UI/SharFrontendFlowSubsystem.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: frontend flow configuration, immutable observations, queue queries, and shared lifecycle helpers only.
// Specification: docs/technical/unreal/frontend-screen-flow-and-settings-runtime.md

#include "UI/SharFrontendFlowSubsystem.h"

#include "Algo/Count.h"
#include "Content/SharPrimaryContentDefinition.h"
#include "UI/SharFrontendCatalogDefinition.h"
#include "UI/SharFrontendCatalogSubsystem.h"
#include "UI/SharFrontendFlowContracts.h"

static bool IsTerminalState(const ESharFrontendTransitionState State)
{
    return State == ESharFrontendTransitionState::Success
        || State == ESharFrontendTransitionState::Failed
        || State == ESharFrontendTransitionState::Cancelled
        || State == ESharFrontendTransitionState::Superseded;
}

static int32 PriorityRank(const ESharFrontendNavigationPriority Priority)
{
    return static_cast<int32>(Priority);
}
bool USharFrontendFlowSubsystem::Configure(
    USharFrontendCatalogSubsystem* InCatalog,
    const FString& InitialFlowRevision,
    const FString& InitialScreenRevision,
    const FName& InitialFocusTargetId
)
{
    if (bConfigured || InCatalog == nullptr || !InCatalog->IsActive()
        || InitialFlowRevision.IsEmpty()
        || !InitialFlowRevision.Contains(TEXT(":"))
        || InitialScreenRevision.IsEmpty()
        || !InitialScreenRevision.Contains(TEXT(":"))
        || !USharPrimaryContentDefinition::IsCanonicalIdentifier(
            InitialFocusTargetId
        ))
    {
        return false;
    }
    const FSharFrontendScreenDefinition* InitialScreen =
        InCatalog->GetInitialScreen();
    if (InitialScreen == nullptr)
    {
        return false;
    }
    Catalog = InCatalog;
    Observation.ActivePrimaryScreenId = InitialScreen->ScreenId;
    Observation.ActiveModalScreenId = NAME_None;
    Observation.PrimaryHistory.Reset();
    Observation.FlowRevision = InitialFlowRevision;
    Observation.ActiveScreenRevision = InitialScreenRevision;
    Observation.ActiveTransitionId = NAME_None;
    Observation.StableFocusTargetId = InitialFocusTargetId;
    bConfigured = true;
    return true;
}

bool USharFrontendFlowSubsystem::IsConfigured() const
{
    return bConfigured;
}

int32 USharFrontendFlowSubsystem::GetQueueCount() const
{
    return Algo::CountIf(
        Transitions,
        [](const FSharFrontendTransitionSnapshot& Transition)
        {
            return Transition.State != ESharFrontendTransitionState::Released;
        }
    );
}

FName USharFrontendFlowSubsystem::GetHeadRequestId() const
{
    for (const FSharFrontendTransitionSnapshot& Transition : Transitions)
    {
        if (Transition.State != ESharFrontendTransitionState::Released
            && !IsTerminalState(Transition.State))
        {
            return Transition.Request.RequestId;
        }
    }
    return NAME_None;
}

int32 USharFrontendFlowSubsystem::GetQueuePosition(
    const FName& RequestId
) const
{
    int32 Position = 0;
    for (const FSharFrontendTransitionSnapshot& Transition : Transitions)
    {
        if (Transition.State == ESharFrontendTransitionState::Released
            || IsTerminalState(Transition.State))
        {
            continue;
        }
        ++Position;
        if (Transition.Request.RequestId == RequestId)
        {
            return Position;
        }
    }
    return 0;
}

const FSharFrontendFlowObservation&
USharFrontendFlowSubsystem::GetObservation() const
{
    return Observation;
}

const FSharFrontendTransitionSnapshot*
USharFrontendFlowSubsystem::FindTransition(const FName& RequestId) const
{
    return Transitions.FindByPredicate(
        [&RequestId](const FSharFrontendTransitionSnapshot& Candidate)
        {
            return Candidate.Request.RequestId == RequestId;
        }
    );
}

FSharFrontendTransitionSnapshot*
USharFrontendFlowSubsystem::FindMutableTransition(const FName& RequestId)
{
    return Transitions.FindByPredicate(
        [&RequestId](const FSharFrontendTransitionSnapshot& Candidate)
        {
            return Candidate.Request.RequestId == RequestId;
        }
    );
}

ESharFrontendTerminalResult USharFrontendFlowSubsystem::GetTerminalResult(
    const FName& RequestId
) const
{
    const FSharFrontendTransitionSnapshot* Transition =
        FindTransition(RequestId);
    return Transition == nullptr
        ? ESharFrontendTerminalResult::None
        : Transition->TerminalResult;
}

FName USharFrontendFlowSubsystem::GetEffectiveScreenId() const
{
    return Observation.ActiveModalScreenId.IsNone()
        ? Observation.ActivePrimaryScreenId
        : Observation.ActiveModalScreenId;
}

FString USharFrontendFlowSubsystem::MakeNextFlowRevision()
{
    return FString::Printf(
        TEXT("frontend-flow:%lld"),
        NextFlowRevisionOrdinal++
    );
}

FString USharFrontendFlowSubsystem::MakeNextScreenRevision()
{
    return FString::Printf(
        TEXT("frontend-screen:%lld"),
        NextScreenRevisionOrdinal++
    );
}

const FSharFrontendScreenDefinition*
USharFrontendFlowSubsystem::GetDestinationDefinition(
    const FSharFrontendTransitionSnapshot& Transition
) const
{
    return Catalog == nullptr
        ? nullptr
        : Catalog->FindScreen(Transition.Request.DestinationScreenId);
}

bool USharFrontendFlowSubsystem::IsHead(const FName& RequestId) const
{
    return GetHeadRequestId() == RequestId;
}

bool USharFrontendFlowSubsystem::HasEvidence(
    const FSharFrontendTransitionSnapshot& Transition,
    const ESharFrontendReadinessKind Kind
)
{
    return Transition.Evidence.ContainsByPredicate(
        [Kind](const FSharFrontendReadinessEvidence& Candidate)
        {
            return Candidate.Kind == Kind;
        }
    );
}

bool USharFrontendFlowSubsystem::RequirementsSatisfied(
    const FSharFrontendTransitionSnapshot& Transition,
    const TArray<ESharFrontendReadinessKind>& Requirements
)
{
    return Requirements.FindByPredicate(
        [&Transition](const ESharFrontendReadinessKind Kind)
        {
            return !Transition.Evidence.ContainsByPredicate(
                [Kind](const FSharFrontendReadinessEvidence& Candidate)
                {
                    return Candidate.Kind == Kind
                        && Candidate.Status
                            != ESharFrontendEvidenceStatus::Failed;
                }
            );
        }
    ) == nullptr;
}

void USharFrontendFlowSubsystem::SortQueue()
{
    Transitions.StableSort(
        [](const FSharFrontendTransitionSnapshot& Left,
           const FSharFrontendTransitionSnapshot& Right)
        {
            const bool bLeftInactive =
                Left.State == ESharFrontendTransitionState::Released
                || IsTerminalState(Left.State);
            const bool bRightInactive =
                Right.State == ESharFrontendTransitionState::Released
                || IsTerminalState(Right.State);
            if (bLeftInactive != bRightInactive)
            {
                return !bLeftInactive;
            }
            const bool bLeftStarted =
                Left.State != ESharFrontendTransitionState::Pending;
            const bool bRightStarted =
                Right.State != ESharFrontendTransitionState::Pending;
            if (bLeftStarted != bRightStarted)
            {
                return bLeftStarted;
            }
            const int32 LeftPriority = PriorityRank(Left.Request.Priority);
            const int32 RightPriority = PriorityRank(Right.Request.Priority);
            if (LeftPriority != RightPriority)
            {
                return LeftPriority > RightPriority;
            }
            const int32 IdentityOrder = Left.Request.RequestId.Compare(
                Right.Request.RequestId
            );
            return IdentityOrder == 0
                ? Left.InsertionSequence < Right.InsertionSequence
                : IdentityOrder < 0;
        }
    );
}


void USharFrontendFlowSubsystem::CompleteTerminal(
    FSharFrontendTransitionSnapshot& Transition,
    const ESharFrontendTerminalResult Result,
    const ESharFrontendTransitionState State
)
{
    if (Transition.TerminalResult != ESharFrontendTerminalResult::None)
    {
        return;
    }
    Transition.TerminalResult = Result;
    Transition.State = State;
    Observation.ActiveTransitionId = NAME_None;
    SortQueue();
}

void USharFrontendFlowSubsystem::CompleteSuccess(
    FSharFrontendTransitionSnapshot& Transition
)
{
    CompleteTerminal(
        Transition,
        ESharFrontendTerminalResult::Success,
        ESharFrontendTransitionState::Success
    );
}

void USharFrontendFlowSubsystem::RestorePriorObservation(
    const FSharFrontendTransitionSnapshot& Transition
)
{
    Observation = Transition.PriorObservation;
    Observation.ActiveTransitionId = NAME_None;
}

ESharFrontendOperationResult USharFrontendFlowSubsystem::UpdateStableFocus(
    const FName& FocusTargetId,
    const FString& ExpectedFlowRevision
)
{
    if (!bConfigured)
    {
        return ESharFrontendOperationResult::NotConfigured;
    }
    if (!Observation.ActiveTransitionId.IsNone())
    {
        return ESharFrontendOperationResult::ConflictingTransition;
    }
    if (ExpectedFlowRevision != Observation.FlowRevision)
    {
        return ESharFrontendOperationResult::StaleRevision;
    }
    if (!USharPrimaryContentDefinition::IsCanonicalIdentifier(FocusTargetId))
    {
        return ESharFrontendOperationResult::InvalidRequest;
    }
    Observation.StableFocusTargetId = FocusTargetId;
    return ESharFrontendOperationResult::Accepted;
}
