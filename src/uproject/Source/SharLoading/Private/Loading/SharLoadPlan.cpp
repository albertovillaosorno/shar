// File: SharLoadPlan.cpp
// Path: src/uproject/Source/SharLoading/Private/Loading/SharLoadPlan.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: load-plan identity, dependency, and cycle validation only; no adapter execution.
// Specification: docs/technical/unreal/native-asset-load-request-and-streaming-runtime.md
// LARGE-FILE owner=SharLoading; reason=cohesive immutable DAG validation and lookup implementation;
// split=extract catalog-generation diagnostics if graph evidence becomes persistent;
// validation=validate.sh SharLoading plus Unreal automation; review=2027-01.

#include "Loading/SharLoadCoordinatorSubsystem.h"

#include "Algo/AllOf.h"
#include "Algo/AnyOf.h"
#include "Algo/Find.h"
#include "Content/SharPrimaryContentDefinition.h"

static bool IsCanonicalIdentity(const FName& Candidate)
{
    return USharPrimaryContentDefinition::IsCanonicalIdentifier(Candidate);
}

static bool ContainsPlanNode(const FSharLoadPlan& Plan, const FName& NodeId)
{
    return Plan.Nodes.ContainsByPredicate(
        [&NodeId](const FSharLoadPlanNode& Node)
        {
            return Node.NodeId == NodeId;
        }
    );
}

static bool HasDuplicateNodeIds(const FSharLoadPlan& Plan)
{
    return Algo::AnyOf(
        Plan.Nodes,
        [&Plan](const FSharLoadPlanNode& Candidate)
        {
            int32 MatchCount = 0;
            for (const FSharLoadPlanNode& Node : Plan.Nodes)
            {
                MatchCount += Node.NodeId == Candidate.NodeId ? 1 : 0;
            }
            return MatchCount > 1;
        }
    );
}

static bool HasDuplicateDependencies(const FSharLoadPlanNode& Node)
{
    return Algo::AnyOf(
        Node.DependsOn,
        [&Node](const FName& Candidate)
        {
            int32 MatchCount = 0;
            for (const FName& DependencyId : Node.DependsOn)
            {
                MatchCount += DependencyId == Candidate ? 1 : 0;
            }
            return MatchCount > 1;
        }
    );
}

static bool HasValidPlanNodes(const FSharLoadPlan& Plan)
{
    if (HasDuplicateNodeIds(Plan))
    {
        return false;
    }
    return Algo::AllOf(
        Plan.Nodes,
        [&Plan](const FSharLoadPlanNode& Node)
        {
            const bool bValidDependencies = Algo::AllOf(
                Node.DependsOn,
                [&Plan, &Node](const FName& DependencyId)
                {
                    return IsCanonicalIdentity(DependencyId)
                        && DependencyId != Node.NodeId
                        && ContainsPlanNode(Plan, DependencyId);
                }
            );
            return IsCanonicalIdentity(Node.NodeId)
                && IsCanonicalIdentity(Node.DependencyKey)
                && !HasDuplicateDependencies(Node)
                && bValidDependencies;
        }
    );
}

static bool IsAcyclicPlan(const FSharLoadPlan& Plan)
{
    TSet<FName> ResolvedNodeIds;
    int32 ResolvedCount = 0;
    bool bMadeProgress = true;
    while (ResolvedCount < Plan.Nodes.Num() && bMadeProgress)
    {
        bMadeProgress = false;
        for (const FSharLoadPlanNode& Node : Plan.Nodes)
        {
            if (ResolvedNodeIds.Contains(Node.NodeId))
            {
                continue;
            }
            const bool bDependenciesResolved = Algo::AllOf(
                Node.DependsOn,
                [&ResolvedNodeIds](const FName& DependencyId)
                {
                    return ResolvedNodeIds.Contains(DependencyId);
                }
            );
            if (!bDependenciesResolved)
            {
                continue;
            }
            ResolvedNodeIds.Add(Node.NodeId);
            ++ResolvedCount;
            bMadeProgress = true;
        }
    }
    return ResolvedCount == Plan.Nodes.Num();
}

bool USharLoadCoordinatorSubsystem::IsRevisionToken(const FString& Revision)
{
    return Revision.StartsWith(TEXT("sha256:"));
}

const FSharLoadPlan* USharLoadCoordinatorSubsystem::FindPlan(
    const FName& PlanId
) const
{
    return Algo::FindByPredicate(
        Plans,
        [&PlanId](const FSharLoadPlan& Plan)
        {
            return Plan.PlanId == PlanId;
        }
    );
}

FSharLoadRequestSnapshot* USharLoadCoordinatorSubsystem::FindRequest(
    const FName& RequestId
)
{
    return Algo::FindByPredicate(
        Requests,
        [&RequestId](const FSharLoadRequestSnapshot& Snapshot)
        {
            return Snapshot.Request.RequestId == RequestId;
        }
    );
}

const FSharLoadRequestSnapshot* USharLoadCoordinatorSubsystem::FindRequest(
    const FName& RequestId
) const
{
    return Algo::FindByPredicate(
        Requests,
        [&RequestId](const FSharLoadRequestSnapshot& Snapshot)
        {
            return Snapshot.Request.RequestId == RequestId;
        }
    );
}

FSharLoadNodeSnapshot* USharLoadCoordinatorSubsystem::FindNode(
    FSharLoadRequestSnapshot& Snapshot,
    const FName& NodeId
)
{
    return Algo::FindByPredicate(
        Snapshot.Nodes,
        [&NodeId](const FSharLoadNodeSnapshot& Node)
        {
            return Node.NodeId == NodeId;
        }
    );
}

const FSharLoadNodeSnapshot* USharLoadCoordinatorSubsystem::FindNode(
    const FSharLoadRequestSnapshot& Snapshot,
    const FName& NodeId
)
{
    return Algo::FindByPredicate(
        Snapshot.Nodes,
        [&NodeId](const FSharLoadNodeSnapshot& Node)
        {
            return Node.NodeId == NodeId;
        }
    );
}

bool USharLoadCoordinatorSubsystem::ValidatePlan(const FSharLoadPlan& Plan)
{
    return IsCanonicalIdentity(Plan.PlanId)
        && IsRevisionToken(Plan.PlanRevision)
        && !Plan.Nodes.IsEmpty()
        && HasValidPlanNodes(Plan)
        && IsAcyclicPlan(Plan);
}

bool USharLoadCoordinatorSubsystem::RegisterPlan(const FSharLoadPlan& Plan)
{
    if (!ValidatePlan(Plan) || FindPlan(Plan.PlanId) != nullptr)
    {
        return false;
    }
    Plans.Add(Plan);
    return true;
}
