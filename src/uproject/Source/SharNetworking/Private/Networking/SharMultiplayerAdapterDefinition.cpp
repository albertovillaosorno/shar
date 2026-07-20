// File: SharMultiplayerAdapterDefinition.cpp
// Path: src/uproject/Source/SharNetworking/Private/Networking/SharMultiplayerAdapterDefinition.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: load-free multiplayer adapter declaration validation only; no transport or session activation.
// ADR: docs/adr/modding/mod-owned-multiplayer-adapters-and-community-servers.md
// LARGE-FILE owner=SharNetworking; reason=cohesive adapter declaration validation;
// split=extract package validation if package negotiation becomes independently versioned;
// validation=validate.sh SharNetworking plus Unreal automation; review=2027-01.

#include "Networking/SharMultiplayerAdapterDefinition.h"

#include "Algo/AnyOf.h"
#include "Content/SharPrimaryContentDefinition.h"
#include "Platform/SharPlatformProfileDefinition.h"

static void AddNetworkDefinitionError(
    TArray<FText>& OutErrors,
    const TCHAR* Message
)
{
    OutErrors.Add(FText::FromString(Message));
}

static bool IsCanonicalNetworkId(const FName& Candidate)
{
    return USharPrimaryContentDefinition::IsCanonicalIdentifier(Candidate);
}

static bool IsSha256Identity(const FString& Candidate)
{
    return Candidate.StartsWith(TEXT("sha256:"));
}

static void AppendIdentityErrors(
    const USharMultiplayerAdapterDefinition& Definition,
    TArray<FText>& OutErrors
)
{
    const bool bInvalidIdentity =
        !IsCanonicalNetworkId(Definition.MultiplayerModeId)
        || !IsCanonicalNetworkId(Definition.ProtocolId)
        || !IsCanonicalNetworkId(Definition.TeardownPolicyId);
    if (bInvalidIdentity)
    {
        AddNetworkDefinitionError(
            OutErrors,
            TEXT("Multiplayer mode, protocol, and teardown identities must be canonical.")
        );
    }
    if (Definition.ProtocolRevision.IsEmpty())
    {
        AddNetworkDefinitionError(
            OutErrors,
            TEXT("Multiplayer protocol revision must be explicit.")
        );
    }
    const bool bInvalidRevision =
        !IsSha256Identity(Definition.RuntimeContractRevision)
        || !IsSha256Identity(Definition.RequiredCatalogRevision)
        || !IsSha256Identity(Definition.PackageSetDigest);
    if (bInvalidRevision)
    {
        AddNetworkDefinitionError(
            OutErrors,
            TEXT("Runtime, catalog, and package-set revisions require SHA-256 identities.")
        );
    }
}

static void AppendCanonicalListErrors(
    const TArray<FName>& Values,
    const TCHAR* MissingMessage,
    const TCHAR* InvalidMessage,
    const TCHAR* DuplicateMessage,
    TArray<FText>& OutErrors
)
{
    if (Values.IsEmpty())
    {
        AddNetworkDefinitionError(OutErrors, MissingMessage);
    }
    TSet<FName> Seen;
    for (const FName& Value : Values)
    {
        if (!IsCanonicalNetworkId(Value))
        {
            AddNetworkDefinitionError(OutErrors, InvalidMessage);
        }
        if (Seen.Contains(Value))
        {
            AddNetworkDefinitionError(OutErrors, DuplicateMessage);
        }
        Seen.Add(Value);
    }
}

static bool TargetsMatch(
    const FSharNetworkServerTarget& Left,
    const FSharNetworkServerTarget& Right
)
{
    return Left.Platform == Right.Platform
        && Left.Architecture == Right.Architecture;
}

static void AppendTargetErrors(
    const TArray<FSharNetworkServerTarget>& Targets,
    TArray<FText>& OutErrors
)
{
    if (Targets.IsEmpty())
    {
        AddNetworkDefinitionError(
            OutErrors,
            TEXT("Multiplayer adapter requires at least one exact server target.")
        );
    }
    TArray<FSharNetworkServerTarget> SeenTargets;
    for (const FSharNetworkServerTarget& Target : Targets)
    {
        if (!USharPlatformProfileDefinition::IsSupportedTarget(
            Target.Platform,
            Target.Architecture
        ))
        {
            AddNetworkDefinitionError(
                OutErrors,
                TEXT("Multiplayer server target is not supported by platform policy.")
            );
        }
        const bool bDuplicate = Algo::AnyOf(
            SeenTargets,
            [&Target](const FSharNetworkServerTarget& SeenTarget)
            {
                return TargetsMatch(Target, SeenTarget);
            }
        );
        if (bDuplicate)
        {
            AddNetworkDefinitionError(
                OutErrors,
                TEXT("Multiplayer server targets must be unique.")
            );
        }
        SeenTargets.Add(Target);
    }
}

static bool PackageIsValid(const FSharNetworkRequiredPackage& Package)
{
    return IsCanonicalNetworkId(Package.NamespaceId)
        && !Package.Version.IsEmpty()
        && Package.Version.Contains(TEXT("."))
        && IsSha256Identity(Package.PackageDigest);
}

static void AppendPackageErrors(
    const TArray<FSharNetworkRequiredPackage>& Packages,
    TArray<FText>& OutErrors
)
{
    TSet<FName> SeenNamespaces;
    for (const FSharNetworkRequiredPackage& Package : Packages)
    {
        if (!PackageIsValid(Package))
        {
            AddNetworkDefinitionError(
                OutErrors,
                TEXT("Required network packages need canonical namespace, dotted version, and SHA-256 digest.")
            );
        }
        if (SeenNamespaces.Contains(Package.NamespaceId))
        {
            AddNetworkDefinitionError(
                OutErrors,
                TEXT("Required network package namespaces must be unique.")
            );
        }
        SeenNamespaces.Add(Package.NamespaceId);
    }
}

static void AppendTrustAndPolicyErrors(
    const USharMultiplayerAdapterDefinition& Definition,
    TArray<FText>& OutErrors
)
{
    const bool bUnapprovedNative =
        Definition.NativeCodePolicy
            == ESharNetworkNativeCodePolicy::ExplicitlyTrustedNative
        && !Definition.bExplicitUserApprovalRequired;
    if (bUnapprovedNative)
    {
        AddNetworkDefinitionError(
            OutErrors,
            TEXT("Native multiplayer adapters require explicit full-process trust approval.")
        );
    }
    const bool bInvalidCustomAchievement =
        Definition.AchievementPolicy
            == ESharNetworkAchievementPolicy::CustomProvider
        && Definition.SavePolicy != ESharNetworkSavePolicy::NamespacedModOwned;
    if (bInvalidCustomAchievement)
    {
        AddNetworkDefinitionError(
            OutErrors,
            TEXT("Custom multiplayer achievements require namespaced mod-owned persistence.")
        );
    }
}

void USharMultiplayerAdapterDefinition::GatherValidationErrors(
    TArray<FText>& OutErrors
) const
{
    AppendIdentityErrors(*this, OutErrors);
    AppendCanonicalListErrors(
        ClientRoleIds,
        TEXT("Multiplayer adapter requires at least one client role."),
        TEXT("Multiplayer client roles must be canonical."),
        TEXT("Multiplayer client roles must be unique."),
        OutErrors
    );
    AppendCanonicalListErrors(
        RequiredCapabilityIds,
        TEXT("Multiplayer adapter requires at least one capability."),
        TEXT("Multiplayer capabilities must be canonical."),
        TEXT("Multiplayer capabilities must be unique."),
        OutErrors
    );
    AppendTargetErrors(ServerTargets, OutErrors);
    AppendPackageErrors(RequiredPackages, OutErrors);
    AppendTrustAndPolicyErrors(*this, OutErrors);
}
