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
//   - Shar world readiness subsystem composition module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Shar world readiness subsystem composition module.
// - Description:
//   - Implements the declared composition module responsibility for project.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Shar world readiness subsystem composition module.

#pragma once

#include "CoreMinimal.h"
#include "Subsystems/WorldSubsystem.h"

#include "SharWorldReadinessSubsystem.generated.h"

UENUM(BlueprintType)
enum class ESharWorldReadinessResult : uint8
{
    Accepted,
    InvalidRequest,
    BarrierMissing,
    DuplicateBarrier,
    CheckpointMissing,
    DuplicateCheckpoint,
    StaleWorld,
    AlreadyReady,
    NotReady,
};

USTRUCT(BlueprintType)
struct SHARLOADING_API FSharWorldReadinessBarrier
{
    GENERATED_BODY()

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Identity")
    FName BarrierId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Identity")
    FName WorldId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString WorldRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Checkpoint")
    TArray<FName> RequiredCheckpointIds;
};

USTRUCT(BlueprintType)
struct SHARLOADING_API FSharWorldCheckpointCompletion
{
    GENERATED_BODY()

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Identity")
    FName BarrierId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Identity")
    FName CheckpointId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString WorldRevision;
};

USTRUCT(BlueprintType)
struct SHARLOADING_API FSharWorldReadinessSnapshot
{
    GENERATED_BODY()

    UPROPERTY(BlueprintReadOnly, Category = "Identity")
    FName BarrierId;

    UPROPERTY(BlueprintReadOnly, Category = "Identity")
    FName WorldId;

    UPROPERTY(BlueprintReadOnly, Category = "Revision")
    FString WorldRevision;

    UPROPERTY(BlueprintReadOnly, Category = "Checkpoint")
    TArray<FName> RequiredCheckpointIds;

    UPROPERTY(BlueprintReadOnly, Category = "Checkpoint")
    TArray<FName> CompletedCheckpointIds;

    UPROPERTY(BlueprintReadOnly, Category = "State")
    bool bReady = false;

    UPROPERTY(BlueprintReadOnly, Category = "State")
    int32 Revision = 0;
};

UCLASS()
class SHARLOADING_API USharWorldReadinessSubsystem final
    : public UWorldSubsystem
{
    GENERATED_BODY()

public:
    UFUNCTION(BlueprintCallable, Category = "SHAR|Loading")
    bool ConfigureWorld(const FName& InWorldId, const FString& InWorldRevision);

    UFUNCTION(BlueprintCallable, Category = "SHAR|Loading")
    ESharWorldReadinessResult RegisterBarrier(
        const FSharWorldReadinessBarrier& Barrier
    );

    UFUNCTION(BlueprintCallable, Category = "SHAR|Loading")
    ESharWorldReadinessResult CompleteCheckpoint(
        const FSharWorldCheckpointCompletion& Completion
    );

    UFUNCTION(BlueprintPure, Category = "SHAR|Loading")
    [[nodiscard]] bool IsReady(const FName& BarrierId) const;

    UFUNCTION(BlueprintPure, Category = "SHAR|Loading")
    [[nodiscard]] int32 GetCompletedCheckpointCount(
        const FName& BarrierId
    ) const;

    UFUNCTION(BlueprintPure, Category = "SHAR|Loading")
    [[nodiscard]] int32 GetRequiredCheckpointCount(
        const FName& BarrierId
    ) const;

    UFUNCTION(BlueprintCallable, Category = "SHAR|Loading")
    int32 TeardownWorld();

private:
    UPROPERTY(Transient)
    FName WorldId;

    UPROPERTY(Transient)
    FString WorldRevision;

    UPROPERTY(Transient)
    TArray<FSharWorldReadinessSnapshot> Barriers;

    [[nodiscard]] FSharWorldReadinessSnapshot* FindBarrier(
        const FName& BarrierId
    );
    [[nodiscard]] const FSharWorldReadinessSnapshot* FindBarrier(
        const FName& BarrierId
    ) const;
    [[nodiscard]] static bool IsRevisionToken(const FString& Revision);
    [[nodiscard]] static bool IsValidBarrier(
        const FSharWorldReadinessBarrier& Barrier
    );
    static void RefreshReady(FSharWorldReadinessSnapshot& Snapshot);
};
