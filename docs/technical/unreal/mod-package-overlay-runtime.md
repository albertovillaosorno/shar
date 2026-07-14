# Mod package overlay runtime

- Status: Active
- Last reviewed: 2026-07-14

## Governing decisions

- [Validated game-feature mod overlays](../../adr/unreal/runtime/validated-game-feature-mod-overlays.md)
- [Local drop-in mod packages and AI skills](../../adr/modding/drop-in-mod-packages-and-ai-skills.md)
- [Local mod trust and distribution boundary](../../adr/modding/mod-safety-scanner-and-distribution.md)
- [Shared runtime tagging, modding, and platform compatibility](../../adr/unreal/runtime/shared-runtime-tagging-modding-and-platform-compatibility.md)

## Purpose

This specification defines how an already normalized and validated local package
becomes a native Unreal overlay without loose file replacement, editor scanning,
partial activation, or implicit native-code trust.

## Ownership

`USharModOverlaySubsystem` is a game-instance subsystem. It receives immutable
validated package records and candidate active-set plans through application
ports. It does not parse transport archives, decide trust, repair declarations,
or discover packages from storage.

The subsystem owns:

- native projection planning;
- portable data-overlay registration;
- target-cooked asset-overlay mounting;
- Game Feature lifecycle coordination;
- catalog and Data Registry overlay revisions;
- World Partition content registration;
- read-back verification;
- atomic active-revision replacement; and
- rollback to the prior accepted overlay graph.

The package validator remains authority over package identity, dependency order,
conflicts, supersession, target compatibility, and admission.

## Overlay classes

An accepted package declares one of three native projection classes.

| Class | Runtime material | Portability |
| :--- | :--- | :--- |
| `data_overlay` | Structured semantic rows, tags, localization, tuning, definitions, and references. | Portable when every required capability resolves. |
| `cooked_asset_overlay` | Target-cooked Unreal assets and Game Feature content. | Exact Unreal build, target, architecture, cook, and container compatibility required. |
| `native_extension` | Executable module or library. | Inactive under this contract. |

A mixed package uses the strictest class. A package cannot relabel a cooked asset
or executable member as portable data.

## Native projection declaration

`FSharModNativeProjection` contains:

| Field | Contract |
| :--- | :--- |
| `PackageId` | Canonical validated package identity. |
| `PackageRevision` | Exact admitted package revision. |
| `ProjectionClass` | Data overlay, cooked asset overlay, or native extension. |
| `ProjectionRevision` | Deterministic normalized projection revision. |
| `TargetId` | Required for target-specific material. |
| `ArchitectureId` | Required for target-specific material. |
| `UnrealVersion` | Exact compatible engine version for cooked content. |
| `ProjectCookRevision` | Exact compatible project and cook contract. |
| `GameFeatureId` | Optional canonical Game Feature projection identity. |
| `PrimaryAssets` | Added or overridden semantic primary-asset rows. |
| `RegistrySources` | Data Registry sources registered by the overlay. |
| `WorldContent` | Level, Data Layer, placement, or World Partition additions. |
| `LocalizationRows` | Canonical locale and text-key overlays. |
| `AudioRows` | Canonical audio-state or presentation overlays. |
| `RequiredPlugins` | Exact native plugin capabilities already present in the game. |
| `LoadBundles` | Required definition, gameplay, presentation, audio, and world bundles. |
| `ChangeSetHash` | Hash of the deterministic semantic change set. |

Physical archive paths, mount roots, filenames, and discovery order are not
projection identity.

## Portable data overlays

A data overlay contains normalized rows rather than arbitrary Unreal objects. The
runtime validates each row against the same repository-owned schema used by base
content.

Supported targets include:

- gameplay definitions and aliases;
- mission, objective, race, reward, offer, and progression rows;
- vehicle and character tuning within declared extensibility bounds;
- localization and dialogue-event references;
- music, ambience, and audio-event bindings;
- frontend and user-interface presentation records;
- population groups and placement records;
- mod-owned campaign definitions that do not mutate the base campaign sequence;
  and
- repository-owned configuration capabilities explicitly declared modifiable.

A data overlay may add a new canonical identity or override an extensible field
of an existing identity. It cannot mutate immutable base identity, save-key
meaning,
engine class ownership, source provenance, or a field not declared extensible.

Rows are stored in revision-bound repository-owned registries. The subsystem
constructs one immutable merged view from base rows followed by package priority
and dependency order. A tie or ambiguous provider has already failed package
validation and cannot reach this stage.

## Target-cooked asset overlays

A cooked asset overlay contains assets built for one exact supported target and
project cook revision. Runtime recooking and editor-only conversion are forbidden.

The package supplies an approved mounted container and one `UGameFeatureData`
projection. The projection may declare:

- primary asset scan and bundle metadata;
- Data Registry sources;
- additional gameplay or presentation definitions;
- components added to repository-owned extension points;
- World Partition content and Runtime Data Layers;
- user-interface, audio, media, animation, material, mesh, and effect assets; and
- package-owned test or review metadata excluded from shipping activation.

Game Feature actions may target only repository-owned extension points and
canonical semantic identities declared by the package. They cannot patch an
arbitrary actor class, replace a base object by path collision, execute editor
utilities, or access undeclared native capabilities.

The mounted content root is generated from package identity and revision. It is
not a public target and cannot change package priority.

## Native extensions

`native_extension` projections remain inactive. The runtime returns a typed
unsupported-trust result containing the exact target and missing policy.

A future native-extension implementation requires a separate accepted decision
covering at least signature authority, publisher identity, ABI, process boundary,
permissions, platform loading, crash containment, update rollback, and security
response. Structural package validation is not sufficient.

## Candidate activation transaction

Activation receives a revision-bound candidate active-set plan and performs:

1. verify package, policy, catalog, target, and current active-set revisions;
1. validate every native projection and target-specific member again;
1. stage portable rows in isolated registries;
1. stage cooked containers without exposing their assets to gameplay;
1. register candidate Game Feature URLs and dependencies;
1. load required definition bundles;
1. activate candidate Game Features in deterministic dependency order;
1. register Data Registry sources and catalog overlays;
1. register world content and validate Data Layer ownership;
1. build the merged catalog and alias view;
1. validate every dependency closure and save-visible identity;
1. read back Game Feature, registry, Asset Manager, and world-content state;
1. compare the read-back state with the candidate plan;
1. atomically publish the new overlay revision; and
1. release superseded Game Features, registries, mounts, and bundles only after
   publication succeeds.

Before publication, no gameplay query may observe candidate rows or assets.

## Deactivation and update

Removal and update construct a complete replacement active set. The subsystem
never subtracts package effects from live mutable objects in place.

Deactivation:

1. validates that active missions or interactions do not require the package;
1. creates a merged candidate view without the package;
1. verifies save-visible identities and missing-content policy;
1. switches gameplay queries to the replacement revision;
1. deactivates Game Feature actions;
1. unregisters candidate-only registry and catalog sources;
1. releases world content and mounted containers when no longer referenced; and
1. verifies that base and remaining overlay state match the accepted plan.

Saved canonical identities remain in the save document. Unavailable identities
are reported through the existing missing-content contract rather than deleted.

## World and campaign integration

Mod-owned world content declares its owning campaign, level, base world, Runtime
Data Layers, placement identities, and streaming dependencies.

An overlay may:

- add placements to a declared extensible level zone;
- add a mod-owned Data Layer to a repository-owned extension point;
- add an independent mod campaign and its worlds; or
- replace presentation through a declared variant slot.

It may not insert a level into `base_campaign`, change base level order, claim a
base placement identity, or infer ownership from a map path.

World content activates only after the owning base world and required layers are
known. Removing an active world overlay either completes a governed transition to
a safe base state or fails before deactivation.

## Save and determinism contract

The portable save records active package identities and revisions separately from
gameplay progression. It never stores mount routes, Game Feature URLs, object
paths, or package discovery order as semantic state.

Equivalent package records and base revisions produce equivalent:

- activation order;
- merged row identities and values;
- alias resolution;
- primary asset identifiers;
- Game Feature state;
- registry source order;
- world content membership;
- missing-content diagnostics; and
- active-set revision.

## Failure behavior

Activation fails closed on:

- stale package, policy, catalog, target, or active-set revision;
- an unsupported projection class;
- engine, target, architecture, ABI, plugin, or cook mismatch;
- a semantic row that violates its schema or extensibility policy;
- an unknown or immutable target;
- path or primary-asset identity collision;
- invalid Game Feature dependency or action;
- incomplete bundle, registry, alias, or world-content closure;
- failed mount, load, activation, registration, read-back, or publication;
- a save-visible identity whose missing-content policy is undefined; or
- any candidate result that differs from the deterministic plan.

Failure deactivates and unregisters staged candidate state, releases staged
mounts and bundles, and preserves the prior accepted overlay revision.

## Verification

Automated evidence includes:

- equivalent archive and directory imports yielding one projection;
- portable data overlays producing equivalent merged rows across supported
  targets;
- cooked overlays rejecting the wrong Unreal build, target, architecture, cook,
  or plugin set;
- deterministic Game Feature dependency and activation ordering;
- base and overlay primary-asset collision rejection;
- Data Registry priority and read-back tests;
- World Partition and Runtime Data Layer ownership tests;
- base-campaign mutation rejection;
- fault injection at every activation and deactivation stage;
- stale-preview and stale-active-set rejection;
- save reload with present, updated, removed, and unavailable packages;
- repeated activation producing equivalent merged state; and
- native-extension packages remaining inactive without the separate trust
  implementation.

## Known limits

This specification does not provide a hosted marketplace, remote discovery,
runtime asset cooking, arbitrary native-code sandbox, automatic package signing,
or cross-device package synchronization.
