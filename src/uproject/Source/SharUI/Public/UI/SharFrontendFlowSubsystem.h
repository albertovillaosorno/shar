// File: SharFrontendFlowSubsystem.h
// Path: src/uproject/Source/SharUI/Public/UI/SharFrontendFlowSubsystem.h
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: game-instance frontend navigation arbitration, readiness fencing, history, modal state, rollback, and terminal results only.
// Specification: docs/technical/unreal/frontend-screen-flow-and-settings-runtime.md

#pragma once

#include "CoreMinimal.h"
#include "Subsystems/GameInstanceSubsystem.h"

#include "UI/SharFrontendCatalogSubsystem.h"
// NOLINTNEXTLINE(llvm-include-order) -- Unreal requires the generated header last.
#include "UI/SharFrontendFlowContracts.h"
#include "SharFrontendFlowSubsystem.generated.h"

UCLASS()
class SHARUI_API USharFrontendFlowSubsystem final
    : public UGameInstanceSubsystem
{
    GENERATED_BODY()

public:
    UFUNCTION(BlueprintCallable, Category = "SHAR|Frontend")
    bool Configure(
        USharFrontendCatalogSubsystem* InCatalog,
        const FString& InitialFlowRevision,
        const FString& InitialScreenRevision,
        const FName& InitialFocusTargetId
    );

    UFUNCTION(BlueprintCallable, Category = "SHAR|Frontend")
    ESharFrontendOperationResult Submit(
        const FSharFrontendNavigationRequest& Request
    );

    UFUNCTION(BlueprintCallable, Category = "SHAR|Frontend")
    ESharFrontendOperationResult Begin(const FName& RequestId);

    UFUNCTION(BlueprintCallable, Category = "SHAR|Frontend")
    ESharFrontendOperationResult AcceptEvidence(
        const FSharFrontendReadinessEvidence& Evidence
    );

    UFUNCTION(BlueprintCallable, Category = "SHAR|Frontend")
    ESharFrontendOperationResult Commit(const FName& RequestId);

    UFUNCTION(BlueprintCallable, Category = "SHAR|Frontend")
    ESharFrontendOperationResult Resolve(
        const FSharFrontendTransitionResolution& Resolution
    );

    UFUNCTION(BlueprintCallable, Category = "SHAR|Frontend")
    ESharFrontendOperationResult Release(const FName& RequestId);

    UFUNCTION(BlueprintCallable, Category = "SHAR|Frontend")
    ESharFrontendOperationResult UpdateStableFocus(
        const FName& FocusTargetId,
        const FString& ExpectedFlowRevision
    );

    UFUNCTION(BlueprintPure, Category = "SHAR|Frontend")
    [[nodiscard]] bool IsConfigured() const;

    UFUNCTION(BlueprintPure, Category = "SHAR|Frontend")
    [[nodiscard]] int32 GetQueueCount() const;

    UFUNCTION(BlueprintPure, Category = "SHAR|Frontend")
    [[nodiscard]] FName GetHeadRequestId() const;

    UFUNCTION(BlueprintPure, Category = "SHAR|Frontend")
    [[nodiscard]] int32 GetQueuePosition(const FName& RequestId) const;

    [[nodiscard]] const FSharFrontendFlowObservation& GetObservation() const;

    [[nodiscard]] const FSharFrontendTransitionSnapshot* FindTransition(
        const FName& RequestId
    ) const;

    [[nodiscard]] ESharFrontendTerminalResult GetTerminalResult(
        const FName& RequestId
    ) const;

private:
    UPROPERTY(Transient)
    TObjectPtr<USharFrontendCatalogSubsystem> Catalog = nullptr;

    UPROPERTY(Transient)
    FSharFrontendFlowObservation Observation;

    UPROPERTY(Transient)
    TArray<FSharFrontendTransitionSnapshot> Transitions;

    UPROPERTY(Transient)
    int32 NextInsertionSequence = 1;

    UPROPERTY(Transient)
    int64 NextFlowRevisionOrdinal = 1;

    UPROPERTY(Transient)
    int64 NextScreenRevisionOrdinal = 1;

    UPROPERTY(Transient)
    bool bConfigured = false;

    [[nodiscard]] FName GetEffectiveScreenId() const;
    [[nodiscard]] FString MakeNextFlowRevision();
    [[nodiscard]] FString MakeNextScreenRevision();
    [[nodiscard]] FSharFrontendTransitionSnapshot* FindMutableTransition(
        const FName& RequestId
    );
    [[nodiscard]] const FSharFrontendScreenDefinition* GetDestinationDefinition(
        const FSharFrontendTransitionSnapshot& Transition
    ) const;
    [[nodiscard]] bool IsHead(const FName& RequestId) const;
    [[nodiscard]] static bool HasEvidence(
        const FSharFrontendTransitionSnapshot& Transition,
        ESharFrontendReadinessKind Kind
    );
    [[nodiscard]] static bool RequirementsSatisfied(
        const FSharFrontendTransitionSnapshot& Transition,
        const TArray<ESharFrontendReadinessKind>& Requirements
    );
    [[nodiscard]] ESharFrontendOperationResult ValidateRequest(
        const FSharFrontendNavigationRequest& Request
    ) const;
    [[nodiscard]] ESharFrontendOperationResult ValidateRequestEnvelope(
        const FSharFrontendNavigationRequest& Request
    ) const;
    [[nodiscard]] ESharFrontendOperationResult ValidateRequestRoute(
        const FSharFrontendNavigationRequest& Request
    ) const;
    [[nodiscard]] ESharFrontendOperationResult ValidateRequestHistory(
        const FSharFrontendNavigationRequest& Request
    ) const;
    [[nodiscard]] ESharFrontendOperationResult ValidateModalHistory(
        const FSharFrontendNavigationRequest& Request,
        const FSharFrontendScreenDefinition& Destination
    ) const;
    [[nodiscard]] ESharFrontendOperationResult ValidatePrimaryHistory(
        const FSharFrontendNavigationRequest& Request
    ) const;
    [[nodiscard]] ESharFrontendOperationResult ValidateEvidence(
        const FSharFrontendTransitionSnapshot& Transition,
        const FSharFrontendReadinessEvidence& Evidence
    ) const;
    [[nodiscard]] ESharFrontendOperationResult ValidateCommitSubmission(
        const FSharFrontendTransitionSnapshot* Transition,
        const FSharFrontendScreenDefinition* Destination,
        const FName& RequestId
    ) const;
    ESharFrontendOperationResult ApplyNavigationCommit(
        FSharFrontendTransitionSnapshot& Transition,
        const FSharFrontendScreenDefinition& Destination
    );
    ESharFrontendOperationResult ApplyPrimaryHistory(
        const FSharFrontendNavigationRequest& Request,
        const FName& DestinationScreenId
    );
    [[nodiscard]] ESharFrontendOperationResult ValidateEvidenceSubmission(
        const FSharFrontendTransitionSnapshot* Transition,
        const FSharFrontendReadinessEvidence& Evidence
    ) const;
    ESharFrontendOperationResult ApplyAcceptedEvidence(
        FSharFrontendTransitionSnapshot& Transition,
        const FSharFrontendScreenDefinition& Destination,
        const FSharFrontendReadinessEvidence& Evidence
    );
    void SortQueue();
    void CompleteSuccess(FSharFrontendTransitionSnapshot& Transition);
    void CompleteTerminal(
        FSharFrontendTransitionSnapshot& Transition,
        ESharFrontendTerminalResult Result,
        ESharFrontendTransitionState State
    );
    void RestorePriorObservation(
        const FSharFrontendTransitionSnapshot& Transition
    );
};
