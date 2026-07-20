// File: SharNetworking.Build.cs
// Path: src/uproject/Source/SharNetworking/SharNetworking.Build.cs
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: deferred multiplayer adapter declarations and compatibility schemas only.
// ADR: docs/adr/modding/mod-owned-multiplayer-adapters-and-community-servers.md

using UnrealBuildTool;

public class SharNetworking : ModuleRules
{
    public SharNetworking(ReadOnlyTargetRules target) : base(target)
    {
        PCHUsage = PCHUsageMode.UseExplicitOrSharedPCHs;
        PublicDependencyModuleNames.AddRange(
            new[]
            {
                "Core",
                "CoreUObject",
                "Engine",
                "SharContent",
            }
        );
    }
}
