// File: SharFrontendCatalogSubsystem.cpp
// Path: src/uproject/Source/SharUI/Private/UI/SharFrontendCatalogSubsystem.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: in-memory frontend catalog registration, activation, and immutable lookup only.
// Specification: docs/technical/unreal/frontend-screen-flow-and-settings-runtime.md

#include "UI/SharFrontendCatalogSubsystem.h"

#include "Content/SharPrimaryContentDefinition.h"
#include "UI/SharFrontendCatalogDefinition.h"


static ESharFrontendCatalogResult ValidateScreenUniqueness(
    const TArray<TObjectPtr<USharFrontendCatalogDefinition>>& Definitions
)
{
    TSet<FName> ScreenIds;
    for (const TObjectPtr<USharFrontendCatalogDefinition>& Definition
         : Definitions)
    {
        if (Definition == nullptr)
        {
            return ESharFrontendCatalogResult::InvalidDefinition;
        }
        for (const FSharFrontendScreenDefinition& Screen : Definition->Screens)
        {
            if (ScreenIds.Contains(Screen.ScreenId))
            {
                return ESharFrontendCatalogResult::DuplicateScreen;
            }
            ScreenIds.Add(Screen.ScreenId);
        }
    }
    return ESharFrontendCatalogResult::Accepted;
}

bool USharFrontendCatalogSubsystem::Configure(
    const FString& InCatalogRevision,
    const FName& InRootCatalogId
)
{
    if (bActive || InCatalogRevision.IsEmpty()
        || !InCatalogRevision.Contains(TEXT(":"))
        || !USharPrimaryContentDefinition::IsCanonicalIdentifier(
            InRootCatalogId
        ))
    {
        return false;
    }
    CatalogRevision = InCatalogRevision;
    RootCatalogId = InRootCatalogId;
    return true;
}

ESharFrontendCatalogResult USharFrontendCatalogSubsystem::RegisterCatalog(
    USharFrontendCatalogDefinition* Definition
)
{
    if (bActive)
    {
        return ESharFrontendCatalogResult::AlreadyActive;
    }
    if (Definition == nullptr)
    {
        return ESharFrontendCatalogResult::InvalidDefinition;
    }
    TArray<FText> Errors;
    Definition->GatherValidationErrors(Errors);
    if (!Errors.IsEmpty())
    {
        return ESharFrontendCatalogResult::InvalidDefinition;
    }
    if (FindCatalog(Definition->CanonicalId) != nullptr)
    {
        return ESharFrontendCatalogResult::DuplicateCatalog;
    }
    Definitions.Add(Definition);
    return ESharFrontendCatalogResult::Accepted;
}

ESharFrontendCatalogResult USharFrontendCatalogSubsystem::Activate()
{
    if (bActive)
    {
        return ESharFrontendCatalogResult::AlreadyActive;
    }
    if (CatalogRevision.IsEmpty() || !CatalogRevision.Contains(TEXT(":")))
    {
        return ESharFrontendCatalogResult::InvalidRevision;
    }
    if (!USharPrimaryContentDefinition::IsCanonicalIdentifier(RootCatalogId))
    {
        return ESharFrontendCatalogResult::InvalidRootCatalog;
    }
    if (Definitions.IsEmpty())
    {
        return ESharFrontendCatalogResult::EmptyCatalog;
    }
    if (FindCatalog(RootCatalogId) == nullptr)
    {
        return ESharFrontendCatalogResult::MissingRootCatalog;
    }

    const ESharFrontendCatalogResult Validation =
        ValidateScreenUniqueness(Definitions);
    if (Validation != ESharFrontendCatalogResult::Accepted)
    {
        return Validation;
    }
    bActive = true;
    return ESharFrontendCatalogResult::Accepted;
}

bool USharFrontendCatalogSubsystem::IsActive() const
{
    return bActive;
}

int32 USharFrontendCatalogSubsystem::GetCatalogCount() const
{
    return Definitions.Num();
}

const USharFrontendCatalogDefinition*
USharFrontendCatalogSubsystem::FindCatalog(const FName& CatalogId) const
{
    for (const TObjectPtr<USharFrontendCatalogDefinition>& Definition
         : Definitions)
    {
        if (Definition != nullptr && Definition->CanonicalId == CatalogId)
        {
            return Definition;
        }
    }
    return nullptr;
}

const FSharFrontendScreenDefinition*
USharFrontendCatalogSubsystem::FindScreen(const FName& ScreenId) const
{
    for (const TObjectPtr<USharFrontendCatalogDefinition>& Definition
         : Definitions)
    {
        if (Definition == nullptr)
        {
            continue;
        }
        const FSharFrontendScreenDefinition* Screen =
            Definition->FindScreen(ScreenId);
        if (Screen != nullptr)
        {
            return Screen;
        }
    }
    return nullptr;
}

const FSharFrontendScreenDefinition*
USharFrontendCatalogSubsystem::GetInitialScreen() const
{
    const USharFrontendCatalogDefinition* RootCatalog =
        FindCatalog(RootCatalogId);
    return RootCatalog == nullptr
        ? nullptr
        : RootCatalog->FindScreen(RootCatalog->InitialScreenId);
}

const FString& USharFrontendCatalogSubsystem::GetCatalogRevision() const
{
    return CatalogRevision;
}

const FName& USharFrontendCatalogSubsystem::GetRootCatalogId() const
{
    return RootCatalogId;
}
