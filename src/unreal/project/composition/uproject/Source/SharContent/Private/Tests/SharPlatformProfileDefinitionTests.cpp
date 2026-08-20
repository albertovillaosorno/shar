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
//   - Shar platform profile definition tests composition module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Shar platform profile definition tests composition module.
// - Description:
//   - Implements the declared composition module responsibility for project.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Shar platform profile definition tests composition module.

#if WITH_DEV_AUTOMATION_TESTS

#include "Platform/SharPlatformProfileDefinition.h"

#include "Misc/AutomationTest.h"

static void FillPlatformBase(USharPlatformProfileDefinition& Profile)
{
    Profile.CanonicalId = FName(TEXT("android_arm64_mobile"));
    Profile.DisplayName = FText::FromString(TEXT("Android ARM64"));
    Profile.SourcePackageIds = {FName(TEXT("platform_contract"))};
    Profile.RevisionToken = TEXT("sha256:platform_profile_v1");
    Profile.ValidationProfile = FName(TEXT("platform_profile_v1"));
    Profile.OwningFeature = FName(TEXT("base"));
}

namespace
{
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharPlatformProfileValidationTest,
    "SHAR.Content.PlatformProfiles.Validation",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
} // namespace

bool FSharPlatformProfileValidationTest::RunTest(const FString& Parameters)
{
    (void)Parameters;
    auto* Profile = NewObject<USharPlatformProfileDefinition>();
    FillPlatformBase(*Profile);
    Profile->TargetPlatform = ESharTargetPlatform::Android;
    Profile->CpuArchitecture = ESharCpuArchitecture::Arm64;
    Profile->RendererProfile = ESharRendererProfile::MobileForward;
    Profile->bSupportsDedicatedServer = false;
    Profile->bSupportsHardwareRayTracing = false;
    Profile->bSupportsNanite = false;
    Profile->bSupportsLumen = false;

    TArray<FText> Errors;
    Profile->GatherValidationErrors(Errors);
    TestTrue(TEXT("Valid Android ARM64 profile passes"), Errors.IsEmpty());

    Errors.Reset();
    Profile->bSupportsDedicatedServer = true;
    Profile->GatherValidationErrors(Errors);
    // jig-ignore-next-line: exact syntax is indivisible
    TestFalse(TEXT("Android dedicated-server claim is rejected"), Errors.IsEmpty());
    TestTrue(
        TEXT("Windows ARM64 is declared"),
        USharPlatformProfileDefinition::IsSupportedTarget(
            ESharTargetPlatform::Windows,
            ESharCpuArchitecture::Arm64
        )
    );
    TestTrue(
        TEXT("Linux x86-64 is declared"),
        USharPlatformProfileDefinition::IsSupportedTarget(
            ESharTargetPlatform::Linux,
            ESharCpuArchitecture::X8664
        )
    );
    return true;
}

#endif
