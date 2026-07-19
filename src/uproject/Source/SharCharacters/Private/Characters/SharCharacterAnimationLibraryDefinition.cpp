// File:
//   - SharCharacterAnimationLibraryDefinition.cpp
// Path:
//   - src/uproject/Source/SharCharacters/Private/Characters/SharCharacterAnimationLibraryDefinition.cpp
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
//   - Load-free validation and Primary Asset identity for shared animation data.
// - Must-Not:
//   - Load clips, evaluate poses, retarget Skeletons, or mutate runtime state.
// - Allows:
//   - Structural validation of rig compatibility and semantic clip definitions.
// - Split-When:
//   - One clip family needs an independently loaded Primary Asset definition.
// - Merge-When:
//   - Another implementation owns the same rig-family library invariants.
// - Summary:
//   - Implements shared character-animation library validation.
// - Description:
//   - Rejects duplicate or incomplete central animation definitions before use.
// - Usage:
//   - Called by import validation, Data Validation, and automation tests.
// - Defaults:
//   - Performs no synchronous loads and accepts explicit finite timing values.
//
// ADRs:
// - docs/adr/unreal/runtime/shared-rig-family-animation-libraries.md
// - docs/adr/unreal/architecture/aaa-native-content-and-gameplay-foundation.md
//
// Large file:
//   - false
//

#include "Characters/SharCharacterAnimationLibraryDefinition.h"

#include "Content/SharPrimaryContentDefinition.h"
#include "Engine/DataAsset.h"

static void AppendLibraryDefinitionErrors(
    const USharCharacterAnimationLibraryDefinition& Definition,
    TArray<FText>& OutErrors
)
{
    if (!USharPrimaryContentDefinition::IsCanonicalIdentifier(
        Definition.RigFamilyId
    ))
    {
        OutErrors.Add(NSLOCTEXT(
            "SharCharacterAnimationLibraryDefinition",
            "InvalidRigFamily",
            "RigFamilyId must be a canonical lowercase identifier."
        ));
    }
    if (!USharPrimaryContentDefinition::IsCanonicalIdentifier(
        Definition.RigProfileId
    ))
    {
        OutErrors.Add(NSLOCTEXT(
            "SharCharacterAnimationLibraryDefinition",
            "InvalidRigProfile",
            "RigProfileId must be a canonical lowercase identifier."
        ));
    }
    if (Definition.Skeleton.IsNull())
    {
        OutErrors.Add(NSLOCTEXT(
            "SharCharacterAnimationLibraryDefinition",
            "MissingSkeleton",
            "Skeleton is required."
        ));
    }
    if (Definition.AnimationClass.IsNull())
    {
        OutErrors.Add(NSLOCTEXT(
            "SharCharacterAnimationLibraryDefinition",
            "MissingAnimationClass",
            "AnimationClass is required."
        ));
    }
}

static void AppendClipIdentityErrors(
    const FSharCharacterAnimationClipDefinition& Clip,
    TSet<FName>& SeenClipIds,
    TArray<FText>& OutErrors
)
{
    if (!USharPrimaryContentDefinition::IsCanonicalIdentifier(Clip.ClipId))
    {
        OutErrors.Add(NSLOCTEXT(
            "SharCharacterAnimationLibraryDefinition",
            "InvalidClipId",
            "Every ClipId must be a canonical lowercase identifier."
        ));
    }
    else if (SeenClipIds.Contains(Clip.ClipId))
    {
        OutErrors.Add(FText::Format(
            NSLOCTEXT(
                "SharCharacterAnimationLibraryDefinition",
                "DuplicateClipId",
                "Duplicate ClipId '{0}'."
            ),
            FText::FromName(Clip.ClipId)
        ));
    }
    else
    {
        SeenClipIds.Add(Clip.ClipId);
    }

    if (!USharPrimaryContentDefinition::IsCanonicalIdentifier(
        Clip.SemanticRoleId
    ))
    {
        OutErrors.Add(FText::Format(
            NSLOCTEXT(
                "SharCharacterAnimationLibraryDefinition",
                "InvalidSemanticRole",
                "Clip '{0}' has an invalid SemanticRoleId."
            ),
            FText::FromName(Clip.ClipId)
        ));
    }
}

static void AppendClipAssetErrors(
    const FSharCharacterAnimationClipDefinition& Clip,
    TSet<FSoftObjectPath>& SeenAnimationAssets,
    TArray<FText>& OutErrors
)
{
    const FSoftObjectPath AnimationPath =
        Clip.AnimationAsset.ToSoftObjectPath();
    if (AnimationPath.IsNull())
    {
        OutErrors.Add(FText::Format(
            NSLOCTEXT(
                "SharCharacterAnimationLibraryDefinition",
                "MissingAnimationAsset",
                "Clip '{0}' must reference a native animation asset."
            ),
            FText::FromName(Clip.ClipId)
        ));
    }
    else if (SeenAnimationAssets.Contains(AnimationPath))
    {
        OutErrors.Add(FText::Format(
            NSLOCTEXT(
                "SharCharacterAnimationLibraryDefinition",
                "DuplicateAnimationAsset",
                "Animation asset '{0}' is assigned to more than one clip."
            ),
            FText::FromString(AnimationPath.ToString())
        ));
    }
    else
    {
        SeenAnimationAssets.Add(AnimationPath);
    }
}

static void AppendClipTimingErrors(
    const FSharCharacterAnimationClipDefinition& Clip,
    TArray<FText>& OutErrors
)
{
    if (!FMath::IsFinite(Clip.ExpectedDurationSeconds)
        || Clip.ExpectedDurationSeconds < 0.0)
    {
        OutErrors.Add(FText::Format(
            NSLOCTEXT(
                "SharCharacterAnimationLibraryDefinition",
                "InvalidClipDuration",
                "Clip '{0}' has an invalid expected duration."
            ),
            FText::FromName(Clip.ClipId)
        ));
    }
    if (!FMath::IsFinite(Clip.SampleRateFramesPerSecond)
        || Clip.SampleRateFramesPerSecond <= 0.0)
    {
        OutErrors.Add(FText::Format(
            NSLOCTEXT(
                "SharCharacterAnimationLibraryDefinition",
                "InvalidSampleRate",
                "Clip '{0}' has an invalid sample rate."
            ),
            FText::FromName(Clip.ClipId)
        ));
    }
    if (Clip.bLooping && Clip.ExpectedDurationSeconds <= 0.0)
    {
        OutErrors.Add(FText::Format(
            NSLOCTEXT(
                "SharCharacterAnimationLibraryDefinition",
                "LoopingClipWithoutDuration",
                "Looping clip '{0}' must have a positive expected duration."
            ),
            FText::FromName(Clip.ClipId)
        ));
    }
}

void USharCharacterAnimationLibraryDefinition::GatherValidationErrors(
    TArray<FText>& OutErrors
) const
{
    Super::GatherValidationErrors(OutErrors);
    AppendLibraryDefinitionErrors(*this, OutErrors);
    if (Clips.IsEmpty())
    {
        OutErrors.Add(NSLOCTEXT(
            "SharCharacterAnimationLibraryDefinition",
            "MissingClips",
            "Clips must contain at least one semantic animation definition."
        ));
        return;
    }

    TSet<FName> SeenClipIds;
    TSet<FSoftObjectPath> SeenAnimationAssets;
    for (const FSharCharacterAnimationClipDefinition& Clip : Clips)
    {
        AppendClipIdentityErrors(Clip, SeenClipIds, OutErrors);
        AppendClipAssetErrors(Clip, SeenAnimationAssets, OutErrors);
        AppendClipTimingErrors(Clip, OutErrors);
    }
}

FPrimaryAssetType
USharCharacterAnimationLibraryDefinition::GetDefinitionAssetType() const
{
    return {TEXT("SharCharacterAnimationLibrary")};
}
