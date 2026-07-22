// File: SharWorldDefinition.cpp
// Path: src/uproject/Source/SharWorld/Private/World/SharWorldDefinition.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: load-free connected-world, region, and Data Layer validation only.
// ADR: docs/adr/unreal/architecture/aaa-native-content-and-gameplay-foundation.md
// LARGE-FILE owner=SharWorld; reason=cohesive world-definition validation;
// split=extract graph validation if more world-definition families appear;
// validation=validate.sh SharWorld plus Unreal automation; review=2027-01.

#include "World/SharWorldDefinition.h"

#include "Algo/AllOf.h"
#include "Algo/Find.h"
#include "Content/SharPrimaryContentDefinition.h"
#include "Engine/DataAsset.h"

static void AddWorldError(TArray<FText>& OutErrors, const TCHAR* Message)
{
    OutErrors.Add(FText::FromString(Message));
}

static const FSharDataLayerDefinition* FindLayer(
    const TArray<FSharDataLayerDefinition>& Layers,
    const FName& LayerId
)
{
    return Algo::FindByPredicate(
        Layers,
        [&LayerId](const FSharDataLayerDefinition& Layer)
        {
            return Layer.LayerId == LayerId;
        }
    );
}

static void AppendRegionErrors(
    const TArray<FSharWorldRegionDefinition>& Regions,
    TArray<FText>& OutErrors
)
{
    TSet<FName> SeenRegionIds;
    for (const FSharWorldRegionDefinition& Region : Regions)
    {
        const bool bInvalidIdentity =
            !USharPrimaryContentDefinition::IsCanonicalIdentifier(
                Region.RegionId
            )
            || !USharPrimaryContentDefinition::IsCanonicalIdentifier(
                Region.RuntimeGridId
            )
            || !USharPrimaryContentDefinition::IsCanonicalIdentifier(
                Region.HlodProfileId
            );
        if (bInvalidIdentity)
        {
            AddWorldError(
                OutErrors,
                TEXT("World regions require canonical region, grid, and HLOD identities.")
            );
        }
        if (SeenRegionIds.Contains(Region.RegionId))
        {
            AddWorldError(
                OutErrors,
                TEXT("World region identities must be unique.")
            );
        }
        SeenRegionIds.Add(Region.RegionId);
    }
}

static void AppendLayerShapeErrors(
    const FSharDataLayerDefinition& Layer,
    const TArray<FSharDataLayerDefinition>& Layers,
    TArray<FText>& OutErrors
)
{
    if (!USharPrimaryContentDefinition::IsCanonicalIdentifier(Layer.LayerId))
    {
        AddWorldError(
            OutErrors,
            TEXT("Data Layer identities must be canonical.")
        );
    }

    TSet<FName> SeenRequirements;
    for (const FName& RequiredLayerId : Layer.RequiredLayerIds)
    {
        const bool bInvalidRequirement =
            !USharPrimaryContentDefinition::IsCanonicalIdentifier(
                RequiredLayerId
            )
            || RequiredLayerId == Layer.LayerId
            || FindLayer(Layers, RequiredLayerId) == nullptr;
        if (bInvalidRequirement)
        {
            AddWorldError(
                OutErrors,
                TEXT("Data Layer prerequisites must reference another declared canonical layer.")
            );
        }
        if (SeenRequirements.Contains(RequiredLayerId))
        {
            AddWorldError(
                OutErrors,
                TEXT("Data Layer prerequisites must be unique.")
            );
        }
        SeenRequirements.Add(RequiredLayerId);
    }
}

static bool RequirementsResolved(
    const FSharDataLayerDefinition& Layer,
    const TSet<FName>& ResolvedLayerIds
)
{
    return Algo::AllOf(
        Layer.RequiredLayerIds,
        [&ResolvedLayerIds](const FName& RequiredLayerId)
        {
            return ResolvedLayerIds.Contains(RequiredLayerId);
        }
    );
}

static bool ResolveReadyLayers(
    const TArray<FSharDataLayerDefinition>& Layers,
    TSet<FName>& ResolvedLayerIds
)
{
    bool bChanged = false;
    for (const FSharDataLayerDefinition& Layer : Layers)
    {
        if (ResolvedLayerIds.Contains(Layer.LayerId)
            || !RequirementsResolved(Layer, ResolvedLayerIds))
        {
            continue;
        }
        ResolvedLayerIds.Add(Layer.LayerId);
        bChanged = true;
    }
    return bChanged;
}

static void AppendLayerGraphErrors(
    const TArray<FSharDataLayerDefinition>& Layers,
    TArray<FText>& OutErrors
)
{
    TSet<FName> ResolvedLayerIds;
    while (ResolveReadyLayers(Layers, ResolvedLayerIds))
    {
    }
    for (const FSharDataLayerDefinition& Layer : Layers)
    {
        if (!ResolvedLayerIds.Contains(Layer.LayerId))
        {
            AddWorldError(
                OutErrors,
                TEXT("Data Layer prerequisite graph contains a cycle.")
            );
            return;
        }
    }
}

static void AppendLayerErrors(
    const TArray<FSharDataLayerDefinition>& Layers,
    TArray<FText>& OutErrors
)
{
    TSet<FName> SeenLayerIds;
    for (const FSharDataLayerDefinition& Layer : Layers)
    {
        if (SeenLayerIds.Contains(Layer.LayerId))
        {
            AddWorldError(
                OutErrors,
                TEXT("Data Layer identities must be unique.")
            );
        }
        SeenLayerIds.Add(Layer.LayerId);
        AppendLayerShapeErrors(Layer, Layers, OutErrors);
    }
    AppendLayerGraphErrors(Layers, OutErrors);
}

void USharWorldDefinition::GatherValidationErrors(
    TArray<FText>& OutErrors
) const
{
    USharPrimaryContentDefinition::GatherValidationErrors(OutErrors);
    Orientation.GatherValidationErrors(OutErrors);
    if (Regions.IsEmpty())
    {
        AddWorldError(
            OutErrors,
            TEXT("Connected world definitions require at least one region.")
        );
    }
    if (DataLayers.IsEmpty())
    {
        AddWorldError(
            OutErrors,
            TEXT("Connected world definitions require at least one Data Layer.")
        );
    }
    if (!FMath::IsFinite(DayLengthSeconds) || DayLengthSeconds <= 0.0F)
    {
        AddWorldError(
            OutErrors,
            TEXT("World day length must be finite and positive.")
        );
    }
    if (!bUsesWorldPartition)
    {
        AddWorldError(
            OutErrors,
            TEXT("The canonical connected world requires World Partition.")
        );
    }
    AppendRegionErrors(Regions, OutErrors);
    AppendLayerErrors(DataLayers, OutErrors);
}

FPrimaryAssetType USharWorldDefinition::GetDefinitionAssetType() const
{
    return {TEXT("SharWorld")};
}
