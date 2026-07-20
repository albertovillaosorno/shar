// File: SharFrontendCatalogDefinition.cpp
// Path: src/uproject/Source/SharUI/Private/UI/SharFrontendCatalogDefinition.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: load-free frontend screen-definition validation and lookup only.
// Specification: docs/technical/unreal/frontend-screen-flow-and-settings-runtime.md

#include "UI/SharFrontendCatalogDefinition.h"

#include "Content/SharPrimaryContentDefinition.h"
#include "Engine/DataAsset.h"

static bool IsRequiredIdentityValid(const FName& Value)
{
    return !Value.IsNone()
        && USharPrimaryContentDefinition::IsCanonicalIdentifier(Value);
}

static void AddIdentityError(
    TArray<FText>& OutErrors,
    const TCHAR* Field,
    const FName& Value
)
{
    OutErrors.Add(FText::Format(
        NSLOCTEXT(
            "SharFrontendCatalogDefinition",
            "InvalidIdentity",
            "Frontend {0} identity '{1}' is not canonical."
        ),
        FText::FromString(Field),
        FText::FromName(Value)
    ));
}

template <typename TValue>
static bool HasDuplicateValues(const TArray<TValue>& Values)
{
    TSet<TValue> UniqueValues;
    for (const TValue& Value : Values)
    {
        if (UniqueValues.Contains(Value))
        {
            return true;
        }
        UniqueValues.Add(Value);
    }
    return false;
}

static void ValidateScreenIdentity(
    const FSharFrontendScreenDefinition& Screen,
    TSet<FName>& ScreenIds,
    TArray<FText>& OutErrors
)
{
    if (!IsRequiredIdentityValid(Screen.ScreenId))
    {
        AddIdentityError(OutErrors, TEXT("screen"), Screen.ScreenId);
        return;
    }
    if (ScreenIds.Contains(Screen.ScreenId))
    {
        OutErrors.Add(FText::FromString(
            TEXT("Frontend screen identities must be unique.")
        ));
        return;
    }
    ScreenIds.Add(Screen.ScreenId);
}

static void ValidateRequiredScreenIdentities(
    const FSharFrontendScreenDefinition& Screen,
    TArray<FText>& OutErrors
)
{
    if (!IsRequiredIdentityValid(Screen.ViewModelSchemaId))
    {
        AddIdentityError(
            OutErrors,
            TEXT("view-model schema"),
            Screen.ViewModelSchemaId
        );
    }
    if (!IsRequiredIdentityValid(Screen.SemanticActionSetId))
    {
        AddIdentityError(
            OutErrors,
            TEXT("semantic action set"),
            Screen.SemanticActionSetId
        );
    }
    if (!IsRequiredIdentityValid(Screen.EntryPredicateId))
    {
        AddIdentityError(
            OutErrors,
            TEXT("entry predicate"),
            Screen.EntryPredicateId
        );
    }
    if (!IsRequiredIdentityValid(Screen.ExitPolicyId))
    {
        AddIdentityError(
            OutErrors,
            TEXT("exit policy"),
            Screen.ExitPolicyId
        );
    }
    if (!IsRequiredIdentityValid(Screen.FocusPolicyId))
    {
        AddIdentityError(
            OutErrors,
            TEXT("focus policy"),
            Screen.FocusPolicyId
        );
    }
}

static void ValidateBundleIds(
    const FSharFrontendScreenDefinition& Screen,
    TArray<FText>& OutErrors
)
{
    TSet<FName> BundleIds;
    for (const FName& BundleId : Screen.RequiredBundleIds)
    {
        if (!IsRequiredIdentityValid(BundleId))
        {
            AddIdentityError(OutErrors, TEXT("bundle"), BundleId);
            continue;
        }
        if (BundleIds.Contains(BundleId))
        {
            OutErrors.Add(FText::FromString(
                TEXT("Frontend screen bundle identities must be unique.")
            ));
            continue;
        }
        BundleIds.Add(BundleId);
    }
}

static void ValidateDestinationIds(
    const FSharFrontendScreenDefinition& Screen,
    TArray<FText>& OutErrors
)
{
    TSet<FName> DestinationIds;
    for (const FName& DestinationId : Screen.AllowedDestinationScreenIds)
    {
        if (!IsRequiredIdentityValid(DestinationId))
        {
            AddIdentityError(
                OutErrors,
                TEXT("destination screen"),
                DestinationId
            );
            continue;
        }
        if (DestinationId == Screen.ScreenId)
        {
            OutErrors.Add(FText::FromString(
                TEXT("Frontend screens cannot navigate to themselves.")
            ));
            continue;
        }
        if (DestinationIds.Contains(DestinationId))
        {
            OutErrors.Add(FText::FromString(
                TEXT("Frontend destination identities must be unique per screen.")
            ));
            continue;
        }
        DestinationIds.Add(DestinationId);
    }
}


static bool HasCrossPhaseRequirement(
    const FSharFrontendScreenDefinition& Screen
)
{
    for (const ESharFrontendReadinessKind PreCommitKind
         : Screen.PreCommitRequirements)
    {
        for (const ESharFrontendReadinessKind PostCommitKind
             : Screen.PostCommitRequirements)
        {
            if (PreCommitKind == PostCommitKind)
            {
                return true;
            }
        }
    }
    return false;
}

static void ValidateReadinessRequirements(
    const FSharFrontendScreenDefinition& Screen,
    TArray<FText>& OutErrors
)
{
    if (HasDuplicateValues(Screen.PreCommitRequirements))
    {
        OutErrors.Add(FText::FromString(
            TEXT("Frontend pre-commit readiness requirements must be unique.")
        ));
    }
    if (HasDuplicateValues(Screen.PostCommitRequirements))
    {
        OutErrors.Add(FText::FromString(
            TEXT("Frontend post-commit readiness requirements must be unique.")
        ));
    }
    if (HasCrossPhaseRequirement(Screen))
    {
        OutErrors.Add(FText::FromString(
            TEXT("Frontend readiness kinds cannot span both commit phases.")
        ));
    }
}

static void ValidateScreen(
    const FSharFrontendScreenDefinition& Screen,
    TSet<FName>& ScreenIds,
    TArray<FText>& OutErrors
)
{
    ValidateScreenIdentity(Screen, ScreenIds, OutErrors);
    ValidateRequiredScreenIdentities(Screen, OutErrors);
    ValidateBundleIds(Screen, OutErrors);
    ValidateDestinationIds(Screen, OutErrors);
    ValidateReadinessRequirements(Screen, OutErrors);
}

static void ValidateInitialScreen(
    const USharFrontendCatalogDefinition& Definition,
    TArray<FText>& OutErrors
)
{
    const FSharFrontendScreenDefinition* InitialScreen =
        Definition.FindScreen(Definition.InitialScreenId);
    if (InitialScreen == nullptr)
    {
        OutErrors.Add(FText::FromString(
            TEXT("Frontend initial screen must resolve within the catalog.")
        ));
        return;
    }
    if (InitialScreen->Layer != ESharFrontendLayer::Boot
        && InitialScreen->Layer != ESharFrontendLayer::Primary)
    {
        OutErrors.Add(FText::FromString(
            TEXT("Frontend initial screen must use the boot or primary layer.")
        ));
    }
}

static void ValidateDestinationResolution(
    const USharFrontendCatalogDefinition& Definition,
    const FSharFrontendScreenDefinition& Screen,
    TArray<FText>& OutErrors
)
{
    for (const FName& DestinationId : Screen.AllowedDestinationScreenIds)
    {
        if (Definition.FindScreen(DestinationId) == nullptr)
        {
            OutErrors.Add(FText::Format(
                NSLOCTEXT(
                    "SharFrontendCatalogDefinition",
                    "MissingDestination",
                    "Frontend destination '{0}' does not resolve."
                ),
                FText::FromName(DestinationId)
            ));
        }
    }
}

void USharFrontendCatalogDefinition::GatherValidationErrors(
    TArray<FText>& OutErrors
) const
{
    USharPrimaryContentDefinition::GatherValidationErrors(OutErrors);
    if (!IsRequiredIdentityValid(InitialScreenId))
    {
        AddIdentityError(OutErrors, TEXT("initial screen"), InitialScreenId);
    }
    if (Screens.IsEmpty())
    {
        OutErrors.Add(FText::FromString(
            TEXT("Frontend catalog requires at least one screen definition.")
        ));
        return;
    }

    TSet<FName> ScreenIds;
    for (const FSharFrontendScreenDefinition& Screen : Screens)
    {
        ValidateScreen(Screen, ScreenIds, OutErrors);
    }
    ValidateInitialScreen(*this, OutErrors);
    for (const FSharFrontendScreenDefinition& Screen : Screens)
    {
        ValidateDestinationResolution(*this, Screen, OutErrors);
    }
}

const FSharFrontendScreenDefinition* USharFrontendCatalogDefinition::FindScreen(
    const FName& ScreenId
) const
{
    for (const FSharFrontendScreenDefinition& Screen : Screens)
    {
        if (Screen.ScreenId == ScreenId)
        {
            return &Screen;
        }
    }
    return nullptr;
}

FPrimaryAssetType USharFrontendCatalogDefinition::GetDefinitionAssetType() const
{
    return {TEXT("SharFrontendCatalog")};
}
