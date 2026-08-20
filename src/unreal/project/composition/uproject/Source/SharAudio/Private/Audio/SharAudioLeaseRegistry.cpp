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
//   - Shar audio lease registry composition module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Shar audio lease registry composition module.
// - Description:
//   - Implements the declared composition module responsibility for project.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Shar audio lease registry composition module.

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
        // jig-ignore-next-line: exact syntax is indivisible
        ActiveCount += State.Result == ESharAudioPlaybackResult::Accepted ? 1 : 0;
    }
    return ActiveCount;
}

// jig-ignore-next-line: exact syntax is indivisible
const TArray<FSharAudioPlaybackState>& USharAudioLeaseRegistry::GetStates() const
{
    return States;
}
