// File: SharApplicationModeCoordinator.h
// Path: src/uproject/Source/SharApplication/Public/Application/SharApplicationModeCoordinator.h
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: game-instance application-mode request arbitration and transition transactions only; world, save, input, audio, UI, loading, and presentation services retain state authority.
// Specification: docs/technical/unreal/application-lifecycle-and-mode-runtime.md
// LARGE-FILE owner=SharApplication; reason=cohesive reflected application-transition contract;
// split=extract diagnostics if transition history becomes persistent;
// validation=validate.sh SharApplication plus Unreal automation; review=2027-01.

#pragma once

#include "CoreMinimal.h"
#include "Subsystems/GameInstanceSubsystem.h"

#include "Application/SharApplicationModeCatalogSubsystem.h"
#include "SharApplicationModeCoordinator.generated.h"

UENUM(BlueprintType)
enum class ESharApplicationTransitionPriority : uint8
{
    Development,
    Gameplay,
    User,
    Suspension,
    Recovery,
    FatalExit,
};

UENUM(BlueprintType)
enum class ESharApplicationTransitionState : uint8
{
    Pending,
    Validating,
    Preparing,
    VerifyingReadiness,
    ReadyToCommit,
    Committed,
    VerifyingTarget,
    Success,
    Failed,
    Cancelled,
    Superseded,
    Recovered,
    Released,
};

UENUM(BlueprintType)
enum class ESharApplicationServiceStatus : uint8
{
    Ready,
    Degraded,
    Unavailable,
    Failed,
};

UENUM(BlueprintType)
enum class ESharApplicationTerminalResult : uint8
{
    None,
    Success,
    Failed,
    Cancelled,
    Superseded,
    Recovered,
};

UENUM(BlueprintType)
enum class ESharApplicationTransitionCommand : uint8
{
    Fail,
    Cancel,
    Supersede,
};

UENUM(BlueprintType)
enum class ESharApplicationOperationResult : uint8
{
    Accepted,
    InvalidRequest,
    CatalogMissing,
    CatalogInactive,
    ModeMissing,
    TransitionNotAllowed,
    DuplicateRequest,
    NotFound,
    NotHead,
    ConflictingTransition,
    StaleRevision,
    ServiceMissing,
    DuplicateEvidence,
    DependencyBlocked,
    InvalidState,
    AlreadyTerminal,
    RecoveryMissing,
    Released,
};

USTRUCT(BlueprintType)
struct SHARAPPLICATION_API FSharApplicationModeRequest
{
    GENERATED_BODY()

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Identity")
    FName RequestId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Mode")
    FName SourceModeId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Mode")
    FName TargetModeId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Reason")
    FName ReasonId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Caller")
    FName CallerId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Priority")
    ESharApplicationTransitionPriority Priority =
        ESharApplicationTransitionPriority::User;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString CatalogRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString SourceModeRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString TargetModeRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString SessionRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString ProfileRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString WorldRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString RequestRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Mode")
    FName ReturnModeId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Deadline")
    double DeadlineSeconds = 0.0;
};

USTRUCT(BlueprintType)
struct SHARAPPLICATION_API FSharApplicationServiceEvidence
{
    GENERATED_BODY()

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Identity")
    FName RequestId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Identity")
    FName ServiceId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Status")
    ESharApplicationServiceStatus Status =
        ESharApplicationServiceStatus::Ready;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString CatalogRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString RequestRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString ServiceRevision;
};

USTRUCT(BlueprintType)
struct SHARAPPLICATION_API FSharApplicationBarrierEvidence
{
    GENERATED_BODY()

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Identity")
    FName RequestId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Identity")
    FName BarrierId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString CatalogRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString RequestRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString TargetModeRevision;
};

USTRUCT(BlueprintType)
struct SHARAPPLICATION_API FSharApplicationTransitionResolution
{
    GENERATED_BODY()

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Identity")
    FName RequestId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Result")
    ESharApplicationTransitionCommand Command =
        ESharApplicationTransitionCommand::Fail;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString CatalogRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString RequestRevision;
};

USTRUCT(BlueprintType)
struct SHARAPPLICATION_API FSharApplicationModeObservation
{
    GENERATED_BODY()

    UPROPERTY(BlueprintReadOnly, Category = "Mode")
    FName ActiveModeId;

    UPROPERTY(BlueprintReadOnly, Category = "Revision")
    FString ActiveModeRevision;

    UPROPERTY(BlueprintReadOnly, Category = "Mode")
    FName WorldId;

    UPROPERTY(BlueprintReadOnly, Category = "Revision")
    FString WorldRevision;

    UPROPERTY(BlueprintReadOnly, Category = "Revision")
    FString ProfileRevision;

    UPROPERTY(BlueprintReadOnly, Category = "Revision")
    FString SessionRevision;

    UPROPERTY(BlueprintReadOnly, Category = "Transition")
    FName ActiveTransitionId;
};

USTRUCT(BlueprintType)
struct SHARAPPLICATION_API FSharApplicationTransitionSnapshot
{
    GENERATED_BODY()

    UPROPERTY(BlueprintReadOnly, Category = "Request")
    FSharApplicationModeRequest Request;

    UPROPERTY(BlueprintReadOnly, Category = "State")
    ESharApplicationTransitionState State =
        ESharApplicationTransitionState::Pending;

    UPROPERTY(BlueprintReadOnly, Category = "Result")
    ESharApplicationTerminalResult TerminalResult =
        ESharApplicationTerminalResult::None;

    UPROPERTY(BlueprintReadOnly, Category = "Ordering")
    int32 InsertionSequence = 0;

    UPROPERTY(BlueprintReadOnly, Category = "Evidence")
    TArray<FSharApplicationServiceEvidence> ServiceEvidence;

    UPROPERTY(BlueprintReadOnly, Category = "Evidence")
    bool bBarrierAccepted = false;

    UPROPERTY(BlueprintReadOnly, Category = "Commit")
    bool bCommitted = false;

    UPROPERTY(BlueprintReadOnly, Category = "Lifetime")
    bool bReleased = false;
};

UCLASS()
class SHARAPPLICATION_API USharApplicationModeCoordinator final
    : public UGameInstanceSubsystem
{
    GENERATED_BODY()

public:
    UFUNCTION(BlueprintCallable, Category = "SHAR|Application")
    bool Configure(
        USharApplicationModeCatalogSubsystem* InCatalog,
        const FSharApplicationModeObservation& InitialObservation
    );

    UFUNCTION(BlueprintCallable, Category = "SHAR|Application")
    ESharApplicationOperationResult Submit(
        const FSharApplicationModeRequest& Request
    );

    ESharApplicationOperationResult Begin(const FName& RequestId);

    ESharApplicationOperationResult RecordServiceEvidence(
        const FSharApplicationServiceEvidence& Evidence
    );

    ESharApplicationOperationResult BeginReadinessVerification(
        const FName& RequestId
    );

    ESharApplicationOperationResult AcceptBarrier(
        const FSharApplicationBarrierEvidence& Evidence
    );

    ESharApplicationOperationResult Commit(const FName& RequestId);

    ESharApplicationOperationResult Complete(const FName& RequestId);

    ESharApplicationOperationResult Resolve(
        const FSharApplicationTransitionResolution& Resolution
    );

    ESharApplicationOperationResult Release(const FName& RequestId);

    UFUNCTION(BlueprintPure, Category = "SHAR|Application")
    [[nodiscard]] int32 GetQueuePosition(const FName& RequestId) const;

    UFUNCTION(BlueprintPure, Category = "SHAR|Application")
    [[nodiscard]] ESharApplicationTransitionState GetState(
        const FName& RequestId
    ) const;

    UFUNCTION(BlueprintPure, Category = "SHAR|Application")
    [[nodiscard]] ESharApplicationTerminalResult GetTerminalResult(
        const FName& RequestId
    ) const;

    UFUNCTION(BlueprintPure, Category = "SHAR|Application")
    [[nodiscard]] FSharApplicationModeObservation GetObservation() const;

    UFUNCTION(BlueprintPure, Category = "SHAR|Application")
    [[nodiscard]] int32 GetUnreleasedTransitionCount() const;

private:
    UPROPERTY(Transient)
    USharApplicationModeCatalogSubsystem* Catalog = nullptr;

    UPROPERTY(Transient)
    FSharApplicationModeObservation Observation;

    UPROPERTY(Transient)
    TArray<FSharApplicationTransitionSnapshot> Transitions;

    UPROPERTY(Transient)
    int32 NextInsertionSequence = 0;

    [[nodiscard]] FSharApplicationTransitionSnapshot* FindTransition(
        const FName& RequestId
    );
    [[nodiscard]] const FSharApplicationTransitionSnapshot* FindTransition(
        const FName& RequestId
    ) const;
    [[nodiscard]] bool IsHead(
        const FSharApplicationTransitionSnapshot& Snapshot
    ) const;
    [[nodiscard]] bool HasConflictingActiveTransition(
        const FName& RequestId
    ) const;
    [[nodiscard]] static bool Outranks(
        const FSharApplicationTransitionSnapshot& Left,
        const FSharApplicationTransitionSnapshot& Right
    );
    [[nodiscard]] static bool IsTerminalState(
        ESharApplicationTransitionState State
    );
    [[nodiscard]] static bool IsRevisionToken(const FString& Revision);
    [[nodiscard]] static bool IsValidRequest(
        const FSharApplicationModeRequest& Request
    );
    [[nodiscard]] ESharApplicationOperationResult ClassifySubmission(
        const FSharApplicationModeRequest& Request
    ) const;
    [[nodiscard]] int32 CountPendingTransitions() const;
    [[nodiscard]] static bool MatchesEvidenceRevision(
        const FSharApplicationTransitionSnapshot& Snapshot,
        const FString& CatalogRevision,
        const FString& RequestRevision
    );
    [[nodiscard]] static bool AreRequiredServicesReady(
        const FSharApplicationTransitionSnapshot& Snapshot,
        const USharApplicationModeDefinition& Target
    );
    [[nodiscard]] static ESharApplicationOperationResult
    ClassifyPreparingEvidence(
        const FSharApplicationTransitionSnapshot& Snapshot
    );
    [[nodiscard]] static ESharApplicationOperationResult
    ClassifyServiceEvidence(
        const FSharApplicationTransitionSnapshot& Snapshot,
        const USharApplicationModeDefinition& Target,
        const FSharApplicationServiceEvidence& Evidence
    );
    [[nodiscard]] ESharApplicationOperationResult PublishTerminal(
        FSharApplicationTransitionSnapshot& Snapshot,
        ESharApplicationTransitionState State,
        ESharApplicationTerminalResult Result
    );
    [[nodiscard]] ESharApplicationOperationResult RecoverCommittedFailure(
        FSharApplicationTransitionSnapshot& Snapshot
    );
};
