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
//   - Prepared, committed, and compensated vehicle presentation construction.
// - Must-Not:
//   - Spawn Pawns, select content, save assets, or invent missing source data.
// - Allows:
//   - Loading reviewed presentation references and configuring one target Pawn.
// - Split-When:
//   - Asset resolution or simulation startup gains an independent lifecycle.
// - Merge-When:
//   - Another transaction owns the identical Pawn construction lifecycle.
// - Summary:
//   - Applies one validated vehicle definition and presentation atomically.
// - Description:
//   - Resolves native assets, verifies compatibility, and supports rollback.
// - Usage:
//   - Prepare against a target Pawn, then commit or roll back explicitly.
// - Defaults:
//   - Invalid references or incompatible native assets fail closed.
//

//! Compensable native Chaos vehicle construction transaction.

#pragma once

#include "ChaosWheeledVehicleMovementComponent.h"
#include "CoreMinimal.h"
#include "UObject/Object.h"

#include "SharVehicleConstructionTransaction.generated.h"

class ASharVehiclePawn;
class UAnimInstance;
class UChaosVehicleWheel;
class UMaterialInterface;
class UPhysicsAsset;
class USkeletalMesh;
class USkeleton;
class USharVehicleDefinition;
class USharVehiclePresentationDefinition;

UENUM(BlueprintType)
enum class ESharVehicleConstructionState : uint8
{
    Idle,
    Prepared,
    Committed,
    RolledBack,
};

UCLASS(BlueprintType)
class SHARVEHICLES_API USharVehicleConstructionTransaction final
    : public UObject
{
    GENERATED_BODY()

public:
    UFUNCTION(BlueprintCallable, Category = "SHAR|Vehicle Construction")
    bool Prepare(
        ASharVehiclePawn* InTargetPawn,
        USharVehicleDefinition* InDefinition,
        USharVehiclePresentationDefinition* InPresentation
    );

    UFUNCTION(BlueprintCallable, Category = "SHAR|Vehicle Construction")
    bool Commit();

    UFUNCTION(BlueprintCallable, Category = "SHAR|Vehicle Construction")
    bool Rollback();

    UFUNCTION(BlueprintPure, Category = "SHAR|Vehicle Construction")
    [[nodiscard]] ESharVehicleConstructionState GetState() const;

    UFUNCTION(BlueprintPure, Category = "SHAR|Vehicle Construction")
    [[nodiscard]] FText GetLastError() const;

private:
    UPROPERTY(Transient)
    ESharVehicleConstructionState State = ESharVehicleConstructionState::Idle;

    UPROPERTY(Transient)
    TObjectPtr<ASharVehiclePawn> TargetPawn;

    UPROPERTY(Transient)
    TObjectPtr<USharVehicleDefinition> Definition;

    UPROPERTY(Transient)
    TObjectPtr<USharVehiclePresentationDefinition> Presentation;

    UPROPERTY(Transient)
    TObjectPtr<USkeletalMesh> ResolvedSkeletalMesh;

    UPROPERTY(Transient)
    TObjectPtr<USkeleton> ResolvedSkeleton;

    UPROPERTY(Transient)
    TObjectPtr<UPhysicsAsset> ResolvedPhysicsAsset;

    UPROPERTY(Transient)
    TObjectPtr<UClass> ResolvedAnimationClass;

    UPROPERTY(Transient)
    TArray<TObjectPtr<UMaterialInterface>> ResolvedMaterials;

    UPROPERTY(Transient)
    TArray<TSubclassOf<UChaosVehicleWheel>> ResolvedWheelClasses;

    UPROPERTY(Transient)
    TObjectPtr<USkeletalMesh> PreviousSkeletalMesh;

    UPROPERTY(Transient)
    TObjectPtr<UPhysicsAsset> PreviousPhysicsAsset;

    UPROPERTY(Transient)
    TObjectPtr<UClass> PreviousAnimationClass;

    UPROPERTY(Transient)
    TArray<TObjectPtr<UMaterialInterface>> PreviousMaterials;

    UPROPERTY(Transient)
    TArray<FChaosWheelSetup> PreviousWheelSetups;

    UPROPERTY(Transient)
    uint8 PreviousAnimationMode = 0;

    UPROPERTY(Transient)
    float PreviousMassKilograms = 0.0F;

    UPROPERTY(Transient)
    float PreviousMaximumEngineTorque = 0.0F;

    UPROPERTY(Transient)
    FText LastError;

    bool ResolveAndValidate();
    bool CapturePreviousState();
    bool ApplyPreparedState();
    bool VerifyCommittedState() const;
    void RestorePreviousState();
    void SetError(const FString& Message);
};
