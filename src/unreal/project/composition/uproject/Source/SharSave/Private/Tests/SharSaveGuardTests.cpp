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
//   - Shar save guard tests composition module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Shar save guard tests composition module.
// - Description:
//   - Implements the declared composition module responsibility for project.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Shar save guard tests composition module.

#if WITH_DEV_AUTOMATION_TESTS

#include "SharSaveTestFixtures.h"

#include "Misc/AutomationTest.h"
#include "Save/SharSaveContracts.h"
#include "Save/SharSaveRepositorySubsystem.h"

namespace
{
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharSaveStaleEvidenceGuardTest,
    "SHAR.Save.Transaction.StaleEvidenceAndTerminalGuards",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
} // namespace

bool FSharSaveStaleEvidenceGuardTest::RunTest(const FString& Parameters)
{
    (void)Parameters;
    const FSharSaveRuntimeFixture Runtime = MakeSaveRuntime();
    const FSharSaveSlotId Slot = MakeSaveSlotId(FName(TEXT("slot_b")));
    const FSharSaveOperationRequest Request = MakeSaveRequest(
        {
            .OperationId = FName(TEXT("save_stale_guard")),
            .Kind = ESharSaveOperationKind::Save,
            .Priority = ESharSaveOperationPriority::Manual,
            .Slot = Slot,
        },
        TEXT("sha256:empty_v1")
    );
    const FSharSaveDocumentDescriptor Document =
        MakeSaveDocument(TEXT("sha256:accepted_v2"), CurrentSaveSchemaVersion);
    Runtime.Repository->Submit(Request);
    PrepareSaveCandidate(*Runtime.Repository, Request, Document);
    FSharSaveAdapterEvidence Evidence = MakeSaveEvidence({
        .OperationId = Request.OperationId,
        .Slot = Slot,
        .Stage = ESharSaveAdapterStage::CandidateWritten,
        .Document = Document,
        .ResultingAcceptedRevision = FString(),
    });
    Evidence.OperationRevision = TEXT("sha256:operation_old");

    TestTrue(
        TEXT("Stale adapter evidence is rejected"),
        Runtime.Repository->AcceptAdapterEvidence(Evidence)
            == ESharSaveOperationResult::StaleRevision
    );
    TestTrue(
        TEXT("Cancelled operation publishes one terminal result"),
        Runtime.Repository->Resolve(MakeSaveResolution(
            Request.OperationId,
            ESharSaveResolutionCommand::Cancel
        )) == ESharSaveOperationResult::Accepted
    );
    TestTrue(
        TEXT("Duplicate resolution is rejected"),
        Runtime.Repository->Resolve(MakeSaveResolution(
            Request.OperationId,
            ESharSaveResolutionCommand::Timeout
        )) == ESharSaveOperationResult::AlreadyTerminal
    );
    TestTrue(
        TEXT("Cancellation preserves accepted slot revision"),
        Runtime.Repository->GetSlotState(Slot).AcceptedRevision
            == TEXT("sha256:empty_v1")
    );
    TestTrue(
        TEXT("Terminal operation releases explicitly"),
        Runtime.Repository->Release(Request.OperationId)
            == ESharSaveOperationResult::Accepted
    );
    TestTrue(
        TEXT("Released operation no longer counts as active"),
        Runtime.Repository->GetUnreleasedOperationCount() == 0
    );
    return true;
}

#endif
