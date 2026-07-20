// File: SharNetworkingModule.cpp
// Path: src/uproject/Source/SharNetworking/Private/SharNetworkingModule.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: module registration only; no transport, discovery, or session process.
// ADR: docs/adr/modding/mod-owned-multiplayer-adapters-and-community-servers.md

#include "Modules/ModuleManager.h"

IMPLEMENT_MODULE(FDefaultModuleImpl, SharNetworking);
