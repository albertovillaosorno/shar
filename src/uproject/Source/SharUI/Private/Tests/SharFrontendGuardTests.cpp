// File: SharFrontendGuardTests.cpp
// Path: src/uproject/Source/SharUI/Private/Tests/SharFrontendGuardTests.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: modal rollback, stale evidence, exactly-one terminal result, and release guard tests only.
// Specification: docs/technical/unreal/frontend-screen-flow-and-settings-runtime.md

#if WITH_DEV_AUTOMATION_TESTS

#include "SharFrontendTestFixtures.h"

#include "Misc/AutomationTest.h"
#include "UI/SharFrontendCatalogDefinition.h"
#include "UI/SharFrontendFlowContracts.h"
#include "UI/SharFrontendFlowSubsystem.h"

namespace
{
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharFrontendModalRollbackTest,
    "SHAR.Frontend.Flow.ModalRollback",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)

IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharFrontendStaleEvidenceTerminalGuardTest,
    "SHAR.Frontend.Flow.StaleEvidenceAndTerminalGuards",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
} // namespace

bool FSharFrontendModalRollbackTest::RunTest(const FString& Parameters)
{
    (void)Parameters;
    const FSharFrontendRuntimeFixture Runtime = MakeFrontendRuntime();
    const FSharFrontendNavigationRequest OpenModal = MakeFrontendRequest(
        Runtime,
        FName(TEXT("open_confirm_quit")),
        FName(TEXT("confirm_quit")),
        ESharFrontendNavigationPriority::User,
        ESharFrontendHistoryPolicy::Preserve
    );
    Runtime.FlowSubsystem->Submit(OpenModal);
    Runtime.FlowSubsystem->Begin(OpenModal.RequestId);
    AcceptFrontendPreCommit(Runtime, OpenModal.RequestId);
    Runtime.FlowSubsystem->Commit(OpenModal.RequestId);
    AcceptFrontendPostCommit(Runtime, OpenModal.RequestId);
    Runtime.FlowSubsystem->Release(OpenModal.RequestId);

    const FSharFrontendFlowObservation AcceptedModal =
        Runtime.FlowSubsystem->GetObservation();
    TestTrue(
        TEXT("Modal is active after successful verification"),
        AcceptedModal.ActiveModalScreenId == FName(TEXT("confirm_quit"))
    );

    const FSharFrontendNavigationRequest DismissModal = MakeFrontendRequest(
        Runtime,
        FName(TEXT("dismiss_confirm_quit")),
        FName(TEXT("main_menu")),
        ESharFrontendNavigationPriority::User,
        ESharFrontendHistoryPolicy::Pop
    );
    Runtime.FlowSubsystem->Submit(DismissModal);
    Runtime.FlowSubsystem->Begin(DismissModal.RequestId);
    AcceptFrontendPreCommit(Runtime, DismissModal.RequestId);
    TestTrue(
        TEXT("Modal dismissal candidate commits"),
        Runtime.FlowSubsystem->Commit(DismissModal.RequestId)
            == ESharFrontendOperationResult::Accepted
    );
    TestTrue(
        TEXT("Widget activation evidence is accepted"),
        Runtime.FlowSubsystem->AcceptEvidence(MakeFrontendEvidence(
            Runtime,
            DismissModal.RequestId,
            ESharFrontendReadinessKind::WidgetActivation,
            ESharFrontendEvidenceStatus::Ready
        )) == ESharFrontendOperationResult::Accepted
    );
    TestTrue(
        TEXT("Failed focus evidence rejects committed dismissal"),
        Runtime.FlowSubsystem->AcceptEvidence(MakeFrontendEvidence(
            Runtime,
            DismissModal.RequestId,
            ESharFrontendReadinessKind::Focus,
            ESharFrontendEvidenceStatus::Failed
        )) == ESharFrontendOperationResult::EvidenceFailed
    );

    const FSharFrontendFlowObservation RolledBack =
        Runtime.FlowSubsystem->GetObservation();
    TestTrue(
        TEXT("Rollback restores the accepted modal"),
        RolledBack.ActiveModalScreenId == FName(TEXT("confirm_quit"))
    );
    TestTrue(
        TEXT("Rollback restores the prior flow revision"),
        RolledBack.FlowRevision == AcceptedModal.FlowRevision
    );
    TestTrue(
        TEXT("Dismissal publishes one failed terminal result"),
        Runtime.FlowSubsystem->GetTerminalResult(DismissModal.RequestId)
            == ESharFrontendTerminalResult::Failed
    );
    return true;
}

bool FSharFrontendStaleEvidenceTerminalGuardTest::RunTest(
    const FString& Parameters
)
{
    (void)Parameters;
    const FSharFrontendRuntimeFixture Runtime = MakeFrontendRuntime();
    const FSharFrontendNavigationRequest Request = MakeFrontendRequest(
        Runtime,
        FName(TEXT("open_options_guarded")),
        FName(TEXT("options")),
        ESharFrontendNavigationPriority::User,
        ESharFrontendHistoryPolicy::Push
    );
    Runtime.FlowSubsystem->Submit(Request);
    Runtime.FlowSubsystem->Begin(Request.RequestId);

    FSharFrontendReadinessEvidence StaleEvidence = MakeFrontendEvidence(
        Runtime,
        Request.RequestId,
        ESharFrontendReadinessKind::DomainSnapshot,
        ESharFrontendEvidenceStatus::Ready
    );
    StaleEvidence.DestinationScreenRevision =
        TEXT("sha256:stale_destination");
    TestTrue(
        TEXT("Stale destination evidence is rejected"),
        Runtime.FlowSubsystem->AcceptEvidence(StaleEvidence)
            == ESharFrontendOperationResult::StaleRevision
    );

    AcceptFrontendPreCommit(Runtime, Request.RequestId);
    Runtime.FlowSubsystem->Commit(Request.RequestId);
    AcceptFrontendPostCommit(Runtime, Request.RequestId);
    FSharFrontendTransitionResolution Resolution;
    Resolution.RequestId = Request.RequestId;
    Resolution.Command = ESharFrontendResolutionCommand::Fail;
    Resolution.CatalogRevision = Request.CatalogRevision;
    Resolution.RequestRevision = Request.RequestRevision;
    TestTrue(
        TEXT("Terminal success cannot be replaced by failure"),
        Runtime.FlowSubsystem->Resolve(Resolution)
            == ESharFrontendOperationResult::AlreadyTerminal
    );
    TestTrue(
        TEXT("Successful terminal result remains unchanged"),
        Runtime.FlowSubsystem->GetTerminalResult(Request.RequestId)
            == ESharFrontendTerminalResult::Success
    );
    TestTrue(
        TEXT("Terminal transition releases"),
        Runtime.FlowSubsystem->Release(Request.RequestId)
            == ESharFrontendOperationResult::Accepted
    );
    TestTrue(
        TEXT("Released transition cannot release twice"),
        Runtime.FlowSubsystem->Release(Request.RequestId)
            == ESharFrontendOperationResult::Released
    );
    return true;
}

#endif
