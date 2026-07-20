// File: SharCameraTests.cpp
// Path: src/uproject/Source/SharCamera/Private/Tests/SharCameraTests.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: transient camera definition and arbitration tests; no world or camera manager loading.
// ADR: docs/adr/unreal/architecture/aaa-native-content-and-gameplay-foundation.md
// LARGE-FILE owner=SharCamera; reason=three cohesive camera-contract scenarios;
// split=separate arbitration tests if camera request behavior expands;
// validation=validate.sh SharCamera plus Unreal automation; review=2027-01.

#if WITH_DEV_AUTOMATION_TESTS

#include "Camera/SharCameraArbitrator.h"
#include "Camera/SharCameraProfileDefinition.h"

#include "Engine/DataAsset.h"
#include "Misc/AutomationTest.h"

static FPrimaryAssetId MakeCameraProfileId(const TCHAR* Name)
{
    return {
        FPrimaryAssetType(TEXT("SharCameraProfile")),
        FName(Name),
    };
}

static void FillCameraProfileBase(USharCameraProfileDefinition& Profile)
{
    Profile.CanonicalId = FName(TEXT("vehicle_chase"));
    Profile.DisplayName = FText::FromString(TEXT("Vehicle chase"));
    Profile.SourcePackageIds = {FName(TEXT("camera_contract"))};
    Profile.RevisionToken = TEXT("sha256:camera_profile_v1");
    Profile.ValidationProfile = FName(TEXT("camera_profile_v1"));
    Profile.OwningFeature = FName(TEXT("base"));
    Profile.ModeKind = ESharCameraModeKind::Chase;
    Profile.TargetKind = ESharCameraTargetKind::Vehicle;
    Profile.PriorityClass = ESharCameraPriorityClass::Default;
    Profile.PresetId = FName(TEXT("vehicle_chase_preset_v1"));
    Profile.TransitionPolicyId = FName(TEXT("camera_blend_v1"));
    Profile.CollisionPolicyId = FName(TEXT("camera_collision_v1"));
    Profile.InputPolicyId = FName(TEXT("vehicle_camera_input_v1"));
    Profile.VerificationPolicyId = FName(TEXT("vehicle_framing_v1"));
    Profile.bAllowsReverseView = true;
}

static FSharCameraRequest MakeDefaultFollowRequest()
{
    FSharCameraRequest Request;
    Request.RequestId = FName(TEXT("default_follow"));
    Request.RequesterId = FName(TEXT("player_camera"));
    Request.CameraProfileId = MakeCameraProfileId(TEXT("character_follow"));
    Request.TargetId = FName(TEXT("player_character"));
    Request.PriorityClass = ESharCameraPriorityClass::Default;
    return Request;
}

static FSharCameraRequest MakeCinematicRequest()
{
    FSharCameraRequest Request;
    Request.RequestId = FName(TEXT("mission_cinematic"));
    Request.RequesterId = FName(TEXT("mission_01"));
    Request.CameraProfileId = MakeCameraProfileId(TEXT("mission_cinematic"));
    Request.TargetId = FName(TEXT("mission_camera_actor"));
    Request.PriorityClass = ESharCameraPriorityClass::Cinematic;
    return Request;
}

namespace
{
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharCameraProfileValidationTest,
    "SHAR.Camera.Profile.Validation",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharCameraPriorityArbitrationTest,
    "SHAR.Camera.Arbitration.PriorityAndRestoration",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharCameraTieBreakerTest,
    "SHAR.Camera.Arbitration.StableTieBreaker",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
} // namespace

bool FSharCameraProfileValidationTest::RunTest(const FString& Parameters)
{
    (void)Parameters;
    auto* Profile = NewObject<USharCameraProfileDefinition>();
    FillCameraProfileBase(*Profile);

    TArray<FText> Errors;
    Profile->GatherValidationErrors(Errors);
    TestTrue(TEXT("Valid camera profile passes"), Errors.IsEmpty());

    Profile->ModeKind = ESharCameraModeKind::Animated;
    Profile->bAllowsSkipInput = false;
    Errors.Reset();
    Profile->GatherValidationErrors(Errors);
    TestFalse(
        TEXT("Animated profile without skip policy is rejected"),
        Errors.IsEmpty()
    );
    return true;
}

bool FSharCameraPriorityArbitrationTest::RunTest(
    const FString& Parameters
)
{
    (void)Parameters;
    auto* Arbitrator = NewObject<USharCameraArbitrator>();
    const FSharCameraRequest FollowRequest = MakeDefaultFollowRequest();
    const FSharCameraRequest CinematicRequest = MakeCinematicRequest();

    TestTrue(
        TEXT("Default camera becomes active"),
        Arbitrator->SubmitRequest(FollowRequest)
            == ESharCameraRequestStatus::Active
    );
    TestTrue(
        TEXT("Cinematic camera supersedes default view"),
        Arbitrator->SubmitRequest(CinematicRequest)
            == ESharCameraRequestStatus::Active
    );
    TestTrue(
        TEXT("Default request remains queued"),
        Arbitrator->GetRequestStatus(FollowRequest.RequestId)
            == ESharCameraRequestStatus::Queued
    );
    TestTrue(
        TEXT("Cinematic cancellation succeeds"),
        Arbitrator->CancelRequest(CinematicRequest.RequestId)
    );
    TestTrue(
        TEXT("Default request is restored"),
        Arbitrator->GetActiveRequestId() == FollowRequest.RequestId
    );
    return true;
}

bool FSharCameraTieBreakerTest::RunTest(const FString& Parameters)
{
    (void)Parameters;
    auto* Arbitrator = NewObject<USharCameraArbitrator>();

    FSharCameraRequest ZetaRequest = MakeDefaultFollowRequest();
    ZetaRequest.RequestId = FName(TEXT("zeta_request"));
    ZetaRequest.RequesterId = FName(TEXT("zeta_requester"));
    Arbitrator->SubmitRequest(ZetaRequest);

    FSharCameraRequest AlphaRequest = MakeDefaultFollowRequest();
    AlphaRequest.RequestId = FName(TEXT("alpha_request"));
    AlphaRequest.RequesterId = FName(TEXT("alpha_requester"));
    Arbitrator->SubmitRequest(AlphaRequest);

    TestTrue(
        TEXT("Lexically stable requester wins equal priority"),
        Arbitrator->GetActiveRequestId() == AlphaRequest.RequestId
    );
    TestTrue(
        TEXT("Duplicate request is rejected"),
        Arbitrator->SubmitRequest(AlphaRequest)
            == ESharCameraRequestStatus::Rejected
    );
    return true;
}

#endif
