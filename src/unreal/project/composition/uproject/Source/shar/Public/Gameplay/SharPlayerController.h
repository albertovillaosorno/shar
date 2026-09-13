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
//   - Project identity for native local-player controller and possession.
// - Must-Not:
//   - Enable gameplay input outside application-mode input leases.
// - Allows:
//   - Native Player Controller lifecycle and future typed input adapters.
// - Split-When:
//   - Input mapping or avatar handoff gains an independent controller adapter.
// - Merge-When:
//   - Another project controller owns the identical possession boundary.
// - Summary:
//   - Defines the minimal SHAR Player Controller bootstrap.
// - Description:
//   - Adds no input mappings or gameplay state before their typed policies
//   - exist.
// - Usage:
//   - Selected by ASharGameMode for local players.
// - Defaults:
//   - Inherits native APlayerController behavior only.
//

//! Project-owned native Player Controller boundary.

#pragma once

#include "CoreMinimal.h"
#include "GameFramework/PlayerController.h"

#include "SharPlayerController.generated.h"

UCLASS()
class SHAR_API ASharPlayerController final : public APlayerController
{
    GENERATED_BODY()
};
