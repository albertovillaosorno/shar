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
//   - Shar vehicle selection transaction composition module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Shar vehicle selection transaction composition module.
// - Description:
//   - Implements the declared composition module responsibility for project.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Shar vehicle selection transaction composition module.

#pragma once

#include "CoreMinimal.h"
#include "Engine/DataAsset.h"

#include "SharVehicleSelectionTransaction.generated.h"

UENUM(BlueprintType)
enum class ESharVehicleSelectionState : uint8
{
    Idle,
    Requested,
    SpawnReserved,
    Committed,
    RolledBack,
};

UCLASS(BlueprintType)
class SHARVEHICLES_API USharVehicleSelectionTransaction final : public UObject
{
    GENERATED_BODY()

public:
    UFUNCTION(BlueprintCallable, Category = "SHAR|Vehicle Selection")
    bool Begin(
        const FPrimaryAssetId& InPreviousVehicleId,
        const FPrimaryAssetId& InRequestedVehicleId
    );

    UFUNCTION(BlueprintCallable, Category = "SHAR|Vehicle Selection")
    bool MarkSpawnReserved(const FName& InReservationId);

    UFUNCTION(BlueprintCallable, Category = "SHAR|Vehicle Selection")
    bool Commit();

    UFUNCTION(BlueprintCallable, Category = "SHAR|Vehicle Selection")
    bool Rollback();

    UFUNCTION(BlueprintPure, Category = "SHAR|Vehicle Selection")
    [[nodiscard]] ESharVehicleSelectionState GetState() const;

    UFUNCTION(BlueprintPure, Category = "SHAR|Vehicle Selection")
    [[nodiscard]] FPrimaryAssetId GetPreviousVehicleId() const;

    UFUNCTION(BlueprintPure, Category = "SHAR|Vehicle Selection")
    [[nodiscard]] FPrimaryAssetId GetRequestedVehicleId() const;

    UFUNCTION(BlueprintPure, Category = "SHAR|Vehicle Selection")
    [[nodiscard]] FName GetReservationId() const;

    UFUNCTION(BlueprintPure, Category = "SHAR|Vehicle Selection")
    [[nodiscard]] FText GetLastError() const;

private:
    UPROPERTY(Transient)
    ESharVehicleSelectionState State = ESharVehicleSelectionState::Idle;

    UPROPERTY(Transient)
    FPrimaryAssetId PreviousVehicleId;

    UPROPERTY(Transient)
    FPrimaryAssetId RequestedVehicleId;

    UPROPERTY(Transient)
    FName ReservationId;

    UPROPERTY(Transient)
    FText LastError;

    void SetError(const TCHAR* Message);
};
