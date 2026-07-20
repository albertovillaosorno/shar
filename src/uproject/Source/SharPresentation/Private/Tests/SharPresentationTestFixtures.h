// File: SharPresentationTestFixtures.h
// Path: src/uproject/Source/SharPresentation/Private/Tests/SharPresentationTestFixtures.h
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: explicit transient presentation test fixtures only.
// Specification: docs/technical/unreal/presentation-playback-runtime.md
// LARGE-FILE owner=SharPresentation; reason=cohesive typed presentation test fixtures;
// split=extract channel fixtures if additional arbitration policies appear;
// validation=validate.sh SharPresentation plus Unreal automation; review=2027-01.

#pragma once

#include "Presentation/SharPresentationPlaybackSubsystem.h"

#include "Engine/DataAsset.h"

struct FSharPresentationRequestFixture
{
    FName RequestId;
    FName PresentationId;
    FName OwnerId;
    FName ChannelId;
    int32 Priority = 0;
    bool bSkipAllowed = false;
};

inline FPrimaryAssetId MakePresentationAssetId(const FName& Name)
{
    return {
        FPrimaryAssetType(TEXT("SharPresentation")),
        Name,
    };
}

inline FSharPresentationChannelPolicy MakePresentationChannel(
    const ESharPresentationDuplicatePolicy DuplicatePolicy
)
{
    FSharPresentationChannelPolicy Policy;
    Policy.ChannelId = FName(TEXT("cinematic"));
    Policy.MaximumPending = FSharPresentationChannelPolicy::DefaultMaximumPending;
    Policy.MaximumActive = FSharPresentationChannelPolicy::DefaultMaximumActive;
    Policy.DuplicatePolicy = DuplicatePolicy;
    Policy.StarvationPolicyId = FName(TEXT("bounded_fifo_v1"));
    Policy.TeardownPolicyId = FName(TEXT("cancel_and_release_v1"));
    return Policy;
}

inline FSharPresentationRequest MakePresentationRequest(
    const FSharPresentationRequestFixture& Fixture
)
{
    FSharPresentationRequest Request;
    Request.RequestId = Fixture.RequestId;
    Request.PresentationId = MakePresentationAssetId(Fixture.PresentationId);
    Request.OwnerId = Fixture.OwnerId;
    Request.OwnerRevision = TEXT("sha256:owner_v1");
    Request.ParticipantId = FName(TEXT("player_01"));
    Request.TargetId = FName(TEXT("kwik_e_mart"));
    Request.WorldRevision = TEXT("sha256:world_v1");
    Request.RequestRevision = TEXT("sha256:request_v1");
    Request.ChannelId = Fixture.ChannelId;
    Request.Priority = Fixture.Priority;
    Request.bSkipAllowed = Fixture.bSkipAllowed;
    return Request;
}

inline FSharPresentationCallbackRevision MakePresentationRevision()
{
    FSharPresentationCallbackRevision Revision;
    Revision.OwnerRevision = TEXT("sha256:owner_v1");
    Revision.WorldRevision = TEXT("sha256:world_v1");
    Revision.RequestRevision = TEXT("sha256:request_v1");
    return Revision;
}

inline USharPresentationPlaybackSubsystem* MakePresentationSubsystem(
    const ESharPresentationDuplicatePolicy DuplicatePolicy
)
{
    auto* Subsystem = NewObject<USharPresentationPlaybackSubsystem>();
    Subsystem->ConfigureWorld(
        FName(TEXT("springfield_world")),
        TEXT("sha256:world_v1")
    );
    Subsystem->RegisterChannel(MakePresentationChannel(DuplicatePolicy));
    return Subsystem;
}
