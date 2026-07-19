// File: SharGameInstance.h
// Path: src/uproject/Source/SharUI/Public/UI/SharGameInstance.h
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: Blueprint-facing boot, title, menu, loading, gameplay, and shutdown routing; no concrete widget classes.
// ADR: docs/adr/unreal/architecture/aaa-native-content-and-gameplay-foundation.md

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
