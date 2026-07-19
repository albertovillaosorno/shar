// File:
//   - SharCharacterPresentationDefinition.cpp
// Path:
//   - src/uproject/Source/SharCharacters/Private/Characters/SharCharacterPresentationDefinition.cpp
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
//   - Load-free validation and Primary Asset identity for character presentations.
// - Must-Not:
//   - Import assets, load dependencies, retarget animation, or spawn characters.
// - Allows:
//   - Structural validation of soft native-asset references and semantic profiles.
// - Split-When:
//   - A presentation subsystem gains a separate independently testable contract.
// - Merge-When:
//   - Another implementation owns the same complete-presentation invariants.
// - Summary:
//   - Implements character presentation identity and validation.
// - Description:
//   - Rejects incomplete native character presentations before publication.
// - Usage:
//   - Called by import validation, Data Validation, and automation tests.
// - Defaults:
//   - Requires every essential reference without synchronously loading it.
//
// ADRs:
// - docs/adr/unreal/architecture/aaa-native-content-and-gameplay-foundation.md
// - docs/adr/unreal/import-adapters/staged-mesh-import-and-world-assembly.md
//
// Large file:
//   - false
//

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
