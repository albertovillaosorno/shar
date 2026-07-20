// File: SharSaveTransactionTests.cpp
// Path: src/uproject/Source/SharSave/Private/Tests/SharSaveTransactionTests.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: successful atomic save and interrupted-write preservation tests only.
// Specification: docs/technical/unreal/platform-save-storage-and-lifecycle.md
// LARGE-FILE owner=SharSave; reason=two cohesive save transaction lifecycle scenarios;
// split=separate atomic replacement tests if journaling policies expand;
// validation=validate.sh SharSave plus Unreal automation; review=2027-01.

#if WITH_DEV_AUTOMATION_TESTS

#include "SharSaveTestFixtures.h"

#include "Misc/AutomationTest.h"
#include "Save/SharSaveContracts.h"
#include "Save/SharSaveRepositorySubsystem.h"

namespace
{
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharSaveSuccessLifecycleTest,
    "SHAR.Save.Transaction.SuccessLifecycle",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharSaveInterruptedWriteTest,
    "SHAR.Save.Transaction.InterruptedWritePreservesAccepted",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
} // namespace

bool FSharSaveSuccessLifecycleTest::RunTest(const FString& Parameters)
{
    (void)Parameters;
    const FSharSaveRuntimeFixture Runtime = MakeSaveRuntime();
    const FSharSaveSlotId Slot = MakeSaveSlotId(FName(TEXT("slot_b")));
    const FSharSaveOperationRequest Request = MakeSaveRequest(
        {
            .OperationId = FName(TEXT("save_success")),
            .Kind = ESharSaveOperationKind::Save,
            .Priority = ESharSaveOperationPriority::Manual,
            .Slot = Slot,
        },
        TEXT("sha256:empty_v1")
    );
    const FSharSaveDocumentDescriptor Document =
        MakeSaveDocument(TEXT("sha256:accepted_v2"), CurrentSaveSchemaVersion);
    TestTrue(
        TEXT("Save request is accepted"),
        Runtime.Repository->Submit(Request)
            == ESharSaveOperationResult::Accepted
    );
    TestTrue(
        TEXT("Save begins in preparation"),
        Runtime.Repository->Begin(Request.OperationId)
            == ESharSaveOperationResult::Accepted
    );
    TestTrue(
        TEXT("Validated candidate starts adapter writing"),
        Runtime.Repository->AcceptCandidate(Request.OperationId, Document)
            == ESharSaveOperationResult::Accepted
    );
    Runtime.Repository->AcceptAdapterEvidence(MakeSaveEvidence({
        .OperationId = Request.OperationId,
        .Slot = Slot,
        .Stage = ESharSaveAdapterStage::CandidateWritten,
        .Document = Document,
        .ResultingAcceptedRevision = FString(),
    }));
    Runtime.Repository->AcceptAdapterEvidence(MakeSaveEvidence({
        .OperationId = Request.OperationId,
        .Slot = Slot,
        .Stage = ESharSaveAdapterStage::DurableFlushCompleted,
        .Document = Document,
        .ResultingAcceptedRevision = FString(),
    }));
    Runtime.Repository->AcceptAdapterEvidence(MakeSaveEvidence({
        .OperationId = Request.OperationId,
        .Slot = Slot,
        .Stage = ESharSaveAdapterStage::ReadBackValidated,
        .Document = Document,
        .ResultingAcceptedRevision = FString(),
    }));
    TestTrue(
        TEXT("Read-back validation does not replace accepted revision"),
        Runtime.Repository->GetSlotState(Slot).AcceptedRevision
            == TEXT("sha256:empty_v1")
    );
    TestTrue(
        TEXT("Atomic replacement publishes success"),
        Runtime.Repository->AcceptAdapterEvidence(MakeSaveEvidence({
            .OperationId = Request.OperationId,
            .Slot = Slot,
            .Stage = ESharSaveAdapterStage::AtomicReplaceCompleted,
            .Document = Document,
            .ResultingAcceptedRevision = Document.DocumentRevision,
        })) == ESharSaveOperationResult::Accepted
    );
    const FSharSaveSlotState AcceptedSlot =
        Runtime.Repository->GetSlotState(Slot);
    TestTrue(TEXT("Committed slot is occupied"), AcceptedSlot.bOccupied);
    TestTrue(
        TEXT("Committed slot exposes the verified candidate revision"),
        AcceptedSlot.AcceptedRevision == Document.DocumentRevision
    );
    TestTrue(
        TEXT("Successful save has one terminal result"),
        Runtime.Repository->GetTerminalResult(Request.OperationId)
            == ESharSaveTerminalResult::Success
    );
    return true;
}

bool FSharSaveInterruptedWriteTest::RunTest(const FString& Parameters)
{
    (void)Parameters;
    const FSharSaveRuntimeFixture Runtime = MakeSaveRuntime();
    const FSharSaveSlotId Slot = MakeSaveSlotId(FName(TEXT("slot_a")));
    const FSharSaveOperationRequest Request = MakeSaveRequest(
        {
            .OperationId = FName(TEXT("save_interrupted")),
            .Kind = ESharSaveOperationKind::Save,
            .Priority = ESharSaveOperationPriority::Autosave,
            .Slot = Slot,
        },
        TEXT("sha256:accepted_v1")
    );
    const FSharSaveDocumentDescriptor Document =
        MakeSaveDocument(TEXT("sha256:accepted_v2"), CurrentSaveSchemaVersion);
    Runtime.Repository->Submit(Request);
    PrepareSaveCandidate(*Runtime.Repository, Request, Document);
    Runtime.Repository->AcceptAdapterEvidence(MakeSaveEvidence({
        .OperationId = Request.OperationId,
        .Slot = Slot,
        .Stage = ESharSaveAdapterStage::CandidateWritten,
        .Document = Document,
        .ResultingAcceptedRevision = FString(),
    }));

    TestTrue(
        TEXT("Interrupted write publishes failure"),
        Runtime.Repository->Resolve(MakeSaveResolution(
            Request.OperationId,
            ESharSaveResolutionCommand::Fail
        )) == ESharSaveOperationResult::Accepted
    );
    TestTrue(
        TEXT("Interrupted write preserves previous accepted revision"),
        Runtime.Repository->GetSlotState(Slot).AcceptedRevision
            == TEXT("sha256:accepted_v1")
    );
    TestTrue(
        TEXT("Interrupted write preserves occupied state"),
        Runtime.Repository->GetSlotState(Slot).bOccupied
    );
    return true;
}

#endif
