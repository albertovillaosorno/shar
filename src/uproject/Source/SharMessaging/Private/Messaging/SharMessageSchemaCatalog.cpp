// File: SharMessageSchemaCatalog.cpp
// Path: src/uproject/Source/SharMessaging/Private/Messaging/SharMessageSchemaCatalog.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: schema metadata validation and registration only; no publication or payload ownership.
// Specification: docs/technical/unreal/typed-event-and-observation-routing-runtime.md
// LARGE-FILE owner=SharMessaging; reason=cohesive semantic-channel and schema-policy validation;
// split=extract alias validation if aliases become runtime-visible;
// validation=validate.sh SharMessaging plus Unreal automation; review=2027-01.

#include "Messaging/SharMessageSchemaCatalog.h"

#include "Algo/Find.h"
#include "Content/SharPrimaryContentDefinition.h"

static bool IsCanonicalSchemaIdentity(const FName& Candidate)
{
    return USharPrimaryContentDefinition::IsCanonicalIdentifier(Candidate);
}

static bool IsSchemaRevision(const FString& Candidate)
{
    return Candidate.StartsWith(TEXT("sha256:"));
}

static bool IsSemanticChannelCharacter(const TCHAR Character)
{
    const bool bLowercase = Character >= 'a' && Character <= 'z';
    const bool bDigit = Character >= '0' && Character <= '9';
    return bLowercase || bDigit || Character == '_' || Character == '.';
}

namespace
{
struct FSemanticChannelState
{
    bool bSegmentHasValue = false;
    bool bPreviousWasUnderscore = false;
};
} // namespace

static bool AdvanceSemanticChannelState(
    const TCHAR Character,
    FSemanticChannelState& State
)
{
    if (!IsSemanticChannelCharacter(Character))
    {
        return false;
    }
    if (Character == '.')
    {
        const bool bValidBoundary =
            State.bSegmentHasValue && !State.bPreviousWasUnderscore;
        State.bSegmentHasValue = false;
        State.bPreviousWasUnderscore = false;
        return bValidBoundary;
    }
    if (Character == '_')
    {
        const bool bValidUnderscore =
            State.bSegmentHasValue && !State.bPreviousWasUnderscore;
        State.bPreviousWasUnderscore = true;
        return bValidUnderscore;
    }
    State.bSegmentHasValue = true;
    State.bPreviousWasUnderscore = false;
    return true;
}

bool USharMessageSchemaCatalog::IsSemanticChannel(const FName& ChannelId)
{
    if (ChannelId.IsNone())
    {
        return false;
    }
    FSemanticChannelState State;
    for (const TCHAR Character : ChannelId.ToString())
    {
        if (!AdvanceSemanticChannelState(Character, State))
        {
            return false;
        }
    }
    return State.bSegmentHasValue && !State.bPreviousWasUnderscore;
}

const FSharMessageSchemaDefinition* USharMessageSchemaCatalog::FindSchema(
    const FName& SchemaId
) const
{
    return Algo::FindByPredicate(
        Schemas,
        [&SchemaId](const FSharMessageSchemaDefinition& Definition)
        {
            return Definition.SchemaId == SchemaId;
        }
    );
}

const FSharMessageSchemaDefinition*
USharMessageSchemaCatalog::FindSchemaByChannel(const FName& ChannelId) const
{
    return Algo::FindByPredicate(
        Schemas,
        [&ChannelId](const FSharMessageSchemaDefinition& Definition)
        {
            return Definition.ChannelId == ChannelId;
        }
    );
}

bool USharMessageSchemaCatalog::RegisterSchema(
    const FSharMessageSchemaDefinition& Definition
)
{
    const bool bInvalidIdentity =
        !IsCanonicalSchemaIdentity(Definition.SchemaId)
        || !IsSemanticChannel(Definition.ChannelId)
        || !IsSchemaRevision(Definition.SchemaRevision)
        || !IsCanonicalSchemaIdentity(Definition.OwningModuleId)
        || !IsCanonicalSchemaIdentity(Definition.PublisherFamilyId);
    const bool bInvalidBounds =
        Definition.MaximumCanonicalIdentities <= 0
        || Definition.MaximumRecursionDepth <= 0;
    const bool bInvalidLifetime =
        Definition.bDurable && Definition.bAllowsTransientObjectReferences;
    if (bInvalidIdentity || bInvalidBounds || bInvalidLifetime)
    {
        return false;
    }
    const bool bDuplicate = Schemas.ContainsByPredicate(
        [&Definition](const FSharMessageSchemaDefinition& Existing)
        {
            return Existing.SchemaId == Definition.SchemaId
                || Existing.ChannelId == Definition.ChannelId;
        }
    );
    if (bDuplicate)
    {
        return false;
    }
    Schemas.Add(Definition);
    return true;
}

int32 USharMessageSchemaCatalog::GetSchemaCount() const
{
    return Schemas.Num();
}
