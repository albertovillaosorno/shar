// File: SharModdingModule.cpp
// Path: src/uproject/Source/SharModding/Private/SharModdingModule.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: module registration only; no package scanning, mounting, or activation side effects.
// ADR: docs/adr/unreal/architecture/aaa-native-content-and-gameplay-foundation.md

#include "Modules/ModuleManager.h"

IMPLEMENT_MODULE(FDefaultModuleImpl, SharModding);
