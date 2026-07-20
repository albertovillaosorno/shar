// File: SharLoadCoordinatorSubsystem.h
// Path: src/uproject/Source/SharLoading/Public/Loading/SharLoadCoordinatorSubsystem.h
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: immutable load plans, request arbitration, progress, cancellation, and terminal results only; adapters own actual handles and package operations.
// Specification: docs/technical/unreal/native-asset-load-request-and-streaming-runtime.md
// LARGE-FILE owner=SharLoading; reason=cohesive reflected load-plan and request transaction contract;
// split=extract diagnostics if immutable progress history becomes persistent;
// validation=validate.sh SharLoading plus Unreal automation; review=2027-01.

#pragma once

#include "CoreMinimal.h"
#include "Engine/DataAsset.h"
#include "Subsystems/GameInstanceSubsystem.h"

#include "SharLoadCoordinatorSubsystem.generated.h"

UENUM(BlueprintType)
enum class ESharLoadNodeKind : uint8
{
    PackageAvailability,
    FeatureActivation,
    AssetBundle,
    WorldPreparation,
    DataLayer,
    Audio,
    UserInterface,
    Media,
    VerificationBarrier,
};

UENUM(BlueprintType)
enum class ESharLoadCancellationPolicy : uint8
{
    RejectDuplicate,
    CancelExisting,
    ReplacePending,
    RetainSharedWork,
};

UENUM(BlueprintType)
enum class ESharLoadResultPolicy : uint8
{
    Required,
    Optional,
    DegradedAllowed,
    PresentationOnly,
};

UENUM(BlueprintType)
enum class ESharLoadRequestState : uint8
{
    Pending,
    Resolving,
    Running,
    Verifying,
    ReadyToCommit,
    Success,
    Unavailable,
    Rejected,
    Failed,
    TimedOut,
    Cancelled,
    Superseded,
    Degraded,
    Released,
};

UENUM(BlueprintType)
enum class ESharLoadNodeState : uint8
{
    Pending,
    Active,
    Completed,
    Failed,
    Cancelled,
};

UENUM(BlueprintType)
enum class ESharLoadTerminalResult : uint8
{
    None,
    Success,
    Unavailable,
    Rejected,
    Failed,
    TimedOut,
    Cancelled,
    Superseded,
    Degraded,
};

UENUM(BlueprintType)
enum class ESharLoadTerminalCommand : uint8
{
    MarkUnavailable,
    Timeout,
    Cancel,
    Supersede,
};

UENUM(BlueprintType)
enum class ESharLoadOperationResult : uint8
{
    Accepted,
    InvalidRequest,
    PlanMissing,
    DuplicateRequest,
    QueueFull,
    NotFound,
    NotHead,
    DependencyBlocked,
    StaleRevision,
    InvalidState,
    AlreadyTerminal,
    SharedWorkRetained,
    Released,
};

USTRUCT(BlueprintType)
struct SHARLOADING_API FSharLoadPlanNode
{
    GENERATED_BODY()

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Identity")
    FName NodeId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Identity")
    FName DependencyKey;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Node")
    ESharLoadNodeKind NodeKind = ESharLoadNodeKind::AssetBundle;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Dependency")
    TArray<FName> DependsOn;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Policy")
    bool bRequired = true;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Policy")
    bool bShareable = true;
};

USTRUCT(BlueprintType)
struct SHARLOADING_API FSharLoadPlan
{
    GENERATED_BODY()

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Identity")
    FName PlanId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString PlanRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Plan")
    TArray<FSharLoadPlanNode> Nodes;
};

USTRUCT(BlueprintType)
struct SHARLOADING_API FSharLoadRequest
{
    GENERATED_BODY()

    static constexpr int32 DefaultMaximumAssetIds = 32;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Identity")
    FName RequestId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Identity")
    FName PlanId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Identity")
    FName ScopeId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Identity")
    FName CallerId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Priority")
    int32 Priority = 0;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Assets")
    TArray<FPrimaryAssetId> AssetIds;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString CatalogRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString ScopeRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString RequestRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Deadline")
    double DeadlineSeconds = 0.0;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Deadline")
    bool bLongRunningAllowed = false;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Policy")
    ESharLoadCancellationPolicy CancellationPolicy =
        ESharLoadCancellationPolicy::RejectDuplicate;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Policy")
    ESharLoadResultPolicy ResultPolicy = ESharLoadResultPolicy::Required;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Barrier")
    FName ReadinessBarrierId;
};

USTRUCT(BlueprintType)
struct SHARLOADING_API FSharLoadTerminalRequest
{
    GENERATED_BODY()

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Identity")
    FName RequestId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Result")
    ESharLoadTerminalCommand Command = ESharLoadTerminalCommand::Cancel;
};

USTRUCT(BlueprintType)
struct SHARLOADING_API FSharLoadNodeAttemptRequest
{
    GENERATED_BODY()

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Identity")
    FName RequestId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Identity")
    FName NodeId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Identity")
    FName AttemptId;
};

USTRUCT(BlueprintType)
struct SHARLOADING_API FSharLoadCallbackRevision
{
    GENERATED_BODY()

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString CatalogRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString ScopeRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString RequestRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FName AttemptId;
};

USTRUCT(BlueprintType)
struct SHARLOADING_API FSharLoadNodeCallbackRequest
{
    GENERATED_BODY()

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Identity")
    FName RequestId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Identity")
    FName NodeId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FSharLoadCallbackRevision Revision;
};

USTRUCT(BlueprintType)
struct SHARLOADING_API FSharLoadBarrierCallbackRequest
{
    GENERATED_BODY()

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Identity")
    FName RequestId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Identity")
    FName BarrierId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FSharLoadCallbackRevision Revision;
};

USTRUCT(BlueprintType)
struct SHARLOADING_API FSharLoadNodeSnapshot
{
    GENERATED_BODY()

    UPROPERTY(BlueprintReadOnly, Category = "Identity")
    FName NodeId;

    UPROPERTY(BlueprintReadOnly, Category = "Identity")
    FName DependencyKey;

    UPROPERTY(BlueprintReadOnly, Category = "State")
    ESharLoadNodeState State = ESharLoadNodeState::Pending;

    UPROPERTY(BlueprintReadOnly, Category = "Revision")
    FName AttemptId;
};

USTRUCT(BlueprintType)
struct SHARLOADING_API FSharLoadProgress
{
    GENERATED_BODY()

    UPROPERTY(BlueprintReadOnly, Category = "Identity")
    FName RequestId;

    UPROPERTY(BlueprintReadOnly, Category = "Identity")
    FName PlanId;

    UPROPERTY(BlueprintReadOnly, Category = "Count")
    int32 CompletedNodeCount = 0;

    UPROPERTY(BlueprintReadOnly, Category = "Count")
    int32 ActiveNodeCount = 0;

    UPROPERTY(BlueprintReadOnly, Category = "Count")
    int32 PendingNodeCount = 0;

    UPROPERTY(BlueprintReadOnly, Category = "Count")
    int32 FailedNodeCount = 0;

    UPROPERTY(BlueprintReadOnly, Category = "Count")
    int32 CancelledNodeCount = 0;

    UPROPERTY(BlueprintReadOnly, Category = "Progress")
    FName CurrentRequiredBarrierId;

    UPROPERTY(BlueprintReadOnly, Category = "Progress")
    int32 Revision = 0;
};

USTRUCT(BlueprintType)
struct SHARLOADING_API FSharLoadRequestSnapshot
{
    GENERATED_BODY()

    UPROPERTY(BlueprintReadOnly, Category = "Request")
    FSharLoadRequest Request;

    UPROPERTY(BlueprintReadOnly, Category = "State")
    ESharLoadRequestState State = ESharLoadRequestState::Pending;

    UPROPERTY(BlueprintReadOnly, Category = "Result")
    ESharLoadTerminalResult TerminalResult = ESharLoadTerminalResult::None;

    UPROPERTY(BlueprintReadOnly, Category = "Ordering")
    int32 InsertionSequence = 0;

    UPROPERTY(BlueprintReadOnly, Category = "Progress")
    FSharLoadProgress Progress;

    UPROPERTY(BlueprintReadOnly, Category = "Nodes")
    TArray<FSharLoadNodeSnapshot> Nodes;

    UPROPERTY(BlueprintReadOnly, Category = "Lifetime")
    bool bSharedDependenciesAcquired = false;

    UPROPERTY(BlueprintReadOnly, Category = "Lifetime")
    bool bSharedDependenciesReleased = false;

    UPROPERTY(BlueprintReadOnly, Category = "Lifetime")
    bool bReleased = false;
};

USTRUCT()
struct FSharSharedDependencyUse
{
    GENERATED_BODY()

    UPROPERTY()
    FName DependencyKey;

    UPROPERTY()
    FString CatalogRevision;

    UPROPERTY()
    FName ScopeId;

    UPROPERTY()
    int32 ConsumerCount = 0;

    UPROPERTY()
    bool bReady = false;
};

UCLASS()
class SHARLOADING_API USharLoadCoordinatorSubsystem final
    : public UGameInstanceSubsystem
{
    GENERATED_BODY()

public:
    UFUNCTION(BlueprintCallable, Category = "SHAR|Loading")
    bool ConfigureCatalog(const FString& InCatalogRevision);

    UFUNCTION(BlueprintCallable, Category = "SHAR|Loading")
    bool RegisterPlan(const FSharLoadPlan& Plan);

    UFUNCTION(BlueprintCallable, Category = "SHAR|Loading")
    ESharLoadOperationResult Submit(const FSharLoadRequest& Request);

    ESharLoadOperationResult BeginRequest(const FName& RequestId);

    ESharLoadOperationResult BeginNode(
        const FSharLoadNodeAttemptRequest& Attempt
    );

    ESharLoadOperationResult CompleteNode(
        const FSharLoadNodeCallbackRequest& Callback
    );

    ESharLoadOperationResult FailNode(
        const FSharLoadNodeCallbackRequest& Callback
    );

    ESharLoadOperationResult BeginVerification(const FName& RequestId);

    ESharLoadOperationResult AcceptBarrier(
        const FSharLoadBarrierCallbackRequest& Callback
    );

    ESharLoadOperationResult CommitSuccess(const FName& RequestId);

    ESharLoadOperationResult ResolveTerminal(
        const FSharLoadTerminalRequest& TerminalRequest
    );

    ESharLoadOperationResult Release(const FName& RequestId);

    UFUNCTION(BlueprintPure, Category = "SHAR|Loading")
    [[nodiscard]] int32 GetQueuePosition(const FName& RequestId) const;

    UFUNCTION(BlueprintPure, Category = "SHAR|Loading")
    [[nodiscard]] ESharLoadRequestState GetState(const FName& RequestId) const;

    UFUNCTION(BlueprintPure, Category = "SHAR|Loading")
    [[nodiscard]] ESharLoadTerminalResult GetTerminalResult(
        const FName& RequestId
    ) const;

    UFUNCTION(BlueprintPure, Category = "SHAR|Loading")
    [[nodiscard]] FSharLoadProgress GetProgress(const FName& RequestId) const;

    UFUNCTION(BlueprintPure, Category = "SHAR|Loading")
    [[nodiscard]] int32 GetSharedConsumerCount(
        const FName& DependencyKey,
        const FName& ScopeId
    ) const;

    UFUNCTION(BlueprintPure, Category = "SHAR|Loading")
    [[nodiscard]] int32 GetUnreleasedRequestCount() const;

private:
    UPROPERTY(Transient)
    FString CatalogRevision;

    UPROPERTY(Transient)
    TArray<FSharLoadPlan> Plans;

    UPROPERTY(Transient)
    TArray<FSharLoadRequestSnapshot> Requests;

    UPROPERTY(Transient)
    TArray<FSharSharedDependencyUse> SharedDependencies;

    UPROPERTY(Transient)
    int32 NextInsertionSequence = 0;

    [[nodiscard]] const FSharLoadPlan* FindPlan(const FName& PlanId) const;
    [[nodiscard]] FSharLoadRequestSnapshot* FindRequest(const FName& RequestId);
    [[nodiscard]] const FSharLoadRequestSnapshot* FindRequest(
        const FName& RequestId
    ) const;
    [[nodiscard]] static FSharLoadNodeSnapshot* FindNode(
        FSharLoadRequestSnapshot& Snapshot,
        const FName& NodeId
    );
    [[nodiscard]] static const FSharLoadNodeSnapshot* FindNode(
        const FSharLoadRequestSnapshot& Snapshot,
        const FName& NodeId
    );
    [[nodiscard]] bool IsHead(const FSharLoadRequestSnapshot& Snapshot) const;
    [[nodiscard]] static bool Outranks(
        const FSharLoadRequestSnapshot& Left,
        const FSharLoadRequestSnapshot& Right
    );
    [[nodiscard]] static bool IsTerminalState(ESharLoadRequestState State);
    [[nodiscard]] static bool IsRevisionToken(const FString& Revision);
    [[nodiscard]] static bool IsValidRequest(const FSharLoadRequest& Request);
    [[nodiscard]] int32 CountPendingRequests() const;
    [[nodiscard]] FSharLoadRequestSnapshot* FindEquivalentRequest(
        const FSharLoadRequest& Request
    );
    [[nodiscard]] ESharLoadOperationResult ResolveEquivalentRequest(
        FSharLoadRequestSnapshot& Existing,
        const FSharLoadRequest& Replacement
    );
    void AppendRequestSnapshot(
        const FSharLoadRequest& Request,
        const FSharLoadPlan& Plan
    );
    static void AccumulateNodeProgress(
        FSharLoadProgress& Progress,
        ESharLoadNodeState State
    );
    [[nodiscard]] static bool ValidatePlan(const FSharLoadPlan& Plan);
    [[nodiscard]] static bool DependenciesCompleted(
        const FSharLoadPlan& Plan,
        const FSharLoadRequestSnapshot& Snapshot,
        const FName& NodeId
    );
    [[nodiscard]] static bool MatchesRevision(
        const FSharLoadRequestSnapshot& Snapshot,
        const FSharLoadNodeSnapshot& Node,
        const FSharLoadCallbackRevision& Revision
    );
    static void RefreshProgress(FSharLoadRequestSnapshot& Snapshot);
    void AcquireSharedDependencies(
        const FSharLoadPlan& Plan,
        FSharLoadRequestSnapshot& Snapshot
    );
    [[nodiscard]] bool ReleaseSharedDependencies(
        const FSharLoadPlan& Plan,
        FSharLoadRequestSnapshot& Snapshot
    );
    [[nodiscard]] static ESharLoadOperationResult PublishTerminal(
        FSharLoadRequestSnapshot& Snapshot,
        ESharLoadRequestState State,
        ESharLoadTerminalResult Result
    );
};
