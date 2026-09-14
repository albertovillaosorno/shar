// Copyright:
//   - Copyright © 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT
//
// Boundary-Contract:
// - Owns:
//   - Shar input runtime module registration.
// - Must-Not:
//   - Own application-mode, gameplay-domain, or device-platform policy.
// - Allows:
//   - Registering the per-local-player input lease runtime.
// - Split-When:
//   - Another independently loadable input adapter is required.
// - Merge-When:
//   - Another module owns the identical input-runtime boundary.
// - Summary:
//   - Registers the SharInput runtime module.
// - Description:
//   - Uses the default Unreal module lifecycle.
// - Usage:
//   - Loaded by UnrealBuildTool as a runtime module.
// - Defaults:
//   - No process-global input state.
//

//! Shar input runtime module registration.

#include "Modules/ModuleManager.h"

IMPLEMENT_MODULE(FDefaultModuleImpl, SharInput);
