// File: SharAudioLeaseRegistry.cpp
// Path: src/uproject/Source/SharAudio/Private/Audio/SharAudioLeaseRegistry.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: deterministic audio request and lease lifecycle only; no native source playback.
// ADR: docs/adr/unreal/architecture/aaa-native-content-and-gameplay-foundation.md
// LARGE-FILE owner=SharAudio; reason=cohesive lease lifecycle implementation;
// split=extract owner teardown if request retention policies expand;
// validation=validate.sh SharAudio plus Unreal automation; review=2027-01.

#include "Audio/SharAudioLeaseRegistry.h"

#include "Algo/Find.h"
#include "Audio/SharAudioProfileDefinition.h"
#include "Content/SharPrimaryContentDefinition.h"
#include "Engine/DataAsset.h"

FSharAudioPlaybackState* USharAudioLeaseRegistry::FindState(
    const FName& RequestId
)
{
    return Algo::FindByPredicate(
        States,
        [&RequestId](const FSharAudioPlaybackState& State)
        {
            return State.Request.RequestId == RequestId;
        }
    );
}

const FSharAudioPlaybackState* USharAudioLeaseRegistry::FindState(
    const FName& RequestId
) const
{
    return Algo::FindByPredicate(
        States,
        [&RequestId](const FSharAudioPlaybackState& State)
        {
            return State.Request.RequestId == RequestId;
        }
    );
}

bool USharAudioLeaseRegistry::IsValidRequestIdentity(
    const FSharAudioPlaybackRequest& Request
)
{
    const bool bValidBaseIdentity =
        USharPrimaryContentDefinition::IsCanonicalIdentifier(Request.RequestId)
        && USharPrimaryContentDefinition::IsCanonicalIdentifier(Request.OwnerId)
        && Request.ProfileId.IsValid();
    if (!bValidBaseIdentity)
    {
        return false;
    }
    return Request.LeaseId.IsNone()
        || USharPrimaryContentDefinition::IsCanonicalIdentifier(
            Request.LeaseId
        );
}

bool USharAudioLeaseRegistry::RequestRequiresLease(
    const FSharAudioPlaybackRequest& Request
)
{
    return Request.bLooping
        || USharAudioProfileDefinition::RequiresLease(Request.PlaybackPolicy);
}

ESharAudioPlaybackResult USharAudioLeaseRegistry::BeginPlayback(
    const FSharAudioPlaybackRequest& Request
)
{
    if (!IsValidRequestIdentity(Request))
    {
        return ESharAudioPlaybackResult::InvalidRequest;
    }
    if (FindState(Request.RequestId) != nullptr)
    {
        return ESharAudioPlaybackResult::DuplicateRequest;
    }
    if (RequestRequiresLease(Request) && Request.LeaseId.IsNone())
    {
        return ESharAudioPlaybackResult::LeaseRequired;
    }

    FSharAudioPlaybackState State;
    State.Request = Request;
    State.Result = ESharAudioPlaybackResult::Accepted;
    States.Add(State);
    return ESharAudioPlaybackResult::Accepted;
}

bool USharAudioLeaseRegistry::CompletePlayback(const FName& RequestId)
{
    FSharAudioPlaybackState* State = FindState(RequestId);
    if (State == nullptr || State->Result != ESharAudioPlaybackResult::Accepted)
    {
        return false;
    }
    State->Result = ESharAudioPlaybackResult::Completed;
    return true;
}

bool USharAudioLeaseRegistry::CancelPlayback(const FName& RequestId)
{
    FSharAudioPlaybackState* State = FindState(RequestId);
    if (State == nullptr || State->Result != ESharAudioPlaybackResult::Accepted)
    {
        return false;
    }
    State->Result = ESharAudioPlaybackResult::Cancelled;
    return true;
}

int32 USharAudioLeaseRegistry::ReleaseOwner(const FName& OwnerId)
{
    int32 ReleasedCount = 0;
    for (FSharAudioPlaybackState& State : States)
    {
        const bool bOwnedActiveRequest =
            State.Request.OwnerId == OwnerId
            && State.Result == ESharAudioPlaybackResult::Accepted;
        if (bOwnedActiveRequest)
        {
            State.Result = ESharAudioPlaybackResult::Cancelled;
            ++ReleasedCount;
        }
    }
    return ReleasedCount;
}

ESharAudioPlaybackResult USharAudioLeaseRegistry::GetResult(
    const FName& RequestId
) const
{
    const FSharAudioPlaybackState* State = FindState(RequestId);
    return State == nullptr
        ? ESharAudioPlaybackResult::NotFound
        : State->Result;
}

int32 USharAudioLeaseRegistry::GetActiveCount() const
{
    int32 ActiveCount = 0;
    for (const FSharAudioPlaybackState& State : States)
    {
        ActiveCount += State.Result == ESharAudioPlaybackResult::Accepted ? 1 : 0;
    }
    return ActiveCount;
}

const TArray<FSharAudioPlaybackState>& USharAudioLeaseRegistry::GetStates() const
{
    return States;
}
