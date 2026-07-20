// File: SharSaveSchemaDefinition.cpp
// Path: src/uproject/Source/SharSave/Private/Save/SharSaveSchemaDefinition.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: load-free portable save schema and migration-chain validation only.
// Specification: docs/technical/unreal/platform-save-storage-and-lifecycle.md
// LARGE-FILE owner=SharSave; reason=cohesive schema and migration-chain validation;
// split=extract migration diagnostics if plans become persistent artifacts;
// validation=validate.sh SharSave plus Unreal automation; review=2027-01.

#include "Save/SharSaveSchemaDefinition.h"

#include "Algo/AnyOf.h"
#include "Content/SharPrimaryContentDefinition.h"
#include "Engine/DataAsset.h"

static bool IsCanonicalSaveIdentity(const FName& Candidate)
{
    return USharPrimaryContentDefinition::IsCanonicalIdentifier(Candidate);
}

static bool IsRevisionToken(const FString& Revision)
{
    return Revision.StartsWith(TEXT("sha256:"));
}

static void AddSchemaError(TArray<FText>& OutErrors, const TCHAR* Message)
{
    OutErrors.Add(FText::FromString(Message));
}

static bool HasDuplicateNames(const TArray<FName>& Identities)
{
    return Algo::AnyOf(
        Identities,
        [&Identities](const FName& Candidate)
        {
            int32 MatchCount = 0;
            for (const FName& Identity : Identities)
            {
                MatchCount += Identity == Candidate ? 1 : 0;
            }
            return MatchCount > 1;
        }
    );
}

static bool HasDuplicateMigrationSources(
    const TArray<FSharSaveMigrationStep>& Steps
)
{
    return Algo::AnyOf(
        Steps,
        [&Steps](const FSharSaveMigrationStep& Candidate)
        {
            int32 MatchCount = 0;
            for (const FSharSaveMigrationStep& Step : Steps)
            {
                MatchCount += Step.SourceVersion == Candidate.SourceVersion
                    ? 1
                    : 0;
            }
            return MatchCount > 1;
        }
    );
}

static const FSharSaveMigrationStep* FindMigrationStep(
    const TArray<FSharSaveMigrationStep>& Steps,
    const int32 SourceVersion
)
{
    for (const FSharSaveMigrationStep& Step : Steps)
    {
        if (Step.SourceVersion == SourceVersion)
        {
            return &Step;
        }
    }
    return nullptr;
}

static bool IsValidMigrationStep(const FSharSaveMigrationStep& Step)
{
    return Step.SourceVersion > 0
        && Step.TargetVersion == Step.SourceVersion + 1
        && IsCanonicalSaveIdentity(Step.MigrationId)
        && IsRevisionToken(Step.MigrationRevision)
        && Step.bIdempotent;
}

static void AppendSaveSchemaScalarErrors(
    const USharSaveSchemaDefinition& Definition,
    TArray<FText>& OutErrors
)
{
    if (Definition.CurrentSchemaVersion <= 0
        || Definition.MaximumContentRequirements <= 0)
    {
        AddSchemaError(
            OutErrors,
            TEXT("Save schema version and content-requirement bound must be positive.")
        );
    }
}

static void AppendSaveSchemaSectionErrors(
    const USharSaveSchemaDefinition& Definition,
    TArray<FText>& OutErrors
)
{
    if (Definition.RequiredSectionIds.IsEmpty()
        || HasDuplicateNames(Definition.RequiredSectionIds))
    {
        AddSchemaError(
            OutErrors,
            TEXT("Save schema requires unique canonical section identities.")
        );
    }
    for (const FName& SectionId : Definition.RequiredSectionIds)
    {
        if (!IsCanonicalSaveIdentity(SectionId))
        {
            AddSchemaError(
                OutErrors,
                TEXT("Save schema section identities must be canonical.")
            );
        }
    }
}

static void AppendSaveSchemaMigrationErrors(
    const USharSaveSchemaDefinition& Definition,
    TArray<FText>& OutErrors
)
{
    if (HasDuplicateMigrationSources(Definition.MigrationSteps))
    {
        AddSchemaError(
            OutErrors,
            TEXT("Save schema migration source versions must be unique.")
        );
    }
    for (const FSharSaveMigrationStep& Step : Definition.MigrationSteps)
    {
        if (!IsValidMigrationStep(Step)
            || Step.TargetVersion > Definition.CurrentSchemaVersion)
        {
            AddSchemaError(
                OutErrors,
                TEXT("Save migration steps must be idempotent consecutive canonical revisions.")
            );
        }
    }
    if (!Definition.CanMigrateFrom(1))
    {
        AddSchemaError(
            OutErrors,
            TEXT("Save schema must provide a complete migration chain from version one.")
        );
    }
}

void USharSaveSchemaDefinition::GatherValidationErrors(
    TArray<FText>& OutErrors
) const
{
    USharPrimaryContentDefinition::GatherValidationErrors(OutErrors);
    AppendSaveSchemaScalarErrors(*this, OutErrors);
    AppendSaveSchemaSectionErrors(*this, OutErrors);
    AppendSaveSchemaMigrationErrors(*this, OutErrors);
}

bool USharSaveSchemaDefinition::BuildMigrationPlan(
    const int32 SourceVersion,
    TArray<FName>& OutMigrationIds
) const
{
    OutMigrationIds.Reset();
    if (SourceVersion <= 0 || SourceVersion > CurrentSchemaVersion)
    {
        return false;
    }
    int32 Version = SourceVersion;
    while (Version < CurrentSchemaVersion)
    {
        const FSharSaveMigrationStep* Step =
            FindMigrationStep(MigrationSteps, Version);
        if (Step == nullptr || !IsValidMigrationStep(*Step))
        {
            OutMigrationIds.Reset();
            return false;
        }
        OutMigrationIds.Add(Step->MigrationId);
        Version = Step->TargetVersion;
    }
    return Version == CurrentSchemaVersion;
}

bool USharSaveSchemaDefinition::CanMigrateFrom(
    const int32 SourceVersion
) const
{
    TArray<FName> MigrationIds;
    return BuildMigrationPlan(SourceVersion, MigrationIds);
}

FPrimaryAssetType USharSaveSchemaDefinition::GetDefinitionAssetType() const
{
    return {TEXT("SharSaveSchema")};
}
