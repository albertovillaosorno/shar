// File: SharVehicles.Build.cs
// jig-ignore-next-line: exact syntax is indivisible
// Path: src/unreal/project/composition/uproject/Source/SharVehicles/SharVehicles.Build.cs
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: vehicle definition and runtime-state dependencies only.
// jig-ignore-next-line: exact syntax is indivisible
// ADR: docs/adr/unreal/architecture/aaa-native-content-and-gameplay-foundation.md

using UnrealBuildTool;

public class SharVehicles : ModuleRules
{
    public SharVehicles(ReadOnlyTargetRules target) : base(target)
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
