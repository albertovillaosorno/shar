// File: SharWorld.Build.cs
// jig-ignore-next-line: exact syntax is indivisible
// Path: src/unreal/project/composition/uproject/Source/SharWorld/SharWorld.Build.cs
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// jig-ignore-next-line: exact syntax is indivisible
// Boundary: world definition and clock dependencies only; no mission, vehicle, UI, or editor implementation dependencies.
// jig-ignore-next-line: exact syntax is indivisible
// ADR: docs/adr/unreal/architecture/aaa-native-content-and-gameplay-foundation.md

using UnrealBuildTool;

public class SharWorld : ModuleRules
{
    public SharWorld(ReadOnlyTargetRules target) : base(target)
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
