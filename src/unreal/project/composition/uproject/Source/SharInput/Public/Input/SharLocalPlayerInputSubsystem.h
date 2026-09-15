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
//   - Per-local-player SHAR mapping-context lease lifecycle.
// - Must-Not:
//   - Own application modes, physical device discovery, or gameplay actions.
// - Allows:
//   - Staging, committing, observing, and releasing Enhanced Input contexts.
// - Split-When:
//   - Device assignment or rebinding gains an independent lifecycle.
// - Merge-When:
//   - Another local-player subsystem owns identical mapping-context leases.
// - Summary:
//   - Defines the local-player semantic input lease authority.
// - Description:
//   - Activates mappings only on commit and pairs every active lease with
//   - release.
// - Usage:
//   - Auto-created once for each ULocalPlayer.
// - Defaults:
//   - Invalid, stale, or unavailable input dependencies fail closed.
//

//! Per-local-player mapping-context lease authority.

// CSpell:ignore SHARINPUT

#pragma once

#include "CoreMinimal.h"
#include "Subsystems/LocalPlayerSubsystem.h"

#include "SharLocalPlayerInputSubsystem.generated.h"

class UEnhancedInputLocalPlayerSubsystem;
class UInputMappingContext;

UENUM(BlueprintType)
enum class ESharInputContextLeaseState : uint8
{
    Staged,
    Active,
    Released,
};

UENUM(BlueprintType)
enum class ESharInputContextLeaseResult : uint8
{
    Accepted,
    InvalidRequest,
    DuplicateLease,
    MappingContextInUse,
    NotFound,
    StaleRevision,
    EnhancedInputUnavailable,
    InvalidState,
    AlreadyReleased,
    MappingApplyFailed,
};

USTRUCT(BlueprintType)
struct SHARINPUT_API FSharInputContextLeaseRequest
{
    GENERATED_BODY()

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Identity")
    FName LeaseId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Identity")
    FName ContextId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Owner")
    FName OwnerModeId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString OwnerModeRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString TransitionRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString LeaseRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Mapping")
    TObjectPtr<UInputMappingContext> MappingContext = nullptr;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Mapping")
    int32 Priority = 0;
};

USTRUCT(BlueprintType)
struct SHARINPUT_API FSharInputContextLeaseObservation
{
    GENERATED_BODY()

    UPROPERTY(BlueprintReadOnly, Category = "Identity")
    FName LeaseId;

    UPROPERTY(BlueprintReadOnly, Category = "Identity")
    FName ContextId;

    UPROPERTY(BlueprintReadOnly, Category = "Owner")
    FName OwnerModeId;

    UPROPERTY(BlueprintReadOnly, Category = "Revision")
    FString OwnerModeRevision;

    UPROPERTY(BlueprintReadOnly, Category = "Revision")
    FString TransitionRevision;

    UPROPERTY(BlueprintReadOnly, Category = "Revision")
    FString LeaseRevision;

    UPROPERTY(BlueprintReadOnly, Category = "Mapping")
    int32 Priority = 0;

    UPROPERTY(BlueprintReadOnly, Category = "State")
    ESharInputContextLeaseState State = ESharInputContextLeaseState::Staged;
};

USTRUCT()
struct FSharInputContextLeaseRecord
{
    GENERATED_BODY()

    UPROPERTY(Transient)
    FSharInputContextLeaseRequest Request;

    UPROPERTY(Transient)
    ESharInputContextLeaseState State = ESharInputContextLeaseState::Staged;
};

UCLASS()
class SHARINPUT_API USharLocalPlayerInputSubsystem final
    : public ULocalPlayerSubsystem
{
    GENERATED_BODY()

public:
    virtual void Initialize(FSubsystemCollectionBase& Collection) override;
    virtual void Deinitialize() override;

    UFUNCTION(BlueprintCallable, Category = "SHAR|Input")
    ESharInputContextLeaseResult StageLease(
        const FSharInputContextLeaseRequest& Request
    );

    UFUNCTION(BlueprintPure, Category = "SHAR|Input")
    ESharInputContextLeaseResult CheckCommitReadiness(
        const FName& LeaseId,
        const FString& LeaseRevision,
        const FString& TransitionRevision
    ) const;

    UFUNCTION(BlueprintCallable, Category = "SHAR|Input")
    ESharInputContextLeaseResult CommitLease(
        const FName& LeaseId,
        const FString& LeaseRevision,
        const FString& TransitionRevision
    );

    UFUNCTION(BlueprintCallable, Category = "SHAR|Input")
    ESharInputContextLeaseResult ReleaseLease(
        const FName& LeaseId,
        const FString& LeaseRevision
    );

    UFUNCTION(BlueprintPure, Category = "SHAR|Input")
    bool GetLeaseObservation(
        const FName& LeaseId,
        FSharInputContextLeaseObservation& OutObservation
    ) const;

    UFUNCTION(BlueprintPure, Category = "SHAR|Input")
    int32 GetActiveLeaseCount() const;

    UFUNCTION(BlueprintPure, Category = "SHAR|Input")
    int32 GetStagedLeaseCount() const;

private:
    UPROPERTY(Transient)
    TArray<FSharInputContextLeaseRecord> Leases;

    [[nodiscard]] FSharInputContextLeaseRecord* FindLease(
        const FName& LeaseId
    );
    [[nodiscard]] const FSharInputContextLeaseRecord* FindLease(
        const FName& LeaseId
    ) const;
    [[nodiscard]] UEnhancedInputLocalPlayerSubsystem* GetEnhancedInput() const;
    [[nodiscard]] bool IsMappingOwned(
        const UInputMappingContext* Context
    ) const;
    [[nodiscard]] static bool IsValidRequest(
        const FSharInputContextLeaseRequest& Request
    );
    void ReleaseActiveLeasesForTeardown();
};
