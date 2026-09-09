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
//   - Native vehicle presentation preparation, commit, read-back, and rollback.
// - Must-Not:
//   - Spawn actors, mutate source assets, save packages, or infer missing data.
// - Allows:
//   - Synchronous resolution of reviewed soft references for one transaction.
// - Split-When:
//   - Resolution, application, or rollback requires an independent lifecycle.
// - Merge-When:
//   - Another transaction owns the identical native construction behavior.
// - Summary:
//   - Applies reviewed vehicle assets to a project Chaos Pawn atomically.
// - Description:
//   - Validates exact mesh/rig/material/wheel compatibility before mutation.
// - Usage:
//   - Prepare, then commit; roll back explicitly when the owner abandons it.
// - Defaults:
//   - Any unresolved or incompatible construction input fails closed.
//

//! Compensable native Chaos vehicle construction implementation.

#include "Vehicles/SharVehicleConstructionTransaction.h"

#include "ChaosVehicleWheel.h"
#include "ChaosWheeledVehicleMovementComponent.h"
#include "Components/SkeletalMeshComponent.h"
#include "Engine/SkeletalMesh.h"
#include "Materials/MaterialInterface.h"
#include "PhysicsEngine/PhysicsAsset.h"
#include "VehicleAnimationInstance.h"
#include "Vehicles/SharVehicleDefinition.h"
#include "Vehicles/SharVehiclePawn.h"
#include "Vehicles/SharVehiclePresentationDefinition.h"

namespace
{
bool HasValidationErrors(const USharVehicleDefinition& Definition)
{
    TArray<FText> Errors;
    Definition.GatherValidationErrors(Errors);
    return !Errors.IsEmpty();
}

bool HasValidationErrors(const USharVehiclePresentationDefinition& Presentation)
{
    TArray<FText> Errors;
    Presentation.GatherValidationErrors(Errors);
    return !Errors.IsEmpty();
}
} // namespace

void USharVehicleConstructionTransaction::SetError(const FString& Message)
{
    LastError = FText::FromString(Message);
}

bool USharVehicleConstructionTransaction::Prepare(
    ASharVehiclePawn* InTargetPawn,
    USharVehicleDefinition* InDefinition,
    USharVehiclePresentationDefinition* InPresentation
)
{
    if (State != ESharVehicleConstructionState::Idle)
    {
        SetError(TEXT("vehicle construction transaction is already used"));
        return false;
    }
    if (
        InTargetPawn == nullptr
        || InDefinition == nullptr
        || InPresentation == nullptr
    )
    {
        SetError(TEXT("vehicle construction requires pawn and definitions"));
        return false;
    }
    if (
        HasValidationErrors(*InDefinition)
        || HasValidationErrors(*InPresentation)
    )
    {
        SetError(TEXT("vehicle construction definitions are invalid"));
        return false;
    }
    if (InDefinition->DefaultPresentationId != InPresentation->CanonicalId)
    {
        SetError(TEXT("vehicle default presentation identity does not match"));
        return false;
    }

    TargetPawn = InTargetPawn;
    Definition = InDefinition;
    Presentation = InPresentation;
    if (!ResolveAndValidate() || !CapturePreviousState())
    {
        TargetPawn = nullptr;
        Definition = nullptr;
        Presentation = nullptr;
        return false;
    }
    State = ESharVehicleConstructionState::Prepared;
    LastError = FText::GetEmpty();
    return true;
}

bool USharVehicleConstructionTransaction::ResolveAndValidate()
{
    check(Presentation != nullptr);
    ResolvedSkeletalMesh = Presentation->SkeletalMesh.LoadSynchronous();
    ResolvedSkeleton = Presentation->Skeleton.LoadSynchronous();
    ResolvedPhysicsAsset = Presentation->PhysicsAsset.LoadSynchronous();
    ResolvedAnimationClass = Presentation->AnimationClass.LoadSynchronous();
    if (
        ResolvedSkeletalMesh == nullptr
        || ResolvedSkeleton == nullptr
        || ResolvedPhysicsAsset == nullptr
        || ResolvedAnimationClass == nullptr
    )
    {
        SetError(TEXT("vehicle presentation native references did not load"));
        return false;
    }
    if (ResolvedSkeletalMesh->GetSkeleton() != ResolvedSkeleton)
    {
        SetError(TEXT("vehicle Skeletal Mesh uses a different Skeleton"));
        return false;
    }
    if (
        !ResolvedAnimationClass->IsChildOf(
            UVehicleAnimationInstance::StaticClass()
        )
    )
    {
        SetError(
            TEXT(
                "vehicle animation class must derive from "
                "VehicleAnimationInstance"
            )
        );
        return false;
    }

    ResolvedMaterials.Reset();
    for (
        const TSoftObjectPtr<UMaterialInterface>& Material
        : Presentation->MaterialInstances
    )
    {
        UMaterialInterface* ResolvedMaterial = Material.LoadSynchronous();
        if (ResolvedMaterial == nullptr)
        {
            SetError(TEXT("vehicle presentation material did not load"));
            return false;
        }
        ResolvedMaterials.Add(ResolvedMaterial);
    }
    if (ResolvedSkeletalMesh->GetMaterials().Num() != ResolvedMaterials.Num())
    {
        SetError(
            TEXT("vehicle material count does not match Skeletal Mesh slots")
        );
        return false;
    }
    for (
        const FSharVehicleLightPresentationBinding& Binding
        : Presentation->LightBindings
    )
    {
        if (
            ResolvedSkeletalMesh->GetRefSkeleton().FindBoneIndex(
                Binding.BoneName
            ) == INDEX_NONE
        )
        {
            SetError(FString::Printf(
                TEXT("vehicle light bone '%s' is absent from Skeletal Mesh"),
                *Binding.BoneName.ToString()
            ));
            return false;
        }
        for (const int32 SlotIndex : Binding.MaterialSlotIndices)
        {
            if (!ResolvedMaterials.IsValidIndex(SlotIndex))
            {
                SetError(TEXT("vehicle light material slot is out of range"));
                return false;
            }
        }
    }

    ResolvedWheelClasses.Reset();
    for (
        const FSharVehicleWheelPresentationBinding& Wheel
        : Presentation->Wheels
    )
    {
        UClass* WheelClass = Wheel.WheelClass.LoadSynchronous();
        if (WheelClass == nullptr)
        {
            SetError(TEXT("vehicle wheel class did not load"));
            return false;
        }
        if (
            ResolvedSkeletalMesh->GetRefSkeleton().FindBoneIndex(Wheel.BoneName)
            == INDEX_NONE
        )
        {
            SetError(FString::Printf(
                TEXT("vehicle wheel bone '%s' is absent from Skeletal Mesh"),
                *Wheel.BoneName.ToString()
            ));
            return false;
        }
        if (ResolvedPhysicsAsset->FindBodyIndex(Wheel.BoneName) == INDEX_NONE)
        {
            SetError(FString::Printf(
                TEXT("vehicle wheel bone '%s' has no Physics Asset body"),
                *Wheel.BoneName.ToString()
            ));
            return false;
        }
        ResolvedWheelClasses.Add(WheelClass);
    }
    return true;
}

bool USharVehicleConstructionTransaction::CapturePreviousState()
{
    check(TargetPawn != nullptr);
    USkeletalMeshComponent* Mesh = TargetPawn->GetMesh();
    auto* Movement = Cast<UChaosWheeledVehicleMovementComponent>(
        TargetPawn->GetVehicleMovementComponent()
    );
    if (Mesh == nullptr || Movement == nullptr)
    {
        SetError(TEXT("target Pawn does not expose Chaos wheeled components"));
        return false;
    }

    PreviousSkeletalMesh = Mesh->GetSkeletalMeshAsset();
    PreviousPhysicsAsset = Mesh->GetPhysicsAsset();
    PreviousAnimationClass = Mesh->GetAnimClass();
    PreviousAnimationMode = static_cast<uint8>(Mesh->GetAnimationMode());
    PreviousMaterials.Reset();
    for (UMaterialInterface* Material : Mesh->GetMaterials())
    {
        PreviousMaterials.Add(Material);
    }
    PreviousWheelSetups = Movement->WheelSetups;
    PreviousMassKilograms = Movement->Mass;
    PreviousMaximumEngineTorque = Movement->EngineSetup.MaxTorque;
    return true;
}

bool USharVehicleConstructionTransaction::Commit()
{
    if (State != ESharVehicleConstructionState::Prepared)
    {
        SetError(TEXT("vehicle construction must be prepared before commit"));
        return false;
    }
    if (!ApplyPreparedState() || !VerifyCommittedState())
    {
        RestorePreviousState();
        State = ESharVehicleConstructionState::RolledBack;
        if (LastError.IsEmpty())
        {
            SetError(TEXT("vehicle construction read-back failed"));
        }
        return false;
    }
    State = ESharVehicleConstructionState::Committed;
    LastError = FText::GetEmpty();
    return true;
}

bool USharVehicleConstructionTransaction::ApplyPreparedState()
{
    check(TargetPawn != nullptr);
    check(Definition != nullptr);
    check(Presentation != nullptr);
    USkeletalMeshComponent* Mesh = TargetPawn->GetMesh();
    auto* Movement = Cast<UChaosWheeledVehicleMovementComponent>(
        TargetPawn->GetVehicleMovementComponent()
    );
    if (Mesh == nullptr || Movement == nullptr)
    {
        SetError(TEXT("target Pawn lost Chaos wheeled components"));
        return false;
    }

    Mesh->SetSkeletalMesh(ResolvedSkeletalMesh);
    Mesh->SetPhysicsAsset(ResolvedPhysicsAsset, false);
    Mesh->SetAnimationMode(EAnimationMode::AnimationBlueprint);
    Mesh->SetAnimInstanceClass(ResolvedAnimationClass);
    Mesh->EmptyOverrideMaterials();
    for (int32 Index = 0; Index < ResolvedMaterials.Num(); ++Index)
    {
        Mesh->SetMaterial(Index, ResolvedMaterials[Index]);
    }

    Movement->WheelSetups.Reset(Presentation->Wheels.Num());
    for (int32 Index = 0; Index < Presentation->Wheels.Num(); ++Index)
    {
        FChaosWheelSetup Setup;
        Setup.WheelClass = ResolvedWheelClasses[Index];
        Setup.BoneName = Presentation->Wheels[Index].BoneName;
        Movement->WheelSetups.Add(Setup);
    }
    Movement->Mass = Definition->Physics.MassKilograms;
    Movement->EngineSetup.MaxTorque =
        Definition->Physics.EngineTorqueNewtonMeters;
    return true;
}

bool USharVehicleConstructionTransaction::VerifyCommittedState() const
{
    check(TargetPawn != nullptr);
    check(Definition != nullptr);
    check(Presentation != nullptr);
    USkeletalMeshComponent* Mesh = TargetPawn->GetMesh();
    const auto* Movement = Cast<UChaosWheeledVehicleMovementComponent>(
        TargetPawn->GetVehicleMovementComponent()
    );
    if (
        Mesh == nullptr
        || Movement == nullptr
        || Mesh->GetSkeletalMeshAsset() != ResolvedSkeletalMesh
        || Mesh->GetPhysicsAsset() != ResolvedPhysicsAsset
        || Mesh->GetAnimClass() != ResolvedAnimationClass
        || Movement->WheelSetups.Num() != Presentation->Wheels.Num()
        || !FMath::IsNearlyEqual(
            Movement->Mass,
            Definition->Physics.MassKilograms
        )
        || !FMath::IsNearlyEqual(
            Movement->EngineSetup.MaxTorque,
            Definition->Physics.EngineTorqueNewtonMeters
        )
    )
    {
        return false;
    }
    for (int32 Index = 0; Index < ResolvedMaterials.Num(); ++Index)
    {
        if (Mesh->GetMaterial(Index) != ResolvedMaterials[Index])
        {
            return false;
        }
    }
    for (int32 Index = 0; Index < Movement->WheelSetups.Num(); ++Index)
    {
        if (
            Movement->WheelSetups[Index].WheelClass
                != ResolvedWheelClasses[Index]
            || Movement->WheelSetups[Index].BoneName
                != Presentation->Wheels[Index].BoneName
        )
        {
            return false;
        }
    }
    return true;
}

bool USharVehicleConstructionTransaction::Rollback()
{
    if (
        State != ESharVehicleConstructionState::Prepared
        && State != ESharVehicleConstructionState::Committed
    )
    {
        SetError(
            TEXT("vehicle construction cannot roll back in current state")
        );
        return false;
    }
    RestorePreviousState();
    State = ESharVehicleConstructionState::RolledBack;
    LastError = FText::GetEmpty();
    return true;
}

void USharVehicleConstructionTransaction::RestorePreviousState()
{
    if (TargetPawn == nullptr)
    {
        return;
    }
    USkeletalMeshComponent* Mesh = TargetPawn->GetMesh();
    auto* Movement = Cast<UChaosWheeledVehicleMovementComponent>(
        TargetPawn->GetVehicleMovementComponent()
    );
    if (Mesh != nullptr)
    {
        Mesh->SetSkeletalMesh(PreviousSkeletalMesh);
        Mesh->SetPhysicsAsset(PreviousPhysicsAsset, false);
        Mesh->SetAnimInstanceClass(PreviousAnimationClass);
        Mesh->SetAnimationMode(
            static_cast<EAnimationMode::Type>(PreviousAnimationMode)
        );
        Mesh->EmptyOverrideMaterials();
        for (int32 Index = 0; Index < PreviousMaterials.Num(); ++Index)
        {
            Mesh->SetMaterial(Index, PreviousMaterials[Index]);
        }
    }
    if (Movement != nullptr)
    {
        Movement->WheelSetups = PreviousWheelSetups;
        Movement->Mass = PreviousMassKilograms;
        Movement->EngineSetup.MaxTorque = PreviousMaximumEngineTorque;
    }
}

ESharVehicleConstructionState
USharVehicleConstructionTransaction::GetState() const
{
    return State;
}

FText USharVehicleConstructionTransaction::GetLastError() const
{
    return LastError;
}
