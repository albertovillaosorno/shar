// File:
//   - SharContent.Build.cs
// Path:
//   - src/uproject/Source/SharContent/SharContent.Build.cs
//
// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE
// Path-Rule:
//   - All paths in this header are repository-root relative.
//
// Boundary-Contract:
// - Owns:
//   - UnrealBuildTool dependencies for shared native content definitions.
// - Must-Not:
//   - Depend on gameplay-family modules or editor-only implementation.
// - Allows:
//   - Core asset identity, tags, validation, and Asset Manager contracts.
// - Split-When:
//   - A dependency belongs only to one independently buildable content family.
// - Merge-When:
//   - Another module owns the same content-definition dependency boundary.
// - Summary:
//   - Declares the shared SHAR content module dependencies.
// - Description:
//   - Keeps reusable Primary Asset contracts independent of gameplay domains.
// - Usage:
//   - Loaded by content-family modules that publish native definitions.
// - Defaults:
//   - Depends only on Core, CoreUObject, Engine, and GameplayTags.
//
// ADRs:
// - docs/adr/unreal/architecture/aaa-native-content-and-gameplay-foundation.md
//
// Large file:
//   - false
//

using UnrealBuildTool;

public class SharContent : ModuleRules
{
    public SharContent(ReadOnlyTargetRules target) : base(target)
    {
        PCHUsage = PCHUsageMode.UseExplicitOrSharedPCHs;
        PublicDependencyModuleNames.AddRange(
            new[]
            {
                "Core",
                "CoreUObject",
                "Engine",
                "GameplayTags",
            }
        );
    }
}
