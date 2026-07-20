// File: SharNetworkingTests.cpp
// Path: src/uproject/Source/SharNetworking/Private/Tests/SharNetworkingTests.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: transient adapter and handshake tests; no sockets, travel, or server processes.
// ADR: docs/adr/modding/mod-owned-multiplayer-adapters-and-community-servers.md
// LARGE-FILE owner=SharNetworking; reason=four cohesive declaration and handshake scenarios;
// split=separate handshake mismatches if optional compatibility fields expand;
// validation=validate.sh SharNetworking plus Unreal automation; review=2027-01.

#if WITH_DEV_AUTOMATION_TESTS

#include "Networking/SharCompatibilityHandshake.h"
#include "Networking/SharMultiplayerAdapterDefinition.h"
#include "Platform/SharPlatformProfileDefinition.h"

#include "Misc/AutomationTest.h"

static void FillAdapterBase(USharMultiplayerAdapterDefinition& Definition)
{
    Definition.MultiplayerModeId = FName(TEXT("community_race"));
    Definition.ProtocolId = FName(TEXT("community_race_protocol"));
    Definition.ProtocolRevision = TEXT("1.0.0");
    Definition.RuntimeContractRevision = TEXT("sha256:runtime_v1");
    Definition.RequiredCatalogRevision = TEXT("sha256:catalog_v1");
    Definition.PackageSetDigest = TEXT("sha256:package_set_v1");
    Definition.AuthorityModel = ESharNetworkAuthorityModel::DedicatedServer;
    Definition.ClientRoleIds = {
        FName(TEXT("player")),
        FName(TEXT("spectator")),
    };

    FSharNetworkServerTarget Target;
    Target.Platform = ESharTargetPlatform::Windows;
    Target.Architecture = ESharCpuArchitecture::X8664;
    Definition.ServerTargets.Add(Target);

    FSharNetworkRequiredPackage Package;
    Package.NamespaceId = FName(TEXT("example_race"));
    Package.Version = TEXT("1.0.0");
    Package.PackageDigest = TEXT("sha256:example_race_v1");
    Definition.RequiredPackages.Add(Package);
    Definition.RequiredCapabilityIds.Add(
        FName(TEXT("vehicle_replication"))
    );
    Definition.SavePolicy = ESharNetworkSavePolicy::EphemeralSession;
    Definition.AchievementPolicy =
        ESharNetworkAchievementPolicy::BaseIncompatible;
    Definition.DiscoveryPolicy = ESharNetworkDiscoveryPolicy::DirectAddress;
    Definition.TeardownPolicyId = FName(TEXT("network_adapter_teardown_v1"));
}

static int32 CountAdapterValidationErrors(
    const ESharNetworkNativeCodePolicy NativeCodePolicy,
    const bool bExplicitApprovalRequired
)
{
    auto* Definition = NewObject<USharMultiplayerAdapterDefinition>();
    FillAdapterBase(*Definition);
    Definition->NativeCodePolicy = NativeCodePolicy;
    Definition->bExplicitUserApprovalRequired = bExplicitApprovalRequired;

    TArray<FText> Errors;
    Definition->GatherValidationErrors(Errors);
    return Errors.Num();
}

static FSharCompatibilitySnapshot MakeClientSnapshot()
{
    FSharCompatibilitySnapshot Snapshot;
    Snapshot.SessionId = FName(TEXT("community_race_session"));
    Snapshot.SessionRoleId = FName(TEXT("player"));
    Snapshot.ProtocolId = FName(TEXT("community_race_protocol"));
    Snapshot.ProtocolRevision = TEXT("1.0.0");
    Snapshot.RuntimeContractRevision = TEXT("sha256:runtime_v1");
    Snapshot.CatalogRevision = TEXT("sha256:catalog_v1");
    Snapshot.PackageSetDigest = TEXT("sha256:package_set_v1");
    Snapshot.Platform = ESharTargetPlatform::Windows;
    Snapshot.Architecture = ESharCpuArchitecture::X8664;
    Snapshot.AuthorityModel = ESharNetworkAuthorityModel::DedicatedServer;
    Snapshot.SavePolicy = ESharNetworkSavePolicy::EphemeralSession;
    Snapshot.AchievementPolicy =
        ESharNetworkAchievementPolicy::BaseIncompatible;
    Snapshot.CapabilityIds = {
        FName(TEXT("camera_observation")),
        FName(TEXT("vehicle_replication")),
    };
    return Snapshot;
}

static FSharCompatibilitySnapshot MakeServerSnapshot()
{
    FSharCompatibilitySnapshot Snapshot = MakeClientSnapshot();
    Snapshot.SessionRoleId = FName(TEXT("server"));
    Snapshot.CapabilityIds = {FName(TEXT("vehicle_replication"))};
    return Snapshot;
}

namespace
{
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharNetworkingAdapterValidationTest,
    "SHAR.Networking.Adapter.Validation",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharNetworkingCompatibleHandshakeTest,
    "SHAR.Networking.Handshake.Compatible",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharNetworkingPackageMismatchTest,
    "SHAR.Networking.Handshake.PackageMismatch",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharNetworkingCapabilityMismatchTest,
    "SHAR.Networking.Handshake.CapabilityMismatch",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
} // namespace

bool FSharNetworkingAdapterValidationTest::RunTest(
    const FString& Parameters
)
{
    (void)Parameters;
    const int32 ContentOnlyErrorCount = CountAdapterValidationErrors(
        ESharNetworkNativeCodePolicy::ContentOnly,
        false
    );
    const int32 UnapprovedNativeErrorCount = CountAdapterValidationErrors(
        ESharNetworkNativeCodePolicy::ExplicitlyTrustedNative,
        false
    );
    TestTrue(
        TEXT("Valid content-only adapter passes"),
        ContentOnlyErrorCount == 0
    );
    TestTrue(
        TEXT("Unapproved native adapter is rejected"),
        UnapprovedNativeErrorCount > 0
    );
    return true;
}

bool FSharNetworkingCompatibleHandshakeTest::RunTest(
    const FString& Parameters
)
{
    (void)Parameters;
    const FSharCompatibilitySnapshot Client = MakeClientSnapshot();
    const FSharCompatibilitySnapshot Server = MakeServerSnapshot();
    FName MismatchField;

    TestTrue(
        TEXT("Compatible snapshots are admitted"),
        USharCompatibilityHandshake::Evaluate(
            Client,
            Server,
            MismatchField
        )
            == ESharCompatibilityResult::Compatible
    );
    TestTrue(
        TEXT("Compatible result has no mismatch field"),
        MismatchField.IsNone()
    );
    return true;
}

bool FSharNetworkingPackageMismatchTest::RunTest(
    const FString& Parameters
)
{
    (void)Parameters;
    const FSharCompatibilitySnapshot Client = MakeClientSnapshot();
    FSharCompatibilitySnapshot Server = MakeServerSnapshot();
    Server.PackageSetDigest = TEXT("sha256:different_package_set");
    FName MismatchField;

    TestTrue(
        TEXT("Package-set mismatch is rejected"),
        USharCompatibilityHandshake::Evaluate(
            Client,
            Server,
            MismatchField
        )
            == ESharCompatibilityResult::PackageSetMismatch
    );
    TestTrue(
        TEXT("Package mismatch names the incompatible field"),
        MismatchField == FName(TEXT("package_set"))
    );
    return true;
}

bool FSharNetworkingCapabilityMismatchTest::RunTest(
    const FString& Parameters
)
{
    (void)Parameters;
    const FSharCompatibilitySnapshot Client = MakeClientSnapshot();
    FSharCompatibilitySnapshot Server = MakeServerSnapshot();
    Server.CapabilityIds.Add(FName(TEXT("mission_authority")));
    FName MismatchField;

    TestTrue(
        TEXT("Missing required capability is rejected"),
        USharCompatibilityHandshake::Evaluate(
            Client,
            Server,
            MismatchField
        )
            == ESharCompatibilityResult::CapabilityMismatch
    );
    TestTrue(
        TEXT("Capability mismatch names the incompatible field"),
        MismatchField == FName(TEXT("capabilities"))
    );
    return true;
}

#endif
