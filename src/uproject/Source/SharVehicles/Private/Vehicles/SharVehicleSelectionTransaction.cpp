// File: SharVehicleSelectionTransaction.cpp
// Path: src/uproject/Source/SharVehicles/Private/Vehicles/SharVehicleSelectionTransaction.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: deterministic selection transaction transitions only; no spawning, ownership, or payment authority.
// ADR: docs/adr/unreal/architecture/aaa-native-content-and-gameplay-foundation.md
// LARGE-FILE owner=SharVehicles; reason=cohesive phone-booth transaction lifecycle;
// split=extract reservation evidence if world-spawn orchestration grows;
// validation=validate.sh SharVehicles plus Unreal automation; review=2027-01.

#include "Vehicles/SharVehicleSelectionTransaction.h"

#include "Content/SharPrimaryContentDefinition.h"
#include "Engine/DataAsset.h"

void USharVehicleSelectionTransaction::SetError(const TCHAR* Message)
{
    LastError = FText::FromString(Message);
}

bool USharVehicleSelectionTransaction::Begin(
    const FPrimaryAssetId& InPreviousVehicleId,
    const FPrimaryAssetId& InRequestedVehicleId
)
{
    const bool bInvalid =
        State != ESharVehicleSelectionState::Idle
        || !InRequestedVehicleId.IsValid()
        || InRequestedVehicleId == InPreviousVehicleId;
    if (bInvalid)
    {
        SetError(
            TEXT("Vehicle selection requires an idle transaction and a different valid requested vehicle.")
        );
        return false;
    }
    PreviousVehicleId = InPreviousVehicleId;
    RequestedVehicleId = InRequestedVehicleId;
    ReservationId = FName();
    LastError = FText();
    State = ESharVehicleSelectionState::Requested;
    return true;
}

bool USharVehicleSelectionTransaction::MarkSpawnReserved(
    const FName& InReservationId
)
{
    const bool bInvalid =
        State != ESharVehicleSelectionState::Requested
        || !USharPrimaryContentDefinition::IsCanonicalIdentifier(
            InReservationId
        );
    if (bInvalid)
    {
        SetError(
            TEXT("Spawn reservation requires a requested transaction and canonical reservation identity.")
        );
        return false;
    }
    ReservationId = InReservationId;
    LastError = FText();
    State = ESharVehicleSelectionState::SpawnReserved;
    return true;
}

bool USharVehicleSelectionTransaction::Commit()
{
    if (State != ESharVehicleSelectionState::SpawnReserved)
    {
        SetError(
            TEXT("Vehicle selection can commit only after spawn reservation.")
        );
        return false;
    }
    LastError = FText();
    State = ESharVehicleSelectionState::Committed;
    return true;
}

bool USharVehicleSelectionTransaction::Rollback()
{
    const bool bCanRollback =
        State == ESharVehicleSelectionState::Requested
        || State == ESharVehicleSelectionState::SpawnReserved;
    if (!bCanRollback)
    {
        SetError(
            TEXT("Only an uncommitted vehicle selection can roll back.")
        );
        return false;
    }
    ReservationId = FName();
    LastError = FText();
    State = ESharVehicleSelectionState::RolledBack;
    return true;
}

ESharVehicleSelectionState
USharVehicleSelectionTransaction::GetState() const
{
    return State;
}

FPrimaryAssetId
USharVehicleSelectionTransaction::GetPreviousVehicleId() const
{
    return PreviousVehicleId;
}

FPrimaryAssetId
USharVehicleSelectionTransaction::GetRequestedVehicleId() const
{
    return RequestedVehicleId;
}

FName USharVehicleSelectionTransaction::GetReservationId() const
{
    return ReservationId;
}

FText USharVehicleSelectionTransaction::GetLastError() const
{
    return LastError;
}
