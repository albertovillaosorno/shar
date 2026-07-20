// File: SharSaveTestFixtures.h
// Path: src/uproject/Source/SharSave/Private/Tests/SharSaveTestFixtures.h
// Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier: MIT
// Boundary: explicit transient save schema, catalog, slot, operation, document, and adapter evidence fixtures only.
// Specification: docs/technical/unreal/platform-save-storage-and-lifecycle.md
// LARGE-FILE owner=SharSave; reason=cohesive typed portable-save test fixtures;
// split=extract adapter fixtures if provider remediation becomes implemented;
// validation=validate.sh SharSave plus Unreal automation; review=2027-01.

#pragma once

#include "Save/SharSaveContracts.h"
#include "Save/SharSaveRepositorySubsystem.h"
#include "Save/SharSaveSchemaCatalogSubsystem.h"
#include "Save/SharSaveSchemaDefinition.h"

#include "Engine/GameInstance.h"

constexpr double DefaultSaveDeadlineSeconds = 30.0;
constexpr int64 DefaultSerializedSaveLength = 4096;
constexpr int32 InitialSaveSchemaVersion = 1;
constexpr int32 IntermediateSaveSchemaVersion = 2;
constexpr int32 CurrentSaveSchemaVersion = 3;
constexpr int32 FutureSaveSchemaVersion = 4;
constexpr int32 FixtureMaximumContentRequirements = 8;

struct FSharSaveRuntimeFixture
{
    UGameInstance* GameInstance = nullptr;
    USharSaveSchemaCatalogSubsystem* Catalog = nullptr;
    USharSaveRepositorySubsystem* Repository = nullptr;
};

struct FSharSaveRequestFixture
{
    FName OperationId;
    ESharSaveOperationKind Kind = ESharSaveOperationKind::Save;
    ESharSaveOperationPriority Priority = ESharSaveOperationPriority::Manual;
    FSharSaveSlotId Slot;
};

struct FSharSaveEvidenceFixture
{
    FName OperationId;
    FSharSaveSlotId Slot;
    ESharSaveAdapterStage Stage = ESharSaveAdapterStage::CandidateWritten;
    FSharSaveDocumentDescriptor Document;
    FString ResultingAcceptedRevision;
};

inline void FillSaveDefinitionBase(USharSaveSchemaDefinition& Definition)
{
    Definition.CanonicalId = FName(TEXT("portable_v1"));
    Definition.DisplayName = FText::FromString(TEXT("Portable Save V1"));
    Definition.SourcePackageIds = {FName(TEXT("portable_save_contract"))};
    Definition.RevisionToken = TEXT("sha256:save_schema_v3");
    Definition.ValidationProfile = FName(TEXT("portable_save_v1"));
    Definition.OwningFeature = FName(TEXT("base"));
    Definition.CurrentSchemaVersion = CurrentSaveSchemaVersion;
    Definition.RequiredSectionIds = {
        FName(TEXT("integrity")),
        FName(TEXT("progression")),
        FName(TEXT("resume")),
    };
    Definition.MaximumContentRequirements = FixtureMaximumContentRequirements;
    Definition.MigrationSteps = {
        {
            .SourceVersion = InitialSaveSchemaVersion,
            .TargetVersion = IntermediateSaveSchemaVersion,
            .MigrationId = FName(TEXT("save_v1_to_v2")),
            .MigrationRevision = TEXT("sha256:migration_v1_to_v2"),
            .bIdempotent = true,
        },
        {
            .SourceVersion = IntermediateSaveSchemaVersion,
            .TargetVersion = CurrentSaveSchemaVersion,
            .MigrationId = FName(TEXT("save_v2_to_v3")),
            .MigrationRevision = TEXT("sha256:migration_v2_to_v3"),
            .bIdempotent = true,
        },
    };
}

inline USharSaveSchemaDefinition* MakeSaveSchema()
{
    auto* Definition = NewObject<USharSaveSchemaDefinition>();
    FillSaveDefinitionBase(*Definition);
    return Definition;
}

inline USharSaveSchemaCatalogSubsystem* MakeSaveCatalog(
    UGameInstance& GameInstance
)
{
    auto* Catalog = NewObject<USharSaveSchemaCatalogSubsystem>(&GameInstance);
    Catalog->ConfigureRevision(TEXT("sha256:save_catalog_v1"));
    Catalog->RegisterSchema(MakeSaveSchema());
    Catalog->Activate();
    return Catalog;
}

inline FSharSaveSlotId MakeSaveSlotId(const FName& SlotId)
{
    return {
        .ProfileId = FName(TEXT("profile_local_01")),
        .SlotId = SlotId,
    };
}

inline FSharSaveSlotState MakeSaveSlotState(
    const FSharSaveSlotId& Slot,
    const bool bOccupied
)
{
    FSharSaveSlotState State;
    State.Slot = Slot;
    State.AcceptedRevision = bOccupied
        ? TEXT("sha256:accepted_v1")
        : TEXT("sha256:empty_v1");
    State.ContainerRevision = TEXT("sha256:container_v1");
    State.SchemaId = bOccupied ? FName(TEXT("portable_v1")) : FName();
    State.SchemaVersion = bOccupied ? CurrentSaveSchemaVersion : 0;
    State.IntegrityRevision = bOccupied
        ? TEXT("sha256:integrity_v1")
        : FString();
    State.bOccupied = bOccupied;
    return State;
}

inline FSharSaveRuntimeFixture MakeSaveRuntime()
{
    FSharSaveRuntimeFixture Fixture;
    Fixture.GameInstance = NewObject<UGameInstance>();
    Fixture.Catalog = MakeSaveCatalog(*Fixture.GameInstance);
    Fixture.Repository = NewObject<USharSaveRepositorySubsystem>(
        Fixture.GameInstance
    );
    Fixture.Repository->Configure(
        Fixture.Catalog,
        FName(TEXT("desktop_provider")),
        TEXT("sha256:container_v1")
    );
    Fixture.Repository->RegisterSlot(
        MakeSaveSlotState(MakeSaveSlotId(FName(TEXT("slot_a"))), true)
    );
    Fixture.Repository->RegisterSlot(
        MakeSaveSlotState(MakeSaveSlotId(FName(TEXT("slot_b"))), false)
    );
    return Fixture;
}

inline FSharSaveOperationRequest MakeSaveRequest(
    const FSharSaveRequestFixture& Fixture,
    const FString& ExpectedAcceptedRevision
)
{
    FSharSaveOperationRequest Request;
    Request.OperationId = Fixture.OperationId;
    Request.Kind = Fixture.Kind;
    Request.Priority = Fixture.Priority;
    Request.Slot = Fixture.Slot;
    Request.SchemaId = FName(TEXT("portable_v1"));
    Request.ProviderId = FName(TEXT("desktop_provider"));
    Request.ExpectedAcceptedRevision = ExpectedAcceptedRevision;
    Request.CatalogRevision = TEXT("sha256:save_catalog_v1");
    Request.ContainerRevision = TEXT("sha256:container_v1");
    Request.OperationRevision = TEXT("sha256:operation_v1");
    Request.DeadlineSeconds = DefaultSaveDeadlineSeconds;
    return Request;
}

inline FSharSaveDocumentDescriptor MakeSaveDocument(
    const FString& DocumentRevision,
    const int32 SchemaVersion
)
{
    FSharSaveDocumentDescriptor Document;
    Document.SchemaId = FName(TEXT("portable_v1"));
    Document.SchemaVersion = SchemaVersion;
    Document.DocumentRevision = DocumentRevision;
    Document.CatalogRevision = TEXT("sha256:save_catalog_v1");
    Document.SnapshotRevision = TEXT("sha256:domain_snapshot_v1");
    Document.ContentRequirementIds = {
        FName(TEXT("base_content")),
        FName(TEXT("chapter_01")),
    };
    Document.SectionIds = {
        FName(TEXT("integrity")),
        FName(TEXT("progression")),
        FName(TEXT("resume")),
    };
    Document.SerializedLength = DefaultSerializedSaveLength;
    Document.IntegrityRevision = TEXT("sha256:integrity_candidate_v1");
    return Document;
}

inline FSharSaveAdapterEvidence MakeSaveEvidence(
    const FSharSaveEvidenceFixture& Fixture
)
{
    FSharSaveAdapterEvidence Evidence;
    Evidence.OperationId = Fixture.OperationId;
    Evidence.Slot = Fixture.Slot;
    Evidence.Stage = Fixture.Stage;
    Evidence.OperationRevision = TEXT("sha256:operation_v1");
    Evidence.ContainerRevision = TEXT("sha256:container_v1");
    Evidence.ExpectedAcceptedRevision = Fixture.Slot.SlotId
            == FName(TEXT("slot_b"))
        ? TEXT("sha256:empty_v1")
        : TEXT("sha256:accepted_v1");
    Evidence.ResultingAcceptedRevision = Fixture.ResultingAcceptedRevision;
    Evidence.Document = Fixture.Document;
    return Evidence;
}

inline FSharSaveOperationResolution MakeSaveResolution(
    const FName& OperationId,
    const ESharSaveResolutionCommand Command
)
{
    FSharSaveOperationResolution Resolution;
    Resolution.OperationId = OperationId;
    Resolution.Command = Command;
    Resolution.OperationRevision = TEXT("sha256:operation_v1");
    Resolution.ContainerRevision = TEXT("sha256:container_v1");
    return Resolution;
}

inline void PrepareSaveCandidate(
    USharSaveRepositorySubsystem& Repository,
    const FSharSaveOperationRequest& Request,
    const FSharSaveDocumentDescriptor& Document
)
{
    Repository.Begin(Request.OperationId);
    Repository.AcceptCandidate(Request.OperationId, Document);
}

inline void CompleteSaveTransaction(
    USharSaveRepositorySubsystem& Repository,
    const FSharSaveOperationRequest& Request,
    const FSharSaveDocumentDescriptor& Document
)
{
    PrepareSaveCandidate(Repository, Request, Document);
    Repository.AcceptAdapterEvidence(MakeSaveEvidence({
        .OperationId = Request.OperationId,
        .Slot = Request.Slot,
        .Stage = ESharSaveAdapterStage::CandidateWritten,
        .Document = Document,
        .ResultingAcceptedRevision = FString(),
    }));
    Repository.AcceptAdapterEvidence(MakeSaveEvidence({
        .OperationId = Request.OperationId,
        .Slot = Request.Slot,
        .Stage = ESharSaveAdapterStage::DurableFlushCompleted,
        .Document = Document,
        .ResultingAcceptedRevision = FString(),
    }));
    Repository.AcceptAdapterEvidence(MakeSaveEvidence({
        .OperationId = Request.OperationId,
        .Slot = Request.Slot,
        .Stage = ESharSaveAdapterStage::ReadBackValidated,
        .Document = Document,
        .ResultingAcceptedRevision = FString(),
    }));
    Repository.AcceptAdapterEvidence(MakeSaveEvidence({
        .OperationId = Request.OperationId,
        .Slot = Request.Slot,
        .Stage = ESharSaveAdapterStage::AtomicReplaceCompleted,
        .Document = Document,
        .ResultingAcceptedRevision = Document.DocumentRevision,
    }));
}
