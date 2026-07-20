// File: SharSaveRepositoryQueue.cpp
// Path: src/uproject/Source/SharSave/Private/Save/SharSaveRepositoryQueue.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: save repository configuration, slot registration, request validation, ordering, submission, and start only.
// Specification: docs/technical/unreal/platform-save-storage-and-lifecycle.md
// LARGE-FILE owner=SharSave; reason=cohesive save-operation admission and arbitration;
// split=extract provider capacity policy when quota becomes implemented;
// validation=validate.sh SharSave plus Unreal automation; review=2027-01.

#include "Save/SharSaveRepositorySubsystem.h"
#include "Save/SharSaveContracts.h"

#include "Algo/AnyOf.h"
#include "Algo/Find.h"
#include "Content/SharPrimaryContentDefinition.h"
#include "Save/SharSaveSchemaCatalogSubsystem.h"

static constexpr int32 MaximumPendingSaveOperations = 32;

static bool IsCanonicalSaveIdentity(const FName& Candidate)
{
    return USharPrimaryContentDefinition::IsCanonicalIdentifier(Candidate);
}

bool USharSaveRepositorySubsystem::SlotIdsMatch(
    const FSharSaveSlotId& Left,
    const FSharSaveSlotId& Right
)
{
    return Left.ProfileId == Right.ProfileId && Left.SlotId == Right.SlotId;
}

bool USharSaveRepositorySubsystem::IsCanonicalSlot(
    const FSharSaveSlotId& Slot
)
{
    return IsCanonicalSaveIdentity(Slot.ProfileId)
        && IsCanonicalSaveIdentity(Slot.SlotId);
}

bool USharSaveRepositorySubsystem::IsRevisionToken(
    const FString& Revision
)
{
    return Revision.StartsWith(TEXT("sha256:"));
}

bool USharSaveRepositorySubsystem::IsTerminalState(
    const ESharSaveOperationState State
)
{
    return State == ESharSaveOperationState::Completed
        || State == ESharSaveOperationState::Failed
        || State == ESharSaveOperationState::TimedOut
        || State == ESharSaveOperationState::Cancelled;
}

bool USharSaveRepositorySubsystem::IsValidOperationSpec(
    const FSharSaveOperationRequest& Request
)
{
    const bool bInvalidIdentity =
        !IsCanonicalSaveIdentity(Request.OperationId)
        || !IsCanonicalSlot(Request.Slot)
        || !IsCanonicalSaveIdentity(Request.SchemaId)
        || !IsCanonicalSaveIdentity(Request.ProviderId);
    const bool bInvalidRevision =
        !IsRevisionToken(Request.ExpectedAcceptedRevision)
        || !IsRevisionToken(Request.CatalogRevision)
        || !IsRevisionToken(Request.ContainerRevision)
        || !IsRevisionToken(Request.OperationRevision);
    const bool bInvalidDeadline =
        !FMath::IsFinite(Request.DeadlineSeconds)
        || Request.DeadlineSeconds <= 0.0;
    return !bInvalidIdentity && !bInvalidRevision && !bInvalidDeadline;
}

FSharSaveSlotState* USharSaveRepositorySubsystem::FindSlot(
    const FSharSaveSlotId& Slot
)
{
    return Algo::FindByPredicate(
        Slots,
        [&Slot](const FSharSaveSlotState& Candidate)
        {
            return SlotIdsMatch(Candidate.Slot, Slot);
        }
    );
}

const FSharSaveSlotState* USharSaveRepositorySubsystem::FindSlot(
    const FSharSaveSlotId& Slot
) const
{
    return Algo::FindByPredicate(
        Slots,
        [&Slot](const FSharSaveSlotState& Candidate)
        {
            return SlotIdsMatch(Candidate.Slot, Slot);
        }
    );
}

FSharSaveOperationSnapshot* USharSaveRepositorySubsystem::FindOperation(
    const FName& OperationId
)
{
    return Algo::FindByPredicate(
        Operations,
        [&OperationId](const FSharSaveOperationSnapshot& Snapshot)
        {
            return Snapshot.Request.OperationId == OperationId;
        }
    );
}

const FSharSaveOperationSnapshot* USharSaveRepositorySubsystem::FindOperation(
    const FName& OperationId
) const
{
    return Algo::FindByPredicate(
        Operations,
        [&OperationId](const FSharSaveOperationSnapshot& Snapshot)
        {
            return Snapshot.Request.OperationId == OperationId;
        }
    );
}

bool USharSaveRepositorySubsystem::Outranks(
    const FSharSaveOperationSnapshot& Left,
    const FSharSaveOperationSnapshot& Right
)
{
    const auto LeftPriority = static_cast<uint8>(Left.Request.Priority);
    const auto RightPriority = static_cast<uint8>(Right.Request.Priority);
    if (LeftPriority != RightPriority)
    {
        return LeftPriority > RightPriority;
    }
    return Left.Request.OperationId.LexicalLess(Right.Request.OperationId);
}

bool USharSaveRepositorySubsystem::IsHead(
    const FSharSaveOperationSnapshot& Snapshot
) const
{
    if (Snapshot.bReleased || Snapshot.State != ESharSaveOperationState::Queued)
    {
        return false;
    }
    return !Algo::AnyOf(
        Operations,
        [&Snapshot](const FSharSaveOperationSnapshot& Other)
        {
            const bool bComparable =
                !Other.bReleased
                && Other.State == ESharSaveOperationState::Queued
                && Other.Request.OperationId
                    != Snapshot.Request.OperationId;
            return bComparable && Outranks(Other, Snapshot);
        }
    );
}

bool USharSaveRepositorySubsystem::HasConflictingActiveOperation(
    const FSharSaveOperationSnapshot& Snapshot
) const
{
    return Algo::AnyOf(
        Operations,
        [&Snapshot](const FSharSaveOperationSnapshot& Other)
        {
            return !Other.bReleased
                && Other.Request.OperationId != Snapshot.Request.OperationId
                && SlotIdsMatch(Other.Request.Slot, Snapshot.Request.Slot)
                && Other.State != ESharSaveOperationState::Queued
                && !IsTerminalState(Other.State);
        }
    );
}

bool USharSaveRepositorySubsystem::Configure(
    USharSaveSchemaCatalogSubsystem* InSchemaCatalog,
    const FName& InProviderId,
    const FString& InContainerRevision
)
{
    const bool bInvalid =
        InSchemaCatalog == nullptr
        || !InSchemaCatalog->IsActive()
        || !IsCanonicalSaveIdentity(InProviderId)
        || !IsRevisionToken(InContainerRevision);
    if (bInvalid)
    {
        return false;
    }
    SchemaCatalog = InSchemaCatalog;
    ProviderId = InProviderId;
    ContainerRevision = InContainerRevision;
    Slots.Reset();
    Operations.Reset();
    NextInsertionSequence = 0;
    return true;
}

ESharSaveOperationResult USharSaveRepositorySubsystem::RegisterSlot(
    const FSharSaveSlotState& SlotState
)
{
    if (SchemaCatalog == nullptr)
    {
        return ESharSaveOperationResult::CatalogMissing;
    }
    const bool bInvalidBase =
        !IsCanonicalSlot(SlotState.Slot)
        || !IsRevisionToken(SlotState.AcceptedRevision)
        || !IsRevisionToken(SlotState.ContainerRevision);
    const bool bInvalidOccupied =
        SlotState.bOccupied
        && (!IsCanonicalSaveIdentity(SlotState.SchemaId)
            || SlotState.SchemaVersion <= 0
            || !IsRevisionToken(SlotState.IntegrityRevision)
            || SchemaCatalog->FindSchema(SlotState.SchemaId) == nullptr);
    if (bInvalidBase || bInvalidOccupied)
    {
        return ESharSaveOperationResult::InvalidRequest;
    }
    if (FindSlot(SlotState.Slot) != nullptr)
    {
        return ESharSaveOperationResult::DuplicateSlot;
    }
    Slots.Add(SlotState);
    return ESharSaveOperationResult::Accepted;
}

ESharSaveOperationResult
USharSaveRepositorySubsystem::ClassifyCatalogOperation(
    const FSharSaveOperationRequest& Request
) const
{
    if (SchemaCatalog == nullptr)
    {
        return ESharSaveOperationResult::CatalogMissing;
    }
    if (!SchemaCatalog->IsActive())
    {
        return ESharSaveOperationResult::CatalogInactive;
    }
    if (!IsValidOperationSpec(Request) || Request.ProviderId != ProviderId)
    {
        return ESharSaveOperationResult::InvalidRequest;
    }
    if (Request.CatalogRevision != SchemaCatalog->GetCatalogRevision()
        || Request.ContainerRevision != ContainerRevision)
    {
        return ESharSaveOperationResult::StaleRevision;
    }
    return SchemaCatalog->FindSchema(Request.SchemaId) == nullptr
        ? ESharSaveOperationResult::SchemaMissing
        : ESharSaveOperationResult::Accepted;
}

ESharSaveOperationResult
USharSaveRepositorySubsystem::ClassifySlotOperation(
    const FSharSaveOperationRequest& Request
) const
{
    const FSharSaveSlotState* SlotState = FindSlot(Request.Slot);
    if (SlotState == nullptr)
    {
        return ESharSaveOperationResult::SlotMissing;
    }
    if (SlotState->AcceptedRevision != Request.ExpectedAcceptedRevision
        || SlotState->ContainerRevision != Request.ContainerRevision)
    {
        return ESharSaveOperationResult::StaleRevision;
    }
    const bool bRequiresOccupied =
        Request.Kind == ESharSaveOperationKind::Load
        || Request.Kind == ESharSaveOperationKind::Delete
        || Request.Kind == ESharSaveOperationKind::Recover;
    return bRequiresOccupied && !SlotState->bOccupied
        ? ESharSaveOperationResult::SlotMissing
        : ESharSaveOperationResult::Accepted;
}

bool USharSaveRepositorySubsystem::HasSlotConflict(
    const FSharSaveOperationRequest& OperationSpec
) const
{
    return Algo::AnyOf(
        Operations,
        [&OperationSpec](const FSharSaveOperationSnapshot& Snapshot)
        {
            return !Snapshot.bReleased
                && !IsTerminalState(Snapshot.State)
                && SlotIdsMatch(
                    Snapshot.Request.Slot,
                    OperationSpec.Slot
                );
        }
    );
}

ESharSaveOperationResult USharSaveRepositorySubsystem::ClassifyOperationAdmission(
    const FSharSaveOperationRequest& Request
) const
{
    const ESharSaveOperationResult CatalogResult =
        ClassifyCatalogOperation(Request);
    if (CatalogResult != ESharSaveOperationResult::Accepted)
    {
        return CatalogResult;
    }
    const ESharSaveOperationResult SlotResult = ClassifySlotOperation(Request);
    if (SlotResult != ESharSaveOperationResult::Accepted)
    {
        return SlotResult;
    }
    if (FindOperation(Request.OperationId) != nullptr)
    {
        return ESharSaveOperationResult::DuplicateOperation;
    }
    return HasSlotConflict(Request)
        ? ESharSaveOperationResult::ConflictingOperation
        : ESharSaveOperationResult::Accepted;
}

int32 USharSaveRepositorySubsystem::CountPendingOperations() const
{
    int32 PendingCount = 0;
    for (const FSharSaveOperationSnapshot& Snapshot : Operations)
    {
        PendingCount += !Snapshot.bReleased
                && Snapshot.State == ESharSaveOperationState::Queued
            ? 1
            : 0;
    }
    return PendingCount;
}

ESharSaveOperationResult USharSaveRepositorySubsystem::Submit(
    const FSharSaveOperationRequest& Request
)
{
    const ESharSaveOperationResult Classification =
        ClassifyOperationAdmission(Request);
    if (Classification != ESharSaveOperationResult::Accepted)
    {
        return Classification;
    }
    if (CountPendingOperations() >= MaximumPendingSaveOperations)
    {
        return ESharSaveOperationResult::ConflictingOperation;
    }
    FSharSaveOperationSnapshot Snapshot;
    Snapshot.Request = Request;
    Snapshot.InsertionSequence = NextInsertionSequence;
    ++NextInsertionSequence;
    Operations.Add(Snapshot);
    return ESharSaveOperationResult::Accepted;
}

ESharSaveOperationResult USharSaveRepositorySubsystem::Begin(
    const FName& OperationId
)
{
    FSharSaveOperationSnapshot* Snapshot = FindOperation(OperationId);
    if (Snapshot == nullptr)
    {
        return ESharSaveOperationResult::NotFound;
    }
    if (Snapshot->bReleased)
    {
        return ESharSaveOperationResult::Released;
    }
    if (Snapshot->State != ESharSaveOperationState::Queued)
    {
        return ESharSaveOperationResult::InvalidState;
    }
    if (!IsHead(*Snapshot))
    {
        return ESharSaveOperationResult::NotHead;
    }
    if (HasConflictingActiveOperation(*Snapshot))
    {
        return ESharSaveOperationResult::ConflictingOperation;
    }
    const FSharSaveSlotState* SlotState = FindSlot(Snapshot->Request.Slot);
    if (SlotState == nullptr
        || SlotState->AcceptedRevision
            != Snapshot->Request.ExpectedAcceptedRevision
        || SlotState->ContainerRevision
            != Snapshot->Request.ContainerRevision)
    {
        return ESharSaveOperationResult::StaleRevision;
    }
    switch (Snapshot->Request.Kind)
    {
    case ESharSaveOperationKind::Save:
        Snapshot->State = ESharSaveOperationState::Preparing;
        break;
    case ESharSaveOperationKind::Load:
    case ESharSaveOperationKind::Recover:
        Snapshot->State = ESharSaveOperationState::Reading;
        break;
    case ESharSaveOperationKind::Delete:
        Snapshot->State = ESharSaveOperationState::Deleting;
        break;
    default:
        return ESharSaveOperationResult::InvalidRequest;
    }
    return ESharSaveOperationResult::Accepted;
}

int32 USharSaveRepositorySubsystem::GetQueuePosition(
    const FName& OperationId
) const
{
    const FSharSaveOperationSnapshot* Snapshot = FindOperation(OperationId);
    if (Snapshot == nullptr || Snapshot->bReleased
        || Snapshot->State != ESharSaveOperationState::Queued)
    {
        return 0;
    }
    int32 Position = 1;
    for (const FSharSaveOperationSnapshot& Other : Operations)
    {
        const bool bComparable =
            !Other.bReleased
            && Other.State == ESharSaveOperationState::Queued
            && Other.Request.OperationId != Snapshot->Request.OperationId;
        Position += bComparable && Outranks(Other, *Snapshot) ? 1 : 0;
    }
    return Position;
}
