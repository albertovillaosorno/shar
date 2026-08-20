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
//   - Shar interaction test fixtures composition module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Shar interaction test fixtures composition module.
// - Description:
//   - Implements the declared composition module responsibility for project.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Shar interaction test fixtures composition module.

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
