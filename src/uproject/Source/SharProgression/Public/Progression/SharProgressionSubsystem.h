// File: SharProgressionSubsystem.h
// Path: src/uproject/Source/SharProgression/Public/Progression/SharProgressionSubsystem.h
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: game-instance active profile, revisioned progression mutation, commit correlation, and immutable projection authority only; save I/O, platform accounts, UI, and gameplay execution remain external.
// Specification: docs/technical/unreal/progression-collectibles-and-cheats.md
// LARGE-FILE owner=SharProgression; reason=cohesive profile and progression control-plane API;
// split=extract campaign projection queries when multiple domain snapshots are joined;
// validation=validate.sh SharProgression plus Unreal automation; review=2027-01.

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
