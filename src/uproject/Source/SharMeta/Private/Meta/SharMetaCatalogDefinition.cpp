// File: SharMetaCatalogDefinition.cpp
// Path: src/uproject/Source/SharMeta/Private/Meta/SharMetaCatalogDefinition.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: load-free shared meta-catalog validation and cheat lookup only.
// Specification: docs/technical/unreal/progression-collectibles-and-cheats.md

#include "Meta/SharMetaCatalogDefinition.h"

#include "Algo/AnyOf.h"
#include "Content/SharPrimaryContentDefinition.h"
#include "Engine/DataAsset.h"

static void AddMetaCatalogError(TArray<FText>& OutErrors, const TCHAR* Message)
{
    OutErrors.Add(FText::FromString(Message));
}

static bool HasValidFeedbackIdentity(const FName& FeedbackEvent)
{
    return !FeedbackEvent.IsNone()
        && USharPrimaryContentDefinition::IsCanonicalIdentifier(FeedbackEvent);
}

static bool HasValidEffectParameters(const FSharCheatDefinition& Definition)
{
    if (Definition.EffectKind != ESharCheatEffectKind::ProgressionTransaction)
    {
        return true;
    }
    return USharPrimaryContentDefinition::IsCanonicalIdentifier(
               Definition.EffectParameters.OperationId
           )
        && !Definition.EffectParameters.TargetId.IsNone()
        && Definition.EffectParameters.Quantity != 0;
}

static bool IsValidCheatDefinition(const FSharCheatDefinition& Definition)
{
    const bool bValidIdentity =
        USharPrimaryContentDefinition::IsCanonicalIdentifier(
            Definition.CheatId
        );
    const bool bValidSequence =
        Definition.InputTokens.Num()
        == FSharCheatDefinition::RequiredInputTokenCount;
    const bool bValidFeedback =
        HasValidFeedbackIdentity(Definition.SuccessFeedbackEvent)
        && HasValidFeedbackIdentity(Definition.UnavailableFeedbackEvent)
        && HasValidFeedbackIdentity(Definition.DisabledFeedbackEvent)
        && HasValidFeedbackIdentity(Definition.InvalidSequenceFeedbackEvent);
    const bool bValidLifetime =
        Definition.Lifetime != ESharCheatLifetime::PersistentTransaction
        || Definition.ActivationMode
            == ESharCheatActivationMode::ImmediateCommand;
    return bValidIdentity && bValidSequence && bValidFeedback && bValidLifetime
        && HasValidEffectParameters(Definition);
}

static constexpr int64 CheatTokenRadix = 16;

static int64 BuildCheatSequenceKey(
    const TArray<ESharCheatInputToken>& InputTokens
)
{
    int64 Key = 0;
    for (const ESharCheatInputToken Token : InputTokens)
    {
        Key = (Key * CheatTokenRadix) + static_cast<uint8>(Token);
    }
    return Key;
}

static bool HasDuplicateCheatIds(
    const TArray<FSharCheatDefinition>& Definitions
)
{
    return Algo::AnyOf(
        Definitions,
        [&Definitions](const FSharCheatDefinition& Candidate)
        {
            return Algo::AnyOf(
                Definitions,
                [&Candidate](const FSharCheatDefinition& Other)
                {
                    return &Other != &Candidate
                        && Other.CheatId == Candidate.CheatId;
                }
            );
        }
    );
}

static bool HasDuplicateSequences(
    const TArray<FSharCheatDefinition>& Definitions
)
{
    return Algo::AnyOf(
        Definitions,
        [&Definitions](const FSharCheatDefinition& Candidate)
        {
            const int64 CandidateKey =
                BuildCheatSequenceKey(Candidate.InputTokens);
            return Algo::AnyOf(
                Definitions,
                [&Candidate, CandidateKey](const FSharCheatDefinition& Other)
                {
                    return &Other != &Candidate
                        && Other.InputTokens.Num()
                            == Candidate.InputTokens.Num()
                        && BuildCheatSequenceKey(Other.InputTokens)
                            == CandidateKey;
                }
            );
        }
    );
}

void USharMetaCatalogDefinition::GatherValidationErrors(
    TArray<FText>& OutErrors
) const
{
    USharPrimaryContentDefinition::GatherValidationErrors(OutErrors);
    if (Cheats.IsEmpty())
    {
        AddMetaCatalogError(
            OutErrors,
            TEXT("Meta catalog must contain at least one cheat definition.")
        );
    }
    if (HasDuplicateCheatIds(Cheats) || HasDuplicateSequences(Cheats))
    {
        AddMetaCatalogError(
            OutErrors,
            TEXT("Cheat identities and four-token sequences must be unique.")
        );
    }
    for (const FSharCheatDefinition& Definition : this->Cheats)
    {
        if (!IsValidCheatDefinition(Definition))
        {
            AddMetaCatalogError(
                OutErrors,
                TEXT("Cheat definition identity, policy, feedback, or typed effect parameters are invalid.")
            );
        }
    }
}

const FSharCheatDefinition* USharMetaCatalogDefinition::FindCheat(
    const FName& CheatId
) const
{
    for (const FSharCheatDefinition& Definition : this->Cheats)
    {
        if (Definition.CheatId == CheatId)
        {
            return &Definition;
        }
    }
    return nullptr;
}

const FSharCheatDefinition* USharMetaCatalogDefinition::FindCheatBySequence(
    const TArray<ESharCheatInputToken>& InputTokens
) const
{
    for (const FSharCheatDefinition& Definition : this->Cheats)
    {
        if (Definition.InputTokens.Num() == InputTokens.Num()
            && BuildCheatSequenceKey(Definition.InputTokens)
                == BuildCheatSequenceKey(InputTokens))
        {
            return &Definition;
        }
    }
    return nullptr;
}

FPrimaryAssetType USharMetaCatalogDefinition::GetDefinitionAssetType() const
{
    return {TEXT("SharMetaCatalog")};
}
