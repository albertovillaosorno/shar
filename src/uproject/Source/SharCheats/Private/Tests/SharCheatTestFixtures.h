// File: SharCheatTestFixtures.h
// Path: src/uproject/Source/SharCheats/Private/Tests/SharCheatTestFixtures.h
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: explicit transient meta catalog, cheat context, recognizer, activation, and postcondition fixtures only.
// Specification: docs/technical/unreal/progression-collectibles-and-cheats.md
// LARGE-FILE owner=SharCheats; reason=cohesive typed cheat runtime test fixtures;
// split=extract catalog fixtures when credits and calendar runtimes join SharMeta;
// validation=validate.sh SharCheats plus Unreal automation; review=2027-01.

#pragma once

#include "Cheats/SharCheatEffectSubsystem.h"
#include "Cheats/SharCheatSubsystem.h"
#include "Engine/GameInstance.h"
#include "Meta/SharMetaCatalogDefinition.h"
#include "Meta/SharMetaCatalogSubsystem.h"

constexpr int64 CheatInputTimeoutOrdinal = 100;
constexpr int64 CheatInputOrdinalUp = 2;
constexpr int64 CheatInputOrdinalLeft = 3;
constexpr int64 CheatInputOrdinalDown = 4;
constexpr int64 CheatInputOrdinalRight = 5;
constexpr int32 CheatQueuePositionSecond = 2;
constexpr int32 CheatQueuePositionThird = 3;
constexpr int64 CoinCheatQuantity = 100;

struct FSharCheatRuntimeFixture
{
    UGameInstance* GameInstance = nullptr;
    USharMetaCatalogSubsystem* CatalogSubsystem = nullptr;
    USharCheatEffectSubsystem* EffectSubsystem = nullptr;
    USharCheatSubsystem* CheatSubsystem = nullptr;
};

inline FSharCheatDefinition MakeCheatDefinition(
    const FName& CheatId,
    const TArray<ESharCheatInputToken>& InputTokens,
    const ESharCheatPrerequisite Prerequisite,
    const ESharCheatActivationMode ActivationMode,
    const ESharCheatLifetime Lifetime
)
{
    FSharCheatDefinition Definition;
    Definition.CheatId = CheatId;
    Definition.InputTokens = InputTokens;
    Definition.Prerequisite = Prerequisite;
    Definition.ActivationMode = ActivationMode;
    Definition.Lifetime = Lifetime;
    Definition.SuccessFeedbackEvent = FName(TEXT("cheat_accepted"));
    Definition.UnavailableFeedbackEvent = FName(TEXT("cheat_unavailable"));
    Definition.DisabledFeedbackEvent = FName(TEXT("cheat_disabled"));
    Definition.InvalidSequenceFeedbackEvent = FName(TEXT("cheat_invalid"));
    return Definition;
}

inline USharMetaCatalogDefinition* MakeMetaCatalogDefinition()
{
    auto* Definition = NewObject<USharMetaCatalogDefinition>();
    Definition->CanonicalId = FName(TEXT("base_meta"));
    Definition->DisplayName = FText::FromString(TEXT("Base Meta Catalog"));
    Definition->SourcePackageIds = {FName(TEXT("meta_contract"))};
    Definition->RevisionToken = TEXT("sha256:meta_definition_v1");
    Definition->ValidationProfile = FName(TEXT("base_meta_v1"));
    Definition->OwningFeature = FName(TEXT("base"));

    FSharCheatDefinition UnlockCards = MakeCheatDefinition(
        FName(TEXT("unlock_cards")),
        {
            ESharCheatInputToken::Up,
            ESharCheatInputToken::Left,
            ESharCheatInputToken::Down,
            ESharCheatInputToken::Right,
        },
        ESharCheatPrerequisite::CompletedStory,
        ESharCheatActivationMode::Toggle,
        ESharCheatLifetime::Chapter
    );
    UnlockCards.EffectKind = ESharCheatEffectKind::UnlockOverlay;
    UnlockCards.EffectParameters.TargetId = FName(TEXT("collector_cards"));

    FSharCheatDefinition ShowSpeedometer = MakeCheatDefinition(
        FName(TEXT("show_speedometer")),
        {
            ESharCheatInputToken::Right,
            ESharCheatInputToken::Right,
            ESharCheatInputToken::Left,
            ESharCheatInputToken::Left,
        },
        ESharCheatPrerequisite::LoadedProfile,
        ESharCheatActivationMode::Toggle,
        ESharCheatLifetime::Mission
    );
    ShowSpeedometer.EffectKind = ESharCheatEffectKind::HudOverlay;
    ShowSpeedometer.EffectParameters.TargetId = FName(TEXT("speedometer"));

    FSharCheatDefinition ExtraCoins = MakeCheatDefinition(
        FName(TEXT("extra_coins")),
        {
            ESharCheatInputToken::Up,
            ESharCheatInputToken::Up,
            ESharCheatInputToken::Down,
            ESharCheatInputToken::Down,
        },
        ESharCheatPrerequisite::LoadedProfile,
        ESharCheatActivationMode::ImmediateCommand,
        ESharCheatLifetime::PersistentTransaction
    );
    ExtraCoins.EffectKind = ESharCheatEffectKind::ProgressionTransaction;
    ExtraCoins.EffectParameters.OperationId = FName(TEXT("grant_currency"));
    ExtraCoins.EffectParameters.TargetId = FName(TEXT("coins"));
    ExtraCoins.EffectParameters.Quantity = CoinCheatQuantity;
    ExtraCoins.EffectParameters.bRepeatable = true;

    FSharCheatDefinition SceneTree = MakeCheatDefinition(
        FName(TEXT("developer_scene_tree")),
        {
            ESharCheatInputToken::Left,
            ESharCheatInputToken::Right,
            ESharCheatInputToken::Left,
            ESharCheatInputToken::Right,
        },
        ESharCheatPrerequisite::DeveloperBuild,
        ESharCheatActivationMode::EnableOnly,
        ESharCheatLifetime::Session
    );
    SceneTree.EffectKind = ESharCheatEffectKind::Diagnostic;
    SceneTree.EffectParameters.TargetId = FName(TEXT("scene_tree"));

    Definition->Cheats = {
        UnlockCards,
        ShowSpeedometer,
        ExtraCoins,
        SceneTree,
    };
    return Definition;
}

inline FSharCheatRuntimeContext MakeCheatRuntimeContext()
{
    FSharCheatRuntimeContext Context;
    Context.ContextRevision = TEXT("sha256:cheat_context_v1");
    Context.ProfileRevision = TEXT("sha256:profile_v1");
    Context.ApplicationModeRevision = TEXT("sha256:gameplay_mode_v1");
    Context.SessionRevision = TEXT("sha256:session_v1");
    Context.ChapterRevision = TEXT("sha256:chapter_v1");
    Context.MissionRevision = TEXT("sha256:mission_v1");
    Context.bProfileLoaded = true;
    Context.bStoryCompleted = true;
    Context.bDeveloperBuild = true;
    Context.bCheatsAvailable = true;
    return Context;
}

inline FSharCheatRuntimeFixture MakeCheatRuntime()
{
    FSharCheatRuntimeFixture Fixture;
    Fixture.GameInstance = NewObject<UGameInstance>();
    Fixture.CatalogSubsystem =
        NewObject<USharMetaCatalogSubsystem>(Fixture.GameInstance);
    Fixture.CatalogSubsystem->ConfigureRevision(TEXT("sha256:meta_catalog_v1"));
    Fixture.CatalogSubsystem->RegisterCatalog(MakeMetaCatalogDefinition());
    Fixture.CatalogSubsystem->Activate();
    Fixture.EffectSubsystem =
        NewObject<USharCheatEffectSubsystem>(Fixture.GameInstance);
    Fixture.EffectSubsystem->Configure(
        Fixture.CatalogSubsystem,
        MakeCheatRuntimeContext()
    );
    Fixture.CheatSubsystem =
        NewObject<USharCheatSubsystem>(Fixture.GameInstance);
    Fixture.CheatSubsystem->Configure(
        Fixture.CatalogSubsystem,
        Fixture.EffectSubsystem
    );
    return Fixture;
}

inline FSharCheatArmRequest MakeCheatArmRequest(
    const FName& RecognitionId,
    const FName& LocalPlayerId
)
{
    FSharCheatArmRequest Request;
    Request.RecognitionId = RecognitionId;
    Request.LocalPlayerId = LocalPlayerId;
    Request.ControllerId = FName(TEXT("controller_01"));
    Request.CatalogId = FName(TEXT("base_meta"));
    Request.CatalogRevision = TEXT("sha256:meta_catalog_v1");
    Request.ContextRevision = TEXT("sha256:cheat_context_v1");
    Request.InputProfileRevision = TEXT("sha256:input_profile_v1");
    Request.RecognitionRevision = TEXT("sha256:recognition_v1");
    Request.TimeoutOrdinal = CheatInputTimeoutOrdinal;
    return Request;
}


inline FSharCheatArmRequest MakeCheatArmRequest(
    const FName& RecognitionId
)
{
    return MakeCheatArmRequest(
        RecognitionId,
        FName(TEXT("local_player_01"))
    );
}

inline FSharCheatInputEvent MakeCheatInputEvent(
    const FSharCheatArmRequest& Request,
    const FName& DeliveryId,
    const ESharCheatInputToken Token,
    const int64 InputOrdinal,
    const ESharCheatInputTransition Transition
)
{
    FSharCheatInputEvent Event;
    Event.RecognitionId = Request.RecognitionId;
    Event.DeliveryId = DeliveryId;
    Event.LocalPlayerId = Request.LocalPlayerId;
    Event.ControllerId = Request.ControllerId;
    Event.CatalogRevision = Request.CatalogRevision;
    Event.ContextRevision = Request.ContextRevision;
    Event.InputProfileRevision = Request.InputProfileRevision;
    Event.Transition = Transition;
    Event.Token = Token;
    Event.InputOrdinal = InputOrdinal;
    return Event;
}

inline FSharCheatInputEvent MakeCheatInputEvent(
    const FSharCheatArmRequest& Request,
    const FName& DeliveryId,
    const ESharCheatInputToken Token,
    const int64 InputOrdinal
)
{
    return MakeCheatInputEvent(
        Request,
        DeliveryId,
        Token,
        InputOrdinal,
        ESharCheatInputTransition::TokenDown
    );
}

inline FSharCheatActivationRequest MakeCheatActivationRequest(
    const FName& ActivationId,
    const FName& CheatId,
    const ESharCheatEffectPriority Priority,
    const ESharCheatEffectAction Action,
    const FName& LocalPlayerId
)
{
    FSharCheatActivationRequest Request;
    Request.ActivationId = ActivationId;
    Request.RecognitionId = FName(TEXT("recognition_direct"));
    Request.CheatId = CheatId;
    Request.LocalPlayerId = LocalPlayerId;
    Request.CatalogId = FName(TEXT("base_meta"));
    Request.Priority = Priority;
    Request.Action = Action;
    Request.CatalogRevision = TEXT("sha256:meta_catalog_v1");
    Request.ContextRevision = TEXT("sha256:cheat_context_v1");
    Request.ActivationRevision = TEXT("sha256:activation_v1");
    return Request;
}


inline FSharCheatActivationRequest MakeCheatActivationRequest(
    const FName& ActivationId,
    const FName& CheatId,
    const ESharCheatEffectPriority Priority,
    const ESharCheatEffectAction Action
)
{
    return MakeCheatActivationRequest(
        ActivationId,
        CheatId,
        Priority,
        Action,
        FName(TEXT("local_player_01"))
    );
}

inline FSharCheatPostconditionEvidence MakeCheatPostconditionEvidence(
    const FSharCheatActivationRequest& Request
)
{
    FSharCheatPostconditionEvidence Evidence;
    Evidence.ActivationId = Request.ActivationId;
    Evidence.CheatId = Request.CheatId;
    Evidence.CatalogRevision = Request.CatalogRevision;
    Evidence.ContextRevision = Request.ContextRevision;
    Evidence.ActivationRevision = Request.ActivationRevision;
    Evidence.EffectOwnerRevision = TEXT("sha256:effect_owner_v1");
    return Evidence;
}

inline FSharCheatActivationResolution MakeCheatResolution(
    const FSharCheatActivationRequest& Request,
    const ESharCheatResolutionCommand Command
)
{
    FSharCheatActivationResolution Resolution;
    Resolution.ActivationId = Request.ActivationId;
    Resolution.Command = Command;
    Resolution.ContextRevision = Request.ContextRevision;
    Resolution.ActivationRevision = Request.ActivationRevision;
    return Resolution;
}
