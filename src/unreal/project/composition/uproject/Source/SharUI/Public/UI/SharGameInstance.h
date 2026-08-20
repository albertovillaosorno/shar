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
//   - Shar game instance composition module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Shar game instance composition module.
// - Description:
//   - Implements the declared composition module responsibility for project.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Shar game instance composition module.

#pragma once

#include "CoreMinimal.h"
#include "Engine/DataAsset.h"
#include "Engine/GameInstance.h"

#include "SharGameInstance.generated.h"

UENUM(BlueprintType)
enum class ESharStartupState : uint8
{
    Cold,
    Booting,
    Title,
    MainMenu,
    LoadingExperience,
    Gameplay,
    ShuttingDown,
};

UCLASS(Blueprintable, BlueprintType)
class SHARUI_API USharGameInstance final : public UGameInstance
{
    GENERATED_BODY()

public:
    void Init() override;
    void Shutdown() override;

    UFUNCTION(BlueprintCallable, Category = "SHAR|Startup")
    bool StartBootFlow();

    UFUNCTION(BlueprintCallable, Category = "SHAR|Startup")
    bool CompleteBoot();

    UFUNCTION(BlueprintCallable, Category = "SHAR|Startup")
    bool OpenMainMenu();

    UFUNCTION(BlueprintCallable, Category = "SHAR|Startup")
    bool BeginExperienceLoad(const FPrimaryAssetId& ExperienceId);

    UFUNCTION(BlueprintCallable, Category = "SHAR|Startup")
    bool EnterGameplay();

    UFUNCTION(BlueprintCallable, Category = "SHAR|Startup")
    bool ReturnToMenu();

    UFUNCTION(BlueprintPure, Category = "SHAR|Startup")
    [[nodiscard]] ESharStartupState GetStartupState() const;

    UFUNCTION(BlueprintPure, Category = "SHAR|Startup")
    [[nodiscard]] FPrimaryAssetId GetPendingExperienceId() const;

    UFUNCTION(BlueprintPure, Category = "SHAR|Startup")
    [[nodiscard]] FText GetStartupError() const;

    UFUNCTION(BlueprintImplementableEvent, Category = "SHAR|Startup")
    void OnStartupStateChanged(ESharStartupState NewState);

private:
    UPROPERTY(Transient)
    ESharStartupState StartupState = ESharStartupState::Cold;

    UPROPERTY(Transient)
    FPrimaryAssetId PendingExperienceId;

    UPROPERTY(Transient)
    FText StartupError;

    [[nodiscard]] bool CanTransitionTo(
        ESharStartupState NewState
    ) const;
    bool TransitionTo(
        ESharStartupState NewState,
        const TCHAR* InvalidMessage
    );
    void ApplyState(ESharStartupState NewState);
};
