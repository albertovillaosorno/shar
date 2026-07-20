// File: SharInteractionModule.cpp
// Path: src/uproject/Source/SharInteraction/Private/SharInteractionModule.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: module registration only; no process-wide interaction authority.
// ADR: docs/adr/unreal/runtime/contextual-interaction-query-and-transaction.md

#include "Modules/ModuleManager.h"

IMPLEMENT_MODULE(FDefaultModuleImpl, SharInteraction);
