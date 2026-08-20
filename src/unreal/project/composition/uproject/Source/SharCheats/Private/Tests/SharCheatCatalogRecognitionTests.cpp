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
//   - Shar cheat catalog recognition tests composition module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Shar cheat catalog recognition tests composition module.
// - Description:
//   - Implements the declared composition module responsibility for project.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Shar cheat catalog recognition tests composition module.

#if WITH_DEV_AUTOMATION_TESTS

#include "SharCheatTestFixtures.h"

#include "Cheats/SharCheatContracts.h"
#include "Cheats/SharCheatEffectSubsystem.h"
#include "Cheats/SharCheatSubsystem.h"
#include "Meta/SharMetaCatalogDefinition.h"

#include "Misc/AutomationTest.h"

namespace
{
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharCheatCatalogValidationTest,
    "SHAR.Cheats.Catalog.Validation",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)

IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharCheatSuccessfulRecognitionTest,
    "SHAR.Cheats.Recognition.SuccessfulMatch",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
} // namespace

bool FSharCheatCatalogValidationTest::RunTest(const FString& Parameters)
{
    (void)Parameters;
    // jig-ignore-next-line: exact syntax is indivisible
    const USharMetaCatalogDefinition* ValidDefinition = MakeMetaCatalogDefinition();
    TArray<FText> Errors;
    ValidDefinition->GatherValidationErrors(Errors);
    TestTrue(TEXT("Canonical meta catalog validates"), Errors.IsEmpty());

    USharMetaCatalogDefinition* DuplicateSequence =
        MakeMetaCatalogDefinition();
    DuplicateSequence->Cheats.Last().InputTokens = {
        ESharCheatInputToken::Up,
        ESharCheatInputToken::Left,
        ESharCheatInputToken::Down,
        ESharCheatInputToken::Right,
    };
    Errors.Reset();
    DuplicateSequence->GatherValidationErrors(Errors);
    // jig-ignore-next-line: exact syntax is indivisible
    TestTrue(TEXT("Duplicate semantic sequence is rejected"), !Errors.IsEmpty());

    USharMetaCatalogDefinition* InvalidPersistentToggle =
        MakeMetaCatalogDefinition();
    InvalidPersistentToggle->Cheats.Last().Lifetime =
        ESharCheatLifetime::PersistentTransaction;
    Errors.Reset();
    InvalidPersistentToggle->GatherValidationErrors(Errors);
    TestTrue(
        TEXT("Persistent transaction cannot be a toggle"),
        !Errors.IsEmpty()
    );
    return true;
}

bool FSharCheatSuccessfulRecognitionTest::RunTest(
    const FString& Parameters
)
{
    (void)Parameters;
    const FSharCheatRuntimeFixture Runtime = MakeCheatRuntime();
    const FSharCheatArmRequest Request = MakeCheatArmRequest(
        FName(TEXT("recognize_unlock_cards"))
    );
    TestTrue(
        TEXT("Recognizer arms for one local player"),
        Runtime.CheatSubsystem->Arm(Request)
            == ESharCheatOperationResult::Accepted
    );
    TestTrue(
        TEXT("Button-up delivery does not advance recognition"),
        Runtime.CheatSubsystem->AcceptInput(MakeCheatInputEvent(
            Request,
            FName(TEXT("delivery_noise")),
            ESharCheatInputToken::Up,
            1,
            ESharCheatInputTransition::TokenUp
        )) == ESharCheatOperationResult::IgnoredInput
    );

    TestTrue(
        TEXT("Up token is accepted"),
        Runtime.CheatSubsystem->AcceptInput(MakeCheatInputEvent(
            Request,
            FName(TEXT("delivery_up")),
            ESharCheatInputToken::Up,
            CheatInputOrdinalUp
        )) == ESharCheatOperationResult::Accepted
    );
    TestTrue(
        TEXT("Left token is accepted"),
        Runtime.CheatSubsystem->AcceptInput(MakeCheatInputEvent(
            Request,
            FName(TEXT("delivery_left")),
            ESharCheatInputToken::Left,
            CheatInputOrdinalLeft
        )) == ESharCheatOperationResult::Accepted
    );
    TestTrue(
        TEXT("Down token is accepted"),
        Runtime.CheatSubsystem->AcceptInput(MakeCheatInputEvent(
            Request,
            FName(TEXT("delivery_down")),
            ESharCheatInputToken::Down,
            CheatInputOrdinalDown
        )) == ESharCheatOperationResult::Accepted
    );
    TestTrue(
        TEXT("Right token completes the sequence"),
        Runtime.CheatSubsystem->AcceptInput(MakeCheatInputEvent(
            Request,
            FName(TEXT("delivery_right")),
            ESharCheatInputToken::Right,
            CheatInputOrdinalRight
        )) == ESharCheatOperationResult::Accepted
    );
    TestTrue(
        TEXT("Four-token sequence matches one cheat"),
        Runtime.CheatSubsystem->GetRecognitionOutcome(Request.RecognitionId)
            == ESharCheatRecognitionOutcome::Matched
    );
    TestTrue(
        TEXT("Matched recognition publishes a queued effect request"),
        Runtime.EffectSubsystem->GetQueuePosition(Request.RecognitionId) == 1
    );
    return true;
}

#endif
