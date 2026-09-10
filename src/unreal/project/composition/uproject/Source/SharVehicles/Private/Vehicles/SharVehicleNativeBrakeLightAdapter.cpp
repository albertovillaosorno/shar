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
//   - Native brake emitter construction, attachment, refresh, and teardown.
// - Must-Not:
//   - Infer source tuning, regroup hardpoints, or create glow billboards.
// - Allows:
//   - Convert reviewed brake hardpoints into transient spot light components.
// - Split-When:
//   - Reverse-light output requires a distinct native component policy.
// - Merge-When:
//   - Vehicle construction owns the identical transient presentation lifecycle.
// - Summary:
//   - Implements semantic-to-native vehicle brake-light output.
// - Description:
//   - Creates one spot light per authored brake binding and exact bone anchor.
// - Usage:
//   - Called after native vehicle construction has installed the presentation.
// - Defaults:
//   - Invalid, incomplete, or mismatched configuration fails without mutation.
//

//! Semantic vehicle brake state to native Unreal spot light output.

#include "Vehicles/SharVehicleNativeBrakeLightAdapter.h"

#include "Vehicles/SharVehiclePawn.h"
#include "Vehicles/SharVehiclePresentationDefinition.h"
#include "Vehicles/SharVehiclePresentationState.h"

#include "Components/SkeletalMeshComponent.h"
#include "Components/SpotLightComponent.h"
#include "Engine/EngineTypes.h"

namespace
{
bool IsFiniteBrakeColor(const FLinearColor& Color)
{
    return FMath::IsFinite(Color.R)
        && FMath::IsFinite(Color.G)
        && FMath::IsFinite(Color.B)
        && FMath::IsFinite(Color.A);
}

bool IsValidBrakeConfiguration(
    const FSharVehicleNativeBrakeLightConfiguration& Configuration
)
{
    const FVector& Direction = Configuration.BoneLocalDirection;
    const bool bDirectionValid =
        FMath::IsFinite(Direction.X)
        && FMath::IsFinite(Direction.Y)
        && FMath::IsFinite(Direction.Z)
        && !Direction.IsNearlyZero();
    const bool bColorValid =
        IsFiniteBrakeColor(Configuration.LightColor)
        && Configuration.LightColor.R >= 0.0F
        && Configuration.LightColor.G >= 0.0F
        && Configuration.LightColor.B >= 0.0F
        && Configuration.LightColor.GetMax() > 0.0F;
    return FMath::IsFinite(Configuration.IntensityLumens)
        && Configuration.IntensityLumens > 0.0F
        && FMath::IsFinite(Configuration.AttenuationRadiusCentimeters)
        && Configuration.AttenuationRadiusCentimeters > 0.0F
        && FMath::IsFinite(Configuration.InnerConeAngleDegrees)
        && Configuration.InnerConeAngleDegrees >= 0.0F
        && FMath::IsFinite(Configuration.OuterConeAngleDegrees)
        && Configuration.OuterConeAngleDegrees
            > Configuration.InnerConeAngleDegrees
        && Configuration.OuterConeAngleDegrees < 90.0F
        && bDirectionValid
        && bColorValid;
}
} // namespace

bool USharVehicleNativeBrakeLightAdapter::ConfigureBrakeLights(
    ASharVehiclePawn* InTargetPawn,
    USharVehiclePresentationDefinition* InPresentation,
    USharVehiclePresentationState* InState,
    const FSharVehicleNativeBrakeLightConfiguration& InConfiguration
)
{
    if (
        InTargetPawn == nullptr
        || InPresentation == nullptr
        || InState == nullptr
        || !IsValidBrakeConfiguration(InConfiguration)
    )
    {
        return false;
    }
    TArray<FText> Errors;
    InPresentation->GatherValidationErrors(Errors);
    USkeletalMeshComponent* Mesh = InTargetPawn->GetMesh();
    if (
        !Errors.IsEmpty()
        || Mesh == nullptr
        || Mesh->GetSkeletalMeshAsset() != InPresentation->SkeletalMesh.Get()
    )
    {
        return false;
    }

    TArray<const FSharVehicleLightPresentationBinding*> BrakeBindings;
    for (
        const FSharVehicleLightPresentationBinding& Binding
        : InPresentation->LightBindings
    )
    {
        if (Binding.Role != ESharVehicleLightPresentationRole::Brake)
        {
            continue;
        }
        if (Mesh->GetBoneIndex(Binding.BoneName) == INDEX_NONE)
        {
            return false;
        }
        BrakeBindings.Add(&Binding);
    }

    ResetBrakeLights();
    if (!InState->Configure(InPresentation))
    {
        return false;
    }
    TargetPawn = InTargetPawn;
    State = InState;
    BrakeLightEmitters.Reserve(BrakeBindings.Num());
    for (const FSharVehicleLightPresentationBinding* Binding : BrakeBindings)
    {
        FSharVehicleNativeBrakeLightEmitter& Emitter =
            BrakeLightEmitters.AddDefaulted_GetRef();
        Emitter.BoneName = Binding->BoneName;
        Emitter.BindingId = Binding->BindingId;
        const FName ComponentName = MakeUniqueObjectName(
            InTargetPawn,
            USpotLightComponent::StaticClass(),
            FName(TEXT("SharNativeBrakeLight"))
        );
        Emitter.Component = NewObject<USpotLightComponent>(
            InTargetPawn,
            ComponentName,
            RF_Transient
        );
        if (Emitter.Component == nullptr)
        {
            ResetBrakeLights();
            return false;
        }
        InTargetPawn->AddInstanceComponent(Emitter.Component);
        Emitter.Component->SetupAttachment(Mesh, Emitter.BoneName);
        Emitter.Component->SetRelativeLocation(FVector::ZeroVector);
        Emitter.Component->SetRelativeRotation(
            InConfiguration.BoneLocalDirection.GetSafeNormal().Rotation()
        );
        Emitter.Component->SetMobility(EComponentMobility::Movable);
        Emitter.Component->SetIntensityUnits(ELightUnits::Lumens);
        Emitter.Component->SetIntensity(InConfiguration.IntensityLumens);
        Emitter.Component->SetAttenuationRadius(
            InConfiguration.AttenuationRadiusCentimeters
        );
        Emitter.Component->SetInnerConeAngle(
            InConfiguration.InnerConeAngleDegrees
        );
        Emitter.Component->SetOuterConeAngle(
            InConfiguration.OuterConeAngleDegrees
        );
        Emitter.Component->SetLightColor(InConfiguration.LightColor);
        Emitter.Component->SetVisibility(false);
        if (InTargetPawn->GetWorld() != nullptr)
        {
            Emitter.Component->RegisterComponent();
        }
    }
    return RefreshBrakeLights();
}

bool USharVehicleNativeBrakeLightAdapter::RefreshBrakeLights()
{
    if (TargetPawn == nullptr || State == nullptr || !State->IsConfigured())
    {
        return false;
    }
    for (FSharVehicleNativeBrakeLightEmitter& Emitter : BrakeLightEmitters)
    {
        if (Emitter.Component == nullptr)
        {
            return false;
        }
        bool bVisible = false;
        if (!State->GetBindingVisibility(Emitter.BindingId, bVisible))
        {
            return false;
        }
        Emitter.Component->SetVisibility(bVisible);
    }
    return true;
}

void USharVehicleNativeBrakeLightAdapter::ResetBrakeLights()
{
    for (FSharVehicleNativeBrakeLightEmitter& Emitter : BrakeLightEmitters)
    {
        if (Emitter.Component == nullptr)
        {
            continue;
        }
        if (TargetPawn != nullptr)
        {
            TargetPawn->RemoveInstanceComponent(Emitter.Component);
        }
        Emitter.Component->DestroyComponent();
    }
    BrakeLightEmitters.Reset();
    TargetPawn = nullptr;
    State = nullptr;
}

int32 USharVehicleNativeBrakeLightAdapter::GetBrakeLightEmitterCount() const
{
    return BrakeLightEmitters.Num();
}

FName USharVehicleNativeBrakeLightAdapter::GetBrakeLightEmitterBone(
    const int32 Index
) const
{
    return BrakeLightEmitters.IsValidIndex(Index)
        ? BrakeLightEmitters[Index].BoneName
        : NAME_None;
}

USpotLightComponent* USharVehicleNativeBrakeLightAdapter::GetBrakeLightEmitter(
    const int32 Index
) const
{
    return BrakeLightEmitters.IsValidIndex(Index)
        ? BrakeLightEmitters[Index].Component.Get()
        : nullptr;
}
