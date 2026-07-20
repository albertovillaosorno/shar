// File: SharCheatEffectLifecycleTests.cpp
// Path: src/uproject/Source/SharCheats/Private/Tests/SharCheatEffectLifecycleTests.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: deterministic effect queue ordering and successful correlated postcondition lifecycle tests only.
// Specification: docs/technical/unreal/progression-collectibles-and-cheats.md

#if WITH_DEV_AUTOMATION_TESTS

#include "SharCheatTestFixtures.h"

#include "Cheats/SharCheatContracts.h"
#include "Cheats/SharCheatEffectSubsystem.h"

#include "Misc/AutomationTest.h"

namespace
{
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharCheatDeterministicQueueTest,
    "SHAR.Cheats.Effects.DeterministicQueue",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)

IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharCheatSuccessfulEffectLifecycleTest,
    "SHAR.Cheats.Effects.SuccessLifecycle",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
} // namespace

bool FSharCheatDeterministicQueueTest::RunTest(const FString& Parameters)
{
    (void)Parameters;
    const FSharCheatRuntimeFixture Runtime = MakeCheatRuntime();
    const FSharCheatActivationRequest UserRequest =
        MakeCheatActivationRequest(
            FName(TEXT("zeta_user_effect")),
            FName(TEXT("developer_scene_tree")),
            ESharCheatEffectPriority::User,
            ESharCheatEffectAction::Enable
        );
    const FSharCheatActivationRequest RecoveryBeta =
        MakeCheatActivationRequest(
            FName(TEXT("beta_recovery_effect")),
            FName(TEXT("show_speedometer")),
            ESharCheatEffectPriority::Recovery,
            ESharCheatEffectAction::Enable
        );
    const FSharCheatActivationRequest RecoveryAlpha =
        MakeCheatActivationRequest(
            FName(TEXT("alpha_recovery_effect")),
            FName(TEXT("unlock_cards")),
            ESharCheatEffectPriority::Recovery,
            ESharCheatEffectAction::Enable
        );
    TestTrue(
        TEXT("User activation queues"),
        Runtime.EffectSubsystem->Submit(UserRequest)
            == ESharCheatOperationResult::Accepted
    );
    TestTrue(
        TEXT("Recovery beta activation queues"),
        Runtime.EffectSubsystem->Submit(RecoveryBeta)
            == ESharCheatOperationResult::Accepted
    );
    TestTrue(
        TEXT("Recovery alpha activation queues"),
        Runtime.EffectSubsystem->Submit(RecoveryAlpha)
            == ESharCheatOperationResult::Accepted
    );
    TestTrue(
        TEXT("Recovery priority and lexical identity select alpha first"),
        Runtime.EffectSubsystem->GetQueuePosition(
            RecoveryAlpha.ActivationId
        ) == 1
    );
    TestTrue(
        TEXT("Recovery beta is second"),
        Runtime.EffectSubsystem->GetQueuePosition(
            RecoveryBeta.ActivationId
        ) == CheatQueuePositionSecond
    );
    TestTrue(
        TEXT("User activation is third"),
        Runtime.EffectSubsystem->GetQueuePosition(UserRequest.ActivationId)
            == CheatQueuePositionThird
    );
    TestTrue(
        TEXT("Lower-ranked activation cannot begin"),
        Runtime.EffectSubsystem->Begin(UserRequest.ActivationId)
            == ESharCheatOperationResult::NotHead
    );
    TestTrue(
        TEXT("Deterministic head begins"),
        Runtime.EffectSubsystem->Begin(RecoveryAlpha.ActivationId)
            == ESharCheatOperationResult::Accepted
    );
    return true;
}

bool FSharCheatSuccessfulEffectLifecycleTest::RunTest(
    const FString& Parameters
)
{
    (void)Parameters;
    const FSharCheatRuntimeFixture Runtime = MakeCheatRuntime();
    const FSharCheatActivationRequest Request = MakeCheatActivationRequest(
        FName(TEXT("activate_unlock_cards")),
        FName(TEXT("unlock_cards")),
        ESharCheatEffectPriority::User,
        ESharCheatEffectAction::Enable
    );
    TestTrue(
        TEXT("Effect request is accepted"),
        Runtime.EffectSubsystem->Submit(Request)
            == ESharCheatOperationResult::Accepted
    );
    TestTrue(
        TEXT("Head request begins dispatch"),
        Runtime.EffectSubsystem->Begin(Request.ActivationId)
            == ESharCheatOperationResult::Accepted
    );
    TestTrue(
        TEXT("Dispatch waits for owner postcondition"),
        Runtime.EffectSubsystem->MarkDispatched(Request.ActivationId)
            == ESharCheatOperationResult::Accepted
    );
    TestTrue(
        TEXT("Effect is not active before correlated evidence"),
        !Runtime.EffectSubsystem->IsEnabled(
            Request.LocalPlayerId,
            Request.CheatId
        )
    );
    TestTrue(
        TEXT("Correlated postcondition completes activation"),
        Runtime.EffectSubsystem->AcceptPostconditionEvidence(
            MakeCheatPostconditionEvidence(Request)
        ) == ESharCheatOperationResult::Accepted
    );
    TestTrue(
        TEXT("Accepted chapter overlay becomes visible"),
        Runtime.EffectSubsystem->IsEnabled(
            Request.LocalPlayerId,
            Request.CheatId
        )
    );
    TestTrue(
        TEXT("Activation publishes one successful terminal result"),
        Runtime.EffectSubsystem->GetTerminalResult(Request.ActivationId)
            == ESharCheatTerminalResult::Success
    );
    TestTrue(
        TEXT("Completed activation releases explicitly"),
        Runtime.EffectSubsystem->Release(Request.ActivationId)
            == ESharCheatOperationResult::Accepted
    );
    return true;
}

#endif
