// File: SharUI.Build.cs
// Path: src/uproject/Source/SharUI/SharUI.Build.cs
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: frontend catalog and flow control only; widgets, saves, settings, loading, application transitions, and gameplay execution remain external.
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
