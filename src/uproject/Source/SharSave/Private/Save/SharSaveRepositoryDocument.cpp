// File: SharSaveRepositoryDocument.cpp
// Path: src/uproject/Source/SharSave/Private/Save/SharSaveRepositoryDocument.cpp
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: portable document descriptor validation, candidate acceptance, and adapter-correlation checks only.
// Specification: docs/technical/unreal/platform-save-storage-and-lifecycle.md
// LARGE-FILE owner=SharSave; reason=cohesive save-document integrity and correlation validation;
// split=extract content requirement resolution when catalog availability ports exist;
// validation=validate.sh SharSave plus Unreal automation; review=2027-01.

#include "Save/SharSaveRepositorySubsystem.h"
#include "Save/SharSaveContracts.h"

#include "Algo/AllOf.h"
#include "Algo/AnyOf.h"
#include "Content/SharPrimaryContentDefinition.h"
#include "Save/SharSaveSchemaCatalogSubsystem.h"
#include "Save/SharSaveSchemaDefinition.h"

static bool IsCanonicalDocumentIdentity(const FName& Candidate)
{
    return USharPrimaryContentDefinition::IsCanonicalIdentifier(Candidate);
}

static bool HasDuplicateDocumentIdentities(const TArray<FName>& Identities)
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

static bool ContainsRequiredSections(
    const FSharSaveDocumentDescriptor& Document,
    const USharSaveSchemaDefinition& Schema
)
{
    return Algo::AllOf(
        Schema.RequiredSectionIds,
        [&Document](const FName& RequiredSectionId)
        {
            return Document.SectionIds.ContainsByPredicate(
                [&RequiredSectionId](const FName& SectionId)
                {
                    return SectionId == RequiredSectionId;
                }
            );
        }
    );
}

bool USharSaveRepositorySubsystem::ValidateDocument(
    const FSharSaveDocumentDescriptor& Document,
    const USharSaveSchemaDefinition& Schema,
    const bool bRequireCurrentVersion
) const
{
    const bool bInvalidIdentity =
        Document.SchemaId != Schema.CanonicalId
        || !IsRevisionToken(Document.DocumentRevision)
        || !IsRevisionToken(Document.CatalogRevision)
        || !IsRevisionToken(Document.SnapshotRevision)
        || !IsRevisionToken(Document.IntegrityRevision);
    const bool bInvalidIntegrity =
        Document.SerializedLength <= 0
        || Document.CatalogRevision != SchemaCatalog->GetCatalogRevision();
    const bool bInvalidCollections =
        Document.ContentRequirementIds.Num()
            > Schema.MaximumContentRequirements
        || HasDuplicateDocumentIdentities(Document.ContentRequirementIds)
        || HasDuplicateDocumentIdentities(Document.SectionIds)
        || !Algo::AllOf(
            Document.ContentRequirementIds,
            [](const FName& ContentId)
            {
                return IsCanonicalDocumentIdentity(ContentId);
            }
        )
        || !Algo::AllOf(
            Document.SectionIds,
            [](const FName& SectionId)
            {
                return IsCanonicalDocumentIdentity(SectionId);
            }
        )
        || !ContainsRequiredSections(Document, Schema);
    const bool bInvalidVersion = bRequireCurrentVersion
        ? Document.SchemaVersion != Schema.CurrentSchemaVersion
        : !Schema.CanMigrateFrom(Document.SchemaVersion);
    return !bInvalidIdentity
        && !bInvalidIntegrity
        && !bInvalidCollections
        && !bInvalidVersion;
}

bool USharSaveRepositorySubsystem::EvidenceMatches(
    const FSharSaveOperationSnapshot& Snapshot,
    const FSharSaveAdapterEvidence& Evidence
)
{
    return Snapshot.Request.OperationId == Evidence.OperationId
        && SlotIdsMatch(Snapshot.Request.Slot, Evidence.Slot)
        && Snapshot.Request.OperationRevision == Evidence.OperationRevision
        && Snapshot.Request.ContainerRevision == Evidence.ContainerRevision
        && Snapshot.Request.ExpectedAcceptedRevision
            == Evidence.ExpectedAcceptedRevision;
}

ESharSaveOperationResult USharSaveRepositorySubsystem::AcceptCandidate(
    const FName& OperationId,
    const FSharSaveDocumentDescriptor& Candidate
)
{
    FSharSaveOperationSnapshot* Snapshot = FindOperation(OperationId);
    if (Snapshot == nullptr)
    {
        return ESharSaveOperationResult::NotFound;
    }
    if (Snapshot->bReleased)
    {
        return ESharSaveOperationResult::Released;
    }
    if (IsTerminalState(Snapshot->State))
    {
        return ESharSaveOperationResult::AlreadyTerminal;
    }
    if (Snapshot->Request.Kind != ESharSaveOperationKind::Save
        || Snapshot->State != ESharSaveOperationState::Preparing)
    {
        return ESharSaveOperationResult::InvalidState;
    }
    const USharSaveSchemaDefinition* Schema =
        SchemaCatalog->FindSchema(Snapshot->Request.SchemaId);
    if (Schema == nullptr)
    {
        return ESharSaveOperationResult::SchemaMissing;
    }
    if (!ValidateDocument(Candidate, *Schema, true)
        || Candidate.SchemaId != Snapshot->Request.SchemaId)
    {
        return ESharSaveOperationResult::IntegrityMismatch;
    }
    Snapshot->CandidateDocument = Candidate;
    Snapshot->bCandidateAccepted = true;
    Snapshot->State = ESharSaveOperationState::Writing;
    return ESharSaveOperationResult::Accepted;
}
