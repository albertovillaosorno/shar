// File: SharProgressionCatalogSubsystem.cpp
// Path: src/uproject/Source/SharProgression/Private/Progression/SharProgressionCatalogSubsystem.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: immutable progression catalog registration and lookup only.
// Specification: docs/technical/unreal/progression-collectibles-and-cheats.md

#include "Progression/SharProgressionCatalogSubsystem.h"

#include "Algo/Find.h"
#include "Progression/SharProgressionCatalogDefinition.h"

static bool IsProgressionRevision(const FString& Revision)
{
    return Revision.StartsWith(TEXT("sha256:"));
}

bool USharProgressionCatalogSubsystem::ConfigureRevision(
    const FString& InCatalogRevision
)
{
    if (!IsProgressionRevision(InCatalogRevision))
    {
        return false;
    }
    CatalogRevision = InCatalogRevision;
    Definitions.Reset();
    bActive = false;
    return true;
}

ESharProgressionCatalogResult
USharProgressionCatalogSubsystem::RegisterCatalog(
    USharProgressionCatalogDefinition* Definition
)
{
    if (bActive)
    {
        return ESharProgressionCatalogResult::AlreadyActive;
    }
    if (CatalogRevision.IsEmpty() || Definition == nullptr)
    {
        return ESharProgressionCatalogResult::InvalidDefinition;
    }
    TArray<FText> Errors;
    Definition->GatherValidationErrors(Errors);
    if (!Errors.IsEmpty())
    {
        return ESharProgressionCatalogResult::InvalidDefinition;
    }
    if (FindCatalog(Definition->CanonicalId) != nullptr)
    {
        return ESharProgressionCatalogResult::DuplicateCatalog;
    }
    Definitions.Add(Definition);
    return ESharProgressionCatalogResult::Accepted;
}

ESharProgressionCatalogResult USharProgressionCatalogSubsystem::Activate()
{
    if (bActive)
    {
        return ESharProgressionCatalogResult::AlreadyActive;
    }
    if (Definitions.IsEmpty())
    {
        return ESharProgressionCatalogResult::EmptyCatalog;
    }
    bActive = true;
    return ESharProgressionCatalogResult::Accepted;
}

const USharProgressionCatalogDefinition*
USharProgressionCatalogSubsystem::FindCatalog(const FName& CatalogId) const
{
    const TObjectPtr<USharProgressionCatalogDefinition>* Definition =
        Algo::FindByPredicate(
            Definitions,
            [&CatalogId](
                const TObjectPtr<USharProgressionCatalogDefinition>& Candidate
            )
            {
                return Candidate != nullptr
                    && Candidate->CanonicalId == CatalogId;
            }
        );
    return Definition == nullptr ? nullptr : *Definition;
}

bool USharProgressionCatalogSubsystem::IsActive() const
{
    return bActive;
}

int32 USharProgressionCatalogSubsystem::GetCatalogCount() const
{
    return Definitions.Num();
}

const FString& USharProgressionCatalogSubsystem::GetCatalogRevision() const
{
    return CatalogRevision;
}
