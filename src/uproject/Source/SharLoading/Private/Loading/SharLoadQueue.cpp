// File: SharLoadQueue.cpp
// Path: src/uproject/Source/SharLoading/Private/Loading/SharLoadQueue.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: request validation, deterministic queue ordering, duplicate policy, and logical shared-dependency ownership only.
// Specification: docs/technical/unreal/native-asset-load-request-and-streaming-runtime.md
// LARGE-FILE owner=SharLoading; reason=cohesive load-request arbitration and shared-work accounting;
// split=extract scope budgets if capacity becomes platform-dependent;
// validation=validate.sh SharLoading plus Unreal automation; review=2027-01.

#include "Loading/SharLoadCoordinatorSubsystem.h"

#include "Algo/AnyOf.h"
#include "Algo/Find.h"
#include "Content/SharPrimaryContentDefinition.h"
#include "Engine/DataAsset.h"

static constexpr int32 MaximumPendingRequestCount = 64;

static bool IsCanonicalIdentity(const FName& Candidate)
{
    return USharPrimaryContentDefinition::IsCanonicalIdentifier(Candidate);
}

static bool HasInvalidAssetIds(const FSharLoadRequest& Request)
{
    return Request.AssetIds.IsEmpty()
        || Request.AssetIds.Num() > FSharLoadRequest::DefaultMaximumAssetIds
        || Request.AssetIds.ContainsByPredicate(
            [](const FPrimaryAssetId& AssetId)
            {
                return !AssetId.IsValid();
            }
        );
}

bool USharLoadCoordinatorSubsystem::ConfigureCatalog(
    const FString& InCatalogRevision
)
{
    if (!IsRevisionToken(InCatalogRevision))
    {
        return false;
    }
    CatalogRevision = InCatalogRevision;
    Plans.Reset();
    Requests.Reset();
    SharedDependencies.Reset();
    NextInsertionSequence = 0;
    return true;
}

bool USharLoadCoordinatorSubsystem::IsValidRequest(
    const FSharLoadRequest& Request
)
{
    const bool bInvalidIdentity =
        !IsCanonicalIdentity(Request.RequestId)
        || !IsCanonicalIdentity(Request.PlanId)
        || !IsCanonicalIdentity(Request.ScopeId)
        || !IsCanonicalIdentity(Request.CallerId)
        || !IsCanonicalIdentity(Request.ReadinessBarrierId);
    const bool bInvalidRevision =
        !IsRevisionToken(Request.CatalogRevision)
        || !IsRevisionToken(Request.ScopeRevision)
        || !IsRevisionToken(Request.RequestRevision);
    const bool bInvalidDeadline =
        !FMath::IsFinite(Request.DeadlineSeconds)
        || (Request.DeadlineSeconds <= 0.0 && !Request.bLongRunningAllowed);
    return !bInvalidIdentity
        && !bInvalidRevision
        && !bInvalidDeadline
        && !HasInvalidAssetIds(Request);
}

bool USharLoadCoordinatorSubsystem::Outranks(
    const FSharLoadRequestSnapshot& Left,
    const FSharLoadRequestSnapshot& Right
)
{
    if (Left.Request.Priority != Right.Request.Priority)
    {
        return Left.Request.Priority > Right.Request.Priority;
    }
    return Left.Request.RequestId.LexicalLess(Right.Request.RequestId);
}

bool USharLoadCoordinatorSubsystem::IsTerminalState(
    const ESharLoadRequestState State
)
{
    return State == ESharLoadRequestState::Success
        || State == ESharLoadRequestState::Unavailable
        || State == ESharLoadRequestState::Rejected
        || State == ESharLoadRequestState::Failed
        || State == ESharLoadRequestState::TimedOut
        || State == ESharLoadRequestState::Cancelled
        || State == ESharLoadRequestState::Superseded
        || State == ESharLoadRequestState::Degraded;
}

bool USharLoadCoordinatorSubsystem::IsHead(
    const FSharLoadRequestSnapshot& Snapshot
) const
{
    if (Snapshot.bReleased || Snapshot.State != ESharLoadRequestState::Pending)
    {
        return false;
    }
    const bool bHasOutrankingRequest = Algo::AnyOf(
        Requests,
        [&Snapshot](const FSharLoadRequestSnapshot& Other)
        {
            const bool bComparable =
                !Other.bReleased
                && Other.State == ESharLoadRequestState::Pending
                && Other.Request.RequestId != Snapshot.Request.RequestId;
            return bComparable && Outranks(Other, Snapshot);
        }
    );
    return !bHasOutrankingRequest;
}

void USharLoadCoordinatorSubsystem::AccumulateNodeProgress(
    FSharLoadProgress& Progress,
    const ESharLoadNodeState State
)
{
    switch (State)
    {
    case ESharLoadNodeState::Completed:
        ++Progress.CompletedNodeCount;
        break;
    case ESharLoadNodeState::Active:
        ++Progress.ActiveNodeCount;
        break;
    case ESharLoadNodeState::Pending:
        ++Progress.PendingNodeCount;
        break;
    case ESharLoadNodeState::Failed:
        ++Progress.FailedNodeCount;
        break;
    case ESharLoadNodeState::Cancelled:
        ++Progress.CancelledNodeCount;
        break;
    default:
        break;
    }
}

void USharLoadCoordinatorSubsystem::RefreshProgress(
    FSharLoadRequestSnapshot& Snapshot
)
{
    Snapshot.Progress.CompletedNodeCount = 0;
    Snapshot.Progress.ActiveNodeCount = 0;
    Snapshot.Progress.PendingNodeCount = 0;
    Snapshot.Progress.FailedNodeCount = 0;
    Snapshot.Progress.CancelledNodeCount = 0;
    for (const FSharLoadNodeSnapshot& Node : Snapshot.Nodes)
    {
        AccumulateNodeProgress(Snapshot.Progress, Node.State);
    }
    ++Snapshot.Progress.Revision;
}

int32 USharLoadCoordinatorSubsystem::CountPendingRequests() const
{
    int32 PendingCount = 0;
    for (const FSharLoadRequestSnapshot& Snapshot : Requests)
    {
        if (!Snapshot.bReleased
            && Snapshot.State == ESharLoadRequestState::Pending)
        {
            ++PendingCount;
        }
    }
    return PendingCount;
}

FSharLoadRequestSnapshot*
USharLoadCoordinatorSubsystem::FindEquivalentRequest(
    const FSharLoadRequest& Request
)
{
    for (FSharLoadRequestSnapshot& Snapshot : Requests)
    {
        const bool bEquivalent =
            !Snapshot.bReleased
            && !IsTerminalState(Snapshot.State)
            && Snapshot.Request.PlanId == Request.PlanId
            && Snapshot.Request.ScopeId == Request.ScopeId
            && Snapshot.Request.CallerId == Request.CallerId;
        if (bEquivalent)
        {
            return &Snapshot;
        }
    }
    return nullptr;
}

ESharLoadOperationResult
USharLoadCoordinatorSubsystem::ResolveEquivalentRequest(
    FSharLoadRequestSnapshot& Existing,
    const FSharLoadRequest& Replacement
)
{
    switch (Replacement.CancellationPolicy)
    {
    case ESharLoadCancellationPolicy::RejectDuplicate:
        return ESharLoadOperationResult::DuplicateRequest;
    case ESharLoadCancellationPolicy::ReplacePending:
        if (Existing.State != ESharLoadRequestState::Pending)
        {
            return ESharLoadOperationResult::InvalidState;
        }
        return PublishTerminal(
            Existing,
            ESharLoadRequestState::Superseded,
            ESharLoadTerminalResult::Superseded
        );
    case ESharLoadCancellationPolicy::CancelExisting:
    {
        const ESharLoadOperationResult CancelResult = ResolveTerminal({
            .RequestId = Existing.Request.RequestId,
            .Command = ESharLoadTerminalCommand::Cancel,
        });
        return CancelResult == ESharLoadOperationResult::Accepted
                || CancelResult == ESharLoadOperationResult::SharedWorkRetained
            ? ESharLoadOperationResult::Accepted
            : CancelResult;
    }
    case ESharLoadCancellationPolicy::RetainSharedWork:
        return ESharLoadOperationResult::Accepted;
    default:
        return ESharLoadOperationResult::InvalidRequest;
    }
}

void USharLoadCoordinatorSubsystem::AppendRequestSnapshot(
    const FSharLoadRequest& Request,
    const FSharLoadPlan& Plan
)
{
    FSharLoadRequestSnapshot Snapshot;
    Snapshot.Request = Request;
    Snapshot.InsertionSequence = NextInsertionSequence;
    ++NextInsertionSequence;
    Snapshot.Progress.RequestId = Request.RequestId;
    Snapshot.Progress.PlanId = Request.PlanId;
    Snapshot.Progress.CurrentRequiredBarrierId = Request.ReadinessBarrierId;
    for (const FSharLoadPlanNode& PlanNode : Plan.Nodes)
    {
        FSharLoadNodeSnapshot Node;
        Node.NodeId = PlanNode.NodeId;
        Node.DependencyKey = PlanNode.DependencyKey;
        Snapshot.Nodes.Add(Node);
    }
    RefreshProgress(Snapshot);
    Requests.Add(Snapshot);
}

ESharLoadOperationResult USharLoadCoordinatorSubsystem::Submit(
    const FSharLoadRequest& Request
)
{
    if (CatalogRevision.IsEmpty() || !IsValidRequest(Request))
    {
        return ESharLoadOperationResult::InvalidRequest;
    }
    if (Request.CatalogRevision != CatalogRevision)
    {
        return ESharLoadOperationResult::StaleRevision;
    }
    const FSharLoadPlan* Plan = FindPlan(Request.PlanId);
    if (Plan == nullptr)
    {
        return ESharLoadOperationResult::PlanMissing;
    }
    if (FindRequest(Request.RequestId) != nullptr)
    {
        return ESharLoadOperationResult::DuplicateRequest;
    }
    if (CountPendingRequests() >= MaximumPendingRequestCount)
    {
        return ESharLoadOperationResult::QueueFull;
    }
    FSharLoadRequestSnapshot* Equivalent = FindEquivalentRequest(Request);
    if (Equivalent != nullptr)
    {
        const ESharLoadOperationResult Resolution =
            ResolveEquivalentRequest(*Equivalent, Request);
        if (Resolution != ESharLoadOperationResult::Accepted)
        {
            return Resolution;
        }
    }
    AppendRequestSnapshot(Request, *Plan);
    return ESharLoadOperationResult::Accepted;
}

int32 USharLoadCoordinatorSubsystem::GetQueuePosition(
    const FName& RequestId
) const
{
    const FSharLoadRequestSnapshot* Snapshot = FindRequest(RequestId);
    if (Snapshot == nullptr || Snapshot->bReleased
        || Snapshot->State != ESharLoadRequestState::Pending)
    {
        return 0;
    }
    int32 Position = 1;
    for (const FSharLoadRequestSnapshot& Other : Requests)
    {
        const bool bComparable =
            !Other.bReleased
            && Other.State == ESharLoadRequestState::Pending
            && Other.Request.RequestId != Snapshot->Request.RequestId;
        Position += bComparable && Outranks(Other, *Snapshot) ? 1 : 0;
    }
    return Position;
}

static bool MatchesSharedDependency(
    const FSharSharedDependencyUse& Dependency,
    const FSharLoadPlanNode& PlanNode,
    const FSharLoadRequestSnapshot& Snapshot
)
{
    return Dependency.DependencyKey == PlanNode.DependencyKey
        && Dependency.CatalogRevision == Snapshot.Request.CatalogRevision
        && Dependency.ScopeId == Snapshot.Request.ScopeId;
}

static FSharSharedDependencyUse* FindSharedDependency(
    TArray<FSharSharedDependencyUse>& SharedDependencies,
    const FSharLoadPlanNode& PlanNode,
    const FSharLoadRequestSnapshot& Snapshot
)
{
    for (FSharSharedDependencyUse& Dependency : SharedDependencies)
    {
        if (MatchesSharedDependency(Dependency, PlanNode, Snapshot))
        {
            return &Dependency;
        }
    }
    return nullptr;
}

static void AddSharedDependency(
    TArray<FSharSharedDependencyUse>& SharedDependencies,
    const FSharLoadPlanNode& PlanNode,
    const FSharLoadRequestSnapshot& Snapshot
)
{
    FSharSharedDependencyUse Dependency;
    Dependency.DependencyKey = PlanNode.DependencyKey;
    Dependency.CatalogRevision = Snapshot.Request.CatalogRevision;
    Dependency.ScopeId = Snapshot.Request.ScopeId;
    Dependency.ConsumerCount = 1;
    SharedDependencies.Add(Dependency);
}

static void ApplyReadySharedDependency(
    FSharLoadRequestSnapshot& Snapshot,
    const FSharLoadPlanNode& PlanNode,
    const FSharSharedDependencyUse& Dependency
)
{
    if (!Dependency.bReady)
    {
        return;
    }
    FSharLoadNodeSnapshot* Node = nullptr;
    Node = Algo::FindByPredicate(
        Snapshot.Nodes,
        [&PlanNode](const FSharLoadNodeSnapshot& Candidate)
        {
            return Candidate.NodeId == PlanNode.NodeId;
        }
    );
    if (Node != nullptr)
    {
        Node->State = ESharLoadNodeState::Completed;
    }
}

static bool ReleaseOneSharedDependency(
    TArray<FSharSharedDependencyUse>& SharedDependencies,
    const FSharLoadPlanNode& PlanNode,
    const FSharLoadRequestSnapshot& Snapshot
)
{
    FSharSharedDependencyUse* Dependency = FindSharedDependency(
        SharedDependencies,
        PlanNode,
        Snapshot
    );
    if (Dependency == nullptr || Dependency->ConsumerCount <= 0)
    {
        return false;
    }
    --Dependency->ConsumerCount;
    return Dependency->ConsumerCount > 0;
}

void USharLoadCoordinatorSubsystem::AcquireSharedDependencies(
    const FSharLoadPlan& Plan,
    FSharLoadRequestSnapshot& Snapshot
)
{
    for (const FSharLoadPlanNode& PlanNode : Plan.Nodes)
    {
        if (!PlanNode.bShareable)
        {
            continue;
        }
        FSharSharedDependencyUse* Existing = FindSharedDependency(
            SharedDependencies,
            PlanNode,
            Snapshot
        );
        if (Existing == nullptr)
        {
            AddSharedDependency(SharedDependencies, PlanNode, Snapshot);
            continue;
        }
        ++Existing->ConsumerCount;
        ApplyReadySharedDependency(Snapshot, PlanNode, *Existing);
    }
    Snapshot.bSharedDependenciesAcquired = true;
    RefreshProgress(Snapshot);
}

bool USharLoadCoordinatorSubsystem::ReleaseSharedDependencies(
    const FSharLoadPlan& Plan,
    FSharLoadRequestSnapshot& Snapshot
)
{
    if (!Snapshot.bSharedDependenciesAcquired
        || Snapshot.bSharedDependenciesReleased)
    {
        return false;
    }
    bool bRetained = false;
    for (const FSharLoadPlanNode& PlanNode : Plan.Nodes)
    {
        if (PlanNode.bShareable)
        {
            bRetained = ReleaseOneSharedDependency(
                SharedDependencies,
                PlanNode,
                Snapshot
            ) || bRetained;
        }
    }
    Snapshot.bSharedDependenciesReleased = true;
    return bRetained;
}

int32 USharLoadCoordinatorSubsystem::GetSharedConsumerCount(
    const FName& DependencyKey,
    const FName& ScopeId
) const
{
    for (const FSharSharedDependencyUse& Dependency : SharedDependencies)
    {
        if (Dependency.DependencyKey == DependencyKey
            && Dependency.ScopeId == ScopeId
            && Dependency.CatalogRevision == CatalogRevision)
        {
            return Dependency.ConsumerCount;
        }
    }
    return 0;
}
