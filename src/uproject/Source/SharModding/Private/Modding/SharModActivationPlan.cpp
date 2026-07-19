// File: SharModActivationPlan.cpp
// Path: src/uproject/Source/SharModding/Private/Modding/SharModActivationPlan.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: deterministic mod dependency, conflict, cycle, and replacement planning; no package activation.
// ADR: docs/adr/unreal/architecture/aaa-native-content-and-gameplay-foundation.md

#include "Modding/SharModActivationPlan.h"

#include "Algo/AllOf.h"
#include "Algo/Find.h"
#include "Modding/SharModDescriptor.h"

static void AddPlanError(
    FSharModActivationPlan& Plan,
    const TCHAR* Message
)
{
    Plan.Errors.Add(FText::FromString(Message));
}

static const USharModDescriptor* FindDescriptor(
    const TArray<const USharModDescriptor*>& Descriptors,
    const FName& NamespaceId
)
{
    const USharModDescriptor* const* Match = Algo::FindByPredicate(
        Descriptors,
        [&NamespaceId](const USharModDescriptor* Descriptor)
        {
            return Descriptor != nullptr
                && Descriptor->NamespaceId == NamespaceId;
        }
    );
    return Match == nullptr ? nullptr : *Match;
}

static bool DependenciesArePlanned(
    const USharModDescriptor& Descriptor,
    const TSet<FName>& PlannedNamespaces
)
{
    return Algo::AllOf(
        Descriptor.RequiredModNamespaces,
        [&PlannedNamespaces](const FName& Dependency)
        {
            return PlannedNamespaces.Contains(Dependency);
        }
    );
}

static void AppendDescriptorValidationErrors(
    const USharModDescriptor& Descriptor,
    FSharModActivationPlan& Plan
)
{
    TArray<FText> DescriptorErrors;
    Descriptor.GatherValidationErrors(DescriptorErrors);
    for (const FText& Error : DescriptorErrors)
    {
        Plan.Errors.Add(Error);
    }
}

static void AppendNamespaceErrors(
    const USharModDescriptor& Descriptor,
    TSet<FName>& SeenNamespaces,
    FSharModActivationPlan& Plan
)
{
    if (SeenNamespaces.Contains(Descriptor.NamespaceId))
    {
        AddPlanError(
            Plan,
            TEXT("Mod activation input contains a duplicate namespace.")
        );
    }
    SeenNamespaces.Add(Descriptor.NamespaceId);
}

static void AppendDependencyErrors(
    const USharModDescriptor& Descriptor,
    const TArray<const USharModDescriptor*>& Descriptors,
    FSharModActivationPlan& Plan
)
{
    for (const FName& Dependency : Descriptor.RequiredModNamespaces)
    {
        if (FindDescriptor(Descriptors, Dependency) == nullptr)
        {
            AddPlanError(Plan, TEXT("A required mod dependency is missing."));
        }
    }
}

static void AppendDeclaredConflictErrors(
    const USharModDescriptor& Descriptor,
    const TArray<const USharModDescriptor*>& Descriptors,
    FSharModActivationPlan& Plan
)
{
    for (const FName& Conflict : Descriptor.ConflictingModNamespaces)
    {
        if (FindDescriptor(Descriptors, Conflict) != nullptr)
        {
            AddPlanError(
                Plan,
                TEXT("Two requested mods declare an activation conflict.")
            );
        }
    }
}

static void AppendDescriptorSetErrors(
    const TArray<const USharModDescriptor*>& Descriptors,
    FSharModActivationPlan& Plan
)
{
    TSet<FName> SeenNamespaces;
    for (const USharModDescriptor* Descriptor : Descriptors)
    {
        if (Descriptor == nullptr)
        {
            AddPlanError(
                Plan,
                TEXT("Mod activation input cannot contain null descriptors.")
            );
            continue;
        }
        AppendDescriptorValidationErrors(*Descriptor, Plan);
        AppendNamespaceErrors(*Descriptor, SeenNamespaces, Plan);
        AppendDependencyErrors(*Descriptor, Descriptors, Plan);
        AppendDeclaredConflictErrors(*Descriptor, Descriptors, Plan);
    }
}

static bool ReplacementsConflict(
    const USharModDescriptor& Left,
    const USharModDescriptor& Right
)
{
    for (const FSharModReplacementDefinition& LeftReplacement
        : Left.Replacements)
    {
        for (const FSharModReplacementDefinition& RightReplacement
            : Right.Replacements)
        {
            const bool bSameTarget =
                LeftReplacement.TargetAssetId
                == RightReplacement.TargetAssetId;
            const bool bSameScope =
                LeftReplacement.ScopeId == RightReplacement.ScopeId;
            if (bSameTarget && bSameScope)
            {
                return true;
            }
        }
    }
    return false;
}

static bool IsOrderedPair(
    const USharModDescriptor& Left,
    const USharModDescriptor& Right
)
{
    return Left.NamespaceId.LexicalLess(Right.NamespaceId);
}

static void AppendReplacementConflictErrorsForLeft(
    const USharModDescriptor& Left,
    const TArray<const USharModDescriptor*>& Descriptors,
    FSharModActivationPlan& Plan
)
{
    for (const USharModDescriptor* Right : Descriptors)
    {
        if (Right == nullptr || !IsOrderedPair(Left, *Right))
        {
            continue;
        }
        if (ReplacementsConflict(Left, *Right))
        {
            AddPlanError(
                Plan,
                TEXT("Two mods replace one exclusive asset scope.")
            );
        }
    }
}

static void AppendReplacementConflictErrors(
    const TArray<const USharModDescriptor*>& Descriptors,
    FSharModActivationPlan& Plan
)
{
    for (const USharModDescriptor* Left : Descriptors)
    {
        if (Left != nullptr)
        {
            AppendReplacementConflictErrorsForLeft(
                *Left,
                Descriptors,
                Plan
            );
        }
    }
}

static bool TryAppendReadyDescriptor(
    const USharModDescriptor& Descriptor,
    TSet<FName>& PlannedNamespaces,
    FSharModActivationPlan& Plan
)
{
    if (PlannedNamespaces.Contains(Descriptor.NamespaceId))
    {
        return false;
    }
    if (!DependenciesArePlanned(Descriptor, PlannedNamespaces))
    {
        return false;
    }
    PlannedNamespaces.Add(Descriptor.NamespaceId);
    Plan.OrderedDescriptors.Add(&Descriptor);
    return true;
}

static bool AppendReadyDescriptors(
    const TArray<const USharModDescriptor*>& Descriptors,
    TSet<FName>& PlannedNamespaces,
    FSharModActivationPlan& Plan
)
{
    bool bMadeProgress = false;
    for (const USharModDescriptor* Descriptor : Descriptors)
    {
        if (Descriptor == nullptr)
        {
            continue;
        }
        bMadeProgress = TryAppendReadyDescriptor(
            *Descriptor,
            PlannedNamespaces,
            Plan
        ) || bMadeProgress;
    }
    return bMadeProgress;
}

FSharModActivationPlan FSharModActivationPlanner::Build(
    const TArray<const USharModDescriptor*>& Descriptors
)
{
    FSharModActivationPlan Plan;
    AppendDescriptorSetErrors(Descriptors, Plan);
    AppendReplacementConflictErrors(Descriptors, Plan);
    if (!Plan.Errors.IsEmpty())
    {
        return Plan;
    }

    TSet<FName> PlannedNamespaces;
    while (Plan.OrderedDescriptors.Num() < Descriptors.Num())
    {
        if (!AppendReadyDescriptors(Descriptors, PlannedNamespaces, Plan))
        {
            AddPlanError(Plan, TEXT("Mod dependency graph contains a cycle."));
            Plan.OrderedDescriptors.Reset();
            return Plan;
        }
    }
    Plan.bCanActivate = true;
    return Plan;
}
