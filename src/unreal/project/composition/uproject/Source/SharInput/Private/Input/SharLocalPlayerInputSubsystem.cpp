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
//   - Per-local-player SHAR mapping-context lease lifecycle.
// - Must-Not:
//   - Commit application modes or infer input readiness from asset existence.
// - Allows:
//   - Adapting validated SHAR leases to Enhanced Input mapping contexts.
// - Split-When:
//   - Device assignment or rebinding requires an independent adapter.
// - Merge-When:
//   - Another local-player subsystem owns identical lease semantics.
// - Summary:
//   - Implements staged and committed Enhanced Input context leases.
// - Description:
//   - Fails closed without live Enhanced Player Input and releases on teardown.
// - Usage:
//   - Driven by higher-level application and avatar composition.
// - Defaults:
//   - Staging has no native input side effects.
//

//! Per-local-player mapping-context lease implementation.

#include "Input/SharLocalPlayerInputSubsystem.h"

#include "Algo/Find.h"
#include "Content/SharPrimaryContentDefinition.h"
#include "EnhancedInputSubsystems.h"
#include "EnhancedPlayerInput.h"
#include "Engine/LocalPlayer.h"
#include "InputMappingContext.h"
#include "Subsystems/SubsystemCollection.h"

namespace
{
bool IsRevisionToken(const FString& Revision)
{
    return Revision.StartsWith(TEXT("sha256:"));
}

bool IsCanonicalInputId(const FName& Candidate)
{
    return USharPrimaryContentDefinition::IsCanonicalIdentifier(Candidate);
}
} // namespace

void USharLocalPlayerInputSubsystem::Initialize(
    FSubsystemCollectionBase& Collection
)
{
    Super::Initialize(Collection);
    Collection.InitializeDependency<UEnhancedInputLocalPlayerSubsystem>();
}

void USharLocalPlayerInputSubsystem::Deinitialize()
{
    ReleaseActiveLeasesForTeardown();
    Leases.Reset();
    Super::Deinitialize();
}

bool USharLocalPlayerInputSubsystem::IsValidRequest(
    const FSharInputContextLeaseRequest& Request
)
{
    return IsCanonicalInputId(Request.LeaseId)
        && IsCanonicalInputId(Request.ContextId)
        && IsCanonicalInputId(Request.OwnerModeId)
        && IsRevisionToken(Request.OwnerModeRevision)
        && IsRevisionToken(Request.TransitionRevision)
        && IsRevisionToken(Request.LeaseRevision)
        && Request.MappingContext != nullptr;
}

FSharInputContextLeaseRecord* USharLocalPlayerInputSubsystem::FindLease(
    const FName& LeaseId
)
{
    return Algo::FindByPredicate(
        Leases,
        [&LeaseId](const FSharInputContextLeaseRecord& Lease)
        {
            return Lease.Request.LeaseId == LeaseId;
        }
    );
}

const FSharInputContextLeaseRecord* USharLocalPlayerInputSubsystem::FindLease(
    const FName& LeaseId
) const
{
    return Algo::FindByPredicate(
        Leases,
        [&LeaseId](const FSharInputContextLeaseRecord& Lease)
        {
            return Lease.Request.LeaseId == LeaseId;
        }
    );
}

UEnhancedInputLocalPlayerSubsystem*
USharLocalPlayerInputSubsystem::GetEnhancedInput() const
{
    const ULocalPlayer* LocalPlayer = GetLocalPlayer();
    return LocalPlayer == nullptr
        ? nullptr
        : LocalPlayer->GetSubsystem<UEnhancedInputLocalPlayerSubsystem>();
}

bool USharLocalPlayerInputSubsystem::IsMappingOwned(
    const UInputMappingContext* Context
) const
{
    return Leases.ContainsByPredicate(
        [Context](const FSharInputContextLeaseRecord& Lease)
        {
            return Lease.State != ESharInputContextLeaseState::Released
                && Lease.Request.MappingContext == Context;
        }
    );
}

ESharInputContextLeaseResult USharLocalPlayerInputSubsystem::StageLease(
    const FSharInputContextLeaseRequest& Request
)
{
    if (!IsValidRequest(Request))
    {
        return ESharInputContextLeaseResult::InvalidRequest;
    }
    if (FindLease(Request.LeaseId) != nullptr)
    {
        return ESharInputContextLeaseResult::DuplicateLease;
    }
    if (IsMappingOwned(Request.MappingContext))
    {
        return ESharInputContextLeaseResult::MappingContextInUse;
    }

    FSharInputContextLeaseRecord Record;
    Record.Request = Request;
    Leases.Add(MoveTemp(Record));
    return ESharInputContextLeaseResult::Accepted;
}

ESharInputContextLeaseResult
USharLocalPlayerInputSubsystem::CheckCommitReadiness(
    const FName& LeaseId,
    const FString& LeaseRevision,
    const FString& TransitionRevision
) const
{
    const FSharInputContextLeaseRecord* Lease = FindLease(LeaseId);
    if (Lease == nullptr)
    {
        return ESharInputContextLeaseResult::NotFound;
    }
    if (Lease->Request.LeaseRevision != LeaseRevision
        || Lease->Request.TransitionRevision != TransitionRevision)
    {
        return ESharInputContextLeaseResult::StaleRevision;
    }
    if (Lease->State == ESharInputContextLeaseState::Released)
    {
        return ESharInputContextLeaseResult::AlreadyReleased;
    }
    if (Lease->State != ESharInputContextLeaseState::Staged)
    {
        return ESharInputContextLeaseResult::InvalidState;
    }
    UEnhancedInputLocalPlayerSubsystem* EnhancedInput = GetEnhancedInput();
    if (EnhancedInput == nullptr || EnhancedInput->GetPlayerInput() == nullptr)
    {
        return ESharInputContextLeaseResult::EnhancedInputUnavailable;
    }
    return EnhancedInput->HasMappingContext(Lease->Request.MappingContext)
        ? ESharInputContextLeaseResult::MappingContextInUse
        : ESharInputContextLeaseResult::Accepted;
}

ESharInputContextLeaseResult USharLocalPlayerInputSubsystem::CommitLease(
    const FName& LeaseId,
    const FString& LeaseRevision,
    const FString& TransitionRevision
)
{
    const ESharInputContextLeaseResult Readiness = CheckCommitReadiness(
        LeaseId,
        LeaseRevision,
        TransitionRevision
    );
    if (Readiness != ESharInputContextLeaseResult::Accepted)
    {
        return Readiness;
    }
    FSharInputContextLeaseRecord* Lease = FindLease(LeaseId);
    UEnhancedInputLocalPlayerSubsystem* EnhancedInput = GetEnhancedInput();
    if (Lease == nullptr || EnhancedInput == nullptr)
    {
        return ESharInputContextLeaseResult::EnhancedInputUnavailable;
    }

    EnhancedInput->AddMappingContext(
        Lease->Request.MappingContext,
        Lease->Request.Priority
    );
    int32 AppliedPriority = 0;
    if (!EnhancedInput->HasMappingContext(
            Lease->Request.MappingContext,
            AppliedPriority
        ) || AppliedPriority != Lease->Request.Priority)
    {
        EnhancedInput->RemoveMappingContext(Lease->Request.MappingContext);
        return ESharInputContextLeaseResult::MappingApplyFailed;
    }

    Lease->State = ESharInputContextLeaseState::Active;
    return ESharInputContextLeaseResult::Accepted;
}

ESharInputContextLeaseResult USharLocalPlayerInputSubsystem::ReleaseLease(
    const FName& LeaseId,
    const FString& LeaseRevision
)
{
    FSharInputContextLeaseRecord* Lease = FindLease(LeaseId);
    if (Lease == nullptr)
    {
        return ESharInputContextLeaseResult::NotFound;
    }
    if (Lease->Request.LeaseRevision != LeaseRevision)
    {
        return ESharInputContextLeaseResult::StaleRevision;
    }
    if (Lease->State == ESharInputContextLeaseState::Released)
    {
        return ESharInputContextLeaseResult::AlreadyReleased;
    }

    if (Lease->State == ESharInputContextLeaseState::Active)
    {
        if (UEnhancedInputLocalPlayerSubsystem* EnhancedInput =
                GetEnhancedInput())
        {
            if (EnhancedInput->GetPlayerInput() != nullptr)
            {
                EnhancedInput->RemoveMappingContext(
                    Lease->Request.MappingContext
                );
            }
        }
    }
    Lease->State = ESharInputContextLeaseState::Released;
    return ESharInputContextLeaseResult::Accepted;
}

bool USharLocalPlayerInputSubsystem::GetLeaseObservation(
    const FName& LeaseId,
    FSharInputContextLeaseObservation& OutObservation
) const
{
    const FSharInputContextLeaseRecord* Lease = FindLease(LeaseId);
    if (Lease == nullptr)
    {
        return false;
    }
    OutObservation.LeaseId = Lease->Request.LeaseId;
    OutObservation.ContextId = Lease->Request.ContextId;
    OutObservation.OwnerModeId = Lease->Request.OwnerModeId;
    OutObservation.OwnerModeRevision = Lease->Request.OwnerModeRevision;
    OutObservation.TransitionRevision = Lease->Request.TransitionRevision;
    OutObservation.LeaseRevision = Lease->Request.LeaseRevision;
    OutObservation.Priority = Lease->Request.Priority;
    OutObservation.State = Lease->State;
    return true;
}

int32 USharLocalPlayerInputSubsystem::GetActiveLeaseCount() const
{
    int32 Count = 0;
    for (const FSharInputContextLeaseRecord& Lease : Leases)
    {
        Count += Lease.State == ESharInputContextLeaseState::Active ? 1 : 0;
    }
    return Count;
}

int32 USharLocalPlayerInputSubsystem::GetStagedLeaseCount() const
{
    int32 Count = 0;
    for (const FSharInputContextLeaseRecord& Lease : Leases)
    {
        Count += Lease.State == ESharInputContextLeaseState::Staged ? 1 : 0;
    }
    return Count;
}

void USharLocalPlayerInputSubsystem::ReleaseActiveLeasesForTeardown()
{
    UEnhancedInputLocalPlayerSubsystem* EnhancedInput = GetEnhancedInput();
    for (FSharInputContextLeaseRecord& Lease : Leases)
    {
        if (Lease.State != ESharInputContextLeaseState::Active)
        {
            continue;
        }
        if (EnhancedInput != nullptr
            && EnhancedInput->GetPlayerInput() != nullptr)
        {
            EnhancedInput->RemoveMappingContext(Lease.Request.MappingContext);
        }
        Lease.State = ESharInputContextLeaseState::Released;
    }
}
