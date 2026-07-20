// File: SharCheatEffectSubsystem.cpp
// Path: src/uproject/Source/SharCheats/Private/Cheats/SharCheatEffectSubsystem.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: cheat-effect configuration, request validation, deterministic queueing, and immutable queries only.
// Specification: docs/technical/unreal/progression-collectibles-and-cheats.md
// LARGE-FILE owner=SharCheats; reason=cohesive effect request admission and query behavior;
// split=separate validation helpers if new effect-owner families add policies;
// validation=validate.sh SharCheats plus Unreal automation; review=2027-01.

#include "Cheats/SharCheatEffectSubsystem.h"

#include "Cheats/SharCheatContracts.h"
#include "Cheats/SharCheatIdentity.h"

#include "Meta/SharMetaCatalogDefinition.h"
#include "Meta/SharMetaCatalogSubsystem.h"

bool USharCheatEffectSubsystem::Configure(
    USharMetaCatalogSubsystem* InCatalogSubsystem,
    const FSharCheatRuntimeContext& InitialContext
)
{
    if (bConfigured || InCatalogSubsystem == nullptr
        || !InCatalogSubsystem->IsActive() || !IsContextValid(InitialContext))
    {
        return false;
    }
    CatalogSubsystem = InCatalogSubsystem;
    Context = InitialContext;
    bConfigured = true;
    return true;
}

ESharCheatOperationResult USharCheatEffectSubsystem::UpdateContext(
    const FSharCheatContextUpdate& Update
)
{
    if (!bConfigured)
    {
        return ESharCheatOperationResult::NotConfigured;
    }
    if (Update.ExpectedContextRevision != Context.ContextRevision)
    {
        return ESharCheatOperationResult::StaleRevision;
    }
    if (!IsContextValid(Update.UpdatedContext)
        || Update.UpdatedContext.ContextRevision == Context.ContextRevision)
    {
        return ESharCheatOperationResult::InvalidRequest;
    }
    const FSharCheatRuntimeContext PreviousContext = Context;
    Context = Update.UpdatedContext;
    CancelStaleActivations();
    ExpireEffects(PreviousContext);
    return ESharCheatOperationResult::Accepted;
}

ESharCheatOperationResult USharCheatEffectSubsystem::Submit(
    const FSharCheatActivationRequest& Request
)
{
    const ESharCheatOperationResult ValidationResult = ValidateRequest(Request);
    if (ValidationResult != ESharCheatOperationResult::Accepted)
    {
        return ValidationResult;
    }
    FSharCheatActivationSnapshot Snapshot;
    Snapshot.Request = Request;
    Snapshot.State = ESharCheatEffectState::Queued;
    Snapshot.TerminalResult = ESharCheatTerminalResult::None;
    Snapshot.InsertionSequence = NextInsertionSequence;
    ++NextInsertionSequence;
    Activations.Add(Snapshot);
    return ESharCheatOperationResult::Accepted;
}

int32 USharCheatEffectSubsystem::GetQueuePosition(
    const FName& ActivationId
) const
{
    const FSharCheatActivationSnapshot* Activation =
        FindActivation(ActivationId);
    if (Activation == nullptr || Activation->State != ESharCheatEffectState::Queued)
    {
        return 0;
    }
    int32 Position = 1;
    for (const FSharCheatActivationSnapshot& Candidate : Activations)
    {
        if (Candidate.State == ESharCheatEffectState::Queued
            && Outranks(Candidate, *Activation))
        {
            ++Position;
        }
    }
    return Position;
}

ESharCheatEffectState USharCheatEffectSubsystem::GetActivationState(
    const FName& ActivationId
) const
{
    const FSharCheatActivationSnapshot* Activation =
        FindActivation(ActivationId);
    return Activation == nullptr
        ? ESharCheatEffectState::Released
        : Activation->State;
}

ESharCheatTerminalResult USharCheatEffectSubsystem::GetTerminalResult(
    const FName& ActivationId
) const
{
    const FSharCheatActivationSnapshot* Activation =
        FindActivation(ActivationId);
    return Activation == nullptr
        ? ESharCheatTerminalResult::None
        : Activation->TerminalResult;
}

bool USharCheatEffectSubsystem::IsEnabled(
    const FName& LocalPlayerId,
    const FName& CheatId
) const
{
    return EnabledEffects.ContainsByPredicate(
        [&LocalPlayerId, &CheatId](const FSharEnabledCheatEffect& Effect)
        {
            return Effect.LocalPlayerId == LocalPlayerId
                && Effect.CheatId == CheatId;
        }
    );
}

FSharCheatRuntimeContext USharCheatEffectSubsystem::GetContext() const
{
    return Context;
}

FSharCheatRuntimeObservation USharCheatEffectSubsystem::GetObservation() const
{
    FSharCheatRuntimeObservation Observation;
    Observation.Context = Context;
    Observation.EnabledEffects = EnabledEffects;
    Observation.UnreleasedActivationCount = CountUnreleasedActivations();
    return Observation;
}

bool USharCheatEffectSubsystem::IsCanonicalIdentity(const FName& Candidate)
{
    return SharCheatIdentity::IsCanonical(Candidate);
}

bool USharCheatEffectSubsystem::IsRevisionToken(const FString& Revision)
{
    return !Revision.IsEmpty() && Revision.Contains(TEXT(":"));
}

bool USharCheatEffectSubsystem::IsTerminalState(
    const ESharCheatEffectState State
)
{
    return State == ESharCheatEffectState::Completed
        || State == ESharCheatEffectState::Failed
        || State == ESharCheatEffectState::Cancelled;
}

bool USharCheatEffectSubsystem::Outranks(
    const FSharCheatActivationSnapshot& Left,
    const FSharCheatActivationSnapshot& Right
)
{
    if (Left.Request.Priority != Right.Request.Priority)
    {
        return static_cast<uint8>(Left.Request.Priority)
            > static_cast<uint8>(Right.Request.Priority);
    }
    const FString LeftId = Left.Request.ActivationId.ToString();
    const FString RightId = Right.Request.ActivationId.ToString();
    if (LeftId != RightId)
    {
        return LeftId < RightId;
    }
    return Left.InsertionSequence < Right.InsertionSequence;
}

bool USharCheatEffectSubsystem::IsContextValid(
    const FSharCheatRuntimeContext& Candidate
)
{
    return IsRevisionToken(Candidate.ContextRevision)
        && IsRevisionToken(Candidate.ProfileRevision)
        && IsRevisionToken(Candidate.ApplicationModeRevision)
        && IsRevisionToken(Candidate.SessionRevision)
        && IsRevisionToken(Candidate.ChapterRevision)
        && IsRevisionToken(Candidate.MissionRevision);
}

bool USharCheatEffectSubsystem::RequestMatchesAuthority(
    const FSharCheatActivationRequest& Request
) const
{
    return CatalogSubsystem != nullptr
        && Request.CatalogRevision == CatalogSubsystem->GetCatalogRevision()
        && Request.ContextRevision == Context.ContextRevision;
}

ESharCheatOperationResult USharCheatEffectSubsystem::ValidateRequest(
    const FSharCheatActivationRequest& Request
) const
{
    if (!bConfigured || CatalogSubsystem == nullptr)
    {
        return ESharCheatOperationResult::NotConfigured;
    }
    if (!CatalogSubsystem->IsActive())
    {
        return ESharCheatOperationResult::CatalogInactive;
    }
    if (!IsRequestWellFormed(Request))
    {
        return ESharCheatOperationResult::InvalidRequest;
    }
    if (!RequestMatchesAuthority(Request))
    {
        return ESharCheatOperationResult::StaleRevision;
    }
    if (FindActivation(Request.ActivationId) != nullptr)
    {
        return ESharCheatOperationResult::DuplicateRequest;
    }
    return ValidateDefinitionState(Request);
}

bool USharCheatEffectSubsystem::IsRequestWellFormed(
    const FSharCheatActivationRequest& Request
)
{
    return IsCanonicalIdentity(Request.ActivationId)
        && IsCanonicalIdentity(Request.RecognitionId)
        && IsCanonicalIdentity(Request.CheatId)
        && IsCanonicalIdentity(Request.LocalPlayerId)
        && IsCanonicalIdentity(Request.CatalogId)
        && IsRevisionToken(Request.CatalogRevision)
        && IsRevisionToken(Request.ContextRevision)
        && IsRevisionToken(Request.ActivationRevision);
}

ESharCheatOperationResult
USharCheatEffectSubsystem::ValidateDefinitionState(
    const FSharCheatActivationRequest& Request
) const
{
    const USharMetaCatalogDefinition* Catalog =
        CatalogSubsystem->FindCatalog(Request.CatalogId);
    const FSharCheatDefinition* Definition = Catalog == nullptr
        ? nullptr
        : Catalog->FindCheat(Request.CheatId);
    if (Definition == nullptr)
    {
        return ESharCheatOperationResult::DefinitionMissing;
    }
    if (!IsActionSupported(*Definition, Request.Action))
    {
        return ESharCheatOperationResult::UnsupportedAction;
    }
    const bool bEnabled = IsEnabled(Request.LocalPlayerId, Request.CheatId);
    if ((Request.Action == ESharCheatEffectAction::Enable && bEnabled)
        || (Request.Action == ESharCheatEffectAction::Disable && !bEnabled))
    {
        return ESharCheatOperationResult::AlreadyApplied;
    }
    return HasConflictingActivation(Request)
        ? ESharCheatOperationResult::ConflictingRequest
        : ESharCheatOperationResult::Accepted;
}

bool USharCheatEffectSubsystem::IsActionSupported(
    const FSharCheatDefinition& Definition,
    const ESharCheatEffectAction Action
)
{
    if (Definition.ActivationMode == ESharCheatActivationMode::ImmediateCommand)
    {
        return Action == ESharCheatEffectAction::Execute;
    }
    if (Definition.ActivationMode == ESharCheatActivationMode::EnableOnly)
    {
        return Action == ESharCheatEffectAction::Enable;
    }
    return Action == ESharCheatEffectAction::Enable
        || Action == ESharCheatEffectAction::Disable;
}

bool USharCheatEffectSubsystem::HasConflictingActivation(
    const FSharCheatActivationRequest& Request
) const
{
    return Activations.ContainsByPredicate(
        [&Request](const FSharCheatActivationSnapshot& Activation)
        {
            return Activation.State != ESharCheatEffectState::Released
                && !IsTerminalState(Activation.State)
                && Activation.Request.LocalPlayerId == Request.LocalPlayerId
                && Activation.Request.CheatId == Request.CheatId;
        }
    );
}

FSharCheatActivationSnapshot* USharCheatEffectSubsystem::FindActivation(
    const FName& ActivationId
)
{
    for (FSharCheatActivationSnapshot& Activation : Activations)
    {
        if (Activation.Request.ActivationId == ActivationId)
        {
            return &Activation;
        }
    }
    return nullptr;
}

const FSharCheatActivationSnapshot* USharCheatEffectSubsystem::FindActivation(
    const FName& ActivationId
) const
{
    for (const FSharCheatActivationSnapshot& Activation : Activations)
    {
        if (Activation.Request.ActivationId == ActivationId)
        {
            return &Activation;
        }
    }
    return nullptr;
}

int32 USharCheatEffectSubsystem::CountUnreleasedActivations() const
{
    int32 Count = 0;
    for (const FSharCheatActivationSnapshot& Activation : Activations)
    {
        Count += Activation.State == ESharCheatEffectState::Released ? 0 : 1;
    }
    return Count;
}
