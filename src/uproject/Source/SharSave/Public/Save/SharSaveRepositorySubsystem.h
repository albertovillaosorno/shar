// File: SharSaveRepositorySubsystem.h
// Path: src/uproject/Source/SharSave/Public/Save/SharSaveRepositorySubsystem.h
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: game-instance logical save-slot and transaction coordination only; serialization bytes, physical storage, platform accounts, domain snapshots, and UI remain external.
// Specification: docs/technical/unreal/platform-save-storage-and-lifecycle.md
// LARGE-FILE owner=SharSave; reason=cohesive reflected save operation and slot-state contract;
// split=extract provider-state projections when quota and remediation become implemented;
// validation=validate.sh SharSave plus Unreal automation; review=2027-01.

#pragma once

#include "CoreMinimal.h"
#include "Subsystems/GameInstanceSubsystem.h"

#include "Save/SharSaveContracts.h"
#include "Save/SharSaveSchemaCatalogSubsystem.h"
#include "SharSaveRepositorySubsystem.generated.h"

UCLASS()
class SHARSAVE_API USharSaveRepositorySubsystem final
    : public UGameInstanceSubsystem
{
    GENERATED_BODY()

public:
    UFUNCTION(BlueprintCallable, Category = "SHAR|Save")
    bool Configure(
        USharSaveSchemaCatalogSubsystem* InSchemaCatalog,
        const FName& InProviderId,
        const FString& InContainerRevision
    );

    UFUNCTION(BlueprintCallable, Category = "SHAR|Save")
    ESharSaveOperationResult RegisterSlot(
        const FSharSaveSlotState& SlotState
    );

    UFUNCTION(BlueprintCallable, Category = "SHAR|Save")
    ESharSaveOperationResult Submit(
        const FSharSaveOperationRequest& Request
    );

    ESharSaveOperationResult Begin(const FName& OperationId);

    ESharSaveOperationResult AcceptCandidate(
        const FName& OperationId,
        const FSharSaveDocumentDescriptor& Candidate
    );

    ESharSaveOperationResult AcceptAdapterEvidence(
        const FSharSaveAdapterEvidence& Evidence
    );

    ESharSaveOperationResult Resolve(
        const FSharSaveOperationResolution& Resolution
    );

    ESharSaveOperationResult Release(const FName& OperationId);

    UFUNCTION(BlueprintPure, Category = "SHAR|Save")
    [[nodiscard]] int32 GetQueuePosition(const FName& OperationId) const;

    UFUNCTION(BlueprintPure, Category = "SHAR|Save")
    [[nodiscard]] ESharSaveOperationState GetState(
        const FName& OperationId
    ) const;

    UFUNCTION(BlueprintPure, Category = "SHAR|Save")
    [[nodiscard]] ESharSaveTerminalResult GetTerminalResult(
        const FName& OperationId
    ) const;

    UFUNCTION(BlueprintPure, Category = "SHAR|Save")
    [[nodiscard]] FSharSaveSlotState GetSlotState(
        const FSharSaveSlotId& Slot
    ) const;

    UFUNCTION(BlueprintPure, Category = "SHAR|Save")
    [[nodiscard]] int32 GetUnreleasedOperationCount() const;

private:
    UPROPERTY(Transient)
    USharSaveSchemaCatalogSubsystem* SchemaCatalog = nullptr;

    UPROPERTY(Transient)
    FName ProviderId;

    UPROPERTY(Transient)
    FString ContainerRevision;

    UPROPERTY(Transient)
    TArray<FSharSaveSlotState> Slots;

    UPROPERTY(Transient)
    TArray<FSharSaveOperationSnapshot> Operations;

    UPROPERTY(Transient)
    int32 NextInsertionSequence = 0;

    [[nodiscard]] static bool SlotIdsMatch(
        const FSharSaveSlotId& Left,
        const FSharSaveSlotId& Right
    );
    [[nodiscard]] static bool IsCanonicalSlot(
        const FSharSaveSlotId& Slot
    );
    [[nodiscard]] static bool IsRevisionToken(const FString& Revision);
    [[nodiscard]] static bool IsTerminalState(ESharSaveOperationState State);
    [[nodiscard]] static bool IsValidOperationSpec(
        const FSharSaveOperationRequest& Request
    );
    [[nodiscard]] static bool Outranks(
        const FSharSaveOperationSnapshot& Left,
        const FSharSaveOperationSnapshot& Right
    );
    [[nodiscard]] FSharSaveSlotState* FindSlot(const FSharSaveSlotId& Slot);
    [[nodiscard]] const FSharSaveSlotState* FindSlot(
        const FSharSaveSlotId& Slot
    ) const;
    [[nodiscard]] FSharSaveOperationSnapshot* FindOperation(
        const FName& OperationId
    );
    [[nodiscard]] const FSharSaveOperationSnapshot* FindOperation(
        const FName& OperationId
    ) const;
    [[nodiscard]] bool IsHead(
        const FSharSaveOperationSnapshot& Snapshot
    ) const;
    [[nodiscard]] bool HasConflictingActiveOperation(
        const FSharSaveOperationSnapshot& Snapshot
    ) const;
    [[nodiscard]] ESharSaveOperationResult ClassifyOperationAdmission(
        const FSharSaveOperationRequest& Request
    ) const;
    [[nodiscard]] ESharSaveOperationResult ClassifyCatalogOperation(
        const FSharSaveOperationRequest& Request
    ) const;
    [[nodiscard]] ESharSaveOperationResult ClassifySlotOperation(
        const FSharSaveOperationRequest& Request
    ) const;
    [[nodiscard]] bool HasSlotConflict(
        const FSharSaveOperationRequest& OperationSpec
    ) const;
    [[nodiscard]] int32 CountPendingOperations() const;
    [[nodiscard]] bool ValidateDocument(
        const FSharSaveDocumentDescriptor& Document,
        const USharSaveSchemaDefinition& Schema,
        bool bRequireCurrentVersion
    ) const;
    [[nodiscard]] static bool EvidenceMatches(
        const FSharSaveOperationSnapshot& Snapshot,
        const FSharSaveAdapterEvidence& Evidence
    );
    [[nodiscard]] static ESharSaveOperationResult PublishTerminal(
        FSharSaveOperationSnapshot& Snapshot,
        ESharSaveOperationState State,
        ESharSaveTerminalResult Result
    );
    ESharSaveOperationResult AcceptSaveEvidence(
        FSharSaveOperationSnapshot& Snapshot,
        const FSharSaveAdapterEvidence& Evidence
    );
    static ESharSaveOperationResult AcceptCandidateWritten(
        FSharSaveOperationSnapshot& Snapshot,
        const FSharSaveAdapterEvidence& Evidence
    );
    static ESharSaveOperationResult AcceptDurableFlush(
        FSharSaveOperationSnapshot& Snapshot
    );
    static ESharSaveOperationResult AcceptReadBack(
        FSharSaveOperationSnapshot& Snapshot,
        const FSharSaveAdapterEvidence& Evidence
    );
    ESharSaveOperationResult AcceptAtomicReplace(
        FSharSaveOperationSnapshot& Snapshot,
        const FSharSaveAdapterEvidence& Evidence
    );
    ESharSaveOperationResult AcceptReadEvidence(
        FSharSaveOperationSnapshot& Snapshot,
        const FSharSaveAdapterEvidence& Evidence
    );
    ESharSaveOperationResult AcceptDeleteEvidence(
        FSharSaveOperationSnapshot& Snapshot,
        const FSharSaveAdapterEvidence& Evidence
    );
};
