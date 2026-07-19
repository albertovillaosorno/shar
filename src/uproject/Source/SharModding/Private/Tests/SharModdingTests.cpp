// File: SharModdingTests.cpp
// Path: src/uproject/Source/SharModding/Private/Tests/SharModdingTests.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: transient mod descriptor and activation-plan tests; no package mounting or code execution.
// ADR: docs/adr/unreal/architecture/aaa-native-content-and-gameplay-foundation.md

#if WITH_DEV_AUTOMATION_TESTS

#include "Modding/SharModActivationPlan.h"
#include "Modding/SharModDescriptor.h"

#include "Misc/AutomationTest.h"

static constexpr int32 ExpectedModCount = 2;

static USharModDescriptor* MakeMod(const TCHAR* Namespace)
{
    auto* Descriptor = NewObject<USharModDescriptor>();
    Descriptor->CanonicalId = FName(Namespace);
    Descriptor->NamespaceId = FName(Namespace);
    Descriptor->DisplayName = FText::FromString(TEXT("Synthetic mod"));
    Descriptor->SourcePackageIds = {FName(TEXT("synthetic_mod_package"))};
    Descriptor->RevisionToken = TEXT("sha256:mod_revision_v1");
    Descriptor->ValidationProfile = FName(TEXT("mod_descriptor_v1"));
    Descriptor->OwningFeature = FName(TEXT("base"));
    Descriptor->Version = TEXT("1.0.0");
    Descriptor->PackageSetDigest = TEXT("sha256:package_set_v1");
    return Descriptor;
}

namespace
{
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharModActivationPlanTest,
    "SHAR.Modding.ActivationPlan",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharNativeModTrustTest,
    "SHAR.Modding.NativeTrust",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
} // namespace

bool FSharModActivationPlanTest::RunTest(const FString& Parameters)
{
    (void)Parameters;
    auto* BaseMod = MakeMod(TEXT("base_extension"));
    auto* DependentMod = MakeMod(TEXT("dependent_extension"));
    DependentMod->RequiredModNamespaces.Add(FName(TEXT("base_extension")));

    const TArray<const USharModDescriptor*> Descriptors = {
        DependentMod,
        BaseMod,
    };
    const FSharModActivationPlan Plan =
        FSharModActivationPlanner::Build(Descriptors);
    TestTrue(TEXT("Dependency graph activates"), Plan.bCanActivate);
    TestTrue(
        TEXT("Activation order contains both mods"),
        Plan.OrderedDescriptors.Num() == ExpectedModCount
    );
    TestTrue(
        TEXT("Dependent mod remains last after its dependency"),
        Plan.OrderedDescriptors.Last()->NamespaceId
            == FName(TEXT("dependent_extension"))
    );

    BaseMod->RequiredModNamespaces.Add(FName(TEXT("dependent_extension")));
    const FSharModActivationPlan CyclePlan = FSharModActivationPlanner::Build(Descriptors);
    TestFalse(TEXT("Dependency cycle is rejected"), CyclePlan.bCanActivate);
    return true;
}

bool FSharNativeModTrustTest::RunTest(const FString& Parameters)
{
    (void)Parameters;
    auto* NativeMod = MakeMod(TEXT("native_extension"));
    NativeMod->TrustTier = ESharModTrustTier::Native;
    TArray<FText> Errors;
    NativeMod->GatherValidationErrors(Errors);
    TestFalse(TEXT("Native mod requires explicit approval"), Errors.IsEmpty());

    NativeMod->bExplicitUserApprovalRequired = true;
    Errors.Reset();
    NativeMod->GatherValidationErrors(Errors);
    TestTrue(TEXT("Explicit native trust descriptor passes"), Errors.IsEmpty());
    return true;
}

#endif
