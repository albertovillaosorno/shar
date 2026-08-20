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
//   - Shar audio tests composition module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Shar audio tests composition module.
// - Description:
//   - Implements the declared composition module responsibility for project.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Shar audio tests composition module.

#if WITH_DEV_AUTOMATION_TESTS

#include "Audio/SharAudioLeaseRegistry.h"
#include "Audio/SharAudioProfileDefinition.h"

#include "Engine/DataAsset.h"
#include "Misc/AutomationTest.h"

static constexpr int32 ExpectedOwnerReleaseCount = 2;
static constexpr int32 ExpectedActiveRequestCount = 2;

static FPrimaryAssetId MakeAudioProfileId(const TCHAR* Name)
{
    return {
        FPrimaryAssetType(TEXT("SharAudioProfile")),
        FName(Name),
    };
}

static void FillAudioProfileBase(USharAudioProfileDefinition& Profile)
{
    Profile.CanonicalId = FName(TEXT("vehicle_engine_loop"));
    Profile.DisplayName = FText::FromString(TEXT("Vehicle engine loop"));
    Profile.SourcePackageIds = {FName(TEXT("audio_contract"))};
    Profile.RevisionToken = TEXT("sha256:audio_profile_v1");
    Profile.ValidationProfile = FName(TEXT("audio_profile_v1"));
    Profile.OwningFeature = FName(TEXT("base"));
    Profile.SourceAssetId = FName(TEXT("vehicle_engine_source"));
    Profile.Role = ESharAudioRole::Vehicle;
    Profile.PlaybackPolicy = ESharAudioPlaybackPolicy::LeasedContinuous;
    Profile.ParameterSchemaId = FName(TEXT("vehicle_engine_parameters_v1"));
    Profile.AttenuationPolicyId = FName(TEXT("vehicle_attenuation_v1"));
    Profile.ConcurrencyPolicyId = FName(TEXT("vehicle_concurrency_v1"));
    Profile.RoutingPolicyId = FName(TEXT("vehicle_routing_v1"));
    Profile.ResidencyPolicyId = FName(TEXT("vehicle_residency_v1"));
    Profile.bPositional = true;
}

static FSharAudioPlaybackRequest MakeLeasedRequest()
{
    FSharAudioPlaybackRequest Request;
    Request.RequestId = FName(TEXT("player_vehicle_engine"));
    Request.OwnerId = FName(TEXT("player_vehicle"));
    Request.ProfileId = MakeAudioProfileId(TEXT("vehicle_engine_loop"));
    Request.LeaseId = FName(TEXT("player_vehicle_engine_lease"));
    Request.PlaybackPolicy = ESharAudioPlaybackPolicy::LeasedContinuous;
    Request.bLooping = true;
    return Request;
}

namespace
{
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharAudioProfileValidationTest,
    "SHAR.Audio.Profile.Validation",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharAudioLeaseRequirementTest,
    "SHAR.Audio.Leases.RequirementAndCompletion",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharAudioOwnerTeardownTest,
    "SHAR.Audio.Leases.OwnerTeardown",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
} // namespace

bool FSharAudioProfileValidationTest::RunTest(const FString& Parameters)
{
    (void)Parameters;
    auto* Profile = NewObject<USharAudioProfileDefinition>();
    FillAudioProfileBase(*Profile);

    TArray<FText> Errors;
    Profile->GatherValidationErrors(Errors);
    TestTrue(TEXT("Valid audio profile passes"), Errors.IsEmpty());

    Profile->CompletionPolicy = ESharAudioCompletionPolicy::Ignored;
    Errors.Reset();
    Profile->GatherValidationErrors(Errors);
    TestFalse(
        TEXT("Leased source without completion policy is rejected"),
        Errors.IsEmpty()
    );
    return true;
}

bool FSharAudioLeaseRequirementTest::RunTest(const FString& Parameters)
{
    (void)Parameters;
    auto* Registry = NewObject<USharAudioLeaseRegistry>();
    FSharAudioPlaybackRequest Request = MakeLeasedRequest();
    Request.LeaseId = FName();
    TestTrue(
        TEXT("Looping source requires a lease"),
        Registry->BeginPlayback(Request)
            == ESharAudioPlaybackResult::LeaseRequired
    );

    Request.LeaseId = FName(TEXT("player_vehicle_engine_lease"));
    TestTrue(
        TEXT("Leased source is accepted"),
        Registry->BeginPlayback(Request)
            == ESharAudioPlaybackResult::Accepted
    );
    TestTrue(
        TEXT("Accepted source completes once"),
        Registry->CompletePlayback(Request.RequestId)
    );
    TestFalse(
        TEXT("Completed source cannot complete twice"),
        Registry->CompletePlayback(Request.RequestId)
    );
    return true;
}

bool FSharAudioOwnerTeardownTest::RunTest(const FString& Parameters)
{
    (void)Parameters;
    auto* Registry = NewObject<USharAudioLeaseRegistry>();
    const FSharAudioPlaybackRequest FirstRequest = MakeLeasedRequest();
    Registry->BeginPlayback(FirstRequest);

    FSharAudioPlaybackRequest SecondRequest = MakeLeasedRequest();
    SecondRequest.RequestId = FName(TEXT("player_vehicle_skid"));
    SecondRequest.LeaseId = FName(TEXT("player_vehicle_skid_lease"));
    Registry->BeginPlayback(SecondRequest);

    TestTrue(
        TEXT("Two owner-scoped requests are active"),
        Registry->GetActiveCount() == ExpectedActiveRequestCount
    );
    TestTrue(
        TEXT("Owner teardown cancels both requests"),
        Registry->ReleaseOwner(FirstRequest.OwnerId)
            == ExpectedOwnerReleaseCount
    );
    TestTrue(
        TEXT("No request remains active after teardown"),
        Registry->GetActiveCount() == 0
    );
    return true;
}

#endif
