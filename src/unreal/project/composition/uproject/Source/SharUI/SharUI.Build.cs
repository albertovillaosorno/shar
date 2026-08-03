// File: SharUI.Build.cs
// Path: src/unreal/project/composition/uproject/Source/SharUI/SharUI.Build.cs
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// jig-ignore-next-line: exact syntax is indivisible
// Boundary: frontend catalog and flow control only; widgets, saves, settings, loading, application transitions, and gameplay execution remain external.
// jig-ignore-next-line: exact syntax is indivisible
// Specification: docs/technical/unreal/frontend-screen-flow-and-settings-runtime.md

using UnrealBuildTool;

public class SharUI : ModuleRules
{
    public SharUI(ReadOnlyTargetRules target) : base(target)
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
