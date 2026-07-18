<!-- markdownlint-disable-next-line MD013 -->
# Historical technical-design, QA, performance, and production evidence normalization

- Status: Active
- Last reviewed: 2026-07-17

## Governing decisions and specifications

<!-- markdownlint-disable-next-line MD013 -->
- [Runtime parity boundary](../../adr/unreal/runtime/remake-parity-boundary.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Historical source-document evidence classification and publication boundary](historical-source-document-evidence-classification-and-publication-boundary.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Historical core-design and dialogue evidence normalization](historical-core-design-and-dialogue-evidence-normalization.md)
- [Configuration and asset validation](config-and-asset-validation.md)
- [Unreal test taxonomy](testing/test-taxonomy.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Native art authoring, style, and asset validation contract](native-art-authoring-style-and-asset-validation-contract.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Memory ownership, budget, and diagnostics runtime](memory-ownership-budget-and-diagnostics-runtime.md)
- [Platform quality and optimization](platform-quality-and-optimization.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Developer command and diagnostic runtime](developer-command-and-diagnostic-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Native platform bootstrap and error-recovery runtime](native-platform-bootstrap-and-error-recovery-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Deterministic conversion pipeline](../pipeline/deterministic-conversion-pipeline.md)

## Purpose

This specification defines how historical QA sheets, technical-design records,
architecture documents, art requirements, performance notes, memory maps,
pipeline instructions, mastering procedures, coding standards, postmortems,
risk registers, roadmaps, and production-administration records may contribute
bounded reconstruction evidence without becoming target architecture or public
source-document reproductions.

Historical technical detail is evidence, not implementation authority. It may
establish a behavior, constraint, failure mode, test category, data
relationship, or quality goal only after reconciliation with current repository
decisions, actual assets, native Unreal behavior, and deterministic validation.

## Scope and exclusions

This boundary covers private evidence that describes:

- QA cases, content-review schemas, defects, observations, and expected results;
- source game systems, managers, interfaces, class diagrams, and pseudocode;
- art, animation, rendering, camera, character, vehicle, world, mission, audio,
  frontend, input, save, and presentation requirements;
- source export, conversion, build, mastering, deployment, and content
  pipelines;
- source-platform frame, memory, bandwidth, storage, and asset budgets;
- code-review, style, postmortem, risk, milestone, and roadmap records; and
- workstation, network, ownership, assignment, and schedule administration.

It does not authorize copying source code, class layouts, pseudocode,
architecture diagrams, commands, absolute paths, addresses, employee data,
platform secrets, proprietary tool procedures, or obsolete platform policy into
the public repository.

## Ownership

<!-- markdownlint-disable MD013 -->

| Authority | Responsibility |
| :--- | :--- |
| Evidence intake | Inventory, digest, structure, media classification, and private identity. |
| Privacy and publication review | Personal, employment, network, rights, and public-output decisions. |
| Technical normalization | Complete fact extraction, semantic routing, conflict tracking, and terminal classification. |
| Current domain specifications | Target behavior, native Unreal ownership, typed identities, and accepted architecture. |
| Native Unreal and platform tooling | Runtime behavior, profiling, packaging, cooking, diagnostics, and platform integration. |
| Validation | Deterministic tests, asset read-back, performance evidence, leak checks, and public-safety scans. |

<!-- markdownlint-enable MD013 -->

No historical manager, source interface, platform budget, review checklist, or
production procedure becomes a target owner.

## Evidence-family transaction

One technical-evidence family is processed atomically:

1. inventory every declared member and record its opaque private revision;
1. classify every row, paragraph, table, diagram reference, code fragment, and
   administrative field;
1. separate semantic requirements from source implementation and production
   metadata;
1. map every retained fact to one current domain owner and stable public-safe
   identity;
1. compare equivalent, changed, superseded, rejected, and conflicting facts;
1. validate accepted facts against current architecture, native behavior,
   assets, tests, and measured evidence;
1. publish only repository-authored specifications, schemas, tests, or assets;
   and
1. record one terminal result for the complete family.

A failed family publishes no partial architecture, budget, checklist, command,
network record, performance target, or runtime definition.

## Closed evidence families

### QA and content-review evidence

QA sheets may establish candidate test cases, object categories, locations,
expected behavior, observed behavior, severity, reproducibility, platform,
build, and content-revision context.

A normalized QA case contains:

- stable case and tested-content identities;
- world, region, object, vehicle, character, asset, mission, or system context;
- test preconditions and controlled configuration;
- expected and observed outcomes;
- defect category, severity, reproducibility, and evidence quality;
- the current domain owner and required regression test; and
- accepted, adapted, superseded, rejected, duplicate, or unresolved status.

Legends and category sheets are test-schema evidence, not result sets. A marked
cell, color, ordinal, date, changelist, worksheet name, or reviewer note cannot
establish acceptance by itself. Blank, untested, unavailable, not-applicable,
passed, failed, and inconclusive remain distinct states.

Collision findings route to native collision, physical-material, world-entity,
vehicle, character, navigation, and placement owners. Content-review findings
route to the owning art, animation, audio, UI, gameplay, catalog, or platform
contract. Review status never replaces native asset validation or runtime tests.

### Source technical-design and architecture dossiers

A technical-design dossier may describe source managers, contexts, components,
interfaces, callback graphs, class hierarchies, enums, pseudocode, global state,
manual lifetime rules, custom allocators, render layers, loaders, scripts, or
platform branches.

The reviewer extracts only product-safe semantic facts such as:

- required behavior and observable state transitions;
- input, output, timing, ordering, interruption, and recovery rules;
- ownership boundaries and external dependencies that remain meaningful;
- content, asset, world, mission, vehicle, character, camera, audio, UI, or save
  relationships;
- quality, accessibility, determinism, and failure requirements; and
- candidate test scenarios and invariants.

Source class names, pointer ownership, singleton graphs, manager APIs, global
arrays, manual initialization order, source event buses, custom rendering loops,
fixed player slots, raw resource names, and platform conditionals are
implementation evidence only. Current domain specifications and Unreal-native
facilities remain authoritative.

A broad source architecture is decomposed by domain. It cannot be translated as
one target architecture document, and it cannot override hexagonal boundaries,
CQRS separation, native subsystems, world and game-feature lifetimes, Asset
Manager ownership, CommonUI, Enhanced Input, Chaos, Niagara, MetaSounds, or
current repository contracts.

### Technical-art requirements and budgets

Historical art records may establish candidate mesh roles, topology, LOD,
materials, UVs, textures, animation, skeleton, collision, shadow, toon-line,
locator, presentation, and validation intent.

Source polygon counts, texture dimensions, bit depths, platform memory numbers,
file naming, exporter settings, scene hierarchy, and fixed object counts are
candidate constraints only. Acceptance requires current asset identity, units,
quality tier, platform profile, visual review, deterministic import, and native
read-back.

An art-budget recommendation may inform current profiling scenarios. It does not
become a current budget merely because it was labeled final, shipped on an older
platform, or appeared in a technical specification.

### Sound and presentation design records

Broad sound-design and presentation records may establish semantic event roles,
music states, dialogue relationships, spatial behavior, attenuation, priority,
streaming, interruption, cinematic synchronization, UI feedback, and quality
intent.

Source sound managers, resource scripts, bank layouts, fixed channel counts,
platform libraries, filenames, source locators, or custom playback classes do
not ship. Audio facts route to the current music, dialogue, vehicle-audio,
gameplay-audio, spatial-audio, platform-cooking, and presentation owners.

Character-owned sound-event tables remain governed by the dialogue evidence
importer and owner-scoped source-family reconciliation. A broad sound document
cannot create a second event catalog.

### Source pipeline, export, and mastering records

Historical pipeline evidence may establish semantic stages such as validation,
conversion, dependency resolution, platform variation, cooking, packaging,
verification, and artifact publication.

Source commands, scripts, grammars, drive letters, depot locations, workstation
layouts, proprietary exporters, optical-media procedures, cartridge procedures,
manual file copying, source-control instructions, and platform product codes are
obsolete or confidential process evidence. They are never published as current
commands or executed by reconstruction tooling.

Current pipeline authority belongs to repository-owned deterministic conversion,
native Unreal import, cooking, packaging, validation, and guarded publication.
An accepted migration fact must state its semantic purpose, typed inputs and
outputs, failure behavior, determinism proof, and target owner without retaining
a source command or path.

### Performance and memory evidence

Historical performance reports and memory maps may provide candidate categories,
bottlenecks, measurement methods, quality tradeoffs, and regression scenarios.
Each retained measurement records:

- source platform class and relevant hardware profile;
- tested build and content revision when safely known;
- metric identity, unit, sample window, aggregation, and measurement method;
- workload, scene, asset, mission, vehicle, camera, or rendering context;
- expected range, observed result, and uncertainty;
- current platform-profile applicability; and
- accepted, adapted, superseded, rejected, or unresolved status.

Old frame rates, memory capacities, bandwidth values, buffer layouts, fixed pool
sizes, content counts, and platform-specific optimization tricks do not become
current limits. Current budgets derive from supported target profiles and native
measurements using Unreal Insights, Memory Trace, LLM, platform profilers, asset
audits, automated performance tests, and cooked-build evidence.

A historical optimization may become a test hypothesis, but never a target rule
without measured benefit, visual and gameplay equivalence, maintainability, and
current-platform validation.

### Process, standards, postmortems, risks, and roadmaps

Historical coding standards, review primers, development plans, postmortems,
risk registers, and roadmaps are production-process evidence. They may suggest
neutral quality principles, test scenarios, known failure patterns, or candidate
technical risks after independent review.

They do not replace current repository formatting, language, architecture,
validation, commit, documentation, security, or release policy. Source naming
conventions, file templates, review schedules, milestone promises, staffing,
deliverables, estimates, dates, and apparent priority are not target authority.

A postmortem observation is correlated with current code, assets, tests, and
architecture before adoption. A risk or roadmap item is a proposal, not proof
that the feature existed, remains required, or belongs in the current product.
Every retained recommendation receives one current owner and terminal decision.

### Credits and attribution records

Credits and attribution lists route exclusively through the current legal,
privacy, provenance, and credits-publication boundary. Historical role and name
pairs are candidates, not permission to publish or infer employment history.

Technical evidence may cite an opaque attribution-review identity. Runtime and
technical documentation must not reproduce unreviewed names, contact details,
company relationships, acknowledgements, URLs, trademarks, or third-party
technology claims.

### Network and workstation administration

Addresses, machine assignments, account names, workstation ownership, debug
endpoints, internal hostnames, storage mappings, office topology, and similar
administrative records produce no public architecture, runtime, test fixture,
example configuration, or diagnostic output.

Such records are classified as private operational data. Technical review may
retain only a generic conclusion such as requiring a configurable endpoint or a
platform adapter when that conclusion is independently justified by current
product requirements. The source values themselves are never copied.

## Current-domain routing

<!-- markdownlint-disable MD013 -->

| Historical topic | Current authority |
| :--- | :--- |
| Avatar, character, vehicle entry, and control | Playable-avatar, vehicle-access, vehicle-physics, and input specifications. |
| Camera modes, targets, transitions, and arbitration | Camera system and camera-rig specifications. |
| Character animation, facial behavior, and choreography | Character-animation catalog, typed action sequence, and presentation playback. |
| Missions, scripts, events, objectives, and locators | Mission, typed-event, spatial-placement, interaction, and world-entity specifications. |
| Road networks, pedestrian paths, traffic, and vehicle AI | Road-network, pedestrian-path, race-route, and vehicle-AI specifications. |
| Rendering, shadows, toon lines, culling, and layers | Native render-frame, art-validation, material, visibility, and VFX specifications. |
| Frontend, GUI, platform errors, save, and input | CommonUI, frontend flow, platform bootstrap, storage, and semantic-input specifications. |
| Audio, dialogue, music, and sound pipelines | Dialogue, music, gameplay-audio, vehicle-audio, spatial-audio, and platform-audio specifications. |
| Loading, streaming, memory, and resource lifetime | Native asset-load, world streaming, memory ownership, and application-lifecycle specifications. |
| Art pipeline, import, conversion, and asset assembly | Deterministic conversion, native import, cooked construction, and asset validation. |
| Performance, quality, and platform budgets | Platform-quality, memory-budget, diagnostics, and supported-target specifications. |
| QA, content review, and regression evidence | Test taxonomy, config and asset validation, native read-back, and owning domain tests. |

<!-- markdownlint-enable MD013 -->

The routing table is not a substitute for the owning documents. It prevents a
historical technical dossier from becoming a parallel source of truth.

## Public outputs

Allowed public outputs are limited to repository-authored:

- behavioral requirements and architecture decisions;
- typed schemas, identities, mappings, and validation rules;
- deterministic import or normalization contracts;
- redacted aggregate QA or performance findings;
- unit, integration, asset, editor, runtime, and performance tests;
- current platform budgets supported by native measurements; and
- independently created Unreal assets and configuration.

Public outputs exclude source prose, tables, code, pseudocode, diagrams,
commands, paths, addresses, names, schedules, build-distribution instructions,
private review comments, and obsolete platform procedures.

## Failure behavior

Normalization fails closed when:

- a family member is missing or its revision relation is unresolved;
- a fact has no current domain owner or stable semantic identity;
- source implementation is presented as target architecture;
- units, scale, platform, workload, or measurement method are ambiguous;
- QA state, expected behavior, or tested revision cannot be established;
- a source command, address, personal record, or confidential path would enter a
  public artifact;
- a performance or memory claim lacks current native evidence;
- a historical standard conflicts with current repository authority;
- credits or third-party claims lack publication approval;
- source-only dependencies remain in a target asset or specification; or
- native validation or read-back disagrees with the proposed normalized fact.

The previously accepted public specification, test, budget, pipeline, and asset
revisions remain active.

## Verification

Automated verification covers:

- complete member and token accounting;
- deterministic family ordering and duplicate collapse;
- QA schema versus result separation;
- blank, untested, passed, failed, and inconclusive states;
- current-domain routing for every retained fact;
- rejection of source class names, commands, paths, and addresses;
- personal, employment, schedule, and network-data exclusion;
- source architecture supersession by current owners;
- unit-preserving performance and memory normalization;
- current-platform benchmark and read-back requirements;
- coding-standard and roadmap non-authority;
- credits and third-party publication gating;
- public-safe diagnostic rendering; and
- atomic publication with rollback on any failure.

## Invariants

- Historical technical detail never becomes target architecture by translation.
- Every retained fact has exactly one current owner and terminal decision.
- QA forms and category sheets are schemas until actual observations exist.
- Old platform budgets never become current limits without native measurement.
- Source commands, paths, addresses, and personal records never enter public
  artifacts.
- Current repository policy outranks historical coding and production standards.
- Public output remains independently authored, deterministic, validated, and
  safe for external review.
