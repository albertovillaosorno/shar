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
//   - Shar frontend flow contracts composition module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Shar frontend flow contracts composition module.
// - Description:
//   - Implements the declared composition module responsibility for project.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Shar frontend flow contracts composition module.

#pragma once

#include "CoreMinimal.h"

// jig-ignore-next-line: exact syntax is indivisible
// NOLINTNEXTLINE(llvm-include-order) -- Unreal requires the generated header last.
#include "UI/SharFrontendCatalogDefinition.h"
#include "SharFrontendFlowContracts.generated.h"

UENUM(BlueprintType)
enum class ESharFrontendNavigationPriority : uint8
{
    Development,
    Background,
    User,
    Recovery,
    Fatal,
};

UENUM(BlueprintType)
enum class ESharFrontendTransitionState : uint8
{
    Pending,
    Preparing,
    VerifyingReadiness,
    ReadyToCommit,
    Committed,
    VerifyingTarget,
    Success,
    Failed,
    Cancelled,
    Superseded,
    Released,
};

UENUM(BlueprintType)
enum class ESharFrontendEvidenceStatus : uint8
{
    Ready,
    Degraded,
    Failed,
};

UENUM(BlueprintType)
enum class ESharFrontendTerminalResult : uint8
{
    None,
    Success,
    Failed,
    Cancelled,
    Superseded,
};

UENUM(BlueprintType)
enum class ESharFrontendResolutionCommand : uint8
{
    Fail,
    Cancel,
    Supersede,
};

UENUM(BlueprintType)
enum class ESharFrontendOperationResult : uint8
{
    Accepted,
    NotConfigured,
    CatalogInactive,
    InvalidRequest,
    SourceMismatch,
    DestinationMissing,
    TransitionNotAllowed,
    DuplicateRequest,
    ConflictingTransition,
    NotFound,
    NotHead,
    StaleRevision,
    EvidenceNotRequired,
    DuplicateEvidence,
    EvidenceFailed,
    EvidenceMissing,
    InvalidState,
    AlreadyTerminal,
    HistoryEmpty,
    Released,
};

USTRUCT(BlueprintType)
struct SHARUI_API FSharFrontendNavigationRequest
{
    GENERATED_BODY()

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Identity")
    FName RequestId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Screen")
    FName SourceScreenId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Screen")
    FName DestinationScreenId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Caller")
    FName CallerId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Player")
    FName LocalPlayerId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Priority")
    ESharFrontendNavigationPriority Priority =
        ESharFrontendNavigationPriority::User;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "History")
    ESharFrontendHistoryPolicy HistoryPolicy =
        ESharFrontendHistoryPolicy::Push;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString CatalogRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString FlowRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString SourceScreenRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString RequestRevision;
};

USTRUCT(BlueprintType)
struct SHARUI_API FSharFrontendReadinessEvidence
{
    GENERATED_BODY()

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Identity")
    FName RequestId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Identity")
    FName EvidenceId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Readiness")
    ESharFrontendReadinessKind Kind =
        ESharFrontendReadinessKind::DomainSnapshot;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Readiness")
    ESharFrontendEvidenceStatus Status =
        ESharFrontendEvidenceStatus::Ready;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString CatalogRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString RequestRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString DestinationScreenRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString EvidenceRevision;
};

USTRUCT(BlueprintType)
struct SHARUI_API FSharFrontendTransitionResolution
{
    GENERATED_BODY()

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Identity")
    FName RequestId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Result")
    ESharFrontendResolutionCommand Command =
        ESharFrontendResolutionCommand::Fail;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString CatalogRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString RequestRevision;
};

USTRUCT(BlueprintType)
struct SHARUI_API FSharFrontendFlowObservation
{
    GENERATED_BODY()

    UPROPERTY(BlueprintReadOnly, Category = "Screen")
    FName ActivePrimaryScreenId;

    UPROPERTY(BlueprintReadOnly, Category = "Screen")
    FName ActiveModalScreenId;

    UPROPERTY(BlueprintReadOnly, Category = "History")
    TArray<FName> PrimaryHistory;

    UPROPERTY(BlueprintReadOnly, Category = "Revision")
    FString FlowRevision;

    UPROPERTY(BlueprintReadOnly, Category = "Revision")
    FString ActiveScreenRevision;

    UPROPERTY(BlueprintReadOnly, Category = "Transition")
    FName ActiveTransitionId;

    UPROPERTY(BlueprintReadOnly, Category = "Focus")
    FName StableFocusTargetId;
};

USTRUCT(BlueprintType)
struct SHARUI_API FSharFrontendTransitionSnapshot
{
    GENERATED_BODY()

    UPROPERTY(BlueprintReadOnly, Category = "Request")
    FSharFrontendNavigationRequest Request;

    UPROPERTY(BlueprintReadOnly, Category = "State")
    ESharFrontendTransitionState State =
        ESharFrontendTransitionState::Pending;

    UPROPERTY(BlueprintReadOnly, Category = "Result")
    ESharFrontendTerminalResult TerminalResult =
        ESharFrontendTerminalResult::None;

    UPROPERTY(BlueprintReadOnly, Category = "Ordering")
    int32 InsertionSequence = 0;

    UPROPERTY(BlueprintReadOnly, Category = "Evidence")
    TArray<FSharFrontendReadinessEvidence> Evidence;

    UPROPERTY(BlueprintReadOnly, Category = "Snapshot")
    FSharFrontendFlowObservation PriorObservation;

    UPROPERTY(BlueprintReadOnly, Category = "Revision")
    FString CandidateScreenRevision;

    UPROPERTY(BlueprintReadOnly, Category = "Commit")
    bool bCommitted = false;

    UPROPERTY(BlueprintReadOnly, Category = "Lifetime")
    bool bReleased = false;
};
