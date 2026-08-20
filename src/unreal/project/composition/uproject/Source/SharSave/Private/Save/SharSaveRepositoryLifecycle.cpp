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
//   - Shar save repository lifecycle composition module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Shar save repository lifecycle composition module.
// - Description:
//   - Implements the declared composition module responsibility for project.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Shar save repository lifecycle composition module.

#include "Save/SharSaveRepositorySubsystem.h"
#include "Save/SharSaveContracts.h"

ESharSaveOperationResult USharSaveRepositorySubsystem::PublishTerminal(
    FSharSaveOperationSnapshot& Snapshot,
    const ESharSaveOperationState State,
    const ESharSaveTerminalResult Result
)
{
    if (Snapshot.bReleased)
    {
        return ESharSaveOperationResult::Released;
    }
    if (IsTerminalState(Snapshot.State))
    {
        return ESharSaveOperationResult::AlreadyTerminal;
    }
    Snapshot.State = State;
    Snapshot.TerminalResult = Result;
    return ESharSaveOperationResult::Accepted;
}

ESharSaveOperationResult USharSaveRepositorySubsystem::Resolve(
    const FSharSaveOperationResolution& Resolution
)
{
    FSharSaveOperationSnapshot* Snapshot =
        FindOperation(Resolution.OperationId);
    if (Snapshot == nullptr)
    {
        return ESharSaveOperationResult::NotFound;
    }
    if (Snapshot->bReleased)
    {
        return ESharSaveOperationResult::Released;
    }
    if (IsTerminalState(Snapshot->State))
    {
        return ESharSaveOperationResult::AlreadyTerminal;
    }
    if (Snapshot->Request.OperationRevision != Resolution.OperationRevision
        || Snapshot->Request.ContainerRevision
            != Resolution.ContainerRevision)
    {
        return ESharSaveOperationResult::StaleRevision;
    }
    switch (Resolution.Command)
    {
    case ESharSaveResolutionCommand::Fail:
        return PublishTerminal(
            *Snapshot,
            ESharSaveOperationState::Failed,
            ESharSaveTerminalResult::Failed
        );
    case ESharSaveResolutionCommand::Timeout:
        return PublishTerminal(
            *Snapshot,
            ESharSaveOperationState::TimedOut,
            ESharSaveTerminalResult::TimedOut
        );
    case ESharSaveResolutionCommand::Cancel:
        return PublishTerminal(
            *Snapshot,
            ESharSaveOperationState::Cancelled,
            ESharSaveTerminalResult::Cancelled
        );
    default:
        return ESharSaveOperationResult::InvalidRequest;
    }
}

ESharSaveOperationResult USharSaveRepositorySubsystem::Release(
    const FName& OperationId
)
{
    FSharSaveOperationSnapshot* Snapshot = FindOperation(OperationId);
    if (Snapshot == nullptr)
    {
        return ESharSaveOperationResult::NotFound;
    }
    if (Snapshot->bReleased)
    {
        return ESharSaveOperationResult::Released;
    }
    if (!IsTerminalState(Snapshot->State))
    {
        return ESharSaveOperationResult::InvalidState;
    }
    Snapshot->bReleased = true;
    Snapshot->State = ESharSaveOperationState::Released;
    return ESharSaveOperationResult::Accepted;
}

ESharSaveOperationState USharSaveRepositorySubsystem::GetState(
    const FName& OperationId
) const
{
    const FSharSaveOperationSnapshot* Snapshot = FindOperation(OperationId);
    return Snapshot == nullptr
        ? ESharSaveOperationState::Failed
        : Snapshot->State;
}

ESharSaveTerminalResult USharSaveRepositorySubsystem::GetTerminalResult(
    const FName& OperationId
) const
{
    const FSharSaveOperationSnapshot* Snapshot = FindOperation(OperationId);
    return Snapshot == nullptr
        ? ESharSaveTerminalResult::None
        : Snapshot->TerminalResult;
}

FSharSaveSlotState USharSaveRepositorySubsystem::GetSlotState(
    const FSharSaveSlotId& Slot
) const
{
    const FSharSaveSlotState* SlotState = FindSlot(Slot);
    return SlotState == nullptr ? FSharSaveSlotState{} : *SlotState;
}

int32 USharSaveRepositorySubsystem::GetUnreleasedOperationCount() const
{
    int32 Count = 0;
    for (const FSharSaveOperationSnapshot& Snapshot : Operations)
    {
        Count += Snapshot.bReleased ? 0 : 1;
    }
    return Count;
}
