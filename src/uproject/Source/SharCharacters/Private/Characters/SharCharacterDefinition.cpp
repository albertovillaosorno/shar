// File:
//   - SharCharacterDefinition.cpp
// Path:
//   - src/uproject/Source/SharCharacters/Private/Characters/SharCharacterDefinition.cpp
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
//   - Load-free validation and Primary Asset identity for character definitions.
// - Must-Not:
//   - Spawn characters, inspect progression, load presentations, or run gameplay.
// - Allows:
//   - Structural validation of character profiles and soft implementation links.
// - Split-When:
//   - A character domain rule needs an independently versioned definition asset.
// - Merge-When:
//   - Another implementation owns the same canonical character invariants.
// - Summary:
//   - Implements character definition identity and validation.
// - Description:
//   - Keeps character identity stable while concrete gameplay remains replaceable.
// - Usage:
//   - Called by import validation, catalog tests, and Unreal Data Validation.
// - Defaults:
//   - Requires explicit implementation profiles and valid capsule dimensions.
//
// ADRs:
// - docs/adr/unreal/architecture/aaa-native-content-and-gameplay-foundation.md
// - docs/adr/unreal/runtime/data-driven-gameplay-content-catalog.md
//
// Large file:
//   - false
//

#include "Characters/SharCharacterDefinition.h"

#include "Characters/SharCharacterPresentationDefinition.h"
#include "Content/SharPrimaryContentDefinition.h"
#include "Engine/DataAsset.h"

static void AppendRequiredAssetErrors(
    const USharCharacterDefinition& Definition,
    TArray<FText>& OutErrors
)
{
    if (Definition.CharacterClass.IsNull())
    {
        OutErrors.Add(NSLOCTEXT(
            "SharCharacterDefinition",
            "MissingCharacterClass",
            "CharacterClass is required."
        ));
    }
    if (Definition.DefaultPresentation.IsNull())
    {
        OutErrors.Add(NSLOCTEXT(
            "SharCharacterDefinition",
            "MissingDefaultPresentation",
            "DefaultPresentation is required."
        ));
    }
}

static void AppendPresentationVariantErrors(
    const USharCharacterDefinition& Definition,
    TArray<FText>& OutErrors
)
{
    TSet<FSoftObjectPath> SeenPresentations;
    const FSoftObjectPath DefaultPath =
        Definition.DefaultPresentation.ToSoftObjectPath();
    for (
        const TSoftObjectPtr<USharCharacterPresentationDefinition>& Presentation
        : Definition.PresentationVariants
    )
    {
        const FSoftObjectPath PresentationPath =
            Presentation.ToSoftObjectPath();
        if (PresentationPath.IsNull())
        {
            OutErrors.Add(NSLOCTEXT(
                "SharCharacterDefinition",
                "NullPresentationVariant",
                "PresentationVariants contains an empty reference."
            ));
            continue;
        }
        if (PresentationPath == DefaultPath)
        {
            OutErrors.Add(NSLOCTEXT(
                "SharCharacterDefinition",
                "DefaultPresentationRepeated",
                "PresentationVariants cannot repeat DefaultPresentation."
            ));
        }
        if (SeenPresentations.Contains(PresentationPath))
        {
            OutErrors.Add(FText::Format(
                NSLOCTEXT(
                    "SharCharacterDefinition",
                    "DuplicatePresentationVariant",
                    "PresentationVariants contains duplicate path '{0}'."
                ),
                FText::FromString(PresentationPath.ToString())
            ));
            continue;
        }
        SeenPresentations.Add(PresentationPath);
    }
}

static void AppendRequiredProfileError(
    const FName& ProfileId,
    const FText& FieldName,
    TArray<FText>& OutErrors
)
{
    if (!USharPrimaryContentDefinition::IsCanonicalIdentifier(ProfileId))
    {
        OutErrors.Add(FText::Format(
            NSLOCTEXT(
                "SharCharacterDefinition",
                "InvalidRequiredProfile",
                "{0} must be a canonical lowercase identifier."
            ),
            FieldName
        ));
    }
}

static void AppendRequiredProfileErrors(
    const USharCharacterDefinition& Definition,
    TArray<FText>& OutErrors
)
{
    AppendRequiredProfileError(
        Definition.MovementProfileId,
        NSLOCTEXT(
            "SharCharacterDefinition",
            "MovementProfileField",
            "MovementProfileId"
        ),
        OutErrors
    );
    AppendRequiredProfileError(
        Definition.AbilitySetId,
        NSLOCTEXT(
            "SharCharacterDefinition",
            "AbilitySetField",
            "AbilitySetId"
        ),
        OutErrors
    );
    AppendRequiredProfileError(
        Definition.CameraProfileId,
        NSLOCTEXT(
            "SharCharacterDefinition",
            "CameraProfileField",
            "CameraProfileId"
        ),
        OutErrors
    );
    AppendRequiredProfileError(
        Definition.VoiceProfileId,
        NSLOCTEXT(
            "SharCharacterDefinition",
            "VoiceProfileField",
            "VoiceProfileId"
        ),
        OutErrors
    );
    AppendRequiredProfileError(
        Definition.FootprintProfileId,
        NSLOCTEXT(
            "SharCharacterDefinition",
            "FootprintProfileField",
            "FootprintProfileId"
        ),
        OutErrors
    );
    AppendRequiredProfileError(
        Definition.UnlockPolicyId,
        NSLOCTEXT(
            "SharCharacterDefinition",
            "UnlockPolicyField",
            "UnlockPolicyId"
        ),
        OutErrors
    );
}

static void AppendCapsuleErrors(
    const USharCharacterDefinition& Definition,
    TArray<FText>& OutErrors
)
{
    if (!FMath::IsFinite(Definition.CapsuleRadiusCentimeters)
        || Definition.CapsuleRadiusCentimeters <= 0.0)
    {
        OutErrors.Add(NSLOCTEXT(
            "SharCharacterDefinition",
            "InvalidCapsuleRadius",
            "CapsuleRadiusCentimeters must be finite and greater than zero."
        ));
    }
    if (!FMath::IsFinite(Definition.CapsuleHalfHeightCentimeters)
        || Definition.CapsuleHalfHeightCentimeters <= 0.0)
    {
        OutErrors.Add(NSLOCTEXT(
            "SharCharacterDefinition",
            "InvalidCapsuleHalfHeight",
            "CapsuleHalfHeightCentimeters must be finite and greater than zero."
        ));
    }
    if (
        Definition.CapsuleHalfHeightCentimeters
        < Definition.CapsuleRadiusCentimeters
    )
    {
        OutErrors.Add(NSLOCTEXT(
            "SharCharacterDefinition",
            "CapsuleHalfHeightBelowRadius",
            "CapsuleHalfHeightCentimeters cannot be smaller than the radius."
        ));
    }
}

void USharCharacterDefinition::GatherValidationErrors(
    TArray<FText>& OutErrors
) const
{
    Super::GatherValidationErrors(OutErrors);
    AppendRequiredAssetErrors(*this, OutErrors);
    AppendPresentationVariantErrors(*this, OutErrors);
    AppendRequiredProfileErrors(*this, OutErrors);
    AppendCapsuleErrors(*this, OutErrors);
}

FPrimaryAssetType USharCharacterDefinition::GetDefinitionAssetType() const
{
    return {TEXT("SharCharacter")};
}
