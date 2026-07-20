// File: SharCheatRecognitionGuardTests.cpp
// Path: src/uproject/Source/SharCheats/Private/Tests/SharCheatRecognitionGuardTests.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: recognizer cancellation on correlated context changes and stale-input terminal guards only.
// Specification: docs/technical/unreal/progression-collectibles-and-cheats.md

#if WITH_DEV_AUTOMATION_TESTS

#include "SharCheatTestFixtures.h"

#include "Cheats/SharCheatContracts.h"
#include "Cheats/SharCheatSubsystem.h"
#include "Meta/SharMetaCatalogDefinition.h"

#include "Misc/AutomationTest.h"

static constexpr int64 InputOrdinalAfterContextChange = 2;

namespace
{
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharCheatContextCancellationTest,
    "SHAR.Cheats.Recognition.ContextCancellation",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
} // namespace

bool FSharCheatContextCancellationTest::RunTest(const FString& Parameters)
{
    (void)Parameters;
    const FSharCheatRuntimeFixture Runtime = MakeCheatRuntime();
    const FSharCheatArmRequest Request = MakeCheatArmRequest(
        FName(TEXT("recognize_context_cancel"))
    );
    TestTrue(
        TEXT("Recognizer arms before the mode change"),
        Runtime.CheatSubsystem->Arm(Request)
            == ESharCheatOperationResult::Accepted
    );
    TestTrue(
        TEXT("First semantic token enters collecting state"),
        Runtime.CheatSubsystem->AcceptInput(MakeCheatInputEvent(
            Request,
            FName(TEXT("delivery_before_context_change")),
            ESharCheatInputToken::Up,
            1
        )) == ESharCheatOperationResult::Accepted
    );

    FSharCheatContextUpdate Update;
    Update.ExpectedContextRevision = TEXT("sha256:cheat_context_v1");
    Update.UpdatedContext = MakeCheatRuntimeContext();
    Update.UpdatedContext.ContextRevision = TEXT("sha256:cheat_context_v2");
    Update.UpdatedContext.ApplicationModeRevision =
        TEXT("sha256:pause_mode_v1");
    TestTrue(
        TEXT("Correlated context update is accepted"),
        Runtime.CheatSubsystem->UpdateContext(Update)
            == ESharCheatOperationResult::Accepted
    );
    TestTrue(
        TEXT("Context change cancels the transient recognizer"),
        Runtime.CheatSubsystem->GetRecognitionOutcome(Request.RecognitionId)
            == ESharCheatRecognitionOutcome::InputCancelled
    );
    TestTrue(
        TEXT("Old-context input cannot revive a terminal recognizer"),
        Runtime.CheatSubsystem->AcceptInput(MakeCheatInputEvent(
            Request,
            FName(TEXT("delivery_after_context_change")),
            ESharCheatInputToken::Left,
            InputOrdinalAfterContextChange
        )) == ESharCheatOperationResult::AlreadyTerminal
    );
    TestTrue(
        TEXT("Recognizer releases explicitly"),
        Runtime.CheatSubsystem->Release(Request.RecognitionId)
            == ESharCheatOperationResult::Accepted
    );
    return true;
}

#endif
