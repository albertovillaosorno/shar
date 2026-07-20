// File: SharWorldModule.cpp
// Path: src/uproject/Source/SharWorld/Private/SharWorldModule.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: module registration only; no world singleton, streaming request, or Data Layer mutation.
// ADR: docs/adr/unreal/architecture/aaa-native-content-and-gameplay-foundation.md

#include "Modules/ModuleManager.h"

IMPLEMENT_MODULE(FDefaultModuleImpl, SharWorld);
