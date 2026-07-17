# Historical core-design and dialogue evidence normalization

- Status: Active
- Last reviewed: 2026-07-17

## Governing decisions and specifications

<!-- markdownlint-disable-next-line MD013 -->
- [Historical source-document evidence classification and publication boundary](historical-source-document-evidence-classification-and-publication-boundary.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Gameplay census, presentation, and development-content boundary](gameplay-census-presentation-and-development-boundary.md)
- [Unreal gameplay content catalog](gameplay-content-catalog.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Open sandbox campaign design](../gameplay/open-sandbox-campaign-design.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Multiplayer adapter and community-server extension](../modding/multiplayer-adapter-and-community-server-extension.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Dialogue selection, queue, and playback runtime](dialogue-selection-queue-and-playback-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Device configuration and save-slot runtime](device-configuration-and-save-slot-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Platform save storage and lifecycle](platform-save-storage-and-lifecycle.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Typed event and observation routing runtime](typed-event-and-observation-routing-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Native art authoring, style, and asset validation contract](native-art-authoring-style-and-asset-validation-contract.md)

## Purpose

This specification defines how historical core-design documents, content lists,
flow-chart exports, gameplay-event inventories, character-availability tables,
voice and conversation spreadsheets, and other directly reviewable design
records
become public normalized requirements without turning source-era implementation,
platform, approval, staffing, or file-layout assumptions into target authority.

It covers:

- evidence classification and bounded extraction;
- conflict and supersession policy;
- product goals versus obsolete implementation proposals;
- content-catalog and availability normalization;
- campaign, mission, boss, vehicle, traffic, camera, presentation, input, HUD,
  collision, audio, and save-system evidence;
- rejected head-to-head multiplayer and bullet-time assumptions;
- mod-owned community-server compatibility;
- conversation-sheet normalization;
- empty spreadsheet companion rejection;
- native Unreal ownership and read-back; and
- public-output, validation, diagnostics, and failure boundaries.

The public repository does not publish source documents, source dialogue text,
source filenames, approval matrices, personal working records, production
status,
revision history, author names, workstation paths, platform-certification text,
or
spreadsheet-export residue.

## Evidence classes

Every reviewed record resolves to one closed class:

1. **Product goal** — player-facing intent that may survive after technical and
   product review.
1. **Semantic definition evidence** — candidate identities, roles,
   relationships,
   placements, events, objectives, or content membership.
1. **Runtime-behavior evidence** — candidate state, transition, failure, timing,
   interaction, or presentation rules.
1. **Platform-era evidence** — historical controller, storage, certification,
   memory-card, or console terminology requiring semantic translation.
1. **Implementation proposal** — source-era code, tool, file, editor, or
   data-flow
   idea that is not target architecture.
1. **Superseded mode evidence** — historical gameplay mode or rule explicitly
   replaced by a current accepted decision.
1. **Production or approval metadata** — staffing, ownership, completion,
   review,
   licensing, legal, approval, date, or delivery state; excluded from runtime.
1. **Empty or generated companion** — zero-byte sheet, export residue, or
   generated
   shell with no semantic content; excluded from the ledger.

A document may contribute facts to several public owners, but each extracted
fact
has one terminal classification and one accepted owner.

## Authority hierarchy

Conflicts resolve in this order:

1. newest explicit operator decision;
1. accepted ADR;
1. active public technical specification;
1. validated normalized manifest or catalog revision;
1. reviewed historical design fact;
1. source-era implementation proposal;
1. source filename, row order, folder, formatting, or platform wording.

A lower authority cannot reopen a rejected feature, change a stable identity,
replace native Unreal ownership, or override a newer product boundary.

## Normalization transaction

Core-design intake follows one deterministic transaction:

1. verify the record is directly reviewable text and not excluded
   administration,
   approval, personal, empty, raw-art, binary, or generated evidence;
1. record source class and safe private identity;
1. inventory headings, tables, columns, counts, and cross-references;
1. separate product semantics from revision history, authorship, approvals,
   staffing, dates, paths, and tooling;
1. identify candidate stable identities and aliases;
1. map each fact to an existing public owner or reject it as superseded;
1. compare conflicts against the authority hierarchy;
1. normalize units, timing, platform actions, roles, and state names;
1. produce repository-authored schemas, rules, diagnostics, and tests;
1. verify native Unreal ownership and target-platform policy;
1. publish only reviewed normalized facts; and
1. record a terminal covered, superseded, rejected, or no-public-output result.

The runtime never opens historical design documents or spreadsheets.

## Product goals and implementation proposals

A product goal describes an outcome such as readable navigation, satisfying
vehicle handling, expressive characters, fast mission comprehension, useful
collision feedback, or accessible frontend flow.

An implementation proposal describes one possible historical mechanism such as:

- fixed arrays or numeric slots;
- Maya-authored runtime locators;
- platform-specific button names;
- memory-card slot menus;
- source filenames as event identity;
- hard-coded counts;
- scripted camera cuts stored in source tables;
- source-era physics or traffic managers;
- one console's controller or storage behavior; or
- a head-to-head mode coupled to global slow motion.

The target may preserve the goal while replacing the mechanism with native
Unreal
systems and current repository architecture.

## Content catalogs

Content lists normalize into stable definitions and typed relationship rows.
Source row order, completion columns, model-ready flags, animator names,
approval
state, and free-form production comments are not runtime fields.

### Pedestrian catalog

A pedestrian catalog row may establish:

- canonical character or archetype identity;
- body, palette, costume, or presentation variant;
- walker, driver, passenger, ambient, mission, interior, or unavailable role;
- chapter, location, population, and spawn eligibility;
- voice, animation, collision, navigation, and quality-profile compatibility;
- required native asset bundles; and
- replacement or deprecation state.

A model-ready flag is import-review evidence. It does not spawn a character or
prove that mesh, Skeleton, animation, material, collision, or platform
validation
passed.

### Ambient-gag catalog

An ambient-gag row may establish gag identity, interior or world zone,
participants, temporary props, semantic description, audio and animation
bindings, scheduling, repeatability, cooldown, streaming, quality, and teardown.

Source scene names, audio filenames, animator columns, and status comments are
provenance only. Runtime consumes typed gag definitions and native asset
references.

### Walker and driver availability

Availability evidence normalizes into role bindings by character, vehicle,
mission, chapter, placement, and revision. It distinguishes:

- playable driver;
- forced mission driver;
- passenger;
- walker;
- mission giver;
- ambient pedestrian;
- unavailable role; and
- presentation-only participant.

Historical matrix positions and fixed level or mission strings do not become
identity. A role is accepted only when character, vehicle, mission, animation,
seat, dialogue, collision, and loading requirements resolve.

### Wasp placements

Wasp-location evidence normalizes into stable hazard-spawn definitions
containing
world, chapter, location, placement, transform or anchor, activation,
persistence,
reward reserve, progression, streaming, and validation policy.

A prose location description is review evidence, not a runtime transform.
Destruction remains exactly once by persistent placement identity.

## Core-design records

Broad core-design documents are evidence collections rather than one target
specification. Individual facts are routed to the owning runtime.

### Animation

Animation evidence may establish locomotion, idle, action, reaction, dialogue,
vehicle-handoff, prop, camera, marker, root-motion, interruption, and character-
specific intent. Native Skeleton, Animation Sequence, Animation Blueprint,
Montage, Sync Group, Motion Warping, and catalog contracts remain authoritative.

Historical clip counts, filenames, DCC processes, and fixed body assumptions do
not become target identity.

### Boss encounters

Boss evidence normalizes into one `USharBossEncounterDefinition` per accepted
encounter revision. It declares boss identity, chapter slot, arena, mission,
phases, movement and navigation policy, damage, capture or puzzle conditions,
camera, audio, VFX, failure, reward, persistence, streaming, and teardown.

Historical Truckasaurus proposals are candidate encounter patterns. Claims such
as one boss per level, infinite mass, direct waypoint pursuit, or a fixed number
of encounters remain unaccepted until the current campaign definition includes
them.

### Bullet time and head-to-head modes

Historical bullet-time evidence belongs to a superseded local head-to-head mode.
The base game does not ship that mode, a flag-steal ruleset, or a global slow-
motion attack mechanic.

A future mod-owned mode may define scoped time presentation only when it
declares:

- namespaced mode identity;
- authoritative server or standalone owner;
- affected world, actors, audio, camera, input, physics, animation, and UI;
- replication and prediction policy;
- fairness and accessibility policy;
- start, cancellation, restoration, and disconnect behavior; and
- tests proving no effect on the base campaign or another session.

Global time dilation is never inferred from a historical meter or source event.
The base campaign remains unaffected.

### Camera

Camera evidence normalizes into registered rigs, subjects, targets, framing,
look-ahead, obstruction, collision, transitions, priorities, split-screen or
local
player scope, accessibility, and restoration policy.

Historical camera constants and platform-specific assumptions are tuning
evidence,
not compile-time authority.

### Collectibles and collector cards

Card and collectible evidence is reconciled against the accepted catalog:

- seven card sets;
- seven cards per set;
- one durable collection key per card;
- canonical placement and chapter ownership;
- localized title and detail presentation;
- exactly-once persistence and completion contribution; and
- gallery and reward projection.

Historical ten-card proposals, secret tenth-card rules, Sunday-drive-only rules,
or conflicting totals are superseded unless a future namespaced content package
introduces them without changing base identities.

### Collision and physics

Collision evidence may establish breakable, movable, static, vehicle, character,
trigger, query, physical-material, reset, damage, and presentation intent.
Native Chaos simulation, collision channels, Physics Assets, components, and
validated data profiles remain authority.

Source-era collision managers, infinite-mass shortcuts, custom solvers, and
art-file naming rules do not ship.

### Dialogue system

Dialogue-system evidence may establish conversation, one-liner, participant,
role, event, priority, probability, positional, interruption, subtitle, mouth,
mix, and usage intent. Detailed ownership remains in the dialogue runtime.

Source file naming, short tokens, underscore parsing, source row order, and
hard-
coded event arrays are import evidence only.

### Driving model and vehicles

Driving and vehicle evidence may establish handling goals, vehicle classes,
control response, traction, braking, suspension, wheel, mass, damage, reset,
seat, camera, audio, traffic, artificial-intelligence, mission, and presentation
roles.

Native Chaos vehicle definitions own simulation. Historical vehicle counts,
examples, category names, platform controls, and direct source tuning values are
candidate data only.

### Integrated core design

A broad integrated design document is decomposed by domain. Its product goals
may
inform current campaign, mission, character, vehicle, world, audio, UI, and
presentation contracts. Revision histories, delivery status, authorship,
approval, schedules, feature promises, and obsolete architecture produce no
runtime output.

No broad document can override a newer focused specification.

### External mission and story proposal sets

A non-empty external partner, licensor, publisher, or writer-facing mission and
story framework is semantic proposal evidence. It may contribute candidate:

- chapter themes and narrative beats;
- mission families, objectives, stages, failures, rewards, and presentation;
- playable, forced, supporting, ambient, and cinematic character roles;
- vehicle classes, handling roles, chase or escape use, and mission bindings;
- landmarks, locations, interiors, routes, time-of-day, and streaming scope;
- boss, race, tutorial, collectible, dialogue, camera, audio, and FMV intent;
  and
- explicit technical or content constraints that remain compatible with current
  accepted architecture.

Each source document belongs to one versioned proposal set. Intake records its
safe private identity, revision relation, section or table identity, candidate
fact identities, cross-document conflicts, and terminal decision. Repeated or
combined drafts may collapse into one proposal set; a longer or later-looking
source does not automatically supersede another source or an accepted public
contract.

Conflicting proposals such as seven versus nine levels, five versus eight
controllable characters, different mission-type counts, one boss per level,
fixed character-per-level ownership, or incompatible landmark, interior, and
vehicle allocations remain unresolved until the authority hierarchy selects one
accepted result. They do not change the current seven-chapter open sandbox,
playable roster, canonical content catalog, progression, achievements, or save
schema.

Questions, brainstorming, review comments, legal or approval requests, staffing,
authorship, dates, delivery status, source references, and production notes are
private workflow metadata. A legal-approval working document is excluded from
semantic coverage rather than converted into gameplay content.

Every retained proposal fact ends as accepted, adapted, superseded, rejected, or
unresolved. Only accepted or explicitly adapted facts may generate typed
mission,
chapter, character, vehicle, location, interior, presentation, or catalog
definitions. Unresolved proposal sets publish no partial runtime graph.

### Feature-specific design records

A focused game-feature record may contribute product goals, semantic identities,
state transitions, failure and recovery rules, timing, rewards, presentation,
accessibility, content dependencies, and candidate implementation constraints.
Each retained fact maps to the active campaign, mission, character, vehicle,
artificial-intelligence, economy, collectible, traversal, traffic, navigation,
hazard, save, UI, audio, or presentation owner.

Feature documents do not become a parallel design authority. Repeated integrated
summaries collapse into existing definitions, while hard-coded counts,
source-era
algorithms, platform controls, fixed memory budgets, source commands, and direct
asset filenames remain proposals or provenance. A focused historical record
cannot override a newer domain specification merely because it contains more
detail.

### Visual and art-requirement records

Textual art-requirement evidence may establish semantic icon roles, meter
states,
screen composition, transition purpose, readability, visibility, damage states,
mission-flow presentation, camera intent, and accessibility requirements. It may
also identify required content families and state channels that existing native
assets must represent.

It does not publish source artwork, screenshots, dimensions copied without
current
justification, digital-content-creation instructions, employee assignments,
approval state, asset filenames as identity, or production schedules. Runtime
presentation remains owned by repository-authored UI, camera, VFX, material,
animation, and content definitions.

### High-level, sequel, and future concepts

High-concept, high-level-design, sequel, and future-feature records are
versioned
proposal sets. They may supply candidate themes, player goals, world structure,
combat, traversal, vehicle, mission, multiplayer, progression, presentation, and
accessibility ideas, but they do not expand current product scope automatically.

A revision label, apparent recency, sequel title, repeated idea, or broader
feature
list does not supersede the accepted campaign, single-player base mode,
mod-owned
multiplayer boundary, canonical catalogs, platform policy, or native Unreal
architecture. Every adopted idea requires an explicit current owner and terminal
accepted or adapted decision; all others remain superseded, rejected, or
unresolved proposal evidence.

### Instruction manuals and player-facing documentation

A historical instruction manual may establish low-authority evidence about
player-visible terminology, actions, controls, menus, save behavior, mission and
progression concepts, collectibles, vehicles, characters, hazards, camera,
audio,
UI flow, and expected user feedback. Each fact must be reconciled with active
technical specifications and native read-back before acceptance.

Manual wording never defines target architecture or current platform support.
Source-era hardware requirements, installation steps, platform button labels,
certification wording, publisher placeholders, print-layout instructions, draft
biographies, legal notices, marketing claims, screenshots, and unfinished
markers
remain private, rights-sensitive, superseded, or rejected evidence. Semantic
controls map to current input actions; old storage and platform wording maps to
current provider outcomes only when an active contract accepts it.

### Frontend and platform error flows

Frontend evidence normalizes into Common UI routes, screens, actions, focus,
modal ownership, settings, profile selection, save operations, loading,
accessibility, controller recovery, and terminal results.

Platform memory-card messages normalize into semantic storage outcomes such as:

- provider unavailable;
- user or slot unavailable;
- save missing;
- storage full or quota exceeded;
- data corrupt;
- incompatible revision;
- operation cancelled;
- retryable failure;
- deletion or reset required; and
- continue without saving when product policy permits it.

Physical slots, market formatting, format-card commands, numeric error rows, and
console button names are historical wording only. Native save and platform
storage APIs own the operation.

### Input and gameplay controls

Control evidence normalizes into semantic input actions, mapping contexts,
triggers, modifiers, device capabilities, accessibility, haptics, and local-
player ownership.

Tutorial dialogue and UI display localized action names from the active mapping.
They do not embed obsolete console button names as canonical instructions.

### Gameplay-event inventory

A gameplay-event inventory contributes candidate typed event identities and
participant or payload schemas. It does not create a process-global enum, string
bus, callback table, or implicit gameplay authority.

Every accepted event has one owner, schema revision, publisher policy,
subscriber policy, ordering, replay, lifetime, diagnostics, and networking
scope.

### HUD and navigation

HUD evidence normalizes into local-player viewmodels and registered presentation
profiles for mission state, timers, collectibles, currency, damage, notoriety,
vehicle telemetry, radar, route cues, messages, and transitions.

Navigation evidence normalizes into deterministic road projection, route paths,
intersection cues, wrong-way feedback, marker visibility, settings, streaming,
and stale-revision rejection.

### Jumping and on-foot traversal

Jumping evidence may establish action intent, movement constraints, obstacle and
platform interactions, camera support, animation, collision, and accessibility.
Native Character Movement, movement modes, traces, and authored profiles remain
authoritative.

Historical claims that every character has identical movement or fixed jump
numbers are candidate tuning only.

### Missions and mission scripting

Mission evidence decomposes into mission, stage, objective, participant,
vehicle,
route, timer, failure, restart, reward, dialogue, presentation, world-entity,
interaction, and progression definitions.

Source script commands, Sunday-drive terminology, source objective counts, and
fixed level ordering are evidence only. Runtime consumes validated typed assets
and application transactions.

### Multiplayer

The base campaign is single-player and local-authority. Historical split-screen
or head-to-head designs do not become first-party modes.

The repository provides extension architecture for validated mod-owned modes and
community-hosted servers. A multiplayer package owns separate namespaced mode,
protocol, authority, replication, discovery, administration, persistence, and
compatibility definitions. It cannot reinterpret base campaign saves,
achievements, mission checkpoints, or progression.

### Presentation

Presentation evidence may establish transition purpose, camera composition,
length, skip policy, letterbox, iris, fade, music, audio, animation, UI, and
accessibility intent. Native playback, camera, Common UI, Sequencer, and audio
systems own execution.

Historical style references and fixed durations remain review evidence unless an
accepted presentation definition adopts them.

### Character and voice lists

Character-list evidence may establish candidate canonical identity, alias,
walker, driver, passenger, mission, ambient, interior, voice, and availability
relationships.

Voice actor names, redo status, totals, approval columns, and production notes
do
not become runtime character fields. Public credits require the separate legal
and attribution review process.

### Sound and story

Sound evidence may establish dialogue, music, ambience, vehicle, collision, UI,
mission, cinematic, mix, spatial, concurrency, localization, and streaming
intent.
Native audio assets and runtime contracts remain authority.

Story evidence may establish narrative goals, chapter themes, character roles,
mission context, and presentation intent after reconciliation with the accepted
campaign. Draft plot ordering and unused missions do not create progression.

### Vehicle switching

Vehicle-switching evidence normalizes into access eligibility, driver and
passenger roles, seat and door identities, speed and obstruction checks,
entry/exit choreography, possession, collision, camera, dialogue, restoration,
and persistence.

Historical button names, fixed speed thresholds, and source locator workflows
are
candidate tuning only.

### Traffic, chase, escape, and boss artificial intelligence

Artificial-intelligence evidence may establish route goals, target policy,
follow, chase, evade, attack, guard, return, disable, catch-up, difficulty,
spawn, reservation, obstacle, damage, failure, reward, and presentation intent.

Native StateTree, Mass or actor-backed population where selected, navigation,
road graphs, Chaos vehicles, scene queries, and typed application ports own
execution. Historical claims such as ignoring traffic rules, one fixed boss,
infinite mass, or source-authored runtime paths require explicit current
definitions.

## Conversation spreadsheet normalization

A non-empty conversation sheet is semantic dialogue evidence. It is converted
into typed line and conversation definitions through a deterministic importer.

### Source-column mapping

Source columns may contribute:

- location or mission context;
- candidate event or conversation label;
- speaker identity from the owning sheet;
- dialogue text for private comparison and localization intake;
- source or reuse classification;
- candidate audio identity;
- legacy audio alias; and
- private approval or receipt state.

Only reviewed normalized semantics and approved public metadata leave the
private
intake boundary.

### Stable line identity

A line identity is derived from explicit semantic fields and a versioned
mapping,
not source row number or filename. It includes speaker, event, context,
conversation membership, ordinal, role, variant, locale, and definition revision
where applicable.

Duplicate source rows may resolve to one line, several contextual bindings, or a
rejected conflict. Empty rows and heading rows create no line.

### Conversation identity and order

A conversation definition declares participants, context, ordered required and
optional lines, start and completion policy, interruption, restart, positional
policy, subtitles, mouth presentation, audio mix, and fallback.

Source ordering is evidence but must be validated against explicit ordinals and
participant roles before publication.

### Event and context mapping

Mission, tutorial, reward, collectible, vehicle, interior, ambient, cinematic,
and system contexts map to typed event identities. Short labels and source
filenames are aliases only.

Platform-specific tutorial variants map to one semantic tutorial action plus
platform or input-presentation conditions. Current localized action names come
from the active input mapping.

### Audio and locale binding

Audio aliases resolve to normalized audio manifests and native assets. Missing,
duplicate, ambiguous, or incompatible audio blocks the required line.

Dialogue text enters the localization pipeline under a stable key. It is not
copied into gameplay code or used as line identity. Dialogue Waves may carry
spoken text, subtitle overrides, contexts, and recording guidance, while the
project dialogue runtime owns conversation selection and queue behavior.

### Approval and rights metadata

Approval, received-from-licensor, source, recording, delivery, and legal-review
columns remain private workflow metadata. They can block publication but cannot
select a runtime line, grant content, or become packaged diagnostics.

### Empty companion sheets

Zero-byte sheets and generated empty companions are excluded from semantic
coverage. The ledger policy audits and prunes them deterministically so they
cannot reappear as pending work.

## Sound-event spreadsheet normalization

A non-empty sound-event sheet is semantic dialogue-selection evidence for one
canonical character, generic population archetype, vehicle archetype, location,
or presentation owner. It does not execute as a runtime table.

### Sound-event source-column mapping

Source columns may contribute:

- candidate event label;
- candidate line or selection-member label;
- private dialogue text for comparison and localization intake;
- source or reuse classification;
- candidate priority token;
- current and legacy audio aliases; and
- private approval, receipt, recording, or delivery state.

The owning sheet name is provenance only. Import resolves it through the
canonical character, archetype, vehicle, location, or presentation catalog
before
publishing any line or event binding.

### Event alias normalization

Raw labels are compared case-insensitively after whitespace, punctuation, and
known spelling normalization. Variants such as `doorbell dialog` and `doorbell
dialogue`, or corrected forms of historical typographical errors, may map to one
registered semantic event alias.

Labels such as `none`, `(none)`, `none yet`, prose instructions, category notes,
or combined implementation commentary do not create runtime events. They are
rejected, classified as review notes, or split only through an explicit reviewed
mapping. A source label cannot silently create a new channel because it appears
in several sheets.

Every retained event mapping declares:

- canonical event and alias identities;
- owning gameplay or presentation domain;
- required speaker or archetype role;
- participant and world-context requirements;
- positional or non-positional policy;
- candidate line or selection-group membership;
- priority, interruption, lifetime, concurrency, and repeat policy;
- locale, subtitle, audio, and fallback requirements;
- diagnostic coverage state; and
- definition and source-mapping revisions.

### Priority normalization

The historical priority tokens map through one closed import table:

| Source token | Semantic priority |
| :--- | :--- |
| `MPI` | `must_play_immediately` |
| `MPL` | `must_play` |
| `SPL` | `high` or another explicitly reviewed bounded must-consider class |
| `OPL` | `occasional` or another explicitly reviewed optional class |

The exact `SPL` and `OPL` targets are versioned mapping decisions because source
sheets used them inconsistently across event families. A source token never
becomes a numeric queue priority directly.

Blank priority is accepted only when the event binding supplies an explicit
default. Prose legends, audio identifiers, misspelled instructions, mixed
tokens,
or unknown values in the priority column fail import or require an explicit
mapping. `MPI` does not bypass gameplay authority; it changes queue and
interruption policy only after the semantic event has already been accepted.

### Source and reuse classification

Historical source tokens such as new, reused, prior-title, written, or another
short code remain provenance and rights-review evidence. They may select a
private review path or audio-reuse check but cannot change event identity,
priority, probability, gameplay outcome, or participant eligibility.

### Coverage matrix

Import publishes a development-only coverage matrix by canonical speaker or
archetype and canonical event. It records required, optional, mapped, missing,
rejected, duplicate, fallback, and approved states without packaging source text
or production metadata.

Coverage does not force playback, mark a line as used, change probability,
upgrade priority, or make an optional event required. Generic population sheets
bind to archetypes rather than creating named characters. Vehicle and location
sheets bind to their registered presentation owners rather than inventing a
speaker identity.

## Public outputs

Allowed public outputs include:

- repository-authored schemas and specifications;
- stable semantic identities and aliases;
- normalized counts and completeness summaries;
- accepted compatibility and supersession decisions;
- deterministic import and validation rules;
- native Unreal ownership boundaries;
- tests and diagnostics; and
- approved public attribution or provenance records through their separate
  review process.

Prohibited public outputs include:

- source documents or converted copies;
- source dialogue text or raw rows;
- source audio filenames and private approval states;
- personal names from working records;
- revision histories and author metadata;
- source paths and workstation information;
- legal or approval matrices;
- empty sheet inventories;
- source screenshots; and
- obsolete platform instructions presented as current requirements.

## Validation

Validation proves:

- every retained record is eligible semantic text;
- excluded administration, approval, personal, empty, raw-art, and binary
  evidence
  is absent from the ledger;
- every extracted fact has one class and terminal owner;
- conflicts follow the authority hierarchy;
- every external proposal fact belongs to one versioned proposal set and has one
  terminal accepted, adapted, superseded, rejected, or unresolved result;
- conflicting level, character, mission, boss, landmark, interior, vehicle, and
  story allocations cannot alter accepted campaign or catalog state implicitly;
- feature-specific facts resolve to active domain owners without creating a
  parallel authority graph;
- visual requirements resolve to semantic presentation states without publishing
  source artwork or production metadata;
- high-level and sequel concepts remain proposal sets unless explicitly adopted;
- manual-derived facts exclude old hardware, installation, platform, layout,
  publisher-placeholder, biography, legal, marketing, and unfinished content;
- superseded modes cannot enter base-game catalogs;
- content identities and aliases are unique;
- campaign, mission, boss, character, vehicle, pedestrian, gag, hazard, event,
  dialogue, save, input, camera, audio, UI, and presentation references resolve;
- dialogue participants, context, ordinals, audio, locale, subtitles, and
  fallback are complete;
- every retained sound-event alias resolves uniquely to one canonical event and
  owner;
- every source priority token maps through the closed versioned priority table;
- placeholders, prose cells, malformed priorities, and ambiguous speaker owners
  produce no runtime binding;
- sound-event coverage matrices agree with the accepted line, event, archetype,
  audio, locale, and fallback catalogs;
- approval and production metadata is absent from runtime assets;
- platform-specific actions map to semantic input and storage outcomes;
- generated output is deterministic; and
- native read-back matches accepted definitions.

## Failure behavior

Normalization fails closed on ambiguous identity, unresolved conflict, missing
owner, unsupported feature revival, malformed source structure, duplicate or
missing dialogue context, unresolved audio, invalid locale, unknown event alias,
placeholder event, malformed or unknown priority token, ambiguous archetype
binding, personal or approval metadata leakage, stale catalog revision,
nondeterministic output, or native read-back mismatch.

Failure publishes no partial catalog, dialogue set, event schema, mode,
save-flow,
or runtime definition. The previous accepted revision remains active.

## Tests

Automated tests cover:

- eligible semantic records and excluded approval, nickname, raw-asset, binary,
  and zero-byte records;
- deterministic pruning and idempotence;
- product-goal versus implementation-proposal classification;
- authority conflict and supersession;
- external mission/story proposal-set grouping, duplicate-draft collapse,
  cross-document conflict, accepted adaptation, rejection, unresolved isolation,
  and zero partial publication;
- feature-specific owner routing and duplicate integrated-summary collapse;
- visual-requirement state extraction with source-art and production-data
  exclusion;
- high-level, sequel, and future-concept adoption and supersession;
- instruction-manual semantic extraction, platform translation, placeholder
  rejection, and current-specification reconciliation;
- rejected head-to-head and bullet-time base modes;
- mod-owned multiplayer namespace isolation;
- card-count conflict reconciliation;
- platform storage-error normalization;
- semantic input-action normalization;
- pedestrian, gag, role, hazard, character, and vehicle catalog conversion;
- gameplay-event schema generation;
- dialogue row, heading, duplicate, context, participant, ordinal, audio,
  locale, and approval-column handling;
- sound-event alias normalization, placeholder rejection, speaker and archetype
  ownership, source-priority mapping, malformed priority rejection, source and
  reuse provenance, and coverage-matrix generation;
- native asset and localization read-back; and
- repeated import with zero semantic diff.

## Invariants

- Raw and empty artifacts do not receive semantic coverage rows.
- Source documents never execute or ship.
- Product goals do not preserve obsolete implementation automatically.
- Newer accepted decisions supersede historical counts and modes.
- External proposal sets never override accepted campaign or catalog state by
  document length, apparent recency, repetition, or stakeholder origin.
- Unresolved external proposals publish no partial runtime definitions.
- The base campaign remains single-player.
- Multiplayer capability remains a mod-owned extension boundary.
- Historical bullet time is not a base-game feature.
- Platform storage and input wording normalize to semantic outcomes and actions.
- Conversation identity is independent of row number and source filename.
- Sound-event identity is independent of sheet name, raw label spelling, source
  token, and row position.
- Raw priority tokens map only through the closed versioned import table.
- Coverage diagnostics never change queue selection or playback state.
- Approval, staffing, revision, and personal metadata never become runtime
  state.
- Every accepted fact has one public owner and one validation method.
- Native Unreal systems remain runtime authority.
- Failed normalization leaves the previous accepted revision unchanged.
