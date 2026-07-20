// File: SharLoadingTestFixtures.h
// Path: src/uproject/Source/SharLoading/Private/Tests/SharLoadingTestFixtures.h
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: explicit transient load plans, requests, callbacks, coordinators, and readiness fixtures only.
// Specification: docs/technical/unreal/native-asset-load-request-and-streaming-runtime.md
// LARGE-FILE owner=SharLoading; reason=cohesive typed loading test fixtures;
// split=extract world-readiness fixtures if additional barrier kinds appear;
// validation=validate.sh SharLoading plus Unreal automation; review=2027-01.

#pragma once

#include "Loading/SharLoadCoordinatorSubsystem.h"
#include "Loading/SharWorldReadinessSubsystem.h"

#include "Engine/DataAsset.h"
#include "Engine/GameInstance.h"

constexpr double DefaultFixtureDeadlineSeconds = 30.0;

struct FSharLoadPlanFixture
{
    FName PlanId;
    FName RootNodeId;
    FName RootDependencyKey;
    FName ChildNodeId;
    FName ChildDependencyKey;
    FName OptionalNodeId;
    FName OptionalDependencyKey;
    bool bIncludeOptional = false;
};

struct FSharLoadRequestFixture
{
    FName RequestId;
    FName PlanId;
    FName ScopeId;
    FName CallerId;
    int32 Priority = 0;
    ESharLoadCancellationPolicy CancellationPolicy =
        ESharLoadCancellationPolicy::RejectDuplicate;
    ESharLoadResultPolicy ResultPolicy = ESharLoadResultPolicy::Required;
};

inline FSharLoadPlan MakePlan(const FSharLoadPlanFixture& Fixture)
{
    FSharLoadPlan Plan;
    Plan.PlanId = Fixture.PlanId;
    Plan.PlanRevision = TEXT("sha256:plan_v1");

    FSharLoadPlanNode Root;
    Root.NodeId = Fixture.RootNodeId;
    Root.DependencyKey = Fixture.RootDependencyKey;
    Root.NodeKind = ESharLoadNodeKind::PackageAvailability;
    Root.bRequired = true;
    Root.bShareable = true;
    Plan.Nodes.Add(Root);

    FSharLoadPlanNode Child;
    Child.NodeId = Fixture.ChildNodeId;
    Child.DependencyKey = Fixture.ChildDependencyKey;
    Child.NodeKind = ESharLoadNodeKind::WorldPreparation;
    Child.DependsOn = {Fixture.RootNodeId};
    Child.bRequired = true;
    Child.bShareable = true;
    Plan.Nodes.Add(Child);

    if (Fixture.bIncludeOptional)
    {
        FSharLoadPlanNode Optional;
        Optional.NodeId = Fixture.OptionalNodeId;
        Optional.DependencyKey = Fixture.OptionalDependencyKey;
        Optional.NodeKind = ESharLoadNodeKind::Media;
        Optional.DependsOn = {Fixture.RootNodeId};
        Optional.bRequired = false;
        Optional.bShareable = false;
        Plan.Nodes.Add(Optional);
    }
    return Plan;
}

inline FSharLoadRequest MakeRequest(
    const FSharLoadRequestFixture& Fixture
)
{
    FSharLoadRequest Request;
    Request.RequestId = Fixture.RequestId;
    Request.PlanId = Fixture.PlanId;
    Request.ScopeId = Fixture.ScopeId;
    Request.CallerId = Fixture.CallerId;
    Request.Priority = Fixture.Priority;
    Request.AssetIds = {
        FPrimaryAssetId(
            FPrimaryAssetType(TEXT("SharWorld")),
            FName(TEXT("springfield_world"))
        ),
    };
    Request.CatalogRevision = TEXT("sha256:catalog_v1");
    Request.ScopeRevision = TEXT("sha256:scope_v1");
    Request.RequestRevision = TEXT("sha256:request_v1");
    Request.DeadlineSeconds = DefaultFixtureDeadlineSeconds;
    Request.bLongRunningAllowed = false;
    Request.CancellationPolicy = Fixture.CancellationPolicy;
    Request.ResultPolicy = Fixture.ResultPolicy;
    Request.ReadinessBarrierId = FName(TEXT("world_ready_v1"));
    return Request;
}

inline FSharLoadCallbackRevision MakeCallbackRevision(
    const FName& AttemptId
)
{
    FSharLoadCallbackRevision Revision;
    Revision.CatalogRevision = TEXT("sha256:catalog_v1");
    Revision.ScopeRevision = TEXT("sha256:scope_v1");
    Revision.RequestRevision = TEXT("sha256:request_v1");
    Revision.AttemptId = AttemptId;
    return Revision;
}

inline USharLoadCoordinatorSubsystem* MakeEmptyCoordinator()
{
    using FCoordinatorSubsystem = USharLoadCoordinatorSubsystem;
    auto* GameInstance = NewObject<UGameInstance>();
    auto* Coordinator = NewObject<FCoordinatorSubsystem>(GameInstance);
    Coordinator->ConfigureCatalog(TEXT("sha256:catalog_v1"));
    return Coordinator;
}

inline USharLoadCoordinatorSubsystem* MakeCoordinator(
    const FSharLoadPlan& Plan
)
{
    USharLoadCoordinatorSubsystem* Coordinator = MakeEmptyCoordinator();
    Coordinator->RegisterPlan(Plan);
    return Coordinator;
}

inline FSharWorldReadinessBarrier MakeWorldReadinessBarrier()
{
    FSharWorldReadinessBarrier Barrier;
    Barrier.BarrierId = FName(TEXT("springfield_gameplay_ready"));
    Barrier.WorldId = FName(TEXT("springfield_world"));
    Barrier.WorldRevision = TEXT("sha256:world_v1");
    Barrier.RequiredCheckpointIds = {
        FName(TEXT("actors_ready")),
        FName(TEXT("collision_ready")),
        FName(TEXT("navigation_ready")),
    };
    return Barrier;
}

inline FSharLoadPlan MakeRequiredPlan()
{
    return MakePlan({
        .PlanId = FName(TEXT("springfield_world_plan")),
        .RootNodeId = FName(TEXT("package_ready")),
        .RootDependencyKey = FName(TEXT("springfield_package")),
        .ChildNodeId = FName(TEXT("world_ready")),
        .ChildDependencyKey = FName(TEXT("springfield_world_bundle")),
        .OptionalNodeId = FName(TEXT("optional_media")),
        .OptionalDependencyKey = FName(TEXT("springfield_media")),
        .bIncludeOptional = false,
    });
}

inline FSharLoadPlan MakeOptionalPlan()
{
    return MakePlan({
        .PlanId = FName(TEXT("springfield_optional_plan")),
        .RootNodeId = FName(TEXT("package_ready")),
        .RootDependencyKey = FName(TEXT("springfield_package")),
        .ChildNodeId = FName(TEXT("world_ready")),
        .ChildDependencyKey = FName(TEXT("springfield_world_bundle")),
        .OptionalNodeId = FName(TEXT("optional_media")),
        .OptionalDependencyKey = FName(TEXT("springfield_media")),
        .bIncludeOptional = true,
    });
}
