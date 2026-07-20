// File: SharCheatEffectGuardTests.cpp
// Path: src/uproject/Source/SharCheats/Private/Tests/SharCheatEffectGuardTests.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: stale postcondition rejection, failure preservation, terminal uniqueness, and release tests only.
// Specification: docs/technical/unreal/progression-collectibles-and-cheats.md

#if WITH_DEV_AUTOMATION_TESTS

#include "SharCheatTestFixtures.h"

#include "Cheats/SharCheatContracts.h"
#include "Cheats/SharCheatEffectSubsystem.h"

#include "Misc/AutomationTest.h"

namespace
{
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharCheatStaleEvidenceTerminalGuardTest,
    "SHAR.Cheats.Effects.StaleEvidenceAndTerminalGuards",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
} // namespace

bool FSharCheatStaleEvidenceTerminalGuardTest::RunTest(
    const FString& Parameters
)
{
    (void)Parameters;
    const FSharCheatRuntimeFixture Runtime = MakeCheatRuntime();
    const FSharCheatActivationRequest Request = MakeCheatActivationRequest(
        FName(TEXT("activate_speedometer_guard")),
        FName(TEXT("show_speedometer")),
        ESharCheatEffectPriority::User,
        ESharCheatEffectAction::Enable
    );
    Runtime.EffectSubsystem->Submit(Request);
    Runtime.EffectSubsystem->Begin(Request.ActivationId);
    Runtime.EffectSubsystem->MarkDispatched(Request.ActivationId);

    FSharCheatPostconditionEvidence StaleEvidence =
        MakeCheatPostconditionEvidence(Request);
    StaleEvidence.ActivationRevision = TEXT("sha256:stale_activation");
    TestTrue(
        TEXT("Stale owner evidence is rejected"),
        Runtime.EffectSubsystem->AcceptPostconditionEvidence(StaleEvidence)
            == ESharCheatOperationResult::StaleRevision
    );
    TestTrue(
        TEXT("Stale evidence does not enable the effect"),
        !Runtime.EffectSubsystem->IsEnabled(
            Request.LocalPlayerId,
            Request.CheatId
        )
    );

    FSharCheatActivationResolution StaleResolution = MakeCheatResolution(
        Request,
        ESharCheatResolutionCommand::Fail
    );
    StaleResolution.ContextRevision = TEXT("sha256:stale_context");
    TestTrue(
        TEXT("Stale failure resolution is rejected"),
        Runtime.EffectSubsystem->Resolve(StaleResolution)
            == ESharCheatOperationResult::StaleRevision
    );
    TestTrue(
        TEXT("Correlated cancellation publishes one terminal result"),
        Runtime.EffectSubsystem->Resolve(MakeCheatResolution(
            Request,
            ESharCheatResolutionCommand::Cancel
        )) == ESharCheatOperationResult::Accepted
    );
    TestTrue(
        TEXT("Second terminal publication is rejected"),
        Runtime.EffectSubsystem->Resolve(MakeCheatResolution(
            Request,
            ESharCheatResolutionCommand::Fail
        )) == ESharCheatOperationResult::AlreadyTerminal
    );
    TestTrue(
        TEXT("Cancelled activation preserves disabled state"),
        !Runtime.EffectSubsystem->IsEnabled(
            Request.LocalPlayerId,
            Request.CheatId
        )
    );
    TestTrue(
        TEXT("Cancelled activation releases explicitly"),
        Runtime.EffectSubsystem->Release(Request.ActivationId)
            == ESharCheatOperationResult::Accepted
    );
    return true;
}

#endif
