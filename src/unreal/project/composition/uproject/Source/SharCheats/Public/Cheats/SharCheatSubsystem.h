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
