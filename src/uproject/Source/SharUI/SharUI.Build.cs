// File: SharUI.Build.cs
// Path: src/uproject/Source/SharUI/SharUI.Build.cs
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: startup and UI-state dependencies only; no mission or gameplay implementation state.
// ADR: docs/adr/unreal/architecture/aaa-native-content-and-gameplay-foundation.md

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
