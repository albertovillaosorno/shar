# Legacy runtime identity normalization

- Status: Active
- Last reviewed: 2026-07-14

## Governing decisions

<!-- markdownlint-disable-next-line MD013 -->
- [Data-driven Unreal gameplay content catalog](../../adr/unreal/runtime/data-driven-gameplay-content-catalog.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Converted asset ingestion boundary](../../adr/unreal/import-adapters/converted-asset-ingestion-boundary.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Import review boundary](../../adr/unreal/import-adapters/import-review-boundary.md)
- [Runtime parity boundary](../../adr/unreal/runtime/remake-parity-boundary.md)

## Purpose

This specification defines how source ordinals, bit masks, chunk numbers,
platform paths, fixed capacities, substring tables, and callback integers are
normalized into stable native Unreal identities and typed policies.

Legacy values remain useful conversion evidence. They do not remain the primary
runtime identity merely because older source used an enum, array index, numeric
chunk identifier, or string literal.

## Ownership

<!-- markdownlint-disable MD013 -->

| Authority | Responsibility |
| :--- | :--- |
| Conversion pipeline | Parse source values and emit normalized identity evidence. |
| Gameplay catalog | Canonical gameplay and presentation identities. |
| Import adapters | Create native assets and record source-to-native mappings. |
| Runtime subsystems | Consume typed native identities and policies. |
| Provenance records | Preserve source ordinal, name, hash, chunk, and mapping revision. |

<!-- markdownlint-enable MD013 -->

No runtime subsystem may infer domain ownership from a source numeric range.

## Normalization record

Every translated source identity produces one `FSharLegacyIdentityMapping` with:

<!-- markdownlint-disable MD013 -->

| Field | Contract |
| :--- | :--- |
| `SourceDomain` | Closed source family such as vehicle, character, particle, chunk, or callback. |
| `SourceValue` | Exact numeric, bit-mask, or normalized textual value. |
| `SourceName` | Optional exact source label retained as provenance. |
| `CanonicalId` | Stable repository-owned native identity. |
| `NativeType` | Expected data asset, gameplay tag, enum, class, or package type. |
| `Availability` | Shipping, contextual, development, provenance-only, or rejected. |
| `MappingRevision` | Immutable conversion-policy revision. |
| `Evidence` | Package, manifest, or generated-record identity supporting the mapping. |

<!-- markdownlint-enable MD013 -->

Mappings are generated, reviewed, and immutable for a published catalog
revision. Runtime code does not construct them from display text.

## Canonical identity rules

Canonical identities:

- are globally unique within their declared domain;
- use stable semantic names rather than source array positions;
- survive sorting, insertion, deletion, and platform changes;
- carry aliases only through validated generated mappings;
- never depend on localized display text;
- reject ambiguous source values; and
- retain source ordinals only for import diagnostics and migration.

A native enum may represent a small closed execution vocabulary, but the enum
value is generated from the canonical schema and is not required to equal the
source ordinal.

## Primitive width, alignment, and byte order

Source platform aliases for integer width are conversion provenance only.
Canonical records use explicit fixed-width signed and unsigned fields wherever
serialization, hashing, identity, networking, or deterministic arithmetic depend
on width.

A source value that may be unaligned is decoded through a bounded reader that
declares:

- source width and signedness;
- byte order;
- required alignment or unaligned-read policy;
- overflow and truncation behavior;
- canonical native destination type; and
- mapping and schema revision.

Runtime code never reinterprets adjacent words as a wider native integer,
depends
on compiler padding, or changes comparison and hashing by target architecture.
Malformed width, alignment, or byte-order evidence fails conversion.

## Actor-state values

Source flying-actor state numbers map to canonical StateTree state identities
and
gameplay tags such as fade-in, passive, preparing attack, ready, attacking, and
destroyed.

The numeric source state is provenance. Native transitions, interruption,
timeouts, and verification follow the flying-hazard StateTree contract. Save or
replication state uses canonical state identity and definition revision, never
an
unversioned source integer.

## Blob-shadow mappings

Substring-based object-name searches are converted into explicit presentation
bindings during import. Each binding records:

- canonical source mesh or mesh-family identity;
- shadow presentation definition;
- optional socket or placement rule;
- scale and orientation policy;
- availability by graphics preset; and
- import evidence.

Runtime loading does not scan arbitrary object names for partial matches. Two
source patterns that match one asset ambiguously fail conversion. Missing shadow
presentation does not change collision or gameplay identity.

## Breakable identities

Source breakable numbers map to stable breakable-definition identities. A
breakable definition owns:

- intact and broken presentation;
- collision transition;
- damage and destruction policy;
- particle, audio, decal, and debris identities;
- reward or mission application-port request when applicable;
- persistence and respawn policy; and
- platform and quality presentation variants.

Sparse source numbers remain provenance and cannot determine array placement.
The source maximum-name count is not a native capacity.

## Character identities

Source walker ordinals map to canonical character identities in the gameplay
catalog. Playable, ambient, mission, dialogue, costume, and cinematic placements
reference the same character identity with separate placement or role rows.

A source ordinal cannot be written to save data, used as a package name, or used
to infer whether the character is playable. Unknown or unsupported source values
remain unresolved conversion findings.

## Directional-arrow policy

Source arrow bit masks are translated into a closed native routing policy:

| Native policy | Meaning |
| :--- | :--- |
| `intersection` | Project guidance at the next route intersection. |
| `nearest_road` | Project guidance to the nearest valid route segment. |
| `combined` | Apply both validated projections. |
| `hidden` | Do not render an arrow for the current route observation. |

The native policy is not interpreted through integer bit arithmetic at runtime.
Invalid or contradictory source combinations fail conversion.

## Capacity constants

Source maximum-player, maximum-NPC, command-table, callback, camera, and similar
values are historical implementation capacities. They become one of:

- verified gameplay rule;
- platform or performance budget;
- imported-data validation bound;
- development-tool limit; or
- provenance-only observation.

A historical array size is never copied into native code without an explicit
current contract. Native collections use authored definitions, platform budgets,
and validated dynamic storage. Split-screen player limits remain a separate
product decision and test matrix.

## Movie and media names

Platform-specific source file paths map to canonical cinematic identities and
platform packaging recipes. Native runtime references use soft object or media
source identities, not drive letters, backslashes, disc roots, or filename-based
platform selection.

Each media mapping records:

- cinematic identity;
- source media evidence;
- normalized MOV or HAP evidence when applicable;
- native media source and sequence package;
- audio and localization policy;
- startup, story, gallery, or credits role; and
- platform cook variants.

Missing optional startup media follows the application-mode fallback contract.
Missing required story media reports a typed presentation failure without
changing progression.

## Particle identities

Source particle numbers map to stable Niagara-system or presentation-definition
identities. A particle definition declares spawn transform, attachment,
lifetime,
bounds, scalability, pooling, quality policy, and gameplay independence.

A source particle number cannot select damage, reward, or destruction behavior.
Those effects remain owned by the relevant domain transaction.

## Physical-property classes

Source physical-property class numbers map through the physical-material catalog
to collision class, native physical material, density or mass policy, friction,
restitution, surface type, and impact-response definitions.

The source class value remains import evidence only. Runtime collision queries
use
native channels, object types, component tags, and physical materials.

## Chunk identifiers

Source package chunk numbers belong exclusively to extraction, normalization,
and
import schema selection. They never become gameplay identities.

A chunk mapping records:

- exact numeric chunk identifier;
- parser schema identity and revision;
- expected parent and child relationships;
- normalized output type;
- unsupported or optional status; and
- evidence requirements.

Unknown chunk identifiers remain explicit findings. A parser cannot reinterpret
an unknown number as the nearest known type or silently discard required
children.
Native UAssets do not store source chunk numbers except in provenance metadata.

## State-prop callbacks

Source callback integers map to typed effect definitions and application-port
requests. The supported families include:

- state transition observation;
- remove actor or collision component;
- spawn a declared coin reward transaction;
- fire a projectile or energy effect;
- change speed or movement policy;
- apply radial force;
- emit a presentation effect;
- publish destruction observations;
- request camera shake; and
- domain-specific collectible or prop results.

A callback number cannot directly invoke arbitrary code. Every mapping declares
payload schema, owning subsystem, idempotency, authority, timeout when
applicable,
and verification.

## Vehicle identities

All source vehicle enums, aliases, traffic arrays, and platform-specific roster
values map to the canonical vehicle catalog. One vehicle identity may have
separate acquisition, traffic, mission, opponent, presentation, tuning, and
platform rows.

Source ordinal gaps, duplicate historical lists, placeholder values, and
not-yet-enabled labels do not create new native identities. Save data, mission
state, phone-booth retrieval, traffic, and race definitions use canonical
vehicle
identity and definition revision.

## Source names and hashes

Hashed names and short source labels are aliases or provenance, not canonical
identity. Normalization records the exact observed form and resolves it through
an explicit generated alias table.

Alias generation rejects:

- one alias resolving to multiple canonical identities;
- case or punctuation normalization collisions;
- accidental substring matching;
- platform-only aliases without a platform policy;
- empty or sentinel names; and
- display text used as a technical key.

## Import transaction

Identity normalization occurs before native mutation:

1. parse the source value under an exact schema;
1. resolve one mapping revision;
1. validate the canonical target and native type;
1. verify dependencies and aliases;
1. include the mapping in the approved import plan;
1. perform native mutation;
1. read back the resulting native identity and dependencies; and
1. publish provenance only after the plan matches.

A missing, ambiguous, or stale mapping fails before final package publication.

## Runtime boundary

Shipping runtime code consumes generated native definitions. It must not:

- include legacy source header tables as gameplay authority;
- switch on source chunk numbers;
- index arrays by source vehicle, character, particle, or breakable ordinal;
- search arbitrary object names for behavior;
- construct platform media paths;
- enforce historical fixed capacities without a current policy; or
- dispatch source callback integers directly.

A narrow compatibility adapter may accept a source value only at an import,
migration, mod-ingestion, or test boundary and must return a typed mapping
result.

## Save and migration

New save schemas store canonical identities and definition revisions. When an
older imported save contains a source ordinal, migration uses the exact mapping
revision associated with that save format.

Migration rejects ambiguous values and records unresolved state for recovery. It
does not guess from current catalog order. Once migrated, the source ordinal is
not written back as current authority.

## Mods

A mod may add namespaced canonical identities and explicit aliases. It cannot
claim a first-party source ordinal or alias already mapped by the active catalog
unless an accepted override policy permits the exact identity and dependency
revision.

Mod manifests declare every legacy alias they ingest. Unknown values remain
rejected rather than falling through to first-party defaults.

## Validation

Generated validation rejects:

- duplicate canonical identities;
- one source value mapped to multiple targets under one revision;
- one target assigned incompatible native types;
- unresolved required values;
- accidental source-ordinal array indexing in runtime definitions;
- platform paths in native gameplay data;
- substring behavior mappings without explicit source assets;
- callbacks without typed payload and authority;
- chunk mappings without parser schema; and
- save migrations without an exact source-format revision.

## Tests

Required tests include:

- deterministic mapping generation;
- sparse and duplicate source ordinal handling;
- alias collision rejection;
- actor-state StateTree projection;
- blob-shadow explicit binding;
- breakable and particle presentation lookup;
- character and vehicle identity migration;
- directional-arrow policy conversion;
- historical capacity classification;
- platform media identity resolution;
- physical-property mapping;
- unknown chunk preservation;
- typed state-prop callback dispatch;
- save migration with an exact historical mapping revision; and
- mod alias conflict rejection.

## Invariants

- Source numbers are provenance, not gameplay identity.
- Source paths are provenance, not native package identity.
- Unknown values remain explicit findings.
- Runtime behavior cannot depend on source array order.
- Every compatibility mapping is versioned and testable.
- Native identity read-back must match the approved import plan.
