// File: SharModding.Build.cs
// Path: src/uproject/Source/SharModding/SharModding.Build.cs
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: mod descriptor and activation-planning dependencies only; no concrete gameplay modules.
// ADR: docs/adr/unreal/architecture/aaa-native-content-and-gameplay-foundation.md

using UnrealBuildTool;

public class SharModding : ModuleRules
{
    public SharModding(ReadOnlyTargetRules target) : base(target)
    {
        PCHUsage = PCHUsageMode.UseExplicitOrSharedPCHs;
        PublicDependencyModuleNames.AddRange(
            new[]
            {
                "Core",
                "CoreUObject",
                "Engine",
                "GameplayTags",
                "SharContent",
            }
        );
    }
}
