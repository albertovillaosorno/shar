// File: SharProgressionContracts.h
// Path: src/uproject/Source/SharProgression/Public/Progression/SharProgressionContracts.h
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: reflected profile identity, progression snapshot, mutation, commit evidence, lifecycle, and immutable projection records only; no subsystem behavior.
// Specification: docs/technical/unreal/progression-collectibles-and-cheats.md
// LARGE-FILE owner=SharProgression; reason=cohesive reflected profile and progression transaction value contract;
// split=extract projection records when campaign and gallery query families are implemented;
// validation=validate.sh SharProgression plus Unreal automation; review=2027-01.

#pragma once

#include "CoreMinimal.h"
#include "Progression/SharProgressionState.h"

#include "SharProgressionContracts.generated.h"

UENUM(BlueprintType)
enum class ESharProfileLifecycleState : uint8
{
    Unconfigured,
    Ready,
    Failed,
};

UENUM(BlueprintType)
enum class ESharProgressionMutationPriority : uint8
{
    Background,
    Gameplay,
    User,
    Recovery,
};

UENUM(BlueprintType)
enum class ESharProgressionMutationState : uint8
{
    Queued,
    Preparing,
    AwaitingCommit,
    Completed,
    Failed,
    Cancelled,
    Released,
};

UENUM(BlueprintType)
enum class ESharProgressionTerminalResult : uint8
{
    None,
    Success,
    Failed,
    Cancelled,
};

UENUM(BlueprintType)
enum class ESharProgressionResolutionCommand : uint8
{
    Fail,
    Cancel,
};

UENUM(BlueprintType)
enum class ESharProgressionMutationResult : uint8
{
    Accepted,
    InvalidRequest,
    CatalogMissing,
    CatalogInactive,
    CatalogDefinitionMissing,
    ProfileNotReady,
    DuplicateMutation,
    ConflictingMutation,
    NotFound,
    NotHead,
    StaleRevision,
    InvalidState,
    UnsupportedOperation,
    PolicyViolation,
    AlreadyApplied,
    QuantityOverflow,
    AlreadyTerminal,
    Released,
};

USTRUCT(BlueprintType)
struct SHARPROGRESSION_API FSharProfileIdentity
{
    GENERATED_BODY()

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Identity")
    FName ProfileId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString ProfileRevision;
};

USTRUCT(BlueprintType)
struct SHARPROGRESSION_API FSharProgressionSnapshot
{
    GENERATED_BODY()

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Profile")
    FSharProfileIdentity Profile;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Catalog")
    FName CatalogId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Catalog")
    FString CatalogRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Save")
    FString SaveRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString SnapshotRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Schema")
    int32 SchemaVersion = 0;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "State")
    TArray<FSharProgressionValue> Values;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "State")
    TArray<FName> AppliedPermanentTransactions;
};

USTRUCT(BlueprintType)
struct SHARPROGRESSION_API FSharProgressionMutationRequest
{
    GENERATED_BODY()

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Identity")
    FName MutationId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Priority")
    ESharProgressionMutationPriority Priority =
        ESharProgressionMutationPriority::Gameplay;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Profile")
    FSharProfileIdentity Profile;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Catalog")
    FName CatalogId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString ExpectedCatalogRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString ExpectedSaveRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString ExpectedSnapshotRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString MutationRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Operation")
    TArray<FSharRewardRequest> Operations;
};

USTRUCT(BlueprintType)
struct SHARPROGRESSION_API FSharProgressionCommitEvidence
{
    GENERATED_BODY()

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Identity")
    FName MutationId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Profile")
    FSharProfileIdentity Profile;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString CatalogRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString ExpectedSaveRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString ExpectedSnapshotRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString MutationRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString ResultingSaveRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString ResultingSnapshotRevision;
};

USTRUCT(BlueprintType)
struct SHARPROGRESSION_API FSharProgressionMutationResolution
{
    GENERATED_BODY()

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Identity")
    FName MutationId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Result")
    ESharProgressionResolutionCommand Command =
        ESharProgressionResolutionCommand::Fail;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString ProfileRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString MutationRevision;
};

USTRUCT(BlueprintType)
struct SHARPROGRESSION_API FSharProgressionMutationSnapshot
{
    GENERATED_BODY()

    UPROPERTY(BlueprintReadOnly, Category = "Request")
    FSharProgressionMutationRequest Request;

    UPROPERTY(BlueprintReadOnly, Category = "State")
    ESharProgressionMutationState State =
        ESharProgressionMutationState::Queued;

    UPROPERTY(BlueprintReadOnly, Category = "Result")
    ESharProgressionTerminalResult TerminalResult =
        ESharProgressionTerminalResult::None;

    UPROPERTY(BlueprintReadOnly, Category = "Ordering")
    int32 InsertionSequence = 0;

    UPROPERTY(BlueprintReadOnly, Category = "Candidate")
    FSharProgressionSnapshot CandidateSnapshot;

    UPROPERTY(BlueprintReadOnly, Category = "Lifetime")
    bool bReleased = false;
};

USTRUCT(BlueprintType)
struct SHARPROGRESSION_API FSharProgressionObservation
{
    GENERATED_BODY()

    UPROPERTY(BlueprintReadOnly, Category = "Profile")
    ESharProfileLifecycleState ProfileState =
        ESharProfileLifecycleState::Unconfigured;

    UPROPERTY(BlueprintReadOnly, Category = "Snapshot")
    FSharProgressionSnapshot ActiveSnapshot;

    UPROPERTY(BlueprintReadOnly, Category = "Mutation")
    int32 UnreleasedMutationCount = 0;
};

USTRUCT(BlueprintType)
struct SHARPROGRESSION_API FSharProgressionCountQuery
{
    GENERATED_BODY()

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Operation")
    FName OperationId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Identity")
    TArray<FName> RequiredTargetIds;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Identity")
    TArray<FName> ExcludedTargetIds;
};

USTRUCT(BlueprintType)
struct SHARPROGRESSION_API FSharProgressionCountProjection
{
    GENERATED_BODY()

    UPROPERTY(BlueprintReadOnly, Category = "Count")
    int32 Numerator = 0;

    UPROPERTY(BlueprintReadOnly, Category = "Count")
    int32 Denominator = 0;

    UPROPERTY(BlueprintReadOnly, Category = "Count")
    bool bComplete = false;

    UPROPERTY(BlueprintReadOnly, Category = "Revision")
    FString CatalogRevision;

    UPROPERTY(BlueprintReadOnly, Category = "Revision")
    FString SaveRevision;

    UPROPERTY(BlueprintReadOnly, Category = "Revision")
    FString SnapshotRevision;
};
