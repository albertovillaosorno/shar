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
//   - Shar application mode definition composition module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Shar application mode definition composition module.
// - Description:
//   - Implements the declared composition module responsibility for project.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Shar application mode definition composition module.

#include "Application/SharApplicationModeDefinition.h"

#include "Algo/AllOf.h"
#include "Algo/AnyOf.h"
#include "Content/SharPrimaryContentDefinition.h"
#include "Engine/DataAsset.h"

static bool IsCanonicalModeIdentity(const FName& Candidate)
{
    return USharPrimaryContentDefinition::IsCanonicalIdentifier(Candidate);
}

static bool IsCanonicalOrNone(const FName& Candidate)
{
    return Candidate.IsNone() || IsCanonicalModeIdentity(Candidate);
}

static bool HasDuplicateModeIds(const TArray<FName>& Ids)
{
    return Algo::AnyOf(
        Ids,
        [&Ids](const FName& Candidate)
        {
            int32 MatchCount = 0;
            for (const FName& ModeId : Ids)
            {
                MatchCount += ModeId == Candidate ? 1 : 0;
            }
            return MatchCount > 1;
        }
    );
}

static bool HasInvalidModeIds(const TArray<FName>& Ids)
{
    return HasDuplicateModeIds(Ids)
        || !Algo::AllOf(
            Ids,
            [](const FName& ModeId)
            {
                return IsCanonicalModeIdentity(ModeId);
            }
        );
}

static void AddModeError(TArray<FText>& OutErrors, const TCHAR* Message)
{
    OutErrors.Add(FText::FromString(Message));
}

static void AppendIdentityAndGraphErrors(
    const USharApplicationModeDefinition& Definition,
    TArray<FText>& OutErrors
)
{
    const bool bInvalidPlans =
        !IsCanonicalModeIdentity(Definition.EntryPlanId)
        || !IsCanonicalModeIdentity(Definition.ExitPlanId)
        || !IsCanonicalModeIdentity(Definition.ReadinessBarrierId);
    const bool bInvalidGraph =
        HasInvalidModeIds(Definition.AllowedPredecessorIds)
        || HasInvalidModeIds(Definition.AllowedSuccessorIds)
        || HasInvalidModeIds(Definition.RequiredServiceIds);
    if (bInvalidPlans || bInvalidGraph)
    {
        AddModeError(
            OutErrors,
            // jig-ignore-next-line: exact syntax is indivisible
            TEXT("Application mode plans, graph edges, services, and readiness barrier must use unique canonical identities.")
        );
    }
    const bool bSelfEdge =
        Definition.AllowedPredecessorIds.ContainsByPredicate(
            [&Definition](const FName& ModeId)
            {
                return ModeId == Definition.CanonicalId;
            }
        )
        || Definition.AllowedSuccessorIds.ContainsByPredicate(
            [&Definition](const FName& ModeId)
            {
                return ModeId == Definition.CanonicalId;
            }
        );
    if (bSelfEdge)
    {
        // jig-ignore-next-line: exact syntax is indivisible
        AddModeError(OutErrors, TEXT("Application mode cannot reference itself as a graph edge."));
    }
}

static void AppendKindErrors(
    const USharApplicationModeDefinition& Definition,
    TArray<FText>& OutErrors
)
{
    if (Definition.ModeKind == ESharApplicationModeKind::Entry
        && !Definition.AllowedPredecessorIds.IsEmpty())
    {
        // jig-ignore-next-line: exact syntax is indivisible
        AddModeError(OutErrors, TEXT("Entry mode cannot declare predecessors."));
    }
    if (Definition.ModeKind == ESharApplicationModeKind::Exit
        && !Definition.AllowedSuccessorIds.IsEmpty())
    {
        AddModeError(OutErrors, TEXT("Exit mode cannot declare successors."));
    }
    if (Definition.ModeKind == ESharApplicationModeKind::Loading)
    {
        const bool bInvalidLoading =
            !IsCanonicalModeIdentity(Definition.SuccessModeId)
            || !IsCanonicalModeIdentity(Definition.RecoveryModeId)
            || !Definition.bSupportsCancellation
            || !Definition.bHasBoundedTimeout;
        if (bInvalidLoading)
        {
            AddModeError(
                OutErrors,
                // jig-ignore-next-line: exact syntax is indivisible
                TEXT("Loading mode requires canonical success and recovery targets plus cancellation and bounded timeout.")
            );
        }
    }
    if (Definition.ModeKind == ESharApplicationModeKind::Overlay
        && !IsCanonicalModeIdentity(Definition.ReturnModeId))
    {
        // jig-ignore-next-line: exact syntax is indivisible
        AddModeError(OutErrors, TEXT("Overlay mode requires a canonical return mode."));
    }
}

static void AppendOwnershipErrors(
    const USharApplicationModeDefinition& Definition,
    TArray<FText>& OutErrors
)
{
    const bool bInvalidRecovery =
        !IsCanonicalOrNone(Definition.SuccessModeId)
        || !IsCanonicalOrNone(Definition.RecoveryModeId)
        || !IsCanonicalOrNone(Definition.ReturnModeId);
    if (bInvalidRecovery)
    {
        // jig-ignore-next-line: exact syntax is indivisible
        AddModeError(OutErrors, TEXT("Optional application mode targets must be canonical when present."));
    }
    if (Definition.WorldPolicy == ESharApplicationWorldPolicy::Retain
        && Definition.RecoveryModeId.IsNone())
    {
        // jig-ignore-next-line: exact syntax is indivisible
        AddModeError(OutErrors, TEXT("World-retaining mode requires an explicit recovery target."));
    }
    if (Definition.bDemonstrationMode
        && Definition.ProgressionPolicy
            == ESharApplicationProgressionPolicy::Durable)
    {
        // jig-ignore-next-line: exact syntax is indivisible
        AddModeError(OutErrors, TEXT("Demonstration mode cannot own durable progression."));
    }
}

void USharApplicationModeDefinition::GatherValidationErrors(
    TArray<FText>& OutErrors
) const
{
    USharPrimaryContentDefinition::GatherValidationErrors(OutErrors);
    AppendIdentityAndGraphErrors(*this, OutErrors);
    AppendKindErrors(*this, OutErrors);
    AppendOwnershipErrors(*this, OutErrors);
}

FPrimaryAssetType USharApplicationModeDefinition::GetDefinitionAssetType() const
{
    return {TEXT("SharApplicationMode")};
}
