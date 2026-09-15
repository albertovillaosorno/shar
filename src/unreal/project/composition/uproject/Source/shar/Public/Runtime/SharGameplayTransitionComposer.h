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
//   - Correlation of gameplay loading, world readiness, and input leases at the
//   - application transition composition boundary.
// - Must-Not:
//   - Own application mode state, loading state, world readiness, or input
//   - mappings independently from their native authorities.
// - Allows:
//   - Publishing correlated service evidence and ordering gameplay commit
//   - effects around the application coordinator commit point.
// - Split-When:
//   - Another mode family requires materially different commit effects.
// - Merge-When:
//   - Another root adapter owns the identical gameplay transition transaction.
// - Summary:
//   - Correlates gameplay readiness and activates input only after mode commit.
// - Description:
//   - Reads authority snapshots, records service evidence, and fail-closes
//   - partial post-commit input activation through application recovery.
// - Usage:
//   - Owned by the project game instance and invoked by gameplay loading flow.
// - Defaults:
//   - Stateless; invalid, stale, or incomplete collaborators are rejected.
//

//! Root gameplay transition readiness and commit composition.

#pragma once

#include "CoreMinimal.h"
#include "UObject/Object.h"

#include "SharGameplayTransitionComposer.generated.h"

class USharApplicationModeCatalogSubsystem;
class USharApplicationModeCoordinator;
class USharLoadCoordinatorSubsystem;
class USharLocalPlayerInputSubsystem;
class USharWorldReadinessSubsystem;

struct FSharGameplayInputLeaseBinding
{
    USharLocalPlayerInputSubsystem* InputSubsystem = nullptr;
    FName LeaseId;
};

struct FSharGameplayTransitionCompositionRequest
{
    FName ApplicationRequestId;
    FName LoadRequestId;
    FName WorldBarrierId;
    FName LoadingServiceId;
    FName WorldServiceId;
    FName InputServiceId;
    TArray<FSharGameplayInputLeaseBinding> InputLeases;
};

UENUM(BlueprintType)
enum class ESharGameplayTransitionCompositionResult : uint8
{
    Accepted,
    InvalidRequest,
    ApplicationUnavailable,
    LoadingUnavailable,
    WorldUnavailable,
    InputUnavailable,
    StaleRevision,
    ApplicationRejected,
    InputCommitFailed,
    LoadingCommitFailed,
    RecoveryFailed,
};

UCLASS()
class SHAR_API USharGameplayTransitionComposer final : public UObject
{
    GENERATED_BODY()

public:
    ESharGameplayTransitionCompositionResult PrepareGameplayReadiness(
        const FSharGameplayTransitionCompositionRequest& Request,
        USharApplicationModeCatalogSubsystem* ApplicationCatalog,
        USharApplicationModeCoordinator* ApplicationCoordinator,
        USharLoadCoordinatorSubsystem* LoadCoordinator,
        USharWorldReadinessSubsystem* WorldReadiness
    ) const;

    ESharGameplayTransitionCompositionResult CommitGameplay(
        const FSharGameplayTransitionCompositionRequest& Request,
        USharApplicationModeCatalogSubsystem* ApplicationCatalog,
        USharApplicationModeCoordinator* ApplicationCoordinator,
        USharLoadCoordinatorSubsystem* LoadCoordinator,
        USharWorldReadinessSubsystem* WorldReadiness
    ) const;
};
