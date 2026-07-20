// File: SharActionResourceArbiter.h
// Path: src/uproject/Source/SharAction/Public/Action/SharActionResourceArbiter.h
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: typed shared and exclusive action-resource leases only; no task scheduling or domain mutation.
// ADR: docs/adr/unreal/runtime/typed-state-tree-action-sequences.md
// LARGE-FILE owner=SharAction; reason=cohesive reflected resource-request and lease state contract;
// split=extract diagnostics if lease history becomes persistent;
// validation=validate.sh SharAction plus Unreal automation; review=2027-01.

#pragma once

#include "Action/SharActionDefinition.h"
#include "CoreMinimal.h"

#include "SharActionResourceArbiter.generated.h"

UENUM(BlueprintType)
enum class ESharActionLeaseResult : uint8
{
    Granted,
    InvalidRequest,
    DuplicateLease,
    ResourceConflict,
    NotFound,
    Released,
};

USTRUCT(BlueprintType)
struct SHARACTION_API FSharActionResourceRequest
{
    GENERATED_BODY()

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Identity")
    FName LeaseId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Identity")
    FName OwnerId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Identity")
    FName ResourceId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString OwnerRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Resource")
    ESharActionResourceAccess Access = ESharActionResourceAccess::Exclusive;
};

USTRUCT(BlueprintType)
struct SHARACTION_API FSharActionLeaseState
{
    GENERATED_BODY()

    UPROPERTY(BlueprintReadOnly, Category = "Identity")
    FName LeaseId;

    UPROPERTY(BlueprintReadOnly, Category = "Identity")
    FName OwnerId;

    UPROPERTY(BlueprintReadOnly, Category = "Identity")
    FName ResourceId;

    UPROPERTY(BlueprintReadOnly, Category = "Revision")
    FString OwnerRevision;

    UPROPERTY(BlueprintReadOnly, Category = "Resource")
    ESharActionResourceAccess Access = ESharActionResourceAccess::Exclusive;

    UPROPERTY(BlueprintReadOnly, Category = "Resource")
    bool bActive = false;
};

UCLASS(BlueprintType)
class SHARACTION_API USharActionResourceArbiter final : public UObject
{
    GENERATED_BODY()

public:
    UFUNCTION(BlueprintCallable, Category = "SHAR|Action")
    ESharActionLeaseResult Acquire(
        const FSharActionResourceRequest& Request
    );

    UFUNCTION(BlueprintCallable, Category = "SHAR|Action")
    ESharActionLeaseResult Release(const FName& LeaseId);

    UFUNCTION(BlueprintCallable, Category = "SHAR|Action")
    int32 ReleaseOwner(const FName& OwnerId);

    UFUNCTION(BlueprintPure, Category = "SHAR|Action")
    [[nodiscard]] bool IsAvailable(
        const FName& ResourceId,
        ESharActionResourceAccess RequestedAccess
    ) const;

    UFUNCTION(BlueprintPure, Category = "SHAR|Action")
    [[nodiscard]] bool IsLeaseActive(const FName& LeaseId) const;

    UFUNCTION(BlueprintPure, Category = "SHAR|Action")
    [[nodiscard]] int32 GetActiveLeaseCount() const;

private:
    UPROPERTY(Transient)
    TArray<FSharActionLeaseState> Leases;

    [[nodiscard]] FSharActionLeaseState* FindLease(const FName& LeaseId);
    [[nodiscard]] const FSharActionLeaseState* FindLease(
        const FName& LeaseId
    ) const;
    [[nodiscard]] static bool IsValidRequest(
        const FSharActionResourceRequest& Request
    );
};
