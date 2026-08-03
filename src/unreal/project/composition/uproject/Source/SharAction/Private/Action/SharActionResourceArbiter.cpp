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
//   - Shar action resource arbiter composition module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Shar action resource arbiter composition module.
// - Description:
//   - Implements the declared composition module responsibility for project.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Shar action resource arbiter composition module.

#include "Action/SharActionResourceArbiter.h"

#include "Action/SharActionDefinition.h"
#include "Algo/AnyOf.h"
#include "Algo/Find.h"
#include "Content/SharPrimaryContentDefinition.h"

static bool IsCanonicalLeaseId(const FName& Candidate)
{
    return USharPrimaryContentDefinition::IsCanonicalIdentifier(Candidate);
}

static bool IsLeaseRevision(const FString& Candidate)
{
    return Candidate.StartsWith(TEXT("sha256:"));
}

FSharActionLeaseState* USharActionResourceArbiter::FindLease(
    const FName& LeaseId
)
{
    return Algo::FindByPredicate(
        Leases,
        [&LeaseId](const FSharActionLeaseState& Lease)
        {
            return Lease.LeaseId == LeaseId;
        }
    );
}

const FSharActionLeaseState* USharActionResourceArbiter::FindLease(
    const FName& LeaseId
) const
{
    return Algo::FindByPredicate(
        Leases,
        [&LeaseId](const FSharActionLeaseState& Lease)
        {
            return Lease.LeaseId == LeaseId;
        }
    );
}

bool USharActionResourceArbiter::IsValidRequest(
    const FSharActionResourceRequest& Request
)
{
    return IsCanonicalLeaseId(Request.LeaseId)
        && IsCanonicalLeaseId(Request.OwnerId)
        && IsCanonicalLeaseId(Request.ResourceId)
        && IsLeaseRevision(Request.OwnerRevision);
}

bool USharActionResourceArbiter::IsAvailable(
    const FName& ResourceId,
    const ESharActionResourceAccess RequestedAccess
) const
{
    if (!IsCanonicalLeaseId(ResourceId))
    {
        return false;
    }
    return !Algo::AnyOf(
        Leases,
        [&ResourceId, RequestedAccess](const FSharActionLeaseState& Lease)
        {
            if (!Lease.bActive || Lease.ResourceId != ResourceId)
            {
                return false;
            }
            const bool bExclusiveRequest =
                RequestedAccess == ESharActionResourceAccess::Exclusive;
            const bool bExclusiveLease =
                Lease.Access == ESharActionResourceAccess::Exclusive;
            return bExclusiveRequest || bExclusiveLease;
        }
    );
}

ESharActionLeaseResult USharActionResourceArbiter::Acquire(
    const FSharActionResourceRequest& Request
)
{
    if (!IsValidRequest(Request))
    {
        return ESharActionLeaseResult::InvalidRequest;
    }
    if (FindLease(Request.LeaseId) != nullptr)
    {
        return ESharActionLeaseResult::DuplicateLease;
    }
    if (!IsAvailable(Request.ResourceId, Request.Access))
    {
        return ESharActionLeaseResult::ResourceConflict;
    }

    FSharActionLeaseState Lease;
    Lease.LeaseId = Request.LeaseId;
    Lease.OwnerId = Request.OwnerId;
    Lease.ResourceId = Request.ResourceId;
    Lease.OwnerRevision = Request.OwnerRevision;
    Lease.Access = Request.Access;
    Lease.bActive = true;
    Leases.Add(Lease);
    return ESharActionLeaseResult::Granted;
}

ESharActionLeaseResult USharActionResourceArbiter::Release(
    const FName& LeaseId
)
{
    FSharActionLeaseState* Lease = FindLease(LeaseId);
    if (Lease == nullptr || !Lease->bActive)
    {
        return ESharActionLeaseResult::NotFound;
    }
    Lease->bActive = false;
    return ESharActionLeaseResult::Released;
}

int32 USharActionResourceArbiter::ReleaseOwner(const FName& OwnerId)
{
    if (!IsCanonicalLeaseId(OwnerId))
    {
        return 0;
    }
    int32 ReleasedCount = 0;
    for (FSharActionLeaseState& Lease : Leases)
    {
        if (Lease.OwnerId == OwnerId && Lease.bActive)
        {
            Lease.bActive = false;
            ++ReleasedCount;
        }
    }
    return ReleasedCount;
}

bool USharActionResourceArbiter::IsLeaseActive(const FName& LeaseId) const
{
    const FSharActionLeaseState* Lease = FindLease(LeaseId);
    return Lease != nullptr && Lease->bActive;
}

int32 USharActionResourceArbiter::GetActiveLeaseCount() const
{
    int32 ActiveCount = 0;
    for (const FSharActionLeaseState& Lease : Leases)
    {
        ActiveCount += Lease.bActive ? 1 : 0;
    }
    return ActiveCount;
}
