// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT
//
// Boundary-Contract:
// - Owns:
//   - Shar primary content validation composition module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Shar primary content validation composition module.
// - Description:
//   - Implements the declared composition module responsibility for project.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Shar primary content validation composition module.

#pragma once
// File:
//   - SharPrimaryContentValidation.h
// jig-ignore-next-line: exact syntax is indivisible
// Path: src/unreal/project/composition/uproject/Source/SharContent/Private/Content/SharPrimaryContentValidation.h
//
// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT
// Path-Rule:
//   - All paths in this header are repository-root relative.
//
// Boundary-Contract:
// - Owns:
//   - Private load-free validation operations for primary content definitions.
// - Must-Not:
//   - Load assets, expose a public module API, or apply family-specific policy.
// - Allows:
//   - Focused identity, provenance, and dependency validation functions.
// - Split-When:
//   - One validation family gains independent state or external consumers.
// - Merge-When:
//   - The primary definition no longer requires independently testable checks.
// - Summary:
//   - Declares cohesive primary-content validation operations.
// - Description:
//   - Keeps strict complexity limits without changing the UObject contract.
// - Usage:
//   - Called only by USharPrimaryContentDefinition implementation code.
// - Defaults:
//   - Performs deterministic validation without synchronous loading.
//
// ADRs:
// - docs/adr/unreal/architecture/aaa-native-content-and-gameplay-foundation.md
//
// Large file:
//   - false
//

#include "CoreMinimal.h"
#include "Engine/DataAsset.h"

class USharPrimaryContentDefinition;

class FSharPrimaryContentValidation final
{
public:
    static void AppendIdentityErrors(
        const USharPrimaryContentDefinition& Definition,
        const FPrimaryAssetType& AssetType,
        TArray<FText>& OutErrors
    );

    static void AppendProvenanceErrors(
        const USharPrimaryContentDefinition& Definition,
        TArray<FText>& OutErrors
    );

    static void AppendDependencyErrors(
        const USharPrimaryContentDefinition& Definition,
        const FPrimaryAssetId& SelfId,
        TArray<FText>& OutErrors
    );
};
