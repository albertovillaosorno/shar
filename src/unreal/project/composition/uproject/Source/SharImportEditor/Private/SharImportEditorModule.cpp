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
//   - Shar import editor module registration.
// - Must-Not:
//   - Own runtime gameplay policy or mutate content outside generated roots.
// - Allows:
//   - Editor-only inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one imported asset family gains an independent lifecycle.
// - Merge-When:
//   - Merge when another editor module owns the identical responsibility.
// - Summary:
//   - Shar import editor module registration.
// - Description:
//   - Registers and unregisters the project ToolsetRegistry toolset.
// - Usage:
//   - Used through the SharImportEditor module and its native toolset boundary.
// - Defaults:
//   - Invalid, ambiguous, or replacement requests fail explicitly.
//

//! Shar import editor module registration.

#include "Import/SharImportToolset.h"

#include "Modules/ModuleManager.h"
#include "ToolsetRegistry/UToolsetRegistry.h"

class FSharImportEditorModule final : public IModuleInterface
{
public:
    virtual void StartupModule() override
    {
        UToolsetRegistry::RegisterToolsetClass(
            USharImportToolset::StaticClass());
    }

    virtual void ShutdownModule() override
    {
        UToolsetRegistry::UnregisterToolsetClass(
            USharImportToolset::StaticClass());
    }
};

IMPLEMENT_MODULE(FSharImportEditorModule, SharImportEditor)
