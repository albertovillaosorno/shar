// File:
//   - sharEditor.Target.cs
// Path:
//   - src/uproject/Source/sharEditor.Target.cs
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
//   - UnrealBuildTool rules for the SHAR editor target.
// - Must-Not:
//   - Change runtime gameplay policy, redistribute engine code, or add
//   - unrelated modules.
// - Allows:
//   - Editor target type, build settings, include order, compiler strictness,
//   - and SHAR module registration.
// - Split-When:
//   - A separately built editor target requires independent rules or platform
//   - policy.
// - Merge-When:
//   - Another target rules file configures the same editor target with no
//   - distinct invariant.
// - Summary:
//   - Configures the SHAR Unreal Editor target.
// - Description:
//   - Selects Unreal 5.8 editor build defaults, enforces strict compilation,
//   - and adds the authored shar runtime module.
// - Usage:
//   - Loaded by UnrealBuildTool when generating or building the SHAR editor
//   - target.
// - Defaults:
//   - Builds a non-unity editor target with warning failures, no globally
//   - forced C++ exceptions, BuildSettingsVersion.V7, and Unreal 5.8 include
//   - order.
//
// ADRs:
// - docs/adr/unreal/project/cpp-primary-blueprint-compatible-project.md
//
// Large file:
//   - false
//

// Defines only the editor-target policy consumed by UnrealBuildTool and keeps
// editor behavior inside explicit authored modules and adapters.

using UnrealBuildTool;

public class sharEditorTarget : TargetRules
{
    public sharEditorTarget(TargetInfo target) : base(target)
    {
        Type = TargetType.Editor;
        DefaultBuildSettings = BuildSettingsVersion.V7;
        IncludeOrderVersion = EngineIncludeOrderVersion.Unreal5_8;
        bWarningsAsErrors = true;
        bUseUnityBuild = false;
        bForceEnableExceptions = false;
        ExtraModuleNames.Add("shar");
    }
}
