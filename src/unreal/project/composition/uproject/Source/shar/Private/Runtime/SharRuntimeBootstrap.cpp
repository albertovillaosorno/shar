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
//   - Cross-module runtime composition after root content is available.
// - Must-Not:
//   - Own application-mode state, load source formats, or invent content.
// - Allows:
//   - Wiring validated catalog and coordinator collaborators exactly once.
// - Split-When:
//   - Another domain requires an independently testable bootstrap transaction.
// - Merge-When:
//   - Another composition object owns the identical collaborator wiring.
// - Summary:
//   - Implements application lifecycle composition from the root catalog.
// - Description:
//   - Requires the complete declared mode family before coordinator setup.
// - Usage:
//   - Invoked after Asset Manager has supplied the accepted root definitions.
// - Defaults:
//   - Rejects missing, partial, or duplicate configuration.
//

//! Application lifecycle composition from the root gameplay catalog.

#include "Runtime/SharRuntimeBootstrap.h"

#include "Application/SharApplicationModeCatalogSubsystem.h"
#include "Application/SharApplicationModeCoordinator.h"
#include "Catalog/SharGameplayCatalog.h"
#include "Catalog/SharGameplayCatalogSubsystem.h"

namespace
{
const FSharGameplayCatalogFamily* FindApplicationModeFamily(
    const USharGameplayCatalogSubsystem& RootCatalog
)
{
    const USharGameplayCatalog* Definition = RootCatalog.GetCatalog();
    return Definition == nullptr
        ? nullptr
        : Definition->FindFamilyByType(
            FPrimaryAssetType(TEXT("SharApplicationMode"))
        );
}

bool IsCompleteModeSet(
    const FSharGameplayCatalogFamily& Family,
    const TArray<USharApplicationModeDefinition*>& Modes
)
{
    if (Family.DefinitionIds.Num() != Modes.Num())
    {
        return false;
    }
    TSet<FPrimaryAssetId> SeenModes;
    for (const USharApplicationModeDefinition* Mode : Modes)
    {
        if (Mode == nullptr || !Family.DefinitionIds.Contains(
            Mode->GetPrimaryAssetId()
        ))
        {
            return false;
        }
        TArray<FText> Errors;
        Mode->GatherValidationErrors(Errors);
        if (!Errors.IsEmpty())
        {
            return false;
        }
        SeenModes.Add(Mode->GetPrimaryAssetId());
    }
    return SeenModes.Num() == Modes.Num();
}

const USharApplicationModeDefinition* FindEntryMode(
    const TArray<USharApplicationModeDefinition*>& Modes
)
{
    USharApplicationModeDefinition* const* Entry = Modes.FindByPredicate(
        [](const USharApplicationModeDefinition* Mode)
        {
            return Mode != nullptr
                && Mode->ModeKind == ESharApplicationModeKind::Entry;
        }
    );
    return Entry == nullptr ? nullptr : *Entry;
}

FSharApplicationModeObservation MakeInitialEntryObservation(
    const USharApplicationModeDefinition& Entry
)
{
    FSharApplicationModeObservation Observation;
    Observation.ActiveModeId = Entry.CanonicalId;
    Observation.ActiveModeRevision = Entry.RevisionToken;
    return Observation;
}
} // namespace

ESharRuntimeBootstrapResult USharRuntimeBootstrap::ConfigureApplicationRuntime(
    USharGameplayCatalogSubsystem* RootCatalog,
    USharApplicationModeCatalogSubsystem* ApplicationCatalog,
    USharApplicationModeCoordinator* Coordinator,
    const TArray<USharApplicationModeDefinition*>& Modes
)
{
    if (bApplicationRuntimeConfigured)
    {
        return ESharRuntimeBootstrapResult::AlreadyConfigured;
    }
    if (RootCatalog == nullptr || !RootCatalog->IsActive())
    {
        return ESharRuntimeBootstrapResult::InvalidRootCatalog;
    }
    const FSharGameplayCatalogFamily* ModeFamily =
        FindApplicationModeFamily(*RootCatalog);
    if (ModeFamily == nullptr || !IsCompleteModeSet(*ModeFamily, Modes))
    {
        return ESharRuntimeBootstrapResult::IncompleteApplicationModes;
    }
    const USharApplicationModeDefinition* EntryMode = FindEntryMode(Modes);
    if (Coordinator == nullptr || EntryMode == nullptr)
    {
        return ESharRuntimeBootstrapResult::CoordinatorRejected;
    }
    const FSharApplicationModeObservation InitialObservation =
        MakeInitialEntryObservation(*EntryMode);
    if (!USharApplicationModeCoordinator::IsValidInitialObservation(
            InitialObservation,
            *EntryMode
        ))
    {
        return ESharRuntimeBootstrapResult::CoordinatorRejected;
    }
    if (ApplicationCatalog == nullptr
        || !ApplicationCatalog->ConfigureRootCatalog(RootCatalog))
    {
        return ESharRuntimeBootstrapResult::ApplicationCatalogRejected;
    }
    for (USharApplicationModeDefinition* Mode : Modes)
    {
        if (ApplicationCatalog->RegisterMode(Mode)
            != ESharApplicationCatalogResult::Accepted)
        {
            return ESharRuntimeBootstrapResult::ApplicationCatalogRejected;
        }
    }
    if (ApplicationCatalog->Activate()
        != ESharApplicationCatalogResult::Accepted)
    {
        return ESharRuntimeBootstrapResult::ApplicationCatalogRejected;
    }
    if (!Coordinator->Configure(ApplicationCatalog, InitialObservation))
    {
        return ESharRuntimeBootstrapResult::CoordinatorRejected;
    }
    bApplicationRuntimeConfigured = true;
    return ESharRuntimeBootstrapResult::Accepted;
}

bool USharRuntimeBootstrap::IsApplicationRuntimeConfigured() const
{
    return bApplicationRuntimeConfigured;
}
