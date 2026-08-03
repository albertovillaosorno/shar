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
//   - Shar mission runtime composition module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Shar mission runtime composition module.
// - Description:
//   - Implements the declared composition module responsibility for project.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Shar mission runtime composition module.

#include "Missions/SharMissionRuntime.h"

#include "Missions/SharMissionDefinition.h"

void USharMissionRuntime::SetFailure(const TCHAR* Message)
{
    LastError = FText::FromString(Message);
}

const FSharMissionStageDefinition* USharMissionRuntime::FindStage(
    const FName& StageId
) const
{
    if (ActiveDefinition == nullptr)
    {
        return nullptr;
    }
    // jig-ignore-next-line: exact syntax is indivisible
    for (const FSharMissionStageDefinition& StageDefinition : ActiveDefinition->Stages)
    {
        if (StageDefinition.StageId == StageId)
        {
            return &StageDefinition;
        }
    }
    return nullptr;
}

bool USharMissionRuntime::MoveToStage(const FName& StageId)
{
    const FSharMissionStageDefinition* StageDefinition = FindStage(StageId);
    if (StageDefinition == nullptr)
    {
        SetFailure(TEXT("Mission transition references an unavailable stage."));
        State = ESharMissionRuntimeState::Failed;
        return false;
    }

    ActiveStageId = StageId;
    switch (StageDefinition->TerminalOutcome)
    {
        case ESharMissionTerminalOutcome::Success:
            State = ESharMissionRuntimeState::Succeeded;
            break;
        case ESharMissionTerminalOutcome::Failure:
            State = ESharMissionRuntimeState::Failed;
            break;
        case ESharMissionTerminalOutcome::Abort:
            State = ESharMissionRuntimeState::Aborted;
            break;
        case ESharMissionTerminalOutcome::None:
            State = ESharMissionRuntimeState::Active;
            break;
        default:
            SetFailure(TEXT("Mission stage uses an unknown terminal outcome."));
            State = ESharMissionRuntimeState::Failed;
            return false;
    }
    return true;
}

bool USharMissionRuntime::StartMission(USharMissionDefinition* Definition)
{
    if (State == ESharMissionRuntimeState::Active)
    {
        // jig-ignore-next-line: exact syntax is indivisible
        SetFailure(TEXT("An active mission must finish or abort before another starts."));
        return false;
    }
    if (Definition == nullptr)
    {
        SetFailure(TEXT("Mission runtime requires a definition."));
        return false;
    }

    TArray<FText> ValidationErrors;
    Definition->GatherValidationErrors(ValidationErrors);
    if (!ValidationErrors.IsEmpty())
    {
        SetFailure(TEXT("Mission definition failed validation."));
        return false;
    }

    ActiveDefinition = Definition;
    LastError = FText();
    return MoveToStage(Definition->InitialStageId);
}

bool USharMissionRuntime::ResolveObjective(const bool bSucceeded)
{
    if (State != ESharMissionRuntimeState::Active)
    {
        SetFailure(TEXT("Only an active mission can resolve an objective."));
        return false;
    }
    // jig-ignore-next-line: exact syntax is indivisible
    const FSharMissionStageDefinition* StageDefinition = FindStage(ActiveStageId);
    if (StageDefinition == nullptr)
    {
        SetFailure(TEXT("Active mission stage is unavailable."));
        State = ESharMissionRuntimeState::Failed;
        return false;
    }

    const FName NextStageId = bSucceeded
        ? StageDefinition->SuccessStageId
        : StageDefinition->FailureStageId;
    if (NextStageId.IsNone())
    {
        State = bSucceeded
            ? ESharMissionRuntimeState::Succeeded
            : ESharMissionRuntimeState::Failed;
        return true;
    }
    return MoveToStage(NextStageId);
}

bool USharMissionRuntime::AbortMission()
{
    if (State != ESharMissionRuntimeState::Active)
    {
        SetFailure(TEXT("Only an active mission can be aborted."));
        return false;
    }
    State = ESharMissionRuntimeState::Aborted;
    return true;
}

ESharMissionRuntimeState USharMissionRuntime::GetState() const
{
    return State;
}

FName USharMissionRuntime::GetActiveStageId() const
{
    return ActiveStageId;
}

FText USharMissionRuntime::GetLastError() const
{
    return LastError;
}

FSharMissionRuntimeSnapshot USharMissionRuntime::CreateSnapshot() const
{
    FSharMissionRuntimeSnapshot Snapshot;
    if (ActiveDefinition != nullptr)
    {
        Snapshot.MissionId = ActiveDefinition->GetPrimaryAssetId();
        Snapshot.SaveSchemaVersion = ActiveDefinition->SaveSchemaVersion;
    }
    Snapshot.StageId = ActiveStageId;
    Snapshot.State = State;
    return Snapshot;
}
