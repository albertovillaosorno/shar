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
//   - Editor-only native import tools for generated SHAR content.
// - Must-Not:
//   - Save packages, overwrite assets, or import outside generated roots.
// - Allows:
//   - Automated factory-backed imports with explicit result identities.
// - Split-When:
//   - Split when one imported asset family gains independent policy.
// - Merge-When:
//   - Merge when another editor toolset owns identical import contracts.
// - Summary:
//   - SHAR generated content import toolset.
// - Description:
//   - Exposes bounded editor imports through Unreal ToolsetRegistry and MCP.
// - Usage:
//   - Called by the repository plan application after physical preflight.
// - Defaults:
//   - Imports are synchronous, automated, unsaved, and non-replacing.
//

#pragma once


#include "ToolsetRegistry/ToolsetDefinition.h"

#include "SharImportToolset.generated.h"

UCLASS(BlueprintType)
class SHARIMPORTEDITOR_API USharImportToolset : public UToolsetDefinition
{
    GENERATED_BODY()

public:
    /**
     * Imports one reviewed FBX as a StaticMesh under /Game/Generated/SHAR.
     * Authored normals are preserved and auxiliary assets are not generated.
     * The caller remains responsible for saving and postcondition read-back.
     * @param SourceFile Absolute verified FBX source path.
     * @param FolderPath Generated Unreal content folder.
     * @param AssetName Exact destination asset name.
     * @return The single StaticMesh object path produced by the import task.
     */
    UFUNCTION(meta = (AICallable), Category = "SharImportToolset")
    static TArray<FString> ImportStaticMesh(
        const FString& SourceFile,
        const FString& FolderPath,
        const FString& AssetName
    );

    /**
     * Imports one reviewed FBX as a SkeletalMesh plus a new Skeleton companion.
     * PhysicsAsset and animation creation are disabled for this transaction.
     * The caller remains responsible for saving and postcondition read-back.
     * @param SourceFile Absolute verified FBX source path.
     * @param FolderPath Generated Unreal content folder.
     * @param AssetName Exact destination SkeletalMesh asset name.
     * @return SkeletalMesh then Skeleton object paths owned by the transaction.
     */
    UFUNCTION(meta = (AICallable), Category = "SharImportToolset")
    static TArray<FString> ImportSkeletalMesh(
        const FString& SourceFile,
        const FString& FolderPath,
        const FString& AssetName
    );

    /**
     * Imports one WAV file as a SoundWave under /Game/Generated/SHAR.
     * The caller remains responsible for saving and postcondition read-back.
     * @param SourceFile Absolute verified WAV source path.
     * @param FolderPath Generated Unreal content folder.
     * @param AssetName Exact destination asset name.
     * @return Object paths produced by the automated import task.
     */
    UFUNCTION(meta = (AICallable), Category = "SharImportToolset")
    static TArray<FString> ImportSoundWave(
        const FString& SourceFile,
        const FString& FolderPath,
        const FString& AssetName
    );

    /**
     * Copies one verified HAP MOV into Content/Movies and creates its source.
     * The package remains dirty for explicit save by the plan transaction.
     * @param SourceFile Absolute verified MOV source path.
     * @param FolderPath Generated Unreal content folder.
     * @param AssetName Exact destination asset name.
     * @return The created FileMediaSource object path.
     */
    UFUNCTION(meta = (AICallable), Category = "SharImportToolset")
    static TArray<FString> ImportFileMediaSource(
        const FString& SourceFile,
        const FString& FolderPath,
        const FString& AssetName
    );

    /** Return the stored relative movie path for one FileMediaSource. */
    UFUNCTION(meta = (AICallable), Category = "SharImportToolset")
    static FString GetFileMediaSourcePath(const FString& AssetPath);

    /** Test whether the deterministic external movie payload exists. */
    UFUNCTION(meta = (AICallable), Category = "SharImportToolset")
    static bool FileMediaSourcePayloadExists(const FString& AssetPath);

    /** Delete only the deterministic external movie payload for an asset. */
    UFUNCTION(meta = (AICallable), Category = "SharImportToolset")
    static bool DeleteFileMediaSourcePayload(const FString& AssetPath);
};
