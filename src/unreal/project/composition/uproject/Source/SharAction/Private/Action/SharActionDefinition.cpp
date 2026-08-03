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
//   - Shar action definition composition module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Shar action definition composition module.
// - Description:
//   - Implements the declared composition module responsibility for project.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Shar action definition composition module.

#include "Action/SharActionDefinition.h"

#include "Content/SharPrimaryContentDefinition.h"
#include "Engine/DataAsset.h"

static void AddActionDefinitionError(
    TArray<FText>& OutErrors,
    const TCHAR* Message
)
{
    OutErrors.Add(FText::FromString(Message));
}

static bool IsCanonicalActionId(const FName& Candidate)
{
    return USharPrimaryContentDefinition::IsCanonicalIdentifier(Candidate);
}

static void AppendPolicyIdentityErrors(
    const USharActionDefinition& Definition,
    TArray<FText>& OutErrors
)
{
    const bool bInvalid =
        !IsCanonicalActionId(Definition.ParameterSchemaId)
        || !IsCanonicalActionId(Definition.PreconditionsPolicyId)
        || !IsCanonicalActionId(Definition.CancellationPolicyId)
        || !IsCanonicalActionId(Definition.VerificationPolicyId)
        || !IsCanonicalActionId(Definition.PresentationPolicyId)
        || !IsCanonicalActionId(Definition.ExecutorId);
    if (bInvalid)
    {
        AddActionDefinitionError(
            OutErrors,
            // jig-ignore-next-line: exact syntax is indivisible
            TEXT("Action parameter, policy, presentation, verification, and executor identities must be canonical.")
        );
    }
}

static void AppendResourceErrors(
    const TArray<FSharActionResourceClaim>& Claims,
    TArray<FText>& OutErrors
)
{
    TSet<FName> SeenResources;
    for (const FSharActionResourceClaim& Claim : Claims)
    {
        if (!IsCanonicalActionId(Claim.ResourceId))
        {
            AddActionDefinitionError(
                OutErrors,
                TEXT("Action resource identities must be canonical.")
            );
        }
        if (SeenResources.Contains(Claim.ResourceId))
        {
            AddActionDefinitionError(
                OutErrors,
                TEXT("Action resource claims must be unique per definition.")
            );
        }
        SeenResources.Add(Claim.ResourceId);
    }
}

static void AppendTimeoutErrors(
    const USharActionDefinition& Definition,
    TArray<FText>& OutErrors
)
{
    const bool bInvalidFinite = !FMath::IsFinite(Definition.TimeoutSeconds);
    const bool bInvalidBound =
        Definition.TimeoutSeconds <= 0.0F && !Definition.bAllowsNoTimeout;
    const bool bContradictoryNoTimeout =
        Definition.bAllowsNoTimeout && Definition.TimeoutSeconds > 0.0F;
    if (bInvalidFinite || bInvalidBound)
    {
        AddActionDefinitionError(
            OutErrors,
            // jig-ignore-next-line: exact syntax is indivisible
            TEXT("Action timeout must be finite and positive unless no-timeout is explicitly allowed.")
        );
    }
    if (bContradictoryNoTimeout)
    {
        AddActionDefinitionError(
            OutErrors,
            TEXT("No-timeout actions must use a non-positive timeout value.")
        );
    }
}

void USharActionDefinition::GatherValidationErrors(
    TArray<FText>& OutErrors
) const
{
    USharPrimaryContentDefinition::GatherValidationErrors(OutErrors);
    AppendPolicyIdentityErrors(*this, OutErrors);
    AppendResourceErrors(RequiredResources, OutErrors);
    AppendTimeoutErrors(*this, OutErrors);
}

FPrimaryAssetType USharActionDefinition::GetDefinitionAssetType() const
{
    return {TEXT("SharAction")};
}
