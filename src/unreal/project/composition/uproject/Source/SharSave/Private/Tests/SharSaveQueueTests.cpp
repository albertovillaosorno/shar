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
//   - Shar save queue tests composition module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Shar save queue tests composition module.
// - Description:
//   - Implements the declared composition module responsibility for project.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Shar save queue tests composition module.

#if WITH_DEV_AUTOMATION_TESTS

#include "SharSaveTestFixtures.h"

#include "Misc/AutomationTest.h"
#include "Save/SharSaveContracts.h"
#include "Save/SharSaveRepositorySubsystem.h"

namespace
{
constexpr int32 FirstSaveQueuePosition = 1;
constexpr int32 SecondSaveQueuePosition = 2;
constexpr int32 ThirdSaveQueuePosition = 3;

IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharSaveQueueOrderingTest,
    "SHAR.Save.Queue.DeterministicOrdering",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
} // namespace

bool FSharSaveQueueOrderingTest::RunTest(const FString& Parameters)
{
    (void)Parameters;
    const FSharSaveRuntimeFixture Runtime = MakeSaveRuntime();
    const FSharSaveOperationRequest Background = MakeSaveRequest(
        {
            .OperationId = FName(TEXT("background_save")),
            .Kind = ESharSaveOperationKind::Save,
            .Priority = ESharSaveOperationPriority::Background,
            .Slot = MakeSaveSlotId(FName(TEXT("slot_b"))),
        },
        TEXT("sha256:empty_v1")
    );
    const FSharSaveSlotState SlotC = MakeSaveSlotState(
        MakeSaveSlotId(FName(TEXT("slot_c"))),
        false
    );
    Runtime.Repository->RegisterSlot(SlotC);
    const FSharSaveOperationRequest ManualB = MakeSaveRequest(
        {
            .OperationId = FName(TEXT("manual_save_b")),
            .Kind = ESharSaveOperationKind::Save,
            .Priority = ESharSaveOperationPriority::Manual,
            .Slot = SlotC.Slot,
        },
        TEXT("sha256:empty_v1")
    );
    const FSharSaveSlotState SlotD = MakeSaveSlotState(
        MakeSaveSlotId(FName(TEXT("slot_d"))),
        false
    );
    Runtime.Repository->RegisterSlot(SlotD);
    const FSharSaveOperationRequest ManualA = MakeSaveRequest(
        {
            .OperationId = FName(TEXT("manual_save_a")),
            .Kind = ESharSaveOperationKind::Save,
            .Priority = ESharSaveOperationPriority::Manual,
            .Slot = SlotD.Slot,
        },
        TEXT("sha256:empty_v1")
    );
    Runtime.Repository->Submit(Background);
    Runtime.Repository->Submit(ManualB);
    Runtime.Repository->Submit(ManualA);

    TestTrue(
        TEXT("Equal manual priority uses lexical operation identity"),
        Runtime.Repository->GetQueuePosition(ManualA.OperationId)
            == FirstSaveQueuePosition
    );
    TestTrue(
        TEXT("Second manual operation follows lexical identity"),
        Runtime.Repository->GetQueuePosition(ManualB.OperationId)
            == SecondSaveQueuePosition
    );
    TestTrue(
        TEXT("Background operation follows manual operations"),
        Runtime.Repository->GetQueuePosition(Background.OperationId)
            == ThirdSaveQueuePosition
    );
    TestTrue(
        TEXT("Non-head save cannot begin"),
        Runtime.Repository->Begin(Background.OperationId)
            == ESharSaveOperationResult::NotHead
    );
    return true;
}

#endif
