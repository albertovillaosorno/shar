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
//   - Process-lifetime Unreal game-instance composition boundary.
// - Must-Not:
//   - Own application-mode state or duplicate domain coordinator state.
// - Allows:
//   - Native Unreal game-instance lifecycle and subsystem composition.
// - Split-When:
//   - One composed adapter gains an independent lifecycle.
// - Merge-When:
//   - Another project game instance owns the identical composition boundary.
// - Summary:
//   - Defines the state-free SHAR game-instance shell.
// - Description:
//   - Leaves application lifecycle authority to SharApplication coordinators.
// - Usage:
//   - Selected by project settings as the runtime game-instance class.
// - Defaults:
//   - Adds no parallel startup state machine.
//

//! State-free SHAR game-instance composition shell.

#pragma once

#include "CoreMinimal.h"
#include "Engine/GameInstance.h"

#include "Runtime/SharGameplayTransitionComposer.h"
#include "Runtime/SharRuntimeAssetLoader.h"
#include "Runtime/SharRuntimeBootstrap.h"

#include "SharGameInstance.generated.h"

UCLASS()
class SHAR_API USharGameInstance final : public UGameInstance
{
    GENERATED_BODY()

public:
    void Init() override;

    UFUNCTION(BlueprintPure, Category = "SHAR|Runtime")
    [[nodiscard]] USharRuntimeBootstrap* GetRuntimeBootstrap() const;

    UFUNCTION(BlueprintPure, Category = "SHAR|Runtime")
    [[nodiscard]] USharRuntimeAssetLoader* GetRuntimeAssetLoader() const;

    UFUNCTION(BlueprintPure, Category = "SHAR|Runtime")
    [[nodiscard]] USharGameplayTransitionComposer*
    GetGameplayTransitionComposer() const;

private:
    void StartRuntimeAssetLoading();
    void HandleRuntimeAssetLoadTerminal(
        ESharRuntimeAssetLoadStatus Status,
        ESharRuntimeAssetLoadFailure Failure
    );

    UPROPERTY(Transient)
    TObjectPtr<USharRuntimeBootstrap> RuntimeBootstrap;

    UPROPERTY(Transient)
    TObjectPtr<USharRuntimeAssetLoader> RuntimeAssetLoader;

    UPROPERTY(Transient)
    TObjectPtr<USharGameplayTransitionComposer> GameplayTransitionComposer;
};
