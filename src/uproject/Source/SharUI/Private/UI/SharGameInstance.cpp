// File: SharGameInstance.cpp
// Path: src/uproject/Source/SharUI/Private/UI/SharGameInstance.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: startup transition semantics and Blueprint notifications only; no concrete widgets or maps.
// ADR: docs/adr/unreal/architecture/aaa-native-content-and-gameplay-foundation.md

#include "UI/SharGameInstance.h"

#include "Engine/DataAsset.h"
#include "Engine/GameInstance.h"

void USharGameInstance::ApplyState(const ESharStartupState NewState)
{
    StartupState = NewState;
    StartupError = FText();
    OnStartupStateChanged(NewState);
}

bool USharGameInstance::CanTransitionTo(
    const ESharStartupState NewState
) const
{
    switch (StartupState)
    {
        case ESharStartupState::Cold:
            return NewState == ESharStartupState::Booting;
        case ESharStartupState::Booting:
            return NewState == ESharStartupState::Title;
        case ESharStartupState::Title:
            return NewState == ESharStartupState::MainMenu;
        case ESharStartupState::MainMenu:
            return NewState == ESharStartupState::LoadingExperience;
        case ESharStartupState::LoadingExperience:
            return NewState == ESharStartupState::Gameplay;
        case ESharStartupState::Gameplay:
            return NewState == ESharStartupState::MainMenu;
        case ESharStartupState::ShuttingDown:
        default:
            return false;
    }
}

bool USharGameInstance::TransitionTo(
    const ESharStartupState NewState,
    const TCHAR* InvalidMessage
)
{
    if (!CanTransitionTo(NewState))
    {
        StartupError = FText::FromString(InvalidMessage);
        return false;
    }
    ApplyState(NewState);
    return true;
}

void USharGameInstance::Init()
{
    UGameInstance::Init();
    StartBootFlow();
}

void USharGameInstance::Shutdown()
{
    ApplyState(ESharStartupState::ShuttingDown);
    UGameInstance::Shutdown();
}

bool USharGameInstance::StartBootFlow()
{
    PendingExperienceId = FPrimaryAssetId();
    return TransitionTo(
        ESharStartupState::Booting,
        TEXT("Boot flow can only start from the cold state.")
    );
}

bool USharGameInstance::CompleteBoot()
{
    return TransitionTo(
        ESharStartupState::Title,
        TEXT("Boot completion requires the booting state.")
    );
}

bool USharGameInstance::OpenMainMenu()
{
    return TransitionTo(
        ESharStartupState::MainMenu,
        TEXT("Main menu can only open after the title state.")
    );
}

bool USharGameInstance::BeginExperienceLoad(
    const FPrimaryAssetId& ExperienceId
)
{
    if (!ExperienceId.IsValid())
    {
        StartupError = FText::FromString(
            TEXT("Experience load requires a valid Primary Asset identity.")
        );
        return false;
    }
    PendingExperienceId = ExperienceId;
    return TransitionTo(
        ESharStartupState::LoadingExperience,
        TEXT("Experience loading can only begin from the main menu.")
    );
}

bool USharGameInstance::EnterGameplay()
{
    return TransitionTo(
        ESharStartupState::Gameplay,
        TEXT("Gameplay entry requires a completed experience load.")
    );
}

bool USharGameInstance::ReturnToMenu()
{
    if (!CanTransitionTo(ESharStartupState::MainMenu))
    {
        StartupError = FText::FromString(
            TEXT("Only gameplay can return directly to the main menu.")
        );
        return false;
    }
    PendingExperienceId = FPrimaryAssetId();
    ApplyState(ESharStartupState::MainMenu);
    return true;
}

ESharStartupState USharGameInstance::GetStartupState() const
{
    return StartupState;
}

FPrimaryAssetId USharGameInstance::GetPendingExperienceId() const
{
    return PendingExperienceId;
}

FText USharGameInstance::GetStartupError() const
{
    return StartupError;
}
