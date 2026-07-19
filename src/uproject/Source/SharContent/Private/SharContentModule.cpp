// File:
//   - SharContentModule.cpp
// Path:
//   - src/uproject/Source/SharContent/Private/SharContentModule.cpp
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
//   - Registration of the reusable SHAR content module.
// - Must-Not:
//   - Create runtime state, load assets, or perform editor mutation.
// - Allows:
//   - One default Unreal module registration.
// - Split-When:
//   - Module startup gains independently testable lifecycle behavior.
// - Merge-When:
//   - Another translation unit owns only this module registration.
// - Summary:
//   - Registers the SHAR content module.
// - Description:
//   - Exposes content-definition types without startup side effects.
// - Usage:
//   - Linked into targets that consume SHAR Primary Asset definitions.
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

IMPLEMENT_MODULE(FDefaultModuleImpl, SharContent)
