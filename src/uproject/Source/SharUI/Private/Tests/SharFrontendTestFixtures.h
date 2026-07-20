// File: SharFrontendTestFixtures.h
// Path: src/uproject/Source/SharUI/Private/Tests/SharFrontendTestFixtures.h
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: transient frontend catalogs, flow requests, and correlated evidence fixtures only.
// Specification: docs/technical/unreal/frontend-screen-flow-and-settings-runtime.md
// LARGE-FILE owner=SharUI; reason=cohesive typed frontend runtime test fixtures;
// split=extract settings fixtures when device-configuration sessions are implemented;
// validation=validate.sh SharUI plus Unreal automation; review=2027-01.

#pragma once

#include "Engine/GameInstance.h"
#include "UI/SharFrontendCatalogDefinition.h"
#include "UI/SharFrontendCatalogSubsystem.h"
#include "UI/SharFrontendFlowSubsystem.h"

constexpr int32 FrontendQueuePositionSecond = 2;
constexpr int32 FrontendQueuePositionThird = 3;

struct FSharFrontendRuntimeFixture
{
    UGameInstance* GameInstance = nullptr;
    USharFrontendCatalogSubsystem* CatalogSubsystem = nullptr;
    USharFrontendFlowSubsystem* FlowSubsystem = nullptr;
};

inline FName ReadinessEvidenceId(const ESharFrontendReadinessKind Kind)
{
    switch (Kind)
    {
        case ESharFrontendReadinessKind::DomainSnapshot:
            return FName(TEXT("domain_snapshot_ready"));
        case ESharFrontendReadinessKind::AssetBundles:
            return FName(TEXT("asset_bundles_ready"));
        case ESharFrontendReadinessKind::ViewModel:
            return FName(TEXT("view_model_ready"));
        case ESharFrontendReadinessKind::LayerReservation:
            return FName(TEXT("layer_reservation_ready"));
        case ESharFrontendReadinessKind::WidgetActivation:
            return FName(TEXT("widget_activation_ready"));
        case ESharFrontendReadinessKind::Focus:
            return FName(TEXT("default_focus_target"));
        case ESharFrontendReadinessKind::ActionRouting:
        default:
            return FName(TEXT("action_routing_ready"));
    }
}

inline FSharFrontendScreenDefinition MakeFrontendScreen(
    const FName& ScreenId,
    const ESharFrontendLayer Layer,
    const TArray<FName>& Destinations
)
{
    FSharFrontendScreenDefinition Screen;
    Screen.ScreenId = ScreenId;
    Screen.Layer = Layer;
    Screen.ViewModelSchemaId = FName(TEXT("frontend_view_model_v1"));
    Screen.SemanticActionSetId = FName(TEXT("frontend_actions"));
    Screen.EntryPredicateId = FName(TEXT("frontend_entry_allowed"));
    Screen.ExitPolicyId = FName(TEXT("frontend_exit_transaction"));
    Screen.FocusPolicyId = FName(TEXT("restore_semantic_focus"));
    Screen.RequiredBundleIds = {FName(TEXT("frontend_base"))};
    Screen.AllowedDestinationScreenIds = Destinations;
    Screen.PreCommitRequirements = {
        ESharFrontendReadinessKind::DomainSnapshot,
        ESharFrontendReadinessKind::AssetBundles,
        ESharFrontendReadinessKind::ViewModel,
        ESharFrontendReadinessKind::LayerReservation,
    };
    Screen.PostCommitRequirements = {
        ESharFrontendReadinessKind::WidgetActivation,
        ESharFrontendReadinessKind::Focus,
        ESharFrontendReadinessKind::ActionRouting,
    };
    return Screen;
}

inline USharFrontendCatalogDefinition* MakeFrontendCatalogDefinition()
{
    auto* Definition = NewObject<USharFrontendCatalogDefinition>();
    Definition->CanonicalId = FName(TEXT("base_frontend"));
    Definition->DisplayName = FText::FromString(TEXT("Base Frontend Catalog"));
    Definition->SourcePackageIds = {FName(TEXT("frontend_contract"))};
    Definition->RevisionToken = TEXT("sha256:frontend_definition_v1");
    Definition->ValidationProfile = FName(TEXT("base_frontend_v1"));
    Definition->OwningFeature = FName(TEXT("base"));
    Definition->InitialScreenId = FName(TEXT("main_menu"));
    Definition->Screens = {
        MakeFrontendScreen(
            FName(TEXT("main_menu")),
            ESharFrontendLayer::Primary,
            {
                FName(TEXT("options")),
                FName(TEXT("gallery")),
                FName(TEXT("confirm_quit")),
            }
        ),
        MakeFrontendScreen(
            FName(TEXT("options")),
            ESharFrontendLayer::Primary,
            {FName(TEXT("main_menu"))}
        ),
        MakeFrontendScreen(
            FName(TEXT("gallery")),
            ESharFrontendLayer::Primary,
            {FName(TEXT("main_menu"))}
        ),
        MakeFrontendScreen(
            FName(TEXT("confirm_quit")),
            ESharFrontendLayer::Modal,
            {FName(TEXT("main_menu"))}
        ),
    };
    return Definition;
}

inline FSharFrontendRuntimeFixture MakeFrontendRuntime()
{
    FSharFrontendRuntimeFixture Fixture;
    Fixture.GameInstance = NewObject<UGameInstance>();
    Fixture.CatalogSubsystem =
        NewObject<USharFrontendCatalogSubsystem>(Fixture.GameInstance);
    Fixture.CatalogSubsystem->Configure(
        TEXT("sha256:frontend_catalog_v1"),
        FName(TEXT("base_frontend"))
    );
    Fixture.CatalogSubsystem->RegisterCatalog(
        MakeFrontendCatalogDefinition()
    );
    Fixture.CatalogSubsystem->Activate();
    Fixture.FlowSubsystem =
        NewObject<USharFrontendFlowSubsystem>(Fixture.GameInstance);
    Fixture.FlowSubsystem->Configure(
        Fixture.CatalogSubsystem,
        TEXT("sha256:frontend_flow_initial"),
        TEXT("sha256:main_menu_initial"),
        FName(TEXT("main_menu_primary_action"))
    );
    return Fixture;
}

inline FSharFrontendNavigationRequest MakeFrontendRequest(
    const FSharFrontendRuntimeFixture& Fixture,
    const FName& RequestId,
    const FName& DestinationScreenId,
    const ESharFrontendNavigationPriority Priority,
    const ESharFrontendHistoryPolicy HistoryPolicy
)
{
    const FSharFrontendFlowObservation& Observation =
        Fixture.FlowSubsystem->GetObservation();
    FSharFrontendNavigationRequest Request;
    Request.RequestId = RequestId;
    Request.SourceScreenId = Observation.ActiveModalScreenId.IsNone()
        ? Observation.ActivePrimaryScreenId
        : Observation.ActiveModalScreenId;
    Request.DestinationScreenId = DestinationScreenId;
    Request.CallerId = FName(TEXT("frontend_test"));
    Request.LocalPlayerId = FName(TEXT("local_player_one"));
    Request.Priority = Priority;
    Request.HistoryPolicy = HistoryPolicy;
    Request.CatalogRevision = Fixture.CatalogSubsystem->GetCatalogRevision();
    Request.FlowRevision = Observation.FlowRevision;
    Request.SourceScreenRevision = Observation.ActiveScreenRevision;
    Request.RequestRevision = TEXT("sha256:frontend_request_v1");
    return Request;
}

inline FSharFrontendReadinessEvidence MakeFrontendEvidence(
    const FSharFrontendRuntimeFixture& Fixture,
    const FName& RequestId,
    const ESharFrontendReadinessKind Kind,
    const ESharFrontendEvidenceStatus Status
)
{
    const FSharFrontendTransitionSnapshot* Transition =
        Fixture.FlowSubsystem->FindTransition(RequestId);
    FSharFrontendReadinessEvidence Evidence;
    Evidence.RequestId = RequestId;
    Evidence.EvidenceId = ReadinessEvidenceId(Kind);
    Evidence.Kind = Kind;
    Evidence.Status = Status;
    Evidence.CatalogRevision = Fixture.CatalogSubsystem->GetCatalogRevision();
    Evidence.RequestRevision = Transition == nullptr
        ? FString()
        : Transition->Request.RequestRevision;
    Evidence.DestinationScreenRevision = Transition == nullptr
        ? FString()
        : Transition->CandidateScreenRevision;
    Evidence.EvidenceRevision = TEXT("sha256:frontend_evidence_v1");
    return Evidence;
}

inline bool AcceptFrontendRequirements(
    const FSharFrontendRuntimeFixture& Fixture,
    const FName& RequestId,
    const TArray<ESharFrontendReadinessKind>& Requirements
)
{
    for (const ESharFrontendReadinessKind Kind : Requirements)
    {
        if (Fixture.FlowSubsystem->AcceptEvidence(
                MakeFrontendEvidence(
                    Fixture,
                    RequestId,
                    Kind,
                    ESharFrontendEvidenceStatus::Ready
                )
            ) != ESharFrontendOperationResult::Accepted)
        {
            return false;
        }
    }
    return true;
}

inline bool AcceptFrontendPreCommit(
    const FSharFrontendRuntimeFixture& Fixture,
    const FName& RequestId
)
{
    return AcceptFrontendRequirements(
        Fixture,
        RequestId,
        {
            ESharFrontendReadinessKind::DomainSnapshot,
            ESharFrontendReadinessKind::AssetBundles,
            ESharFrontendReadinessKind::ViewModel,
            ESharFrontendReadinessKind::LayerReservation,
        }
    );
}

inline bool AcceptFrontendPostCommit(
    const FSharFrontendRuntimeFixture& Fixture,
    const FName& RequestId
)
{
    return AcceptFrontendRequirements(
        Fixture,
        RequestId,
        {
            ESharFrontendReadinessKind::WidgetActivation,
            ESharFrontendReadinessKind::Focus,
            ESharFrontendReadinessKind::ActionRouting,
        }
    );
}
