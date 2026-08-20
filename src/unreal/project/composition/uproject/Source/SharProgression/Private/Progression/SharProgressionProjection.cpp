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
//   - Shar progression projection composition module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Shar progression projection composition module.
// - Description:
//   - Implements the declared composition module responsibility for project.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Shar progression projection composition module.

#include "Progression/SharProgressionSubsystem.h"

#include "Algo/AnyOf.h"
#include "Content/SharPrimaryContentDefinition.h"
#include "Progression/SharProgressionCatalogDefinition.h"
#include "Progression/SharProgressionCatalogSubsystem.h"
#include "Progression/SharProgressionContracts.h"
#include "Progression/SharProgressionState.h"

static const FSharProgressionValue* FindSnapshotValue(
    const FSharProgressionSnapshot& Snapshot,
    const FName& OperationId,
    const FName& TargetId
)
{
    for (const FSharProgressionValue& Value : Snapshot.Values)
    {
        if (Value.OperationId == OperationId && Value.TargetId == TargetId)
        {
            return &Value;
        }
    }
    return nullptr;
}

ESharProgressionMutationState USharProgressionSubsystem::GetMutationState(
    const FName& MutationId
) const
{
    const FSharProgressionMutationSnapshot* Mutation =
        FindMutation(MutationId);
    return Mutation == nullptr
        ? ESharProgressionMutationState::Failed
        : Mutation->State;
}

ESharProgressionTerminalResult USharProgressionSubsystem::GetTerminalResult(
    const FName& MutationId
) const
{
    const FSharProgressionMutationSnapshot* Mutation =
        FindMutation(MutationId);
    return Mutation == nullptr
        ? ESharProgressionTerminalResult::None
        : Mutation->TerminalResult;
}

FSharProgressionObservation USharProgressionSubsystem::GetObservation() const
{
    FSharProgressionObservation Observation;
    Observation.ProfileState = ProfileState;
    Observation.ActiveSnapshot = ActiveSnapshot;
    Observation.UnreleasedMutationCount = CountUnreleasedMutations();
    return Observation;
}

int32 USharProgressionSubsystem::GetQuantity(
    const FName& OperationId,
    const FName& TargetId
) const
{
    const FSharProgressionValue* Value = FindSnapshotValue(
        ActiveSnapshot,
        OperationId,
        TargetId
    );
    return Value == nullptr ? 0 : Value->Quantity;
}

bool USharProgressionSubsystem::HasAppliedTransaction(
    const FName& TransactionId
) const
{
    return Algo::AnyOf(
        ActiveSnapshot.AppliedPermanentTransactions,
        [&TransactionId](const FName& AppliedId)
        {
            return AppliedId == TransactionId;
        }
    );
}

static bool HasDuplicateProjectionIdentities(const TArray<FName>& Identities)
{
    return Algo::AnyOf(
        Identities,
        [&Identities](const FName& Candidate)
        {
            int32 MatchCount = 0;
            for (const FName& Identity : Identities)
            {
                MatchCount += Identity == Candidate ? 1 : 0;
            }
            return MatchCount > 1;
        }
    );
}

static bool IsValidProjectionIdentitySet(
    const TArray<FName>& Identities,
    const bool bRequireNonEmpty
)
{
    if (bRequireNonEmpty && Identities.IsEmpty())
    {
        return false;
    }
    if (HasDuplicateProjectionIdentities(Identities))
    {
        return false;
    }
    return !Algo::AnyOf(
        Identities,
        [](const FName& Identity)
        {
            return !USharPrimaryContentDefinition::IsCanonicalIdentifier(
                Identity
            );
        }
    );
}

static bool IsValidCountQuery(
    const FSharProgressionCountQuery& Query
)
{
    return USharPrimaryContentDefinition::IsCanonicalIdentifier(
        Query.OperationId
    )
        && IsValidProjectionIdentitySet(Query.RequiredTargetIds, true)
        && IsValidProjectionIdentitySet(Query.ExcludedTargetIds, false);
}

static void AccumulateCountProjection(
    const USharProgressionSubsystem& Subsystem,
    const FSharProgressionCountQuery& Query,
    FSharProgressionCountProjection& Projection
)
{
    for (const FName& TargetId : Query.RequiredTargetIds)
    {
        const bool bExcluded = Algo::AnyOf(
            Query.ExcludedTargetIds,
            [&TargetId](const FName& ExcludedId)
            {
                return ExcludedId == TargetId;
            }
        );
        if (bExcluded)
        {
            continue;
        }
        ++Projection.Denominator;
        Projection.Numerator += Subsystem.GetQuantity(
            Query.OperationId,
            TargetId
        ) > 0
            ? 1
            : 0;
    }
}

bool USharProgressionSubsystem::ProjectCount(
    const FSharProgressionCountQuery& Query,
    FSharProgressionCountProjection& OutProjection
) const
{
    OutProjection = FSharProgressionCountProjection{};
    if (ProfileState != ESharProfileLifecycleState::Ready
        || CatalogSubsystem == nullptr
        || !IsValidCountQuery(Query))
    {
        return false;
    }
    const USharProgressionCatalogDefinition* Catalog =
        CatalogSubsystem->FindCatalog(ActiveSnapshot.CatalogId);
    if (Catalog == nullptr
        || Catalog->FindOperation(Query.OperationId) == nullptr)
    {
        return false;
    }
    AccumulateCountProjection(*this, Query, OutProjection);
    if (OutProjection.Denominator <= 0)
    {
        return false;
    }
    OutProjection.bComplete =
        OutProjection.Numerator == OutProjection.Denominator;
    OutProjection.CatalogRevision = ActiveSnapshot.CatalogRevision;
    OutProjection.SaveRevision = ActiveSnapshot.SaveRevision;
    OutProjection.SnapshotRevision = ActiveSnapshot.SnapshotRevision;
    return true;
}
