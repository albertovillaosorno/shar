// File: SharPresentationPlaybackSubsystem.h
// Path: src/uproject/Source/SharPresentation/Public/Presentation/SharPresentationPlaybackSubsystem.h
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: world-scoped request, queue, revision, lifecycle, terminal result, and release state only; adapters remain external.
// Specification: docs/technical/unreal/presentation-playback-runtime.md
// LARGE-FILE owner=SharPresentation; reason=cohesive reflected playback transaction contract;
// split=extract diagnostics if immutable snapshots gain persistent storage;
// validation=validate.sh SharPresentation plus Unreal automation; review=2027-01.

#pragma once

#include "CoreMinimal.h"
#include "Engine/DataAsset.h"
#include "Subsystems/WorldSubsystem.h"

#include "SharPresentationPlaybackSubsystem.generated.h"

UENUM(BlueprintType)
enum class ESharPresentationDuplicatePolicy : uint8
{
    Reject,
    ReplacePending,
};

UENUM(BlueprintType)
enum class ESharPresentationPlaybackState : uint8
{
    Queued,
    Loading,
    Ready,
    Starting,
    Playing,
    Paused,
    Stopping,
    Completed,
    Skipped,
    Cancelled,
    Failed,
    Released,
};

UENUM(BlueprintType)
enum class ESharPresentationTerminalResult : uint8
{
    None,
    Completed,
    Skipped,
    Cancelled,
    Failed,
};

UENUM(BlueprintType)
enum class ESharPresentationOperationResult : uint8
{
    Accepted,
    InvalidRequest,
    ChannelMissing,
    DuplicateRequest,
    QueueFull,
    NotFound,
    NotHead,
    StaleRevision,
    InvalidState,
    AlreadyTerminal,
    Released,
};

USTRUCT(BlueprintType)
struct SHARPRESENTATION_API FSharPresentationChannelPolicy
{
    GENERATED_BODY()

    static constexpr int32 DefaultMaximumPending = 8;
    static constexpr int32 DefaultMaximumActive = 1;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Identity")
    FName ChannelId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Capacity")
    int32 MaximumPending = DefaultMaximumPending;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Capacity")
    int32 MaximumActive = DefaultMaximumActive;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Duplicate")
    ESharPresentationDuplicatePolicy DuplicatePolicy =
        ESharPresentationDuplicatePolicy::Reject;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Policy")
    FName StarvationPolicyId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Policy")
    FName TeardownPolicyId;
};

USTRUCT(BlueprintType)
struct SHARPRESENTATION_API FSharPresentationRequest
{
    GENERATED_BODY()

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Identity")
    FName RequestId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Identity")
    FPrimaryAssetId PresentationId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Ownership")
    FName OwnerId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Ownership")
    FString OwnerRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Target")
    FName ParticipantId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Target")
    FName TargetId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString WorldRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString RequestRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Queue")
    FName ChannelId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Queue")
    int32 Priority = 0;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Skip")
    bool bSkipAllowed = false;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Deadline")
    double CompletionDeadlineSeconds = 0.0;
};

USTRUCT(BlueprintType)
struct SHARPRESENTATION_API FSharPresentationCallbackRevision
{
    GENERATED_BODY()

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString OwnerRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString WorldRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString RequestRevision;
};

USTRUCT(BlueprintType)
struct SHARPRESENTATION_API FSharPresentationPlaybackSnapshot
{
    GENERATED_BODY()

    UPROPERTY(BlueprintReadOnly, Category = "Request")
    FSharPresentationRequest Request;

    UPROPERTY(BlueprintReadOnly, Category = "Queue")
    int32 InsertionSequence = 0;

    UPROPERTY(BlueprintReadOnly, Category = "Lifecycle")
    ESharPresentationPlaybackState State =
        ESharPresentationPlaybackState::Queued;

    UPROPERTY(BlueprintReadOnly, Category = "Lifecycle")
    ESharPresentationTerminalResult TerminalResult =
        ESharPresentationTerminalResult::None;

    UPROPERTY(BlueprintReadOnly, Category = "Lifecycle")
    bool bReleased = false;
};

UCLASS()
class SHARPRESENTATION_API USharPresentationPlaybackSubsystem final
    : public UWorldSubsystem
{
    GENERATED_BODY()

public:
    UFUNCTION(BlueprintCallable, Category = "SHAR|Presentation")
    bool ConfigureWorld(const FName& InWorldId, const FString& InWorldRevision);

    UFUNCTION(BlueprintCallable, Category = "SHAR|Presentation")
    bool RegisterChannel(const FSharPresentationChannelPolicy& Policy);

    UFUNCTION(BlueprintCallable, Category = "SHAR|Presentation")
    ESharPresentationOperationResult Enqueue(
        const FSharPresentationRequest& Request
    );

    UFUNCTION(BlueprintCallable, Category = "SHAR|Presentation")
    ESharPresentationOperationResult BeginLoading(const FName& RequestId);

    UFUNCTION(BlueprintCallable, Category = "SHAR|Presentation")
    ESharPresentationOperationResult MarkReady(
        const FName& RequestId,
        const FSharPresentationCallbackRevision& Revision
    );

    UFUNCTION(BlueprintCallable, Category = "SHAR|Presentation")
    ESharPresentationOperationResult BeginStart(
        const FName& RequestId,
        const FSharPresentationCallbackRevision& Revision
    );

    UFUNCTION(BlueprintCallable, Category = "SHAR|Presentation")
    ESharPresentationOperationResult MarkPlaying(
        const FName& RequestId,
        const FSharPresentationCallbackRevision& Revision
    );

    UFUNCTION(BlueprintCallable, Category = "SHAR|Presentation")
    ESharPresentationOperationResult Pause(const FName& RequestId);

    UFUNCTION(BlueprintCallable, Category = "SHAR|Presentation")
    ESharPresentationOperationResult Resume(const FName& RequestId);

    UFUNCTION(BlueprintCallable, Category = "SHAR|Presentation")
    ESharPresentationOperationResult Complete(const FName& RequestId);

    UFUNCTION(BlueprintCallable, Category = "SHAR|Presentation")
    ESharPresentationOperationResult Skip(const FName& RequestId);

    UFUNCTION(BlueprintCallable, Category = "SHAR|Presentation")
    ESharPresentationOperationResult Cancel(const FName& RequestId);

    UFUNCTION(BlueprintCallable, Category = "SHAR|Presentation")
    ESharPresentationOperationResult Fail(const FName& RequestId);

    UFUNCTION(BlueprintCallable, Category = "SHAR|Presentation")
    ESharPresentationOperationResult Release(const FName& RequestId);

    UFUNCTION(BlueprintCallable, Category = "SHAR|Presentation")
    int32 CancelOwner(const FName& OwnerId);

    UFUNCTION(BlueprintCallable, Category = "SHAR|Presentation")
    int32 ClearChannel(const FName& ChannelId);

    UFUNCTION(BlueprintPure, Category = "SHAR|Presentation")
    [[nodiscard]] ESharPresentationPlaybackState GetState(
        const FName& RequestId
    ) const;

    UFUNCTION(BlueprintPure, Category = "SHAR|Presentation")
    [[nodiscard]] ESharPresentationTerminalResult GetTerminalResult(
        const FName& RequestId
    ) const;

    UFUNCTION(BlueprintPure, Category = "SHAR|Presentation")
    [[nodiscard]] int32 GetQueuePosition(const FName& RequestId) const;

    UFUNCTION(BlueprintPure, Category = "SHAR|Presentation")
    [[nodiscard]] int32 GetActiveRequestCount() const;

    UFUNCTION(BlueprintPure, Category = "SHAR|Presentation")
    [[nodiscard]] int32 GetUnreleasedRequestCount() const;

private:
    UPROPERTY(Transient)
    FName WorldId;

    UPROPERTY(Transient)
    FString WorldRevision;

    UPROPERTY(Transient)
    TArray<FSharPresentationChannelPolicy> Channels;

    UPROPERTY(Transient)
    TArray<FSharPresentationPlaybackSnapshot> Requests;

    UPROPERTY(Transient)
    int32 NextInsertionSequence = 0;

    [[nodiscard]] const FSharPresentationChannelPolicy* FindChannel(
        const FName& ChannelId
    ) const;
    [[nodiscard]] FSharPresentationPlaybackSnapshot* FindRequest(
        const FName& RequestId
    );
    [[nodiscard]] const FSharPresentationPlaybackSnapshot* FindRequest(
        const FName& RequestId
    ) const;
    [[nodiscard]] bool IsHead(
        const FSharPresentationPlaybackSnapshot& QueueEntry
    ) const;
    [[nodiscard]] int32 CountPending(const FName& ChannelId) const;
    [[nodiscard]] int32 CountActive(const FName& ChannelId) const;
    [[nodiscard]] static bool Outranks(
        const FSharPresentationPlaybackSnapshot& Left,
        const FSharPresentationPlaybackSnapshot& Right
    );
    [[nodiscard]] static bool IsPendingState(
        ESharPresentationPlaybackState State
    );
    [[nodiscard]] static bool IsActiveState(
        ESharPresentationPlaybackState State
    );
    [[nodiscard]] static bool IsTerminalState(
        ESharPresentationPlaybackState State
    );
    [[nodiscard]] static bool IsValidRevision(const FString& Revision);
    [[nodiscard]] static bool IsValidRequest(
        const FSharPresentationRequest& Request
    );
    [[nodiscard]] static bool MatchesRevision(
        const FSharPresentationPlaybackSnapshot& Snapshot,
        const FSharPresentationCallbackRevision& Revision
    );
    [[nodiscard]] static ESharPresentationOperationResult PublishTerminal(
        FSharPresentationPlaybackSnapshot& Snapshot,
        ESharPresentationPlaybackState State,
        ESharPresentationTerminalResult Result
    );
    [[nodiscard]] bool ReplacePendingDuplicate(
        const FSharPresentationRequest& Request,
        const FSharPresentationChannelPolicy& Policy
    );
};
