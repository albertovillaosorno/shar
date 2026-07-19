// File:
//   - SharCharactersModule.cpp
// Path:
//   - src/uproject/Source/SharCharacters/Private/SharCharactersModule.cpp
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
//   - Registration of the SHAR character-family module.
// - Must-Not:
//   - Spawn characters, load content, or mutate gameplay state at startup.
// - Allows:
//   - One default Unreal module registration.
// - Split-When:
//   - Character startup gains independently testable lifecycle behavior.
// - Merge-When:
//   - Another translation unit owns only this module registration.
// - Summary:
//   - Registers the SHAR character module.
// - Description:
//   - Exposes character definitions without implicit runtime side effects.
// - Usage:
//   - Linked into the game target through the primary bootstrap module.
// - Defaults:
//   - Uses Unreal's default module implementation.
//
// ADRs:
// - docs/adr/unreal/architecture/aaa-native-content-and-gameplay-foundation.md
//
// Large file:
//   - false
//

#include "Modules/ModuleManager.h"

IMPLEMENT_MODULE(FDefaultModuleImpl, SharCharacters)
