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
//   - Shar application mode catalog subsystem composition module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Shar application mode catalog subsystem composition module.
// - Description:
//   - Implements the declared composition module responsibility for project.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Shar application mode catalog subsystem composition module.

#include "Application/SharApplicationModeCatalogSubsystem.h"

#include "Algo/AllOf.h"
#include "Algo/Find.h"
#include "Application/SharApplicationModeDefinition.h"
#include "Catalog/SharGameplayCatalog.h"
#include "Catalog/SharGameplayCatalogSubsystem.h"

bool USharApplicationModeCatalogSubsystem::ConfigureRootCatalog(
    USharGameplayCatalogSubsystem* InRootCatalog
)
{
    if (InRootCatalog == nullptr || !InRootCatalog->IsActive())
    {
        return false;
    }
    const USharGameplayCatalog* GameplayCatalog =
        InRootCatalog->GetCatalog();
    if (GameplayCatalog == nullptr
        || GameplayCatalog->FindFamilyByType(
            FPrimaryAssetType(TEXT("SharApplicationMode"))
        ) == nullptr)
    {
        return false;
    }
    RootCatalog = InRootCatalog;
    CatalogRevision = InRootCatalog->GetCatalogRevision();
    Definitions.Reset();
    bActive = false;
    return true;
}

const USharApplicationModeDefinition*
USharApplicationModeCatalogSubsystem::FindMode(const FName& ModeId) const
{
    const TObjectPtr<USharApplicationModeDefinition>* Definition =
        Algo::FindByPredicate(
            Definitions,
            // jig-ignore-next-line: exact syntax is indivisible
            [&ModeId](const TObjectPtr<USharApplicationModeDefinition>& Candidate)
            {
                return Candidate != nullptr && Candidate->CanonicalId == ModeId;
            }
        );
    return Definition == nullptr ? nullptr : *Definition;
}

const USharApplicationModeDefinition*
USharApplicationModeCatalogSubsystem::FindModeByKind(
    const ESharApplicationModeKind ModeKind
) const
{
    const TObjectPtr<USharApplicationModeDefinition>* Definition =
        Algo::FindByPredicate(
            Definitions,
            // jig-ignore-next-line: exact syntax is indivisible
            [ModeKind](const TObjectPtr<USharApplicationModeDefinition>& Candidate)
            {
                return Candidate != nullptr && Candidate->ModeKind == ModeKind;
            }
        );
    return Definition == nullptr ? nullptr : *Definition;
}

ESharApplicationCatalogResult
USharApplicationModeCatalogSubsystem::RegisterMode(
    USharApplicationModeDefinition* Definition
)
{
    if (bActive)
    {
        return ESharApplicationCatalogResult::AlreadyActive;
    }
    if (RootCatalog == nullptr || !RootCatalog->IsActive()
        || CatalogRevision != RootCatalog->GetCatalogRevision()
        || Definition == nullptr)
    {
        return ESharApplicationCatalogResult::InvalidDefinition;
    }
    if (!RootCatalog->ContainsDefinition(Definition->GetPrimaryAssetId()))
    {
        return ESharApplicationCatalogResult::DefinitionNotCatalogued;
    }
    TArray<FText> Errors;
    Definition->GatherValidationErrors(Errors);
    if (!Errors.IsEmpty())
    {
        return ESharApplicationCatalogResult::InvalidDefinition;
    }
    if (FindMode(Definition->CanonicalId) != nullptr)
    {
        return ESharApplicationCatalogResult::DuplicateMode;
    }
    Definitions.Add(Definition);
    return ESharApplicationCatalogResult::Accepted;
}

bool USharApplicationModeCatalogSubsystem::AreEdgesResolvable() const
{
    return Algo::AllOf(
        Definitions,
        [this](const TObjectPtr<USharApplicationModeDefinition>& Definition)
        {
            if (Definition == nullptr)
            {
                return false;
            }
            const bool bPredecessorsResolve = Algo::AllOf(
                Definition->AllowedPredecessorIds,
                [this](const FName& ModeId)
                {
                    return FindMode(ModeId) != nullptr;
                }
            );
            const bool bSuccessorsResolve = Algo::AllOf(
                Definition->AllowedSuccessorIds,
                [this](const FName& ModeId)
                {
                    return FindMode(ModeId) != nullptr;
                }
            );
            return bPredecessorsResolve && bSuccessorsResolve;
        }
    );
}

bool USharApplicationModeCatalogSubsystem::AreEdgesReciprocal() const
{
    return Algo::AllOf(
        Definitions,
        [this](const TObjectPtr<USharApplicationModeDefinition>& Definition)
        {
            if (Definition == nullptr)
            {
                return false;
            }
            return Algo::AllOf(
                Definition->AllowedSuccessorIds,
                [this, &Definition](const FName& SuccessorId)
                {
                    const USharApplicationModeDefinition* Successor =
                        FindMode(SuccessorId);
                    return Successor != nullptr
                        && Successor->AllowedPredecessorIds.ContainsByPredicate(
                            [&Definition](const FName& PredecessorId)
                            {
                                return PredecessorId == Definition->CanonicalId;
                            }
                        );
                }
            );
        }
    );
}

bool USharApplicationModeCatalogSubsystem::AreLoadingTargetsResolvable() const
{
    return Algo::AllOf(
        Definitions,
        [this](const TObjectPtr<USharApplicationModeDefinition>& Definition)
        {
            return Definition != nullptr
                && (Definition->ModeKind != ESharApplicationModeKind::Loading
                    || (FindMode(Definition->SuccessModeId) != nullptr
                        && FindMode(Definition->RecoveryModeId) != nullptr));
        }
    );
}

bool USharApplicationModeCatalogSubsystem::AreRecoveryTargetsResolvable() const
{
    return Algo::AllOf(
        Definitions,
        [this](const TObjectPtr<USharApplicationModeDefinition>& Definition)
        {
            return Definition != nullptr
                && (Definition->RecoveryModeId.IsNone()
                    || FindMode(Definition->RecoveryModeId) != nullptr);
        }
    );
}

bool USharApplicationModeCatalogSubsystem::AreOverlayReturnsResolvable() const
{
    return Algo::AllOf(
        Definitions,
        [this](const TObjectPtr<USharApplicationModeDefinition>& Definition)
        {
            if (Definition == nullptr)
            {
                return false;
            }
            if (Definition->ModeKind != ESharApplicationModeKind::Overlay)
            {
                return true;
            }
            const FName ReturnModeId = Definition->ReturnModeId;
            return FindMode(ReturnModeId) != nullptr
                && Definition->AllowedPredecessorIds.Contains(ReturnModeId)
                && Definition->AllowedSuccessorIds.Contains(ReturnModeId);
        }
    );
}

static bool RequiresWorldAuthority(
    const ESharApplicationWorldPolicy Policy
)
{
    return Policy == ESharApplicationWorldPolicy::Prepare
        || Policy == ESharApplicationWorldPolicy::Retain
        || Policy == ESharApplicationWorldPolicy::Own;
}

static bool RequiresSessionAuthority(
    const ESharApplicationSessionPolicy Policy
)
{
    return Policy == ESharApplicationSessionPolicy::Prepare
        || Policy == ESharApplicationSessionPolicy::Retain
        || Policy == ESharApplicationSessionPolicy::Own;
}

bool
USharApplicationModeCatalogSubsystem::AreAuthorityPoliciesCompatible() const
{
    return Algo::AllOf(
        Definitions,
        [this](const TObjectPtr<USharApplicationModeDefinition>& Definition)
        {
            if (Definition == nullptr)
            {
                return false;
            }
            return Algo::AllOf(
                Definition->AllowedPredecessorIds,
                [this, &Definition](const FName& PredecessorId)
                {
                    const USharApplicationModeDefinition* Predecessor =
                        FindMode(PredecessorId);
                    if (Predecessor == nullptr)
                    {
                        return false;
                    }
                    const bool bTargetKeepsWorld =
                        Definition->WorldPolicy
                            == ESharApplicationWorldPolicy::Retain
                        || Definition->WorldPolicy
                            == ESharApplicationWorldPolicy::Own;
                    const bool bWorldCompatible = !bTargetKeepsWorld
                        || RequiresWorldAuthority(Predecessor->WorldPolicy);
                    const bool bTargetKeepsSession =
                        Definition->SessionPolicy
                            == ESharApplicationSessionPolicy::Retain
                        || Definition->SessionPolicy
                            == ESharApplicationSessionPolicy::Own;
                    const bool bSessionCompatible = !bTargetKeepsSession
                        || RequiresSessionAuthority(Predecessor->SessionPolicy);
                    return bWorldCompatible && bSessionCompatible;
                }
            );
        }
    );
}

static bool TryReachApplicationMode(
    const USharApplicationModeDefinition* Definition,
    TSet<FName>& ReachedModeIds
)
{
    if (Definition == nullptr
        || ReachedModeIds.Contains(Definition->CanonicalId))
    {
        return false;
    }
    const bool bHasReachedPredecessor =
        Definition->AllowedPredecessorIds.ContainsByPredicate(
            [&ReachedModeIds](const FName& PredecessorId)
            {
                return ReachedModeIds.Contains(PredecessorId);
            }
        );
    if (!bHasReachedPredecessor)
    {
        return false;
    }
    ReachedModeIds.Add(Definition->CanonicalId);
    return true;
}

bool USharApplicationModeCatalogSubsystem::IsEveryModeReachableFrom(
    const FName& EntryModeId
) const
{
    TSet<FName> ReachedModeIds;
    ReachedModeIds.Add(EntryModeId);
    int32 ReachedCount = 1;
    bool bMadeProgress = true;
    while (ReachedCount < Definitions.Num() && bMadeProgress)
    {
        bMadeProgress = false;
        for (const TObjectPtr<USharApplicationModeDefinition>& Definition :
             Definitions)
        {
            if (TryReachApplicationMode(Definition, ReachedModeIds))
            {
                ++ReachedCount;
                bMadeProgress = true;
            }
        }
    }
    return ReachedCount == Definitions.Num();
}

ESharApplicationCatalogResult
USharApplicationModeCatalogSubsystem::ValidateGraph() const
{
    const USharApplicationModeDefinition* Entry =
        FindModeByKind(ESharApplicationModeKind::Entry);
    if (Entry == nullptr)
    {
        return ESharApplicationCatalogResult::EntryMissing;
    }
    if (FindModeByKind(ESharApplicationModeKind::Exit) == nullptr)
    {
        return ESharApplicationCatalogResult::ExitMissing;
    }
    if (!AreEdgesResolvable())
    {
        return ESharApplicationCatalogResult::EdgeMissing;
    }
    if (!AreEdgesReciprocal())
    {
        return ESharApplicationCatalogResult::EdgeNotReciprocal;
    }
    if (!AreLoadingTargetsResolvable())
    {
        return ESharApplicationCatalogResult::LoadingTargetMissing;
    }
    if (!AreRecoveryTargetsResolvable())
    {
        return ESharApplicationCatalogResult::RecoveryTargetMissing;
    }
    if (!AreOverlayReturnsResolvable())
    {
        return ESharApplicationCatalogResult::OverlayReturnMissing;
    }
    if (!AreAuthorityPoliciesCompatible())
    {
        return ESharApplicationCatalogResult::AuthorityPolicyMismatch;
    }
    return IsEveryModeReachableFrom(Entry->CanonicalId)
        ? ESharApplicationCatalogResult::Accepted
        : ESharApplicationCatalogResult::UnreachableMode;
}

ESharApplicationCatalogResult USharApplicationModeCatalogSubsystem::Activate()
{
    if (RootCatalog == nullptr || !RootCatalog->IsActive()
        || CatalogRevision != RootCatalog->GetCatalogRevision())
    {
        return ESharApplicationCatalogResult::InvalidRevision;
    }
    if (bActive)
    {
        return ESharApplicationCatalogResult::AlreadyActive;
    }
    const ESharApplicationCatalogResult ValidationResult = ValidateGraph();
    if (ValidationResult != ESharApplicationCatalogResult::Accepted)
    {
        return ValidationResult;
    }
    bActive = true;
    return ESharApplicationCatalogResult::Accepted;
}

bool USharApplicationModeCatalogSubsystem::IsTransitionAllowed(
    const FName& SourceModeId,
    const FName& TargetModeId
) const
{
    if (!bActive)
    {
        return false;
    }
    const USharApplicationModeDefinition* Source = FindMode(SourceModeId);
    const USharApplicationModeDefinition* Target = FindMode(TargetModeId);
    return Source != nullptr
        && Target != nullptr
        && Source->AllowedSuccessorIds.ContainsByPredicate(
            [&TargetModeId](const FName& SuccessorId)
            {
                return SuccessorId == TargetModeId;
            }
        );
}

int32 USharApplicationModeCatalogSubsystem::GetModeCount() const
{
    return Definitions.Num();
}

bool USharApplicationModeCatalogSubsystem::IsActive() const
{
    return bActive;
}

const FString& USharApplicationModeCatalogSubsystem::GetCatalogRevision() const
{
    return CatalogRevision;
}
