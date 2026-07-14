// File:
//   - shar.cpp
// Path:
//   - src/uproject/Source/shar/shar.cpp
//
// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE
// Path-Rule:
//   - All paths in this header are repository-root relative.
//
// Boundary-Contract:
// - Owns:
//   - Primary SHAR game-module registration through the public Unreal module
//   - API.
// - Must-Not:
//   - Implement gameplay, asset import, editor automation, or engine internals.
// - Allows:
//   - One primary module macro binding the authored SHAR runtime module to
//   - startup.
// - Split-When:
//   - Registration gains platform-specific or target-specific entry points.
// - Merge-When:
//   - Another source file owns only the same primary module registration with
//   - no distinct invariant.
// - Summary:
//   - Registers the SHAR runtime module entry point.
// - Description:
//   - Connects FDefaultGameModuleImpl to the shar module through Unreal's
//   - public module API.
// - Usage:
//   - Compiled into SHAR game and editor targets to publish module startup
//   - identity.
// - Defaults:
//   - Registers exactly one primary game module and creates no runtime state.
//
// ADRs:
// - docs/adr/unreal/project/cpp-primary-blueprint-compatible-project.md
//
// Large file:
//   - false
//

// Registers the single authored SHAR runtime module with Unreal Engine while
// keeping all gameplay and editor behavior behind later module boundaries.

#include "Modules/ModuleManager.h"

IMPLEMENT_PRIMARY_GAME_MODULE(FDefaultGameModuleImpl, shar, "shar");
