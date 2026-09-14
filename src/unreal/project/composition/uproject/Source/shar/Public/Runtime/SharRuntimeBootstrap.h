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
//   - Cross-module runtime composition after root content is available.
// - Must-Not:
//   - Own application-mode state, load source formats, or invent content.
// - Allows:
//   - Wiring validated catalog and coordinator collaborators exactly once.
// - Split-When:
//   - Another domain requires an independently testable bootstrap transaction.
// - Merge-When:
//   - Another composition object owns the identical collaborator wiring.
// - Summary:
//   - Composes root catalog authority into the application lifecycle runtime.
// - Description:
//   - Activates a complete declared mode set and configures one coordinator.
// - Usage:
//   - Owned by the project game instance after native subsystems are available.
// - Defaults:
//   - Starts unconfigured and fails closed on incomplete collaborators.
//

//! Cross-module runtime composition after root content is available.

#pragma once

#include "Application/SharApplicationModeCoordinator.h"
#include "Catalog/SharGameplayCatalogSubsystem.h"
#include "CoreMinimal.h"

#include "SharRuntimeBootstrap.generated.h"

UENUM(BlueprintType)
enum class ESharRuntimeBootstrapResult : uint8
{
    Accepted,
    AlreadyConfigured,
    InvalidRootCatalog,
    IncompleteApplicationModes,
    ApplicationCatalogRejected,
    CoordinatorRejected,
};

UCLASS()
class SHAR_API USharRuntimeBootstrap final : public UObject
{
    GENERATED_BODY()

public:
    ESharRuntimeBootstrapResult ConfigureApplicationRuntime(
        USharGameplayCatalogSubsystem* RootCatalog,
        USharApplicationModeCatalogSubsystem* ApplicationCatalog,
        USharApplicationModeCoordinator* Coordinator,
        const TArray<USharApplicationModeDefinition*>& Modes
    );

    UFUNCTION(BlueprintPure, Category = "SHAR|Runtime")
    [[nodiscard]] bool IsApplicationRuntimeConfigured() const;

private:
    UPROPERTY(Transient)
    bool bApplicationRuntimeConfigured = false;
};
