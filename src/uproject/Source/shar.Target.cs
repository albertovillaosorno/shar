// File:
//   - shar.Target.cs
// Path:
//   - src/uproject/Source/shar.Target.cs
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
//   - UnrealBuildTool rules for the standalone SHAR game target.
// - Must-Not:
//   - Configure editor-only behavior, private engine code, or unrelated
//   - modules.
// - Allows:
//   - Game target type, build settings, include order, compiler strictness,
//   - and SHAR module registration.
// - Split-When:
//   - A separately built game target requires independent rules or platform
//   - policy.
// - Merge-When:
//   - Another target rules file configures the same game target with no
//   - distinct invariant.
// - Summary:
//   - Configures the standalone SHAR game target.
// - Description:
//   - Selects Unreal 5.8 build defaults, enforces strict compilation, and
//   - adds the authored shar runtime module.
// - Usage:
//   - Loaded by UnrealBuildTool when generating or building the SHAR game
//   - target.
// - Defaults:
//   - Builds a non-unity game target with warning failures, no globally forced
//   - C++ exceptions, BuildSettingsVersion.V7, and Unreal 5.8 include order.
//
// ADRs:
// - docs/adr/unreal/project/cpp-primary-blueprint-compatible-project.md
//
// Large file:
//   - false
//

// Defines only the standalone game-target policy consumed by UnrealBuildTool
// and leaves runtime behavior inside the authored shar module.

using UnrealBuildTool;

public class sharTarget : TargetRules
{
    public sharTarget(TargetInfo target) : base(target)
    {
        Type = TargetType.Game;
        DefaultBuildSettings = BuildSettingsVersion.V7;
        IncludeOrderVersion = EngineIncludeOrderVersion.Unreal5_8;
        bWarningsAsErrors = true;
        bUseUnityBuild = false;
        bForceEnableExceptions = false;
        ExtraModuleNames.Add("shar");
    }
}
