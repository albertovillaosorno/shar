// File: SharMessagingModule.cpp
// Path: src/uproject/Source/SharMessaging/Private/SharMessagingModule.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: module registration only; no process-wide event singleton or command bus.
// Specification: docs/technical/unreal/typed-event-and-observation-routing-runtime.md

#include "Modules/ModuleManager.h"

IMPLEMENT_MODULE(FDefaultModuleImpl, SharMessaging);
