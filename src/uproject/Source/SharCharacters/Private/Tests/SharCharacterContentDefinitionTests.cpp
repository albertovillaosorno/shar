// File:
//   - SharCharacterContentDefinitionTests.cpp
// Path:
//   - src/uproject/Source/SharCharacters/Private/Tests/SharCharacterContentDefinitionTests.cpp
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
//   - Native regression coverage for shared content and character definitions.
// - Must-Not:
//   - Load project assets, mutate editor packages, or depend on private fixtures.
// - Allows:
//   - Transient definitions, synthetic soft paths, and load-free validation.
// - Split-When:
//   - One definition family needs independent fixture setup or editor integration.
// - Merge-When:
//   - Another test owns the same native definition acceptance scenarios.
// - Summary:
//   - Verifies stable character identities and shared animation-library contracts.
// - Description:
//   - Exercises valid and adversarial definitions without requiring imported art.
// - Usage:
//   - Run with the SHAR Unreal automation-test filter.
// - Defaults:
//   - Uses transient objects and nonexistent but syntactically valid soft paths.
//
// ADRs:
// - docs/adr/unreal/runtime/shared-rig-family-animation-libraries.md
// - docs/adr/unreal/architecture/aaa-native-content-and-gameplay-foundation.md
//
// Large file:
//   - false
//

#if WITH_DEV_AUTOMATION_TESTS

#include "Characters/SharCharacterAnimationLibraryDefinition.h"
#include "Characters/SharCharacterDefinition.h"
#include "Characters/SharCharacterPresentationDefinition.h"
#include "Content/SharPrimaryContentDefinition.h"

#include "Animation/AnimInstance.h"
#include "Animation/AnimationAsset.h"
#include "Animation/Skeleton.h"
#include "Engine/SkeletalMesh.h"
#include "GameFramework/Character.h"
#include "Materials/MaterialInterface.h"
#include "Misc/AutomationTest.h"
#include "PhysicsEngine/PhysicsAsset.h"

static constexpr double FixtureBoundsXCentimeters = 45.0;
static constexpr double FixtureBoundsYCentimeters = 35.0;
static constexpr double FixtureBoundsZCentimeters = 90.0;
static constexpr double FixtureCapsuleRadiusCentimeters = 42.0;
static constexpr double FixtureCapsuleHalfHeightCentimeters = 96.0;
static constexpr double InvalidCapsuleHalfHeightCentimeters = 20.0;

template <typename TObject>
static TSoftObjectPtr<TObject> MakeSoftObject(const TCHAR* ObjectPath)
{
    return TSoftObjectPtr<TObject>(FSoftObjectPath(ObjectPath));
}

template <typename TObject>
static TSoftClassPtr<TObject> MakeSoftClass(const TCHAR* ClassPath)
{
    return TSoftClassPtr<TObject>(FSoftObjectPath(ClassPath));
}

static void FillSharedDefinition(
    USharPrimaryContentDefinition& Definition,
    const FName& CanonicalId
)
{
    Definition.CanonicalId = CanonicalId;
    Definition.DisplayName = FText::FromString(TEXT("Synthetic definition"));
    Definition.SourcePackageIds = {FName(TEXT("fixture_package"))};
    Definition.RevisionToken = TEXT("sha256:fixture_revision");
    Definition.ValidationProfile = FName(TEXT("fixture_validation_v1"));
    Definition.OwningFeature = FName(TEXT("base"));
}

static bool ContainsError(
    const TArray<FText>& Errors,
    const TCHAR* ExpectedSubstring
)
{
    return Errors.ContainsByPredicate(
        [ExpectedSubstring](const FText& Error)
        {
            return Error.ToString().Contains(ExpectedSubstring);
        }
    );
}

static USharCharacterAnimationLibraryDefinition* MakeValidAnimationLibrary()
{
    auto* Library = NewObject<USharCharacterAnimationLibraryDefinition>();
    FillSharedDefinition(*Library, FName(TEXT("humanoid_common_v1")));
    Library->RigFamilyId = FName(TEXT("humanoid_common_v1"));
    Library->RigProfileId = FName(TEXT("humanoid_rig_v1"));
    Library->Skeleton = MakeSoftObject<USkeleton>(
        TEXT("/Game/SHAR/Tests/Generated/SKEL_fixture.SKEL_fixture")
    );
    Library->AnimationClass = MakeSoftClass<UAnimInstance>(
        TEXT("/Game/SHAR/Tests/Generated/ABP_fixture.ABP_fixture_C")
    );

    FSharCharacterAnimationClipDefinition Clip;
    Clip.ClipId = FName(TEXT("locomotion_walk_forward"));
    Clip.SemanticRoleId = FName(TEXT("locomotion_walk_forward"));
    Clip.AnimationAsset = MakeSoftObject<UAnimationAsset>(
        TEXT("/Game/SHAR/Tests/Generated/A_fixture_walk.A_fixture_walk")
    );
    Clip.ExpectedDurationSeconds = 1.0;
    Clip.SampleRateFramesPerSecond =
        FSharCharacterAnimationClipDefinition::
            DefaultSampleRateFramesPerSecond;
    Clip.bLooping = true;
    Library->Clips.Add(Clip);
    return Library;
}

static USharCharacterPresentationDefinition* MakeValidPresentation()
{
    auto* Presentation = NewObject<USharCharacterPresentationDefinition>();
    FillSharedDefinition(*Presentation, FName(TEXT("fixture_character_default")));
    Presentation->PresentationVariant = FName(TEXT("default"));
    Presentation->SkeletalMesh = MakeSoftObject<USkeletalMesh>(
        TEXT("/Game/SHAR/Tests/Generated/SK_fixture.SK_fixture")
    );
    Presentation->Skeleton = MakeSoftObject<USkeleton>(
        TEXT("/Game/SHAR/Tests/Generated/SKEL_fixture.SKEL_fixture")
    );
    Presentation->PhysicsAsset = MakeSoftObject<UPhysicsAsset>(
        TEXT("/Game/SHAR/Tests/Generated/PHYS_fixture.PHYS_fixture")
    );
    Presentation->MaterialInstances.Add(MakeSoftObject<UMaterialInterface>(
        TEXT("/Game/SHAR/Tests/Generated/MI_fixture.MI_fixture")
    ));
    Presentation->RigProfileId = FName(TEXT("humanoid_rig_v1"));
    Presentation->AnimationLibrary =
        MakeSoftObject<USharCharacterAnimationLibraryDefinition>(
            TEXT(
                "/Game/SHAR/Tests/Generated/"
                "DA_CharacterAnimationLibrary_fixture."
                "DA_CharacterAnimationLibrary_fixture"
            )
        );
    Presentation->TextureProfileId = FName(TEXT("hero_2k"));
    Presentation->SemanticPreparationRevision = TEXT("sha256:semantic_fixture");
    Presentation->ExpectedHeightCentimeters =
        USharCharacterPresentationDefinition::
            DefaultExpectedHeightCentimeters;
    Presentation->ExpectedBoundsExtentCentimeters = FVector(
        FixtureBoundsXCentimeters,
        FixtureBoundsYCentimeters,
        FixtureBoundsZCentimeters
    );
    return Presentation;
}

static USharCharacterDefinition* MakeValidCharacter()
{
    auto* Character = NewObject<USharCharacterDefinition>();
    FillSharedDefinition(*Character, FName(TEXT("fixture_character")));
    Character->CharacterClass = MakeSoftClass<ACharacter>(
        TEXT("/Game/SHAR/Tests/Generated/BP_fixture.BP_fixture_C")
    );
    Character->DefaultPresentation =
        MakeSoftObject<USharCharacterPresentationDefinition>(
            TEXT(
                "/Game/SHAR/Tests/Generated/"
                "DA_CharacterPresentation_fixture."
                "DA_CharacterPresentation_fixture"
            )
        );
    Character->MovementProfileId = FName(TEXT("character_movement_v1"));
    Character->AbilitySetId = FName(TEXT("character_abilities_v1"));
    Character->CameraProfileId = FName(TEXT("third_person_default_v1"));
    Character->VoiceProfileId = FName(TEXT("fixture_voice_v1"));
    Character->FootprintProfileId = FName(TEXT("standard_footprints_v1"));
    Character->UnlockPolicyId = FName(TEXT("mission_reward_v1"));
    Character->CapsuleRadiusCentimeters = FixtureCapsuleRadiusCentimeters;
    Character->CapsuleHalfHeightCentimeters =
        FixtureCapsuleHalfHeightCentimeters;
    return Character;
}

namespace
{
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharCanonicalContentIdentifierTest,
    "SHAR.Content.Identity.CanonicalIdentifier",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
} // namespace

bool FSharCanonicalContentIdentifierTest::RunTest(const FString& Parameters)
{
    static_cast<void>(Parameters);
    TestTrue(
        TEXT("lowercase snake case is accepted"),
        USharPrimaryContentDefinition::IsCanonicalIdentifier(
            FName(TEXT("humanoid_common_v1"))
        )
    );
    TestFalse(
        TEXT("uppercase is rejected"),
        USharPrimaryContentDefinition::IsCanonicalIdentifier(
            FName(TEXT("UpperCaseOnlyFixture"))
        )
    );
    TestFalse(
        TEXT("leading underscore is rejected"),
        USharPrimaryContentDefinition::IsCanonicalIdentifier(
            FName(TEXT("_leading"))
        )
    );
    TestFalse(
        TEXT("repeated underscore is rejected"),
        USharPrimaryContentDefinition::IsCanonicalIdentifier(
            FName(TEXT("repeated__underscore"))
        )
    );
    TestFalse(
        TEXT("hyphen is rejected"),
        USharPrimaryContentDefinition::IsCanonicalIdentifier(
            FName(TEXT("invalid-hyphen"))
        )
    );
    return true;
}

namespace
{
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharCharacterContentDefinitionAcceptanceTest,
    "SHAR.Content.Characters.ValidDefinitions",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
} // namespace

bool FSharCharacterContentDefinitionAcceptanceTest::RunTest(
    const FString& Parameters
)
{
    static_cast<void>(Parameters);

    const auto* Library = MakeValidAnimationLibrary();
    TArray<FText> LibraryErrors;
    Library->GatherValidationErrors(LibraryErrors);
    TestEqual(TEXT("shared library is valid"), LibraryErrors.Num(), 0);
    TestEqual(
        TEXT("shared library Primary Asset identity is stable"),
        Library->GetPrimaryAssetId().ToString(),
        FString(TEXT("SharCharacterAnimationLibrary:humanoid_common_v1"))
    );

    const auto* Presentation = MakeValidPresentation();
    TArray<FText> PresentationErrors;
    Presentation->GatherValidationErrors(PresentationErrors);
    TestEqual(TEXT("presentation is valid"), PresentationErrors.Num(), 0);
    TestEqual(
        TEXT("presentation Primary Asset identity is stable"),
        Presentation->GetPrimaryAssetId().ToString(),
        FString(TEXT("SharCharacterPresentation:fixture_character_default"))
    );

    const auto* Character = MakeValidCharacter();
    TArray<FText> CharacterErrors;
    Character->GatherValidationErrors(CharacterErrors);
    TestEqual(TEXT("character is valid"), CharacterErrors.Num(), 0);
    TestEqual(
        TEXT("character Primary Asset identity is stable"),
        Character->GetPrimaryAssetId().ToString(),
        FString(TEXT("SharCharacter:fixture_character"))
    );
    return true;
}

namespace
{
IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharCharacterContentDefinitionRejectionTest,
    "SHAR.Content.Characters.InvalidDefinitions",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::ClientContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)
} // namespace

bool FSharCharacterContentDefinitionRejectionTest::RunTest(
    const FString& Parameters
)
{
    static_cast<void>(Parameters);

    USharCharacterAnimationLibraryDefinition* Library =
        MakeValidAnimationLibrary();
    FSharCharacterAnimationClipDefinition Duplicate = Library->Clips.Last();
    Duplicate.SampleRateFramesPerSecond = 0.0;
    Library->Clips.Add(Duplicate);
    TArray<FText> LibraryErrors;
    Library->GatherValidationErrors(LibraryErrors);
    TestTrue(
        TEXT("duplicate clip identity is rejected"),
        ContainsError(LibraryErrors, TEXT("Duplicate ClipId"))
    );
    TestTrue(
        TEXT("duplicate animation asset is rejected"),
        ContainsError(LibraryErrors, TEXT("assigned to more than one clip"))
    );
    TestTrue(
        TEXT("invalid sample rate is rejected"),
        ContainsError(LibraryErrors, TEXT("invalid sample rate"))
    );

    USharCharacterPresentationDefinition* Presentation = MakeValidPresentation();
    Presentation->AnimationLibrary.Reset();
    Presentation->ExpectedBoundsExtentCentimeters.X = -1.0;
    TArray<FText> PresentationErrors;
    Presentation->GatherValidationErrors(PresentationErrors);
    TestTrue(
        TEXT("missing shared library is rejected"),
        ContainsError(PresentationErrors, TEXT("AnimationLibrary is required"))
    );
    TestTrue(
        TEXT("invalid presentation bounds are rejected"),
        ContainsError(PresentationErrors, TEXT("finite positive values"))
    );

    USharCharacterDefinition* Character = MakeValidCharacter();
    Character->CharacterClass.Reset();
    Character->CapsuleHalfHeightCentimeters =
        InvalidCapsuleHalfHeightCentimeters;
    TArray<FText> CharacterErrors;
    Character->GatherValidationErrors(CharacterErrors);
    TestTrue(
        TEXT("missing character class is rejected"),
        ContainsError(CharacterErrors, TEXT("CharacterClass is required"))
    );
    TestTrue(
        TEXT("invalid capsule shape is rejected"),
        ContainsError(CharacterErrors, TEXT("cannot be smaller than the radius"))
    );
    return true;
}

#endif
