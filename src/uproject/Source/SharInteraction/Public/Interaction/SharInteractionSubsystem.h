// File: SharInteractionSubsystem.h
// Path: src/uproject/Source/SharInteraction/Public/Interaction/SharInteractionSubsystem.h
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: per-world interaction source, query, reservation, and transaction state only; no domain effect storage.
// ADR: docs/adr/unreal/runtime/contextual-interaction-query-and-transaction.md
// LARGE-FILE owner=SharInteraction; reason=cohesive reflected interaction runtime contract;
// split=extract result observations if durable publication storage is introduced;
// validation=validate.sh SharInteraction plus Unreal automation; review=2027-01.

#pragma once

#include "CoreMinimal.h"
#include "Engine/DataAsset.h"
#include "Subsystems/WorldSubsystem.h"

#include "SharInteractionSubsystem.generated.h"

UENUM(BlueprintType)
enum class ESharInteractionTransactionPhase : uint8
{
    Query,
    Reserved,
    Revalidated,
    PresentationPrepared,
    EffectsCommitted,
    ResultPublished,
    Released,
    Cancelled,
    Failed,
};

UENUM(BlueprintType)
enum class ESharInteractionResultCode : uint8
{
    Accepted,
    Completed,
    NotFound,
    NotEligible,
    SourceStale,
    InteractorStale,
    SlotUnavailable,
    AlreadyExecuting,
    InvalidRequest,
    InvalidPhase,
    Cancelled,
    DownstreamRejected,
    VerificationFailed,
    CompensationFailed,
};

USTRUCT(BlueprintType)
struct SHARINTERACTION_API FSharInteractionSourceState
{
    GENERATED_BODY()

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Identity")
    FName SourceId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Identity")
    FPrimaryAssetId InteractionId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString SourceRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Source")
    bool bEnabled = true;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Source")
    bool bExclusive = false;
};

USTRUCT(BlueprintType)
struct SHARINTERACTION_API FSharInteractionCandidate
{
    GENERATED_BODY()

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Identity")
    FName SourceId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Identity")
    FPrimaryAssetId InteractionId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString SourceRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Ordering")
    int32 Priority = 0;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Ordering")
    double DistanceSquared = 0.0;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Eligibility")
    bool bEligible = false;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Eligibility")
    FName EligibilityReasonId;
};

USTRUCT(BlueprintType)
struct SHARINTERACTION_API FSharInteractionQuery
{
    GENERATED_BODY()

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Identity")
    FName QueryId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Identity")
    FName InteractorId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString WorldRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString InteractorRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Candidate")
    TArray<FSharInteractionCandidate> Candidates;
};

USTRUCT(BlueprintType)
struct SHARINTERACTION_API FSharInteractionTransactionState
{
    GENERATED_BODY()

    UPROPERTY(BlueprintReadOnly, Category = "Identity")
    FName TransactionId;

    UPROPERTY(BlueprintReadOnly, Category = "Identity")
    FName InteractorId;

    UPROPERTY(BlueprintReadOnly, Category = "Identity")
    FName SourceId;

    UPROPERTY(BlueprintReadOnly, Category = "Identity")
    FPrimaryAssetId InteractionId;

    UPROPERTY(BlueprintReadOnly, Category = "Revision")
    FString WorldRevision;

    UPROPERTY(BlueprintReadOnly, Category = "Revision")
    FString InteractorRevision;

    UPROPERTY(BlueprintReadOnly, Category = "Revision")
    FString SourceRevision;

    UPROPERTY(BlueprintReadOnly, Category = "Transaction")
    ESharInteractionTransactionPhase Phase =
        ESharInteractionTransactionPhase::Query;

    UPROPERTY(BlueprintReadOnly, Category = "Transaction")
    ESharInteractionResultCode Result = ESharInteractionResultCode::Accepted;

    UPROPERTY(BlueprintReadOnly, Category = "Transaction")
    bool bReservationHeld = false;
};

UCLASS()
class SHARINTERACTION_API USharInteractionSubsystem final
    : public UWorldSubsystem
{
    GENERATED_BODY()

public:
    UFUNCTION(BlueprintCallable, Category = "SHAR|Interaction")
    bool ConfigureWorld(const FName& InWorldId, const FString& InWorldRevision);

    UFUNCTION(BlueprintCallable, Category = "SHAR|Interaction")
    bool RegisterSource(const FSharInteractionSourceState& Source);

    UFUNCTION(BlueprintCallable, Category = "SHAR|Interaction")
    bool UnregisterSource(const FName& SourceId);

    UFUNCTION(BlueprintCallable, Category = "SHAR|Interaction")
    ESharInteractionResultCode SelectCandidate(
        const FSharInteractionQuery& Query,
        FSharInteractionCandidate& OutCandidate
    ) const;

    UFUNCTION(BlueprintCallable, Category = "SHAR|Interaction")
    ESharInteractionResultCode BeginTransaction(
        const FSharInteractionQuery& Query,
        const FSharInteractionCandidate& Candidate,
        const FName& TransactionId
    );

    UFUNCTION(BlueprintCallable, Category = "SHAR|Interaction")
    ESharInteractionResultCode AdvanceTransaction(
        const FName& TransactionId,
        ESharInteractionTransactionPhase ExpectedPhase,
        ESharInteractionTransactionPhase NextPhase
    );

    UFUNCTION(BlueprintCallable, Category = "SHAR|Interaction")
    ESharInteractionResultCode CompleteTransaction(
        const FName& TransactionId,
        bool bVerificationSucceeded
    );

    UFUNCTION(BlueprintCallable, Category = "SHAR|Interaction")
    ESharInteractionResultCode CancelTransaction(
        const FName& TransactionId,
        bool bCompensationSucceeded
    );

    UFUNCTION(BlueprintPure, Category = "SHAR|Interaction")
    [[nodiscard]] ESharInteractionTransactionPhase GetTransactionPhase(
        const FName& TransactionId
    ) const;

    UFUNCTION(BlueprintPure, Category = "SHAR|Interaction")
    [[nodiscard]] ESharInteractionResultCode GetTransactionResult(
        const FName& TransactionId
    ) const;

    UFUNCTION(BlueprintPure, Category = "SHAR|Interaction")
    [[nodiscard]] bool IsSourceReserved(const FName& SourceId) const;

    UFUNCTION(BlueprintPure, Category = "SHAR|Interaction")
    [[nodiscard]] int32 GetActiveTransactionCount() const;

private:
    UPROPERTY(Transient)
    FName WorldId;

    UPROPERTY(Transient)
    FString WorldRevision;

    UPROPERTY(Transient)
    TArray<FSharInteractionSourceState> Sources;

    UPROPERTY(Transient)
    TArray<FSharInteractionTransactionState> Transactions;

    [[nodiscard]] const FSharInteractionSourceState* FindSource(
        const FName& SourceId
    ) const;
    [[nodiscard]] FSharInteractionSourceState* FindSource(
        const FName& SourceId
    );
    [[nodiscard]] const FSharInteractionTransactionState* FindTransaction(
        const FName& TransactionId
    ) const;
    [[nodiscard]] FSharInteractionTransactionState* FindTransaction(
        const FName& TransactionId
    );
    [[nodiscard]] static bool CandidateOutranks(
        const FSharInteractionCandidate& Candidate,
        const FSharInteractionCandidate& Current
    );
    [[nodiscard]] ESharInteractionResultCode ValidateQuery(
        const FSharInteractionQuery& Query
    ) const;
    [[nodiscard]] ESharInteractionResultCode ValidateCandidate(
        const FSharInteractionQuery& Query,
        const FSharInteractionCandidate& Candidate
    ) const;
    [[nodiscard]] bool HasActiveInteractorTransaction(
        const FName& InteractorId
    ) const;
    static void ReleaseReservation(
        FSharInteractionTransactionState& Transaction
    );
    void FailTransactionsForSource(const FName& SourceId);
};
