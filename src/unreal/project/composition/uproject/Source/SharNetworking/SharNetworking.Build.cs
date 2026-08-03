// File: SharNetworking.Build.cs
// jig-ignore-next-line: exact syntax is indivisible
// Path: src/unreal/project/composition/uproject/Source/SharNetworking/SharNetworking.Build.cs
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// jig-ignore-next-line: exact syntax is indivisible
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
