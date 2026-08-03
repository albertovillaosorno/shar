// File: SharApplication.Build.cs
// jig-ignore-next-line: exact syntax is indivisible
// Path: src/unreal/project/composition/uproject/Source/SharApplication/SharApplication.Build.cs
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// jig-ignore-next-line: exact syntax is indivisible
// Boundary: application-mode definitions, catalog validation, and transition coordination only; domain services retain state authority.
// jig-ignore-next-line: exact syntax is indivisible
// Specification: docs/technical/unreal/application-lifecycle-and-mode-runtime.md

using UnrealBuildTool;

public class SharApplication : ModuleRules
{
    public SharApplication(ReadOnlyTargetRules target) : base(target)
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
