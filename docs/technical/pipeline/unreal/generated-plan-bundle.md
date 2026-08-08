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
complete publication succeeds. Normalized JSON metadata preserves the exact
schema revision declared by its physical producer; recognized straggler families
fail closed when that bounded schema identity is missing, malformed, or belongs
to a different family.

Physical source reads, byte-count checks, mission semantic preflight, and
SHA-256 generation may execute concurrently through a bounded worker pool. Each
worker first requires a regular non-linked source through the shared filesystem
boundary, binds the pathname to the opened physical file identity, and rejects
identity, byte-length, or modification-time drift after the read. Mission JSON
is retained only for its semantic preflight; other source payloads stream through
the shared incremental SHA-256 boundary instead of being retained wholesale.
Manifest rows are parsed and validated before worker dispatch, completed results
are restored to manifest order before planning, and therefore worker scheduling
cannot alter published ordering or which earlier row owns an error.

`unreal-staging/` is generated, disposable, and ignored by Git. Every
published plan targets Unreal Engine 5.8.1 exactly; changing the engine patch
version requires a deliberate contract revision and regenerated evidence.

## Generated FBX catalog

Only packages with actual model or world geometry may emit model operations.
Decoded `p3d-mesh` evidence with no primitive groups is metadata rather than
geometry and cannot reserve an FBX operation. Scene, locator, camera, animation,
and physics evidence can accompany geometry in the FBX plan, but none can
independently reserve a mesh asset. Non-geometry packages remain at their
concrete native target or `requires-semantic-conversion` boundary instead of
producing placeholder FBX.

Model operations use the ignored `fbx-assets/` root as their only generated FBX
authority:

```text
fbx-assets/
├── catalog.jsonl
└── packages/
    └── <package_name>/
        ├── <package_name>.fbx
        └── textures/
            └── <texture_name>.png
```

The `fbx-export-catalog` command selects every current package whose planner
target is directly importable as `StaticMesh` or `SkeletalMesh`, reuses the
package FBX writers, hashes every generated FBX and external PNG, verifies the
complete catalog in staging, publishes the root with one rename, and reads the
published root back through the same verifier. Any package failure prevents
publication; a failed post-rename read-back removes the rejected root. Existing
accepted roots are never silently replaced by this command.

The JSONL header uses schema `shar-schoenwald.fbx-catalog.v2`, record type
`header`, status `complete`, the exact FBX package count, and the exact declared
artifact file count. Each `fbx` record contains the canonical package identity,
path relative to `fbx-assets/`, byte count, lowercase SHA-256 digest, and binary
FBX version `7700`. Optional `texture` records carry the owning package, exact
`textures/*.png` path, byte count, and lowercase SHA-256 digest. Texture records
are verified provenance for the external FBX package; they do not replace the
separate Unreal `Texture2D` operations and do not independently promote model
readiness. The JSONL uses UTF-8, LF line endings, no blank records, and one final
LF.

An absent root is not treated as an error: model operations remain
`requires-conversion`. Once the root exists, it is an all-or-nothing assertion.
The verifier rejects missing or unclaimed packages, orphan texture records,
duplicate paths, unsafe paths, symbolic links and reparse boundaries, unknown
files, noncanonical fields, stale sizes or hashes, non-PNG texture evidence,
invalid binary headers, and any FBX version other than 7.7. A verified complete
catalog must correspond exactly to every manifest package whose disposition is
`requires-fbx`; it promotes those model operations to `ready` and uses only the
generated FBX digest as each model source revision. A package-level
`SkeletalMesh` entry reserves both its primary object and the deterministic
`<AssetName>_Skeleton` companion; the companion is owned by the same import,
save, read-back, and rollback transaction rather than by a second plan
operation. Composite or otherwise unresolved semantic splits do not claim
catalog entries. A partial catalog never produces a mixed ready/pending bundle.

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

## Consumer preflight

Before any editor-control workflow may consume the generated plans, run the
three gates in increasing order:

```text
shar-unreal-mcp plan-preflight
shar-unreal-mcp plan-execution-preflight
shar-unreal-mcp plan-capabilities
shar-unreal-mcp plan-apply
```

`plan-preflight` is read-only and local. It opens no MCP session. It requires
exactly `plans/index.json` plus the six declared plan files as direct regular
files, rejects symbolic links and reparse boundaries, enforces bounded UTF-8
input and canonical one-line JSON with LF termination, and independently
recalculates every plan revision and the bundle revision using the canonical
Rust hashing contract.

The first gate also validates exact plan order, filenames, dependency revisions,
operation identities, source/readiness mappings, generated destinations,
outputs, validation requirements, case-insensitive collisions, dependency
family order, and cycle freedom. Construction operations must reference exactly
`unreal-staging/manifest.jsonl` through the portable `manifest.jsonl` source and
must repeat the bundle source-manifest revision.

`plan-execution-preflight` remains local. It verifies every applicable source as
a regular non-linked file beneath its declared generated root, streams SHA-256
from a stable file descriptor, detects size or identity changes during the read,
deduplicates shared source evidence, and compiles only exact reviewed native
routes. Operations still in `requires-conversion` are counted but not opened.

`plan-capabilities` reruns both local gates before connecting to Unreal. It lists
the live Toolset Registry and describes only toolsets required by compiled
routes. It validates exact input and output schemas for native import, explicit
save, existence, class, dirty-state read-back, and compensating deletion without
invoking `call_tool`.

Ready PCM WAV operations compile to the project editor toolset
`SharImportEditor.SharImportToolset.ImportSoundWave`. That toolset is loaded at
`PostEngineInit`, validates generated destinations, uses a synchronous automated
`USoundFactory` import task without replacement or implicit save, and returns
the imported Unreal object paths. Capability preflight still requires its live
schema before any SoundWave mutation.

Ready HAP operations compile to
`SharImportEditor.SharImportToolset.ImportFileMediaSource`. Each operation owns
both a generated `UFileMediaSource` package and one deterministic external MOV
payload beneath `Content/Movies/Generated/SHAR/`. The tool copies verified bytes
to a same-directory temporary file, publishes without replacement, stores the
matching project-relative `./Movies/Generated/SHAR/...` path, and leaves the
package dirty for explicit save. Capability preflight requires import, payload
existence, stored-path read-back, and payload deletion schemas before mutation.

`plan-apply` refuses an incomplete execution report before transport creation.
For a complete plan it repeats the capability audit, checks every destination is
absent before mutation, and applies imports serially. Each imported package must
exist, expose the planned class, save explicitly, and report clean afterward.
Media imports additionally require the exact stored relative path and external
payload. A failure compensates every effect created by that transaction in
reverse order; movie payloads are deleted before their assets. Every absence is
independently verified. Preexisting destinations or payloads are never adopted
or deleted. An ambiguous import timeout is probed for either created effect
before rollback.

A gate succeeds as complete only when every operation has verified source bytes,
a reviewed route, and compatible live capabilities. Unsupported families and
readiness blockers remain visible in aggregate counts; no command treats or
executes a partial subset as completion evidence. Successful direct-import
application still does not complete repository-owned factories, world assembly,
runtime binding, validation, cook, or package families.

## Source projections

The aggregate plans accept only verified normalized inputs:

- decoded images become ready texture-import operations;
- PCM WAV files become ready sound-wave import operations;
- verified HAP media files become ready media-source import operations;
- deterministic binary FBX 7.7 destinations become model-import operations that
  remain blocked until the complete generated catalog verifies every declared
  file, then become ready with the verified FBX digest; and
- normalized JSON evidence with a concrete Unreal target becomes a
  native-construction operation that remains blocked until the declared
  repository-owned editor factory is available; and
- normalized source that still requires domain interpretation remains a
  `requires-semantic-conversion` package disposition in import manifest v2 and
  emits no plan operation until a deterministic compiler produces a concrete
  Unreal target.

Every operation has a stable identity derived from canonical source and target
fields. Source paths are portable relative paths. Source revisions are lowercase
SHA-256 digests. Destinations are confined to `/Game/Generated/SHAR/` and are
rejected on case-insensitive collision.

## Package dispositions versus operation readiness

Import manifest v2 separates package classification from executable work.
Packages may be `direct-editor-import`, `requires-fbx`,
`requires-editor-factory`, `requires-semantic-conversion`, or `metadata-only`.
The summary publishes `requires_semantic_conversion` independently from editor
factory counts.

`requires-semantic-conversion` means normalized evidence exists, but the
repository has not yet compiled it into the concrete typed Unreal definition
required by the accepted runtime contract. Such a package reserves no Unreal
object, declares no generic `DataAsset`, and emits no asset-construction
operation. Mission-script bundles are one example: they must compile into typed
mission definitions and bindings for the shared mission StateTree contract,
not one ad hoc StateTree or abstract data asset per source bundle. Pure3D sprite
layouts, Scrooby project headers, TextureFont evidence, and TextBible headers are
also semantic evidence rather than standalone `Texture2D`, `WidgetBlueprint`,
`Font`, or `StringTable` factory inputs. TextureFont extraction preserves its
embedded atlas PNGs plus each fixed 40-byte glyph record as ten raw little-endian
words; field semantics remain unassigned until an authoritative compiler mapping
exists. A direct `Texture2D` target requires a physical PNG member owned by the
package instead of a sprite/layout JSON row.

Only after semantic compilation produces a concrete target can the resulting
work enter an operation plan and acquire operation readiness.

Plan-bundle index v2 carries `semantic_blocker_count` as aggregate completion
evidence. The count participates in the bundle revision and is validated by the
local consumer before any MCP transport exists. Execution preflight therefore
reports `complete=false` whenever the count is nonzero, including a bundle with
zero emitted operations. This keeps unresolved semantic work visible without
inventing a fake operation or a fourth operation-readiness state.

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
