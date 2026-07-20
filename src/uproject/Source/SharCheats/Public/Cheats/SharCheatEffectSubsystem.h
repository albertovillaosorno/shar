// File: SharCheatEffectSubsystem.h
// Path: src/uproject/Source/SharCheats/Public/Cheats/SharCheatEffectSubsystem.h
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: deterministic cheat-effect request arbitration, correlated postcondition evidence, enabled-state lifetime, and immutable observation only; gameplay execution and persistence remain external.
// Specification: docs/technical/unreal/progression-collectibles-and-cheats.md
// LARGE-FILE owner=SharCheats; reason=cohesive effect control-plane API and lifetime authority;
// split=extract enabled-effect projection when query families expand;
// validation=validate.sh SharCheats plus Unreal automation; review=2027-01.

#pragma once

#include "Cheats/SharCheatContracts.h"
#include "CoreMinimal.h"
#include "Meta/SharMetaCatalogSubsystem.h"
#include "Subsystems/GameInstanceSubsystem.h"

#include "SharCheatEffectSubsystem.generated.h"

UCLASS()
class SHARCHEATS_API USharCheatEffectSubsystem final
    : public UGameInstanceSubsystem
{
    GENERATED_BODY()

public:
    UFUNCTION(BlueprintCallable, Category = "SHAR|Cheats")
    bool Configure(
        USharMetaCatalogSubsystem* InCatalogSubsystem,
        const FSharCheatRuntimeContext& InitialContext
    );

    UFUNCTION(BlueprintCallable, Category = "SHAR|Cheats")
    ESharCheatOperationResult UpdateContext(
        const FSharCheatContextUpdate& Update
    );

    UFUNCTION(BlueprintCallable, Category = "SHAR|Cheats")
    ESharCheatOperationResult Submit(
        const FSharCheatActivationRequest& Request
    );

    ESharCheatOperationResult Begin(const FName& ActivationId);
    ESharCheatOperationResult MarkDispatched(const FName& ActivationId);

    ESharCheatOperationResult AcceptPostconditionEvidence(
        const FSharCheatPostconditionEvidence& Evidence
    );

    ESharCheatOperationResult Resolve(
        const FSharCheatActivationResolution& Resolution
    );

    ESharCheatOperationResult Release(const FName& ActivationId);

    UFUNCTION(BlueprintPure, Category = "SHAR|Cheats")
    [[nodiscard]] int32 GetQueuePosition(const FName& ActivationId) const;

    UFUNCTION(BlueprintPure, Category = "SHAR|Cheats")
    [[nodiscard]] ESharCheatEffectState GetActivationState(
        const FName& ActivationId
    ) const;

    UFUNCTION(BlueprintPure, Category = "SHAR|Cheats")
    [[nodiscard]] ESharCheatTerminalResult GetTerminalResult(
        const FName& ActivationId
    ) const;

    UFUNCTION(BlueprintPure, Category = "SHAR|Cheats")
    [[nodiscard]] bool IsEnabled(
        const FName& LocalPlayerId,
        const FName& CheatId
    ) const;

    UFUNCTION(BlueprintPure, Category = "SHAR|Cheats")
    [[nodiscard]] FSharCheatRuntimeContext GetContext() const;

    UFUNCTION(BlueprintPure, Category = "SHAR|Cheats")
    [[nodiscard]] FSharCheatRuntimeObservation GetObservation() const;

private:
    UPROPERTY(Transient)
    USharMetaCatalogSubsystem* CatalogSubsystem = nullptr;

    UPROPERTY(Transient)
    FSharCheatRuntimeContext Context;

    UPROPERTY(Transient)
    TArray<FSharCheatActivationSnapshot> Activations;

    UPROPERTY(Transient)
    TArray<FSharEnabledCheatEffect> EnabledEffects;

    UPROPERTY(Transient)
    int32 NextInsertionSequence = 0;

    UPROPERTY(Transient)
    bool bConfigured = false;

    [[nodiscard]] static bool IsCanonicalIdentity(const FName& Candidate);
    [[nodiscard]] static bool IsRevisionToken(const FString& Revision);
    [[nodiscard]] static bool IsTerminalState(ESharCheatEffectState State);
    [[nodiscard]] static bool Outranks(
        const FSharCheatActivationSnapshot& Left,
        const FSharCheatActivationSnapshot& Right
    );
    [[nodiscard]] static bool IsContextValid(
        const FSharCheatRuntimeContext& Candidate
    );
    [[nodiscard]] bool RequestMatchesAuthority(
        const FSharCheatActivationRequest& Request
    ) const;
    [[nodiscard]] ESharCheatOperationResult ValidateRequest(
        const FSharCheatActivationRequest& Request
    ) const;
    [[nodiscard]] static bool IsRequestWellFormed(
        const FSharCheatActivationRequest& Request
    );
    [[nodiscard]] ESharCheatOperationResult ValidateDefinitionState(
        const FSharCheatActivationRequest& Request
    ) const;
    [[nodiscard]] static bool IsActionSupported(
        const FSharCheatDefinition& Definition,
        ESharCheatEffectAction Action
    );
    [[nodiscard]] bool HasConflictingActivation(
        const FSharCheatActivationRequest& Request
    ) const;
    [[nodiscard]] FSharCheatActivationSnapshot* FindActivation(
        const FName& ActivationId
    );
    [[nodiscard]] const FSharCheatActivationSnapshot* FindActivation(
        const FName& ActivationId
    ) const;
    [[nodiscard]] bool IsHead(
        const FSharCheatActivationSnapshot& Activation
    ) const;
    [[nodiscard]] static bool EvidenceMatches(
        const FSharCheatActivationSnapshot& Activation,
        const FSharCheatPostconditionEvidence& Evidence
    );
    [[nodiscard]] static ESharCheatOperationResult PublishTerminal(
        FSharCheatActivationSnapshot& Activation,
        ESharCheatEffectState State,
        ESharCheatTerminalResult Result
    );
    void ApplySuccessfulPostcondition(
        const FSharCheatActivationSnapshot& Activation,
        const FSharCheatDefinition& Definition
    );
    void CancelStaleActivations();
    void ExpireEffects(const FSharCheatRuntimeContext& PreviousContext);
    [[nodiscard]] int32 CountUnreleasedActivations() const;
};
