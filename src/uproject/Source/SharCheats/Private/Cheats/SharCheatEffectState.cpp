// File: SharCheatEffectState.cpp
// Path: src/uproject/Source/SharCheats/Private/Cheats/SharCheatEffectState.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: enabled cheat-effect projection, context-bound expiration, and stale activation cancellation only.
// Specification: docs/technical/unreal/progression-collectibles-and-cheats.md

#include "Cheats/SharCheatEffectSubsystem.h"

#include "Cheats/SharCheatContracts.h"

#include "Meta/SharMetaCatalogDefinition.h"

void USharCheatEffectSubsystem::ApplySuccessfulPostcondition(
    const FSharCheatActivationSnapshot& Activation,
    const FSharCheatDefinition& Definition
)
{
    if (Activation.Request.Action == ESharCheatEffectAction::Execute)
    {
        return;
    }
    if (Activation.Request.Action == ESharCheatEffectAction::Disable)
    {
        TArray<FSharEnabledCheatEffect> RetainedEffects;
        for (const FSharEnabledCheatEffect& Effect : EnabledEffects)
        {
            const bool bMatchesActivation = Effect.LocalPlayerId
                    == Activation.Request.LocalPlayerId
                && Effect.CheatId == Activation.Request.CheatId;
            if (!bMatchesActivation)
            {
                RetainedEffects.Add(Effect);
            }
        }
        EnabledEffects = RetainedEffects;
        return;
    }
    FSharEnabledCheatEffect Effect;
    Effect.CheatId = Activation.Request.CheatId;
    Effect.LocalPlayerId = Activation.Request.LocalPlayerId;
    Effect.Lifetime = Definition.Lifetime;
    Effect.ActivationRevision = Activation.Request.ActivationRevision;
    Effect.SessionRevision = Context.SessionRevision;
    Effect.ChapterRevision = Context.ChapterRevision;
    Effect.MissionRevision = Context.MissionRevision;
    EnabledEffects.Add(Effect);
}

void USharCheatEffectSubsystem::CancelStaleActivations()
{
    for (FSharCheatActivationSnapshot& Activation : Activations)
    {
        if (Activation.State != ESharCheatEffectState::Released
            && !IsTerminalState(Activation.State))
        {
            (void)PublishTerminal(
                Activation,
                ESharCheatEffectState::Cancelled,
                ESharCheatTerminalResult::Cancelled
            );
        }
    }
}

void USharCheatEffectSubsystem::ExpireEffects(
    const FSharCheatRuntimeContext& PreviousContext
)
{
    const bool bSessionChanged =
        PreviousContext.SessionRevision != Context.SessionRevision;
    const bool bChapterChanged =
        PreviousContext.ChapterRevision != Context.ChapterRevision;
    const bool bMissionChanged =
        PreviousContext.MissionRevision != Context.MissionRevision;
    TArray<FSharEnabledCheatEffect> RetainedEffects;
    for (const FSharEnabledCheatEffect& Effect : EnabledEffects)
    {
        const bool bExpiresWithSession = bSessionChanged;
        const bool bExpiresWithChapter = bChapterChanged
            && (Effect.Lifetime == ESharCheatLifetime::Chapter
                || Effect.Lifetime == ESharCheatLifetime::Mission);
        const bool bExpiresWithMission = bMissionChanged
            && Effect.Lifetime == ESharCheatLifetime::Mission;
        if (!bExpiresWithSession && !bExpiresWithChapter
            && !bExpiresWithMission)
        {
            RetainedEffects.Add(Effect);
        }
    }
    EnabledEffects = RetainedEffects;
}
