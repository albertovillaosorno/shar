// Copyright:
//   - Copyright © 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT
//
// Boundary-Contract:
// - Owns:
//   - Shar progression catalog subsystem composition module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Shar progression catalog subsystem composition module.
// - Description:
//   - Implements the declared composition module responsibility for project.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Shar progression catalog subsystem composition module.

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
