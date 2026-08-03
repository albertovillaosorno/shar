// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT
//
// Boundary-Contract:
// - Owns:
//   - Shar primary content validation composition module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Shar primary content validation composition module.
// - Description:
//   - Implements the declared composition module responsibility for project.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Shar primary content validation composition module.

#include "Content/SharPrimaryContentValidation.h"

#include "Content/SharPrimaryContentDefinition.h"
#include "Engine/DataAsset.h"

static void AppendUniqueNameErrors(
    const TArray<FName>& Values,
    const FText& FieldName,
    TArray<FText>& OutErrors
)
{
    TSet<FName> SeenValues;
    for (const FName& Value : Values)
    {
        if (Value.IsNone())
        {
            OutErrors.Add(FText::Format(
                NSLOCTEXT(
                    "SharPrimaryContentDefinition",
                    "EmptyName",
                    "{0} contains an empty identity."
                ),
                FieldName
            ));
            continue;
        }
        if (SeenValues.Contains(Value))
        {
            OutErrors.Add(FText::Format(
                NSLOCTEXT(
                    "SharPrimaryContentDefinition",
                    "DuplicateName",
                    "{0} contains duplicate identity '{1}'."
                ),
                FieldName,
                FText::FromName(Value)
            ));
            continue;
        }
        SeenValues.Add(Value);
    }
}

static void AppendAliasErrors(
    const USharPrimaryContentDefinition& Definition,
    TArray<FText>& OutErrors
)
{
    AppendUniqueNameErrors(
        Definition.Aliases,
        NSLOCTEXT("SharPrimaryContentDefinition", "AliasesField", "Aliases"),
        OutErrors
    );
    for (const FName& Alias : Definition.Aliases)
    {
        if (!USharPrimaryContentDefinition::IsCanonicalIdentifier(Alias))
        {
            OutErrors.Add(FText::Format(
                NSLOCTEXT(
                    "SharPrimaryContentDefinition",
                    "InvalidAlias",
                    "Alias '{0}' must satisfy the canonical identifier rule."
                ),
                FText::FromName(Alias)
            ));
        }
        if (Alias == Definition.CanonicalId)
        {
            OutErrors.Add(NSLOCTEXT(
                "SharPrimaryContentDefinition",
                "AliasMatchesCanonicalId",
                "Aliases cannot repeat CanonicalId."
            ));
        }
    }
}

void FSharPrimaryContentValidation::AppendIdentityErrors(
    const USharPrimaryContentDefinition& Definition,
    const FPrimaryAssetType& AssetType,
    TArray<FText>& OutErrors
)
{
    if (!AssetType.IsValid())
    {
        OutErrors.Add(NSLOCTEXT(
            "SharPrimaryContentDefinition",
            "InvalidAssetType",
            "The definition class does not declare a valid Primary Asset type."
        ));
    }
    if (!USharPrimaryContentDefinition::IsCanonicalIdentifier(
        Definition.CanonicalId
    ))
    {
        OutErrors.Add(NSLOCTEXT(
            "SharPrimaryContentDefinition",
            "InvalidCanonicalId",
            // jig-ignore-next-line: exact syntax is indivisible
            "CanonicalId must be lowercase ASCII snake_case without repeated underscores."
        ));
    }
    if (Definition.DisplayName.IsEmpty())
    {
        OutErrors.Add(NSLOCTEXT(
            "SharPrimaryContentDefinition",
            "MissingDisplayName",
            "DisplayName must contain localizable player-facing text."
        ));
    }
    AppendAliasErrors(Definition, OutErrors);
}

void FSharPrimaryContentValidation::AppendProvenanceErrors(
    const USharPrimaryContentDefinition& Definition,
    TArray<FText>& OutErrors
)
{
    AppendUniqueNameErrors(
        Definition.SourcePackageIds,
        NSLOCTEXT(
            "SharPrimaryContentDefinition",
            "SourcePackageIdsField",
            "SourcePackageIds"
        ),
        OutErrors
    );
    if (Definition.SourcePackageIds.IsEmpty())
    {
        OutErrors.Add(NSLOCTEXT(
            "SharPrimaryContentDefinition",
            "MissingSourcePackages",
            "At least one deterministic source package identity is required."
        ));
    }
    if (Definition.RevisionToken.TrimStartAndEnd().IsEmpty())
    {
        OutErrors.Add(NSLOCTEXT(
            "SharPrimaryContentDefinition",
            "MissingRevisionToken",
            "RevisionToken must identify the generated-data revision."
        ));
    }
    if (Definition.DefinitionSchemaVersion < 1)
    {
        OutErrors.Add(NSLOCTEXT(
            "SharPrimaryContentDefinition",
            "InvalidSchemaVersion",
            "DefinitionSchemaVersion must be at least one."
        ));
    }
    if (!USharPrimaryContentDefinition::IsCanonicalIdentifier(
        Definition.ValidationProfile
    ))
    {
        OutErrors.Add(NSLOCTEXT(
            "SharPrimaryContentDefinition",
            "InvalidValidationProfile",
            "ValidationProfile must be a canonical lowercase identifier."
        ));
    }
    if (!USharPrimaryContentDefinition::IsCanonicalIdentifier(
        Definition.OwningFeature
    ))
    {
        OutErrors.Add(NSLOCTEXT(
            "SharPrimaryContentDefinition",
            "InvalidOwningFeature",
            "OwningFeature must be a canonical lowercase identifier."
        ));
    }
}

void FSharPrimaryContentValidation::AppendDependencyErrors(
    const USharPrimaryContentDefinition& Definition,
    const FPrimaryAssetId& SelfId,
    TArray<FText>& OutErrors
)
{
    TSet<FPrimaryAssetId> SeenDependencies;
    for (const FPrimaryAssetId& Dependency : Definition.RequiredDefinitions)
    {
        if (!Dependency.IsValid())
        {
            OutErrors.Add(NSLOCTEXT(
                "SharPrimaryContentDefinition",
                "InvalidDependency",
                // jig-ignore-next-line: exact syntax is indivisible
                "RequiredDefinitions contains an invalid Primary Asset identity."
            ));
            continue;
        }
        if (Dependency == SelfId)
        {
            OutErrors.Add(NSLOCTEXT(
                "SharPrimaryContentDefinition",
                "SelfDependency",
                "A definition cannot require itself."
            ));
        }
        if (SeenDependencies.Contains(Dependency))
        {
            OutErrors.Add(FText::Format(
                NSLOCTEXT(
                    "SharPrimaryContentDefinition",
                    "DuplicateDependency",
                    "RequiredDefinitions contains duplicate identity '{0}'."
                ),
                FText::FromString(Dependency.ToString())
            ));
            continue;
        }
        SeenDependencies.Add(Dependency);
    }
}
