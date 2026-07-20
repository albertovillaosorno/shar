// File: SharApplicationModeCatalogSubsystem.cpp
// Path: src/uproject/Source/SharApplication/Private/Application/SharApplicationModeCatalogSubsystem.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: immutable catalog registration, graph proof, and lookup only; transition execution remains external.
// Specification: docs/technical/unreal/application-lifecycle-and-mode-runtime.md
// LARGE-FILE owner=SharApplication; reason=cohesive graph registration and activation implementation;
// split=extract graph diagnostics if validation evidence becomes persistent;
// validation=validate.sh SharApplication plus Unreal automation; review=2027-01.

#include "Application/SharApplicationModeCatalogSubsystem.h"

#include "Algo/AllOf.h"
#include "Algo/Find.h"
#include "Application/SharApplicationModeDefinition.h"

static bool IsRevisionToken(const FString& Revision)
{
    return Revision.StartsWith(TEXT("sha256:"));
}

bool USharApplicationModeCatalogSubsystem::ConfigureRevision(
    const FString& InCatalogRevision
)
{
    if (!IsRevisionToken(InCatalogRevision))
    {
        return false;
    }
    CatalogRevision = InCatalogRevision;
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
    if (CatalogRevision.IsEmpty() || Definition == nullptr)
    {
        return ESharApplicationCatalogResult::InvalidDefinition;
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
    return IsEveryModeReachableFrom(Entry->CanonicalId)
        ? ESharApplicationCatalogResult::Accepted
        : ESharApplicationCatalogResult::UnreachableMode;
}

ESharApplicationCatalogResult USharApplicationModeCatalogSubsystem::Activate()
{
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
