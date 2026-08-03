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
//   - Shar progression subsystem composition module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Shar progression subsystem composition module.
// - Description:
//   - Implements the declared composition module responsibility for project.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Shar progression subsystem composition module.

#pragma once

#include "CoreMinimal.h"
#include "Subsystems/GameInstanceSubsystem.h"

#include "Progression/SharProgressionCatalogSubsystem.h"
#include "Progression/SharProgressionContracts.h"
#include "SharProgressionSubsystem.generated.h"

UCLASS()
class SHARPROGRESSION_API USharProgressionSubsystem final
    : public UGameInstanceSubsystem
{
    GENERATED_BODY()

public:
    UFUNCTION(BlueprintCallable, Category = "SHAR|Progression")
    bool Configure(
        USharProgressionCatalogSubsystem* InCatalogSubsystem,
        const FSharProgressionSnapshot& InitialSnapshot
    );

    UFUNCTION(BlueprintCallable, Category = "SHAR|Progression")
    ESharProgressionMutationResult Submit(
        const FSharProgressionMutationRequest& OperationSpec
    );

    ESharProgressionMutationResult Begin(const FName& MutationId);

    ESharProgressionMutationResult Prepare(const FName& MutationId);

    ESharProgressionMutationResult AcceptCommitEvidence(
        const FSharProgressionCommitEvidence& Evidence
    );

    ESharProgressionMutationResult Resolve(
        const FSharProgressionMutationResolution& Resolution
    );

    ESharProgressionMutationResult Release(const FName& MutationId);

    UFUNCTION(BlueprintPure, Category = "SHAR|Progression")
    [[nodiscard]] int32 GetQueuePosition(const FName& MutationId) const;

    UFUNCTION(BlueprintPure, Category = "SHAR|Progression")
    [[nodiscard]] ESharProgressionMutationState GetMutationState(
        const FName& MutationId
    ) const;

    UFUNCTION(BlueprintPure, Category = "SHAR|Progression")
    [[nodiscard]] ESharProgressionTerminalResult GetTerminalResult(
        const FName& MutationId
    ) const;

    UFUNCTION(BlueprintPure, Category = "SHAR|Progression")
    [[nodiscard]] FSharProgressionObservation GetObservation() const;

    UFUNCTION(BlueprintPure, Category = "SHAR|Progression")
    [[nodiscard]] int32 GetQuantity(
        const FName& OperationId,
        const FName& TargetId
    ) const;

    UFUNCTION(BlueprintPure, Category = "SHAR|Progression")
    [[nodiscard]] bool HasAppliedTransaction(
        const FName& TransactionId
    ) const;

    [[nodiscard]] bool ProjectCount(
        const FSharProgressionCountQuery& Query,
        FSharProgressionCountProjection& OutProjection
    ) const;

private:
    UPROPERTY(Transient)
    USharProgressionCatalogSubsystem* CatalogSubsystem = nullptr;

    UPROPERTY(Transient)
    ESharProfileLifecycleState ProfileState =
        ESharProfileLifecycleState::Unconfigured;

    UPROPERTY(Transient)
    FSharProgressionSnapshot ActiveSnapshot;

    UPROPERTY(Transient)
    TArray<FSharProgressionMutationSnapshot> Mutations;

    UPROPERTY(Transient)
    int32 NextInsertionSequence = 0;

    [[nodiscard]] static bool IsCanonicalIdentity(const FName& Candidate);
    [[nodiscard]] static bool IsRevisionToken(const FString& Revision);
    [[nodiscard]] static bool ProfileIdentitiesMatch(
        const FSharProfileIdentity& Left,
        const FSharProfileIdentity& Right
    );
    [[nodiscard]] static bool IsTerminalState(
        ESharProgressionMutationState State
    );
    [[nodiscard]] static bool Outranks(
        const FSharProgressionMutationSnapshot& Left,
        const FSharProgressionMutationSnapshot& Right
    );
    [[nodiscard]] bool ValidateSnapshot(
        const FSharProgressionSnapshot& Snapshot
    ) const;
    [[nodiscard]] ESharProgressionMutationResult ValidateOperationSpec(
        const FSharProgressionMutationRequest& OperationSpec
    ) const;
    [[nodiscard]] static ESharProgressionMutationResult ValidateOperation(
        const FSharRewardRequest& Operation,
        const USharProgressionCatalogDefinition& Catalog
    );
    [[nodiscard]] ESharProgressionMutationResult ValidateRuntimeState() const;
    [[nodiscard]] static bool HasValidOperationSpecIdentity(
        const FSharProgressionMutationRequest& OperationSpec
    );
    [[nodiscard]] bool OperationSpecMatchesActiveSnapshot(
        const FSharProgressionMutationRequest& OperationSpec
    ) const;
    [[nodiscard]] static ESharProgressionMutationResult ValidateOperationBatch(
        const FSharProgressionMutationRequest& OperationSpec,
        const USharProgressionCatalogDefinition& Catalog
    );
    [[nodiscard]] FSharProgressionMutationSnapshot* FindMutation(
        const FName& MutationId
    );
    [[nodiscard]] const FSharProgressionMutationSnapshot* FindMutation(
        const FName& MutationId
    ) const;
    [[nodiscard]] bool IsHead(
        const FSharProgressionMutationSnapshot& Mutation
    ) const;
    [[nodiscard]] bool HasActiveMutation(
        const FSharProgressionMutationSnapshot& Mutation
    ) const;
    [[nodiscard]] int32 CountUnreleasedMutations() const;
    [[nodiscard]] ESharProgressionMutationResult BuildCandidate(
        FSharProgressionMutationSnapshot& Mutation
    ) const;
    [[nodiscard]] static bool CommitEvidenceMatches(
        const FSharProgressionMutationSnapshot& Mutation,
        const FSharProgressionCommitEvidence& Evidence
    );
    [[nodiscard]] static ESharProgressionMutationResult PublishTerminal(
        FSharProgressionMutationSnapshot& Mutation,
        ESharProgressionMutationState State,
        ESharProgressionTerminalResult Result
    );
};
