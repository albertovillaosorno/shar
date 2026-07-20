// File: SharProgressionQueue.cpp
// Path: src/uproject/Source/SharProgression/Private/Progression/SharProgressionQueue.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: profile configuration, snapshot validation, mutation admission, ordering, submission, and begin only.
// Specification: docs/technical/unreal/progression-collectibles-and-cheats.md
// LARGE-FILE owner=SharProgression; reason=cohesive progression mutation admission and arbitration;
// split=extract profile switching when multiple local profiles are active concurrently;
// validation=validate.sh SharProgression plus Unreal automation; review=2027-01.

#include "Progression/SharProgressionSubsystem.h"

#include "Algo/AnyOf.h"
#include "Algo/Find.h"
#include "Content/SharPrimaryContentDefinition.h"
#include "Progression/SharProgressionCatalogDefinition.h"
#include "Progression/SharProgressionCatalogSubsystem.h"
#include "Progression/SharProgressionContracts.h"
#include "Progression/SharProgressionState.h"

static constexpr int32 MaximumPendingProgressionMutations = 32;

bool USharProgressionSubsystem::IsCanonicalIdentity(const FName& Candidate)
{
    return USharPrimaryContentDefinition::IsCanonicalIdentifier(Candidate);
}

bool USharProgressionSubsystem::IsRevisionToken(const FString& Revision)
{
    return Revision.StartsWith(TEXT("sha256:"));
}

bool USharProgressionSubsystem::ProfileIdentitiesMatch(
    const FSharProfileIdentity& Left,
    const FSharProfileIdentity& Right
)
{
    return Left.ProfileId == Right.ProfileId
        && Left.ProfileRevision == Right.ProfileRevision;
}

bool USharProgressionSubsystem::IsTerminalState(
    const ESharProgressionMutationState State
)
{
    return State == ESharProgressionMutationState::Completed
        || State == ESharProgressionMutationState::Failed
        || State == ESharProgressionMutationState::Cancelled;
}

bool USharProgressionSubsystem::Outranks(
    const FSharProgressionMutationSnapshot& Left,
    const FSharProgressionMutationSnapshot& Right
)
{
    const auto LeftPriority = static_cast<uint8>(Left.Request.Priority);
    const auto RightPriority = static_cast<uint8>(Right.Request.Priority);
    if (LeftPriority != RightPriority)
    {
        return LeftPriority > RightPriority;
    }
    return Left.Request.MutationId.LexicalLess(Right.Request.MutationId);
}

static bool HasDuplicateRewardTransactionIds(
    const TArray<FSharRewardRequest>& Operations
)
{
    return Algo::AnyOf(
        Operations,
        [&Operations](const FSharRewardRequest& Candidate)
        {
            int32 MatchCount = 0;
            for (const FSharRewardRequest& Operation : Operations)
            {
                MatchCount += Operation.TransactionId == Candidate.TransactionId
                    ? 1
                    : 0;
            }
            return MatchCount > 1;
        }
    );
}

static bool SnapshotValueMatchesPolicy(
    const FSharProgressionValue& Value,
    const FSharProgressionOperationDefinition& Definition
)
{
    if (Value.Quantity <= 0 || Value.Quantity > Definition.MaximumQuantity)
    {
        return false;
    }
    return Definition.ValuePolicy != ESharProgressionValuePolicy::SetOnce
        || Value.Quantity == 1;
}

bool USharProgressionSubsystem::ValidateSnapshot(
    const FSharProgressionSnapshot& Snapshot
) const
{
    if (CatalogSubsystem == nullptr || !CatalogSubsystem->IsActive())
    {
        return false;
    }
    const bool bInvalidIdentity =
        !IsCanonicalIdentity(Snapshot.Profile.ProfileId)
        || !IsCanonicalIdentity(Snapshot.CatalogId);
    const bool bInvalidRevision =
        !IsRevisionToken(Snapshot.Profile.ProfileRevision)
        || !IsRevisionToken(Snapshot.CatalogRevision)
        || !IsRevisionToken(Snapshot.SaveRevision)
        || !IsRevisionToken(Snapshot.SnapshotRevision);
    if (bInvalidIdentity || bInvalidRevision
        || Snapshot.CatalogRevision != CatalogSubsystem->GetCatalogRevision())
    {
        return false;
    }
    const USharProgressionCatalogDefinition* Catalog =
        CatalogSubsystem->FindCatalog(Snapshot.CatalogId);
    if (Catalog == nullptr
        || Snapshot.SchemaVersion != Catalog->SnapshotSchemaVersion)
    {
        return false;
    }
    auto* State = NewObject<USharProgressionState>();
    if (!State->InitializeSnapshot(
        Snapshot.Values,
        Snapshot.AppliedPermanentTransactions
    ))
    {
        return false;
    }
    return !Algo::AnyOf(
        Snapshot.Values,
        [Catalog](const FSharProgressionValue& Value)
        {
            const FSharProgressionOperationDefinition* Definition =
                Catalog->FindOperation(Value.OperationId);
            return Definition == nullptr
                || !SnapshotValueMatchesPolicy(Value, *Definition);
        }
    );
}

bool USharProgressionSubsystem::Configure(
    USharProgressionCatalogSubsystem* InCatalogSubsystem,
    const FSharProgressionSnapshot& InitialSnapshot
)
{
    CatalogSubsystem = InCatalogSubsystem;
    if (!ValidateSnapshot(InitialSnapshot))
    {
        CatalogSubsystem = nullptr;
        ProfileState = ESharProfileLifecycleState::Failed;
        return false;
    }
    ActiveSnapshot = InitialSnapshot;
    Mutations.Reset();
    NextInsertionSequence = 0;
    ProfileState = ESharProfileLifecycleState::Ready;
    return true;
}

ESharProgressionMutationResult USharProgressionSubsystem::ValidateOperation(
    const FSharRewardRequest& Operation,
    const USharProgressionCatalogDefinition& Catalog
)
{
    const bool bInvalidIdentity =
        !IsCanonicalIdentity(Operation.TransactionId)
        || !IsCanonicalIdentity(Operation.OperationId)
        || !IsCanonicalIdentity(Operation.TargetId);
    if (bInvalidIdentity || Operation.Quantity <= 0)
    {
        return ESharProgressionMutationResult::InvalidRequest;
    }
    const FSharProgressionOperationDefinition* Definition =
        Catalog.FindOperation(Operation.OperationId);
    if (Definition == nullptr)
    {
        return ESharProgressionMutationResult::UnsupportedOperation;
    }
    if (!Operation.bPermanent || !Definition->bPermanentAllowed)
    {
        return ESharProgressionMutationResult::PolicyViolation;
    }
    return Operation.Quantity > Definition->MaximumQuantity
        ? ESharProgressionMutationResult::QuantityOverflow
        : ESharProgressionMutationResult::Accepted;
}

ESharProgressionMutationResult
USharProgressionSubsystem::ValidateRuntimeState() const
{
    if (CatalogSubsystem == nullptr)
    {
        return ESharProgressionMutationResult::CatalogMissing;
    }
    if (!CatalogSubsystem->IsActive())
    {
        return ESharProgressionMutationResult::CatalogInactive;
    }
    return ProfileState == ESharProfileLifecycleState::Ready
        ? ESharProgressionMutationResult::Accepted
        : ESharProgressionMutationResult::ProfileNotReady;
}

bool USharProgressionSubsystem::HasValidOperationSpecIdentity(
    const FSharProgressionMutationRequest& OperationSpec
)
{
    const bool bValidIdentity =
        IsCanonicalIdentity(OperationSpec.MutationId)
        && IsCanonicalIdentity(OperationSpec.Profile.ProfileId)
        && IsCanonicalIdentity(OperationSpec.CatalogId);
    const bool bValidRevision =
        IsRevisionToken(OperationSpec.Profile.ProfileRevision)
        && IsRevisionToken(OperationSpec.ExpectedCatalogRevision)
        && IsRevisionToken(OperationSpec.ExpectedSaveRevision)
        && IsRevisionToken(OperationSpec.ExpectedSnapshotRevision)
        && IsRevisionToken(OperationSpec.MutationRevision);
    return bValidIdentity && bValidRevision;
}

bool USharProgressionSubsystem::OperationSpecMatchesActiveSnapshot(
    const FSharProgressionMutationRequest& OperationSpec
) const
{
    return ProfileIdentitiesMatch(
        OperationSpec.Profile,
        ActiveSnapshot.Profile
    )
        && OperationSpec.CatalogId == ActiveSnapshot.CatalogId
        && OperationSpec.ExpectedCatalogRevision
            == ActiveSnapshot.CatalogRevision
        && OperationSpec.ExpectedSaveRevision == ActiveSnapshot.SaveRevision
        && OperationSpec.ExpectedSnapshotRevision
            == ActiveSnapshot.SnapshotRevision;
}

ESharProgressionMutationResult
USharProgressionSubsystem::ValidateOperationBatch(
    const FSharProgressionMutationRequest& OperationSpec,
    const USharProgressionCatalogDefinition& Catalog
)
{
    const bool bInvalidBatch =
        OperationSpec.Operations.IsEmpty()
        || OperationSpec.Operations.Num() > Catalog.MaximumMutationOperations
        || HasDuplicateRewardTransactionIds(OperationSpec.Operations);
    if (bInvalidBatch)
    {
        return ESharProgressionMutationResult::InvalidRequest;
    }
    for (const FSharRewardRequest& Operation : OperationSpec.Operations)
    {
        const ESharProgressionMutationResult Result =
            ValidateOperation(Operation, Catalog);
        if (Result != ESharProgressionMutationResult::Accepted)
        {
            return Result;
        }
    }
    return ESharProgressionMutationResult::Accepted;
}

ESharProgressionMutationResult
USharProgressionSubsystem::ValidateOperationSpec(
    const FSharProgressionMutationRequest& OperationSpec
) const
{
    const ESharProgressionMutationResult RuntimeResult =
        ValidateRuntimeState();
    if (RuntimeResult != ESharProgressionMutationResult::Accepted)
    {
        return RuntimeResult;
    }
    if (!HasValidOperationSpecIdentity(OperationSpec))
    {
        return ESharProgressionMutationResult::InvalidRequest;
    }
    if (!OperationSpecMatchesActiveSnapshot(OperationSpec))
    {
        return ESharProgressionMutationResult::StaleRevision;
    }
    const USharProgressionCatalogDefinition* Catalog =
        CatalogSubsystem->FindCatalog(OperationSpec.CatalogId);
    return Catalog == nullptr
        ? ESharProgressionMutationResult::CatalogDefinitionMissing
        : ValidateOperationBatch(OperationSpec, *Catalog);
}

FSharProgressionMutationSnapshot* USharProgressionSubsystem::FindMutation(
    const FName& MutationId
)
{
    return Algo::FindByPredicate(
        Mutations,
        [&MutationId](const FSharProgressionMutationSnapshot& Mutation)
        {
            return Mutation.Request.MutationId == MutationId;
        }
    );
}

const FSharProgressionMutationSnapshot*
USharProgressionSubsystem::FindMutation(const FName& MutationId) const
{
    return Algo::FindByPredicate(
        Mutations,
        [&MutationId](const FSharProgressionMutationSnapshot& Mutation)
        {
            return Mutation.Request.MutationId == MutationId;
        }
    );
}

bool USharProgressionSubsystem::IsHead(
    const FSharProgressionMutationSnapshot& Mutation
) const
{
    if (Mutation.bReleased
        || Mutation.State != ESharProgressionMutationState::Queued)
    {
        return false;
    }
    return !Algo::AnyOf(
        Mutations,
        [&Mutation](const FSharProgressionMutationSnapshot& Other)
        {
            const bool bComparable =
                !Other.bReleased
                && Other.State == ESharProgressionMutationState::Queued
                && Other.Request.MutationId != Mutation.Request.MutationId;
            return bComparable && Outranks(Other, Mutation);
        }
    );
}

bool USharProgressionSubsystem::HasActiveMutation(
    const FSharProgressionMutationSnapshot& Mutation
) const
{
    return Algo::AnyOf(
        Mutations,
        [&Mutation](const FSharProgressionMutationSnapshot& Other)
        {
            return !Other.bReleased
                && Other.Request.MutationId != Mutation.Request.MutationId
                && Other.State != ESharProgressionMutationState::Queued
                && !IsTerminalState(Other.State);
        }
    );
}

int32 USharProgressionSubsystem::CountUnreleasedMutations() const
{
    int32 Count = 0;
    for (const FSharProgressionMutationSnapshot& Mutation : Mutations)
    {
        Count += Mutation.bReleased ? 0 : 1;
    }
    return Count;
}

static int32 CountPendingMutations(
    const TArray<FSharProgressionMutationSnapshot>& Mutations
)
{
    int32 Count = 0;
    for (const FSharProgressionMutationSnapshot& Mutation : Mutations)
    {
        Count += !Mutation.bReleased
                && Mutation.State == ESharProgressionMutationState::Queued
            ? 1
            : 0;
    }
    return Count;
}

ESharProgressionMutationResult USharProgressionSubsystem::Submit(
    const FSharProgressionMutationRequest& OperationSpec
)
{
    if (FindMutation(OperationSpec.MutationId) != nullptr)
    {
        return ESharProgressionMutationResult::DuplicateMutation;
    }
    const ESharProgressionMutationResult Validation =
        ValidateOperationSpec(OperationSpec);
    if (Validation != ESharProgressionMutationResult::Accepted)
    {
        return Validation;
    }
    if (CountPendingMutations(Mutations) >= MaximumPendingProgressionMutations)
    {
        return ESharProgressionMutationResult::ConflictingMutation;
    }
    FSharProgressionMutationSnapshot Mutation;
    Mutation.Request = OperationSpec;
    Mutation.InsertionSequence = NextInsertionSequence;
    ++NextInsertionSequence;
    Mutations.Add(Mutation);
    return ESharProgressionMutationResult::Accepted;
}

ESharProgressionMutationResult USharProgressionSubsystem::Begin(
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
    if (Mutation->State != ESharProgressionMutationState::Queued)
    {
        return ESharProgressionMutationResult::InvalidState;
    }
    if (!IsHead(*Mutation))
    {
        return ESharProgressionMutationResult::NotHead;
    }
    if (HasActiveMutation(*Mutation))
    {
        return ESharProgressionMutationResult::ConflictingMutation;
    }
    const ESharProgressionMutationResult Validation =
        ValidateOperationSpec(Mutation->Request);
    if (Validation != ESharProgressionMutationResult::Accepted)
    {
        return Validation;
    }
    Mutation->State = ESharProgressionMutationState::Preparing;
    return ESharProgressionMutationResult::Accepted;
}

int32 USharProgressionSubsystem::GetQueuePosition(
    const FName& MutationId
) const
{
    const FSharProgressionMutationSnapshot* Mutation =
        FindMutation(MutationId);
    if (Mutation == nullptr || Mutation->bReleased
        || Mutation->State != ESharProgressionMutationState::Queued)
    {
        return 0;
    }
    int32 Position = 1;
    for (const FSharProgressionMutationSnapshot& Other : Mutations)
    {
        const bool bComparable =
            !Other.bReleased
            && Other.State == ESharProgressionMutationState::Queued
            && Other.Request.MutationId != Mutation->Request.MutationId;
        Position += bComparable && Outranks(Other, *Mutation) ? 1 : 0;
    }
    return Position;
}
