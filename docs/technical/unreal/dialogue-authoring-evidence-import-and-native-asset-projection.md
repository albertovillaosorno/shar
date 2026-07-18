# Dialogue authoring evidence import and native asset projection

- Status: Active
- Last reviewed: 2026-07-17

## Governing decisions and specifications

<!-- markdownlint-disable-next-line MD013 -->
- [Hexagonal Unreal runtime](../../adr/unreal/architecture/hexagonal-runtime-and-no-technical-debt.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Converted asset ingestion boundary](../../adr/unreal/import-adapters/converted-asset-ingestion-boundary.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Native gameplay audio, dialogue, and listener boundary](../../adr/unreal/runtime/native-gameplay-audio-dialogue-and-listener-boundary.md)
<!-- markdownlint-disable-next-line MD013 -->
- [State-driven missions, interactions, interiors, and notoriety](../../adr/unreal/runtime/state-driven-missions-interactions-and-notoriety.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Historical source-document evidence classification and publication boundary](historical-source-document-evidence-classification-and-publication-boundary.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Historical core-design and dialogue evidence normalization](historical-core-design-and-dialogue-evidence-normalization.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Dialogue selection, queue, and playback runtime](dialogue-selection-queue-and-playback-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Native cooked-asset construction and registration runtime](native-cooked-asset-construction-and-registration-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Normalized language interchange](../localization/normalized-language-interchange.md)

## Purpose

This specification defines the deterministic editor-time boundary that converts
reviewed dialogue-authoring evidence into repository-owned semantic records and
native Unreal assets. It covers character and archetype event tables, race and
tutorial dialogue scripts, complete campaign dialogue scripts, row and line
classification, owner resolution, event aliases, context segmentation, priority
normalization, line and conversation construction, localization projection,
native asset creation, validation, read-back, diagnostics, and rollback.

The importer exists because historical authoring records are structurally
inconsistent even when their semantics are useful. A valid record may use a
canonical header or begin with a role section, may contain repeated headings and
legends inside the data region, may omit optional audio aliases, may include an
extra provenance-only column, or may be a single-heading text export containing
many missions, races, tutorials, interiors, phone interactions, and speaker
turns. Runtime code cannot compensate for those inconsistencies.

The boundary produces a complete accepted revision or no revision. It never
ships source tables, opens authoring evidence at runtime, copies private
dialogue into gameplay code, infers behavior from filenames, or turns
production metadata into game authority.

## Scope

The importer accepts only evidence that has already passed the historical
publication boundary and has one declared semantic owner. Supported source
families are:

- event tables owned by a canonical named speaker;
- event tables owned by a canonical population archetype;
- event tables owned by another registered presentation owner;
- bounded race-dialogue scripts;
- bounded tutorial-dialogue scripts; and
- complete campaign-dialogue scripts containing multiple gameplay contexts.

The importer does not establish rights, approve attribution, select
performers, confirm recorded media delivery, define mission outcomes, create
gameplay events, or decide whether a historical feature belongs in the current
game. Those are upstream review or owning-domain responsibilities.

## Architectural boundary

Dialogue evidence import is an editor and build-pipeline operation. It is not a
shipping-runtime service.

The boundary is divided into four responsibilities:

1. **Evidence adapter** — reads one reviewed normalized text input, preserves
   record boundaries, and emits lossless private parse tokens.
1. **Normalization application service** — classifies every token, resolves
   owners and aliases, constructs semantic definitions, and produces a complete
   immutable import plan.
1. **Unreal editor adapter** — materializes or updates native assets in a
   disposable staging namespace, runs native validation, and publishes the
   accepted asset revision atomically.
1. **Runtime catalog reader** — reads only cooked repository-owned assets
   through the Asset Manager and dialogue catalog. It has no dependency on
   parsers, authoring columns, source documents, or editor modules.

The future C++ implementation must place editor-only parsers, commandlets,
validators, factories, and asset writers in an editor or developer module. The
shipping runtime module may own data classes and read-only catalog interfaces,
but it must not depend on editor modules, source parsers, CSV libraries,
filesystem discovery, private review state, or authoring-document schemas.

This specification fixes the dependency direction and responsibility boundary.
It deliberately does not assign a module name before the repository adopts a
complete Unreal module map.

## Import transaction

One import request follows this transaction:

1. resolve one reviewed evidence identity and its declared owner without
   exposing its private route;
1. verify the accepted evidence media class and normalized text encoding;
1. tokenize the complete physical input without dropping blank or malformed
   records;
1. classify every physical row or script line exactly once;
1. resolve sections, contexts, owners, event aliases, priorities, variants,
   conversations, and locale bindings;
1. validate all references against the accepted gameplay, character, dialogue,
   audio, localization, and presentation catalogs;
1. build a deterministic immutable import plan;
1. compare that plan with the currently accepted definition revision;
1. materialize changed native assets in a staging namespace;
1. run project data validation and native read-back;
1. compare logical identities, ordering, membership, dependencies, and rendered
   public-safe diagnostics with the import plan;
1. commit the complete new revision atomically; and
1. remove staging state and publish one terminal result.

A failed request publishes no partial line, conversation, selection group,
coverage row, localization key, audio binding, redirect, or native asset.

## Evidence identity and public identity

Private evidence identity and public runtime identity are separate.

Private intake may retain a digest, byte size, encoding, physical row count,
review state, and opaque evidence identity. None of those values become a line,
speaker, event, conversation, selection group, gameplay, or primary-asset
identity.

Public identities derive from reviewed semantic fields and versioned mappings:

- canonical speaker or archetype identity;
- canonical event identity;
- owning gameplay or presentation domain;
- world, chapter, mission, race, tutorial, interior, vehicle, or interaction
  context where applicable;
- conversation identity and ordinal;
- line role and variant identity;
- locale and localization-key identity;
- selection-group membership;
- definition and mapping revisions; and
- feature or overlay namespace when applicable.

Source filename, worksheet name, performer name, physical row number, raw line
text, audio filename, source-product label, table order, file digest, and object
path are not durable runtime identity.

## Complete-consumption ledger

The parser emits one consumption record for every physical table row or script
line. Each consumption record contains:

- input ordinal used only for private diagnostics;
- token class;
- normalized owner and context state before and after the token;
- zero or one produced semantic-record identity;
- zero or more explicitly linked continuation identities;
- rejection or exclusion reason when no semantic record is produced; and
- mapping revision.

Every physical token must terminate in exactly one closed class. A token cannot
be silently skipped, consumed by two classifiers, or left unclassified. The sum
of blank, structural, legend, note, semantic, placeholder, rejected, and error
records must equal the physical token count.

The consumption ledger is private review evidence. Public diagnostics expose
only safe aggregate counts and stable semantic identities.

## Source-family and revision reconciliation

One semantic owner may have several reviewed event-table members from different
authoring passes. A source family may contain a base table, a character-owned
copy, a template-derived copy, a corrected copy, or a table with an additional
provenance-only column. Those physical members are evidence revisions, not
independent runtime catalogs.

Before native projection, the importer builds one owner-scoped reconciliation
set containing:

- the declared canonical owner or archetype;
- every eligible evidence member and its opaque revision identity;
- one parsed consumption ledger per member;
- the accepted schema and mapping revision for each member;
- normalized semantic identities produced by every member;
- equivalent, conflicting, superseded, and rejected relationships; and
- one terminal reconciliation result for the complete set.

The complete declared member set is loaded before any line, localization key,
audio binding, selection group, redirect, or asset revision is published.
Missing members, stale member revisions, or incomplete token accounting fail the
set and preserve the previously accepted public revision.

### Cross-member equality

Physical equality is never sufficient to decide semantic equality. Two members
are equivalent only after owner, event, context, locale, role, variant,
selection-group membership, definition revision, and mapping revision resolve.

Equivalent semantic definitions collapse into one public definition with
private duplicate evidence. A newer physical member may supersede an older
member only through an explicit accepted relationship. File age, folder,
filename, worksheet label, row order, byte equality, copied text, and repeated
presence do not establish precedence.

Two byte-identical tables assigned to different canonical owners remain distinct
owner bindings. Conversely, two differently formatted tables that resolve to
the same owner-scoped semantic definitions may collapse after complete
consumption and validation.

A semantic identity with incompatible text, context, priority, audio binding,
locale, role, variant, eligibility, or interruption behavior is a conflict. The
importer does not choose the fullest row, newest-looking file, or member with an
audio alias. The complete reconciliation set fails until the owning domain
accepts one definition or an explicit supersession mapping.

### Template-derived members

A template-derived table is parsed through the same closed schema and row
taxonomy as every other event table. The word `template`, sparse audio aliases,
placeholder text, repeated default events, or a compact row count cannot make a
row optional, authoritative, or safe to skip.

Template structure may establish expected event coverage for its declared owner.
It cannot create a speaker, archetype, gameplay event, priority, localization
key, recording requirement, or runtime line without accepted semantic fields and
owner mappings.

Placeholder and incomplete rows remain consumed evidence with explicit terminal
states. They cannot overwrite a complete accepted definition, and they cannot
silently remove an existing line merely because a later template omits text or
an audio alias.

### Reconciliation verification

Automated tests cover:

- member-order permutations producing the same public revision;
- six- and seven-column members reconciling under one owner;
- equivalent definitions collapsing across differently formatted members;
- byte-identical members remaining distinct across different owners;
- incomplete template rows unable to replace complete definitions;
- conflicting priority, context, text, locale, or audio bindings failing the
  complete set;
- explicit supersession replacing one accepted member revision;
- stale or missing declared members preserving the previous public revision;
- every physical token remaining represented in its member ledger; and
- public diagnostics exposing only safe aggregate reconciliation counts.

## Event-table shape

### Accepted column families

An event table has one of two accepted physical widths:

- six columns containing event label, event alias, private dialogue text,
  provenance or reuse class, priority token, and optional audio alias; or
- the same six columns plus one provenance-only reference column.

Column names are matched through a closed case-insensitive alias table after
Unicode normalization and surrounding-whitespace removal. Header aliases may
correct capitalization and reviewed spelling variants, but cannot reorder
columns or create new semantics.

The extra reference column is excluded from runtime identity and output. It may
support private review only. Any other column count fails the source unless a
new versioned schema is accepted and tested.

### Header modes

A table may use either:

- **header-first mode**, in which the first semantic row is the canonical
  header; or
- **section-first mode**, in which the file begins with a recognized role or
  category section and therefore uses the accepted default six-column schema.

Section-first mode is allowed only for a reviewed schema revision whose width is
unambiguous. The importer does not infer a schema from whichever row happens to
contain the most populated cells.

A repeated canonical header inside the data region is structural and produces no
line. A malformed or partially matching header is an error, not a dialogue row.

### Closed table-row taxonomy

Each table row resolves to exactly one class:

<!-- markdownlint-disable MD013 -->

| Row class | Required behavior |
| :--- | :--- |
| `blank` | Preserve consumption, produce no semantic record, and leave section state unchanged. |
| `schema_header` | Confirm accepted column order and produce no semantic record. |
| `legend` | Validate known explanatory text and produce no event, line, priority, or source value. |
| `owner_section` | Change the registered owner or archetype scope only through an explicit mapping. |
| `role_section` | Change walker, driver, pedestrian, mission, ambient, interior, or another registered role scope. |
| `context_section` | Change a registered mission, race, tutorial, world, interior, interaction, or presentation context. |
| `line_candidate` | Produce one candidate line or selection-group member after all required mappings resolve. |
| `placeholder` | Record an intentional missing or future line without publishing runtime content. |
| `review_note` | Preserve a private diagnostic and produce no runtime content. |
| `repeated_header` | Confirm schema continuity and produce no semantic record. |
| `rejected` | Record a typed non-semantic reason and produce no runtime content. |
| `error` | Abort the complete source revision. |

<!-- markdownlint-enable MD013 -->

The classifier uses closed structural predicates and reviewed alias tables. It
does not use a general language model, fuzzy semantic guess, or line-text
similarity to decide row meaning.

## Column semantics

### Event label

The event-label column may introduce a new semantic event or continue the
previous registered event within the current section. Blank continuation is
legal only when a prior event is active and the row contains a valid event alias
or candidate line.

Category labels, prose instructions, legends, placeholder terms, and production
notes are not events. An unknown event label cannot create a runtime event by
frequency or repetition.

### Event alias

The event-alias column supplies a reviewed source alias for a canonical semantic
event binding or selection group. It is normalized through a versioned alias
registry. Unknown, ambiguous, cross-domain, or owner-incompatible aliases fail
the row and therefore fail the source revision when the row claims semantic
content.

An alias is not an audio asset, gameplay event implementation, filename, or
queue identity.

### Private dialogue text

Dialogue text supports private comparison, duplicate review, localization
intake, subtitle intent, and line completeness. It never appears in gameplay
code, source comments, public diagnostics, or stable identity.

A semantic line that requires spoken content must resolve to an accepted
localization key or an explicitly declared non-verbal vocalization class. Empty
text is a placeholder or error unless the event definition explicitly permits a
non-verbal line and supplies its canonical semantic identity.

Whitespace normalization for comparison does not rewrite accepted localized
text. Punctuation, spelling, dialect, mature-content classification, and
translation decisions belong to localization and publication review.

### Provenance or reuse class

Historical new, reused, prior-product, written, or comparable tokens remain
private provenance and review evidence. They may select a private rights or
media verification path. They cannot alter event identity, gameplay
eligibility, priority, probability, queue order, speaker ownership, locale
fallback, or runtime loading.

Embedded explanatory legends in this column are structural rows, not provenance
values.

### Priority token

Priority uses the closed mapping owned by historical dialogue normalization. The
recognized authoring tokens are normalized to semantic queue policy only after
the event and owner have resolved.

A priority token:

- cannot acknowledge or complete gameplay;
- cannot bypass event eligibility;
- cannot create an event;
- cannot select a numeric queue position directly;
- cannot override native Sound Concurrency; and
- cannot convert an optional line into required content without an accepted
  event-binding revision.

Blank priority requires an explicit event-binding default. Unknown tokens,
multiple tokens in one cell, prose legends, misspelled instructions, and values
outside the closed mapping fail import.

### Audio alias

The optional audio-alias column is restricted provenance evidence that must
resolve through the normalized audio manifest before a native audio binding is
published. Missing aliases are allowed only for an explicitly unrecorded,
subtitle-only, synthetic-test, or non-verbal definition whose fallback policy is
complete.

Audio aliases never become object paths or primary-asset identities directly.
One alias resolving to multiple incompatible assets, or multiple aliases
claiming one exclusive required recording, fails import.

### Provenance-only reference

An optional seventh column may support private review of a source reference,
recording reference, or comparable provenance. It is excluded from runtime,
public diagnostics, localization identity, attribution, and asset naming.

The presence or absence of this column cannot change line eligibility or
priority.

## Owner and archetype resolution

Every source request declares one expected owner class before row parsing:

- canonical named speaker;
- canonical generic population archetype;
- canonical vehicle or location presentation owner; or
- another registered dialogue owner.

The evidence adapter resolves the private owner label through the accepted
content catalog and passes only the canonical identity into normalization.
Physical filenames and worksheet labels cannot create owners.

Generic archetypes remain archetypes. They do not create named characters,
credits, biographies, or unique world entities.

Identical table payloads assigned to different canonical owners are not
duplicate runtime definitions. Deduplication occurs only after owner, event,
context, locale, role, variant, and mapping revision are included. Byte
equality, text equality, audio-alias equality, or matching row order cannot
collapse distinct owner bindings.

Conflicting owner evidence fails closed. One source cannot change owner midway
unless the schema explicitly supports owner-section rows and every transition
resolves to a registered owner.

## Section state machine

Table sections form a deterministic state machine. State contains:

- owner identity;
- role policy;
- world and chapter scope;
- mission, race, tutorial, interior, vehicle, or interaction context;
- current event identity;
- conversation identity when active;
- locale policy;
- priority default when explicitly registered; and
- mapping revision.

A structural row may transition only the fields it owns. A role section cannot
change owner. A context section cannot change priority. A legend cannot change
anything. A line row consumes current state but cannot mutate it implicitly.

State is reset at source boundaries and at explicit reset sections. Carrying an
event, owner, context, or priority from one source into another is forbidden.

## Script shape

### Monolithic text exports

A script may contain only one Markdown heading while representing many
missions, races, tutorials, interiors, phone interactions, bonus activities,
conversations, and speaker turns. Markdown heading depth is therefore not
semantic authority.

The script parser preserves every physical line and classifies it through a
closed grammar containing:

- document title;
- chapter or world section;
- mission, race, tutorial, bonus, interior, phone, or interaction section;
- conversation title or identifier;
- speaker cue;
- dialogue continuation;
- stage direction;
- gameplay direction;
- presentation direction;
- blank separator;
- revision or production note;
- placeholder;
- rejected non-semantic text; and
- error.

Semantic segmentation uses reviewed marker aliases, explicit numbering where
present, registered speaker aliases, and accepted mission and event catalogs. It
does not rely on Markdown headings, page layout, capitalization alone, source
filename, or arbitrary blank-line counts.

### Speaker turns

A speaker cue must resolve uniquely to a canonical speaker, archetype, narrator,
system voice, or explicitly registered role. A cue opens one speaker turn within
the active context. Following dialogue continuations belong to that turn until a
new structural marker, speaker cue, or explicit termination.

Unknown speaker cues, two owners matching one alias, dialogue before an owner is
established, and owner changes inside an unclosed continuation fail import.

Stage and gameplay directions may provide private context, timing, emotion,
listener, positional, camera, animation, or event evidence. They never become
spoken text unless explicitly classified and reviewed as a line.

### Context segmentation

Every retained script line belongs to exactly one registered context. Supported
context kinds include:

- campaign mission;
- tutorial step;
- race or race phase;
- bonus activity;
- interior interaction;
- phone or mediated interaction;
- vehicle or pedestrian event;
- ambient or world event;
- cinematic or presentation sequence; and
- system or frontend presentation.

A script marker is an alias for a current catalog identity, not authority to
create a mission, race, level, interior, or feature. Unsupported or superseded
contexts are rejected or recorded as non-runtime evidence according to the
current campaign decision.

### Conversation construction

The importer groups speaker turns into a conversation only when the active
context, participant roles, explicit conversation marker, and ordering evidence
agree. It assigns stable ordinals after structural rows and rejected notes are
removed.

A conversation definition must have:

- one canonical conversation identity and revision;
- one owning context;
- an ordered set of required and optional line identities;
- complete speaker and addressee roles;
- start, interruption, restart, cancellation, and terminal policies;
- positional, listener, subtitle, mouth, and mix policies;
- locale and missing-audio fallback; and
- verification scenarios.

Physical line number is retained only in private diagnostics. It is not the
conversation ordinal.

A monolithic script may contribute many independent conversations. Failure in
any required semantic segment rejects the complete source revision; the importer
cannot publish the earlier valid portion and silently omit the failing tail.

## Line and selection-group construction

A valid line candidate resolves to one immutable line definition containing:

- canonical line identity and revision;
- canonical event binding;
- canonical owner and role;
- context and world scope;
- optional conversation identity and ordinal;
- optional selection-group identity and variant order;
- localization and subtitle identities;
- required or optional audio binding;
- semantic priority and interruption policy;
- probability and repeat policy;
- positional, listener, attachment, and lifetime policy;
- mouth and presentation observations;
- locale and accessibility fallback;
- feature namespace; and
- source-mapping revision.

Equivalent variants for one event are selection-group members. Stable member
order derives from canonical identities and explicit variant order, not source
row order. Weighting and probability require accepted values; the number of
similar source rows is not a weight.

Duplicate semantic identities with equivalent definitions collapse into one
record plus private duplicate evidence. Duplicate identities with different
semantic definitions are conflicts and fail import.

## Localization projection

Accepted dialogue text enters the normalized language interchange under stable
localization keys. The importer publishes no raw authoring text to gameplay code
or public diagnostics.

Localization projection preserves:

- spoken-text identity;
- subtitle identity and optional override;
- speaker and addressee context;
- mature-content classification when approved;
- voice-direction or translator context only after publication review;
- locale availability;
- recording and fallback state; and
- localization revision.

A locale may fall back only through an explicit policy. Missing required spoken
or subtitle content fails the line. A localized text key cannot resolve to two
incompatible semantic lines in one revision.

## Native Unreal projection

### Primary data assets

Repository-owned dialogue definitions are materialized as native data assets
registered with the Asset Manager. Primary assets own stable engine-visible
identity, bundle metadata, and soft references to secondary presentation assets.

The minimum native asset families are:

- dialogue-line definitions;
- conversation definitions;
- selection-group definitions;
- event-binding definitions;
- owner or archetype coverage definitions; and
- dialogue-catalog revision metadata.

Native subclasses may derive from `UPrimaryDataAsset` or another registered
class that provides an equivalent stable primary-asset contract. Definition
assets are immutable at runtime. Required audio, subtitle, mouth, and
presentation assets use cook-aware soft references and declared bundles.

A raw table imported directly as a `UDataTable` is not runtime authority. A
DataTable may exist only as a generated editor-side inspection projection when
its schema is closed, its source is the accepted semantic plan, it is excluded
from shipping authority, and native read-back proves it contains no additional
meaning.

### Dialogue Waves

`UDialogueWave` may project spoken text, subtitle override, voice direction,
localized context, and associated sound-wave mappings when those native features
fit the accepted line definition.

Dialogue Waves are presentation assets. They do not own event eligibility,
mission state, queue arbitration, conversation completion, selection history,
priority, probability, or persistence. The repository dialogue catalog remains
semantic authority and references Dialogue Waves through validated soft
bindings.

### Asset Manager and bundles

The Asset Manager discovers dialogue primary assets and audits their cook and
bundle rules. Dialogue bundles are declared by use, such as required gameplay,
locale, chapter, mission, interior, frontend, optional variation, or development
coverage.

A bundle cannot hide an undeclared hard dependency. Required lines for an
active context must be loadable before that context becomes ready. Optional
variants may remain asynchronously loadable, but their absence follows explicit
fallback and cannot change gameplay results.

### Import provenance

Native assets store repository-owned semantic revision and opaque mapping
metadata. They do not store restricted evidence routes, workstation paths,
worksheet names, performer names, historical filenames, raw source text,
review comments, or source audio filenames.

Engine source-import metadata that would persist a private or machine-specific
route is not used as public asset provenance. Reimport uses the repository-owned
normalized plan and opaque evidence resolution outside the cooked asset.

## Editor adapter behavior

The Unreal editor adapter receives a complete immutable import plan. It may not
parse authoring evidence or change semantic classifications.

For each plan it:

1. verifies the expected catalog and mapping revisions;
1. computes the full target asset set before mutation;
1. creates or updates assets in a disposable staging namespace;
1. applies canonical names, primary-asset identities, properties, bundles, and
   soft references;
1. compiles or refreshes dependent native asset data where required;
1. runs project validators and engine reference checks;
1. saves and reloads the staged packages;
1. reads back every semantic field and dependency;
1. compares the read-back projection with the plan;
1. atomically promotes the complete revision; and
1. removes superseded staging packages and publishes one terminal result.

Mutation is serialized per catalog revision. A timeout, cancellation, editor
shutdown, package-save error, source-control denial, asset-registry delay, or
validator failure leaves the previous accepted revision active.

## Validation

### Object validation

Project-owned dialogue definition classes validate their own invariants through
native object validation. Cross-asset and engine-class rules use editor
validators.

Validation rejects:

- invalid or duplicate primary-asset identity;
- missing owner, event, context, locale, or definition revision;
- unresolved conversation line or ordinal;
- invalid selection-group member order;
- unknown priority or event alias;
- missing required audio, subtitle, or fallback;
- undeclared hard reference;
- bundle mismatch;
- cross-feature reference without a declared dependency;
- private route or prohibited provenance text;
- source-era filename used as identity;
- stale mapping, catalog, or world revision;
- dependency cycle;
- development-only asset entering a production bundle; and
- semantic mismatch after package reload.

### Commandlet validation

The complete dialogue asset family must support validation without opening a map
or starting gameplay. The commandlet path loads the relevant Asset Registry and
Asset Manager state, runs native object and project validators, emits bounded
public-safe diagnostics, and returns failure when any required asset is invalid.

Editor UI validation is useful for local review but cannot replace the
commandlet and repository canonical validation used for acceptance.

### Native read-back

Read-back proves:

- every planned primary asset exists exactly once;
- every primary-asset identity resolves through the Asset Manager;
- every definition revision matches the plan;
- all soft references resolve to allowed asset classes;
- bundle membership and cook rules match policy;
- conversation membership and ordinals are complete;
- selection-group ordering and weights are deterministic;
- locale, subtitle, audio, mouth, and fallback bindings are complete;
- no prohibited source metadata is serialized;
- no unexpected asset was created or removed; and
- package reload produces the same logical definition.

Transport success, package existence, or a successful save call alone is not
acceptance evidence.

## Determinism

Equivalent reviewed evidence, accepted catalogs, alias mappings, localization
revision, and import policy produce the same:

- consumption counts by row or line class;
- semantic identities;
- line, conversation, selection-group, and event-binding definitions;
- canonical ordering;
- owner and context membership;
- localization keys;
- native asset names and primary-asset identities;
- soft references and bundle membership;
- validation diagnostics; and
- public-safe coverage summaries.

The importer sorts by canonical semantic identity wherever source order is not a
meaningful conversation ordinal. Hash-map iteration, filesystem enumeration,
Asset Registry discovery order, source row order for equivalent variants,
locale enumeration, and object creation order cannot affect output.

A determinism proof runs the complete import twice from clean staging state and
compares logical plans and native read-back. Binary package bytes are not the
only proof because engine serialization may contain non-semantic state; every
caller-visible semantic field and dependency must match.

## Public-safe coverage

The importer publishes a development-only coverage projection containing only:

- canonical owner or archetype identity;
- canonical event identity;
- context kind and identity;
- required, optional, mapped, missing, fallback, duplicate, rejected, or error
  state;
- line and conversation counts;
- locale and audio-completeness state;
- mapping and catalog revisions; and
- bounded typed diagnostics.

It excludes source dialogue, source filenames, audio filenames, private routes,
physical row numbers, performer or employee names, production notes, approval
state, recording-delivery state, source-product labels, and source references.

Coverage cannot make a line required, upgrade priority, force playback, create a
speaker, or mark gameplay complete.

## Typed failures

The boundary uses typed failures for:

- unsupported media or encoding;
- unsupported table width;
- missing, repeated-invalid, or ambiguous schema;
- unclassified physical token;
- invalid row transition;
- unresolved or conflicting owner;
- unknown or ambiguous event alias;
- unknown context or superseded feature;
- malformed or unknown priority;
- required text, audio, subtitle, locale, or fallback missing;
- conversation segmentation or ordinal failure;
- duplicate semantic identity conflict;
- unresolved audio or localization binding;
- prohibited private metadata;
- stale input revision;
- nondeterministic plan;
- native asset creation or save failure;
- data-validation failure;
- native read-back mismatch; and
- rollback or cleanup failure.

Every failure identifies the semantic source class, safe owner identity,
token class, mapping revision, and actionable reason without publishing private
text or routes.

## Rollback and cleanup

The current accepted catalog remains active until the complete staged revision
passes validation and read-back. Promotion uses one revision boundary.

On failure or cancellation, the adapter:

- rejects the staged revision;
- removes incomplete packages created by that request;
- releases temporary Asset Registry and streamable handles;
- restores no file by broad workspace cleanup;
- leaves unrelated assets and concurrent editor work untouched;
- preserves the prior accepted catalog;
- records one terminal typed result; and
- proves that no staging asset remains registered or cookable.

A cleanup failure is a blocking error. The importer cannot report success while
staged or partially promoted assets remain.

## Verification taxonomy

### Table schema contracts

Tests cover:

- canonical six-column headers;
- accepted case-only header aliases;
- section-first tables with unambiguous default schema;
- the accepted provenance-only seventh column;
- unsupported column counts;
- malformed and partially matching headers;
- repeated valid and invalid headers; and
- deterministic Unicode and line-ending normalization.

### Row classification contracts

Tests cover:

- blank rows;
- legends embedded above or inside data;
- owner, role, and context sections;
- event continuations;
- placeholders and missing text;
- review notes and prose instructions;
- unknown row shapes;
- complete token accounting; and
- zero silent row loss.

### Identity and owner contracts

Tests cover:

- named speakers and generic archetypes;
- ambiguous and missing owners;
- identical table payloads assigned to distinct owners;
- equivalent duplicates within one owner and context;
- conflicting duplicate semantic identities;
- owner changes at legal and illegal boundaries; and
- source names and text excluded from identity.

### Event and priority contracts

Tests cover:

- canonical and corrected event aliases;
- unknown, ambiguous, and cross-domain aliases;
- event continuation under active state;
- all accepted priority tokens;
- explicit defaults for blank priority;
- prose legends and mixed tokens rejected as priority;
- priority unable to bypass event eligibility; and
- source or reuse class unable to alter semantics.

### Script segmentation contracts

Tests cover:

- single-heading scripts with multiple contexts;
- mission, race, tutorial, bonus, interior, phone, ambient, and presentation
  markers;
- speaker cues and dialogue continuations;
- stage and gameplay directions;
- unknown speakers and contexts;
- incomplete final segments;
- conversation ordinals independent of physical lines;
- failure in a late segment rejecting the complete revision; and
- superseded features producing no runtime definitions.

### Native projection contracts

Tests cover:

- primary-asset identity and discovery;
- asset bundles and soft references;
- Dialogue Wave projection without semantic-authority drift;
- localization and subtitle completeness;
- commandlet data validation;
- save, reload, and semantic read-back;
- prohibited provenance exclusion;
- staging rollback;
- repeated import determinism; and
- no runtime parser or editor-module dependency.

## Acceptance criteria

The boundary is accepted only when:

- every physical row or script line is consumed exactly once;
- every semantic record has one canonical owner, event, context, and revision;
- every priority and alias resolves through a closed mapping;
- identical templates remain distinct when their owners differ;
- monolithic scripts segment without relying on Markdown headings;
- private dialogue and provenance remain outside public diagnostics and runtime
  identity;
- the semantic import plan is complete and deterministic;
- native assets validate and match read-back;
- failure leaves the previous revision active and no staging residue; and
- shipping runtime loads only cooked repository-owned assets through the
  accepted catalog and Asset Manager boundary.
