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
//   - Shar native import automation tests.
// - Must-Not:
//   - Own runtime gameplay policy or mutate content outside generated roots.
// - Allows:
//   - Editor-only inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one imported asset family gains an independent lifecycle.
// - Merge-When:
//   - Merge when another editor module owns the identical responsibility.
// - Summary:
//   - Shar native import automation tests.
// - Description:
//   - Exercises WAV and FileMediaSource lifecycle behavior in the editor.
// - Usage:
//   - Used through the SharImportEditor module and its native toolset boundary.
// - Defaults:
//   - Invalid, ambiguous, or replacement requests fail explicitly.
//
#if WITH_DEV_AUTOMATION_TESTS

#include "Import/SharImportToolset.h"
#include "Import/SharImportValidation.h"

#include "HAL/FileManager.h"
#include "FileMediaSource.h"
#include "HAL/PlatformProcess.h"
#include "Misc/AutomationTest.h"
#include "Misc/FileHelper.h"
#include "Misc/PackageName.h"
#include "Misc/Paths.h"
#include "Sound/SoundWave.h"
#include "ToolsetRegistry/UToolsetRegistry.h"

namespace
{
FString WriteMinimalWaveFixture()
{
    static const uint8 WaveBytes[] = {
        'R', 'I', 'F', 'F', 0x26, 0x00, 0x00, 0x00,
        'W', 'A', 'V', 'E', 'f', 'm', 't', ' ',
        0x10, 0x00, 0x00, 0x00, 0x01, 0x00, 0x01, 0x00,
        0x40, 0x1F, 0x00, 0x00, 0x80, 0x3E, 0x00, 0x00,
        0x02, 0x00, 0x10, 0x00, 'd', 'a', 't', 'a',
        0x02, 0x00, 0x00, 0x00, 0x00, 0x00,
    };
    const FString Directory = FPaths::ConvertRelativePathToFull(
        FPaths::ProjectSavedDir() / TEXT("Automation/Tmp/SHAR")
    );
    IFileManager::Get().MakeDirectory(*Directory, true);
    const FString Filename = Directory / TEXT("minimal-import.wav");
    TArray<uint8> Bytes;
    Bytes.Append(WaveBytes, UE_ARRAY_COUNT(WaveBytes));
    return FFileHelper::SaveArrayToFile(Bytes, *Filename) ? Filename : FString();
}
FString WriteMinimalMovieFixture()
{
    const FString Directory = FPaths::ConvertRelativePathToFull(
        FPaths::ProjectSavedDir() / TEXT("Automation/Tmp/SHAR")
    );
    IFileManager::Get().MakeDirectory(*Directory, true);
    const FString Filename = Directory / TEXT("minimal-import.mov");
    static const uint8 MovieBytes[] = {
        'S', 'H', 'A', 'R', '-', 'H', 'A', 'P', '-', 'F', 'I', 'X', 'T', 'U', 'R', 'E',
    };
    TArray<uint8> Bytes;
    Bytes.Append(MovieBytes, UE_ARRAY_COUNT(MovieBytes));
    return FFileHelper::SaveArrayToFile(Bytes, *Filename) ? Filename : FString();
}

} // namespace

IMPLEMENT_SIMPLE_AUTOMATION_TEST(
    FSharImportValidationTest,
    "SHAR.Import.NativeAssets.RequestValidation",
    EAutomationTestFlags::EditorContext
        | EAutomationTestFlags::CommandletContext
        | EAutomationTestFlags::EngineFilter
)

bool FSharImportValidationTest::RunTest(const FString& Parameters)
{
    (void)Parameters;
    FString Error;
    TestTrue(
        TEXT("SHAR import toolset is registered after engine initialization"),
        UToolsetRegistry::IsToolsetClassRegistered(
            USharImportToolset::StaticClass()
        )
    );
    TestTrue(
        TEXT("Canonical generated WAV request passes"),
        UE::SharImportEditor::Private::ValidateSoundWaveRequest(
            TEXT("C:/SHAR/verified.wav"),
            TEXT("/Game/Generated/SHAR/audio/dialog"),
            TEXT("dialog_line"),
            Error
        )
    );
    TestFalse(
        TEXT("Non-generated destination is rejected"),
        UE::SharImportEditor::Private::ValidateSoundWaveRequest(
            TEXT("C:/SHAR/verified.wav"),
            TEXT("/Game/Unowned"),
            TEXT("dialog_line"),
            Error
        )
    );
    TestFalse(
        TEXT("Relative source is rejected"),
        UE::SharImportEditor::Private::ValidateSoundWaveRequest(
            TEXT("verified.wav"),
            TEXT("/Game/Generated/SHAR/audio/dialog"),
            TEXT("dialog_line"),
            Error
        )
    );
    TestFalse(
        TEXT("Non-WAV source is rejected"),
        UE::SharImportEditor::Private::ValidateSoundWaveRequest(
            TEXT("C:/SHAR/verified.mp3"),
            TEXT("/Game/Generated/SHAR/audio/dialog"),
            TEXT("dialog_line"),
            Error
        )
    );

    const FString Fixture = WriteMinimalWaveFixture();
    TestFalse(TEXT("Minimal WAV fixture is written"), Fixture.IsEmpty());
    const FString Folder = TEXT("/Game/Generated/SHAR/__Tests__");
    const FString AssetName = FString::Printf(
        TEXT("TransientSoundWave_%u"),
        FPlatformProcess::GetCurrentProcessId()
    );
    const FString ObjectPath = FString::Printf(
        TEXT("%s/%s.%s"),
        *Folder,
        *AssetName,
        *AssetName
    );
    const TArray<FString> Imported = USharImportToolset::ImportSoundWave(
        Fixture,
        Folder,
        AssetName
    );
    TestTrue(
        TEXT("Automated WAV import returns the planned object path"),
        Imported.Contains(ObjectPath)
    );
    USoundWave* SoundWave = FindObject<USoundWave>(nullptr, *ObjectPath);
    TestNotNull(TEXT("Imported SoundWave exists in memory"), SoundWave);
    if (SoundWave != nullptr)
    {
        UPackage* Package = SoundWave->GetPackage();
        TestTrue(TEXT("Imported package is dirty before explicit save"), Package->IsDirty());
        const FString PackageFilename = FPackageName::LongPackageNameToFilename(
            Package->GetName(),
            FPackageName::GetAssetPackageExtension()
        );
        TestFalse(
            TEXT("Native import does not save the package implicitly"),
            IFileManager::Get().FileExists(*PackageFilename)
        );
        Package->SetDirtyFlag(false);
        SoundWave->ClearFlags(RF_Public | RF_Standalone);
        SoundWave->MarkAsGarbage();
        Package->MarkAsGarbage();
    }
    TestTrue(
        TEXT("Temporary WAV fixture is removed"),
        IFileManager::Get().Delete(*Fixture, false, true)
    );

    using UE::SharImportEditor::Private::FFileMediaSourcePaths;
    const FString MovieFixture = WriteMinimalMovieFixture();
    TestFalse(TEXT("Minimal MOV fixture is written"), MovieFixture.IsEmpty());
    const FString MediaName = FString::Printf(
        TEXT("TransientMedia_%u"),
        FPlatformProcess::GetCurrentProcessId()
    );
    FFileMediaSourcePaths MediaPaths;
    TestTrue(
        TEXT("Canonical media request derives deterministic paths"),
        UE::SharImportEditor::Private::ValidateFileMediaSourceRequest(
            MovieFixture,
            Folder,
            MediaName,
            MediaPaths,
            Error
        )
    );
    TestEqual(
        TEXT("Media payload is package-relative under Content/Movies"),
        MediaPaths.RelativePayloadPath,
        FString::Printf(
            TEXT("./Movies/Generated/SHAR/__Tests__/%s.mov"),
            *MediaName
        )
    );
    const TArray<FString> ImportedMedia =
        USharImportToolset::ImportFileMediaSource(
            MovieFixture,
            Folder,
            MediaName
        );
    TestTrue(
        TEXT("Media import returns the planned object path"),
        ImportedMedia.Contains(MediaPaths.ObjectPath)
    );
    UFileMediaSource* MediaSource = FindObject<UFileMediaSource>(
        nullptr,
        *MediaPaths.ObjectPath
    );
    TestNotNull(TEXT("Imported FileMediaSource exists in memory"), MediaSource);
    if (MediaSource != nullptr)
    {
        TestEqual(
            TEXT("FileMediaSource stores the planned relative path"),
            MediaSource->GetFilePath(),
            MediaPaths.RelativePayloadPath
        );
        TestTrue(
            TEXT("External movie payload exists"),
            USharImportToolset::FileMediaSourcePayloadExists(
                MediaPaths.ObjectPath
            )
        );
        TArray<uint8> SourceBytes;
        TArray<uint8> PayloadBytes;
        TestTrue(
            TEXT("Source movie bytes can be read"),
            FFileHelper::LoadFileToArray(SourceBytes, *MovieFixture)
        );
        TestTrue(
            TEXT("Published movie bytes can be read"),
            FFileHelper::LoadFileToArray(
                PayloadBytes,
                *MediaPaths.FullPayloadPath
            )
        );
        TestEqual(
            TEXT("Published movie bytes equal verified source bytes"),
            PayloadBytes,
            SourceBytes
        );
        UPackage* MediaPackage = MediaSource->GetPackage();
        TestTrue(
            TEXT("Media package is dirty before explicit save"),
            MediaPackage->IsDirty()
        );
        const FString MediaPackageFilename =
            FPackageName::LongPackageNameToFilename(
                MediaPackage->GetName(),
                FPackageName::GetAssetPackageExtension()
            );
        TestFalse(
            TEXT("Media import does not save its package implicitly"),
            IFileManager::Get().FileExists(*MediaPackageFilename)
        );
        TestTrue(
            TEXT("Compensating payload delete succeeds"),
            USharImportToolset::DeleteFileMediaSourcePayload(
                MediaPaths.ObjectPath
            )
        );
        TestFalse(
            TEXT("Compensating payload delete is independently verified"),
            USharImportToolset::FileMediaSourcePayloadExists(
                MediaPaths.ObjectPath
            )
        );
        MediaPackage->SetDirtyFlag(false);
        MediaSource->ClearFlags(RF_Public | RF_Standalone);
        MediaSource->MarkAsGarbage();
        MediaPackage->MarkAsGarbage();
    }
    TestTrue(
        TEXT("Temporary MOV fixture is removed"),
        IFileManager::Get().Delete(*MovieFixture, false, true)
    );
    return true;
}

#endif
