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
//   - Root gameplay catalog activation and read-only runtime lookup.
// - Must-Not:
//   - Discover directories, load source formats, or own domain state.
// - Allows:
//   - Validation and activation of one immutable catalog revision.
// - Split-When:
//   - Asynchronous Asset Manager loading gains an independent lifecycle.
// - Merge-When:
//   - Another game-instance service owns the identical catalog authority.
// - Summary:
//   - Implements the root gameplay catalog runtime authority.
// - Description:
//   - Rejects malformed revisions and exposes one accepted catalog snapshot.
// - Usage:
//   - Configured by the composition root before domain coordinators activate.
// - Defaults:
//   - Starts inactive and rejects replacement in place.
//

//! Root gameplay catalog activation and lookup.

#include "Catalog/SharGameplayCatalogSubsystem.h"

#include "Catalog/SharGameplayCatalog.h"

ESharGameplayCatalogActivationResult USharGameplayCatalogSubsystem::Activate(
    USharGameplayCatalog* InCatalog
)
{
    if (ActiveCatalog != nullptr)
    {
        return ESharGameplayCatalogActivationResult::AlreadyActive;
    }
    if (InCatalog == nullptr
        || !InCatalog->RevisionToken.StartsWith(TEXT("sha256:")))
    {
        return ESharGameplayCatalogActivationResult::InvalidCatalog;
    }
    TArray<FText> Errors;
    InCatalog->GatherValidationErrors(Errors);
    if (!Errors.IsEmpty())
    {
        return ESharGameplayCatalogActivationResult::InvalidCatalog;
    }
    ActiveCatalog = InCatalog;
    return ESharGameplayCatalogActivationResult::Accepted;
}

bool USharGameplayCatalogSubsystem::IsActive() const
{
    return ActiveCatalog != nullptr;
}

FString USharGameplayCatalogSubsystem::GetCatalogRevision() const
{
    return ActiveCatalog == nullptr ? FString() : ActiveCatalog->RevisionToken;
}

bool USharGameplayCatalogSubsystem::ContainsDefinition(
    const FPrimaryAssetId& DefinitionId
) const
{
    return ActiveCatalog != nullptr
        && ActiveCatalog->ContainsDefinition(DefinitionId);
}

const USharGameplayCatalog* USharGameplayCatalogSubsystem::GetCatalog() const
{
    return ActiveCatalog;
}
