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
//   - Shar world readiness composition module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Shar world readiness composition module.
// - Description:
//   - Implements the declared composition module responsibility for project.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Shar world readiness composition module.

#include "Loading/SharWorldReadinessSubsystem.h"

#include "Algo/Find.h"
#include "Content/SharPrimaryContentDefinition.h"

static bool IsCanonicalWorldReadinessId(const FName& Candidate)
{
    return USharPrimaryContentDefinition::IsCanonicalIdentifier(Candidate);
}

static bool HasDuplicateCheckpointIds(const TArray<FName>& CheckpointIds)
{
    return CheckpointIds.ContainsByPredicate(
        [&CheckpointIds](const FName& Candidate)
        {
            int32 MatchCount = 0;
            for (const FName& CheckpointId : CheckpointIds)
            {
                MatchCount += CheckpointId == Candidate ? 1 : 0;
            }
            return MatchCount > 1;
        }
    );
}

bool USharWorldReadinessSubsystem::IsRevisionToken(const FString& Revision)
{
    return Revision.StartsWith(TEXT("sha256:"));
}

bool USharWorldReadinessSubsystem::IsValidBarrier(
    const FSharWorldReadinessBarrier& Barrier
)
{
    return IsCanonicalWorldReadinessId(Barrier.BarrierId)
        && IsCanonicalWorldReadinessId(Barrier.WorldId)
        && IsRevisionToken(Barrier.WorldRevision)
        && !Barrier.RequiredCheckpointIds.IsEmpty()
        && !HasDuplicateCheckpointIds(Barrier.RequiredCheckpointIds)
        && !Barrier.RequiredCheckpointIds.ContainsByPredicate(
            [](const FName& CheckpointId)
            {
                return !IsCanonicalWorldReadinessId(CheckpointId);
            }
        );
}

FSharWorldReadinessSnapshot* USharWorldReadinessSubsystem::FindBarrier(
    const FName& BarrierId
)
{
    return Algo::FindByPredicate(
        Barriers,
        [&BarrierId](const FSharWorldReadinessSnapshot& Snapshot)
        {
            return Snapshot.BarrierId == BarrierId;
        }
    );
}

const FSharWorldReadinessSnapshot* USharWorldReadinessSubsystem::FindBarrier(
    const FName& BarrierId
) const
{
    return Algo::FindByPredicate(
        Barriers,
        [&BarrierId](const FSharWorldReadinessSnapshot& Snapshot)
        {
            return Snapshot.BarrierId == BarrierId;
        }
    );
}

bool USharWorldReadinessSubsystem::ConfigureWorld(
    const FName& InWorldId,
    const FString& InWorldRevision
)
{
    if (!IsCanonicalWorldReadinessId(InWorldId)
        || !IsRevisionToken(InWorldRevision))
    {
        return false;
    }
    WorldId = InWorldId;
    WorldRevision = InWorldRevision;
    Barriers.Reset();
    return true;
}

ESharWorldReadinessResult USharWorldReadinessSubsystem::RegisterBarrier(
    const FSharWorldReadinessBarrier& Barrier
)
{
    if (!IsValidBarrier(Barrier))
    {
        return ESharWorldReadinessResult::InvalidRequest;
    }
    if (Barrier.WorldId != WorldId || Barrier.WorldRevision != WorldRevision)
    {
        return ESharWorldReadinessResult::StaleWorld;
    }
    if (FindBarrier(Barrier.BarrierId) != nullptr)
    {
        return ESharWorldReadinessResult::DuplicateBarrier;
    }
    FSharWorldReadinessSnapshot Snapshot;
    Snapshot.BarrierId = Barrier.BarrierId;
    Snapshot.WorldId = Barrier.WorldId;
    Snapshot.WorldRevision = Barrier.WorldRevision;
    Snapshot.RequiredCheckpointIds = Barrier.RequiredCheckpointIds;
    Snapshot.Revision = 1;
    Barriers.Add(Snapshot);
    return ESharWorldReadinessResult::Accepted;
}

void USharWorldReadinessSubsystem::RefreshReady(
    FSharWorldReadinessSnapshot& Snapshot
)
{
    Snapshot.bReady = Snapshot.RequiredCheckpointIds.Num()
        == Snapshot.CompletedCheckpointIds.Num();
    ++Snapshot.Revision;
}

ESharWorldReadinessResult USharWorldReadinessSubsystem::CompleteCheckpoint(
    const FSharWorldCheckpointCompletion& Completion
)
{
    FSharWorldReadinessSnapshot* Snapshot = FindBarrier(Completion.BarrierId);
    if (Snapshot == nullptr)
    {
        return ESharWorldReadinessResult::BarrierMissing;
    }
    if (Completion.WorldRevision != WorldRevision
        || Snapshot->WorldRevision != Completion.WorldRevision)
    {
        return ESharWorldReadinessResult::StaleWorld;
    }
    if (Snapshot->bReady)
    {
        return ESharWorldReadinessResult::AlreadyReady;
    }
    const bool bRequired = Snapshot->RequiredCheckpointIds.ContainsByPredicate(
        [&Completion](const FName& RequiredId)
        {
            return RequiredId == Completion.CheckpointId;
        }
    );
    if (!bRequired)
    {
        return ESharWorldReadinessResult::CheckpointMissing;
    }
    // jig-ignore-next-line: exact syntax is indivisible
    const bool bDuplicate = Snapshot->CompletedCheckpointIds.ContainsByPredicate(
        [&Completion](const FName& CompletedId)
        {
            return CompletedId == Completion.CheckpointId;
        }
    );
    if (bDuplicate)
    {
        return ESharWorldReadinessResult::DuplicateCheckpoint;
    }
    Snapshot->CompletedCheckpointIds.Add(Completion.CheckpointId);
    RefreshReady(*Snapshot);
    return ESharWorldReadinessResult::Accepted;
}

bool USharWorldReadinessSubsystem::IsReady(const FName& BarrierId) const
{
    const FSharWorldReadinessSnapshot* Snapshot = FindBarrier(BarrierId);
    return Snapshot != nullptr && Snapshot->bReady;
}

int32 USharWorldReadinessSubsystem::GetCompletedCheckpointCount(
    const FName& BarrierId
) const
{
    const FSharWorldReadinessSnapshot* Snapshot = FindBarrier(BarrierId);
    return Snapshot == nullptr ? 0 : Snapshot->CompletedCheckpointIds.Num();
}

int32 USharWorldReadinessSubsystem::GetRequiredCheckpointCount(
    const FName& BarrierId
) const
{
    const FSharWorldReadinessSnapshot* Snapshot = FindBarrier(BarrierId);
    return Snapshot == nullptr ? 0 : Snapshot->RequiredCheckpointIds.Num();
}

int32 USharWorldReadinessSubsystem::TeardownWorld()
{
    const int32 BarrierCount = Barriers.Num();
    Barriers.Reset();
    WorldId = FName();
    WorldRevision = FString();
    return BarrierCount;
}
