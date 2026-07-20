// File: SharLoadExecution.cpp
// Path: src/uproject/Source/SharLoading/Private/Loading/SharLoadExecution.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: request start, dependency-ordered node transitions, callback revision fencing, and readiness verification only.
// Specification: docs/technical/unreal/native-asset-load-request-and-streaming-runtime.md
// LARGE-FILE owner=SharLoading; reason=cohesive load-node execution and verification state machine;
// split=extract adapter attempt diagnostics if retry evidence becomes persistent;
// validation=validate.sh SharLoading plus Unreal automation; review=2027-01.

#include "Loading/SharLoadCoordinatorSubsystem.h"

#include "Algo/AllOf.h"
#include "Content/SharPrimaryContentDefinition.h"

static const FSharLoadPlanNode* FindPlanNode(
    const FSharLoadPlan& Plan,
    const FName& NodeId
)
{
    for (const FSharLoadPlanNode& Node : Plan.Nodes)
    {
        if (Node.NodeId == NodeId)
        {
            return &Node;
        }
    }
    return nullptr;
}

static bool IsCanonicalAttemptId(const FName& AttemptId)
{
    return USharPrimaryContentDefinition::IsCanonicalIdentifier(AttemptId);
}

static bool HasActiveNode(const FSharLoadRequestSnapshot& Snapshot)
{
    return Snapshot.Nodes.ContainsByPredicate(
        [](const FSharLoadNodeSnapshot& Node)
        {
            return Node.State == ESharLoadNodeState::Active;
        }
    );
}

static bool HasFailedNode(const FSharLoadRequestSnapshot& Snapshot)
{
    return Snapshot.Nodes.ContainsByPredicate(
        [](const FSharLoadNodeSnapshot& Node)
        {
            return Node.State == ESharLoadNodeState::Failed;
        }
    );
}

static ESharLoadOperationResult ClassifyActiveNodeCallback(
    const FSharLoadRequestSnapshot& Snapshot,
    const FSharLoadNodeSnapshot& Node,
    const bool bRevisionMatches,
    const bool bTerminalState
)
{
    if (Snapshot.bReleased)
    {
        return ESharLoadOperationResult::Released;
    }
    if (Snapshot.State != ESharLoadRequestState::Running
        || Node.State != ESharLoadNodeState::Active)
    {
        return bTerminalState
            ? ESharLoadOperationResult::AlreadyTerminal
            : ESharLoadOperationResult::InvalidState;
    }
    return bRevisionMatches
        ? ESharLoadOperationResult::Accepted
        : ESharLoadOperationResult::StaleRevision;
}

static void MarkSharedDependencyReady(
    TArray<FSharSharedDependencyUse>& SharedDependencies,
    const FSharLoadRequestSnapshot& Snapshot,
    const FSharLoadNodeSnapshot& Node
)
{
    for (FSharSharedDependencyUse& Dependency : SharedDependencies)
    {
        const bool bMatches =
            Dependency.DependencyKey == Node.DependencyKey
            && Dependency.CatalogRevision == Snapshot.Request.CatalogRevision
            && Dependency.ScopeId == Snapshot.Request.ScopeId;
        if (bMatches)
        {
            Dependency.bReady = true;
            return;
        }
    }
}

bool USharLoadCoordinatorSubsystem::DependenciesCompleted(
    const FSharLoadPlan& Plan,
    const FSharLoadRequestSnapshot& Snapshot,
    const FName& NodeId
)
{
    const FSharLoadPlanNode* PlanNode = FindPlanNode(Plan, NodeId);
    if (PlanNode == nullptr)
    {
        return false;
    }
    return Algo::AllOf(
        PlanNode->DependsOn,
        [&Snapshot](const FName& DependencyId)
        {
            const FSharLoadNodeSnapshot* Dependency = FindNode(
                Snapshot,
                DependencyId
            );
            return Dependency != nullptr
                && Dependency->State == ESharLoadNodeState::Completed;
        }
    );
}

bool USharLoadCoordinatorSubsystem::MatchesRevision(
    const FSharLoadRequestSnapshot& Snapshot,
    const FSharLoadNodeSnapshot& Node,
    const FSharLoadCallbackRevision& Revision
)
{
    return Snapshot.Request.CatalogRevision == Revision.CatalogRevision
        && Snapshot.Request.ScopeRevision == Revision.ScopeRevision
        && Snapshot.Request.RequestRevision == Revision.RequestRevision
        && Node.AttemptId == Revision.AttemptId;
}

ESharLoadOperationResult USharLoadCoordinatorSubsystem::BeginRequest(
    const FName& RequestId
)
{
    FSharLoadRequestSnapshot* Snapshot = FindRequest(RequestId);
    if (Snapshot == nullptr)
    {
        return ESharLoadOperationResult::NotFound;
    }
    if (Snapshot->bReleased)
    {
        return ESharLoadOperationResult::Released;
    }
    if (Snapshot->State != ESharLoadRequestState::Pending)
    {
        return ESharLoadOperationResult::InvalidState;
    }
    if (!IsHead(*Snapshot))
    {
        return ESharLoadOperationResult::NotHead;
    }
    const FSharLoadPlan* Plan = FindPlan(Snapshot->Request.PlanId);
    if (Plan == nullptr)
    {
        return ESharLoadOperationResult::PlanMissing;
    }
    Snapshot->State = ESharLoadRequestState::Resolving;
    AcquireSharedDependencies(*Plan, *Snapshot);
    Snapshot->State = ESharLoadRequestState::Running;
    return ESharLoadOperationResult::Accepted;
}

ESharLoadOperationResult USharLoadCoordinatorSubsystem::BeginNode(
    const FSharLoadNodeAttemptRequest& Attempt
)
{
    FSharLoadRequestSnapshot* Snapshot = FindRequest(Attempt.RequestId);
    if (Snapshot == nullptr)
    {
        return ESharLoadOperationResult::NotFound;
    }
    if (Snapshot->bReleased)
    {
        return ESharLoadOperationResult::Released;
    }
    if (Snapshot->State != ESharLoadRequestState::Running
        || !IsCanonicalAttemptId(Attempt.AttemptId))
    {
        return ESharLoadOperationResult::InvalidState;
    }
    const FSharLoadPlan* Plan = FindPlan(Snapshot->Request.PlanId);
    FSharLoadNodeSnapshot* Node = FindNode(*Snapshot, Attempt.NodeId);
    if (Plan == nullptr || Node == nullptr)
    {
        return ESharLoadOperationResult::NotFound;
    }
    if (Node->State != ESharLoadNodeState::Pending)
    {
        return ESharLoadOperationResult::InvalidState;
    }
    if (!DependenciesCompleted(*Plan, *Snapshot, Attempt.NodeId))
    {
        return ESharLoadOperationResult::DependencyBlocked;
    }
    Node->AttemptId = Attempt.AttemptId;
    Node->State = ESharLoadNodeState::Active;
    RefreshProgress(*Snapshot);
    return ESharLoadOperationResult::Accepted;
}

ESharLoadOperationResult USharLoadCoordinatorSubsystem::CompleteNode(
    const FSharLoadNodeCallbackRequest& Callback
)
{
    FSharLoadRequestSnapshot* Snapshot = FindRequest(Callback.RequestId);
    if (Snapshot == nullptr)
    {
        return ESharLoadOperationResult::NotFound;
    }
    FSharLoadNodeSnapshot* Node = FindNode(*Snapshot, Callback.NodeId);
    if (Node == nullptr)
    {
        return ESharLoadOperationResult::NotFound;
    }
    const ESharLoadOperationResult Classification = ClassifyActiveNodeCallback(
        *Snapshot,
        *Node,
        MatchesRevision(*Snapshot, *Node, Callback.Revision),
        IsTerminalState(Snapshot->State)
    );
    if (Classification != ESharLoadOperationResult::Accepted)
    {
        return Classification;
    }
    Node->State = ESharLoadNodeState::Completed;
    MarkSharedDependencyReady(SharedDependencies, *Snapshot, *Node);
    RefreshProgress(*Snapshot);
    return ESharLoadOperationResult::Accepted;
}

ESharLoadOperationResult USharLoadCoordinatorSubsystem::FailNode(
    const FSharLoadNodeCallbackRequest& Callback
)
{
    FSharLoadRequestSnapshot* Snapshot = FindRequest(Callback.RequestId);
    if (Snapshot == nullptr)
    {
        return ESharLoadOperationResult::NotFound;
    }
    FSharLoadNodeSnapshot* Node = FindNode(*Snapshot, Callback.NodeId);
    const FSharLoadPlan* Plan = FindPlan(Snapshot->Request.PlanId);
    if (Node == nullptr || Plan == nullptr)
    {
        return ESharLoadOperationResult::NotFound;
    }
    const ESharLoadOperationResult Classification = ClassifyActiveNodeCallback(
        *Snapshot,
        *Node,
        MatchesRevision(*Snapshot, *Node, Callback.Revision),
        IsTerminalState(Snapshot->State)
    );
    if (Classification != ESharLoadOperationResult::Accepted)
    {
        return Classification;
    }
    Node->State = ESharLoadNodeState::Failed;
    RefreshProgress(*Snapshot);
    const FSharLoadPlanNode* PlanNode = FindPlanNode(*Plan, Callback.NodeId);
    if (PlanNode == nullptr || !PlanNode->bRequired)
    {
        return ESharLoadOperationResult::Accepted;
    }
    const ESharLoadOperationResult Result = PublishTerminal(
        *Snapshot,
        ESharLoadRequestState::Failed,
        ESharLoadTerminalResult::Failed
    );
    if (Result != ESharLoadOperationResult::Accepted)
    {
        return Result;
    }
    const bool bRetained = ReleaseSharedDependencies(*Plan, *Snapshot);
    return bRetained
        ? ESharLoadOperationResult::SharedWorkRetained
        : ESharLoadOperationResult::Accepted;
}

ESharLoadOperationResult USharLoadCoordinatorSubsystem::BeginVerification(
    const FName& RequestId
)
{
    FSharLoadRequestSnapshot* Snapshot = FindRequest(RequestId);
    if (Snapshot == nullptr)
    {
        return ESharLoadOperationResult::NotFound;
    }
    const FSharLoadPlan* Plan = FindPlan(Snapshot->Request.PlanId);
    if (Plan == nullptr)
    {
        return ESharLoadOperationResult::PlanMissing;
    }
    if (Snapshot->State != ESharLoadRequestState::Running
        || HasActiveNode(*Snapshot))
    {
        return ESharLoadOperationResult::InvalidState;
    }
    for (const FSharLoadPlanNode& PlanNode : Plan->Nodes)
    {
        const FSharLoadNodeSnapshot* Node = FindNode(*Snapshot, PlanNode.NodeId);
        if (PlanNode.bRequired
            && (Node == nullptr || Node->State != ESharLoadNodeState::Completed))
        {
            return ESharLoadOperationResult::DependencyBlocked;
        }
    }
    Snapshot->State = ESharLoadRequestState::Verifying;
    return ESharLoadOperationResult::Accepted;
}

ESharLoadOperationResult USharLoadCoordinatorSubsystem::AcceptBarrier(
    const FSharLoadBarrierCallbackRequest& Callback
)
{
    FSharLoadRequestSnapshot* Snapshot = FindRequest(Callback.RequestId);
    if (Snapshot == nullptr)
    {
        return ESharLoadOperationResult::NotFound;
    }
    if (Snapshot->State != ESharLoadRequestState::Verifying
        || Callback.BarrierId != Snapshot->Request.ReadinessBarrierId)
    {
        return ESharLoadOperationResult::InvalidState;
    }
    const bool bStale =
        Snapshot->Request.CatalogRevision != Callback.Revision.CatalogRevision
        || Snapshot->Request.ScopeRevision != Callback.Revision.ScopeRevision
        || Snapshot->Request.RequestRevision != Callback.Revision.RequestRevision
        || !IsCanonicalAttemptId(Callback.Revision.AttemptId);
    if (bStale)
    {
        return ESharLoadOperationResult::StaleRevision;
    }
    Snapshot->State = ESharLoadRequestState::ReadyToCommit;
    return ESharLoadOperationResult::Accepted;
}

ESharLoadOperationResult USharLoadCoordinatorSubsystem::CommitSuccess(
    const FName& RequestId
)
{
    FSharLoadRequestSnapshot* Snapshot = FindRequest(RequestId);
    if (Snapshot == nullptr)
    {
        return ESharLoadOperationResult::NotFound;
    }
    if (Snapshot->State != ESharLoadRequestState::ReadyToCommit)
    {
        return IsTerminalState(Snapshot->State)
            ? ESharLoadOperationResult::AlreadyTerminal
            : ESharLoadOperationResult::InvalidState;
    }
    const bool bHasFailures = HasFailedNode(*Snapshot);
    if (bHasFailures
        && Snapshot->Request.ResultPolicy == ESharLoadResultPolicy::Required)
    {
        return PublishTerminal(
            *Snapshot,
            ESharLoadRequestState::Failed,
            ESharLoadTerminalResult::Failed
        );
    }
    return PublishTerminal(
        *Snapshot,
        bHasFailures ? ESharLoadRequestState::Degraded
                     : ESharLoadRequestState::Success,
        bHasFailures ? ESharLoadTerminalResult::Degraded
                     : ESharLoadTerminalResult::Success
    );
}
