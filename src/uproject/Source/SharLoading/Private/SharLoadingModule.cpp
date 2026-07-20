// File: SharLoadingModule.cpp
// Path: src/uproject/Source/SharLoading/Private/SharLoadingModule.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: module registration only; no process-wide loading singleton or synchronous source loader.
// Specification: docs/technical/unreal/native-asset-load-request-and-streaming-runtime.md

#include "Modules/ModuleManager.h"

IMPLEMENT_MODULE(FDefaultModuleImpl, SharLoading);
