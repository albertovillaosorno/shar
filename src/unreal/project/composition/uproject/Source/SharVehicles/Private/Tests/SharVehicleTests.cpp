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
//   - Shar vehicle tests composition module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Shar vehicle tests composition module.
// - Description:
//   - Implements the declared composition module responsibility for project.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Shar vehicle tests composition module.

#if WITH_DEV_AUTOMATION_TESTS

#include <initializer_list>

#include "Vehicles/SharVehicleConstructionTransaction.h"
#include "Vehicles/SharVehicleNativeLightAdapter.h"
#include "Vehicles/SharVehicleDefinition.h"
#include "Vehicles/SharVehiclePawn.h"
#include "Vehicles/SharVehiclePresentationDefinition.h"
#include "Vehicles/SharVehiclePresentationState.h"
#include "Vehicles/SharVehicleRuntimeState.h"
#include "Vehicles/SharVehicleSelectionTransaction.h"

#include "Animation/AnimInstance.h"
#include "Animation/Skeleton.h"
#include "ChaosVehicleWheel.h"
#include "ChaosWheeledVehicleMovementComponent.h"
#include "Components/SpotLightComponent.h"
#include "Engine/DataAsset.h"
#include "Engine/SkeletalMesh.h"
#include "Materials/Material.h"
#include "Materials/MaterialInterface.h"
#include "Misc/AutomationTest.h"
#include "PhysicsEngine/PhysicsAsset.h"
#include "PhysicsEngine/SkeletalBodySetup.h"
#include "ReferenceSkeleton.h"
#include "VehicleAnimationInstance.h"

static constexpr float DamagedThreshold = 0.35F;
static constexpr float CriticalThreshold = 0.70F;
static constexpr float DisabledThreshold = 1.0F;
static constexpr float DamagedHandlingMultiplier = 0.85F;
static constexpr float CriticalHandlingMultiplier = 0.60F;
static constexpr float InitialDamage = 0.40F;
static constexpr float FinalDamage = 0.70F;
static constexpr float RepairDamage = 0.20F;
static constexpr float FloatTolerance = 0.001F;

static FPrimaryAssetId MakeVehicleId(const TCHAR* Name)
{
    return {
        FPrimaryAssetType(TEXT("SharVehicle")),
        FName(Name),
    };
}

template <typename TObject>
static TSoftObjectPtr<TObject> MakeVehicleSoftObject(const TCHAR* ObjectPath)
{
    return TSoftObjectPtr<TObject>(FSoftObjectPath(ObjectPath));
}

template <typename TObject>
static TSoftClassPtr<TObject> MakeVehicleSoftClass(const TCHAR* ClassPath)
{
    return TSoftClassPtr<TObject>(FSoftObjectPath(ClassPath));
}

static void FillVehiclePresentationBase(
    USharVehiclePresentationDefinition& Presentation
)
{
    Presentation.CanonicalId = FName(TEXT("family_sedan_default"));
    Presentation.DisplayName = FText::FromString(TEXT("Family sedan default"));
    Presentation.SourcePackageIds = {FName(TEXT("vehicle_contract"))};
    Presentation.RevisionToken = TEXT("sha256:vehicle_presentation_v1");
    Presentation.ValidationProfile =
        FName(TEXT("vehicle_standard_v1"));
    Presentation.OwningFeature = FName(TEXT("base"));
    Presentation.PresentationVariant = FName(TEXT("default"));
    Presentation.SkeletalMesh = MakeVehicleSoftObject<USkeletalMesh>(
        TEXT("/Game/SHAR/Tests/Generated/SK_sedan.SK_sedan")
    );
    Presentation.Skeleton = MakeVehicleSoftObject<USkeleton>(
        TEXT("/Game/SHAR/Tests/Generated/SKEL_sedan.SKEL_sedan")
    );
    Presentation.PhysicsAsset = MakeVehicleSoftObject<UPhysicsAsset>(
        TEXT("/Game/SHAR/Tests/Generated/PHYS_sedan.PHYS_sedan")
    );
    Presentation.AnimationClass = MakeVehicleSoftClass<UAnimInstance>(
        TEXT("/Game/SHAR/Tests/Generated/ABP_sedan.ABP_sedan_C")
    );
    Presentation.MaterialInstances.Add(
        MakeVehicleSoftObject<UMaterialInterface>(
            TEXT("/Game/SHAR/Tests/Generated/MI_sedan.MI_sedan")
        )
    );
    Presentation.RigProfileId = FName(TEXT("vehicle_sedan_v1"));
    Presentation.SemanticPreparationRevision =
        TEXT("sha256:vehicle_semantic_presentation_v1");
}


static FSharVehicleLightPresentationBinding MakeLightBinding(
    const TCHAR* BindingId,
    const ESharVehicleLightPresentationRole Role,
    const TCHAR* BoneName,
    std::initializer_list<int32> MaterialSlots
)
{
    FSharVehicleLightPresentationBinding Binding;
    Binding.BindingId = FName(BindingId);
    Binding.Role = Role;
    Binding.BoneName = FName(BoneName);
    for (const int32 SlotIndex : MaterialSlots)
    {
        Binding.MaterialSlotIndices.Add(SlotIndex);
    }
    return Binding;
}

static FSharVehicleWheelPresentationBinding MakeWheelBinding(
    const TCHAR* WheelId,
    const TCHAR* BoneName
)
{
    FSharVehicleWheelPresentationBinding Wheel;
    Wheel.WheelId = FName(WheelId);
    Wheel.BoneName = FName(BoneName);
    Wheel.WheelClass = MakeVehicleSoftClass<UChaosVehicleWheel>(
        TEXT("/Game/SHAR/Tests/Generated/BP_Wheel.BP_Wheel_C")
    );
    return Wheel;
}

static USharVehiclePresentationDefinition* MakeValidVehiclePresentation()
{
    auto* Presentation = NewObject<USharVehiclePresentationDefinition>();
    FillVehiclePresentationBase(*Presentation);
    Presentation->Wheels = {
        MakeWheelBinding(TEXT("front_left"), TEXT("w0")),
        MakeWheelBinding(TEXT("front_right"), TEXT("w1")),
        MakeWheelBinding(TEXT("rear_left"), TEXT("w2")),
        MakeWheelBinding(TEXT("rear_right"), TEXT("w3")),
    };
    return Presentation;
}

static USharVehicleDefinition* MakeValidVehicle()
{
    auto* Vehicle = NewObject<USharVehicleDefinition>();
    Vehicle->CanonicalId = FName(TEXT("family_sedan"));
    Vehicle->DisplayName = FText::FromString(TEXT("Family sedan"));
    Vehicle->SourcePackageIds = {FName(TEXT("vehicle_contract"))};
    Vehicle->RevisionToken = TEXT("sha256:vehicle_definition_v1");
    Vehicle->ValidationProfile = FName(TEXT("vehicle_standard_v1"));
    Vehicle->OwningFeature = FName(TEXT("base"));
    Vehicle->VehicleFamilyId = FName(TEXT("passenger_car"));
    Vehicle->DefaultPresentationId = FName(TEXT("family_sedan_default"));
    Vehicle->AiProfileId = FName(TEXT("traffic_standard_v1"));
    Vehicle->NetworkPredictionProfileId =
        FName(TEXT("vehicle_prediction_v1"));
    Vehicle->RecoveryPolicyId = FName(TEXT("roadside_reset_v1"));

    FSharVehicleSeatDefinition DriverSeat;
    DriverSeat.SeatId = FName(TEXT("driver"));
    DriverSeat.OccupancyRoleId = FName(TEXT("driver"));
    DriverSeat.EntryTransformId = FName(TEXT("driver_entry"));
    DriverSeat.ExitTransformId = FName(TEXT("driver_exit"));
    DriverSeat.CameraProfileId = FName(TEXT("vehicle_driver_v1"));
    DriverSeat.bDriver = true;
    Vehicle->Seats.Add(DriverSeat);

    FSharVehicleDamageBandDefinition OperationalBand;
    OperationalBand.State = ESharVehicleDamageState::Operational;
    OperationalBand.MinimumNormalizedDamage = 0.0F;
    OperationalBand.HandlingMultiplier = 1.0F;
    OperationalBand.PresentationProfileId =
        FName(TEXT("vehicle_operational_v1"));
    Vehicle->DamageBands.Add(OperationalBand);

    FSharVehicleDamageBandDefinition DamagedBand;
    DamagedBand.State = ESharVehicleDamageState::Damaged;
    DamagedBand.MinimumNormalizedDamage = DamagedThreshold;
    DamagedBand.HandlingMultiplier = DamagedHandlingMultiplier;
    DamagedBand.PresentationProfileId =
        FName(TEXT("vehicle_damaged_v1"));
    Vehicle->DamageBands.Add(DamagedBand);

    FSharVehicleDamageBandDefinition CriticalBand;
    CriticalBand.State = ESharVehicleDamageState::Critical;
    CriticalBand.MinimumNormalizedDamage = CriticalThreshold;
    CriticalBand.HandlingMultiplier = CriticalHandlingMultiplier;
    CriticalBand.PresentationProfileId =
        FName(TEXT("vehicle_critical_v1"));
    Vehicle->DamageBands.Add(CriticalBand);

    FSharVehicleDamageBandDefinition DisabledBand;
    DisabledBand.State = ESharVehicleDamageState::Disabled;
    DisabledBand.MinimumNormalizedDamage = DisabledThreshold;
    DisabledBand.HandlingMultiplier = 0.0F;
    DisabledBand.PresentationProfileId =
        FName(TEXT("vehicle_disabled_v1"));
    Vehicle->DamageBands.Add(DisabledBand);
    return Vehicle;
}

static void AddVehicleTestBone(
    FReferenceSkeletonModifier& Modifier,
    const TCHAR* BoneName,
    const int32 ParentIndex
)
{
    Modifier.Add(
        FMeshBoneInfo(FName(BoneName), FString(BoneName), ParentIndex),
        FTransform::Identity
    );
}

static void AddVehicleTestPhysicsBody(
    UPhysicsAsset& PhysicsAsset,
    const FName BoneName
)
{
    auto* Body = NewObject<USkeletalBodySetup>(&PhysicsAsset);
    Body->BoneName = BoneName;
    PhysicsAsset.SkeletalBodySetups.Add(Body);
}

static USharVehiclePresentationDefinition*
MakeResolvedVehiclePresentation()
{
    auto* Presentation = MakeValidVehiclePresentation();
    auto* Skeleton = NewObject<USkeleton>();
    auto* SkeletalMesh = NewObject<USkeletalMesh>();
    auto* PhysicsAsset = NewObject<UPhysicsAsset>();
    auto* Material = NewObject<UMaterial>();
    auto* SecondaryMaterial = NewObject<UMaterial>();

    FReferenceSkeleton ReferenceSkeleton;
    {
        FReferenceSkeletonModifier Modifier(ReferenceSkeleton, Skeleton);
        AddVehicleTestBone(Modifier, TEXT("root"), INDEX_NONE);
        AddVehicleTestBone(Modifier, TEXT("w0"), 0);
        AddVehicleTestBone(Modifier, TEXT("w1"), 0);
        AddVehicleTestBone(Modifier, TEXT("w2"), 0);
        AddVehicleTestBone(Modifier, TEXT("w3"), 0);
        AddVehicleTestBone(Modifier, TEXT("hll"), 0);
        AddVehicleTestBone(Modifier, TEXT("hlr"), 0);
    }
    SkeletalMesh->SetRefSkeleton(ReferenceSkeleton);
    SkeletalMesh->SetSkeleton(Skeleton);
    AddVehicleTestPhysicsBody(*PhysicsAsset, FName(TEXT("w0")));
    AddVehicleTestPhysicsBody(*PhysicsAsset, FName(TEXT("w1")));
    AddVehicleTestPhysicsBody(*PhysicsAsset, FName(TEXT("w2")));
    AddVehicleTestPhysicsBody(*PhysicsAsset, FName(TEXT("w3")));
    PhysicsAsset->UpdateBodySetupIndexMap();
    TArray<FSkeletalMaterial> Materials;
    Materials.Emplace(Material, FName(TEXT("body")));
    Materials.Emplace(SecondaryMaterial, FName(TEXT("lights")));
    SkeletalMesh->SetMaterials(Materials);

    Presentation->SkeletalMesh = TSoftObjectPtr<USkeletalMesh>(SkeletalMesh);
    Presentation->Skeleton = TSoftObjectPtr<USkeleton>(Skeleton);
    Presentation->PhysicsAsset = TSoftObjectPtr<UPhysicsAsset>(PhysicsAsset);
    Presentation->AnimationClass = TSoftClassPtr<UAnimInstance>(
        UVehicleAnimationInstance::StaticClass()
    );
    Presentation->MaterialInstances = {
        TSoftObjectPtr<UMaterialInterface>(Material),
        TSoftObjectPtr<UMaterialInterface>(SecondaryMaterial),
    };
    Presentation->LightBindings = {
        MakeLightBinding(
            TEXT("headlight_left_lens"),
            ESharVehicleLightPresentationRole::Headlight,
            TEXT("hll"),
            {0}
        ),
        MakeLightBinding(
            TEXT("headlight_left_glow"),
            ESharVehicleLightPresentationRole::Headlight,
            TEXT("hll"),
            {1}
        ),
        MakeLightBinding(
            TEXT("headlight_right_lens"),
            ESharVehicleLightPresentationRole::Headlight,
            TEXT("hlr"),
            {0}
        ),
    };
    for (FSharVehicleWheelPresentationBinding& Wheel : Presentation->Wheels)
    {
        Wheel.WheelClass = TSoftClassPtr<UChaosVehicleWheel>(
            UChaosVehicleWheel::StaticClass()
        );
    }
    return Presentation;
}

namespace
{
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharVehicleDefinitionValidationTest,
    "SHAR.Vehicles.Definition.Validation",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharVehiclePresentationDefinitionValidationTest,
    "SHAR.Vehicles.Presentation.Validation",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharVehiclePresentationLightRuntimeTest,
    "SHAR.Vehicles.Runtime.PresentationLights",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharVehicleNativeHeadlightAdapterTest,
    "SHAR.Vehicles.Runtime.NativeHeadlightAdapter",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharVehicleDamageRuntimeTest,
    "SHAR.Vehicles.Runtime.Damage",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharVehicleSelectionTransactionTest,
    "SHAR.Vehicles.Selection.Transaction",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharVehicleConstructionTransactionTest,
    "SHAR.Vehicles.Construction.Transaction",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
} // namespace

bool FSharVehicleDefinitionValidationTest::RunTest(
    const FString& Parameters
)
{
    (void)Parameters;
    auto* Vehicle = MakeValidVehicle();
    TArray<FText> Errors;
    Vehicle->GatherValidationErrors(Errors);
    TestTrue(TEXT("Valid vehicle definition passes"), Errors.IsEmpty());

    const FSharVehicleSeatDefinition DuplicateSeat = Vehicle->Seats.Last();
    Vehicle->Seats.Add(DuplicateSeat);
    Errors.Reset();
    Vehicle->GatherValidationErrors(Errors);
    TestFalse(TEXT("Duplicate vehicle seat is rejected"), Errors.IsEmpty());
    return true;
}

bool FSharVehiclePresentationDefinitionValidationTest::RunTest(
    const FString& Parameters
)
{
    (void)Parameters;
    auto* Presentation = MakeValidVehiclePresentation();
    TArray<FText> Errors;
    Presentation->GatherValidationErrors(Errors);
    TestTrue(TEXT("Valid vehicle presentation passes"), Errors.IsEmpty());

    Presentation->PhysicsAsset.Reset();
    Errors.Reset();
    Presentation->GatherValidationErrors(Errors);
    TestFalse(TEXT("Missing Physics Asset is rejected"), Errors.IsEmpty());

    Presentation = MakeValidVehiclePresentation();
    Presentation->Wheels[1].WheelId = Presentation->Wheels[0].WheelId;
    Errors.Reset();
    Presentation->GatherValidationErrors(Errors);
    TestFalse(TEXT("Duplicate wheel identity is rejected"), Errors.IsEmpty());

    Presentation = MakeValidVehiclePresentation();
    Presentation->Wheels[1].BoneName =
        Presentation->Wheels[0].BoneName;
    Errors.Reset();
    Presentation->GatherValidationErrors(Errors);
    TestFalse(
        TEXT("Duplicate wheel rig binding is rejected"),
        Errors.IsEmpty()
    );

    Presentation = MakeValidVehiclePresentation();
    Presentation->LightBindings = {
        MakeLightBinding(
            TEXT("brake_primary"),
            ESharVehicleLightPresentationRole::Brake,
            TEXT("brake1"),
            {0}
        ),
    };
    Errors.Reset();
    Presentation->GatherValidationErrors(Errors);
    TestTrue(TEXT("Valid light binding passes"), Errors.IsEmpty());

    Presentation->LightBindings[0].MaterialSlotIndices = {1};
    Errors.Reset();
    Presentation->GatherValidationErrors(Errors);
    TestFalse(TEXT("Out-of-range light slot is rejected"), Errors.IsEmpty());

    Presentation = MakeValidVehiclePresentation();
    Presentation->LightBindings = {
        MakeLightBinding(
            TEXT("brake_primary"),
            ESharVehicleLightPresentationRole::Brake,
            TEXT("brake1"),
            {0}
        ),
        MakeLightBinding(
            TEXT("brake_secondary"),
            ESharVehicleLightPresentationRole::Brake,
            TEXT("brake1"),
            {0}
        ),
    };
    Errors.Reset();
    Presentation->GatherValidationErrors(Errors);
    TestFalse(TEXT("Duplicate light target is rejected"), Errors.IsEmpty());
    return true;
}


bool FSharVehiclePresentationLightRuntimeTest::RunTest(
    const FString& Parameters
)
{
    (void)Parameters;
    auto* Presentation = MakeValidVehiclePresentation();
    Presentation->LightBindings = {
        MakeLightBinding(
            TEXT("headlight_primary"),
            ESharVehicleLightPresentationRole::Headlight,
            TEXT("hll"),
            {0}
        ),
        MakeLightBinding(
            TEXT("brake_primary"),
            ESharVehicleLightPresentationRole::Brake,
            TEXT("brake1"),
            {0}
        ),
        MakeLightBinding(
            TEXT("reverse_primary"),
            ESharVehicleLightPresentationRole::Reverse,
            TEXT("rev1"),
            {0}
        ),
    };
    auto* State = NewObject<USharVehiclePresentationState>();
    TestTrue(
        TEXT("Presentation light state configures"),
        State->Configure(Presentation)
    );
    bool bVisible = true;
    TestTrue(
        TEXT("Configured headlight binding resolves"),
        State->GetBindingVisibility(FName(TEXT("headlight_primary")), bVisible)
    );
    TestFalse(TEXT("Headlights start hidden"), bVisible);
    TestTrue(TEXT("Headlights enable"), State->SetHeadlightsEnabled(true));
    TestTrue(
        TEXT("Enabled headlight binding resolves"),
        State->GetBindingVisibility(FName(TEXT("headlight_primary")), bVisible)
    );
    TestTrue(TEXT("Headlight becomes visible"), bVisible);

    TestTrue(TEXT("Brake state enables"), State->SetBrakeState(true, false));
    TestTrue(
        TEXT("Brake binding resolves"),
        State->GetBindingVisibility(FName(TEXT("brake_primary")), bVisible)
    );
    TestTrue(TEXT("Brake binding becomes visible"), bVisible);
    TestTrue(TEXT("Reverse state enables"), State->SetBrakeState(true, true));
    TestTrue(
        TEXT("Brake binding still resolves"),
        State->GetBindingVisibility(FName(TEXT("brake_primary")), bVisible)
    );
    TestFalse(TEXT("Reverse suppresses brake binding"), bVisible);
    TestTrue(
        TEXT("Reverse binding resolves"),
        State->GetBindingVisibility(FName(TEXT("reverse_primary")), bVisible)
    );
    TestTrue(TEXT("Reverse binding becomes visible"), bVisible);

    TestTrue(
        TEXT("Damage can suppress presentation lights"),
        State->SetLightsSuppressedByDamage(true)
    );
    TestTrue(
        TEXT("Suppressed reverse binding resolves"),
        State->GetBindingVisibility(FName(TEXT("reverse_primary")), bVisible)
    );
    TestFalse(TEXT("Damage suppression hides lights"), bVisible);
    TestTrue(
        TEXT("Damage suppression clears"),
        State->SetLightsSuppressedByDamage(false)
    );
    TestTrue(TEXT("Zero fade is accepted"), State->SetFadeOpacity(0.0F));
    TestTrue(
        TEXT("Faded reverse binding resolves"),
        State->GetBindingVisibility(FName(TEXT("reverse_primary")), bVisible)
    );
    TestFalse(TEXT("Zero fade hides lights"), bVisible);
    TestFalse(
        TEXT("Out-of-range fade is rejected"),
        State->SetFadeOpacity(1.1F)
    );
    return true;
}

bool FSharVehicleNativeHeadlightAdapterTest::RunTest(
    const FString& Parameters
)
{
    (void)Parameters;
    auto* Pawn = NewObject<ASharVehiclePawn>();
    auto* Presentation = MakeResolvedVehiclePresentation();
    Pawn->GetMesh()->SetSkeletalMesh(Presentation->SkeletalMesh.Get());
    auto* State = NewObject<USharVehiclePresentationState>();
    auto* Adapter = NewObject<USharVehicleNativeLightAdapter>();

    FSharVehicleNativeHeadlightConfiguration InvalidConfiguration;
    TestFalse(
        TEXT("Incomplete headlight light settings fail closed"),
        Adapter->ConfigureHeadlights(
            Pawn,
            Presentation,
            State,
            InvalidConfiguration
        )
    );
    TestEqual(
        TEXT("Failed configuration creates no emitters"),
        Adapter->GetHeadlightEmitterCount(),
        0
    );

    FSharVehicleNativeHeadlightConfiguration Configuration;
    Configuration.IntensityLumens = 1250.0F;
    Configuration.AttenuationRadiusCentimeters = 900.0F;
    Configuration.InnerConeAngleDegrees = 15.0F;
    Configuration.OuterConeAngleDegrees = 30.0F;
    Configuration.LightColor = FLinearColor::White;
    Configuration.BoneLocalDirection = FVector::ForwardVector;
    TestTrue(
        TEXT("Reviewed headlight hardpoints configure"),
        Adapter->ConfigureHeadlights(Pawn, Presentation, State, Configuration)
    );
    TestEqual(
        TEXT("Part bindings deduplicate to two physical hardpoints"),
        Adapter->GetHeadlightEmitterCount(),
        2
    );
    TestEqual(
        TEXT("Left hardpoint keeps hll"),
        Adapter->GetHeadlightEmitterBone(0),
        FName(TEXT("hll"))
    );
    TestEqual(
        TEXT("Right hardpoint keeps hlr"),
        Adapter->GetHeadlightEmitterBone(1),
        FName(TEXT("hlr"))
    );
    USpotLightComponent* Left = Adapter->GetHeadlightEmitter(0);
    USpotLightComponent* Right = Adapter->GetHeadlightEmitter(1);
    TestNotNull(TEXT("Left native SpotLight exists"), Left);
    TestNotNull(TEXT("Right native SpotLight exists"), Right);
    if (Left == nullptr || Right == nullptr)
    {
        return false;
    }
    TestEqual(
        TEXT("Left emitter attaches to hll"),
        Left->GetAttachSocketName(),
        FName(TEXT("hll"))
    );
    TestEqual(
        TEXT("Right emitter attaches to hlr"),
        Right->GetAttachSocketName(),
        FName(TEXT("hlr"))
    );
    TestFalse(TEXT("Native headlights start hidden"), Left->IsVisible());
    TestFalse(
        TEXT("Native headlights start hidden together"),
        Right->IsVisible()
    );

    TestTrue(
        TEXT("Semantic headlights enable"),
        State->SetHeadlightsEnabled(true)
    );
    TestTrue(TEXT("Native headlights refresh"), Adapter->RefreshHeadlights());
    TestTrue(TEXT("Left native headlight follows state"), Left->IsVisible());
    TestTrue(TEXT("Right native headlight follows state"), Right->IsVisible());

    TestTrue(
        TEXT("Damage suppresses semantic lights"),
        State->SetLightsSuppressedByDamage(true)
    );
    TestTrue(TEXT("Suppression refreshes"), Adapter->RefreshHeadlights());
    TestFalse(
        TEXT("Suppression hides left native headlight"),
        Left->IsVisible()
    );
    TestFalse(
        TEXT("Suppression hides right native headlight"),
        Right->IsVisible()
    );

    Adapter->ResetHeadlights();
    TestEqual(
        TEXT("Reset removes transient native emitters"),
        Adapter->GetHeadlightEmitterCount(),
        0
    );
    return true;
}


bool FSharVehicleDamageRuntimeTest::RunTest(const FString& Parameters)
{
    (void)Parameters;
    auto* RuntimeState = NewObject<USharVehicleRuntimeState>();
    TestTrue(
        TEXT("Runtime accepts valid vehicle definition"),
        RuntimeState->Configure(MakeValidVehicle())
    );
    TestTrue(
        TEXT("Runtime applies normalized damage"),
        RuntimeState->ApplyNormalizedDamage(InitialDamage)
    );
    TestTrue(
        TEXT("Damage selects damaged state"),
        RuntimeState->GetDamageState() == ESharVehicleDamageState::Damaged
    );
    TestTrue(
        TEXT("Damage degrades handling"),
        FMath::Abs(
            RuntimeState->GetHandlingMultiplier()
                - DamagedHandlingMultiplier
        ) <= FloatTolerance
    );
    TestTrue(
        TEXT("Damage clamps at disabled state"),
        RuntimeState->ApplyNormalizedDamage(FinalDamage)
    );
    TestTrue(
        TEXT("Disabled threshold is selected"),
        RuntimeState->GetDamageState() == ESharVehicleDamageState::Disabled
    );
    TestTrue(
        TEXT("Repair restores lower damage band"),
        RuntimeState->RepairToNormalizedDamage(RepairDamage)
    );
    TestTrue(
        TEXT("Repair restores operational state"),
        RuntimeState->GetDamageState()
            == ESharVehicleDamageState::Operational
    );
    return true;
}

bool FSharVehicleSelectionTransactionTest::RunTest(
    const FString& Parameters
)
{
    (void)Parameters;
    const FPrimaryAssetId PreviousVehicle = MakeVehicleId(
        TEXT("family_sedan")
    );
    const FPrimaryAssetId RequestedVehicle = MakeVehicleId(
        TEXT("sports_car")
    );
    auto* Transaction = NewObject<USharVehicleSelectionTransaction>();
    TestTrue(
        TEXT("Selection request begins"),
        Transaction->Begin(PreviousVehicle, RequestedVehicle)
    );
    TestFalse(TEXT("Selection cannot commit early"), Transaction->Commit());
    TestTrue(
        TEXT("Selection reserves a safe spawn"),
        Transaction->MarkSpawnReserved(FName(TEXT("phone_booth_spawn_01")))
    );
    TestTrue(TEXT("Reserved selection commits"), Transaction->Commit());
    TestTrue(
        TEXT("Committed selection retains requested vehicle"),
        Transaction->GetRequestedVehicleId() == RequestedVehicle
    );

    auto* Rollback = NewObject<USharVehicleSelectionTransaction>();
    TestTrue(
        TEXT("Second selection request begins"),
        Rollback->Begin(PreviousVehicle, RequestedVehicle)
    );
    TestTrue(TEXT("Uncommitted selection rolls back"), Rollback->Rollback());
    TestTrue(
        TEXT("Rollback preserves previous vehicle identity"),
        Rollback->GetPreviousVehicleId() == PreviousVehicle
    );
    return true;
}

bool FSharVehicleConstructionTransactionTest::RunTest(
    const FString& Parameters
)
{
    (void)Parameters;
    auto* Pawn = NewObject<ASharVehiclePawn>();
    auto* Vehicle = MakeValidVehicle();
    auto* Presentation = MakeResolvedVehiclePresentation();
    auto* Movement = Cast<UChaosWheeledVehicleMovementComponent>(
        Pawn->GetVehicleMovementComponent()
    );
    TestNotNull(TEXT("Project Pawn exposes wheeled movement"), Movement);
    if (Movement == nullptr)
    {
        return false;
    }
    Vehicle->Physics.MassKilograms = 1375.0F;
    Vehicle->Physics.EngineTorqueNewtonMeters = 412.0F;
    const float PreviousMass = Movement->Mass;
    const float PreviousTorque = Movement->EngineSetup.MaxTorque;

    auto* Transaction = NewObject<USharVehicleConstructionTransaction>();
    TestTrue(
        TEXT("Resolved vehicle presentation prepares"),
        Transaction->Prepare(Pawn, Vehicle, Presentation)
    );
    TestTrue(
        TEXT("Prepared construction has prepared state"),
        Transaction->GetState() == ESharVehicleConstructionState::Prepared
    );
    TestTrue(
        TEXT("Prepared vehicle construction commits"),
        Transaction->Commit()
    );
    TestTrue(
        TEXT("Committed construction has committed state"),
        Transaction->GetState() == ESharVehicleConstructionState::Committed
    );
    TestTrue(
        TEXT("Committed construction applies Skeletal Mesh"),
        Pawn->GetMesh()->GetSkeletalMeshAsset()
            == Presentation->SkeletalMesh.Get()
    );
    TestTrue(
        TEXT("Committed construction applies Physics Asset"),
        Pawn->GetMesh()->GetPhysicsAsset() == Presentation->PhysicsAsset.Get()
    );
    TestEqual(
        TEXT("Committed construction applies every wheel"),
        Movement->WheelSetups.Num(),
        Presentation->Wheels.Num()
    );
    TestTrue(
        TEXT("Committed construction applies vehicle mass"),
        FMath::IsNearlyEqual(Movement->Mass, Vehicle->Physics.MassKilograms)
    );
    TestTrue(
        TEXT("Committed construction applies engine torque"),
        FMath::IsNearlyEqual(
            Movement->EngineSetup.MaxTorque,
            Vehicle->Physics.EngineTorqueNewtonMeters
        )
    );

    TestTrue(
        TEXT("Committed construction rolls back"),
        Transaction->Rollback()
    );
    TestNull(
        TEXT("Rollback restores empty Skeletal Mesh"),
        Pawn->GetMesh()->GetSkeletalMeshAsset()
    );
    TestEqual(
        TEXT("Rollback restores wheel setup count"),
        Movement->WheelSetups.Num(),
        0
    );
    TestTrue(
        TEXT("Rollback restores vehicle mass"),
        FMath::IsNearlyEqual(Movement->Mass, PreviousMass)
    );
    TestTrue(
        TEXT("Rollback restores engine torque"),
        FMath::IsNearlyEqual(Movement->EngineSetup.MaxTorque, PreviousTorque)
    );

    auto* InvalidPresentation = MakeResolvedVehiclePresentation();
    InvalidPresentation->Wheels[0].BoneName = FName(TEXT("missing_wheel_bone"));
    auto* InvalidTransaction = NewObject<USharVehicleConstructionTransaction>();
    TestFalse(
        TEXT("Construction rejects a wheel absent from the Skeletal Mesh"),
        InvalidTransaction->Prepare(Pawn, Vehicle, InvalidPresentation)
    );
    TestTrue(
        TEXT("Rejected construction remains idle"),
        InvalidTransaction->GetState() == ESharVehicleConstructionState::Idle
    );

    auto* InvalidLightPresentation = MakeResolvedVehiclePresentation();
    InvalidLightPresentation->LightBindings[0].BoneName =
        FName(TEXT("missing_light_bone"));
    auto* InvalidLightTransaction =
        NewObject<USharVehicleConstructionTransaction>();
    TestFalse(
        TEXT("Construction rejects a light absent from the Skeletal Mesh"),
        InvalidLightTransaction->Prepare(
            Pawn,
            Vehicle,
            InvalidLightPresentation
        )
    );

    auto* MissingBodyPresentation = MakeResolvedVehiclePresentation();
    UPhysicsAsset* MissingBodyAsset =
        MissingBodyPresentation->PhysicsAsset.Get();
    TestNotNull(TEXT("Resolved Physics Asset exists"), MissingBodyAsset);
    if (MissingBodyAsset == nullptr)
    {
        return false;
    }
    MissingBodyAsset->SkeletalBodySetups.RemoveAt(0);
    MissingBodyAsset->UpdateBodySetupIndexMap();
    auto* MissingBodyTransaction =
        NewObject<USharVehicleConstructionTransaction>();
    TestFalse(
        TEXT("Construction rejects a wheel without a Physics Asset body"),
        MissingBodyTransaction->Prepare(Pawn, Vehicle, MissingBodyPresentation)
    );
    TestTrue(
        TEXT("Missing-body rejection remains idle"),
        MissingBodyTransaction->GetState()
            == ESharVehicleConstructionState::Idle
    );
    return true;
}

#endif
