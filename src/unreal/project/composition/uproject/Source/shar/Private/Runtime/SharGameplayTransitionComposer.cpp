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
//   - Gameplay transition readiness and commit composition implementation.
// - Must-Not:
//   - Replace application, loading, world, or local-player input authority.
// - Allows:
//   - Reading correlated snapshots, publishing evidence, and ordering commit.
// - Split-When:
//   - Recovery or lease replacement requires an independent transaction type.
// - Merge-When:
//   - Another composition adapter owns the identical cross-authority sequence.
// - Summary:
//   - Implements correlated gameplay readiness and post-commit input
//   - activation.
// - Description:
//   - Requires one transition revision across loading, world, and input before
//   - recording readiness and rolls back partial input on post-commit failure.
// - Usage:
//   - Called after loading has reached its verified commit barrier.
// - Defaults:
//   - No collaborator is mutated until all correlation preflight checks pass.
//

//! Correlated gameplay transition composition implementation.

#include "Runtime/SharGameplayTransitionComposer.h"

#include "Application/SharApplicationModeCatalogSubsystem.h"
#include "Application/SharApplicationModeCoordinator.h"
#include "Application/SharApplicationModeDefinition.h"
#include "Content/SharPrimaryContentDefinition.h"
#include "Input/SharLocalPlayerInputSubsystem.h"
#include "Loading/SharLoadCoordinatorSubsystem.h"
#include "Loading/SharWorldReadinessSubsystem.h"

namespace
{
struct FCorrelatedGameplayState
{
    FSharApplicationTransitionSnapshot Application;
    FSharLoadRequestSnapshot Load;
    FSharWorldReadinessSnapshot World;
    TArray<FSharInputContextLeaseObservation> InputLeases;
};

bool IsCanonicalCompositionId(const FName& Candidate)
{
    return USharPrimaryContentDefinition::IsCanonicalIdentifier(Candidate);
}

bool IsValidCompositionRequest(
    const FSharGameplayTransitionCompositionRequest& Request
)
{
    if (!IsCanonicalCompositionId(Request.ApplicationRequestId)
        || !IsCanonicalCompositionId(Request.LoadRequestId)
        || !IsCanonicalCompositionId(Request.WorldBarrierId)
        || !IsCanonicalCompositionId(Request.LoadingServiceId)
        || !IsCanonicalCompositionId(Request.WorldServiceId)
        || !IsCanonicalCompositionId(Request.InputServiceId)
        || Request.InputLeases.IsEmpty())
    {
        return false;
    }
    for (const FSharGameplayInputLeaseBinding& Binding : Request.InputLeases)
    {
        if (Binding.InputSubsystem == nullptr
            || !IsCanonicalCompositionId(Binding.LeaseId))
        {
            return false;
        }
    }
    return true;
}

ESharGameplayTransitionCompositionResult ReadCorrelatedState(
    const FSharGameplayTransitionCompositionRequest& Request,
    USharApplicationModeCatalogSubsystem* ApplicationCatalog,
    USharApplicationModeCoordinator* ApplicationCoordinator,
    USharLoadCoordinatorSubsystem* LoadCoordinator,
    USharWorldReadinessSubsystem* WorldReadiness,
    const ESharApplicationTransitionState RequiredApplicationState,
    FCorrelatedGameplayState& OutState
)
{
    if (!IsValidCompositionRequest(Request))
    {
        return ESharGameplayTransitionCompositionResult::InvalidRequest;
    }
    if (ApplicationCatalog == nullptr || ApplicationCoordinator == nullptr
        || !ApplicationCatalog->IsActive())
    {
        return ESharGameplayTransitionCompositionResult::ApplicationUnavailable;
    }
    if (!ApplicationCoordinator->GetTransitionSnapshot(
            Request.ApplicationRequestId,
            OutState.Application
        ) || OutState.Application.State != RequiredApplicationState)
    {
        return ESharGameplayTransitionCompositionResult::ApplicationUnavailable;
    }
    const USharApplicationModeDefinition* Target =
        ApplicationCatalog->FindMode(OutState.Application.Request.TargetModeId);
    if (Target == nullptr
        || !Target->RequiredServiceIds.Contains(Request.LoadingServiceId)
        || !Target->RequiredServiceIds.Contains(Request.WorldServiceId)
        || !Target->RequiredServiceIds.Contains(Request.InputServiceId))
    {
        return ESharGameplayTransitionCompositionResult::ApplicationRejected;
    }
    if (LoadCoordinator == nullptr
        || !LoadCoordinator->GetRequestSnapshot(
            Request.LoadRequestId,
            OutState.Load
        )
        || OutState.Load.State != ESharLoadRequestState::ReadyToCommit)
    {
        return ESharGameplayTransitionCompositionResult::LoadingUnavailable;
    }
    if (WorldReadiness == nullptr
        || !WorldReadiness->GetBarrierSnapshot(
            Request.WorldBarrierId,
            OutState.World
        )
        || !OutState.World.bReady)
    {
        return ESharGameplayTransitionCompositionResult::WorldUnavailable;
    }

    const FSharApplicationModeRequest& ApplicationRequest =
        OutState.Application.Request;
    const FSharLoadRequest& LoadRequest = OutState.Load.Request;
    const bool bStale =
        ApplicationCatalog->GetCatalogRevision()
            != ApplicationRequest.CatalogRevision
        || LoadRequest.CatalogRevision != ApplicationRequest.CatalogRevision
        || LoadRequest.RequestRevision != ApplicationRequest.RequestRevision
        || LoadRequest.ScopeId != ApplicationRequest.WorldId
        || LoadRequest.ScopeRevision != ApplicationRequest.WorldRevision
        || LoadRequest.ReadinessBarrierId != Request.WorldBarrierId
        || OutState.World.WorldId != ApplicationRequest.WorldId
        || OutState.World.WorldRevision != ApplicationRequest.WorldRevision
        || OutState.World.TransitionRevision
            != ApplicationRequest.RequestRevision;
    if (bStale)
    {
        return ESharGameplayTransitionCompositionResult::StaleRevision;
    }

    OutState.InputLeases.Reset(Request.InputLeases.Num());
    for (const FSharGameplayInputLeaseBinding& Binding : Request.InputLeases)
    {
        FSharInputContextLeaseObservation Observation;
        if (!Binding.InputSubsystem->GetLeaseObservation(
                Binding.LeaseId,
                Observation
            ))
        {
            return ESharGameplayTransitionCompositionResult::InputUnavailable;
        }
        if (Observation.OwnerModeId != ApplicationRequest.TargetModeId
            || Observation.OwnerModeRevision
                != ApplicationRequest.TargetModeRevision
            || Observation.TransitionRevision
                != ApplicationRequest.RequestRevision)
        {
            return ESharGameplayTransitionCompositionResult::StaleRevision;
        }
        if (Binding.InputSubsystem->CheckCommitReadiness(
                Binding.LeaseId,
                Observation.LeaseRevision,
                Observation.TransitionRevision
            ) != ESharInputContextLeaseResult::Accepted)
        {
            return ESharGameplayTransitionCompositionResult::InputUnavailable;
        }
        OutState.InputLeases.Add(Observation);
    }
    return ESharGameplayTransitionCompositionResult::Accepted;
}

FSharApplicationServiceEvidence MakeServiceEvidence(
    const FSharApplicationModeRequest& Request,
    const FName& ServiceId,
    const FString& ServiceRevision
)
{
    FSharApplicationServiceEvidence Evidence;
    Evidence.RequestId = Request.RequestId;
    Evidence.ServiceId = ServiceId;
    Evidence.Status = ESharApplicationServiceStatus::Ready;
    Evidence.CatalogRevision = Request.CatalogRevision;
    Evidence.RequestRevision = Request.RequestRevision;
    Evidence.ServiceRevision = ServiceRevision;
    return Evidence;
}

void ReleaseCommittedInput(
    const FSharGameplayTransitionCompositionRequest& Request
)
{
    for (const FSharGameplayInputLeaseBinding& Binding : Request.InputLeases)
    {
        if (Binding.InputSubsystem == nullptr)
        {
            continue;
        }
        FSharInputContextLeaseObservation Observation;
        if (Binding.InputSubsystem->GetLeaseObservation(
                Binding.LeaseId,
                Observation
            ) && Observation.State == ESharInputContextLeaseState::Active)
        {
            Binding.InputSubsystem->ReleaseLease(
                Binding.LeaseId,
                Observation.LeaseRevision
            );
        }
    }
}

bool RecoverApplication(
    USharApplicationModeCoordinator& Coordinator,
    const FSharApplicationModeRequest& Request
)
{
    FSharApplicationTransitionResolution Resolution;
    Resolution.RequestId = Request.RequestId;
    Resolution.Command = ESharApplicationTransitionCommand::Fail;
    Resolution.CatalogRevision = Request.CatalogRevision;
    Resolution.RequestRevision = Request.RequestRevision;
    return Coordinator.Resolve(Resolution)
        == ESharApplicationOperationResult::Accepted;
}

void CancelLoad(
    USharLoadCoordinatorSubsystem& Coordinator,
    const FName& RequestId
)
{
    if (Coordinator.GetTerminalResult(RequestId)
        != ESharLoadTerminalResult::None)
    {
        return;
    }
    FSharLoadTerminalRequest Terminal;
    Terminal.RequestId = RequestId;
    Terminal.Command = ESharLoadTerminalCommand::Cancel;
    Coordinator.ResolveTerminal(Terminal);
}
} // namespace

ESharGameplayTransitionCompositionResult
USharGameplayTransitionComposer::PrepareGameplayReadiness(
    const FSharGameplayTransitionCompositionRequest& Request,
    USharApplicationModeCatalogSubsystem* ApplicationCatalog,
    USharApplicationModeCoordinator* ApplicationCoordinator,
    USharLoadCoordinatorSubsystem* LoadCoordinator,
    USharWorldReadinessSubsystem* WorldReadiness
) const
{
    FCorrelatedGameplayState State;
    const ESharGameplayTransitionCompositionResult Preflight =
        ReadCorrelatedState(
            Request,
            ApplicationCatalog,
            ApplicationCoordinator,
            LoadCoordinator,
            WorldReadiness,
            ESharApplicationTransitionState::Preparing,
            State
        );
    if (Preflight != ESharGameplayTransitionCompositionResult::Accepted)
    {
        return Preflight;
    }

    const FSharApplicationModeRequest& ApplicationRequest =
        State.Application.Request;
    const TArray<FSharApplicationServiceEvidence> Evidence = {
        MakeServiceEvidence(
            ApplicationRequest,
            Request.LoadingServiceId,
            State.Load.Request.RequestRevision
        ),
        MakeServiceEvidence(
            ApplicationRequest,
            Request.WorldServiceId,
            State.World.WorldRevision
        ),
        MakeServiceEvidence(
            ApplicationRequest,
            Request.InputServiceId,
            ApplicationRequest.RequestRevision
        ),
    };
    for (const FSharApplicationServiceEvidence& Item : Evidence)
    {
        if (ApplicationCoordinator->RecordServiceEvidence(Item)
            != ESharApplicationOperationResult::Accepted)
        {
            return
                ESharGameplayTransitionCompositionResult::ApplicationRejected;
        }
    }
    return ESharGameplayTransitionCompositionResult::Accepted;
}

ESharGameplayTransitionCompositionResult
USharGameplayTransitionComposer::CommitGameplay(
    const FSharGameplayTransitionCompositionRequest& Request,
    const FSharApplicationLifecycleEvidence& SourceExitEvidence,
    const FSharApplicationLifecycleEvidence& TargetEntryEvidence,
    USharApplicationModeCatalogSubsystem* ApplicationCatalog,
    USharApplicationModeCoordinator* ApplicationCoordinator,
    USharLoadCoordinatorSubsystem* LoadCoordinator,
    USharWorldReadinessSubsystem* WorldReadiness
) const
{
    FCorrelatedGameplayState State;
    const ESharGameplayTransitionCompositionResult Preflight =
        ReadCorrelatedState(
            Request,
            ApplicationCatalog,
            ApplicationCoordinator,
            LoadCoordinator,
            WorldReadiness,
            ESharApplicationTransitionState::ReadyToCommit,
            State
        );
    if (Preflight != ESharGameplayTransitionCompositionResult::Accepted)
    {
        return Preflight;
    }

    if (ApplicationCoordinator->RecordLifecycleEvidence(SourceExitEvidence)
        != ESharApplicationOperationResult::Accepted)
    {
        return ESharGameplayTransitionCompositionResult::ApplicationRejected;
    }
    if (ApplicationCoordinator->RecordLifecycleEvidence(TargetEntryEvidence)
        != ESharApplicationOperationResult::Accepted)
    {
        CancelLoad(*LoadCoordinator, Request.LoadRequestId);
        return RecoverApplication(
            *ApplicationCoordinator,
            State.Application.Request
        ) ? ESharGameplayTransitionCompositionResult::ApplicationRejected
          : ESharGameplayTransitionCompositionResult::RecoveryFailed;
    }
    if (ApplicationCoordinator->Commit(Request.ApplicationRequestId)
        != ESharApplicationOperationResult::Accepted)
    {
        CancelLoad(*LoadCoordinator, Request.LoadRequestId);
        return RecoverApplication(
            *ApplicationCoordinator,
            State.Application.Request
        ) ? ESharGameplayTransitionCompositionResult::ApplicationRejected
          : ESharGameplayTransitionCompositionResult::RecoveryFailed;
    }

    for (int32 Index = 0; Index < Request.InputLeases.Num(); ++Index)
    {
        const FSharGameplayInputLeaseBinding& Binding =
            Request.InputLeases[Index];
        const FSharInputContextLeaseObservation& Observation =
            State.InputLeases[Index];
        if (Binding.InputSubsystem->CommitLease(
                Binding.LeaseId,
                Observation.LeaseRevision,
                Observation.TransitionRevision
            ) != ESharInputContextLeaseResult::Accepted)
        {
            ReleaseCommittedInput(Request);
            CancelLoad(*LoadCoordinator, Request.LoadRequestId);
            return RecoverApplication(
                *ApplicationCoordinator,
                State.Application.Request
            ) ? ESharGameplayTransitionCompositionResult::InputCommitFailed
              : ESharGameplayTransitionCompositionResult::RecoveryFailed;
        }
    }

    if (LoadCoordinator->CommitSuccess(Request.LoadRequestId)
        != ESharLoadOperationResult::Accepted)
    {
        ReleaseCommittedInput(Request);
        CancelLoad(*LoadCoordinator, Request.LoadRequestId);
        return RecoverApplication(
            *ApplicationCoordinator,
            State.Application.Request
        ) ? ESharGameplayTransitionCompositionResult::LoadingCommitFailed
          : ESharGameplayTransitionCompositionResult::RecoveryFailed;
    }
    if (ApplicationCoordinator->Complete(Request.ApplicationRequestId)
        != ESharApplicationOperationResult::Accepted)
    {
        ReleaseCommittedInput(Request);
        return RecoverApplication(
            *ApplicationCoordinator,
            State.Application.Request
        ) ? ESharGameplayTransitionCompositionResult::ApplicationRejected
          : ESharGameplayTransitionCompositionResult::RecoveryFailed;
    }
    return ESharGameplayTransitionCompositionResult::Accepted;
}
