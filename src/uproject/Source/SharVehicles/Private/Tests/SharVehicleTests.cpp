// File: SharVehicleTests.cpp
// Path: src/uproject/Source/SharVehicles/Private/Tests/SharVehicleTests.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: transient vehicle definition, damage, and selection tests only.
// ADR: docs/adr/unreal/architecture/aaa-native-content-and-gameplay-foundation.md
// LARGE-FILE owner=SharVehicles; reason=three cohesive vehicle-contract scenarios;
// split=separate selection tests when world-spawn adapters are introduced;
// validation=validate.sh SharVehicles plus Unreal automation; review=2027-01.

#if WITH_DEV_AUTOMATION_TESTS

#include "Vehicles/SharVehicleDefinition.h"
#include "Vehicles/SharVehicleRuntimeState.h"
#include "Vehicles/SharVehicleSelectionTransaction.h"

#include "Engine/DataAsset.h"
#include "Misc/AutomationTest.h"

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

#endif
