// File: SharSaveSchemaCatalogSubsystem.cpp
// Path: src/uproject/Source/SharSave/Private/Save/SharSaveSchemaCatalogSubsystem.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: immutable save-schema catalog registration and lookup only.
// Specification: docs/technical/unreal/platform-save-storage-and-lifecycle.md

#include "Save/SharSaveSchemaCatalogSubsystem.h"

#include "Algo/Find.h"
#include "Save/SharSaveSchemaDefinition.h"

static bool IsCatalogRevision(const FString& Revision)
{
    return Revision.StartsWith(TEXT("sha256:"));
}

bool USharSaveSchemaCatalogSubsystem::ConfigureRevision(
    const FString& InCatalogRevision
)
{
    if (!IsCatalogRevision(InCatalogRevision))
    {
        return false;
    }
    CatalogRevision = InCatalogRevision;
    Definitions.Reset();
    bActive = false;
    return true;
}

ESharSaveSchemaCatalogResult USharSaveSchemaCatalogSubsystem::RegisterSchema(
    USharSaveSchemaDefinition* Definition
)
{
    if (bActive)
    {
        return ESharSaveSchemaCatalogResult::AlreadyActive;
    }
    if (CatalogRevision.IsEmpty() || Definition == nullptr)
    {
        return ESharSaveSchemaCatalogResult::InvalidDefinition;
    }
    TArray<FText> Errors;
    Definition->GatherValidationErrors(Errors);
    if (!Errors.IsEmpty())
    {
        return ESharSaveSchemaCatalogResult::InvalidDefinition;
    }
    if (FindSchema(Definition->CanonicalId) != nullptr)
    {
        return ESharSaveSchemaCatalogResult::DuplicateSchema;
    }
    Definitions.Add(Definition);
    return ESharSaveSchemaCatalogResult::Accepted;
}

ESharSaveSchemaCatalogResult USharSaveSchemaCatalogSubsystem::Activate()
{
    if (bActive)
    {
        return ESharSaveSchemaCatalogResult::AlreadyActive;
    }
    if (Definitions.IsEmpty())
    {
        return ESharSaveSchemaCatalogResult::EmptyCatalog;
    }
    bActive = true;
    return ESharSaveSchemaCatalogResult::Accepted;
}

const USharSaveSchemaDefinition* USharSaveSchemaCatalogSubsystem::FindSchema(
    const FName& SchemaId
) const
{
    const TObjectPtr<USharSaveSchemaDefinition>* Definition =
        Algo::FindByPredicate(
            Definitions,
            [&SchemaId](const TObjectPtr<USharSaveSchemaDefinition>& Candidate)
            {
                return Candidate != nullptr && Candidate->CanonicalId == SchemaId;
            }
        );
    return Definition == nullptr ? nullptr : *Definition;
}

bool USharSaveSchemaCatalogSubsystem::IsActive() const
{
    return bActive;
}

int32 USharSaveSchemaCatalogSubsystem::GetSchemaCount() const
{
    return Definitions.Num();
}

const FString& USharSaveSchemaCatalogSubsystem::GetCatalogRevision() const
{
    return CatalogRevision;
}
