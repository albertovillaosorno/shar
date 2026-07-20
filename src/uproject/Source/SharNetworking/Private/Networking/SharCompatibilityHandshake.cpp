// File: SharCompatibilityHandshake.cpp
// Path: src/uproject/Source/SharNetworking/Private/Networking/SharCompatibilityHandshake.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: deterministic compatibility comparison only; mismatch rejects before world mutation.
// ADR: docs/adr/modding/mod-owned-multiplayer-adapters-and-community-servers.md
// LARGE-FILE owner=SharNetworking; reason=cohesive fail-closed handshake comparison;
// split=extract optional schema negotiation if adapter-defined fields expand;
// validation=validate.sh SharNetworking plus Unreal automation; review=2027-01.

#include "Networking/SharCompatibilityHandshake.h"

#include "Algo/AllOf.h"
#include "Algo/AnyOf.h"
#include "Content/SharPrimaryContentDefinition.h"
#include "Platform/SharPlatformProfileDefinition.h"

static bool IsSha256Revision(const FString& Revision)
{
    return Revision.StartsWith(TEXT("sha256:"));
}

static void SetMismatchField(FName& OutMismatchField, const TCHAR* Field)
{
    OutMismatchField = FName(Field);
}

bool USharCompatibilityHandshake::IsValidSnapshot(
    const FSharCompatibilitySnapshot& Snapshot
)
{
    const bool bValidIdentity =
        USharPrimaryContentDefinition::IsCanonicalIdentifier(
            Snapshot.SessionId
        )
        && USharPrimaryContentDefinition::IsCanonicalIdentifier(
            Snapshot.SessionRoleId
        )
        && USharPrimaryContentDefinition::IsCanonicalIdentifier(
            Snapshot.ProtocolId
        );
    const bool bValidRevision =
        !Snapshot.ProtocolRevision.IsEmpty()
        && IsSha256Revision(Snapshot.RuntimeContractRevision)
        && IsSha256Revision(Snapshot.CatalogRevision)
        && IsSha256Revision(Snapshot.PackageSetDigest);
    const bool bValidTarget =
        USharPlatformProfileDefinition::IsSupportedTarget(
            Snapshot.Platform,
            Snapshot.Architecture
        );
    const bool bValidCapabilities = Algo::AllOf(
        Snapshot.CapabilityIds,
        [](const FName& CapabilityId)
        {
            return USharPrimaryContentDefinition::IsCanonicalIdentifier(
                CapabilityId
            );
        }
    );
    return bValidIdentity && bValidRevision && bValidTarget
        && bValidCapabilities;
}

bool USharCompatibilityHandshake::HasRequiredCapabilities(
    const TArray<FName>& ClientCapabilities,
    const TArray<FName>& ServerCapabilities
)
{
    return Algo::AllOf(
        ServerCapabilities,
        [&ClientCapabilities](const FName& RequiredCapability)
        {
            return Algo::AnyOf(
                ClientCapabilities,
                [&RequiredCapability](const FName& ClientCapability)
                {
                    return ClientCapability == RequiredCapability;
                }
            );
        }
    );
}

ESharCompatibilityResult USharCompatibilityHandshake::EvaluateRevisions(
    const FSharCompatibilitySnapshot& Client,
    const FSharCompatibilitySnapshot& Server,
    FName& OutMismatchField
)
{
    if (Client.ProtocolId != Server.ProtocolId
        || Client.ProtocolRevision != Server.ProtocolRevision)
    {
        SetMismatchField(OutMismatchField, TEXT("protocol"));
        return ESharCompatibilityResult::ProtocolMismatch;
    }
    if (Client.RuntimeContractRevision != Server.RuntimeContractRevision)
    {
        SetMismatchField(OutMismatchField, TEXT("runtime_contract"));
        return ESharCompatibilityResult::RuntimeMismatch;
    }
    if (Client.PackageSetDigest != Server.PackageSetDigest)
    {
        SetMismatchField(OutMismatchField, TEXT("package_set"));
        return ESharCompatibilityResult::PackageSetMismatch;
    }
    if (Client.CatalogRevision != Server.CatalogRevision)
    {
        SetMismatchField(OutMismatchField, TEXT("catalog"));
        return ESharCompatibilityResult::CatalogMismatch;
    }
    return ESharCompatibilityResult::Compatible;
}

ESharCompatibilityResult USharCompatibilityHandshake::EvaluatePolicy(
    const FSharCompatibilitySnapshot& Client,
    const FSharCompatibilitySnapshot& Server,
    FName& OutMismatchField
)
{
    if (Client.Platform != Server.Platform
        || Client.Architecture != Server.Architecture)
    {
        SetMismatchField(OutMismatchField, TEXT("target"));
        return ESharCompatibilityResult::TargetMismatch;
    }
    if (!HasRequiredCapabilities(Client.CapabilityIds, Server.CapabilityIds))
    {
        SetMismatchField(OutMismatchField, TEXT("capabilities"));
        return ESharCompatibilityResult::CapabilityMismatch;
    }
    if (Client.AuthorityModel != Server.AuthorityModel)
    {
        SetMismatchField(OutMismatchField, TEXT("authority"));
        return ESharCompatibilityResult::AuthorityMismatch;
    }
    if (Client.SavePolicy != Server.SavePolicy)
    {
        SetMismatchField(OutMismatchField, TEXT("save_policy"));
        return ESharCompatibilityResult::SavePolicyMismatch;
    }
    if (Client.AchievementPolicy != Server.AchievementPolicy)
    {
        SetMismatchField(OutMismatchField, TEXT("achievement_policy"));
        return ESharCompatibilityResult::AchievementPolicyMismatch;
    }
    return ESharCompatibilityResult::Compatible;
}

ESharCompatibilityResult USharCompatibilityHandshake::Evaluate(
    const FSharCompatibilitySnapshot& Client,
    const FSharCompatibilitySnapshot& Server,
    FName& OutMismatchField
)
{
    OutMismatchField = FName();
    if (!IsValidSnapshot(Client) || !IsValidSnapshot(Server))
    {
        SetMismatchField(OutMismatchField, TEXT("snapshot"));
        return ESharCompatibilityResult::InvalidSnapshot;
    }
    const ESharCompatibilityResult RevisionResult = EvaluateRevisions(
        Client,
        Server,
        OutMismatchField
    );
    if (RevisionResult != ESharCompatibilityResult::Compatible)
    {
        return RevisionResult;
    }
    return EvaluatePolicy(Client, Server, OutMismatchField);
}
