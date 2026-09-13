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
//   - Implements the state-free SHAR game-instance shell.
// - Description:
//   - Intentionally contains no startup state machine.
// - Usage:
//   - Unreal constructs it through project game settings.
// - Defaults:
//   - Base UGameInstance behavior only.
//

//! State-free SHAR game-instance composition shell.

#include "Runtime/SharGameInstance.h"


void USharGameInstance::Init()
{
    Super::Init();
    RuntimeBootstrap = NewObject<USharRuntimeBootstrap>(this);
}

USharRuntimeBootstrap* USharGameInstance::GetRuntimeBootstrap() const
{
    return RuntimeBootstrap;
}
