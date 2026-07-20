// File: SharCameraArbitrator.cpp
// Path: src/uproject/Source/SharCamera/Private/Camera/SharCameraArbitrator.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: deterministic camera request ordering, cancellation, and restoration only.
// ADR: docs/adr/unreal/architecture/aaa-native-content-and-gameplay-foundation.md
// LARGE-FILE owner=SharCamera; reason=cohesive arbitration lifecycle implementation;
// split=extract priority policy if additional arbitration classes appear;
// validation=validate.sh SharCamera plus Unreal automation; review=2027-01.

#include "Camera/SharCameraArbitrator.h"

#include "Algo/Find.h"
#include "Camera/SharCameraProfileDefinition.h"
#include "Content/SharPrimaryContentDefinition.h"
#include "Engine/DataAsset.h"


static constexpr int32 DebugPriorityRank = 0;
static constexpr int32 DefaultPriorityRank = 1;
static constexpr int32 ContextualGameplayPriorityRank = 2;
static constexpr int32 PlayerSelectedPriorityRank = 3;
static constexpr int32 ConversationPriorityRank = 4;
static constexpr int32 CinematicPriorityRank = 5;
static constexpr int32 SafetyPriorityRank = 6;

FSharCameraRequestState* USharCameraArbitrator::FindRequestState(
    const FName& RequestId
)
{
    return Algo::FindByPredicate(
        Requests,
        [&RequestId](const FSharCameraRequestState& State)
        {
            return State.Request.RequestId == RequestId;
        }
    );
}

const FSharCameraRequestState* USharCameraArbitrator::FindRequestState(
    const FName& RequestId
) const
{
    return Algo::FindByPredicate(
        Requests,
        [&RequestId](const FSharCameraRequestState& State)
        {
            return State.Request.RequestId == RequestId;
        }
    );
}

bool USharCameraArbitrator::IsValidRequest(
    const FSharCameraRequest& Request
)
{
    return USharPrimaryContentDefinition::IsCanonicalIdentifier(
        Request.RequestId
    )
        && USharPrimaryContentDefinition::IsCanonicalIdentifier(
            Request.RequesterId
        )
        && Request.CameraProfileId.IsValid()
        && USharPrimaryContentDefinition::IsCanonicalIdentifier(
            Request.TargetId
        );
}

int32 USharCameraArbitrator::GetPriorityRank(
    const ESharCameraPriorityClass PriorityClass
)
{
    switch (PriorityClass)
    {
    case ESharCameraPriorityClass::Debug:
        return DebugPriorityRank;
    case ESharCameraPriorityClass::Default:
        return DefaultPriorityRank;
    case ESharCameraPriorityClass::ContextualGameplay:
        return ContextualGameplayPriorityRank;
    case ESharCameraPriorityClass::PlayerSelected:
        return PlayerSelectedPriorityRank;
    case ESharCameraPriorityClass::Conversation:
        return ConversationPriorityRank;
    case ESharCameraPriorityClass::Cinematic:
        return CinematicPriorityRank;
    case ESharCameraPriorityClass::Safety:
        return SafetyPriorityRank;
    default:
        return DebugPriorityRank;
    }
}

bool USharCameraArbitrator::Outranks(
    const FSharCameraRequest& Candidate,
    const FSharCameraRequest& Current
)
{
    const int32 CandidateRank = GetPriorityRank(Candidate.PriorityClass);
    const int32 CurrentRank = GetPriorityRank(Current.PriorityClass);
    if (CandidateRank != CurrentRank)
    {
        return CandidateRank > CurrentRank;
    }
    if (Candidate.PriorityOffset != Current.PriorityOffset)
    {
        return Candidate.PriorityOffset > Current.PriorityOffset;
    }
    if (Candidate.RequesterId != Current.RequesterId)
    {
        return Candidate.RequesterId.LexicalLess(Current.RequesterId);
    }
    return Candidate.RequestId.LexicalLess(Current.RequestId);
}

void USharCameraArbitrator::ResetActiveRequestState()
{
    ActiveRequestId = FName();
    ActiveProfileId = FPrimaryAssetId();
    for (FSharCameraRequestState& State : Requests)
    {
        if (State.Status == ESharCameraRequestStatus::Active)
        {
            State.Status = ESharCameraRequestStatus::Queued;
        }
    }
}

FSharCameraRequestState* USharCameraArbitrator::FindBestQueuedRequest()
{
    FSharCameraRequestState* BestState = nullptr;
    for (FSharCameraRequestState& State : Requests)
    {
        if (State.Status != ESharCameraRequestStatus::Queued)
        {
            continue;
        }
        if (BestState == nullptr
            || Outranks(State.Request, BestState->Request))
        {
            BestState = &State;
        }
    }
    return BestState;
}

void USharCameraArbitrator::SupersedeNonDeferrableRequests(
    const FSharCameraRequestState* BestState
)
{
    for (FSharCameraRequestState& State : Requests)
    {
        const bool bShouldSupersede =
            &State != BestState
            && State.Status == ESharCameraRequestStatus::Queued
            && !State.Request.bCanDefer;
        if (bShouldSupersede)
        {
            State.Status = ESharCameraRequestStatus::Superseded;
        }
    }
}

void USharCameraArbitrator::ActivateRequest(
    FSharCameraRequestState* BestState
)
{
    if (BestState == nullptr)
    {
        return;
    }
    BestState->Status = ESharCameraRequestStatus::Active;
    ActiveRequestId = BestState->Request.RequestId;
    ActiveProfileId = BestState->Request.CameraProfileId;
}

void USharCameraArbitrator::RecalculateActiveRequest()
{
    ResetActiveRequestState();
    FSharCameraRequestState* BestState = FindBestQueuedRequest();
    SupersedeNonDeferrableRequests(BestState);
    ActivateRequest(BestState);
}

ESharCameraRequestStatus USharCameraArbitrator::SubmitRequest(
    const FSharCameraRequest& Request
)
{
    if (!IsValidRequest(Request) || FindRequestState(Request.RequestId) != nullptr)
    {
        return ESharCameraRequestStatus::Rejected;
    }

    FSharCameraRequestState NewState;
    NewState.Request = Request;
    NewState.Status = ESharCameraRequestStatus::Queued;
    Requests.Add(NewState);
    RecalculateActiveRequest();

    const FSharCameraRequestState* StoredState =
        FindRequestState(Request.RequestId);
    return StoredState == nullptr
        ? ESharCameraRequestStatus::Rejected
        : StoredState->Status;
}

bool USharCameraArbitrator::CancelRequest(const FName& RequestId)
{
    FSharCameraRequestState* State = FindRequestState(RequestId);
    if (State == nullptr
        || State->Status == ESharCameraRequestStatus::Cancelled
        || State->Status == ESharCameraRequestStatus::Rejected)
    {
        return false;
    }
    State->Status = ESharCameraRequestStatus::Cancelled;
    RecalculateActiveRequest();
    return true;
}

FName USharCameraArbitrator::GetActiveRequestId() const
{
    return ActiveRequestId;
}

FPrimaryAssetId USharCameraArbitrator::GetActiveProfileId() const
{
    return ActiveProfileId;
}

ESharCameraRequestStatus USharCameraArbitrator::GetRequestStatus(
    const FName& RequestId
) const
{
    const FSharCameraRequestState* State = FindRequestState(RequestId);
    return State == nullptr
        ? ESharCameraRequestStatus::Rejected
        : State->Status;
}

const TArray<FSharCameraRequestState>&
USharCameraArbitrator::GetRequests() const
{
    return Requests;
}
