// File: SharActionSequenceDefinition.cpp
// Path: src/uproject/Source/SharAction/Private/Action/SharActionSequenceDefinition.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: load-free action-sequence validation only; no StateTree execution or scheduling.
// ADR: docs/adr/unreal/runtime/typed-state-tree-action-sequences.md
// LARGE-FILE owner=SharAction; reason=cohesive ordered sequence validation;
// split=extract parameter-binding validation if schemas expand;
// validation=validate.sh SharAction plus Unreal automation; review=2027-01.

#include "Action/SharActionSequenceDefinition.h"

#include "Content/SharPrimaryContentDefinition.h"
#include "Engine/DataAsset.h"

static void AddActionSequenceError(
    TArray<FText>& OutErrors,
    const TCHAR* Message
)
{
    OutErrors.Add(FText::FromString(Message));
}

static bool IsCanonicalSequenceId(const FName& Candidate)
{
    return USharPrimaryContentDefinition::IsCanonicalIdentifier(Candidate);
}

static bool IsActionRevision(const FString& Candidate)
{
    return Candidate.StartsWith(TEXT("sha256:"));
}

static void AppendStepErrors(
    const TArray<FSharActionSequenceStep>& Steps,
    TArray<FText>& OutErrors
)
{
    if (Steps.IsEmpty())
    {
        AddActionSequenceError(
            OutErrors,
            TEXT("Action sequence requires at least one ordered step.")
        );
        return;
    }
    TSet<FName> SeenStepIds;
    int32 ExpectedOrdinal = 0;
    for (const FSharActionSequenceStep& Step : Steps)
    {
        const bool bInvalidIdentity =
            !IsCanonicalSequenceId(Step.StepId)
            || !Step.ActionId.IsValid()
            || !IsCanonicalSequenceId(Step.ParameterBindingId)
            || !IsActionRevision(Step.ExpectedActionRevision);
        if (bInvalidIdentity)
        {
            AddActionSequenceError(
                OutErrors,
                TEXT("Action sequence steps require canonical identities, valid action assets, and SHA-256 revisions.")
            );
        }
        if (Step.Ordinal != ExpectedOrdinal)
        {
            AddActionSequenceError(
                OutErrors,
                TEXT("Action sequence step ordinals must be dense and zero-based.")
            );
        }
        if (SeenStepIds.Contains(Step.StepId))
        {
            AddActionSequenceError(
                OutErrors,
                TEXT("Action sequence step identities must be unique.")
            );
        }
        SeenStepIds.Add(Step.StepId);
        ++ExpectedOrdinal;
    }
}

static void AppendContextErrors(
    const TArray<FName>& ContextIds,
    TArray<FText>& OutErrors
)
{
    TSet<FName> SeenContextIds;
    for (const FName& ContextId : ContextIds)
    {
        if (!IsCanonicalSequenceId(ContextId))
        {
            AddActionSequenceError(
                OutErrors,
                TEXT("Action sequence context identities must be canonical.")
            );
        }
        if (SeenContextIds.Contains(ContextId))
        {
            AddActionSequenceError(
                OutErrors,
                TEXT("Action sequence context identities must be unique.")
            );
        }
        SeenContextIds.Add(ContextId);
    }
}

static void AppendSequenceTimeoutErrors(
    const USharActionSequenceDefinition& Definition,
    TArray<FText>& OutErrors
)
{
    const bool bInvalidFinite =
        !FMath::IsFinite(Definition.SequenceTimeoutSeconds);
    const bool bInvalidBound =
        Definition.SequenceTimeoutSeconds <= 0.0F
        && !Definition.bAllowsNoTimeout;
    const bool bContradictoryNoTimeout =
        Definition.bAllowsNoTimeout
        && Definition.SequenceTimeoutSeconds > 0.0F;
    if (bInvalidFinite || bInvalidBound)
    {
        AddActionSequenceError(
            OutErrors,
            TEXT("Action sequence timeout must be finite and positive unless no-timeout is explicitly allowed.")
        );
    }
    if (bContradictoryNoTimeout)
    {
        AddActionSequenceError(
            OutErrors,
            TEXT("No-timeout action sequences must use a non-positive timeout value.")
        );
    }
}

void USharActionSequenceDefinition::GatherValidationErrors(
    TArray<FText>& OutErrors
) const
{
    USharPrimaryContentDefinition::GatherValidationErrors(OutErrors);
    if (!IsCanonicalSequenceId(StateTreeTemplateId)
        || !IsCanonicalSequenceId(VerificationPolicyId))
    {
        AddActionSequenceError(
            OutErrors,
            TEXT("Action sequence template and verification identities must be canonical.")
        );
    }
    AppendStepErrors(Steps, OutErrors);
    AppendContextErrors(RequiredContextIds, OutErrors);
    AppendSequenceTimeoutErrors(*this, OutErrors);
}

FPrimaryAssetType USharActionSequenceDefinition::GetDefinitionAssetType() const
{
    return {TEXT("SharActionSequence")};
}
