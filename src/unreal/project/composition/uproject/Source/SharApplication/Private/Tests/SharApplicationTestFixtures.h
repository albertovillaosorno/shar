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
//   - Shar application test fixtures composition module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Shar application test fixtures composition module.
// - Description:
//   - Implements the declared composition module responsibility for project.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Shar application test fixtures composition module.

#pragma once

#include "Application/SharApplicationModeCatalogSubsystem.h"
#include "Application/SharApplicationModeCoordinator.h"
#include "Application/SharApplicationModeDefinition.h"
#include "Catalog/SharGameplayCatalog.h"
#include "Catalog/SharGameplayCatalogSubsystem.h"

#include "Engine/GameInstance.h"

constexpr double DefaultApplicationDeadlineSeconds = 30.0;

enum class ESharApplicationCatalogShape : uint8
{
    Valid,
    BrokenReciprocalEdge,
    RecoveryToExit,
    MissingOverlayReturn,
};

struct FSharApplicationModeFixture
{
    FName ModeId;
    ESharApplicationModeKind ModeKind = ESharApplicationModeKind::Active;
    TArray<FName> PredecessorIds;
    TArray<FName> SuccessorIds;
    TArray<FName> RequiredServiceIds;
    FName SuccessModeId;
    FName RecoveryModeId;
    FName ReturnModeId;
    ESharApplicationWorldPolicy WorldPolicy =
        ESharApplicationWorldPolicy::None;
    ESharApplicationSessionPolicy SessionPolicy =
        ESharApplicationSessionPolicy::None;
    ESharApplicationProgressionPolicy ProgressionPolicy =
        ESharApplicationProgressionPolicy::None;
    bool bDemonstrationMode = false;
};

struct FSharApplicationRequestFixture
{
    FName RequestId;
    ESharApplicationTransitionPriority Priority =
        ESharApplicationTransitionPriority::User;
    FName CallerId;
};

struct FSharApplicationServiceEvidenceFixture
{
    FName RequestId;
    FName ServiceId;
    ESharApplicationServiceStatus Status =
        ESharApplicationServiceStatus::Ready;
};

struct FSharApplicationRuntimeFixture
{
    UGameInstance* GameInstance = nullptr;
    USharApplicationModeCatalogSubsystem* Catalog = nullptr;
    USharApplicationModeCoordinator* Coordinator = nullptr;
};

inline void FillApplicationDefinitionBase(
    USharApplicationModeDefinition& Definition,
    const FName& ModeId
)
{
    Definition.CanonicalId = ModeId;
    Definition.DisplayName = FText::FromString(ModeId.ToString());
    Definition.SourcePackageIds = {FName(TEXT("application_mode_contract"))};
    Definition.RevisionToken = FString::Printf(
        TEXT("sha256:%s_v1"),
        *ModeId.ToString()
    );
    Definition.ValidationProfile = FName(TEXT("application_mode_v1"));
    Definition.OwningFeature = FName(TEXT("base"));
    Definition.EntryPlanId = FName(TEXT("mode_entry_plan_v1"));
    Definition.ExitPlanId = FName(TEXT("mode_exit_plan_v1"));
    Definition.ReadinessBarrierId = FName(TEXT("mode_ready_barrier_v1"));
    Definition.bSupportsCancellation = true;
    Definition.bHasBoundedTimeout = true;
}

inline USharApplicationModeDefinition* MakeApplicationMode(
    const FSharApplicationModeFixture& Fixture
)
{
    auto* Definition = NewObject<USharApplicationModeDefinition>();
    FillApplicationDefinitionBase(*Definition, Fixture.ModeId);
    Definition->ModeKind = Fixture.ModeKind;
    Definition->AllowedPredecessorIds = Fixture.PredecessorIds;
    Definition->AllowedSuccessorIds = Fixture.SuccessorIds;
    Definition->RequiredServiceIds = Fixture.RequiredServiceIds;
    Definition->SuccessModeId = Fixture.SuccessModeId;
    Definition->RecoveryModeId = Fixture.RecoveryModeId;
    Definition->ReturnModeId = Fixture.ReturnModeId;
    Definition->WorldPolicy = Fixture.WorldPolicy;
    Definition->SessionPolicy = Fixture.SessionPolicy;
    Definition->ProgressionPolicy = Fixture.ProgressionPolicy;
    Definition->bDemonstrationMode = Fixture.bDemonstrationMode;
    return Definition;
}

inline TArray<USharApplicationModeDefinition*> MakeApplicationModes(
    const ESharApplicationCatalogShape Shape
)
{
    TArray<USharApplicationModeDefinition*> Modes;
    Modes.Add(MakeApplicationMode({
        .ModeId = FName(TEXT("entry")),
        .ModeKind = ESharApplicationModeKind::Entry,
        .PredecessorIds = {},
        .SuccessorIds = {FName(TEXT("front_end"))},
        .RequiredServiceIds = {},
        .SuccessModeId = FName(),
        .RecoveryModeId = FName(),
        .ReturnModeId = FName(),
        .WorldPolicy = ESharApplicationWorldPolicy::None,
        .SessionPolicy = ESharApplicationSessionPolicy::None,
        .ProgressionPolicy = ESharApplicationProgressionPolicy::None,
        .bDemonstrationMode = false,
    }));
    Modes.Add(MakeApplicationMode({
        .ModeId = FName(TEXT("front_end")),
        .ModeKind = ESharApplicationModeKind::FrontEnd,
        .PredecessorIds = {FName(TEXT("entry"))},
        .SuccessorIds = {FName(TEXT("loading_gameplay"))},
        .RequiredServiceIds = {},
        .SuccessModeId = FName(),
        .RecoveryModeId = FName(),
        .ReturnModeId = FName(),
        .WorldPolicy = ESharApplicationWorldPolicy::None,
        .SessionPolicy = ESharApplicationSessionPolicy::None,
        .ProgressionPolicy = ESharApplicationProgressionPolicy::ReadOnly,
        .bDemonstrationMode = false,
    }));
    const TArray<FName> LoadingPredecessors =
        Shape == ESharApplicationCatalogShape::BrokenReciprocalEdge
        ? TArray<FName>{FName(TEXT("entry"))}
        : TArray<FName>{FName(TEXT("front_end"))};
    Modes.Add(MakeApplicationMode({
        .ModeId = FName(TEXT("loading_gameplay")),
        .ModeKind = ESharApplicationModeKind::Loading,
        .PredecessorIds = LoadingPredecessors,
        .SuccessorIds = {FName(TEXT("gameplay"))},
        .RequiredServiceIds = {
            FName(TEXT("catalog_service")),
            FName(TEXT("world_service")),
        },
        .SuccessModeId = FName(TEXT("gameplay")),
        .RecoveryModeId = Shape == ESharApplicationCatalogShape::RecoveryToExit
            ? FName(TEXT("exit"))
            : FName(TEXT("front_end")),
        .ReturnModeId = FName(),
        .WorldPolicy = ESharApplicationWorldPolicy::Prepare,
        .SessionPolicy = ESharApplicationSessionPolicy::Prepare,
        .ProgressionPolicy = ESharApplicationProgressionPolicy::ReadOnly,
        .bDemonstrationMode = false,
    }));
    Modes.Add(MakeApplicationMode({
        .ModeId = FName(TEXT("gameplay")),
        .ModeKind = ESharApplicationModeKind::Active,
        .PredecessorIds = {
            FName(TEXT("loading_gameplay")),
            FName(TEXT("pause")),
        },
        .SuccessorIds = {
            FName(TEXT("pause")),
            FName(TEXT("exit")),
        },
        .RequiredServiceIds = {},
        .SuccessModeId = FName(),
        .RecoveryModeId = FName(TEXT("front_end")),
        .ReturnModeId = FName(),
        .WorldPolicy = ESharApplicationWorldPolicy::Own,
        .SessionPolicy = ESharApplicationSessionPolicy::Own,
        .ProgressionPolicy = ESharApplicationProgressionPolicy::Durable,
        .bDemonstrationMode = false,
    }));
    Modes.Add(MakeApplicationMode({
        .ModeId = FName(TEXT("pause")),
        .ModeKind = ESharApplicationModeKind::Overlay,
        .PredecessorIds = {FName(TEXT("gameplay"))},
        .SuccessorIds = {FName(TEXT("gameplay"))},
        .RequiredServiceIds = {},
        .SuccessModeId = FName(),
        .RecoveryModeId = FName(TEXT("front_end")),
        .ReturnModeId = Shape
                == ESharApplicationCatalogShape::MissingOverlayReturn
            ? FName(TEXT("missing_gameplay"))
            : FName(TEXT("gameplay")),
        .WorldPolicy = ESharApplicationWorldPolicy::Retain,
        .SessionPolicy = ESharApplicationSessionPolicy::Retain,
        .ProgressionPolicy = ESharApplicationProgressionPolicy::ReadOnly,
        .bDemonstrationMode = false,
    }));
    Modes.Add(MakeApplicationMode({
        .ModeId = FName(TEXT("exit")),
        .ModeKind = ESharApplicationModeKind::Exit,
        .PredecessorIds = {FName(TEXT("gameplay"))},
        .SuccessorIds = {},
        .RequiredServiceIds = {},
        .SuccessModeId = FName(),
        .RecoveryModeId = FName(),
        .ReturnModeId = FName(),
        .WorldPolicy = ESharApplicationWorldPolicy::TearDown,
        .SessionPolicy = ESharApplicationSessionPolicy::TearDown,
        .ProgressionPolicy = ESharApplicationProgressionPolicy::None,
        .bDemonstrationMode = false,
    }));
    return Modes;
}

inline USharApplicationModeCatalogSubsystem* MakeApplicationCatalog(
    UGameInstance& GameInstance,
    const ESharApplicationCatalogShape Shape,
    const bool bActivate
)
{
    const TArray<USharApplicationModeDefinition*> Modes =
        MakeApplicationModes(Shape);

    auto* RootDefinition = NewObject<USharGameplayCatalog>();
    RootDefinition->CanonicalId = FName(TEXT("gameplay"));
    RootDefinition->DisplayName = FText::FromString(TEXT("Gameplay catalog"));
    RootDefinition->SourcePackageIds = {
        FName(TEXT("application_catalog_contract"))
    };
    RootDefinition->RevisionToken = TEXT("sha256:application_catalog_v1");
    RootDefinition->ValidationProfile =
        FName(TEXT("gameplay_catalog_v1"));
    RootDefinition->OwningFeature = FName(TEXT("base"));

    FSharGameplayCatalogFamily ModeFamily;
    ModeFamily.FamilyId = FName(TEXT("application_modes"));
    ModeFamily.PrimaryAssetTypeName = FName(TEXT("SharApplicationMode"));
    for (const USharApplicationModeDefinition* Mode : Modes)
    {
        ModeFamily.DefinitionIds.Add(Mode->GetPrimaryAssetId());
    }
    RootDefinition->Families.Add(ModeFamily);

    auto* RootCatalog = NewObject<USharGameplayCatalogSubsystem>(
        &GameInstance
    );
    RootCatalog->Activate(RootDefinition);

    auto* Catalog = NewObject<USharApplicationModeCatalogSubsystem>(
        &GameInstance
    );
    Catalog->ConfigureRootCatalog(RootCatalog);
    for (USharApplicationModeDefinition* Mode : Modes)
    {
        Catalog->RegisterMode(Mode);
    }
    if (bActivate)
    {
        Catalog->Activate();
    }
    return Catalog;
}

inline FSharApplicationModeObservation MakeInitialApplicationObservation()
{
    FSharApplicationModeObservation Observation;
    Observation.ActiveModeId = FName(TEXT("front_end"));
    Observation.ActiveModeRevision = TEXT("sha256:front_end_v1");
    Observation.WorldId = FName();
    Observation.WorldRevision.Reset();
    Observation.ProfileRevision = TEXT("sha256:profile_v1");
    Observation.SessionRevision.Reset();
    return Observation;
}

inline FSharApplicationRuntimeFixture MakeApplicationRuntime()
{
    FSharApplicationRuntimeFixture Fixture;
    Fixture.GameInstance = NewObject<UGameInstance>();
    Fixture.Catalog = MakeApplicationCatalog(
        *Fixture.GameInstance,
        ESharApplicationCatalogShape::Valid,
        true
    );
    Fixture.Coordinator = NewObject<USharApplicationModeCoordinator>(
        Fixture.GameInstance
    );
    Fixture.Coordinator->Configure(
        Fixture.Catalog,
        MakeInitialApplicationObservation()
    );
    return Fixture;
}

inline FSharApplicationModeRequest MakeApplicationRequest(
    const FSharApplicationRequestFixture& Fixture
)
{
    FSharApplicationModeRequest Request;
    Request.RequestId = Fixture.RequestId;
    Request.SourceModeId = FName(TEXT("front_end"));
    Request.TargetModeId = FName(TEXT("loading_gameplay"));
    Request.ReasonId = FName(TEXT("start_gameplay"));
    Request.CallerId = Fixture.CallerId;
    Request.Priority = Fixture.Priority;
    Request.CatalogRevision = TEXT("sha256:application_catalog_v1");
    Request.SourceModeRevision = TEXT("sha256:front_end_v1");
    Request.TargetModeRevision = TEXT("sha256:loading_gameplay_v1");
    Request.SessionRevision = TEXT("sha256:gameplay_session_v1");
    Request.ProfileRevision = TEXT("sha256:profile_v1");
    Request.WorldId = FName(TEXT("springfield_world"));
    Request.WorldRevision = TEXT("sha256:springfield_world_v1");
    Request.RequestRevision = TEXT("sha256:transition_v1");
    Request.ReturnModeId = FName(TEXT("front_end"));
    Request.DeadlineSeconds = DefaultApplicationDeadlineSeconds;
    return Request;
}

inline FSharApplicationServiceEvidence MakeApplicationServiceEvidence(
    const FSharApplicationServiceEvidenceFixture& Fixture
)
{
    FSharApplicationServiceEvidence Evidence;
    Evidence.RequestId = Fixture.RequestId;
    Evidence.ServiceId = Fixture.ServiceId;
    Evidence.Status = Fixture.Status;
    Evidence.CatalogRevision = TEXT("sha256:application_catalog_v1");
    Evidence.RequestRevision = TEXT("sha256:transition_v1");
    Evidence.ServiceRevision = TEXT("sha256:service_v1");
    return Evidence;
}

inline FSharApplicationLifecycleEvidence MakeApplicationLifecycleEvidence(
    const FSharApplicationModeRequest& Request,
    const ESharApplicationLifecyclePhase Phase
)
{
    FSharApplicationLifecycleEvidence Evidence;
    Evidence.RequestId = Request.RequestId;
    Evidence.Phase = Phase;
    Evidence.CatalogRevision = Request.CatalogRevision;
    Evidence.RequestRevision = Request.RequestRevision;
    if (Phase == ESharApplicationLifecyclePhase::SourceExit)
    {
        Evidence.ModeId = Request.SourceModeId;
        Evidence.ModeRevision = Request.SourceModeRevision;
        Evidence.PlanId = FName(TEXT("mode_exit_plan_v1"));
    }
    else
    {
        Evidence.ModeId = Request.TargetModeId;
        Evidence.ModeRevision = Request.TargetModeRevision;
        Evidence.PlanId = FName(TEXT("mode_entry_plan_v1"));
    }
    return Evidence;
}

inline FSharApplicationBarrierEvidence MakeApplicationBarrierEvidence(
    const FName& RequestId
)
{
    FSharApplicationBarrierEvidence Evidence;
    Evidence.RequestId = RequestId;
    Evidence.BarrierId = FName(TEXT("mode_ready_barrier_v1"));
    Evidence.CatalogRevision = TEXT("sha256:application_catalog_v1");
    Evidence.RequestRevision = TEXT("sha256:transition_v1");
    Evidence.TargetModeRevision = TEXT("sha256:loading_gameplay_v1");
    return Evidence;
}

inline FSharApplicationTransitionResolution MakeApplicationResolution(
    const FName& RequestId,
    const ESharApplicationTransitionCommand Command
)
{
    FSharApplicationTransitionResolution Resolution;
    Resolution.RequestId = RequestId;
    Resolution.Command = Command;
    Resolution.CatalogRevision = TEXT("sha256:application_catalog_v1");
    Resolution.RequestRevision = TEXT("sha256:transition_v1");
    return Resolution;
}

inline void PrepareApplicationTransition(
    USharApplicationModeCoordinator& Coordinator,
    const FSharApplicationModeRequest& Request
)
{
    Coordinator.Begin(Request.RequestId);
    Coordinator.RecordServiceEvidence(MakeApplicationServiceEvidence({
        .RequestId = Request.RequestId,
        .ServiceId = FName(TEXT("catalog_service")),
        .Status = ESharApplicationServiceStatus::Ready,
    }));
    Coordinator.RecordServiceEvidence(MakeApplicationServiceEvidence({
        .RequestId = Request.RequestId,
        .ServiceId = FName(TEXT("world_service")),
        .Status = ESharApplicationServiceStatus::Ready,
    }));
    Coordinator.BeginReadinessVerification(Request.RequestId);
    Coordinator.AcceptBarrier(
        MakeApplicationBarrierEvidence(Request.RequestId)
    );
    Coordinator.RecordLifecycleEvidence(MakeApplicationLifecycleEvidence(
        Request,
        ESharApplicationLifecyclePhase::SourceExit
    ));
    Coordinator.RecordLifecycleEvidence(MakeApplicationLifecycleEvidence(
        Request,
        ESharApplicationLifecyclePhase::TargetEntry
    ));
}
