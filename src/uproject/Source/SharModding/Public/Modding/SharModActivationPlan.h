// File: SharModActivationPlan.h
// Path: src/uproject/Source/SharModding/Public/Modding/SharModActivationPlan.h
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: deterministic dependency ordering and conflict evidence only; no package mounting.
// ADR: docs/adr/unreal/architecture/aaa-native-content-and-gameplay-foundation.md

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
