// File: SharMessaging.Build.cs
// jig-ignore-next-line: exact syntax is indivisible
// Path: src/unreal/project/composition/uproject/Source/SharMessaging/SharMessaging.Build.cs
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// jig-ignore-next-line: exact syntax is indivisible
// Boundary: typed schema metadata, subscription ownership, and world-scoped fact routing only.
// jig-ignore-next-line: exact syntax is indivisible
// Specification: docs/technical/unreal/typed-event-and-observation-routing-runtime.md

using UnrealBuildTool;

public class SharMessaging : ModuleRules
{
    public SharMessaging(ReadOnlyTargetRules target) : base(target)
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
