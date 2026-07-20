// File: SharSaveRepositoryEvidence.cpp
// Path: src/uproject/Source/SharSave/Private/Save/SharSaveRepositoryEvidence.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: adapter evidence ordering, read-back verification, atomic replacement, load, recovery, and delete completion only.
// Specification: docs/technical/unreal/platform-save-storage-and-lifecycle.md
// LARGE-FILE owner=SharSave; reason=cohesive adapter-stage state machine and accepted-slot mutation;
// split=extract load and delete adapters if provider-specific evidence expands;
// validation=validate.sh SharSave plus Unreal automation; review=2027-01.

#include "Save/SharSaveRepositorySubsystem.h"
#include "Save/SharSaveContracts.h"

#include "Save/SharSaveSchemaCatalogSubsystem.h"
#include "Save/SharSaveSchemaDefinition.h"

static bool NameArraysMatch(
    const TArray<FName>& Left,
    const TArray<FName>& Right
)
{
    if (Left.Num() != Right.Num())
    {
        return false;
    }
    auto RightIterator = Right.begin();
    for (const FName& LeftIdentity : Left)
    {
        if (LeftIdentity != *RightIterator)
        {
            return false;
        }
        ++RightIterator;
    }
    return true;
}

static bool DocumentsMatch(
    const FSharSaveDocumentDescriptor& Left,
    const FSharSaveDocumentDescriptor& Right
)
{
    return Left.SchemaId == Right.SchemaId
        && Left.SchemaVersion == Right.SchemaVersion
        && Left.DocumentRevision == Right.DocumentRevision
        && Left.CatalogRevision == Right.CatalogRevision
        && Left.SnapshotRevision == Right.SnapshotRevision
        && NameArraysMatch(
            Left.ContentRequirementIds,
            Right.ContentRequirementIds
        )
        && NameArraysMatch(Left.SectionIds, Right.SectionIds)
        && Left.SerializedLength == Right.SerializedLength
        && Left.IntegrityRevision == Right.IntegrityRevision;
}

ESharSaveOperationResult
USharSaveRepositorySubsystem::AcceptCandidateWritten(
    FSharSaveOperationSnapshot& Snapshot,
    const FSharSaveAdapterEvidence& Evidence
)
{
    if (Snapshot.State != ESharSaveOperationState::Writing
        || !Snapshot.bCandidateAccepted
        || !DocumentsMatch(Snapshot.CandidateDocument, Evidence.Document))
    {
        return ESharSaveOperationResult::InvalidState;
    }
    Snapshot.bCandidateWritten = true;
    return ESharSaveOperationResult::Accepted;
}

ESharSaveOperationResult USharSaveRepositorySubsystem::AcceptDurableFlush(
    FSharSaveOperationSnapshot& Snapshot
)
{
    if (Snapshot.State != ESharSaveOperationState::Writing
        || !Snapshot.bCandidateWritten)
    {
        return ESharSaveOperationResult::InvalidState;
    }
    Snapshot.bDurableFlushCompleted = true;
    Snapshot.State = ESharSaveOperationState::Verifying;
    return ESharSaveOperationResult::Accepted;
}

ESharSaveOperationResult USharSaveRepositorySubsystem::AcceptReadBack(
    FSharSaveOperationSnapshot& Snapshot,
    const FSharSaveAdapterEvidence& Evidence
)
{
    if (Snapshot.State != ESharSaveOperationState::Verifying
        || !Snapshot.bDurableFlushCompleted
        || !DocumentsMatch(Snapshot.CandidateDocument, Evidence.Document))
    {
        return ESharSaveOperationResult::IntegrityMismatch;
    }
    Snapshot.ResultDocument = Evidence.Document;
    Snapshot.bReadBackValidated = true;
    Snapshot.State = ESharSaveOperationState::Committing;
    return ESharSaveOperationResult::Accepted;
}

ESharSaveOperationResult USharSaveRepositorySubsystem::AcceptAtomicReplace(
    FSharSaveOperationSnapshot& Snapshot,
    const FSharSaveAdapterEvidence& Evidence
)
{
    const bool bInvalid =
        Snapshot.State != ESharSaveOperationState::Committing
        || !Snapshot.bReadBackValidated
        || !IsRevisionToken(Evidence.ResultingAcceptedRevision)
        || Evidence.ResultingAcceptedRevision
            != Snapshot.CandidateDocument.DocumentRevision;
    if (bInvalid)
    {
        return ESharSaveOperationResult::InvalidState;
    }
    FSharSaveSlotState* SlotState = FindSlot(Snapshot.Request.Slot);
    if (SlotState == nullptr)
    {
        return ESharSaveOperationResult::SlotMissing;
    }
    SlotState->AcceptedRevision = Evidence.ResultingAcceptedRevision;
    SlotState->ContainerRevision = Evidence.ContainerRevision;
    SlotState->SchemaId = Snapshot.CandidateDocument.SchemaId;
    SlotState->SchemaVersion = Snapshot.CandidateDocument.SchemaVersion;
    SlotState->IntegrityRevision =
        Snapshot.CandidateDocument.IntegrityRevision;
    SlotState->bOccupied = true;
    Snapshot.bAcceptedRevisionReplaced = true;
    return PublishTerminal(
        Snapshot,
        ESharSaveOperationState::Completed,
        ESharSaveTerminalResult::Success
    );
}

ESharSaveOperationResult USharSaveRepositorySubsystem::AcceptSaveEvidence(
    FSharSaveOperationSnapshot& Snapshot,
    const FSharSaveAdapterEvidence& Evidence
)
{
    switch (Evidence.Stage)
    {
    case ESharSaveAdapterStage::CandidateWritten:
        return AcceptCandidateWritten(Snapshot, Evidence);
    case ESharSaveAdapterStage::DurableFlushCompleted:
        return AcceptDurableFlush(Snapshot);
    case ESharSaveAdapterStage::ReadBackValidated:
        return AcceptReadBack(Snapshot, Evidence);
    case ESharSaveAdapterStage::AtomicReplaceCompleted:
        return AcceptAtomicReplace(Snapshot, Evidence);
    case ESharSaveAdapterStage::ReadCompleted:
    case ESharSaveAdapterStage::DeleteCompleted:
    case ESharSaveAdapterStage::RecoveryCompleted:
    default:
        return ESharSaveOperationResult::InvalidState;
    }
}

ESharSaveOperationResult USharSaveRepositorySubsystem::AcceptReadEvidence(
    FSharSaveOperationSnapshot& Snapshot,
    const FSharSaveAdapterEvidence& Evidence
)
{
    const ESharSaveAdapterStage ExpectedStage =
        Snapshot.Request.Kind == ESharSaveOperationKind::Recover
        ? ESharSaveAdapterStage::RecoveryCompleted
        : ESharSaveAdapterStage::ReadCompleted;
    if (Snapshot.State != ESharSaveOperationState::Reading
        || Evidence.Stage != ExpectedStage)
    {
        return ESharSaveOperationResult::InvalidState;
    }
    const USharSaveSchemaDefinition* Schema =
        SchemaCatalog->FindSchema(Snapshot.Request.SchemaId);
    if (Schema == nullptr)
    {
        return ESharSaveOperationResult::SchemaMissing;
    }
    const FSharSaveSlotState* SlotState = FindSlot(Snapshot.Request.Slot);
    if (SlotState == nullptr || !SlotState->bOccupied)
    {
        return ESharSaveOperationResult::SlotMissing;
    }
    const bool bInvalidRevision =
        Evidence.ResultingAcceptedRevision != SlotState->AcceptedRevision
        || Evidence.Document.DocumentRevision != SlotState->AcceptedRevision;
    if (bInvalidRevision
        || !ValidateDocument(Evidence.Document, *Schema, false))
    {
        return ESharSaveOperationResult::IntegrityMismatch;
    }
    Snapshot.ResultDocument = Evidence.Document;
    return PublishTerminal(
        Snapshot,
        ESharSaveOperationState::Completed,
        ESharSaveTerminalResult::Success
    );
}

ESharSaveOperationResult USharSaveRepositorySubsystem::AcceptDeleteEvidence(
    FSharSaveOperationSnapshot& Snapshot,
    const FSharSaveAdapterEvidence& Evidence
)
{
    if (Snapshot.State != ESharSaveOperationState::Deleting
        || Evidence.Stage != ESharSaveAdapterStage::DeleteCompleted
        || !IsRevisionToken(Evidence.ResultingAcceptedRevision))
    {
        return ESharSaveOperationResult::InvalidState;
    }
    FSharSaveSlotState* SlotState = FindSlot(Snapshot.Request.Slot);
    if (SlotState == nullptr || !SlotState->bOccupied)
    {
        return ESharSaveOperationResult::SlotMissing;
    }
    SlotState->AcceptedRevision = Evidence.ResultingAcceptedRevision;
    SlotState->ContainerRevision = Evidence.ContainerRevision;
    SlotState->SchemaId = FName();
    SlotState->SchemaVersion = 0;
    SlotState->IntegrityRevision = FString();
    SlotState->bOccupied = false;
    return PublishTerminal(
        Snapshot,
        ESharSaveOperationState::Completed,
        ESharSaveTerminalResult::Success
    );
}

ESharSaveOperationResult USharSaveRepositorySubsystem::AcceptAdapterEvidence(
    const FSharSaveAdapterEvidence& Evidence
)
{
    FSharSaveOperationSnapshot* Snapshot = FindOperation(Evidence.OperationId);
    if (Snapshot == nullptr)
    {
        return ESharSaveOperationResult::NotFound;
    }
    if (Snapshot->bReleased)
    {
        return ESharSaveOperationResult::Released;
    }
    if (IsTerminalState(Snapshot->State))
    {
        return ESharSaveOperationResult::AlreadyTerminal;
    }
    if (!EvidenceMatches(*Snapshot, Evidence))
    {
        return ESharSaveOperationResult::StaleRevision;
    }
    switch (Snapshot->Request.Kind)
    {
    case ESharSaveOperationKind::Save:
        return AcceptSaveEvidence(*Snapshot, Evidence);
    case ESharSaveOperationKind::Load:
    case ESharSaveOperationKind::Recover:
        return AcceptReadEvidence(*Snapshot, Evidence);
    case ESharSaveOperationKind::Delete:
        return AcceptDeleteEvidence(*Snapshot, Evidence);
    default:
        return ESharSaveOperationResult::InvalidRequest;
    }
}
