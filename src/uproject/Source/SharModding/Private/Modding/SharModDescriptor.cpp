// File: SharModDescriptor.cpp
// Path: src/uproject/Source/SharModding/Private/Modding/SharModDescriptor.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: load-free mod descriptor validation; no package mounting or code execution.
// ADR: docs/adr/unreal/architecture/aaa-native-content-and-gameplay-foundation.md

#include "Modding/SharModDescriptor.h"

#include "Content/SharPrimaryContentDefinition.h"
#include "Engine/DataAsset.h"

static void AddModError(
    TArray<FText>& OutErrors,
    const TCHAR* Message
)
{
    OutErrors.Add(FText::FromString(Message));
}

static void AppendNamespaceListErrors(
    const TArray<FName>& Values,
    const FName& NamespaceId,
    const TCHAR* InvalidMessage,
    const TCHAR* DuplicateMessage,
    TArray<FText>& OutErrors
)
{
    TSet<FName> Seen;
    for (const FName& Value : Values)
    {
        const bool bInvalid =
            !USharPrimaryContentDefinition::IsCanonicalIdentifier(Value)
            || Value == NamespaceId;
        if (bInvalid)
        {
            AddModError(OutErrors, InvalidMessage);
        }
        if (Seen.Contains(Value))
        {
            AddModError(OutErrors, DuplicateMessage);
        }
        Seen.Add(Value);
    }
}

static void AppendIdentityErrors(
    const USharModDescriptor& Descriptor,
    TArray<FText>& OutErrors
)
{
    if (!USharPrimaryContentDefinition::IsCanonicalIdentifier(
        Descriptor.NamespaceId
    ))
    {
        AddModError(OutErrors, TEXT("Mod namespace must be canonical."));
    }
    if (Descriptor.NamespaceId != Descriptor.CanonicalId)
    {
        AddModError(
            OutErrors,
            TEXT("Mod namespace must equal canonical descriptor identity.")
        );
    }
    if (Descriptor.Version.IsEmpty()
        || !Descriptor.Version.Contains(TEXT(".")))
    {
        AddModError(
            OutErrors,
            TEXT("Mod version must use an explicit dotted version.")
        );
    }
    if (!Descriptor.PackageSetDigest.StartsWith(TEXT("sha256:")))
    {
        AddModError(
            OutErrors,
            TEXT("Mod package set requires a SHA-256 digest identity.")
        );
    }
    if (Descriptor.MinimumGameSchemaVersion <= 0)
    {
        AddModError(
            OutErrors,
            TEXT("Minimum game schema version must be positive.")
        );
    }
}

static void AppendTrustErrors(
    const USharModDescriptor& Descriptor,
    TArray<FText>& OutErrors
)
{
    const bool bUnapprovedNative =
        Descriptor.TrustTier == ESharModTrustTier::Native
        && !Descriptor.bExplicitUserApprovalRequired;
    if (bUnapprovedNative)
    {
        AddModError(
            OutErrors,
            TEXT("Native mods require explicit full-process trust approval.")
        );
    }
    const bool bInvalidServerPolicy =
        Descriptor.TrustTier == ESharModTrustTier::ServerRequired
        && Descriptor.NetworkPolicy
            != ESharModNetworkPolicy::PackageMatchRequired;
    if (bInvalidServerPolicy)
    {
        AddModError(
            OutErrors,
            TEXT("Server-required mods must require package matching.")
        );
    }
}

static void AppendActivationActionErrors(
    const TArray<FName>& ActivationActionIds,
    TArray<FText>& OutErrors
)
{
    TSet<FName> SeenActionIds;
    for (const FName& ActionId : ActivationActionIds)
    {
        if (!USharPrimaryContentDefinition::IsCanonicalIdentifier(ActionId))
        {
            AddModError(
                OutErrors,
                TEXT("Activation actions require canonical identities.")
            );
        }
        if (SeenActionIds.Contains(ActionId))
        {
            AddModError(
                OutErrors,
                TEXT("Activation actions cannot contain duplicates.")
            );
        }
        SeenActionIds.Add(ActionId);
    }
}

static void AppendReplacementErrors(
    const FSharModReplacementDefinition& Replacement,
    TArray<FText>& OutErrors
)
{
    if (!Replacement.TargetAssetId.IsValid())
    {
        AddModError(
            OutErrors,
            TEXT("Mod replacement requires a valid Primary Asset identity.")
        );
    }
    if (!USharPrimaryContentDefinition::IsCanonicalIdentifier(
        Replacement.ScopeId
    ))
    {
        AddModError(
            OutErrors,
            TEXT("Mod replacement scope must be canonical.")
        );
    }
    if (Replacement.Priority < 0)
    {
        AddModError(
            OutErrors,
            TEXT("Mod replacement priority cannot be negative.")
        );
    }
    if (!USharPrimaryContentDefinition::IsCanonicalIdentifier(
        Replacement.RollbackPolicyId
    ))
    {
        AddModError(
            OutErrors,
            TEXT("Mod replacement requires a canonical rollback policy.")
        );
    }
}

static void AppendAllReplacementErrors(
    const TArray<FSharModReplacementDefinition>& Replacements,
    TArray<FText>& OutErrors
)
{
    for (const FSharModReplacementDefinition& Replacement : Replacements)
    {
        AppendReplacementErrors(Replacement, OutErrors);
    }
}

void USharModDescriptor::GatherValidationErrors(
    TArray<FText>& OutErrors
) const
{
    USharPrimaryContentDefinition::GatherValidationErrors(OutErrors);
    AppendIdentityErrors(*this, OutErrors);
    AppendTrustErrors(*this, OutErrors);
    AppendNamespaceListErrors(
        RequiredModNamespaces,
        NamespaceId,
        TEXT("Mod dependencies must be canonical and cannot reference self."),
        TEXT("Mod dependencies cannot contain duplicates."),
        OutErrors
    );
    AppendNamespaceListErrors(
        ConflictingModNamespaces,
        NamespaceId,
        TEXT("Mod conflicts must be canonical and cannot reference self."),
        TEXT("Mod conflicts cannot contain duplicates."),
        OutErrors
    );
    AppendActivationActionErrors(ActivationActionIds, OutErrors);
    AppendAllReplacementErrors(Replacements, OutErrors);
}

FPrimaryAssetType USharModDescriptor::GetDefinitionAssetType() const
{
    return {TEXT("SharModDescriptor")};
}
