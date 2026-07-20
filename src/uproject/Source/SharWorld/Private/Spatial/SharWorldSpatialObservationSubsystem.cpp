// File: SharWorldSpatialObservationSubsystem.cpp
// Path: src/uproject/Source/SharWorld/Private/Spatial/SharWorldSpatialObservationSubsystem.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: revision-fenced spatial registration and occupancy transitions only; no downstream domain mutation.
// ADR: docs/adr/unreal/runtime/contextual-interaction-query-and-transaction.md
// LARGE-FILE owner=SharWorld; reason=cohesive per-world spatial observation lifecycle;
// split=extract diagnostics if rejected-observation retention is introduced;
// validation=validate.sh SharWorld plus Unreal automation; review=2027-01.

#include "Spatial/SharWorldSpatialObservationSubsystem.h"

#include "Algo/Find.h"
#include "Content/SharPrimaryContentDefinition.h"

static bool IsCanonicalObservationId(const FName& Candidate)
{
    return USharPrimaryContentDefinition::IsCanonicalIdentifier(Candidate);
}

static bool IsRevisionIdentity(const FString& Candidate)
{
    return Candidate.StartsWith(TEXT("sha256:"));
}

FSharSpatialRegistrationState*
USharWorldSpatialObservationSubsystem::FindRegistration(
    const FName& PlacementId
)
{
    return Algo::FindByPredicate(
        Registrations,
        [&PlacementId](const FSharSpatialRegistrationState& Registration)
        {
            return Registration.PlacementId == PlacementId;
        }
    );
}

const FSharSpatialRegistrationState*
USharWorldSpatialObservationSubsystem::FindRegistration(
    const FName& PlacementId
) const
{
    return Algo::FindByPredicate(
        Registrations,
        [&PlacementId](const FSharSpatialRegistrationState& Registration)
        {
            return Registration.PlacementId == PlacementId;
        }
    );
}

FSharSpatialOccupancyState*
USharWorldSpatialObservationSubsystem::FindOccupancy(
    const FSharSpatialObservation& Observation
)
{
    return Algo::FindByPredicate(
        Occupancies,
        [&Observation](const FSharSpatialOccupancyState& Occupancy)
        {
            return Occupancy.PlacementId == Observation.PlacementId
                && Occupancy.VolumeId == Observation.VolumeId
                && Occupancy.ParticipantId == Observation.ParticipantId;
        }
    );
}

const FSharSpatialOccupancyState*
USharWorldSpatialObservationSubsystem::FindOccupancy(
    const FName& PlacementId,
    const FName& VolumeId,
    const FName& ParticipantId
) const
{
    return Algo::FindByPredicate(
        Occupancies,
        [&PlacementId, &VolumeId, &ParticipantId](
            const FSharSpatialOccupancyState& Occupancy
        )
        {
            return Occupancy.PlacementId == PlacementId
                && Occupancy.VolumeId == VolumeId
                && Occupancy.ParticipantId == ParticipantId;
        }
    );
}

bool USharWorldSpatialObservationSubsystem::IsValidObservation(
    const FSharSpatialObservation& Observation
)
{
    return IsCanonicalObservationId(Observation.PlacementId)
        && IsCanonicalObservationId(Observation.VolumeId)
        && IsCanonicalObservationId(Observation.ParticipantId)
        && IsRevisionIdentity(Observation.WorldRevision)
        && IsRevisionIdentity(Observation.DefinitionRevision)
        && Observation.SequenceNumber > 0;
}

ESharSpatialObservationResult
USharWorldSpatialObservationSubsystem::ApplyTransition(
    const FSharSpatialObservation& Observation,
    FSharSpatialOccupancyState& Occupancy
)
{
    switch (Observation.Kind)
    {
    case ESharSpatialObservationKind::Enter:
        if (Occupancy.bOccupied)
        {
            return ESharSpatialObservationResult::InvalidTransition;
        }
        Occupancy.bOccupied = true;
        break;
    case ESharSpatialObservationKind::Stay:
        if (!Occupancy.bOccupied)
        {
            return ESharSpatialObservationResult::InvalidTransition;
        }
        break;
    case ESharSpatialObservationKind::Exit:
        if (!Occupancy.bOccupied)
        {
            return ESharSpatialObservationResult::InvalidTransition;
        }
        Occupancy.bOccupied = false;
        break;
    default:
        return ESharSpatialObservationResult::InvalidObservation;
    }
    Occupancy.LastSequenceNumber = Observation.SequenceNumber;
    Occupancy.DefinitionRevision = Observation.DefinitionRevision;
    return ESharSpatialObservationResult::Accepted;
}

bool USharWorldSpatialObservationSubsystem::ConfigureWorld(
    const FName& InWorldId,
    const FString& InWorldRevision
)
{
    if (!IsCanonicalObservationId(InWorldId)
        || !IsRevisionIdentity(InWorldRevision))
    {
        return false;
    }
    WorldId = InWorldId;
    WorldRevision = InWorldRevision;
    Registrations.Reset();
    Occupancies.Reset();
    return true;
}

bool USharWorldSpatialObservationSubsystem::RegisterPlacement(
    const FName& PlacementId,
    const FString& DefinitionRevision
)
{
    const bool bInvalid =
        WorldId.IsNone()
        || !IsCanonicalObservationId(PlacementId)
        || !IsRevisionIdentity(DefinitionRevision);
    if (bInvalid)
    {
        return false;
    }
    FSharSpatialRegistrationState* Existing = FindRegistration(PlacementId);
    if (Existing != nullptr && Existing->bActive)
    {
        return false;
    }
    if (Existing != nullptr)
    {
        Existing->DefinitionRevision = DefinitionRevision;
        Existing->bActive = true;
        return true;
    }
    FSharSpatialRegistrationState Registration;
    Registration.PlacementId = PlacementId;
    Registration.DefinitionRevision = DefinitionRevision;
    Registration.bActive = true;
    Registrations.Add(Registration);
    return true;
}

bool USharWorldSpatialObservationSubsystem::ReleasePlacement(
    const FName& PlacementId
)
{
    FSharSpatialRegistrationState* Registration = FindRegistration(PlacementId);
    if (Registration == nullptr || !Registration->bActive)
    {
        return false;
    }
    Registration->bActive = false;
    for (FSharSpatialOccupancyState& Occupancy : Occupancies)
    {
        if (Occupancy.PlacementId == PlacementId)
        {
            Occupancy.bOccupied = false;
        }
    }
    return true;
}

ESharSpatialObservationResult
USharWorldSpatialObservationSubsystem::Observe(
    const FSharSpatialObservation& Observation
)
{
    if (!IsValidObservation(Observation))
    {
        return ESharSpatialObservationResult::InvalidObservation;
    }
    if (Observation.WorldRevision != WorldRevision)
    {
        return ESharSpatialObservationResult::StaleWorld;
    }
    const FSharSpatialRegistrationState* Registration =
        FindRegistration(Observation.PlacementId);
    if (Registration == nullptr || !Registration->bActive)
    {
        return ESharSpatialObservationResult::UnknownPlacement;
    }
    if (Registration->DefinitionRevision != Observation.DefinitionRevision)
    {
        return ESharSpatialObservationResult::StaleDefinition;
    }

    FSharSpatialOccupancyState* Occupancy = FindOccupancy(Observation);
    if (Occupancy != nullptr
        && Observation.SequenceNumber <= Occupancy->LastSequenceNumber)
    {
        return ESharSpatialObservationResult::Duplicate;
    }
    if (Occupancy == nullptr)
    {
        FSharSpatialOccupancyState NewOccupancy;
        NewOccupancy.PlacementId = Observation.PlacementId;
        NewOccupancy.VolumeId = Observation.VolumeId;
        NewOccupancy.ParticipantId = Observation.ParticipantId;
        Occupancies.Add(NewOccupancy);
        Occupancy = &Occupancies.Last();
    }
    return ApplyTransition(Observation, *Occupancy);
}

bool USharWorldSpatialObservationSubsystem::IsOccupied(
    const FName& PlacementId,
    const FName& VolumeId,
    const FName& ParticipantId
) const
{
    const FSharSpatialOccupancyState* Occupancy = FindOccupancy(
        PlacementId,
        VolumeId,
        ParticipantId
    );
    return Occupancy != nullptr && Occupancy->bOccupied;
}

int32 USharWorldSpatialObservationSubsystem::GetActiveRegistrationCount() const
{
    int32 ActiveCount = 0;
    for (const FSharSpatialRegistrationState& Registration : Registrations)
    {
        ActiveCount += Registration.bActive ? 1 : 0;
    }
    return ActiveCount;
}

int32 USharWorldSpatialObservationSubsystem::GetActiveOccupancyCount() const
{
    int32 ActiveCount = 0;
    for (const FSharSpatialOccupancyState& Occupancy : Occupancies)
    {
        ActiveCount += Occupancy.bOccupied ? 1 : 0;
    }
    return ActiveCount;
}
