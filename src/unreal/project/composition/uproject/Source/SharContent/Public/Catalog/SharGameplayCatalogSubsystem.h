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
//   - Accepted root gameplay catalog revision and read-only runtime lookup.
// - Must-Not:
//   - Discover directories, load source formats, or own domain state.
// - Allows:
//   - Activation and lookup of one validated root catalog.
// - Split-When:
//   - Asynchronous Asset Manager loading gains an independent lifecycle.
// - Merge-When:
//   - Another game-instance service owns the identical catalog authority.
// - Summary:
//   - Defines the root gameplay catalog runtime authority.
// - Description:
//   - Activates one validated catalog and exposes its immutable revision.
// - Usage:
//   - Domain coordinators consume its revision and stable identities.
// - Defaults:
//   - Starts inactive and fails closed on invalid catalog data.
//

//! Root gameplay catalog runtime authority.

#pragma once

#include "Catalog/SharGameplayCatalog.h"
#include "CoreMinimal.h"
#include "Subsystems/GameInstanceSubsystem.h"

#include "SharGameplayCatalogSubsystem.generated.h"

UENUM(BlueprintType)
enum class ESharGameplayCatalogActivationResult : uint8
{
    Accepted,
    InvalidCatalog,
    AlreadyActive,
};

UCLASS()
class SHARCONTENT_API USharGameplayCatalogSubsystem final
    : public UGameInstanceSubsystem
{
    GENERATED_BODY()

public:
    UFUNCTION(BlueprintCallable, Category = "SHAR|Catalog")
    ESharGameplayCatalogActivationResult Activate(
        USharGameplayCatalog* InCatalog
    );

    UFUNCTION(BlueprintPure, Category = "SHAR|Catalog")
    [[nodiscard]] bool IsActive() const;

    UFUNCTION(BlueprintPure, Category = "SHAR|Catalog")
    [[nodiscard]] FString GetCatalogRevision() const;

    UFUNCTION(BlueprintPure, Category = "SHAR|Catalog")
    [[nodiscard]] bool ContainsDefinition(
        const FPrimaryAssetId& DefinitionId
    ) const;

    [[nodiscard]] const USharGameplayCatalog* GetCatalog() const;

private:
    UPROPERTY(Transient)
    TObjectPtr<USharGameplayCatalog> ActiveCatalog;
};
