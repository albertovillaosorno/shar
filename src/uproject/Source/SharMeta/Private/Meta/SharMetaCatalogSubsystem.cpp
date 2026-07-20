// File: SharMetaCatalogSubsystem.cpp
// Path: src/uproject/Source/SharMeta/Private/Meta/SharMetaCatalogSubsystem.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: in-memory meta-catalog registration, activation, and immutable lookup only.
// Specification: docs/technical/unreal/progression-collectibles-and-cheats.md

#include "Meta/SharMetaCatalogSubsystem.h"

#include "Meta/SharMetaCatalogDefinition.h"

bool USharMetaCatalogSubsystem::ConfigureRevision(
    const FString& InCatalogRevision
)
{
    if (bActive || InCatalogRevision.IsEmpty()
        || !InCatalogRevision.Contains(TEXT(":")))
    {
        return false;
    }
    CatalogRevision = InCatalogRevision;
    return true;
}

ESharMetaCatalogResult USharMetaCatalogSubsystem::RegisterCatalog(
    USharMetaCatalogDefinition* Definition
)
{
    if (bActive)
    {
        return ESharMetaCatalogResult::AlreadyActive;
    }
    if (Definition == nullptr)
    {
        return ESharMetaCatalogResult::InvalidDefinition;
    }
    TArray<FText> Errors;
    Definition->GatherValidationErrors(Errors);
    if (!Errors.IsEmpty())
    {
        return ESharMetaCatalogResult::InvalidDefinition;
    }
    if (FindCatalog(Definition->CanonicalId) != nullptr)
    {
        return ESharMetaCatalogResult::DuplicateCatalog;
    }
    Definitions.Add(Definition);
    return ESharMetaCatalogResult::Accepted;
}

ESharMetaCatalogResult USharMetaCatalogSubsystem::Activate()
{
    if (bActive)
    {
        return ESharMetaCatalogResult::AlreadyActive;
    }
    if (CatalogRevision.IsEmpty() || !CatalogRevision.Contains(TEXT(":")))
    {
        return ESharMetaCatalogResult::InvalidRevision;
    }
    if (Definitions.IsEmpty())
    {
        return ESharMetaCatalogResult::EmptyCatalog;
    }
    bActive = true;
    return ESharMetaCatalogResult::Accepted;
}

bool USharMetaCatalogSubsystem::IsActive() const
{
    return bActive;
}

int32 USharMetaCatalogSubsystem::GetCatalogCount() const
{
    return Definitions.Num();
}

const USharMetaCatalogDefinition* USharMetaCatalogSubsystem::FindCatalog(
    const FName& CatalogId
) const
{
    for (const TObjectPtr<USharMetaCatalogDefinition>& Definition : Definitions)
    {
        if (Definition != nullptr && Definition->CanonicalId == CatalogId)
        {
            return Definition;
        }
    }
    return nullptr;
}

const FSharCheatDefinition* USharMetaCatalogSubsystem::FindCheatBySequence(
    const FName& CatalogId,
    const TArray<ESharCheatInputToken>& InputTokens
) const
{
    const USharMetaCatalogDefinition* Catalog = FindCatalog(CatalogId);
    return Catalog == nullptr
        ? nullptr
        : Catalog->FindCheatBySequence(InputTokens);
}

const FString& USharMetaCatalogSubsystem::GetCatalogRevision() const
{
    return CatalogRevision;
}
