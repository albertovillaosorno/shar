// File: SharSaveReadDeleteTests.cpp
// Path: src/uproject/Source/SharSave/Private/Tests/SharSaveReadDeleteTests.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: verified load, recovery, and deletion operation tests only.
// Specification: docs/technical/unreal/platform-save-storage-and-lifecycle.md
// LARGE-FILE owner=SharSave; reason=one cohesive accepted-slot read/recovery/delete scenario;
// split=separate recovery tests if provider journal selection becomes implemented;
// validation=validate.sh SharSave plus Unreal automation; review=2027-01.

#if WITH_DEV_AUTOMATION_TESTS

#include "SharSaveTestFixtures.h"

#include "Misc/AutomationTest.h"
#include "Save/SharSaveContracts.h"
#include "Save/SharSaveRepositorySubsystem.h"

static ESharSaveOperationResult CompleteReadOperation(
    USharSaveRepositorySubsystem& Repository,
    const FSharSaveOperationRequest& OperationSpec,
    const ESharSaveAdapterStage Stage,
    const FSharSaveDocumentDescriptor& Document
)
{
    Repository.Submit(OperationSpec);
    Repository.Begin(OperationSpec.OperationId);
    return Repository.AcceptAdapterEvidence(MakeSaveEvidence({
        .OperationId = OperationSpec.OperationId,
        .Slot = OperationSpec.Slot,
        .Stage = Stage,
        .Document = Document,
        .ResultingAcceptedRevision = TEXT("sha256:accepted_v1"),
    }));
}

namespace
{
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharSaveReadRecoveryDeleteTest,
    "SHAR.Save.Transaction.LoadRecoveryDelete",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
} // namespace

bool FSharSaveReadRecoveryDeleteTest::RunTest(const FString& Parameters)
{
    (void)Parameters;
    const FSharSaveRuntimeFixture Runtime = MakeSaveRuntime();
    const FSharSaveSlotId Slot = MakeSaveSlotId(FName(TEXT("slot_a")));
    const FSharSaveDocumentDescriptor AcceptedDocument =
        MakeSaveDocument(TEXT("sha256:accepted_v1"), CurrentSaveSchemaVersion);

    const FSharSaveOperationRequest LoadRequest = MakeSaveRequest(
        {
            .OperationId = FName(TEXT("load_slot_a")),
            .Kind = ESharSaveOperationKind::Load,
            .Priority = ESharSaveOperationPriority::Manual,
            .Slot = Slot,
        },
        TEXT("sha256:accepted_v1")
    );
    TestTrue(
        TEXT("Verified load completes successfully"),
        CompleteReadOperation(
            *Runtime.Repository,
            LoadRequest,
            ESharSaveAdapterStage::ReadCompleted,
            AcceptedDocument
        ) == ESharSaveOperationResult::Accepted
    );

    const FSharSaveOperationRequest RecoveryRequest = MakeSaveRequest(
        {
            .OperationId = FName(TEXT("recover_slot_a")),
            .Kind = ESharSaveOperationKind::Recover,
            .Priority = ESharSaveOperationPriority::LifecycleCritical,
            .Slot = Slot,
        },
        TEXT("sha256:accepted_v1")
    );
    TestTrue(
        TEXT("Recovery selects the verified accepted revision"),
        CompleteReadOperation(
            *Runtime.Repository,
            RecoveryRequest,
            ESharSaveAdapterStage::RecoveryCompleted,
            AcceptedDocument
        ) == ESharSaveOperationResult::Accepted
    );
    TestTrue(
        TEXT("Load and recovery do not rewrite accepted revision"),
        Runtime.Repository->GetSlotState(Slot).AcceptedRevision
            == TEXT("sha256:accepted_v1")
    );

    const FSharSaveOperationRequest DeleteRequest = MakeSaveRequest(
        {
            .OperationId = FName(TEXT("delete_slot_a")),
            .Kind = ESharSaveOperationKind::Delete,
            .Priority = ESharSaveOperationPriority::Manual,
            .Slot = Slot,
        },
        TEXT("sha256:accepted_v1")
    );
    Runtime.Repository->Submit(DeleteRequest);
    Runtime.Repository->Begin(DeleteRequest.OperationId);
    TestTrue(
        TEXT("Verified delete completes successfully"),
        Runtime.Repository->AcceptAdapterEvidence(MakeSaveEvidence({
            .OperationId = DeleteRequest.OperationId,
            .Slot = Slot,
            .Stage = ESharSaveAdapterStage::DeleteCompleted,
            .Document = FSharSaveDocumentDescriptor{},
            .ResultingAcceptedRevision = TEXT("sha256:deleted_v1"),
        })) == ESharSaveOperationResult::Accepted
    );
    const FSharSaveSlotState DeletedSlot =
        Runtime.Repository->GetSlotState(Slot);
    TestFalse(TEXT("Deleted slot is unoccupied"), DeletedSlot.bOccupied);
    TestTrue(
        TEXT("Deleted slot exposes verified tombstone revision"),
        DeletedSlot.AcceptedRevision == TEXT("sha256:deleted_v1")
    );
    return true;
}

#endif
