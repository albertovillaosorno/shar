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
//   - Shar cheat subsystem composition module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Shar cheat subsystem composition module.
// - Description:
//   - Implements the declared composition module responsibility for project.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Shar cheat subsystem composition module.

#include "Cheats/SharCheatSubsystem.h"

#include "Cheats/SharCheatContracts.h"
#include "Cheats/SharCheatIdentity.h"

#include "Cheats/SharCheatEffectSubsystem.h"
#include "Meta/SharMetaCatalogSubsystem.h"

bool USharCheatSubsystem::Configure(
    USharMetaCatalogSubsystem* InCatalogSubsystem,
    USharCheatEffectSubsystem* InEffectSubsystem
)
{
    if (bConfigured || InCatalogSubsystem == nullptr
        || InEffectSubsystem == nullptr || !InCatalogSubsystem->IsActive()
        || !IsRevisionToken(InEffectSubsystem->GetContext().ContextRevision))
    {
        return false;
    }
    CatalogSubsystem = InCatalogSubsystem;
    EffectSubsystem = InEffectSubsystem;
    bConfigured = true;
    return true;
}

ESharCheatOperationResult USharCheatSubsystem::Arm(
    const FSharCheatArmRequest& Request
)
{
    const ESharCheatOperationResult ValidationResult =
        ValidateArmRequest(Request);
    if (ValidationResult != ESharCheatOperationResult::Accepted)
    {
        return ValidationResult;
    }
    FSharCheatRecognizerSnapshot Recognizer;
    Recognizer.Request = Request;
    Recognizer.State = ESharCheatRecognizerState::Armed;
    Recognizers.Add(Recognizer);
    return ESharCheatOperationResult::Accepted;
}

ESharCheatOperationResult USharCheatSubsystem::UpdateContext(
    const FSharCheatContextUpdate& Update
)
{
    if (!bConfigured || EffectSubsystem == nullptr)
    {
        return ESharCheatOperationResult::NotConfigured;
    }
    const ESharCheatOperationResult Result =
        EffectSubsystem->UpdateContext(Update);
    if (Result == ESharCheatOperationResult::Accepted)
    {
        CancelActiveRecognizers();
    }
    return Result;
}

ESharCheatOperationResult USharCheatSubsystem::Cancel(
    const FName& RecognitionId,
    const FString& RecognitionRevision
)
{
    FSharCheatRecognizerSnapshot* Recognizer = FindRecognizer(RecognitionId);
    if (Recognizer == nullptr)
    {
        return ESharCheatOperationResult::NotFound;
    }
    if (Recognizer->State == ESharCheatRecognizerState::Released)
    {
        return ESharCheatOperationResult::Released;
    }
    if (IsTerminalState(Recognizer->State))
    {
        return ESharCheatOperationResult::AlreadyTerminal;
    }
    if (RecognitionRevision != Recognizer->Request.RecognitionRevision)
    {
        return ESharCheatOperationResult::StaleRevision;
    }
    return PublishOutcome(
        *Recognizer,
        ESharCheatRecognizerState::Rejected,
        ESharCheatRecognitionOutcome::InputCancelled
    );
}

ESharCheatOperationResult USharCheatSubsystem::Release(
    const FName& RecognitionId
)
{
    FSharCheatRecognizerSnapshot* Recognizer = FindRecognizer(RecognitionId);
    if (Recognizer == nullptr)
    {
        return ESharCheatOperationResult::NotFound;
    }
    if (Recognizer->State == ESharCheatRecognizerState::Released)
    {
        return ESharCheatOperationResult::Released;
    }
    if (!IsTerminalState(Recognizer->State))
    {
        return ESharCheatOperationResult::InvalidState;
    }
    Recognizer->State = ESharCheatRecognizerState::Released;
    return ESharCheatOperationResult::Accepted;
}

ESharCheatRecognizerState USharCheatSubsystem::GetRecognizerState(
    const FName& RecognitionId
) const
{
    const FSharCheatRecognizerSnapshot* Recognizer =
        FindRecognizer(RecognitionId);
    return Recognizer == nullptr
        ? ESharCheatRecognizerState::Released
        : Recognizer->State;
}

ESharCheatRecognitionOutcome USharCheatSubsystem::GetRecognitionOutcome(
    const FName& RecognitionId
) const
{
    const FSharCheatRecognizerSnapshot* Recognizer =
        FindRecognizer(RecognitionId);
    return Recognizer == nullptr
        ? ESharCheatRecognitionOutcome::None
        : Recognizer->Outcome;
}

FSharCheatRuntimeObservation USharCheatSubsystem::GetObservation() const
{
    FSharCheatRuntimeObservation Observation = EffectSubsystem == nullptr
        ? FSharCheatRuntimeObservation{}
        : EffectSubsystem->GetObservation();
    Observation.UnreleasedRecognizerCount = CountUnreleasedRecognizers();
    return Observation;
}

bool USharCheatSubsystem::IsCanonicalIdentity(const FName& Candidate)
{
    return SharCheatIdentity::IsCanonical(Candidate);
}

bool USharCheatSubsystem::IsRevisionToken(const FString& Revision)
{
    return !Revision.IsEmpty() && Revision.Contains(TEXT(":"));
}

bool USharCheatSubsystem::IsTerminalState(
    const ESharCheatRecognizerState State
)
{
    return State == ESharCheatRecognizerState::Accepted
        || State == ESharCheatRecognizerState::Rejected;
}

ESharCheatOperationResult USharCheatSubsystem::ValidateArmRequest(
    const FSharCheatArmRequest& Request
) const
{
    // jig-ignore-next-line: exact syntax is indivisible
    if (!bConfigured || CatalogSubsystem == nullptr || EffectSubsystem == nullptr)
    {
        return ESharCheatOperationResult::NotConfigured;
    }
    if (!CatalogSubsystem->IsActive())
    {
        return ESharCheatOperationResult::CatalogInactive;
    }
    if (!IsArmRequestWellFormed(Request))
    {
        return ESharCheatOperationResult::InvalidRequest;
    }
    if (Request.CatalogRevision != CatalogSubsystem->GetCatalogRevision()
        || Request.ContextRevision
            != EffectSubsystem->GetContext().ContextRevision)
    {
        return ESharCheatOperationResult::StaleRevision;
    }
    if (!EffectSubsystem->GetContext().bCheatsAvailable)
    {
        return ESharCheatOperationResult::Unavailable;
    }
    if (CatalogSubsystem->FindCatalog(Request.CatalogId) == nullptr)
    {
        return ESharCheatOperationResult::CatalogMissing;
    }
    if (FindRecognizer(Request.RecognitionId) != nullptr)
    {
        return ESharCheatOperationResult::DuplicateRequest;
    }
    return HasActiveRecognizerForPlayer(Request.LocalPlayerId)
        ? ESharCheatOperationResult::ConflictingRequest
        : ESharCheatOperationResult::Accepted;
}

bool USharCheatSubsystem::IsArmRequestWellFormed(
    const FSharCheatArmRequest& Request
)
{
    return IsCanonicalIdentity(Request.RecognitionId)
        && IsCanonicalIdentity(Request.LocalPlayerId)
        && IsCanonicalIdentity(Request.ControllerId)
        && IsCanonicalIdentity(Request.CatalogId)
        && IsRevisionToken(Request.CatalogRevision)
        && IsRevisionToken(Request.ContextRevision)
        && IsRevisionToken(Request.InputProfileRevision)
        && IsRevisionToken(Request.RecognitionRevision)
        && Request.TimeoutOrdinal > 0;
}

bool USharCheatSubsystem::HasActiveRecognizerForPlayer(
    const FName& LocalPlayerId
) const
{
    return Recognizers.ContainsByPredicate(
        [&LocalPlayerId](const FSharCheatRecognizerSnapshot& Recognizer)
        {
            return Recognizer.Request.LocalPlayerId == LocalPlayerId
                && Recognizer.State != ESharCheatRecognizerState::Released
                && !IsTerminalState(Recognizer.State);
        }
    );
}

FSharCheatRecognizerSnapshot* USharCheatSubsystem::FindRecognizer(
    const FName& RecognitionId
)
{
    for (FSharCheatRecognizerSnapshot& Recognizer : Recognizers)
    {
        if (Recognizer.Request.RecognitionId == RecognitionId)
        {
            return &Recognizer;
        }
    }
    return nullptr;
}

const FSharCheatRecognizerSnapshot* USharCheatSubsystem::FindRecognizer(
    const FName& RecognitionId
) const
{
    for (const FSharCheatRecognizerSnapshot& Recognizer : Recognizers)
    {
        if (Recognizer.Request.RecognitionId == RecognitionId)
        {
            return &Recognizer;
        }
    }
    return nullptr;
}

void USharCheatSubsystem::CancelActiveRecognizers()
{
    for (FSharCheatRecognizerSnapshot& Recognizer : Recognizers)
    {
        if (Recognizer.State != ESharCheatRecognizerState::Released
            && !IsTerminalState(Recognizer.State))
        {
            (void)PublishOutcome(
                Recognizer,
                ESharCheatRecognizerState::Rejected,
                ESharCheatRecognitionOutcome::InputCancelled
            );
        }
    }
}

int32 USharCheatSubsystem::CountUnreleasedRecognizers() const
{
    int32 Count = 0;
    for (const FSharCheatRecognizerSnapshot& Recognizer : Recognizers)
    {
        // jig-ignore-next-line: exact syntax is indivisible
        Count += Recognizer.State == ESharCheatRecognizerState::Released ? 0 : 1;
    }
    return Count;
}
