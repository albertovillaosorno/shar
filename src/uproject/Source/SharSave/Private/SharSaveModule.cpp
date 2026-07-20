// File: SharSaveModule.cpp
// Path: src/uproject/Source/SharSave/Private/SharSaveModule.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: module registration only; no global save manager or platform storage initialization order.
// Specification: docs/technical/unreal/platform-save-storage-and-lifecycle.md

#include "Modules/ModuleManager.h"

IMPLEMENT_MODULE(FDefaultModuleImpl, SharSave);
