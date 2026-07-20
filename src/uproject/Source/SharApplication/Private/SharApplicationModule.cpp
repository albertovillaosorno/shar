// File: SharApplicationModule.cpp
// Path: src/uproject/Source/SharApplication/Private/SharApplicationModule.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: module registration only; no global context stack or subsystem initialization ordering.
// Specification: docs/technical/unreal/application-lifecycle-and-mode-runtime.md

#include "Modules/ModuleManager.h"

IMPLEMENT_MODULE(FDefaultModuleImpl, SharApplication);
