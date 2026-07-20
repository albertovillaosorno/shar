// File: SharWorldReadinessSubsystem.h
// Path: src/uproject/Source/SharLoading/Public/Loading/SharWorldReadinessSubsystem.h
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: world-revision and required-checkpoint barriers only; streaming visibility, actors, physics, and gameplay activation remain external.
// Specification: docs/technical/unreal/native-asset-load-request-and-streaming-runtime.md
// LARGE-FILE owner=SharLoading; reason=cohesive reflected world-readiness barrier contract;
// split=extract diagnostics if checkpoint evidence becomes persistent;
// validation=validate.sh SharLoading plus Unreal automation; review=2027-01.

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
