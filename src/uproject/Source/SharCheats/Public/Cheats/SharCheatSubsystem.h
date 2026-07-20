// File: SharCheatSubsystem.h
// Path: src/uproject/Source/SharCheats/Public/Cheats/SharCheatSubsystem.h
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: local-player semantic cheat arming, four-token recognition, prerequisite checks, context cancellation, and typed effect request publication only; physical input mapping, gameplay execution, UI, and persistence remain external.
// Specification: docs/technical/unreal/progression-collectibles-and-cheats.md
// LARGE-FILE owner=SharCheats; reason=cohesive recognizer control-plane API;
// split=extract recognizer projections when diagnostic history is introduced;
// validation=validate.sh SharCheats plus Unreal automation; review=2027-01.

#pragma once

#include "Cheats/SharCheatContracts.h"
#include "Cheats/SharCheatEffectSubsystem.h"
#include "CoreMinimal.h"
#include "Meta/SharMetaCatalogSubsystem.h"
#include "Subsystems/GameInstanceSubsystem.h"

#include "SharCheatSubsystem.generated.h"

UCLASS()
class SHARCHEATS_API USharCheatSubsystem final : public UGameInstanceSubsystem
{
    GENERATED_BODY()

public:
    UFUNCTION(BlueprintCallable, Category = "SHAR|Cheats")
    bool Configure(
        USharMetaCatalogSubsystem* InCatalogSubsystem,
        USharCheatEffectSubsystem* InEffectSubsystem
    );

    UFUNCTION(BlueprintCallable, Category = "SHAR|Cheats")
    ESharCheatOperationResult Arm(const FSharCheatArmRequest& Request);

    UFUNCTION(BlueprintCallable, Category = "SHAR|Cheats")
    ESharCheatOperationResult AcceptInput(
        const FSharCheatInputEvent& InputEvent
    );

    UFUNCTION(BlueprintCallable, Category = "SHAR|Cheats")
    ESharCheatOperationResult UpdateContext(
        const FSharCheatContextUpdate& Update
    );

    ESharCheatOperationResult Cancel(
        const FName& RecognitionId,
        const FString& RecognitionRevision
    );

    ESharCheatOperationResult Release(const FName& RecognitionId);

    UFUNCTION(BlueprintPure, Category = "SHAR|Cheats")
    [[nodiscard]] ESharCheatRecognizerState GetRecognizerState(
        const FName& RecognitionId
    ) const;

    UFUNCTION(BlueprintPure, Category = "SHAR|Cheats")
    [[nodiscard]] ESharCheatRecognitionOutcome GetRecognitionOutcome(
        const FName& RecognitionId
    ) const;

    UFUNCTION(BlueprintPure, Category = "SHAR|Cheats")
    [[nodiscard]] FSharCheatRuntimeObservation GetObservation() const;

private:
    UPROPERTY(Transient)
    USharMetaCatalogSubsystem* CatalogSubsystem = nullptr;

    UPROPERTY(Transient)
    USharCheatEffectSubsystem* EffectSubsystem = nullptr;

    UPROPERTY(Transient)
    TArray<FSharCheatRecognizerSnapshot> Recognizers;

    UPROPERTY(Transient)
    bool bConfigured = false;

    [[nodiscard]] static bool IsCanonicalIdentity(const FName& Candidate);
    [[nodiscard]] static bool IsRevisionToken(const FString& Revision);
    [[nodiscard]] static bool IsTerminalState(ESharCheatRecognizerState State);
    [[nodiscard]] ESharCheatOperationResult ValidateArmRequest(
        const FSharCheatArmRequest& Request
    ) const;
    [[nodiscard]] static bool IsArmRequestWellFormed(
        const FSharCheatArmRequest& Request
    );
    [[nodiscard]] bool HasActiveRecognizerForPlayer(
        const FName& LocalPlayerId
    ) const;
    [[nodiscard]] static bool InputMatchesRecognizer(
        const FSharCheatRecognizerSnapshot& Recognizer,
        const FSharCheatInputEvent& InputEvent
    );
    [[nodiscard]] static bool PrerequisiteSatisfied(
        ESharCheatPrerequisite Prerequisite,
        const FSharCheatRuntimeContext& Context
    );
    [[nodiscard]] static ESharCheatEffectAction ResolveAction(
        const FSharCheatDefinition& Definition,
        const USharCheatEffectSubsystem& EffectSubsystem,
        const FName& LocalPlayerId
    );
    [[nodiscard]] ESharCheatOperationResult CompleteSequence(
        FSharCheatRecognizerSnapshot& Recognizer
    );
    [[nodiscard]] static ESharCheatOperationResult PublishOutcome(
        FSharCheatRecognizerSnapshot& Recognizer,
        ESharCheatRecognizerState State,
        ESharCheatRecognitionOutcome Outcome
    );
    [[nodiscard]] FSharCheatRecognizerSnapshot* FindRecognizer(
        const FName& RecognitionId
    );
    [[nodiscard]] const FSharCheatRecognizerSnapshot* FindRecognizer(
        const FName& RecognitionId
    ) const;
    void CancelActiveRecognizers();
    [[nodiscard]] int32 CountUnreleasedRecognizers() const;
};
