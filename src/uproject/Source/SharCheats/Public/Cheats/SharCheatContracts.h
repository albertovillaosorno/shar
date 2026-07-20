// File: SharCheatContracts.h
// Path: src/uproject/Source/SharCheats/Public/Cheats/SharCheatContracts.h
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: reflected cheat context, recognition, activation, evidence, lifetime, and immutable observation records only; no subsystem behavior or gameplay execution.
// Specification: docs/technical/unreal/progression-collectibles-and-cheats.md
// LARGE-FILE owner=SharCheats; reason=cohesive reflected cheat control-plane value contract;
// split=extract effect observations when additional effect-owner result families exist;
// validation=validate.sh SharCheats plus Unreal automation; review=2027-01.

#pragma once

#include "CoreMinimal.h"
#include "Meta/SharMetaCatalogDefinition.h"

#include "SharCheatContracts.generated.h"

UENUM(BlueprintType)
enum class ESharCheatRecognizerState : uint8
{
    Inactive,
    Armed,
    Collecting,
    Accepted,
    Rejected,
    Released,
};

UENUM(BlueprintType)
enum class ESharCheatRecognitionOutcome : uint8
{
    None,
    Matched,
    UnknownSequence,
    Unavailable,
    PrerequisiteFailed,
    InputCancelled,
};

UENUM(BlueprintType)
enum class ESharCheatInputTransition : uint8
{
    TokenDown,
    TokenUp,
    KeyRepeat,
    AnalogNoise,
};

UENUM(BlueprintType)
enum class ESharCheatEffectPriority : uint8
{
    Background,
    Gameplay,
    User,
    Recovery,
};

UENUM(BlueprintType)
enum class ESharCheatEffectAction : uint8
{
    Enable,
    Disable,
    Execute,
};

UENUM(BlueprintType)
enum class ESharCheatEffectState : uint8
{
    Queued,
    Dispatching,
    AwaitingPostcondition,
    Completed,
    Failed,
    Cancelled,
    Released,
};

UENUM(BlueprintType)
enum class ESharCheatTerminalResult : uint8
{
    None,
    Success,
    Failed,
    Cancelled,
};

UENUM(BlueprintType)
enum class ESharCheatResolutionCommand : uint8
{
    Fail,
    Cancel,
};

UENUM(BlueprintType)
enum class ESharCheatOperationResult : uint8
{
    Accepted,
    InvalidRequest,
    NotConfigured,
    CatalogMissing,
    CatalogInactive,
    DefinitionMissing,
    DuplicateRequest,
    ConflictingRequest,
    NotFound,
    NotHead,
    StaleRevision,
    WrongOwner,
    IgnoredInput,
    DuplicateInput,
    UnknownSequence,
    Unavailable,
    PrerequisiteFailed,
    InputCancelled,
    InvalidState,
    UnsupportedAction,
    AlreadyApplied,
    AlreadyTerminal,
    Released,
};

USTRUCT(BlueprintType)
struct SHARCHEATS_API FSharCheatRuntimeContext
{
    GENERATED_BODY()

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString ContextRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString ProfileRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString ApplicationModeRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString SessionRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString ChapterRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString MissionRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Availability")
    bool bProfileLoaded = false;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Availability")
    bool bStoryCompleted = false;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Availability")
    bool bDeveloperBuild = false;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Availability")
    bool bCheatsAvailable = false;
};

USTRUCT(BlueprintType)
struct SHARCHEATS_API FSharCheatContextUpdate
{
    GENERATED_BODY()

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString ExpectedContextRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Context")
    FSharCheatRuntimeContext UpdatedContext;
};

USTRUCT(BlueprintType)
struct SHARCHEATS_API FSharCheatArmRequest
{
    GENERATED_BODY()

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Identity")
    FName RecognitionId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Owner")
    FName LocalPlayerId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Owner")
    FName ControllerId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Catalog")
    FName CatalogId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString CatalogRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString ContextRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString InputProfileRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString RecognitionRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Timeout")
    int64 TimeoutOrdinal = 0;
};

USTRUCT(BlueprintType)
struct SHARCHEATS_API FSharCheatInputEvent
{
    GENERATED_BODY()

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Identity")
    FName RecognitionId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Identity")
    FName DeliveryId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Owner")
    FName LocalPlayerId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Owner")
    FName ControllerId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString CatalogRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString ContextRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString InputProfileRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Input")
    ESharCheatInputTransition Transition =
        ESharCheatInputTransition::TokenDown;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Input")
    ESharCheatInputToken Token = ESharCheatInputToken::Up;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Input")
    int64 InputOrdinal = 0;
};

USTRUCT(BlueprintType)
struct SHARCHEATS_API FSharCheatRecognizerSnapshot
{
    GENERATED_BODY()

    UPROPERTY(BlueprintReadOnly, Category = "Request")
    FSharCheatArmRequest Request;

    UPROPERTY(BlueprintReadOnly, Category = "State")
    ESharCheatRecognizerState State = ESharCheatRecognizerState::Inactive;

    UPROPERTY(BlueprintReadOnly, Category = "Result")
    ESharCheatRecognitionOutcome Outcome =
        ESharCheatRecognitionOutcome::None;

    UPROPERTY(BlueprintReadOnly, Category = "Input")
    TArray<ESharCheatInputToken> AcceptedTokens;

    UPROPERTY(BlueprintReadOnly, Category = "Input")
    TArray<FName> AcceptedDeliveryIds;

    UPROPERTY(BlueprintReadOnly, Category = "Match")
    FName MatchedCheatId;
};

USTRUCT(BlueprintType)
struct SHARCHEATS_API FSharCheatActivationRequest
{
    GENERATED_BODY()

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Identity")
    FName ActivationId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Identity")
    FName RecognitionId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Identity")
    FName CheatId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Owner")
    FName LocalPlayerId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Catalog")
    FName CatalogId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Priority")
    ESharCheatEffectPriority Priority = ESharCheatEffectPriority::User;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Action")
    ESharCheatEffectAction Action = ESharCheatEffectAction::Enable;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString CatalogRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString ContextRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString ActivationRevision;
};

USTRUCT(BlueprintType)
struct SHARCHEATS_API FSharCheatPostconditionEvidence
{
    GENERATED_BODY()

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Identity")
    FName ActivationId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Identity")
    FName CheatId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString CatalogRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString ContextRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString ActivationRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString EffectOwnerRevision;
};

USTRUCT(BlueprintType)
struct SHARCHEATS_API FSharCheatActivationResolution
{
    GENERATED_BODY()

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Identity")
    FName ActivationId;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Result")
    ESharCheatResolutionCommand Command = ESharCheatResolutionCommand::Fail;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString ContextRevision;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Revision")
    FString ActivationRevision;
};

USTRUCT(BlueprintType)
struct SHARCHEATS_API FSharCheatActivationSnapshot
{
    GENERATED_BODY()

    UPROPERTY(BlueprintReadOnly, Category = "Request")
    FSharCheatActivationRequest Request;

    UPROPERTY(BlueprintReadOnly, Category = "State")
    ESharCheatEffectState State = ESharCheatEffectState::Queued;

    UPROPERTY(BlueprintReadOnly, Category = "Result")
    ESharCheatTerminalResult TerminalResult = ESharCheatTerminalResult::None;

    UPROPERTY(BlueprintReadOnly, Category = "Ordering")
    int32 InsertionSequence = 0;
};

USTRUCT(BlueprintType)
struct SHARCHEATS_API FSharEnabledCheatEffect
{
    GENERATED_BODY()

    UPROPERTY(BlueprintReadOnly, Category = "Identity")
    FName CheatId;

    UPROPERTY(BlueprintReadOnly, Category = "Owner")
    FName LocalPlayerId;

    UPROPERTY(BlueprintReadOnly, Category = "Lifetime")
    ESharCheatLifetime Lifetime = ESharCheatLifetime::Session;

    UPROPERTY(BlueprintReadOnly, Category = "Revision")
    FString ActivationRevision;

    UPROPERTY(BlueprintReadOnly, Category = "Revision")
    FString SessionRevision;

    UPROPERTY(BlueprintReadOnly, Category = "Revision")
    FString ChapterRevision;

    UPROPERTY(BlueprintReadOnly, Category = "Revision")
    FString MissionRevision;
};

USTRUCT(BlueprintType)
struct SHARCHEATS_API FSharCheatRuntimeObservation
{
    GENERATED_BODY()

    UPROPERTY(BlueprintReadOnly, Category = "Context")
    FSharCheatRuntimeContext Context;

    UPROPERTY(BlueprintReadOnly, Category = "Effect")
    TArray<FSharEnabledCheatEffect> EnabledEffects;

    UPROPERTY(BlueprintReadOnly, Category = "Effect")
    int32 UnreleasedActivationCount = 0;

    UPROPERTY(BlueprintReadOnly, Category = "Recognition")
    int32 UnreleasedRecognizerCount = 0;
};
