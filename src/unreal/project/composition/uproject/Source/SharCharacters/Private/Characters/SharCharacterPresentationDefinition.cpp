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
//   - Shar character presentation definition composition module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Shar character presentation definition composition module.
// - Description:
//   - Implements the declared composition module responsibility for project.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Shar character presentation definition composition module.

#include "Characters/SharCharacterPresentationDefinition.h"

#include "Content/SharPrimaryContentDefinition.h"
#include "Engine/DataAsset.h"

static void AddRequiredSoftReferenceError(
    const bool bIsMissing,
    const FText& FieldName,
    TArray<FText>& OutErrors
)
{
    if (bIsMissing)
    {
        OutErrors.Add(FText::Format(
            NSLOCTEXT(
                "SharCharacterPresentationDefinition",
                "MissingSoftReference",
                "{0} is required."
            ),
            FieldName
        ));
    }
}

static void AppendReferenceErrors(
    const USharCharacterPresentationDefinition& Definition,
    TArray<FText>& OutErrors
)
{
    AddRequiredSoftReferenceError(
        Definition.SkeletalMesh.IsNull(),
        NSLOCTEXT(
            "SharCharacterPresentationDefinition",
            "SkeletalMeshField",
            "SkeletalMesh"
        ),
        OutErrors
    );
    AddRequiredSoftReferenceError(
        Definition.Skeleton.IsNull(),
        NSLOCTEXT(
            "SharCharacterPresentationDefinition",
            "SkeletonField",
            "Skeleton"
        ),
        OutErrors
    );
    AddRequiredSoftReferenceError(
        Definition.PhysicsAsset.IsNull(),
        NSLOCTEXT(
            "SharCharacterPresentationDefinition",
            "PhysicsAssetField",
            "PhysicsAsset"
        ),
        OutErrors
    );
    AddRequiredSoftReferenceError(
        Definition.AnimationLibrary.IsNull(),
        NSLOCTEXT(
            "SharCharacterPresentationDefinition",
            "AnimationLibraryField",
            "AnimationLibrary"
        ),
        OutErrors
    );
}

static void AppendMaterialErrors(
    const USharCharacterPresentationDefinition& Definition,
    TArray<FText>& OutErrors
)
{
    if (Definition.MaterialInstances.IsEmpty())
    {
        OutErrors.Add(NSLOCTEXT(
            "SharCharacterPresentationDefinition",
            "MissingMaterials",
            // jig-ignore-next-line: exact syntax is indivisible
            "MaterialInstances must contain at least one final Material Instance."
        ));
    }
    TSet<FSoftObjectPath> SeenMaterials;
    for (
        const TSoftObjectPtr<UMaterialInterface>& Material
        : Definition.MaterialInstances
    )
    {
        const FSoftObjectPath MaterialPath = Material.ToSoftObjectPath();
        if (MaterialPath.IsNull())
        {
            OutErrors.Add(NSLOCTEXT(
                "SharCharacterPresentationDefinition",
                "NullMaterial",
                "MaterialInstances contains an empty reference."
            ));
            continue;
        }
        if (SeenMaterials.Contains(MaterialPath))
        {
            OutErrors.Add(FText::Format(
                NSLOCTEXT(
                    "SharCharacterPresentationDefinition",
                    "DuplicateMaterial",
                    "MaterialInstances contains duplicate path '{0}'."
                ),
                FText::FromString(MaterialPath.ToString())
            ));
            continue;
        }
        SeenMaterials.Add(MaterialPath);
    }
}

static void AppendProfileErrors(
    const USharCharacterPresentationDefinition& Definition,
    TArray<FText>& OutErrors
)
{
    if (!USharPrimaryContentDefinition::IsCanonicalIdentifier(
        Definition.PresentationVariant
    ))
    {
        OutErrors.Add(NSLOCTEXT(
            "SharCharacterPresentationDefinition",
            "InvalidPresentationVariant",
            "PresentationVariant must be a canonical lowercase identifier."
        ));
    }
    if (!USharPrimaryContentDefinition::IsCanonicalIdentifier(
        Definition.RigProfileId
    ))
    {
        OutErrors.Add(NSLOCTEXT(
            "SharCharacterPresentationDefinition",
            "InvalidRigProfile",
            "RigProfileId must be a canonical lowercase identifier."
        ));
    }
    if (!USharPrimaryContentDefinition::IsCanonicalIdentifier(
        Definition.TextureProfileId
    ))
    {
        OutErrors.Add(NSLOCTEXT(
            "SharCharacterPresentationDefinition",
            "InvalidTextureProfile",
            "TextureProfileId must be a canonical lowercase identifier."
        ));
    }
    if (Definition.SemanticPreparationRevision.TrimStartAndEnd().IsEmpty())
    {
        OutErrors.Add(NSLOCTEXT(
            "SharCharacterPresentationDefinition",
            "MissingSemanticPreparationRevision",
            "SemanticPreparationRevision is required."
        ));
    }
}

static void AppendDimensionErrors(
    const USharCharacterPresentationDefinition& Definition,
    TArray<FText>& OutErrors
)
{
    if (!FMath::IsFinite(Definition.ExpectedHeightCentimeters)
        || Definition.ExpectedHeightCentimeters <= 0.0)
    {
        OutErrors.Add(NSLOCTEXT(
            "SharCharacterPresentationDefinition",
            "InvalidExpectedHeight",
            "ExpectedHeightCentimeters must be finite and greater than zero."
        ));
    }
    if (
        Definition.ExpectedBoundsExtentCentimeters.ContainsNaN()
        || Definition.ExpectedBoundsExtentCentimeters.GetMin() <= 0.0
    )
    {
        OutErrors.Add(NSLOCTEXT(
            "SharCharacterPresentationDefinition",
            "InvalidExpectedBounds",
            // jig-ignore-next-line: exact syntax is indivisible
            "ExpectedBoundsExtentCentimeters must contain finite positive values."
        ));
    }
}

void USharCharacterPresentationDefinition::GatherValidationErrors(
    TArray<FText>& OutErrors
) const
{
    Super::GatherValidationErrors(OutErrors);
    AppendReferenceErrors(*this, OutErrors);
    AppendMaterialErrors(*this, OutErrors);
    AppendProfileErrors(*this, OutErrors);
    AppendDimensionErrors(*this, OutErrors);
}

FPrimaryAssetType
USharCharacterPresentationDefinition::GetDefinitionAssetType() const
{
    return {TEXT("SharCharacterPresentation")};
}
