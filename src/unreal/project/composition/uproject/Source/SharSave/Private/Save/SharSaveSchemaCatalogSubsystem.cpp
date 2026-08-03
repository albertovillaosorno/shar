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
//   - Shar save schema catalog subsystem composition module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Shar save schema catalog subsystem composition module.
// - Description:
//   - Implements the declared composition module responsibility for project.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Shar save schema catalog subsystem composition module.

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
                // jig-ignore-next-line: exact syntax is indivisible
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
