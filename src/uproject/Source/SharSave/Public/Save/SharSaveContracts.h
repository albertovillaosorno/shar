// File: SharSaveContracts.h
// Path: src/uproject/Source/SharSave/Public/Save/SharSaveContracts.h
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: reflected portable save slot, document, operation, evidence, and result records only; no repository behavior.
// Specification: docs/technical/unreal/platform-save-storage-and-lifecycle.md
// LARGE-FILE owner=SharSave; reason=cohesive reflected save transaction value contract;
// split=extract provider remediation records when quota and permissions are implemented;
// validation=validate.sh SharSave plus Unreal automation; review=2027-01.

#pragma once

#include "CoreMinimal.h"

#include "SharSaveContracts.generated.h"

UENUM(BlueprintType)
enum class ESharSaveOperationKind : uint8
{
    Save,
    Load,
    Delete,
    Recover,
};

UENUM(BlueprintType)
enum class ESharSaveOperationPriority : uint8
{
    Background,
    Autosave,
    Manual,
    LifecycleCritical,
};

UENUM(BlueprintType)
enum class ESharSaveOperationState : uint8
{
    Queued,
    Preparing,
    Reading,
    Writing,
    Verifying,
    Committing,
    Deleting,
    Completed,
    Failed,
    TimedOut,
    Cancelled,
    Released,
};

UENUM(BlueprintType)
enum class ESharSaveTerminalResult : uint8
{
    None,
    Success,
    Failed,
    TimedOut,
    Cancelled,
};

UENUM(BlueprintType)
enum class ESharSaveAdapterStage : uint8
{
    CandidateWritten,
    DurableFlushCompleted,
    ReadBackValidated,
    AtomicReplaceCompleted,
    ReadCompleted,
    DeleteCompleted,
    RecoveryCompleted,
};

UENUM(BlueprintType)
enum class ESharSaveResolutionCommand : uint8
{
    Fail,
    Timeout,
    Cancel,
};

UENUM(BlueprintType)
enum class ESharSaveOperationResult : uint8
{
    Accepted,
    InvalidRequest,
    CatalogMissing,
    CatalogInactive,
    SchemaMissing,
    SlotMissing,
    DuplicateSlot,
    DuplicateOperation,
    ConflictingOperation,
    NotFound,
    NotHead,
    StaleRevision,
    InvalidState,
    IntegrityMismatch,
    ContentRequirementInvalid,
    MigrationUnavailable,
    AlreadyTerminal,
    Released,
};

USTRUCT(BlueprintType)
struct SHARSAVE_API FSharSaveSlotId
{
    GENERATED_BODY()

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Identity")
    FName ProfileId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Identity")
    FName SlotId;
};

USTRUCT(BlueprintType)
struct SHARSAVE_API FSharSaveDocumentDescriptor
{
    GENERATED_BODY()

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Schema")
    FName SchemaId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Schema")
    int32 SchemaVersion = 0;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString DocumentRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString CatalogRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString SnapshotRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Content")
    TArray<FName> ContentRequirementIds;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Content")
    TArray<FName> SectionIds;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Integrity")
    int64 SerializedLength = 0;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Integrity")
    FString IntegrityRevision;
};

USTRUCT(BlueprintType)
struct SHARSAVE_API FSharSaveSlotState
{
    GENERATED_BODY()

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Identity")
    FSharSaveSlotId Slot;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString AcceptedRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString ContainerRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Schema")
    FName SchemaId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Schema")
    int32 SchemaVersion = 0;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Integrity")
    FString IntegrityRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "State")
    bool bOccupied = false;
};

USTRUCT(BlueprintType)
struct SHARSAVE_API FSharSaveOperationRequest
{
    GENERATED_BODY()

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Identity")
    FName OperationId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Operation")
    ESharSaveOperationKind Kind = ESharSaveOperationKind::Save;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Operation")
    ESharSaveOperationPriority Priority = ESharSaveOperationPriority::Manual;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Slot")
    FSharSaveSlotId Slot;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Schema")
    FName SchemaId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Provider")
    FName ProviderId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString ExpectedAcceptedRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString CatalogRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString ContainerRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString OperationRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Deadline")
    double DeadlineSeconds = 0.0;
};

USTRUCT(BlueprintType)
struct SHARSAVE_API FSharSaveAdapterEvidence
{
    GENERATED_BODY()

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Identity")
    FName OperationId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Slot")
    FSharSaveSlotId Slot;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Stage")
    ESharSaveAdapterStage Stage = ESharSaveAdapterStage::CandidateWritten;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString OperationRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString ContainerRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString ExpectedAcceptedRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString ResultingAcceptedRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Document")
    FSharSaveDocumentDescriptor Document;
};

USTRUCT(BlueprintType)
struct SHARSAVE_API FSharSaveOperationResolution
{
    GENERATED_BODY()

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Identity")
    FName OperationId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Result")
    ESharSaveResolutionCommand Command = ESharSaveResolutionCommand::Fail;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString OperationRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString ContainerRevision;
};

USTRUCT(BlueprintType)
struct SHARSAVE_API FSharSaveOperationSnapshot
{
    GENERATED_BODY()

    UPROPERTY(BlueprintReadOnly, Category = "Request")
    FSharSaveOperationRequest Request;

    UPROPERTY(BlueprintReadOnly, Category = "State")
    ESharSaveOperationState State = ESharSaveOperationState::Queued;

    UPROPERTY(BlueprintReadOnly, Category = "Result")
    ESharSaveTerminalResult TerminalResult = ESharSaveTerminalResult::None;

    UPROPERTY(BlueprintReadOnly, Category = "Ordering")
    int32 InsertionSequence = 0;

    UPROPERTY(BlueprintReadOnly, Category = "Document")
    FSharSaveDocumentDescriptor CandidateDocument;

    UPROPERTY(BlueprintReadOnly, Category = "Document")
    FSharSaveDocumentDescriptor ResultDocument;

    UPROPERTY(BlueprintReadOnly, Category = "Progress")
    bool bCandidateAccepted = false;

    UPROPERTY(BlueprintReadOnly, Category = "Progress")
    bool bCandidateWritten = false;

    UPROPERTY(BlueprintReadOnly, Category = "Progress")
    bool bDurableFlushCompleted = false;

    UPROPERTY(BlueprintReadOnly, Category = "Progress")
    bool bReadBackValidated = false;

    UPROPERTY(BlueprintReadOnly, Category = "Progress")
    bool bAcceptedRevisionReplaced = false;

    UPROPERTY(BlueprintReadOnly, Category = "Lifetime")
    bool bReleased = false;
};
