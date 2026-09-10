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
//   - Vehicle presentation definition validation.
// - Must-Not:
//   - Load assets, create physics state, or configure a vehicle Pawn.
// - Allows:
//   - Validation of soft references and semantic wheel bindings.
// - Split-When:
//   - One validation family gains independent lifecycle.
// - Merge-When:
//   - Another module owns the identical presentation validation.
// - Summary:
//   - Validates one complete native vehicle presentation definition.
// - Description:
//   - Rejects incomplete or ambiguous construction inputs before asset loading.
// - Usage:
//   - Called through primary content validation.
// - Defaults:
//   - Missing or duplicate construction inputs fail closed.
//

//! Vehicle presentation definition validation.

#include "Vehicles/SharVehiclePresentationDefinition.h"

#include "Content/SharPrimaryContentDefinition.h"
#include "Engine/DataAsset.h"

namespace
{
void AddRequiredReferenceError(
    const bool bMissing,
    const TCHAR* FieldName,
    TArray<FText>& OutErrors
)
{
    if (bMissing)
    {
        OutErrors.Add(FText::Format(
            NSLOCTEXT(
                "SharVehiclePresentationDefinition",
                "MissingReference",
                "{0} is required."
            ),
            FText::FromString(FieldName)
        ));
    }
}

void AppendReferenceErrors(
    const USharVehiclePresentationDefinition& Definition,
    TArray<FText>& OutErrors
)
{
    AddRequiredReferenceError(
        Definition.SkeletalMesh.IsNull(),
        TEXT("SkeletalMesh"),
        OutErrors
    );
    AddRequiredReferenceError(
        Definition.Skeleton.IsNull(),
        TEXT("Skeleton"),
        OutErrors
    );
    AddRequiredReferenceError(
        Definition.PhysicsAsset.IsNull(),
        TEXT("PhysicsAsset"),
        OutErrors
    );
    AddRequiredReferenceError(
        Definition.AnimationClass.IsNull(),
        TEXT("AnimationClass"),
        OutErrors
    );
}

void AppendMaterialErrors(
    const USharVehiclePresentationDefinition& Definition,
    TArray<FText>& OutErrors
)
{
    if (Definition.MaterialInstances.IsEmpty())
    {
        OutErrors.Add(NSLOCTEXT(
            "SharVehiclePresentationDefinition",
            "MissingMaterials",
            "MaterialInstances requires at least one final material."
        ));
    }
    TSet<FSoftObjectPath> Seen;
    for (
        const TSoftObjectPtr<UMaterialInterface>& Material
        : Definition.MaterialInstances
    )
    {
        const FSoftObjectPath Path = Material.ToSoftObjectPath();
        if (Path.IsNull() || Seen.Contains(Path))
        {
            OutErrors.Add(NSLOCTEXT(
                "SharVehiclePresentationDefinition",
                "InvalidMaterial",
                "MaterialInstances cannot contain empty or duplicate "
                "references."
            ));
        }
        Seen.Add(Path);
    }
}


void AppendLightBindingErrors(
    const USharVehiclePresentationDefinition& Definition,
    TArray<FText>& OutErrors
)
{
    TSet<FName> SeenIds;
    TSet<FString> SeenTargets;
    for (
        const FSharVehicleLightPresentationBinding& Binding
        : Definition.LightBindings
    )
    {
        const bool bIsHeadlight =
            Binding.Role == ESharVehicleLightPresentationRole::Headlight;
        const bool bInvalidSlotPolicy = bIsHeadlight
            ? !Binding.MaterialSlotIndices.IsEmpty()
            : Binding.MaterialSlotIndices.IsEmpty();
        bool bInvalid =
            !USharPrimaryContentDefinition::IsCanonicalIdentifier(
                Binding.BindingId
            )
            || Binding.BoneName.IsNone()
            || bInvalidSlotPolicy
            || StaticEnum<ESharVehicleLightPresentationRole>()
                ->IsValidEnumValue(static_cast<int64>(Binding.Role)) == false;
        TSet<int32> SeenIndices;
        for (const int32 SlotIndex : Binding.MaterialSlotIndices)
        {
            if (
                SlotIndex < 0
                || SlotIndex >= Definition.MaterialInstances.Num()
                || SeenIndices.Contains(SlotIndex)
            )
            {
                bInvalid = true;
                continue;
            }
            SeenIndices.Add(SlotIndex);
            const FString Target = FString::Printf(
                TEXT("%d|%s|%d"),
                static_cast<int32>(Binding.Role),
                *Binding.BoneName.ToString(),
                SlotIndex
            );
            if (SeenTargets.Contains(Target))
            {
                bInvalid = true;
            }
            SeenTargets.Add(Target);
        }
        if (SeenIds.Contains(Binding.BindingId))
        {
            bInvalid = true;
        }
        SeenIds.Add(Binding.BindingId);
        if (bInvalid)
        {
            OutErrors.Add(NSLOCTEXT(
                "SharVehiclePresentationDefinition",
                "InvalidLightBinding",
                "Light bindings require unique ids and valid rig bones. "
                "Headlights are slotless; rear lights require unique "
                "in-range material slots."
            ));
        }
    }
}

void AppendWheelErrors(
    const USharVehiclePresentationDefinition& Definition,
    TArray<FText>& OutErrors
)
{
    if (Definition.Wheels.IsEmpty())
    {
        OutErrors.Add(NSLOCTEXT(
            "SharVehiclePresentationDefinition",
            "MissingWheels",
            "A drivable vehicle presentation requires native wheel bindings."
        ));
        return;
    }
    TSet<FName> SeenWheelIds;
    TSet<FName> SeenRigNames;
    for (const FSharVehicleWheelPresentationBinding& Wheel : Definition.Wheels)
    {
        const bool bInvalid =
            !USharPrimaryContentDefinition::IsCanonicalIdentifier(Wheel.WheelId)
            || Wheel.BoneName.IsNone()
            || Wheel.WheelClass.IsNull()
            || SeenWheelIds.Contains(Wheel.WheelId)
            || SeenRigNames.Contains(Wheel.BoneName);
        if (bInvalid)
        {
            OutErrors.Add(NSLOCTEXT(
                "SharVehiclePresentationDefinition",
                "InvalidWheelBinding",
                "Wheel bindings require unique ids, rig names, and wheel "
                "classes."
            ));
        }
        SeenWheelIds.Add(Wheel.WheelId);
        SeenRigNames.Add(Wheel.BoneName);
    }
}

void AppendIdentityErrors(
    const USharVehiclePresentationDefinition& Definition,
    TArray<FText>& OutErrors
)
{
    const bool bInvalid =
        !USharPrimaryContentDefinition::IsCanonicalIdentifier(
            Definition.PresentationVariant
        )
        || !USharPrimaryContentDefinition::IsCanonicalIdentifier(
            Definition.RigProfileId
        )
        || Definition.SemanticPreparationRevision.TrimStartAndEnd().IsEmpty();
    if (bInvalid)
    {
        OutErrors.Add(NSLOCTEXT(
            "SharVehiclePresentationDefinition",
            "InvalidIdentity",
            "Presentation, rig, and preparation identities are required."
        ));
    }
}
} // namespace

void USharVehiclePresentationDefinition::GatherValidationErrors(
    TArray<FText>& OutErrors
) const
{
    Super::GatherValidationErrors(OutErrors);
    AppendReferenceErrors(*this, OutErrors);
    AppendMaterialErrors(*this, OutErrors);
    AppendLightBindingErrors(*this, OutErrors);
    AppendWheelErrors(*this, OutErrors);
    AppendIdentityErrors(*this, OutErrors);
}

FPrimaryAssetType
USharVehiclePresentationDefinition::GetDefinitionAssetType() const
{
    return {TEXT("SharVehiclePresentation")};
}
