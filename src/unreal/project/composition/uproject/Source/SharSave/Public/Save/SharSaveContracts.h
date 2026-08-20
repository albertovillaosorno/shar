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
//   - Shar save contracts composition module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Shar save contracts composition module.
// - Description:
//   - Implements the declared composition module responsibility for project.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Shar save contracts composition module.

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
