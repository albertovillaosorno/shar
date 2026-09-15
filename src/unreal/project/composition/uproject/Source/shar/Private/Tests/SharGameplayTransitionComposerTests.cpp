// Copyright:
//   - Copyright © 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT
//
// Boundary-Contract:
// - Owns:
//   - Gameplay transition composition integration Automation.
// - Must-Not:
//   - Depend on authored worlds, mapping assets, or production data assets.
// - Allows:
//   - Synthetic definitions with real application, loading, world, and input
//   - runtime authorities.
// - Split-When:
//   - Recovery-path coverage requires independent fixture topology.
// - Merge-When:
//   - Another suite proves the same correlated gameplay commit boundary.
// - Summary:
//   - Verifies readiness correlation and post-mode-commit input activation.
// - Description:
//   - Proves stale transition evidence fails closed and gameplay input remains
//   - inactive until the application coordinator commits gameplay.
// - Usage:
//   - Runs in headless Automation with transient native Unreal objects.
// - Defaults:
//   - Uses one local player, one mapping context, and one required load node.
//

//! Correlated gameplay transition composition integration Automation.

#if WITH_DEV_AUTOMATION_TESTS

#include "Runtime/SharGameplayTransitionComposer.h"

#include "Application/SharApplicationModeCatalogSubsystem.h"
#include "Application/SharApplicationModeCoordinator.h"
#include "Application/SharApplicationModeDefinition.h"
#include "Catalog/SharGameplayCatalog.h"
#include "Catalog/SharGameplayCatalogSubsystem.h"
#include "EnhancedInputSubsystems.h"
#include "EnhancedPlayerInput.h"
#include "Engine/Engine.h"
#include "Engine/GameInstance.h"
#include "Engine/LocalPlayer.h"
#include "Engine/World.h"
#include "GameFramework/PlayerController.h"
#include "Input/SharLocalPlayerInputSubsystem.h"
#include "InputMappingContext.h"
#include "Loading/SharLoadCoordinatorSubsystem.h"
#include "Loading/SharWorldReadinessSubsystem.h"
#include "Misc/AutomationTest.h"

namespace
{
constexpr double GameplayTransitionDeadlineSeconds = 30.0;
constexpr int32 GameplayInputPriority = 100;

USharApplicationModeDefinition* MakeComposerMode(
    const FName& Id,
    const ESharApplicationModeKind Kind,
    const TArray<FName>& Predecessors,
    const TArray<FName>& Successors,
    const TArray<FName>& RequiredServices = {}
)
{
    auto* Mode = NewObject<USharApplicationModeDefinition>();
    Mode->CanonicalId = Id;
    Mode->DisplayName = FText::FromName(Id);
    Mode->SourcePackageIds = {FName(TEXT("gameplay_composer_contract"))};
    Mode->RevisionToken = FString::Printf(TEXT("sha256:%s_v1"), *Id.ToString());
    Mode->ValidationProfile = FName(TEXT("application_mode_v1"));
    Mode->OwningFeature = FName(TEXT("base"));
    Mode->ModeKind = Kind;
    Mode->AllowedPredecessorIds = Predecessors;
    Mode->AllowedSuccessorIds = Successors;
    Mode->EntryPlanId = FName(TEXT("entry_plan"));
    Mode->ExitPlanId = FName(TEXT("exit_plan"));
    Mode->RequiredServiceIds = RequiredServices;
    Mode->ReadinessBarrierId = FName(TEXT("gameplay_mode_ready"));
    Mode->bSupportsCancellation = true;
    Mode->bHasBoundedTimeout = true;
    return Mode;
}

struct FComposerApplicationFixture
{
    USharApplicationModeCatalogSubsystem* Catalog = nullptr;
    USharApplicationModeCoordinator* Coordinator = nullptr;
};

FComposerApplicationFixture MakeComposerApplication(UGameInstance& GameInstance)
{
    TArray<USharApplicationModeDefinition*> Modes = {
        MakeComposerMode(
            FName(TEXT("entry")),
            ESharApplicationModeKind::Entry,
            {},
            {FName(TEXT("loading_gameplay"))}
        ),
        MakeComposerMode(
            FName(TEXT("loading_gameplay")),
            ESharApplicationModeKind::Loading,
            {FName(TEXT("entry"))},
            {FName(TEXT("gameplay"))}
        ),
        MakeComposerMode(
            FName(TEXT("gameplay")),
            ESharApplicationModeKind::Active,
            {FName(TEXT("loading_gameplay"))},
            {FName(TEXT("exit"))},
            {
                FName(TEXT("loading_service")),
                FName(TEXT("world_service")),
                FName(TEXT("input_service")),
            }
        ),
        MakeComposerMode(
            FName(TEXT("exit")),
            ESharApplicationModeKind::Exit,
            {FName(TEXT("gameplay"))},
            {}
        ),
    };
    Modes[0]->WorldPolicy = ESharApplicationWorldPolicy::None;
    Modes[0]->SessionPolicy = ESharApplicationSessionPolicy::None;
    Modes[1]->WorldPolicy = ESharApplicationWorldPolicy::Prepare;
    Modes[1]->SessionPolicy = ESharApplicationSessionPolicy::Prepare;
    Modes[2]->WorldPolicy = ESharApplicationWorldPolicy::Own;
    Modes[2]->SessionPolicy = ESharApplicationSessionPolicy::Own;
    Modes[3]->WorldPolicy = ESharApplicationWorldPolicy::TearDown;
    Modes[3]->SessionPolicy = ESharApplicationSessionPolicy::TearDown;
    Modes[1]->SuccessModeId = FName(TEXT("gameplay"));
    Modes[1]->RecoveryModeId = FName(TEXT("entry"));
    Modes[2]->RecoveryModeId = FName(TEXT("entry"));

    auto* RootDefinition = NewObject<USharGameplayCatalog>();
    RootDefinition->CanonicalId = FName(TEXT("gameplay"));
    RootDefinition->DisplayName = FText::FromString(TEXT("Gameplay catalog"));
    RootDefinition->SourcePackageIds = {
        FName(TEXT("gameplay_composer_catalog"))
    };
    RootDefinition->RevisionToken = TEXT("sha256:composer_catalog_v1");
    RootDefinition->ValidationProfile = FName(TEXT("gameplay_catalog_v1"));
    RootDefinition->OwningFeature = FName(TEXT("base"));
    FSharGameplayCatalogFamily Family;
    Family.FamilyId = FName(TEXT("application_modes"));
    Family.PrimaryAssetTypeName = FName(TEXT("SharApplicationMode"));
    for (const USharApplicationModeDefinition* Mode : Modes)
    {
        Family.DefinitionIds.Add(Mode->GetPrimaryAssetId());
    }
    RootDefinition->Families.Add(Family);

    auto* RootCatalog =
        NewObject<USharGameplayCatalogSubsystem>(&GameInstance);
    RootCatalog->Activate(RootDefinition);
    auto* Catalog =
        NewObject<USharApplicationModeCatalogSubsystem>(&GameInstance);
    Catalog->ConfigureRootCatalog(RootCatalog);
    for (USharApplicationModeDefinition* Mode : Modes)
    {
        Catalog->RegisterMode(Mode);
    }
    Catalog->Activate();

    auto* Coordinator =
        NewObject<USharApplicationModeCoordinator>(&GameInstance);
    FSharApplicationModeObservation Initial;
    Initial.ActiveModeId = FName(TEXT("loading_gameplay"));
    Initial.ActiveModeRevision = TEXT("sha256:loading_gameplay_v1");
    Initial.WorldId = FName(TEXT("springfield_world"));
    Initial.WorldRevision = TEXT("sha256:springfield_world_v1");
    Initial.ProfileRevision.Reset();
    Initial.SessionRevision = TEXT("sha256:gameplay_session_v1");
    Coordinator->Configure(Catalog, Initial);
    return {Catalog, Coordinator};
}

FSharApplicationModeRequest MakeGameplayApplicationRequest()
{
    FSharApplicationModeRequest Request;
    Request.RequestId = FName(TEXT("commit_gameplay"));
    Request.SourceModeId = FName(TEXT("loading_gameplay"));
    Request.TargetModeId = FName(TEXT("gameplay"));
    Request.ReasonId = FName(TEXT("loading_ready"));
    Request.CallerId = FName(TEXT("gameplay_composer"));
    Request.Priority = ESharApplicationTransitionPriority::Gameplay;
    Request.CatalogRevision = TEXT("sha256:composer_catalog_v1");
    Request.SourceModeRevision = TEXT("sha256:loading_gameplay_v1");
    Request.TargetModeRevision = TEXT("sha256:gameplay_v1");
    Request.SessionRevision = TEXT("sha256:gameplay_session_v1");
    Request.ProfileRevision.Reset();
    Request.WorldId = FName(TEXT("springfield_world"));
    Request.WorldRevision = TEXT("sha256:springfield_world_v1");
    Request.RequestRevision = TEXT("sha256:commit_gameplay_v1");
    Request.DeadlineSeconds = GameplayTransitionDeadlineSeconds;
    return Request;
}

USharLoadCoordinatorSubsystem* MakeGameplayLoad(
    UGameInstance& GameInstance,
    const FSharApplicationModeRequest& ApplicationRequest,
    const FName& WorldBarrierId
)
{
    auto* Load = NewObject<USharLoadCoordinatorSubsystem>(&GameInstance);
    Load->ConfigureCatalog(ApplicationRequest.CatalogRevision);
    FSharLoadPlan Plan;
    Plan.PlanId = FName(TEXT("gameplay_world_plan"));
    Plan.PlanRevision = TEXT("sha256:gameplay_world_plan_v1");
    FSharLoadPlanNode Node;
    Node.NodeId = FName(TEXT("world_package_ready"));
    Node.DependencyKey = FName(TEXT("springfield_world_package"));
    Node.NodeKind = ESharLoadNodeKind::WorldPreparation;
    Plan.Nodes.Add(Node);
    if (!Load->RegisterPlan(Plan))
    {
        return Load;
    }

    FSharLoadRequest Request;
    Request.RequestId = FName(TEXT("load_gameplay_world"));
    Request.PlanId = Plan.PlanId;
    Request.ScopeId = ApplicationRequest.WorldId;
    Request.CallerId = FName(TEXT("gameplay_composer"));
    Request.Priority = 100;
    Request.AssetIds.Add(FPrimaryAssetId(
        FPrimaryAssetType(TEXT("SharWorld")),
        ApplicationRequest.WorldId
    ));
    Request.CatalogRevision = ApplicationRequest.CatalogRevision;
    Request.ScopeRevision = ApplicationRequest.WorldRevision;
    Request.RequestRevision = ApplicationRequest.RequestRevision;
    Request.DeadlineSeconds = GameplayTransitionDeadlineSeconds;
    Request.ReadinessBarrierId = WorldBarrierId;
    if (Load->Submit(Request) != ESharLoadOperationResult::Accepted
        || Load->BeginRequest(Request.RequestId)
            != ESharLoadOperationResult::Accepted)
    {
        return Load;
    }
    const FName AttemptId(TEXT("world_package_attempt"));
    if (Load->BeginNode({
            .RequestId = Request.RequestId,
            .NodeId = Node.NodeId,
            .AttemptId = AttemptId,
        }) != ESharLoadOperationResult::Accepted)
    {
        return Load;
    }
    FSharLoadCallbackRevision Revision;
    Revision.CatalogRevision = Request.CatalogRevision;
    Revision.ScopeRevision = Request.ScopeRevision;
    Revision.RequestRevision = Request.RequestRevision;
    Revision.AttemptId = AttemptId;
    if (Load->CompleteNode({
            .RequestId = Request.RequestId,
            .NodeId = Node.NodeId,
            .Revision = Revision,
        }) != ESharLoadOperationResult::Accepted
        || Load->BeginVerification(Request.RequestId)
            != ESharLoadOperationResult::Accepted)
    {
        return Load;
    }
    Revision.AttemptId = FName(TEXT("world_barrier_attempt"));
    Load->AcceptBarrier({
        .RequestId = Request.RequestId,
        .BarrierId = WorldBarrierId,
        .Revision = Revision,
    });
    return Load;
}

FSharWorldReadinessBarrier MakeComposerWorldBarrier(
    const FSharApplicationModeRequest& ApplicationRequest,
    const FName& BarrierId,
    const FString& TransitionRevision
)
{
    FSharWorldReadinessBarrier Barrier;
    Barrier.BarrierId = BarrierId;
    Barrier.WorldId = ApplicationRequest.WorldId;
    Barrier.WorldRevision = ApplicationRequest.WorldRevision;
    Barrier.TransitionRevision = TransitionRevision;
    Barrier.RequiredCheckpointIds = {FName(TEXT("gameplay_world_ready"))};
    return Barrier;
}

FSharGameplayTransitionCompositionRequest MakeCompositionRequest(
    USharLocalPlayerInputSubsystem& Input,
    const FName& WorldBarrierId
)
{
    FSharGameplayTransitionCompositionRequest Request;
    Request.ApplicationRequestId = FName(TEXT("commit_gameplay"));
    Request.LoadRequestId = FName(TEXT("load_gameplay_world"));
    Request.WorldBarrierId = WorldBarrierId;
    Request.LoadingServiceId = FName(TEXT("loading_service"));
    Request.WorldServiceId = FName(TEXT("world_service"));
    Request.InputServiceId = FName(TEXT("input_service"));
    Request.InputLeases.Add({&Input, FName(TEXT("gameplay_input_lease"))});
    return Request;
}

IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharGameplayTransitionComposerTest,
    "SHAR.Runtime.GameplayTransition.ReadinessAndCommit",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
} // namespace

bool FSharGameplayTransitionComposerTest::RunTest(const FString& Parameters)
{
    (void)Parameters;
    auto* GameInstance = NewObject<UGameInstance>();
    FComposerApplicationFixture Application =
        MakeComposerApplication(*GameInstance);
    const FSharApplicationModeRequest ApplicationRequest =
        MakeGameplayApplicationRequest();
    TestTrue(
        TEXT("Gameplay transition submits"),
        Application.Coordinator->Submit(ApplicationRequest)
            == ESharApplicationOperationResult::Accepted
    );
    TestTrue(
        TEXT("Gameplay transition begins preparation"),
        Application.Coordinator->Begin(ApplicationRequest.RequestId)
            == ESharApplicationOperationResult::Accepted
    );

    const FName WorldBarrierId(TEXT("springfield_gameplay_ready"));
    USharLoadCoordinatorSubsystem* Load = MakeGameplayLoad(
        *GameInstance,
        ApplicationRequest,
        WorldBarrierId
    );
    auto* WorldReadiness = NewObject<USharWorldReadinessSubsystem>();
    WorldReadiness->ConfigureWorld(
        ApplicationRequest.WorldId,
        ApplicationRequest.WorldRevision
    );
    const FSharWorldReadinessBarrier StaleWorldBarrier =
        MakeComposerWorldBarrier(
            ApplicationRequest,
            WorldBarrierId,
            TEXT("sha256:stale_transition")
        );
    WorldReadiness->RegisterBarrier(StaleWorldBarrier);
    WorldReadiness->CompleteCheckpoint({
        .BarrierId = WorldBarrierId,
        .CheckpointId = FName(TEXT("gameplay_world_ready")),
        .WorldRevision = ApplicationRequest.WorldRevision,
        .TransitionRevision = StaleWorldBarrier.TransitionRevision,
    });

    UWorld* InputWorld = UWorld::CreateWorld(EWorldType::Game, false);
    auto* PlayerController = InputWorld->SpawnActor<APlayerController>();
    auto* LocalPlayer = NewObject<ULocalPlayer>(GEngine);
    LocalPlayer->PlayerAdded(nullptr, 0);
    LocalPlayer->SwitchController(PlayerController);
    PlayerController->PlayerInput = NewObject<UEnhancedPlayerInput>(
        PlayerController
    );
    USharLocalPlayerInputSubsystem* Input =
        LocalPlayer->GetSubsystem<USharLocalPlayerInputSubsystem>();
    UEnhancedInputLocalPlayerSubsystem* EnhancedInput =
        LocalPlayer->GetSubsystem<UEnhancedInputLocalPlayerSubsystem>();
    auto* MappingContext = NewObject<UInputMappingContext>();
    FSharInputContextLeaseRequest InputLease;
    InputLease.LeaseId = FName(TEXT("gameplay_input_lease"));
    InputLease.ContextId = FName(TEXT("gameplay_on_foot"));
    InputLease.OwnerModeId = ApplicationRequest.TargetModeId;
    InputLease.OwnerModeRevision = ApplicationRequest.TargetModeRevision;
    InputLease.TransitionRevision = ApplicationRequest.RequestRevision;
    InputLease.LeaseRevision = TEXT("sha256:gameplay_input_lease_v1");
    InputLease.MappingContext = MappingContext;
    InputLease.Priority = GameplayInputPriority;
    TestTrue(
        TEXT("Gameplay input stages without native activation"),
        Input->StageLease(InputLease) == ESharInputContextLeaseResult::Accepted
            && !EnhancedInput->HasMappingContext(MappingContext)
    );

    FSharApplicationTransitionSnapshot ApplicationSnapshot;
    TestTrue(
        TEXT("Application snapshot remains in preparing"),
        Application.Coordinator->GetTransitionSnapshot(
            ApplicationRequest.RequestId,
            ApplicationSnapshot
        ) && ApplicationSnapshot.State
            == ESharApplicationTransitionState::Preparing
    );
    TestTrue(
        TEXT("Loading reaches correlated ready-to-commit"),
        Load->GetState(FName(TEXT("load_gameplay_world")))
            == ESharLoadRequestState::ReadyToCommit
    );
    TestTrue(
        TEXT("World readiness barrier is ready"),
        WorldReadiness->IsReady(WorldBarrierId)
    );
    TestTrue(
        TEXT("Input lease is commit-ready without activation"),
        Input->CheckCommitReadiness(
            InputLease.LeaseId,
            InputLease.LeaseRevision,
            InputLease.TransitionRevision
        ) == ESharInputContextLeaseResult::Accepted
    );

    auto* Composer = NewObject<USharGameplayTransitionComposer>(GameInstance);
    const FSharGameplayTransitionCompositionRequest Composition =
        MakeCompositionRequest(*Input, WorldBarrierId);
    TestTrue(
        TEXT("Ready world from stale transition fails closed"),
        Composer->PrepareGameplayReadiness(
            Composition,
            Application.Catalog,
            Application.Coordinator,
            Load,
            WorldReadiness
        ) == ESharGameplayTransitionCompositionResult::StaleRevision
    );
    TestFalse(
        TEXT("Stale readiness cannot activate gameplay input"),
        EnhancedInput->HasMappingContext(MappingContext)
    );
    WorldReadiness->ConfigureWorld(
        ApplicationRequest.WorldId,
        ApplicationRequest.WorldRevision
    );
    const FSharWorldReadinessBarrier WorldBarrier = MakeComposerWorldBarrier(
        ApplicationRequest,
        WorldBarrierId,
        ApplicationRequest.RequestRevision
    );
    WorldReadiness->RegisterBarrier(WorldBarrier);
    TestTrue(
        TEXT("World checkpoint completes for current transition"),
        WorldReadiness->CompleteCheckpoint({
                .BarrierId = WorldBarrierId,
                .CheckpointId = FName(TEXT("gameplay_world_ready")),
                .WorldRevision = ApplicationRequest.WorldRevision,
                .TransitionRevision = ApplicationRequest.RequestRevision,
            }) == ESharWorldReadinessResult::Accepted
    );
    const ESharGameplayTransitionCompositionResult PrepareResult =
        Composer->PrepareGameplayReadiness(
            Composition,
            Application.Catalog,
            Application.Coordinator,
            Load,
            WorldReadiness
        );
    TestTrue(
        TEXT("Correlated readiness publishes required services"),
        PrepareResult == ESharGameplayTransitionCompositionResult::Accepted
    );
    TestFalse(
        TEXT("Readiness publication does not activate gameplay input"),
        EnhancedInput->HasMappingContext(MappingContext)
    );
    TestTrue(
        TEXT("Loading mode remains active during readiness"),
        Application.Coordinator->GetObservation().ActiveModeId
            == FName(TEXT("loading_gameplay"))
    );
    TestTrue(
        TEXT("Required services unlock application readiness verification"),
        Application.Coordinator->BeginReadinessVerification(
            ApplicationRequest.RequestId
        ) == ESharApplicationOperationResult::Accepted
    );
    FSharApplicationBarrierEvidence ApplicationBarrier;
    ApplicationBarrier.RequestId = ApplicationRequest.RequestId;
    ApplicationBarrier.BarrierId = FName(TEXT("gameplay_mode_ready"));
    ApplicationBarrier.CatalogRevision = ApplicationRequest.CatalogRevision;
    ApplicationBarrier.RequestRevision = ApplicationRequest.RequestRevision;
    ApplicationBarrier.TargetModeRevision =
        ApplicationRequest.TargetModeRevision;
    TestTrue(
        TEXT("Application readiness barrier accepts correlated transition"),
        Application.Coordinator->AcceptBarrier(ApplicationBarrier)
            == ESharApplicationOperationResult::Accepted
    );
    TestTrue(
        TEXT("Composer commits mode before activating input"),
        Composer->CommitGameplay(
            Composition,
            FSharApplicationLifecycleEvidence{
                .RequestId = ApplicationRequest.RequestId,
                .Phase = ESharApplicationLifecyclePhase::SourceExit,
                .ModeId = ApplicationRequest.SourceModeId,
                .PlanId = FName(TEXT("exit_plan")),
                .CatalogRevision = ApplicationRequest.CatalogRevision,
                .RequestRevision = ApplicationRequest.RequestRevision,
                .ModeRevision = ApplicationRequest.SourceModeRevision,
            },
            FSharApplicationLifecycleEvidence{
                .RequestId = ApplicationRequest.RequestId,
                .Phase = ESharApplicationLifecyclePhase::TargetEntry,
                .ModeId = ApplicationRequest.TargetModeId,
                .PlanId = FName(TEXT("entry_plan")),
                .CatalogRevision = ApplicationRequest.CatalogRevision,
                .RequestRevision = ApplicationRequest.RequestRevision,
                .ModeRevision = ApplicationRequest.TargetModeRevision,
            },
            Application.Catalog,
            Application.Coordinator,
            Load,
            WorldReadiness
        ) == ESharGameplayTransitionCompositionResult::Accepted
    );

    const FSharApplicationModeObservation Observation =
        Application.Coordinator->GetObservation();
    TestTrue(
        TEXT("Committed gameplay publishes world identity and revision"),
        Observation.ActiveModeId == FName(TEXT("gameplay"))
            && Observation.WorldId == ApplicationRequest.WorldId
            && Observation.WorldRevision == ApplicationRequest.WorldRevision
    );
    TestTrue(
        TEXT("Gameplay input activates only after committed mode"),
        EnhancedInput->HasMappingContext(MappingContext)
            && Input->GetActiveLeaseCount() == 1
    );
    TestTrue(
        TEXT("Loading publishes success after gameplay commit"),
        Load->GetTerminalResult(FName(TEXT("load_gameplay_world")))
            == ESharLoadTerminalResult::Success
    );
    TestTrue(
        TEXT("Application transition publishes success"),
        Application.Coordinator->GetTerminalResult(
            ApplicationRequest.RequestId
        ) == ESharApplicationTerminalResult::Success
    );

    Input->ReleaseLease(InputLease.LeaseId, InputLease.LeaseRevision);
    Application.Coordinator->Release(ApplicationRequest.RequestId);
    Load->Release(FName(TEXT("load_gameplay_world")));
    LocalPlayer->PlayerRemoved();
    InputWorld->DestroyWorld(false);
    return true;
}

#endif
