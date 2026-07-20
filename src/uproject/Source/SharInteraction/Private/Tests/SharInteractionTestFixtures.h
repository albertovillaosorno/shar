// File: SharInteractionTestFixtures.h
// Path: src/uproject/Source/SharInteraction/Private/Tests/SharInteractionTestFixtures.h
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: explicit transient interaction test fixtures only.
// ADR: docs/adr/unreal/runtime/contextual-interaction-query-and-transaction.md
// LARGE-FILE owner=SharInteraction; reason=cohesive typed interaction test fixtures;
// split=extract source fixtures if additional source families appear;
// validation=validate.sh SharInteraction plus Unreal automation; review=2027-01.

#pragma once

#include "Interaction/SharInteractionSubsystem.h"

#include "Engine/DataAsset.h"

struct FSharInteractionSourceFixture
{
    FName SourceId;
    FPrimaryAssetId InteractionId;
    bool bExclusive = false;
};

struct FSharInteractionCandidateFixture
{
    FName SourceId;
    FPrimaryAssetId InteractionId;
    int32 Priority = 0;
    double DistanceSquared = 0.0;
};

struct FSharInteractionQueryFixture
{
    FName QueryId;
    FName InteractorId;
};

inline FPrimaryAssetId MakeInteractionId(const TCHAR* Name)
{
    return {
        FPrimaryAssetType(TEXT("SharInteraction")),
        FName(Name),
    };
}

inline FSharInteractionSourceState MakeInteractionSource(
    const FSharInteractionSourceFixture& Fixture
)
{
    FSharInteractionSourceState Source;
    Source.SourceId = Fixture.SourceId;
    Source.InteractionId = Fixture.InteractionId;
    Source.SourceRevision = TEXT("sha256:source_v1");
    Source.bExclusive = Fixture.bExclusive;
    return Source;
}

inline FSharInteractionCandidate MakeInteractionCandidate(
    const FSharInteractionCandidateFixture& Fixture
)
{
    FSharInteractionCandidate Candidate;
    Candidate.SourceId = Fixture.SourceId;
    Candidate.InteractionId = Fixture.InteractionId;
    Candidate.SourceRevision = TEXT("sha256:source_v1");
    Candidate.Priority = Fixture.Priority;
    Candidate.DistanceSquared = Fixture.DistanceSquared;
    Candidate.bEligible = true;
    Candidate.EligibilityReasonId = FName(TEXT("eligible"));
    return Candidate;
}

inline FSharInteractionQuery MakeInteractionQuery(
    const FSharInteractionQueryFixture& Fixture
)
{
    FSharInteractionQuery Query;
    Query.QueryId = Fixture.QueryId;
    Query.InteractorId = Fixture.InteractorId;
    Query.WorldRevision = TEXT("sha256:world_v1");
    Query.InteractorRevision = TEXT("sha256:interactor_v1");
    return Query;
}

inline USharInteractionSubsystem* MakeConfiguredInteractionSubsystem()
{
    auto* Subsystem = NewObject<USharInteractionSubsystem>();
    Subsystem->ConfigureWorld(
        FName(TEXT("springfield_world")),
        TEXT("sha256:world_v1")
    );
    return Subsystem;
}
