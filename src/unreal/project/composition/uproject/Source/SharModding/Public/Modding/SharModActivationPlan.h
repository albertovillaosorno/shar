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
//   - Shar mod activation plan composition module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Shar mod activation plan composition module.
// - Description:
//   - Implements the declared composition module responsibility for project.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Shar mod activation plan composition module.

#pragma once

#include "CoreMinimal.h"
#include "Modding/SharModDescriptor.h"

struct SHARMODDING_API FSharModActivationPlan
{
    bool bCanActivate = false;
    TArray<const USharModDescriptor*> OrderedDescriptors;
    TArray<FText> Errors;
};

class SHARMODDING_API FSharModActivationPlanner final
{
public:
    [[nodiscard]] static FSharModActivationPlan Build(
        const TArray<const USharModDescriptor*>& Descriptors
    );
};
