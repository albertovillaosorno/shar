// File: SharCheatEffectLifetimeTests.cpp
// Path: src/uproject/Source/SharCheats/Private/Tests/SharCheatEffectLifetimeTests.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: session, chapter, and mission effect-lifetime expiration tests only.
// Specification: docs/technical/unreal/progression-collectibles-and-cheats.md

#if WITH_DEV_AUTOMATION_TESTS

#include "SharCheatTestFixtures.h"

#include "Cheats/SharCheatContracts.h"
#include "Cheats/SharCheatEffectSubsystem.h"

#include "Misc/AutomationTest.h"

namespace
{
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharCheatEffectLifetimeExpirationTest,
    "SHAR.Cheats.Effects.LifetimeExpiration",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)

struct FSharCheatLifetimeScenario
{
    FSharCheatRuntimeFixture Runtime;
    FSharCheatActivationRequest SessionEffect;
    FSharCheatActivationRequest ChapterEffect;
    FSharCheatActivationRequest MissionEffect;
};
} // namespace

static bool CompleteCheatActivation(
    USharCheatEffectSubsystem& Subsystem,
    const FSharCheatActivationRequest& Request
)
{
    return Subsystem.Submit(Request) == ESharCheatOperationResult::Accepted
        && Subsystem.Begin(Request.ActivationId)
            == ESharCheatOperationResult::Accepted
        && Subsystem.MarkDispatched(Request.ActivationId)
            == ESharCheatOperationResult::Accepted
        && Subsystem.AcceptPostconditionEvidence(
            MakeCheatPostconditionEvidence(Request)
        ) == ESharCheatOperationResult::Accepted;
}

static FSharCheatLifetimeScenario MakeLifetimeScenario()
{
    FSharCheatLifetimeScenario Scenario;
    Scenario.Runtime = MakeCheatRuntime();
    Scenario.SessionEffect = MakeCheatActivationRequest(
        FName(TEXT("enable_scene_tree_lifetime")),
        FName(TEXT("developer_scene_tree")),
        ESharCheatEffectPriority::User,
        ESharCheatEffectAction::Enable
    );
    Scenario.ChapterEffect = MakeCheatActivationRequest(
        FName(TEXT("enable_unlock_cards_lifetime")),
        FName(TEXT("unlock_cards")),
        ESharCheatEffectPriority::User,
        ESharCheatEffectAction::Enable
    );
    Scenario.MissionEffect = MakeCheatActivationRequest(
        FName(TEXT("enable_speedometer_lifetime")),
        FName(TEXT("show_speedometer")),
        ESharCheatEffectPriority::User,
        ESharCheatEffectAction::Enable
    );
    return Scenario;
}

static bool ActivateLifetimeEffects(FSharCheatLifetimeScenario& Scenario)
{
    return CompleteCheatActivation(
               *Scenario.Runtime.EffectSubsystem,
               Scenario.SessionEffect
           )
        && CompleteCheatActivation(
            *Scenario.Runtime.EffectSubsystem,
            Scenario.ChapterEffect
        )
        && CompleteCheatActivation(
            *Scenario.Runtime.EffectSubsystem,
            Scenario.MissionEffect
        );
}

static bool VerifyMissionBoundary(FSharCheatLifetimeScenario& Scenario)
{
    FSharCheatContextUpdate Update;
    Update.ExpectedContextRevision = TEXT("sha256:cheat_context_v1");
    Update.UpdatedContext = MakeCheatRuntimeContext();
    Update.UpdatedContext.ContextRevision = TEXT("sha256:cheat_context_v2");
    Update.UpdatedContext.MissionRevision = TEXT("sha256:mission_v2");
    return Scenario.Runtime.EffectSubsystem->UpdateContext(Update)
            == ESharCheatOperationResult::Accepted
        && !Scenario.Runtime.EffectSubsystem->IsEnabled(
            Scenario.MissionEffect.LocalPlayerId,
            Scenario.MissionEffect.CheatId
        )
        && Scenario.Runtime.EffectSubsystem->IsEnabled(
            Scenario.ChapterEffect.LocalPlayerId,
            Scenario.ChapterEffect.CheatId
        );
}

static bool VerifyChapterBoundary(FSharCheatLifetimeScenario& Scenario)
{
    FSharCheatContextUpdate Update;
    Update.ExpectedContextRevision = TEXT("sha256:cheat_context_v2");
    Update.UpdatedContext = MakeCheatRuntimeContext();
    Update.UpdatedContext.ContextRevision = TEXT("sha256:cheat_context_v3");
    Update.UpdatedContext.ChapterRevision = TEXT("sha256:chapter_v2");
    Update.UpdatedContext.MissionRevision = TEXT("sha256:mission_v3");
    return Scenario.Runtime.EffectSubsystem->UpdateContext(Update)
            == ESharCheatOperationResult::Accepted
        && !Scenario.Runtime.EffectSubsystem->IsEnabled(
            Scenario.ChapterEffect.LocalPlayerId,
            Scenario.ChapterEffect.CheatId
        )
        && Scenario.Runtime.EffectSubsystem->IsEnabled(
            Scenario.SessionEffect.LocalPlayerId,
            Scenario.SessionEffect.CheatId
        );
}

static bool VerifySessionBoundary(FSharCheatLifetimeScenario& Scenario)
{
    FSharCheatContextUpdate Update;
    Update.ExpectedContextRevision = TEXT("sha256:cheat_context_v3");
    Update.UpdatedContext = MakeCheatRuntimeContext();
    Update.UpdatedContext.ContextRevision = TEXT("sha256:cheat_context_v4");
    Update.UpdatedContext.SessionRevision = TEXT("sha256:session_v2");
    Update.UpdatedContext.ChapterRevision = TEXT("sha256:chapter_v2");
    Update.UpdatedContext.MissionRevision = TEXT("sha256:mission_v3");
    return Scenario.Runtime.EffectSubsystem->UpdateContext(Update)
            == ESharCheatOperationResult::Accepted
        && !Scenario.Runtime.EffectSubsystem->IsEnabled(
            Scenario.SessionEffect.LocalPlayerId,
            Scenario.SessionEffect.CheatId
        );
}

bool FSharCheatEffectLifetimeExpirationTest::RunTest(
    const FString& Parameters
)
{
    (void)Parameters;
    FSharCheatLifetimeScenario Scenario = MakeLifetimeScenario();
    TestTrue(
        TEXT("All lifetime effects reach their postconditions"),
        ActivateLifetimeEffects(Scenario)
    );
    TestTrue(
        TEXT("Mission boundary expires only mission effects"),
        VerifyMissionBoundary(Scenario)
    );
    TestTrue(
        TEXT("Chapter boundary expires chapter effects"),
        VerifyChapterBoundary(Scenario)
    );
    TestTrue(
        TEXT("Session boundary expires session effects"),
        VerifySessionBoundary(Scenario)
    );
    return true;
}

#endif
