# Historical source-document evidence classification and publication boundary

- Status: Active
- Last reviewed: 2026-07-17

## Governing decisions and records

<!-- markdownlint-disable-next-line MD013 -->
- [Runtime parity boundary](../../adr/unreal/runtime/remake-parity-boundary.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Staged mesh import and world assembly](../../adr/unreal/import-adapters/staged-mesh-import-and-world-assembly.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Converted asset ingestion boundary](../../adr/unreal/import-adapters/converted-asset-ingestion-boundary.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Native asset translation without copy-paste](../../adr/pipeline/unreal/native-asset-translation-and-no-copy-paste.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Copyright-safe publication boundary](../../legal/repository/copyright-safe-publication-boundary.md)
- [Privacy and personal
  data](../../legal/doctrines/privacy-and-personal-data.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Native import, material rebuild, and world assembly](native-import-material-and-world-assembly.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Native art authoring, style, and asset validation contract](native-art-authoring-style-and-asset-validation-contract.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Progression, collectibles, cheats, and credits](progression-collectibles-and-cheats.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Authored state-prop animation and event runtime](authored-state-prop-animation-and-event-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Playable avatar, character controller, and footprint runtime](playable-avatar-character-controller-and-footprint-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Character animation clip catalog and vehicle-handoff choreography runtime](character-animation-clip-catalog-and-vehicle-handoff-choreography-runtime.md)
- [Animation clip timing](../fbx/animation/clip-timing.md)
- [Animation rig model](../fbx/animation/rig-model.md)

## Purpose

This specification defines how historical source documents, administrative
records, technical art guides, attribution lists, gameplay notes, generated
companion files, and digital-content-creation scenes may be inspected as private
reconstruction evidence without becoming public repository content or packaged
runtime authority.

Historical evidence is not accepted merely because it exists, is old, is
technically readable, or accompanied the source tree. Every file receives an
explicit classification, purpose, extraction boundary, privacy decision,
publication decision, normalized output, and verification result.

The public repository contains repository-authored specifications, normalized
facts, validated schemas, tests, and independently created native assets. It
does
not reproduce historical documents, employee schedules, meal preferences,
contact details, office administration, source-era prose, screenshots, embedded
images, obsolete paths, or digital-content-creation scene text.

## Ownership

<!-- markdownlint-disable MD013 -->

| Authority | Responsibility |
| :--- | :--- |
| Evidence intake | Inventories private evidence and records cryptographic identity, media type, size, and review state. |
| Privacy review | Classifies personal, employment, schedule, contact, account, and other sensitive information. |
| Publication review | Decides whether any derived fact may enter a public specification, test, catalog, or attribution record. |
| Technical reviewer | Extracts behavior, geometry, animation, workflow, naming, timing, or platform facts needed by a bounded reconstruction task. |
| Legal records | Define publication, attribution, copyright, privacy, retention, and counsel-escalation boundaries. |
| Import pipeline | Consumes only normalized technical inputs and deterministic conversion recipes. |
| Runtime specifications | Own target behavior and native Unreal architecture. Historical documents cannot override accepted public decisions. |
| Validation | Rejects prohibited source text, personal data, private paths, unreviewed names, and unsupported claims from public artifacts. |

<!-- markdownlint-enable MD013 -->

No reviewer may use private evidence as a shortcut around the repository's
architecture, legal, privacy, authorship, or validation policies.

## Evidence identity

Each reviewed item has a private evidence identity separate from public runtime
identity. The private review record contains:

- opaque evidence identity;
- cryptographic digest;
- media type and encoding;
- byte size and structural summary;
- source category;
- review purpose;
- privacy classification;
- copyright and publication classification;
- technical relevance classification;
- extracted fact identities;
- normalized output identities;
- reviewer and review timestamp in private audit state;
- retention and deletion policy; and
- terminal review result.

The public repository does not expose private evidence identifiers, source-tree
paths, digests, reviewer identities, employee identities, machine paths, or
chain-of-custody locations.

## Closed evidence classes

Every item is assigned exactly one primary class and any required secondary
risk tags.

### Production administration

Production-administration evidence includes meal orders, vacation calendars,
staff schedules, contact sheets, office logistics, internal attendance records,
and similar non-product material.

This class is private and non-runtime. It may establish only high-level facts
such as the existence of an historical production period when that fact is
material and independently safe to state. Individual names, dates, absences,
preferences, orders, notes, contact details, group assignments, and formatting
are not extracted into the public repository.

Production-administration evidence cannot:

- define gameplay, content, architecture, schedules, staffing, credits, or
  release dates;
- become test fixtures or sample data;
- populate named-entity dictionaries;
- appear in screenshots or generated reports;
- be summarized person by person;
- be committed in transformed form; or
- be retained merely because it accompanied technical evidence.

A companion stylesheet, frame document, image, script, or generated index used
only to present an administrative record inherits the same private exclusion.

### Attribution and credits candidates

Historical credits lists are candidate attribution evidence, not an automatic
public credits database.

A credits review separates:

- product title and publisher or developer organization candidates;
- role category candidates;
- individual-name candidates;
- licensed technology or service candidates;
- music, performance, writing, and other rights-sensitive candidates;
- special-thanks text;
- trademarks and notices; and
- formatting or ordering that may itself be authored expression.

Public credits data is created only through a curated attribution process that
verifies the exact name, spelling, role, organization, ordering requirement,
license notice, source authority, and publication basis. The target credits
sequence consumes repository-authored localized rows and presentation styles.
It does not parse or display a historical text file at runtime.

Unverified names remain private evidence. Historical ordering, prose, special-
thanks wording, and formatting are not copied by default.

### Gameplay notes and cheat tables

Historical gameplay notes may establish candidate identities, effects, logical
relationships, or platform mappings. They do not become executable scripts or
runtime text tables.

Cheat evidence is normalized into:

- stable cheat identity;
- semantic effect kind;
- exactly defined semantic input tokens;
- platform input-profile projections;
- prerequisite and availability policy;
- activation and lifetime policy;
- typed effect parameters;
- feedback identity; and
- verification evidence.

Platform button labels are presentation mappings. A source-era numeric cheat
identifier, file order, platform key code, prose description, spacing, or table
format is not target identity.

A cheat that mutates persistent state must use the same typed domain transaction
as ordinary gameplay and retain an explicit cheat-origin marker.

### Technical art and world-building guidance

Historical art and world-building documents may provide candidate technical
facts about:

- world decomposition;
- roads, tracks, terrain, landscape, landmarks, and level boundaries;
- static, dynamic, animated, breakable, or stateful props;
- naming and semantic component roles;
- placement, collision, pivots, materials, animation, and export intent;
- visual hierarchy and authored relationships;
- source workflow dependencies;
- quality goals and known limitations; and
- postmortem observations.

The reviewer extracts normalized constraints and compares them against actual
assets, runtime behavior, independent tests, and accepted architecture.
Historical prose, screenshots, embedded media, tool-specific instructions,
absolute paths, source control directions, employee assignments, estimates, and
obsolete exporters are not copied into target documentation.

A historical workflow instruction is not binding merely because it was written
for the source production. The target uses current native Unreal, Maya, Blender,
FBX, validation, and repository policies.

### Generated companion files

Generated HTML frames, headers, stylesheets, file lists, metadata fragments,
Office export support files, and similar companions have no independent product
meaning unless a technical review proves otherwise.

They may establish:

- document composition;
- media relationships;
- character encoding;
- section ordering;
- embedded-resource presence; or
- evidence completeness.

They cannot define gameplay, architecture, credits, schedules, dates, content
identity, or runtime presentation merely because a browser used them to render a
historical document.

### Digital-content-creation animation scenes

A source animation scene is private technical evidence and a conversion input,
not a shipping asset and not public documentation.

The review may derive:

- canonical character and clip identity;
- source time unit and frame rate;
- authored start and end frame;
- animated track set;
- translation, rotation, scale, and custom curve presence;
- root, motion, and skeleton compatibility evidence;
- clip category such as locomotion, reaction, fall, landing, recovery, idle,
  dialogue, or vehicle impact;
- looping, hold, transition, root-motion, additive, and montage policy;
- required animation events and notifies;
- import warnings and remediation decisions; and
- deterministic output digest and read-back results.

The target conversion produces validated FBX or another approved normalized
interchange artifact, then native Skeleton, Animation Sequence, Montage,
Animation Blueprint, notify, curve, and metadata assets as required.

The source scene text, source node names that are not canonical public
identities, private paths, workspace metadata, editor preferences, unused nodes,
backup labels, and comments are not published.

Pose scenes, backup copies, reduced-track experiments, batch commands, and
animation-choice or choreography configuration receive separate evidence
classifications. They may establish pose purpose, chronological comparison,
intentional channel reduction, conversion intent, rig roles, locomotion groups,
blend policy, priorities, foot-plant intent, or vehicle-handoff variants only
after normalization through
<!-- markdownlint-disable-next-line MD013 -->
[Character animation clip catalog and vehicle-handoff choreography runtime](character-animation-clip-catalog-and-vehicle-handoff-choreography-runtime.md).
They are never executed, imported by line order, or treated as canonical merely
because they accompanied animation scenes.

## Animation conversion contract

Animation conversion follows a deterministic transaction:

1. identify the canonical character, skeleton, and clip;
1. verify that the evidence file belongs to the declared character and clip;
1. record source time unit, authored frame interval, and animated-track set;
1. reject non-finite keys, unsupported controllers, missing required tracks,
   ambiguous roots, or incompatible skeleton topology;
1. normalize units, axes, root policy, clip range, and key sampling through a
   versioned recipe;
1. export one bounded animation payload with no unrelated scene content;
1. import against the exact native Skeleton and import profile;
1. create or update one canonical Animation Sequence;
1. create Montage sections, slots, notifies, curves, or transition metadata only
   when the public character contract requires them;
1. read back duration, sample rate, track names, root motion, curves, notifies,
   compression, skeleton binding, and asset dependencies;
1. compare output against the accepted clip contract and tolerances; and
1. publish the normalized asset and provenance row only after validation passes.

A backup-directory name or source filename is evidence, not canonical clip
identity. Alias normalization must be explicit and collision-tested.

## Clip timing

Source time units are converted to exact rational rates. A nominal frame label
is not enough; the recipe records numerator, denominator, start frame, end
frame, inclusive or exclusive endpoint policy, and expected duration.

The output must preserve:

- authored ordering;
- intended holds;
- contact and impact timing;
- fall, landing, and recovery boundaries;
- transition poses;
- loop closure when required;
- event and notify positions; and
- root-motion displacement within declared tolerances.

Render frame rate, editor playback settings, time dilation, or preview speed
cannot redefine clip duration.

## Skeleton and track compatibility

Animation-only import targets one exact native Skeleton revision. Every required
track must resolve to one canonical bone or approved curve. Missing required
tracks, duplicate normalized names, hierarchy mismatch, incompatible bind state,
unsupported scale animation, or ambiguous root ownership fails closed.

Optional source tracks may be ignored only through a versioned allowlist with a
reason. Import never chooses a skeleton by display name or first loaded asset.

A clip does not create a new canonical skeleton merely because the source scene
contains enough track names to infer one.

## Art-document extraction contract

Technical art extraction creates a fact set, not a rewritten source document.
Each fact contains:

- stable fact identity;
- source class;
- concise repository-authored statement;
- affected target domain;
- confidence and corroboration state;
- supersession state;
- public or private classification;
- accepted specification or ADR owner; and
- verification method.

Facts are grouped by target concern such as world assembly, placement,
collision,
materials, state props, animation, naming, streaming, rendering, or validation.
Accepted art facts are normalized through
<!-- markdownlint-disable-next-line MD013 -->
[Native art authoring, style, and asset validation contract](native-art-authoring-style-and-asset-validation-contract.md)
before import. Historical polygon counts, folder rules, workstation steps,
exporter settings, platform viewers, completion checklists, and naming
conventions
remain evidence until a typed authoring profile and native validation rule
accepts
or supersedes them.

A fact with no public owner remains private research and cannot be treated as a
shipping requirement.

## Administrative and personal-data minimization

The intake process assumes that production-administration records may contain
personal data even when they contain no email address or account identifier.
Names combined with schedules, absences, food preferences, team membership,
location, dates, or employment context may identify individuals.

The minimum public result for this class is normally no content at all. When a
high-level historical fact is necessary, it is written without identifying a
person, revealing a schedule, reproducing an order, or preserving the source
format.

Personal-data review includes:

- names and initials;
- employment roles and teams;
- schedule and absence information;
- dates tied to individuals;
- meal, health, accessibility, or preference information;
- contact or account identifiers;
- office and location information;
- comments or annotations;
- document metadata and authorship fields; and
- hidden, embedded, or generated companion content.

Detection is a review aid, not proof of anonymity. A file with no email address
may still contain personal data.

## Public publication classes

A derived result receives one publication class:

- `public_specification_fact`, for a repository-authored technical or behavioral
  statement;
- `public_normalized_schema`, for a target data contract;
- `public_test_vector`, only when independently authored and free of protected
  or
  personal source content;
- `public_attribution_candidate`, pending exact verification and legal review;
- `private_technical_evidence`, retained only in the approved private evidence
  environment;
- `private_personal_or_administrative`, excluded from publication and minimized;
- `private_rights_sensitive`, requiring further authority review;
- `superseded_or_irrelevant`, retained or deleted according to policy; or
- `rejected`, for malformed, duplicate, unsafe, or unsupported evidence.

A publication class is explicit. Absence of a class means publication is
forbidden.

## Allowed public outputs

Subject to the governing legal records, allowed outputs include:

- original architecture and runtime specifications;
- stable semantic identities;
- normalized field schemas;
- unit-labelled numeric constraints when necessary and independently justified;
- deterministic conversion recipes;
- validation requirements;
- aggregate counts that do not reveal protected or personal content;
- independently authored diagrams;
- native Unreal asset identities and read-back evidence;
- concise provenance statements that do not expose non-public evidence; and
- verified attribution rows approved for the credits surface.

## Prohibited public outputs

The public repository must not contain:

- historical document files or converted copies;
- substantial source prose;
- source screenshots or embedded images;
- employee or contractor schedules;
- meal orders or preferences;
- unreviewed names, roles, or contact details;
- private evidence paths or digests;
- source-era absolute paths;
- Office-generated companion bundles;
- digital-content-creation scene text;
- source animation curves or node dumps;
- copyrighted credits formatting or special-thanks prose copied wholesale;
- obsolete source control, exporter, or workstation instructions;
- hidden document metadata;
- test fixtures derived from personal records; or
- claims whose only support is an unclassified historical file.

## Runtime boundary

Packaged runtime never opens a historical document, RTF file, HTML export,
stylesheet, office metadata file, Maya scene, administrative text file, or
source
credits file.

Runtime consumes only validated native assets and repository-owned data such as:

- Data Assets and Data Tables;
- localized text identities;
- input mappings;
- credits sequence rows;
- cheat definitions;
- Animation Sequences and Montages;
- state-prop definitions;
- world, placement, road, material, collision, and streaming definitions;
- platform error definitions; and
- provenance records safe for the public package.

## Credits publication boundary

Credits are both presentation content and attribution evidence. The public
credits catalog must distinguish:

- required legal notices;
- verified individual credits;
- verified organization credits;
- licensed technology notices;
- music and performance notices;
- trademarks;
- repository contributors whose inclusion is permitted by policy; and
- optional acknowledgements.

Every row has a source authority, verification date, exact spelling, locale
policy, ordering policy, and rights classification. A historical source list is
one candidate source among others. It cannot silently overwrite a verified row
or introduce an unreviewed person.

## Cheat-evidence boundary

Historical cheat tables may verify platform-era physical labels, but the target
catalog stores semantic tokens. Platform profiles project those tokens to
current
supported controls and glyphs.

The target may intentionally preserve an observable sequence when the public
parity contract requires it. The implementation remains data-driven and local-
player scoped. It does not retain source table order, numeric source
identifiers,
or a platform-specific gameplay branch.

## Art-guide supersession

A source art guide is superseded whenever an accepted ADR or technical
specification defines a different target approach. Examples include native
Unreal Actors and components, World Partition, Chaos physics, Niagara,
Animation Blueprints, Asset Manager bundles, current FBX conversion, and current
repository validation.

Supersession does not erase the source fact. The private review records that the
fact was considered and identifies the public decision that replaced it.

## Companion-file inheritance

A companion file inherits the strictest classification of the document it
supports unless an independent review proves a separate purpose.

Examples include:

- a calendar stylesheet inheriting private administrative classification;
- a generated frame page inheriting the parent document's classification;
- a file-list XML document inheriting technical-art evidence classification;
- an HTML header fragment inheriting the parent art guide's publication limits;
  and
- an embedded image inheriting copyright, privacy, and technical relevance from
  both its own content and the parent document.

## Retention and deletion

Private evidence retention is purpose-limited. The review records:

- why the item is needed;
- who or what process may access it;
- the minimum retention period;
- whether a normalized output replaces the need for the source;
- whether legal preservation applies;
- deletion or quarantine result; and
- whether backups or generated copies exist.

Completion of a public specification does not automatically justify indefinite
retention of personal or administrative evidence.

## Concurrency and review isolation

Evidence review is read-only. Multiple reviews may run concurrently when their
private records and public target files do not overlap.

A reviewer cannot modify private evidence, normalize it in place, rename it to
hide its origin, or stage it in the public worktree. Temporary derivatives stay
inside approved private or ignored locations and are removed according to the
retention policy.

## Failure behavior

The review fails closed when:

- evidence class is unknown;
- privacy classification is incomplete;
- public purpose is absent;
- source authority is insufficient;
- technical meaning is ambiguous;
- a candidate fact conflicts with accepted public architecture;
- a source animation targets an incompatible skeleton;
- clip timing cannot be established;
- companion files are missing and completeness matters;
- extraction would require publishing protected or personal content;
- attribution cannot be verified;
- a generated output contains private paths or source text; or
- validation cannot prove the public boundary.

Failure leaves the public repository unchanged and records a private terminal
result.

## Diagnostics

Public diagnostics may report aggregate review state such as:

- number of items by evidence class;
- number accepted, rejected, superseded, or awaiting review;
- number of normalized outputs;
- number of public facts by target domain;
- animation conversion pass or failure counts;
- missing public-owner counts; and
- privacy or publication blockers by closed reason code.

Diagnostics cannot print private paths, names, schedules, document text, source
curves, or embedded metadata.

## Validation

Validation proves:

- every reviewed historical item has one primary class;
- every public fact has one accepted public owner;
- every publication decision is explicit;
- administrative and personal-data classes produced no prohibited public output;
- companion files inherited a valid classification;
- credits rows are verified and source-attributed;
- cheat rows use semantic tokens and typed effects;
- art facts are normalized and supersession-aware;
- source animation scenes produced deterministic bounded outputs;
- animation tracks match the target Skeleton;
- clip frame rate, range, duration, curves, and root policy read back correctly;
- no public file contains private evidence identifiers, private paths, hidden
  metadata, copied source prose, or personal details;
- no packaged runtime references historical document formats; and
- public Markdown, spelling, links, legal hygiene, and repository validation
  pass.

## Tests

Required automated or review tests include:

- administrative-record classification and zero-public-output tests;
- companion-file inheritance tests;
- personal-data detector fixtures built from synthetic data only;
- publication-class omission rejection;
- copied-source-text detection;
- private-path and evidence-identifier detection;
- credits candidate versus verified-row separation;
- semantic cheat-token normalization;
- art-fact owner and supersession validation;
- source animation time-unit and frame-range extraction;
- skeleton-track compatibility;
- deterministic animation export and import read-back;
- stale normalized-output rejection;
- unsupported or malformed document failure;
- runtime package scan for historical document formats; and
- teardown of temporary conversion outputs.

## Invariants

- Historical evidence is private by default.
- Production administration never becomes gameplay or public sample data.
- Personal data is minimized even when a file is old or locally available.
- Credits require row-level verification before publication.
- Cheat evidence becomes semantic data, not executable source text.
- Art guidance becomes normalized facts, not copied prose or screenshots.
- Companion files inherit the strictest applicable classification.
- Source animation scenes never ship and never become public documentation.
- Native animation assets must pass skeleton, timing, curve, and root-policy
  read-back.
- Accepted public ADRs and specifications override obsolete source workflows.
- Runtime consumes only validated native assets and repository-owned data.
- A private evidence path, digest, name, schedule, order, or source scene dump
  never appears in the public repository.
