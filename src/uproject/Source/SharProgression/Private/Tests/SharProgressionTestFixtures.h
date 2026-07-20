// File: SharProgressionTestFixtures.h
// Path: src/uproject/Source/SharProgression/Private/Tests/SharProgressionTestFixtures.h
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: explicit transient catalog, profile snapshot, mutation, commit, and projection fixtures only.
// Specification: docs/technical/unreal/progression-collectibles-and-cheats.md
// LARGE-FILE owner=SharProgression; reason=cohesive typed profile and progression test fixtures;
// split=extract projection fixtures when campaign-domain joins are implemented;
// validation=validate.sh SharProgression plus Unreal automation; review=2027-01.

#pragma once

#include "Progression/SharProgressionCatalogDefinition.h"
#include "Progression/SharProgressionCatalogSubsystem.h"
#include "Progression/SharProgressionContracts.h"
#include "Progression/SharProgressionState.h"
#include "Progression/SharProgressionSubsystem.h"

#include "Engine/GameInstance.h"

constexpr int32 ProgressionSnapshotSchemaVersion = 1;
constexpr int32 CurrencyCatalogMaximum = 100000;
constexpr int32 CollectibleCatalogMaximum = 1;
constexpr int32 InitialCoinQuantity = 10;
constexpr int32 GrantedCoinQuantity = 25;
constexpr int32 ExpectedCommittedCoinQuantity = 35;
constexpr int32 ExpectedCollectibleCount = 2;
constexpr int32 FirstQueuePosition = 1;
constexpr int32 SecondQueuePosition = 2;
constexpr int32 ThirdQueuePosition = 3;

struct FSharProgressionRuntimeFixture
{
    UGameInstance* GameInstance = nullptr;
    USharProgressionCatalogSubsystem* CatalogSubsystem = nullptr;
    USharProgressionSubsystem* ProgressionSubsystem = nullptr;
};

inline FSharProgressionOperationDefinition MakeProgressionOperation(
    const FName& OperationId,
    const ESharProgressionValuePolicy ValuePolicy,
    const int32 MaximumQuantity
)
{
    FSharProgressionOperationDefinition Operation;
    Operation.OperationId = OperationId;
    Operation.ValuePolicy = ValuePolicy;
    Operation.MaximumQuantity = MaximumQuantity;
    Operation.bPermanentAllowed = true;
    Operation.bTransientAllowed = false;
    return Operation;
}

inline USharProgressionCatalogDefinition* MakeProgressionCatalogDefinition()
{
    auto* Definition = NewObject<USharProgressionCatalogDefinition>();
    Definition->CanonicalId = FName(TEXT("base_progression"));
    Definition->DisplayName = FText::FromString(TEXT("Base Progression"));
    Definition->SourcePackageIds = {FName(TEXT("progression_contract"))};
    Definition->RevisionToken = TEXT("sha256:progression_definition_v1");
    Definition->ValidationProfile = FName(TEXT("base_progression_v1"));
    Definition->OwningFeature = FName(TEXT("base"));
    Definition->SnapshotSchemaVersion = ProgressionSnapshotSchemaVersion;
    Definition->Operations = {
        MakeProgressionOperation(
            FName(TEXT("grant_currency")),
            ESharProgressionValuePolicy::Additive,
            CurrencyCatalogMaximum
        ),
        MakeProgressionOperation(
            FName(TEXT("unlock_character")),
            ESharProgressionValuePolicy::SetOnce,
            CollectibleCatalogMaximum
        ),
        MakeProgressionOperation(
            FName(TEXT("grant_collectible")),
            ESharProgressionValuePolicy::Additive,
            CollectibleCatalogMaximum
        ),
        MakeProgressionOperation(
            FName(TEXT("set_progression_flag")),
            ESharProgressionValuePolicy::SetOnce,
            CollectibleCatalogMaximum
        ),
    };
    return Definition;
}

inline USharProgressionCatalogSubsystem* MakeProgressionCatalogSubsystem(
    UGameInstance& GameInstance
)
{
    auto* Catalog = NewObject<USharProgressionCatalogSubsystem>(&GameInstance);
    Catalog->ConfigureRevision(TEXT("sha256:progression_catalog_v1"));
    Catalog->RegisterCatalog(MakeProgressionCatalogDefinition());
    Catalog->Activate();
    return Catalog;
}

inline FSharProgressionSnapshot MakeInitialProgressionSnapshot()
{
    FSharProgressionSnapshot Snapshot;
    Snapshot.Profile.ProfileId = FName(TEXT("profile_local_01"));
    Snapshot.Profile.ProfileRevision = TEXT("sha256:profile_v1");
    Snapshot.CatalogId = FName(TEXT("base_progression"));
    Snapshot.CatalogRevision = TEXT("sha256:progression_catalog_v1");
    Snapshot.SaveRevision = TEXT("sha256:save_v1");
    Snapshot.SnapshotRevision = TEXT("sha256:progression_v1");
    Snapshot.SchemaVersion = ProgressionSnapshotSchemaVersion;
    Snapshot.Values = {
        {
            .OperationId = FName(TEXT("grant_currency")),
            .TargetId = FName(TEXT("coins")),
            .Quantity = InitialCoinQuantity,
        },
    };
    Snapshot.AppliedPermanentTransactions = {
        FName(TEXT("initial_currency_import")),
    };
    return Snapshot;
}

inline FSharProgressionRuntimeFixture MakeProgressionRuntime()
{
    FSharProgressionRuntimeFixture Fixture;
    Fixture.GameInstance = NewObject<UGameInstance>();
    Fixture.CatalogSubsystem =
        MakeProgressionCatalogSubsystem(*Fixture.GameInstance);
    Fixture.ProgressionSubsystem = NewObject<USharProgressionSubsystem>(
        Fixture.GameInstance
    );
    Fixture.ProgressionSubsystem->Configure(
        Fixture.CatalogSubsystem,
        MakeInitialProgressionSnapshot()
    );
    return Fixture;
}

inline FSharRewardRequest MakeProgressionOperationRequest(
    const FName& TransactionId,
    const FName& OperationId,
    const FName& TargetId,
    const int32 Quantity
)
{
    FSharRewardRequest Operation;
    Operation.TransactionId = TransactionId;
    Operation.OperationId = OperationId;
    Operation.TargetId = TargetId;
    Operation.Quantity = Quantity;
    Operation.bPermanent = true;
    return Operation;
}

inline FSharProgressionMutationRequest MakeProgressionMutationRequest(
    const FName& MutationId,
    const ESharProgressionMutationPriority Priority,
    const FString& MutationRevision,
    const TArray<FSharRewardRequest>& Operations
)
{
    FSharProgressionMutationRequest Request;
    Request.MutationId = MutationId;
    Request.Priority = Priority;
    Request.Profile.ProfileId = FName(TEXT("profile_local_01"));
    Request.Profile.ProfileRevision = TEXT("sha256:profile_v1");
    Request.CatalogId = FName(TEXT("base_progression"));
    Request.ExpectedCatalogRevision = TEXT("sha256:progression_catalog_v1");
    Request.ExpectedSaveRevision = TEXT("sha256:save_v1");
    Request.ExpectedSnapshotRevision = TEXT("sha256:progression_v1");
    Request.MutationRevision = MutationRevision;
    Request.Operations = Operations;
    return Request;
}

inline FSharProgressionCommitEvidence MakeProgressionCommitEvidence(
    const FSharProgressionMutationRequest& Request,
    const FString& ResultingSaveRevision
)
{
    FSharProgressionCommitEvidence Evidence;
    Evidence.MutationId = Request.MutationId;
    Evidence.Profile = Request.Profile;
    Evidence.CatalogRevision = Request.ExpectedCatalogRevision;
    Evidence.ExpectedSaveRevision = Request.ExpectedSaveRevision;
    Evidence.ExpectedSnapshotRevision = Request.ExpectedSnapshotRevision;
    Evidence.MutationRevision = Request.MutationRevision;
    Evidence.ResultingSaveRevision = ResultingSaveRevision;
    Evidence.ResultingSnapshotRevision = Request.MutationRevision;
    return Evidence;
}

inline FSharProgressionMutationResolution MakeProgressionResolution(
    const FSharProgressionMutationRequest& Request,
    const ESharProgressionResolutionCommand Command
)
{
    FSharProgressionMutationResolution Resolution;
    Resolution.MutationId = Request.MutationId;
    Resolution.Command = Command;
    Resolution.ProfileRevision = Request.Profile.ProfileRevision;
    Resolution.MutationRevision = Request.MutationRevision;
    return Resolution;
}

inline void BeginAndPrepareProgressionMutation(
    USharProgressionSubsystem& Subsystem,
    const FSharProgressionMutationRequest& Request
)
{
    Subsystem.Begin(Request.MutationId);
    Subsystem.Prepare(Request.MutationId);
}
