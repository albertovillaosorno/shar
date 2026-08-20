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
//   - Shar cheat effect lifecycle composition module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Shar cheat effect lifecycle composition module.
// - Description:
//   - Implements the declared composition module responsibility for project.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Shar cheat effect lifecycle composition module.

#include "Cheats/SharCheatEffectSubsystem.h"

#include "Cheats/SharCheatContracts.h"

#include "Meta/SharMetaCatalogDefinition.h"
#include "Meta/SharMetaCatalogSubsystem.h"

ESharCheatOperationResult USharCheatEffectSubsystem::Begin(
    const FName& ActivationId
)
{
    FSharCheatActivationSnapshot* Activation = FindActivation(ActivationId);
    if (Activation == nullptr)
    {
        return ESharCheatOperationResult::NotFound;
    }
    if (Activation->State == ESharCheatEffectState::Released)
    {
        return ESharCheatOperationResult::Released;
    }
    if (IsTerminalState(Activation->State))
    {
        return ESharCheatOperationResult::AlreadyTerminal;
    }
    if (Activation->State != ESharCheatEffectState::Queued)
    {
        return ESharCheatOperationResult::InvalidState;
    }
    if (!IsHead(*Activation))
    {
        return ESharCheatOperationResult::NotHead;
    }
    Activation->State = ESharCheatEffectState::Dispatching;
    return ESharCheatOperationResult::Accepted;
}

ESharCheatOperationResult USharCheatEffectSubsystem::MarkDispatched(
    const FName& ActivationId
)
{
    FSharCheatActivationSnapshot* Activation = FindActivation(ActivationId);
    if (Activation == nullptr)
    {
        return ESharCheatOperationResult::NotFound;
    }
    if (Activation->State == ESharCheatEffectState::Released)
    {
        return ESharCheatOperationResult::Released;
    }
    if (IsTerminalState(Activation->State))
    {
        return ESharCheatOperationResult::AlreadyTerminal;
    }
    if (Activation->State != ESharCheatEffectState::Dispatching)
    {
        return ESharCheatOperationResult::InvalidState;
    }
    Activation->State = ESharCheatEffectState::AwaitingPostcondition;
    return ESharCheatOperationResult::Accepted;
}

ESharCheatOperationResult
USharCheatEffectSubsystem::AcceptPostconditionEvidence(
    const FSharCheatPostconditionEvidence& Evidence
)
{
    FSharCheatActivationSnapshot* Activation =
        FindActivation(Evidence.ActivationId);
    if (Activation == nullptr)
    {
        return ESharCheatOperationResult::NotFound;
    }
    if (Activation->State == ESharCheatEffectState::Released)
    {
        return ESharCheatOperationResult::Released;
    }
    if (IsTerminalState(Activation->State))
    {
        return ESharCheatOperationResult::AlreadyTerminal;
    }
    if (Activation->State != ESharCheatEffectState::AwaitingPostcondition)
    {
        return ESharCheatOperationResult::InvalidState;
    }
    if (!EvidenceMatches(*Activation, Evidence))
    {
        return ESharCheatOperationResult::StaleRevision;
    }
    const USharMetaCatalogDefinition* Catalog =
        CatalogSubsystem->FindCatalog(Activation->Request.CatalogId);
    const FSharCheatDefinition* Definition = Catalog == nullptr
        ? nullptr
        : Catalog->FindCheat(Activation->Request.CheatId);
    if (Definition == nullptr)
    {
        return ESharCheatOperationResult::DefinitionMissing;
    }
    ApplySuccessfulPostcondition(*Activation, *Definition);
    return PublishTerminal(
        *Activation,
        ESharCheatEffectState::Completed,
        ESharCheatTerminalResult::Success
    );
}

ESharCheatOperationResult USharCheatEffectSubsystem::Resolve(
    const FSharCheatActivationResolution& Resolution
)
{
    FSharCheatActivationSnapshot* Activation =
        FindActivation(Resolution.ActivationId);
    if (Activation == nullptr)
    {
        return ESharCheatOperationResult::NotFound;
    }
    if (Activation->State == ESharCheatEffectState::Released)
    {
        return ESharCheatOperationResult::Released;
    }
    if (IsTerminalState(Activation->State))
    {
        return ESharCheatOperationResult::AlreadyTerminal;
    }
    if (Resolution.ContextRevision != Activation->Request.ContextRevision
        || Resolution.ActivationRevision
            != Activation->Request.ActivationRevision)
    {
        return ESharCheatOperationResult::StaleRevision;
    }
    return Resolution.Command == ESharCheatResolutionCommand::Cancel
        ? PublishTerminal(
              *Activation,
              ESharCheatEffectState::Cancelled,
              ESharCheatTerminalResult::Cancelled
          )
        : PublishTerminal(
              *Activation,
              ESharCheatEffectState::Failed,
              ESharCheatTerminalResult::Failed
          );
}

ESharCheatOperationResult USharCheatEffectSubsystem::Release(
    const FName& ActivationId
)
{
    FSharCheatActivationSnapshot* Activation = FindActivation(ActivationId);
    if (Activation == nullptr)
    {
        return ESharCheatOperationResult::NotFound;
    }
    if (Activation->State == ESharCheatEffectState::Released)
    {
        return ESharCheatOperationResult::Released;
    }
    if (!IsTerminalState(Activation->State))
    {
        return ESharCheatOperationResult::InvalidState;
    }
    Activation->State = ESharCheatEffectState::Released;
    return ESharCheatOperationResult::Accepted;
}

bool USharCheatEffectSubsystem::IsHead(
    const FSharCheatActivationSnapshot& Activation
) const
{
    for (const FSharCheatActivationSnapshot& Candidate : Activations)
    {
        // jig-ignore-next-line: exact syntax is indivisible
        if (&Candidate == &Activation || Candidate.State == ESharCheatEffectState::Released
            || IsTerminalState(Candidate.State))
        {
            continue;
        }
        if (Candidate.State != ESharCheatEffectState::Queued)
        {
            return false;
        }
        if (Outranks(Candidate, Activation))
        {
            return false;
        }
    }
    return true;
}

bool USharCheatEffectSubsystem::EvidenceMatches(
    const FSharCheatActivationSnapshot& Activation,
    const FSharCheatPostconditionEvidence& Evidence
)
{
    return Evidence.ActivationId == Activation.Request.ActivationId
        && Evidence.CheatId == Activation.Request.CheatId
        && Evidence.CatalogRevision == Activation.Request.CatalogRevision
        && Evidence.ContextRevision == Activation.Request.ContextRevision
        && Evidence.ActivationRevision == Activation.Request.ActivationRevision
        && IsRevisionToken(Evidence.EffectOwnerRevision);
}

ESharCheatOperationResult USharCheatEffectSubsystem::PublishTerminal(
    FSharCheatActivationSnapshot& Activation,
    const ESharCheatEffectState State,
    const ESharCheatTerminalResult Result
)
{
    if (Activation.State == ESharCheatEffectState::Released)
    {
        return ESharCheatOperationResult::Released;
    }
    if (IsTerminalState(Activation.State))
    {
        return ESharCheatOperationResult::AlreadyTerminal;
    }
    Activation.State = State;
    Activation.TerminalResult = Result;
    return ESharCheatOperationResult::Accepted;
}
