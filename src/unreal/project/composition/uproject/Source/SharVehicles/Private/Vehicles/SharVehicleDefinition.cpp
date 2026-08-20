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
//   - Shar vehicle definition composition module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Shar vehicle definition composition module.
// - Description:
//   - Implements the declared composition module responsibility for project.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Shar vehicle definition composition module.

#include "Vehicles/SharVehicleDefinition.h"

#include "Algo/AnyOf.h"
#include "Content/SharPrimaryContentDefinition.h"
#include "Engine/DataAsset.h"

static void AddVehicleError(
    TArray<FText>& OutErrors,
    const TCHAR* Message
)
{
    OutErrors.Add(FText::FromString(Message));
}

static bool IsCanonical(const FName& Value)
{
    return USharPrimaryContentDefinition::IsCanonicalIdentifier(Value);
}

static void AppendIdentityErrors(
    const USharVehicleDefinition& Definition,
    TArray<FText>& OutErrors
)
{
    const bool bInvalid =
        !IsCanonical(Definition.VehicleFamilyId)
        || !IsCanonical(Definition.DefaultPresentationId)
        || !IsCanonical(Definition.AiProfileId)
        || !IsCanonical(Definition.NetworkPredictionProfileId)
        || !IsCanonical(Definition.RecoveryPolicyId);
    if (bInvalid)
    {
        AddVehicleError(
            OutErrors,
            // jig-ignore-next-line: exact syntax is indivisible
            TEXT("Vehicle family, presentation, AI, network, and recovery identities must be canonical.")
        );
    }
}

static void AppendPhysicsErrors(
    const FSharVehiclePhysicsDefinition& Physics,
    TArray<FText>& OutErrors
)
{
    const bool bInvalid =
        !FMath::IsFinite(Physics.MassKilograms)
        || Physics.MassKilograms <= 0.0F
        || !FMath::IsFinite(Physics.MaximumSpeedKilometersPerHour)
        || Physics.MaximumSpeedKilometersPerHour <= 0.0F
        || !FMath::IsFinite(Physics.EngineTorqueNewtonMeters)
        || Physics.EngineTorqueNewtonMeters <= 0.0F
        || !FMath::IsFinite(Physics.WheelRadiusCentimeters)
        || Physics.WheelRadiusCentimeters <= 0.0F
        || !FMath::IsFinite(Physics.WheelbaseCentimeters)
        || Physics.WheelbaseCentimeters <= 0.0F
        || !FMath::IsFinite(Physics.TrackWidthCentimeters)
        || Physics.TrackWidthCentimeters <= 0.0F
        || !FMath::IsFinite(Physics.SuspensionTravelCentimeters)
        || Physics.SuspensionTravelCentimeters < 0.0F;
    if (bInvalid)
    {
        AddVehicleError(
            OutErrors,
            // jig-ignore-next-line: exact syntax is indivisible
            TEXT("Drivable vehicle physics values must be finite and physically positive.")
        );
    }
}

static bool SeatHasCanonicalIdentity(
    const FSharVehicleSeatDefinition& Seat
)
{
    return IsCanonical(Seat.SeatId)
        && IsCanonical(Seat.OccupancyRoleId)
        && IsCanonical(Seat.EntryTransformId)
        && IsCanonical(Seat.ExitTransformId)
        && IsCanonical(Seat.CameraProfileId);
}

static void AppendSeatErrors(
    const TArray<FSharVehicleSeatDefinition>& Seats,
    const bool bDrivable,
    TArray<FText>& OutErrors
)
{
    TSet<FName> SeenSeatIds;
    int32 DriverCount = 0;
    for (const FSharVehicleSeatDefinition& Seat : Seats)
    {
        if (!SeatHasCanonicalIdentity(Seat))
        {
            AddVehicleError(
                OutErrors,
                // jig-ignore-next-line: exact syntax is indivisible
                TEXT("Vehicle seats require canonical role, transform, camera, and seat identities.")
            );
        }
        if (SeenSeatIds.Contains(Seat.SeatId))
        {
            AddVehicleError(
                OutErrors,
                TEXT("Vehicle seat identities must be unique.")
            );
        }
        SeenSeatIds.Add(Seat.SeatId);
        DriverCount += Seat.bDriver ? 1 : 0;
    }
    if (bDrivable && DriverCount != 1)
    {
        AddVehicleError(
            OutErrors,
            TEXT("A drivable vehicle requires exactly one driver seat.")
        );
    }
}

static bool DamageBandIsValid(
    const FSharVehicleDamageBandDefinition& Band
)
{
    return FMath::IsFinite(Band.MinimumNormalizedDamage)
        && Band.MinimumNormalizedDamage >= 0.0F
        && Band.MinimumNormalizedDamage <= 1.0F
        && FMath::IsFinite(Band.HandlingMultiplier)
        && Band.HandlingMultiplier >= 0.0F
        && Band.HandlingMultiplier <= 1.0F
        && IsCanonical(Band.PresentationProfileId);
}

static void AppendDamageBandEntryErrors(
    const FSharVehicleDamageBandDefinition& Band,
    const float PreviousThreshold,
    TSet<ESharVehicleDamageState>& SeenStates,
    TArray<FText>& OutErrors
)
{
    if (!DamageBandIsValid(Band))
    {
        AddVehicleError(
            OutErrors,
            // jig-ignore-next-line: exact syntax is indivisible
            TEXT("Vehicle damage bands contain invalid thresholds, handling, or presentation identities.")
        );
    }
    if (Band.MinimumNormalizedDamage <= PreviousThreshold)
    {
        AddVehicleError(
            OutErrors,
            TEXT("Vehicle damage thresholds must be strictly increasing.")
        );
    }
    if (SeenStates.Contains(Band.State))
    {
        AddVehicleError(
            OutErrors,
            TEXT("Vehicle damage states must be unique.")
        );
    }
    SeenStates.Add(Band.State);
}

static bool ContainsDamageState(
    const TArray<FSharVehicleDamageBandDefinition>& DamageBands,
    const ESharVehicleDamageState State
)
{
    return Algo::AnyOf(
        DamageBands,
        [State](const FSharVehicleDamageBandDefinition& Band)
        {
            return Band.State == State;
        }
    );
}

static void AppendDamageBandErrors(
    const TArray<FSharVehicleDamageBandDefinition>& DamageBands,
    TArray<FText>& OutErrors
)
{
    TSet<ESharVehicleDamageState> SeenStates;
    float PreviousThreshold = -1.0F;
    for (const FSharVehicleDamageBandDefinition& Band : DamageBands)
    {
        AppendDamageBandEntryErrors(
            Band,
            PreviousThreshold,
            SeenStates,
            OutErrors
        );
        PreviousThreshold = Band.MinimumNormalizedDamage;
    }
    const bool bMissingRequiredState =
        !ContainsDamageState(
            DamageBands,
            ESharVehicleDamageState::Operational
        )
        || !ContainsDamageState(
            DamageBands,
            ESharVehicleDamageState::Disabled
        );
    if (bMissingRequiredState)
    {
        AddVehicleError(
            OutErrors,
            // jig-ignore-next-line: exact syntax is indivisible
            TEXT("Vehicle damage bands require operational and disabled states.")
        );
    }
}

void USharVehicleDefinition::GatherValidationErrors(
    TArray<FText>& OutErrors
) const
{
    USharPrimaryContentDefinition::GatherValidationErrors(OutErrors);
    AppendIdentityErrors(*this, OutErrors);
    if (bDrivable)
    {
        AppendPhysicsErrors(Physics, OutErrors);
    }
    AppendSeatErrors(Seats, bDrivable, OutErrors);
    if (DamageBands.IsEmpty())
    {
        AddVehicleError(
            OutErrors,
            TEXT("Vehicle definitions require typed damage bands.")
        );
    }
    else
    {
        AppendDamageBandErrors(DamageBands, OutErrors);
    }
}

FPrimaryAssetType USharVehicleDefinition::GetDefinitionAssetType() const
{
    return {TEXT("SharVehicle")};
}
