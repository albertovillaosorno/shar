// File: SharCheatRecognition.cpp
// Path: src/uproject/Source/SharCheats/Private/Cheats/SharCheatRecognition.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: semantic token admission, four-token lookup, prerequisite evaluation, and typed activation publication only.
// Specification: docs/technical/unreal/progression-collectibles-and-cheats.md
// LARGE-FILE owner=SharCheats; reason=cohesive deterministic recognizer transition implementation;
// split=extract input-delivery guards when more semantic input sources exist;
// validation=validate.sh SharCheats plus Unreal automation; review=2027-01.

#include "Cheats/SharCheatSubsystem.h"

#include "Cheats/SharCheatContracts.h"

#include "Cheats/SharCheatEffectSubsystem.h"
#include "Meta/SharMetaCatalogDefinition.h"
#include "Meta/SharMetaCatalogSubsystem.h"

ESharCheatOperationResult USharCheatSubsystem::AcceptInput(
    const FSharCheatInputEvent& InputEvent
)
{
    FSharCheatRecognizerSnapshot* Recognizer =
        FindRecognizer(InputEvent.RecognitionId);
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
    if (!IsCanonicalIdentity(InputEvent.DeliveryId)
        || InputEvent.InputOrdinal <= 0)
    {
        return ESharCheatOperationResult::InvalidRequest;
    }
    if (!InputMatchesRecognizer(*Recognizer, InputEvent))
    {
        (void)PublishOutcome(
            *Recognizer,
            ESharCheatRecognizerState::Rejected,
            ESharCheatRecognitionOutcome::InputCancelled
        );
        return ESharCheatOperationResult::StaleRevision;
    }
    if (InputEvent.InputOrdinal > Recognizer->Request.TimeoutOrdinal)
    {
        (void)PublishOutcome(
            *Recognizer,
            ESharCheatRecognizerState::Rejected,
            ESharCheatRecognitionOutcome::InputCancelled
        );
        return ESharCheatOperationResult::InputCancelled;
    }
    if (Recognizer->AcceptedDeliveryIds.ContainsByPredicate(
            [&InputEvent](const FName& DeliveryId)
            {
                return DeliveryId == InputEvent.DeliveryId;
            }
        ))
    {
        return ESharCheatOperationResult::DuplicateInput;
    }
    if (InputEvent.Transition != ESharCheatInputTransition::TokenDown)
    {
        return ESharCheatOperationResult::IgnoredInput;
    }
    Recognizer->AcceptedDeliveryIds.Add(InputEvent.DeliveryId);
    Recognizer->AcceptedTokens.Add(InputEvent.Token);
    Recognizer->State = ESharCheatRecognizerState::Collecting;
    if (Recognizer->AcceptedTokens.Num()
        < FSharCheatDefinition::RequiredInputTokenCount)
    {
        return ESharCheatOperationResult::Accepted;
    }
    return CompleteSequence(*Recognizer);
}

bool USharCheatSubsystem::InputMatchesRecognizer(
    const FSharCheatRecognizerSnapshot& Recognizer,
    const FSharCheatInputEvent& InputEvent
)
{
    return InputEvent.LocalPlayerId == Recognizer.Request.LocalPlayerId
        && InputEvent.ControllerId == Recognizer.Request.ControllerId
        && InputEvent.CatalogRevision == Recognizer.Request.CatalogRevision
        && InputEvent.ContextRevision == Recognizer.Request.ContextRevision
        && InputEvent.InputProfileRevision
            == Recognizer.Request.InputProfileRevision;
}

bool USharCheatSubsystem::PrerequisiteSatisfied(
    const ESharCheatPrerequisite Prerequisite,
    const FSharCheatRuntimeContext& Context
)
{
    switch (Prerequisite)
    {
    case ESharCheatPrerequisite::None:
        return true;
    case ESharCheatPrerequisite::LoadedProfile:
        return Context.bProfileLoaded;
    case ESharCheatPrerequisite::CompletedStory:
        return Context.bProfileLoaded && Context.bStoryCompleted;
    case ESharCheatPrerequisite::DeveloperBuild:
        return Context.bDeveloperBuild;
    default:
        return false;
    }
}

ESharCheatEffectAction USharCheatSubsystem::ResolveAction(
    const FSharCheatDefinition& Definition,
    const USharCheatEffectSubsystem& EffectSubsystem,
    const FName& LocalPlayerId
)
{
    if (Definition.ActivationMode == ESharCheatActivationMode::ImmediateCommand)
    {
        return ESharCheatEffectAction::Execute;
    }
    if (Definition.ActivationMode == ESharCheatActivationMode::EnableOnly)
    {
        return ESharCheatEffectAction::Enable;
    }
    return EffectSubsystem.IsEnabled(LocalPlayerId, Definition.CheatId)
        ? ESharCheatEffectAction::Disable
        : ESharCheatEffectAction::Enable;
}

ESharCheatOperationResult USharCheatSubsystem::CompleteSequence(
    FSharCheatRecognizerSnapshot& Recognizer
)
{
    if (CatalogSubsystem == nullptr || EffectSubsystem == nullptr)
    {
        return ESharCheatOperationResult::NotConfigured;
    }
    if (Recognizer.AcceptedTokens.Num()
        != FSharCheatDefinition::RequiredInputTokenCount)
    {
        return ESharCheatOperationResult::InvalidState;
    }
    const FSharCheatDefinition* Definition =
        CatalogSubsystem->FindCheatBySequence(
            Recognizer.Request.CatalogId,
            Recognizer.AcceptedTokens
        );
    if (Definition == nullptr)
    {
        (void)PublishOutcome(
            Recognizer,
            ESharCheatRecognizerState::Rejected,
            ESharCheatRecognitionOutcome::UnknownSequence
        );
        return ESharCheatOperationResult::UnknownSequence;
    }
    const FSharCheatRuntimeContext Context = EffectSubsystem->GetContext();
    if (!Context.bCheatsAvailable)
    {
        (void)PublishOutcome(
            Recognizer,
            ESharCheatRecognizerState::Rejected,
            ESharCheatRecognitionOutcome::Unavailable
        );
        return ESharCheatOperationResult::Unavailable;
    }
    if (!PrerequisiteSatisfied(Definition->Prerequisite, Context))
    {
        (void)PublishOutcome(
            Recognizer,
            ESharCheatRecognizerState::Rejected,
            ESharCheatRecognitionOutcome::PrerequisiteFailed
        );
        return ESharCheatOperationResult::PrerequisiteFailed;
    }
    FSharCheatActivationRequest Activation;
    Activation.ActivationId = Recognizer.Request.RecognitionId;
    Activation.RecognitionId = Recognizer.Request.RecognitionId;
    Activation.CheatId = Definition->CheatId;
    Activation.LocalPlayerId = Recognizer.Request.LocalPlayerId;
    Activation.CatalogId = Recognizer.Request.CatalogId;
    Activation.Priority = ESharCheatEffectPriority::User;
    Activation.Action = ResolveAction(
        *Definition,
        *EffectSubsystem,
        Recognizer.Request.LocalPlayerId
    );
    Activation.CatalogRevision = Recognizer.Request.CatalogRevision;
    Activation.ContextRevision = Recognizer.Request.ContextRevision;
    Activation.ActivationRevision = Recognizer.Request.RecognitionRevision;
    const ESharCheatOperationResult Result = EffectSubsystem->Submit(Activation);
    if (Result != ESharCheatOperationResult::Accepted)
    {
        (void)PublishOutcome(
            Recognizer,
            ESharCheatRecognizerState::Rejected,
            ESharCheatRecognitionOutcome::Unavailable
        );
        return Result;
    }
    Recognizer.MatchedCheatId = Definition->CheatId;
    return PublishOutcome(
        Recognizer,
        ESharCheatRecognizerState::Accepted,
        ESharCheatRecognitionOutcome::Matched
    );
}

ESharCheatOperationResult USharCheatSubsystem::PublishOutcome(
    FSharCheatRecognizerSnapshot& Recognizer,
    const ESharCheatRecognizerState State,
    const ESharCheatRecognitionOutcome Outcome
)
{
    if (Recognizer.State == ESharCheatRecognizerState::Released)
    {
        return ESharCheatOperationResult::Released;
    }
    if (IsTerminalState(Recognizer.State))
    {
        return ESharCheatOperationResult::AlreadyTerminal;
    }
    Recognizer.State = State;
    Recognizer.Outcome = Outcome;
    return ESharCheatOperationResult::Accepted;
}
