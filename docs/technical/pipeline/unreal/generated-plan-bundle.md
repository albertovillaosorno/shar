<!--
SPDX-FileCopyrightText:
  - Copyright (c) 2026 Alberto Villa Osorno.
SPDX-License-Identifier:
  - MIT
Confidential:
  - false
License-File:
  - LICENSE-MIT
-->

# Generated Unreal plan bundle

- Status: Active
- Last reviewed: 2026-08-06

## Purpose

Define the disposable pipeline artifact that projects verified extraction
evidence into the six aggregate Unreal plan families required before native
editor application.

This bundle is an orchestration and assurance boundary. It does not mutate an
Unreal project, replace package-local import transactions, or claim that a
native asset exists merely because an operation was planned.

## Generation

The canonical command is:

```text
pipeline prepare-unreal game extracted
```

The command verifies the successful minor-unit audit, rebuilds the import
manifest from source evidence, derives every plan revision from canonical
content, writes into a transaction-specific staging directory, verifies each
file, and atomically replaces the accepted `unreal-staging/` root only after the
complete publication succeeds.

`unreal-staging/` is generated, disposable, and ignored by Git. Every
published plan targets Unreal Engine 5.8.1 exactly; changing the engine patch
version requires a deliberate contract revision and regenerated evidence.

## Generated FBX catalog

Model operations use the ignored `fbx-assets/` root as their only generated FBX
authority:

```text
fbx-assets/
├── catalog.jsonl
└── packages/
    └── <package_name>/
        └── <package_name>.fbx
```

The JSONL header uses schema `shar-schoenwald.fbx-catalog.v1`, record type
`header`, status `complete`, and the exact package count. Each `fbx` record
contains the canonical package identity, path relative to `fbx-assets/`, byte
count, lowercase SHA-256 digest, and binary FBX version `7700`. The JSONL uses
UTF-8, LF line endings, no blank records, and one final LF.

An absent root is not treated as an error: model operations remain
`requires-conversion`. Once the root exists, it is an all-or-nothing assertion.
The verifier rejects missing or unclaimed packages, duplicate identities or
paths, unsafe paths, symbolic links and reparse boundaries, unknown files,
noncanonical fields, stale sizes or hashes, invalid binary headers, and any FBX
version other than 7.7. A verified complete catalog promotes every corresponding
model operation to `ready` and uses the generated FBX digest as its source
revision. A partial catalog never produces a mixed ready/pending bundle.

## Published files

One successful transaction publishes exactly nine files:

```text
unreal-staging/
├── manifest.jsonl
├── summary.json
└── plans/
    ├── index.json
    ├── asset-import-plan.json
    ├── asset-construction-plan.json
    ├── world-assembly-plan.json
    ├── runtime-binding-plan.json
    ├── validation-plan.json
    └── package-plan.json
```

The index records the six plan identities, revisions, filenames, and operation
counts. Each plan repeats the source-manifest revision, engine-contract
revision,
target engine version, target platform, direct plan dependencies with
their exact
prerequisite revisions, expected outputs, ordered operations, and required
validation gates.

## Source projections

The aggregate plans accept only verified normalized inputs:

- decoded images become ready texture-import operations;
- PCM WAV files become ready sound-wave import operations;
- verified HAP media files become ready media-source import operations;
- deterministic binary FBX 7.7 destinations become model-import operations that
  remain blocked until the complete generated catalog verifies every declared
  file, then become ready with the verified FBX digest; and
- normalized JSON evidence becomes native-construction operations that remain
  blocked until the declared repository-owned editor factory is available.

Every operation has a stable identity derived from canonical source and target
fields. Source paths are portable relative paths. Source revisions are lowercase
SHA-256 digests. Destinations are confined to `/Game/Generated/SHAR/` and are
rejected on case-insensitive collision.

## Readiness states

Operations use one of three explicit states:

- `ready`: verified interchange bytes already exist and may enter native import;
- `requires-conversion`: an upstream deterministic converter must publish the
  declared file before editor application; or
- `requires-editor-factory`: repository-owned native automation must construct
  the target from normalized data.

A plan file may contain zero operations. An empty world-assembly,
runtime-binding, validation, or package plan is an honest declaration that no
source-backed operations have been emitted for that family yet; it is not
completion evidence.

## Relationship to package transactions

The aggregate bundle groups work by execution responsibility and establishes
cross-family revisions. Native application still partitions work into the
package-local transaction contract defined by
`identity-naming-revisions-and-import-plans.md`.

The editor adapter must resolve trusted local roots, validate the aggregate and
package revisions, acquire the generated-content lease, stage native objects,
read them back independently, save and validate them, and publish or roll back
the transaction. MCP may launch and observe this process, but it does not own
classification, naming, import settings, construction policy, or recovery.

## Determinism and safety

Accepted bundle identity excludes timestamps, process identifiers, temporary
paths, editor session state, local drive spelling, and discovery order.
Reversing input discovery order must produce the same operation identities,
plan revisions, and bundle revision.

Generation fails before publication when it encounters an unsafe path, invalid
or uppercase digest, duplicate operation identity, unknown dependency,
case-insensitive destination collision, unsupported direct-import extension,
undeclared output path, or an existing generated FBX catalog that is not exact.
Interrupted or failed generation leaves the previous accepted root unchanged.
