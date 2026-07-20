// File: SharApplicationModeLifecycle.cpp
// Path: src/uproject/Source/SharApplication/Private/Application/SharApplicationModeLifecycle.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: service evidence, readiness verification, atomic mode commit, recovery, terminal result, and release only.
// Specification: docs/technical/unreal/application-lifecycle-and-mode-runtime.md
// LARGE-FILE owner=SharApplication; reason=cohesive application-transition lifecycle and recovery;
// split=extract diagnostics if transition evidence becomes persistent;
// validation=validate.sh SharApplication plus Unreal automation; review=2027-01.

#include "Application/SharApplicationModeCoordinator.h"

#include "Algo/AllOf.h"
#include "Algo/Find.h"
#include "Application/SharApplicationModeCatalogSubsystem.h"
#include "Application/SharApplicationModeDefinition.h"
#include "Content/SharPrimaryContentDefinition.h"

static bool IsCanonicalEvidenceIdentity(const FName& Candidate)
{
    return USharPrimaryContentDefinition::IsCanonicalIdentifier(Candidate);
}

bool USharApplicationModeCoordinator::MatchesEvidenceRevision(
    const FSharApplicationTransitionSnapshot& Snapshot,
    const FString& CatalogRevision,
    const FString& RequestRevision
)
{
    return Snapshot.Request.CatalogRevision == CatalogRevision
        && Snapshot.Request.RequestRevision == RequestRevision;
}

bool USharApplicationModeCoordinator::AreRequiredServicesReady(
    const FSharApplicationTransitionSnapshot& Snapshot,
    const USharApplicationModeDefinition& Target
)
{
    return Algo::AllOf(
        Target.RequiredServiceIds,
        [&Snapshot, &Target](const FName& ServiceId)
        {
            const FSharApplicationServiceEvidence* Evidence =
                Algo::FindByPredicate(
                    Snapshot.ServiceEvidence,
                    [&ServiceId](
                        const FSharApplicationServiceEvidence& Candidate
                    )
                    {
                        return Candidate.ServiceId == ServiceId;
                    }
                );
            if (Evidence == nullptr)
            {
                return false;
            }
            return Evidence->Status == ESharApplicationServiceStatus::Ready
                || (Evidence->Status
                        == ESharApplicationServiceStatus::Degraded
                    && Target.bAllowsDegradedEntry);
        }
    );
}

ESharApplicationOperationResult
USharApplicationModeCoordinator::ClassifyPreparingEvidence(
    const FSharApplicationTransitionSnapshot& Snapshot
)
{
    if (Snapshot.bReleased)
    {
        return ESharApplicationOperationResult::Released;
    }
    if (Snapshot.State == ESharApplicationTransitionState::Preparing)
    {
        return ESharApplicationOperationResult::Accepted;
    }
    return IsTerminalState(Snapshot.State)
        ? ESharApplicationOperationResult::AlreadyTerminal
        : ESharApplicationOperationResult::InvalidState;
}

ESharApplicationOperationResult
USharApplicationModeCoordinator::ClassifyServiceEvidence(
    const FSharApplicationTransitionSnapshot& Snapshot,
    const USharApplicationModeDefinition& Target,
    const FSharApplicationServiceEvidence& Evidence
)
{
    const bool bRequired = Target.RequiredServiceIds.ContainsByPredicate(
        [&Evidence](const FName& ServiceId)
        {
            return ServiceId == Evidence.ServiceId;
        }
    );
    if (!bRequired)
    {
        return ESharApplicationOperationResult::ServiceMissing;
    }
    const bool bDuplicate = Snapshot.ServiceEvidence.ContainsByPredicate(
        [&Evidence](const FSharApplicationServiceEvidence& Existing)
        {
            return Existing.ServiceId == Evidence.ServiceId;
        }
    );
    return bDuplicate
        ? ESharApplicationOperationResult::DuplicateEvidence
        : ESharApplicationOperationResult::Accepted;
}

ESharApplicationOperationResult
USharApplicationModeCoordinator::RecordServiceEvidence(
    const FSharApplicationServiceEvidence& Evidence
)
{
    FSharApplicationTransitionSnapshot* Snapshot =
        FindTransition(Evidence.RequestId);
    if (Snapshot == nullptr)
    {
        return ESharApplicationOperationResult::NotFound;
    }
    const ESharApplicationOperationResult StateResult =
        ClassifyPreparingEvidence(*Snapshot);
    if (StateResult != ESharApplicationOperationResult::Accepted)
    {
        return StateResult;
    }
    if (!IsCanonicalEvidenceIdentity(Evidence.ServiceId)
        || !IsRevisionToken(Evidence.ServiceRevision))
    {
        return ESharApplicationOperationResult::InvalidRequest;
    }
    if (!MatchesEvidenceRevision(
        *Snapshot,
        Evidence.CatalogRevision,
        Evidence.RequestRevision
    ))
    {
        return ESharApplicationOperationResult::StaleRevision;
    }
    const USharApplicationModeDefinition* Target =
        Catalog->FindMode(Snapshot->Request.TargetModeId);
    if (Target == nullptr)
    {
        return ESharApplicationOperationResult::ModeMissing;
    }
    const ESharApplicationOperationResult EvidenceResult =
        ClassifyServiceEvidence(*Snapshot, *Target, Evidence);
    if (EvidenceResult != ESharApplicationOperationResult::Accepted)
    {
        return EvidenceResult;
    }
    Snapshot->ServiceEvidence.Add(Evidence);
    return ESharApplicationOperationResult::Accepted;
}

ESharApplicationOperationResult
USharApplicationModeCoordinator::BeginReadinessVerification(
    const FName& RequestId
)
{
    FSharApplicationTransitionSnapshot* Snapshot = FindTransition(RequestId);
    if (Snapshot == nullptr)
    {
        return ESharApplicationOperationResult::NotFound;
    }
    if (Snapshot->State != ESharApplicationTransitionState::Preparing)
    {
        return IsTerminalState(Snapshot->State)
            ? ESharApplicationOperationResult::AlreadyTerminal
            : ESharApplicationOperationResult::InvalidState;
    }
    const USharApplicationModeDefinition* Target =
        Catalog->FindMode(Snapshot->Request.TargetModeId);
    if (Target == nullptr)
    {
        return ESharApplicationOperationResult::ModeMissing;
    }
    if (!AreRequiredServicesReady(*Snapshot, *Target))
    {
        return ESharApplicationOperationResult::DependencyBlocked;
    }
    Snapshot->State = ESharApplicationTransitionState::VerifyingReadiness;
    return ESharApplicationOperationResult::Accepted;
}

ESharApplicationOperationResult USharApplicationModeCoordinator::AcceptBarrier(
    const FSharApplicationBarrierEvidence& Evidence
)
{
    FSharApplicationTransitionSnapshot* Snapshot =
        FindTransition(Evidence.RequestId);
    if (Snapshot == nullptr)
    {
        return ESharApplicationOperationResult::NotFound;
    }
    if (Snapshot->State
        != ESharApplicationTransitionState::VerifyingReadiness)
    {
        return IsTerminalState(Snapshot->State)
            ? ESharApplicationOperationResult::AlreadyTerminal
            : ESharApplicationOperationResult::InvalidState;
    }
    if (!MatchesEvidenceRevision(
        *Snapshot,
        Evidence.CatalogRevision,
        Evidence.RequestRevision
    ) || Snapshot->Request.TargetModeRevision != Evidence.TargetModeRevision)
    {
        return ESharApplicationOperationResult::StaleRevision;
    }
    const USharApplicationModeDefinition* Target =
        Catalog->FindMode(Snapshot->Request.TargetModeId);
    if (Target == nullptr)
    {
        return ESharApplicationOperationResult::ModeMissing;
    }
    if (Evidence.BarrierId != Target->ReadinessBarrierId)
    {
        return ESharApplicationOperationResult::InvalidRequest;
    }
    if (Snapshot->bBarrierAccepted)
    {
        return ESharApplicationOperationResult::DuplicateEvidence;
    }
    Snapshot->bBarrierAccepted = true;
    Snapshot->State = ESharApplicationTransitionState::ReadyToCommit;
    return ESharApplicationOperationResult::Accepted;
}

ESharApplicationOperationResult USharApplicationModeCoordinator::Commit(
    const FName& RequestId
)
{
    FSharApplicationTransitionSnapshot* Snapshot = FindTransition(RequestId);
    if (Snapshot == nullptr)
    {
        return ESharApplicationOperationResult::NotFound;
    }
    if (Snapshot->State != ESharApplicationTransitionState::ReadyToCommit
        || !Snapshot->bBarrierAccepted)
    {
        return IsTerminalState(Snapshot->State)
            ? ESharApplicationOperationResult::AlreadyTerminal
            : ESharApplicationOperationResult::InvalidState;
    }
    const bool bStaleSource =
        Observation.ActiveModeId != Snapshot->Request.SourceModeId
        || Observation.ActiveModeRevision
            != Snapshot->Request.SourceModeRevision
        || Catalog->GetCatalogRevision()
            != Snapshot->Request.CatalogRevision;
    if (bStaleSource)
    {
        return ESharApplicationOperationResult::StaleRevision;
    }
    Observation.ActiveModeId = Snapshot->Request.TargetModeId;
    Observation.ActiveModeRevision = Snapshot->Request.TargetModeRevision;
    Observation.SessionRevision = Snapshot->Request.SessionRevision;
    Observation.ProfileRevision = Snapshot->Request.ProfileRevision;
    Observation.WorldRevision = Snapshot->Request.WorldRevision;
    Snapshot->bCommitted = true;
    Snapshot->State = ESharApplicationTransitionState::Committed;
    Snapshot->State = ESharApplicationTransitionState::VerifyingTarget;
    return ESharApplicationOperationResult::Accepted;
}

ESharApplicationOperationResult
USharApplicationModeCoordinator::PublishTerminal(
    FSharApplicationTransitionSnapshot& Snapshot,
    const ESharApplicationTransitionState State,
    const ESharApplicationTerminalResult Result
)
{
    if (Snapshot.bReleased)
    {
        return ESharApplicationOperationResult::Released;
    }
    if (IsTerminalState(Snapshot.State))
    {
        return ESharApplicationOperationResult::AlreadyTerminal;
    }
    Snapshot.State = State;
    Snapshot.TerminalResult = Result;
    if (Observation.ActiveTransitionId == Snapshot.Request.RequestId)
    {
        Observation.ActiveTransitionId = FName();
    }
    return ESharApplicationOperationResult::Accepted;
}

ESharApplicationOperationResult USharApplicationModeCoordinator::Complete(
    const FName& RequestId
)
{
    FSharApplicationTransitionSnapshot* Snapshot = FindTransition(RequestId);
    if (Snapshot == nullptr)
    {
        return ESharApplicationOperationResult::NotFound;
    }
    if (Snapshot->State
        != ESharApplicationTransitionState::VerifyingTarget)
    {
        return IsTerminalState(Snapshot->State)
            ? ESharApplicationOperationResult::AlreadyTerminal
            : ESharApplicationOperationResult::InvalidState;
    }
    return PublishTerminal(
        *Snapshot,
        ESharApplicationTransitionState::Success,
        ESharApplicationTerminalResult::Success
    );
}

ESharApplicationOperationResult
USharApplicationModeCoordinator::RecoverCommittedFailure(
    FSharApplicationTransitionSnapshot& Snapshot
)
{
    const USharApplicationModeDefinition* Target =
        Catalog->FindMode(Snapshot.Request.TargetModeId);
    if (Target == nullptr || Target->RecoveryModeId.IsNone())
    {
        const ESharApplicationOperationResult FailureResult =
            PublishTerminal(
                Snapshot,
                ESharApplicationTransitionState::Failed,
                ESharApplicationTerminalResult::Failed
            );
        return FailureResult == ESharApplicationOperationResult::Accepted
            ? ESharApplicationOperationResult::RecoveryMissing
            : FailureResult;
    }
    const USharApplicationModeDefinition* Recovery =
        Catalog->FindMode(Target->RecoveryModeId);
    if (Recovery == nullptr)
    {
        const ESharApplicationOperationResult FailureResult =
            PublishTerminal(
                Snapshot,
                ESharApplicationTransitionState::Failed,
                ESharApplicationTerminalResult::Failed
            );
        return FailureResult == ESharApplicationOperationResult::Accepted
            ? ESharApplicationOperationResult::RecoveryMissing
            : FailureResult;
    }
    Observation.ActiveModeId = Recovery->CanonicalId;
    Observation.ActiveModeRevision =
        Recovery->CanonicalId == Snapshot.Request.SourceModeId
        ? Snapshot.Request.SourceModeRevision
        : Snapshot.Request.TargetModeRevision;
    return PublishTerminal(
        Snapshot,
        ESharApplicationTransitionState::Recovered,
        ESharApplicationTerminalResult::Recovered
    );
}

ESharApplicationOperationResult USharApplicationModeCoordinator::Resolve(
    const FSharApplicationTransitionResolution& Resolution
)
{
    FSharApplicationTransitionSnapshot* Snapshot =
        FindTransition(Resolution.RequestId);
    if (Snapshot == nullptr)
    {
        return ESharApplicationOperationResult::NotFound;
    }
    if (!MatchesEvidenceRevision(
        *Snapshot,
        Resolution.CatalogRevision,
        Resolution.RequestRevision
    ))
    {
        return ESharApplicationOperationResult::StaleRevision;
    }
    if (Snapshot->bReleased)
    {
        return ESharApplicationOperationResult::Released;
    }
    if (IsTerminalState(Snapshot->State))
    {
        return ESharApplicationOperationResult::AlreadyTerminal;
    }
    if (Snapshot->bCommitted)
    {
        return RecoverCommittedFailure(*Snapshot);
    }
    switch (Resolution.Command)
    {
    case ESharApplicationTransitionCommand::Fail:
        return PublishTerminal(
            *Snapshot,
            ESharApplicationTransitionState::Failed,
            ESharApplicationTerminalResult::Failed
        );
    case ESharApplicationTransitionCommand::Cancel:
        return PublishTerminal(
            *Snapshot,
            ESharApplicationTransitionState::Cancelled,
            ESharApplicationTerminalResult::Cancelled
        );
    case ESharApplicationTransitionCommand::Supersede:
        return PublishTerminal(
            *Snapshot,
            ESharApplicationTransitionState::Superseded,
            ESharApplicationTerminalResult::Superseded
        );
    default:
        return ESharApplicationOperationResult::InvalidRequest;
    }
}

ESharApplicationOperationResult USharApplicationModeCoordinator::Release(
    const FName& RequestId
)
{
    FSharApplicationTransitionSnapshot* Snapshot = FindTransition(RequestId);
    if (Snapshot == nullptr)
    {
        return ESharApplicationOperationResult::NotFound;
    }
    if (Snapshot->bReleased)
    {
        return ESharApplicationOperationResult::Released;
    }
    if (!IsTerminalState(Snapshot->State))
    {
        return ESharApplicationOperationResult::InvalidState;
    }
    Snapshot->bReleased = true;
    Snapshot->State = ESharApplicationTransitionState::Released;
    return ESharApplicationOperationResult::Accepted;
}

ESharApplicationTransitionState USharApplicationModeCoordinator::GetState(
    const FName& RequestId
) const
{
    const FSharApplicationTransitionSnapshot* Snapshot =
        FindTransition(RequestId);
    return Snapshot == nullptr
        ? ESharApplicationTransitionState::Failed
        : Snapshot->State;
}

ESharApplicationTerminalResult
USharApplicationModeCoordinator::GetTerminalResult(
    const FName& RequestId
) const
{
    const FSharApplicationTransitionSnapshot* Snapshot =
        FindTransition(RequestId);
    return Snapshot == nullptr
        ? ESharApplicationTerminalResult::None
        : Snapshot->TerminalResult;
}

FSharApplicationModeObservation
USharApplicationModeCoordinator::GetObservation() const
{
    return Observation;
}

int32 USharApplicationModeCoordinator::GetUnreleasedTransitionCount() const
{
    int32 Count = 0;
    for (const FSharApplicationTransitionSnapshot& Snapshot : Transitions)
    {
        Count += Snapshot.bReleased ? 0 : 1;
    }
    return Count;
}
