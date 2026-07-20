// File: SharSpatialPlacementDefinition.cpp
// Path: src/uproject/Source/SharWorld/Private/Spatial/SharSpatialPlacementDefinition.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: load-free authored placement and volume validation only; no overlap queries or domain mutation.
// ADR: docs/adr/unreal/runtime/contextual-interaction-query-and-transaction.md
// LARGE-FILE owner=SharWorld; reason=cohesive spatial definition validation;
// split=extract shape validation if convex or spline volumes are introduced;
// validation=validate.sh SharWorld plus Unreal automation; review=2027-01.

#include "Spatial/SharSpatialPlacementDefinition.h"

#include "Content/SharPrimaryContentDefinition.h"

static void AddSpatialDefinitionError(
    TArray<FText>& OutErrors,
    const TCHAR* Message
)
{
    OutErrors.Add(FText::FromString(Message));
}

static bool IsCanonicalSpatialId(const FName& Candidate)
{
    return USharPrimaryContentDefinition::IsCanonicalIdentifier(Candidate);
}

static bool TransformIsFinite(
    const FSharSpatialTransformDefinition& Transform
)
{
    return !Transform.Location.ContainsNaN()
        && !Transform.RotationEulerDegrees.ContainsNaN()
        && !Transform.Scale.ContainsNaN();
}

static bool TransformHasPositiveScale(
    const FSharSpatialTransformDefinition& Transform
)
{
    return Transform.Scale.GetMin() > 0.0;
}

static void AppendTransformErrors(
    const FSharSpatialTransformDefinition& Transform,
    TArray<FText>& OutErrors
)
{
    if (!TransformIsFinite(Transform))
    {
        AddSpatialDefinitionError(
            OutErrors,
            TEXT("Spatial transforms must contain only finite values.")
        );
    }
    if (!TransformHasPositiveScale(Transform))
    {
        AddSpatialDefinitionError(
            OutErrors,
            TEXT("Spatial transform scale must remain positive and invertible.")
        );
    }
}

static bool SphereDimensionsAreValid(const FVector& Dimensions)
{
    return !Dimensions.ContainsNaN() && Dimensions.X > 0.0;
}

static bool BoxDimensionsAreValid(const FVector& Dimensions)
{
    return !Dimensions.ContainsNaN() && Dimensions.GetMin() > 0.0;
}

static bool CapsuleDimensionsAreValid(const FVector& Dimensions)
{
    return !Dimensions.ContainsNaN()
        && Dimensions.X > 0.0
        && Dimensions.Z >= Dimensions.X;
}

static bool VolumeDimensionsAreValid(
    const FSharSpatialVolumeDefinition& Volume
)
{
    switch (Volume.Shape)
    {
    case ESharSpatialVolumeShape::Point:
        return !Volume.Dimensions.ContainsNaN();
    case ESharSpatialVolumeShape::Sphere:
        return SphereDimensionsAreValid(Volume.Dimensions);
    case ESharSpatialVolumeShape::OrientedBox:
        return BoxDimensionsAreValid(Volume.Dimensions);
    case ESharSpatialVolumeShape::Capsule:
        return CapsuleDimensionsAreValid(Volume.Dimensions);
    default:
        return false;
    }
}

static bool VolumeTimingIsValid(
    const FSharSpatialVolumeDefinition& Volume
)
{
    return FMath::IsFinite(Volume.BoundaryToleranceCentimeters)
        && Volume.BoundaryToleranceCentimeters >= 0.0F
        && FMath::IsFinite(Volume.HysteresisCentimeters)
        && Volume.HysteresisCentimeters >= 0.0F
        && FMath::IsFinite(Volume.DwellSeconds)
        && Volume.DwellSeconds >= 0.0F
        && FMath::IsFinite(Volume.CooldownSeconds)
        && Volume.CooldownSeconds >= 0.0F;
}

static void AppendVolumeErrors(
    const FSharSpatialVolumeDefinition& Volume,
    TArray<FText>& OutErrors
)
{
    const bool bInvalidIdentity =
        !IsCanonicalSpatialId(Volume.VolumeId)
        || !IsCanonicalSpatialId(Volume.QueryChannelId)
        || !IsCanonicalSpatialId(Volume.ParticipantFilterId)
        || !IsCanonicalSpatialId(Volume.ObservationPolicyId);
    if (bInvalidIdentity)
    {
        AddSpatialDefinitionError(
            OutErrors,
            TEXT("Spatial volume and policy identities must be canonical.")
        );
    }
    AppendTransformErrors(Volume.LocalTransform, OutErrors);
    if (!VolumeDimensionsAreValid(Volume))
    {
        AddSpatialDefinitionError(
            OutErrors,
            TEXT("Spatial volume dimensions must be finite and valid for the declared shape.")
        );
    }
    if (!VolumeTimingIsValid(Volume))
    {
        AddSpatialDefinitionError(
            OutErrors,
            TEXT("Spatial observation tolerances and timing must be finite and non-negative.")
        );
    }
    if (Volume.Priority < 0)
    {
        AddSpatialDefinitionError(
            OutErrors,
            TEXT("Spatial volume priority cannot be negative.")
        );
    }
}

static void AppendAllVolumeErrors(
    const TArray<FSharSpatialVolumeDefinition>& Volumes,
    TArray<FText>& OutErrors
)
{
    if (Volumes.IsEmpty())
    {
        AddSpatialDefinitionError(
            OutErrors,
            TEXT("Spatial placement requires at least one ordered volume.")
        );
    }
    TSet<FName> SeenVolumeIds;
    for (const FSharSpatialVolumeDefinition& Volume : Volumes)
    {
        AppendVolumeErrors(Volume, OutErrors);
        if (SeenVolumeIds.Contains(Volume.VolumeId))
        {
            AddSpatialDefinitionError(
                OutErrors,
                TEXT("Spatial volume identities must be unique per placement.")
            );
        }
        SeenVolumeIds.Add(Volume.VolumeId);
    }
}

namespace
{
struct FSharNameListValidationMessages
{
    const TCHAR* Missing;
    const TCHAR* Invalid;
    const TCHAR* Duplicate;
};
} // namespace

static void AppendNameListErrors(
    const TArray<FName>& Values,
    const bool bRequired,
    const FSharNameListValidationMessages& Messages,
    TArray<FText>& OutErrors
)
{
    if (bRequired && Values.IsEmpty())
    {
        AddSpatialDefinitionError(OutErrors, Messages.Missing);
    }
    TSet<FName> Seen;
    for (const FName& Value : Values)
    {
        if (!IsCanonicalSpatialId(Value))
        {
            AddSpatialDefinitionError(OutErrors, Messages.Invalid);
        }
        if (Seen.Contains(Value))
        {
            AddSpatialDefinitionError(OutErrors, Messages.Duplicate);
        }
        Seen.Add(Value);
    }
}

void USharSpatialPlacementDefinition::GatherValidationErrors(
    TArray<FText>& OutErrors
) const
{
    const bool bInvalidIdentity =
        !IsCanonicalSpatialId(PlacementId)
        || !IsCanonicalSpatialId(OwnerId)
        || !IsCanonicalSpatialId(ActivationPredicateId)
        || !IsCanonicalSpatialId(ParticipantFilterId)
        || !IsCanonicalSpatialId(ObservationPolicyId);
    if (bInvalidIdentity)
    {
        AddSpatialDefinitionError(
            OutErrors,
            TEXT("Spatial placement and policy identities must be canonical.")
        );
    }
    if (!RevisionToken.StartsWith(TEXT("sha256:")))
    {
        AddSpatialDefinitionError(
            OutErrors,
            TEXT("Spatial placement revision requires SHA-256 identity.")
        );
    }
    AppendTransformErrors(Transform, OutErrors);
    AppendAllVolumeErrors(Volumes, OutErrors);
    AppendNameListErrors(
        RoleIds,
        true,
        {
            .Missing = TEXT("Spatial placement requires at least one semantic role."),
            .Invalid = TEXT("Spatial role identities must be canonical."),
            .Duplicate = TEXT("Spatial role identities must be unique."),
        },
        OutErrors
    );
    AppendNameListErrors(
        DataLayerIds,
        false,
        {
            .Missing = TEXT(""),
            .Invalid = TEXT("Spatial Data Layer identities must be canonical."),
            .Duplicate = TEXT("Spatial Data Layer identities must be unique."),
        },
        OutErrors
    );
    AppendNameListErrors(
        BundleIds,
        false,
        {
            .Missing = TEXT(""),
            .Invalid = TEXT("Spatial bundle identities must be canonical."),
            .Duplicate = TEXT("Spatial bundle identities must be unique."),
        },
        OutErrors
    );
    AppendNameListErrors(
        SourceAliases,
        false,
        {
            .Missing = TEXT(""),
            .Invalid = TEXT("Spatial source aliases must be canonical."),
            .Duplicate = TEXT("Spatial source aliases must be unique."),
        },
        OutErrors
    );
}
