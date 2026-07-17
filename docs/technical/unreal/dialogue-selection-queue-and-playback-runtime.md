# Dialogue selection, queue, and playback runtime

- Status: Active
- Last reviewed: 2026-07-17

## Governing decisions and specifications

<!-- markdownlint-disable-next-line MD013 -->
- [Native gameplay audio, dialogue, and listener boundary](../../adr/unreal/runtime/native-gameplay-audio-dialogue-and-listener-boundary.md)
<!-- markdownlint-disable-next-line MD013 -->
- [State-driven missions, interactions, interiors, and notoriety](../../adr/unreal/runtime/state-driven-missions-interactions-and-notoriety.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Platform audio cooking and streaming](platform-audio-cooking-and-streaming.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Presentation playback runtime](presentation-playback-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Spatial audio listener and positional-source runtime](spatial-audio-listener-and-positional-source-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Typed event and observation routing runtime](typed-event-and-observation-routing-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Historical core-design and dialogue evidence normalization](historical-core-design-and-dialogue-evidence-normalization.md)

## Purpose

This specification defines imported dialogue metadata, line and conversation
identity, event binding, participant matching, deterministic variant selection,
priority and probability policy, queue admission, interruption, expiry,
playback,
positional projection, subtitles, mouth-animation observations, pause, teardown,
and diagnostics.

It replaces runtime filename parsing, hard-coded event and character tables,
process-global linked queues, mutable raw dialogue pointers, global random
selection, callbacks that advance gameplay, and debug-only played-state
mutation.

Dialogue is presentation of accepted semantic events. It cannot become mission,
interaction, reward, progression, persistence, character, or world authority.

## Native Unreal foundation

The boundary uses native Unreal facilities:

- `USoundWave`, Dialogue Voice, Dialogue Wave, Sound Cue, or MetaSound assets
  when
  suitable for the accepted content pipeline;
- `UAudioComponent` for controlled dialogue playback;
- Sound Attenuation and Sound Concurrency assets;
- Sound Classes, Sound Mixes, submixes, ducking, and parameter modulation;
- localized text, subtitle, culture, and audio assets;
- animation montage, facial-animation, lip-sync, or typed mouth-animation
  adapters;
- Actor and component attachment for positional speakers;
- Asset Manager bundles and retained handles; and
- game-instance, world, local-player, and feature subsystem lifetimes.

Repository code owns semantic identities, imported metadata, matching,
arbitration, queue state, deterministic variation, correlation, and terminal
results. It does not implement a second audio mixer.

## Ownership

<!-- markdownlint-disable MD013 -->

| Authority | Responsibility |
| :--- | :--- |
| Dialogue catalog | Owns line, conversation, selection-group, event-binding, participant, locale, subtitle, priority, and playback definitions. |
| Event-routing service | Publishes immutable semantic events and participant context. |
| Mission and interaction services | Own domain transactions, eligibility, completion, rewards, progression, and persistence. |
| Dialogue-selection service | Resolves eligible definitions and deterministic variants. |
| Dialogue-queue service | Owns admission, ordering, interruption, expiry, pause, cancellation, and terminal queue results. |
| Unreal Audio Engine | Owns source playback, attenuation, concurrency, routing, mixing, virtualization, and output. |
| Subtitle and accessibility services | Own localized text projection and accessibility policy. |
| Character presentation | Owns mouth, facial, gesture, and look-at projections from accepted line observations. |
| Spatial-audio subsystem | Owns listener policy and positional-source projection. |

<!-- markdownlint-enable MD013 -->

A line can present a domain event but cannot acknowledge that event on behalf of
its owner.

## Runtime identities

The boundary uses stable identities for:

- `FSharDialogueLineId`;
- `FSharDialogueLineRevision`;
- `FSharConversationId`;
- `FSharConversationRevision`;
- `FSharDialogueSelectionGroupId`;
- `FSharDialogueEventBindingId`;
- `FSharDialogueRequestId`;
- `FSharDialogueQueueEntryId`;
- `FSharDialoguePlaybackId`;
- `FSharDialoguePlaybackRevision`;
- `FSharDialogueSpeakerId`;
- `FSharDialogueParticipantRevision`;
- `FSharDialogueUsageRevision`;
- `FSharLocaleRevision`;
- `FSharWorldCompositionRevision`;
- `FSharFeatureRevision`; and
- `FSharDialogueResultId`.

Filename fields, underscore counts, source table ordinals, linked-list
positions,
raw pointers, random-call order, debug coverage flags, and callback addresses
are
not durable identity.

## Dialogue line definition

`USharDialogueLineDefinition` contains:

<!-- markdownlint-disable MD013 -->

| Field | Contract |
| :--- | :--- |
| `LineId` | Canonical line identity. |
| `EventId` | Semantic event or presentation intent. |
| `SpeakerId` | Canonical speaker, archetype, role, or participant binding. |
| `AddresseePolicy` | Optional second participant, group, local-player, or contextual target. |
| `ConversationId` | Optional ordered conversation membership. |
| `ConversationOrdinal` | Canonical line order within that conversation revision. |
| `SelectionGroupId` | Optional group of equivalent or varied lines. |
| `WorldScope` | Allowed world, chapter, region, interior, level, mission, race, or mode policy. |
| `RolePolicy` | Walker, driver, pedestrian, villain, mission actor, ambient actor, or another registered role. |
| `AudioByLocale` | Required and optional localized audio identities. |
| `SubtitleByLocale` | Localized subtitle identity and timing policy. |
| `PlaybackPolicy` | Positional, non-positional, attached, local-player, or shared presentation. |
| `PriorityPolicy` | Admission class, interruption, and queue ordering. |
| `ProbabilityPolicy` | Optional deterministic variation probability. |
| `LifetimePolicy` | Queue expiry, playback deadline, and stale-event policy. |
| `ConcurrencyPolicy` | Native Sound Concurrency and project queue limits. |
| `DuckingPolicy` | Sound Class, submix, cinematic, music, and ambience behavior. |
| `MouthPolicy` | Optional facial, mouth, subtitle, and speaker-observation correlation. |
| `FallbackPolicy` | Locale, speaker, positional, and optional-line fallback. |
| `DefinitionRevision` | Immutable revision for stale-result rejection. |

<!-- markdownlint-enable MD013 -->

Definitions reject missing event identity, duplicate line identity, ambiguous
speaker binding, invalid conversation order, missing required locale, unbounded
probability, undeclared interruption, and incomplete teardown.

## Conversation definition

`USharConversationDefinition` contains:

- conversation and revision identities;
- canonically ordered line identities;
- required and optional speaker roles;
- participant-binding rules;
- start and completion event policy;
- line-to-line timing and interruption policy;
- positional and listener policy;
- subtitle and mouth-animation policy;
- missing-line fallback;
- cancellation and restart behavior; and
- verification scenarios.

A conversation is complete only when every required ordinal resolves exactly
once
and participant constraints agree. Runtime import does not infer completeness
from matching filename fragments.

## Selection-group definition

A selection group represents interchangeable lines or conversations for one
semantic request. It declares:

- group identity and revision;
- member line or conversation identities;
- stable member order;
- weighting or probability policy;
- usage-history policy;
- repeat suppression;
- exhaustion behavior;
- reset scope;
- locale and participant constraints; and
- deterministic seed inputs.

Removing a member or changing weights creates a new group revision. It does not
mutate active queue entries in place.

## Event-binding definition

A dialogue event binding maps one typed semantic event to candidate definitions.
It contains:

- event identity and schema revision;
- required and optional participant roles;
- world, level, mission, race, interior, and mode filters;
- vehicle, on-foot, damage, traffic, collectible, reward, and interaction
  context
  where applicable;
- positional or non-positional policy;
- priority, probability, and lifetime overrides within declared bounds;
- villain, tutorial, mission, ambient, and system categories;
- local-player ownership;
- fallback group; and
- diagnostics and coverage policy.

String fragments such as `start`, `race`, `card`, `coin`, or `damaged` remain
import evidence only. Runtime requests use canonical event identities.

## Import and metadata conversion

Import converts normalized audio and dialogue manifests into typed line,
conversation, selection-group, event-binding, participant, locale, subtitle,
priority, probability, and playback definitions.

Source filenames may provide provenance or migration evidence during import. The
importer validates and records every parsed field, then publishes canonical
assets. Shipping runtime never:

- strips directories from source names;
- counts underscores;
- parses event, speaker, role, order, level, mission, or conversation fields;
- maps short character tokens through hard-coded arrays;
- uses a source hash as speaker identity;
- infers positional policy from a filename prefix; or
- reclassifies dialogue from a cooked object name.

Unknown or ambiguous source metadata fails import or requires an explicit
mapping.

Historical conversation and sound-event spreadsheets follow
<!-- markdownlint-disable-next-line MD013 -->
[Historical core-design and dialogue evidence normalization](historical-core-design-and-dialogue-evidence-normalization.md).
The importer treats non-empty conversation rows as private semantic evidence for
speaker, event, mission or location context, conversation membership, ordinal,
role, variant, audio alias, and locale. Sound-event rows add candidate event
alias, selection-member label, source priority token, archetype or presentation
owner, and coverage expectation. Heading rows, placeholder events, prose notes,
and empty rows create no line or binding; zero-byte companion sheets are
excluded
before coverage.

Dialogue text enters the localization pipeline under stable keys and is not
copied into gameplay code or used as identity. Platform-specific tutorial lines
bind one semantic input action to active localized device presentation instead
of
preserving obsolete button names. Approval, source, recording, delivery,
licensor-receipt, and audio-reuse columns can block publication or select a
private review path but never select runtime content, change priority, or alter
participant eligibility.

Historical per-level mission-dialogue summaries are reconciled with conversation
sheets and accepted mission definitions before publication. They may contribute
candidate speaker roles, semantic events, mission and stage context,
conversation
membership, branch conditions, presentation intent, and localization bindings.

Raw prose, performance directions, replacement notes, source audio aliases,
headings, scene order, and source mission numbers remain private evidence or
provenance. A summary cannot create a required conversation until every retained
line, participant, event, condition, locale, subtitle, audio, and fallback
binding
resolves. Conflicting summaries and sheets produce an explicit finding; the
importer does not prefer the longer, later-looking, or differently named source.

Exact duplicate dialogue records collapse by content digest and semantic
identity.
A changed revision is compared line by line and cannot silently replace an
accepted localized line or conversation member.

## Participant context

`FSharDialogueParticipantSnapshot` contains:

- canonical participant and presentation identities;
- character, archetype, role, costume, and voice profile revisions;
- local-player and controller ownership;
- current vehicle, seat, driver, passenger, and on-foot observations;
- world, region, interior, level, mission, race, and interaction revisions;
- alive, active, streamed, visible, and speaking eligibility;
- Actor, component, socket, transform, and velocity evidence for positional use;
- locale, subtitle, accessibility, and content-filter policy; and
- stable correlation identity.

Selection uses the immutable snapshot. It does not query mutable character or
vehicle objects after acceptance.

## Dialogue request

`FSharDialogueRequest` contains:

- request and semantic event identities;
- expected event schema revision;
- participant snapshots and role assignments;
- world, mode, mission, interaction, local-player, and feature revisions;
- requested line, conversation, or selection-group identity when explicit;
- priority, deadline, and queue policy;
- optional deterministic variation seed;
- positional-source and listener policy;
- subtitle and accessibility policy;
- interruption and cancellation permissions;
- deduplication and idempotency keys; and
- diagnostics context.

Requests are immutable after acceptance. A replacement request supersedes the
old
one through an explicit transaction.

## Eligibility matching

Candidate matching evaluates declared fields only. At minimum it may consider:

- semantic event identity;
- speaker and addressee identity;
- participant role and archetype;
- local-player ownership;
- on-foot or in-vehicle context;
- world, chapter, region, interior, level, mission, race, and mode;
- conversation or selection-group identity;
- villain, tutorial, ambient, mission, or system category;
- locale and subtitle availability;
- positional-source readiness;
- usage and repeat policy;
- feature ownership; and
- content or accessibility restrictions.

A missing optional field does not equal every value unless the definition says
`any`. Unknown values fail closed rather than matching an arbitrary line.

## Matching result

Selection produces an immutable result containing:

- request and result identities;
- exact line, conversation, and selection-group revisions;
- matched and rejected field evidence;
- participant-role bindings;
- selected locale and fallback evidence;
- priority, probability, lifetime, and interruption policies;
- positional and subtitle policy;
- deterministic selection seed and member ordinal;
- expected asset bundle; and
- terminal selection status.

Selection success is not queue admission or playback success.

## Deterministic variation

Variation never uses process-global random state. The declared seed is derived
from stable inputs such as:

- request identity;
- event identity;
- selection-group revision;
- participant identities;
- world and mission revisions;
- usage revision;
- accepted session seed; and
- explicit variation ordinal.

The same complete inputs produce the same selected member. Changing locale or
availability may select a declared locale fallback but cannot silently reorder
unrelated members.

## Probability policy

Optional lines may define a bounded probability after eligibility. The policy
specifies:

- probability numerator and denominator or equivalent exact representation;
- stable seed inputs;
- evaluation scope;
- cooldown and repeat behavior;
- whether rejection is terminal or silently absent; and
- diagnostics visibility.

Probability applies after deterministic matching and before queue admission. It
cannot override a required line or mission-owned conversation.

## Usage and repeat policy

Usage state is explicit, revisioned, and scoped. Supported policies include:

- unrestricted repeat;
- avoid immediate repeat;
- play every member before reset;
- play once per session;
- play once per world revision;
- play once per mission attempt;
- cooldown by semantic event; and
- domain-owned explicit reset.

Development coverage tracking is separate from runtime usage state. Opening a
debug page or enabling diagnostics cannot mark a line as played.

## Priority classes

The initial semantic classes are:

- `must_play_immediately`;
- `must_play`;
- `high`;
- `normal`;
- `occasional`;
- `ambient`;
- `tutorial`;
- `mission_critical`; and
- `system_accessibility`.

Definitions map these classes to bounded numeric ordering only inside the queue
adapter. Raw source priority ordinals do not survive as public identity.

Historical sound-event tokens use one versioned import table:

- `MPI` maps to `must_play_immediately`;
- `MPL` maps to `must_play`;
- `SPL` maps to `high` or another explicitly reviewed bounded must-consider
  class; and
- `OPL` maps to `occasional` or another explicitly reviewed optional class.

Blank priority requires an explicit event-binding default. Unknown tokens, mixed
legends, prose, audio identifiers, or typographical corruption in the priority
column fail import or require an explicit mapping. `must_play_immediately`
affects
queue and interruption behavior only; it cannot make an unaccepted gameplay
event
occur or mutate domain state.

## Queue entry

`FSharDialogueQueueEntry` contains:

- queue-entry and request identities;
- selected line or conversation revision;
- participant and local-player revisions;
- priority class and stable tie-break identity;
- admission time and expiry deadline;
- interruption and preemption policy;
- deduplication group;
- positional-source snapshot or attachment binding;
- asset, subtitle, ducking, and mouth requirements;
- lifecycle state; and
- terminal result correlation.

Queue entries own copied immutable state, never raw pointers to selectable
objects.

## Queue lifecycle

The lifecycle uses the closed states:

1. `selected`;
1. `admission_pending`;
1. `queued`;
1. `preparing_assets`;
1. `ready`;
1. `starting`;
1. `playing`;
1. `paused`;
1. `stopping`;
1. `completed`;
1. `expired`;
1. `cancelled`;
1. `rejected`; and
1. `failed`.

Every accepted entry reaches one terminal result exactly once.

## Queue admission and ordering

Admission validates:

- definition and event revisions;
- participant and world readiness;
- locale and required assets;
- positional-source and listener readiness;
- queue capacity;
- native concurrency policy;
- deduplication and repeat suppression;
- deadline and lifetime;
- interruption permissions; and
- application-mode and focus policy.

Ordering uses priority class, declared secondary ordering, admission time, and
stable queue-entry identity. Container insertion order and callback timing
cannot
select the next line.

## Duplicate suppression

A request may declare duplicate equivalence by:

- exact line identity;
- conversation identity;
- event and speaker;
- event and participant pair;
- semantic deduplication group; or
- explicit idempotency key.

Duplicate suppression has a declared time and scope. Repeated collision or
traffic
events cannot flood the queue, while distinct required mission lines cannot be
collapsed accidentally.

## Preemption and interruption

A candidate may preempt the active entry only when its policy and the active
entry's interruption policy both permit it. The transaction:

1. freezes queue advancement;
1. validates the new request and assets;
1. captures the active playback and subtitle state;
1. stops, fades, pauses, or preserves the active entry as declared;
1. commits the replacement entry;
1. restores queue advancement; and
1. publishes typed results for both entries.

An audio callback arriving during preemption cannot advance the queue twice.

## Expiry and timers

Queue lifetime uses accepted engine or presentation time according to policy.
Every entry declares:

- maximum queued lifetime;
- maximum asset-preparation latency;
- playback-start deadline;
- pause behavior;
- suspension behavior; and
- expiry result.

Expiry does not call gameplay failure or success. Required domain dialogue
reports
a typed presentation failure to its owner, which decides recovery.

## Playback preparation

Preparation resolves and retains:

- localized audio source;
- subtitle text and timing;
- Sound Attenuation and Sound Concurrency assets;
- Sound Class, submix, ducking, and mix policy;
- positional source or non-positional output;
- speaker and mouth-animation binding;
- local-player and listener policy; and
- required asset handles.

A loaded sound is not proof that its speaker, world, listener, or request
remains
valid.

## Playback commit

Playback starts only after queue, assets, speaker, listener, subtitle, ducking,
world, application mode, and feature revisions agree.

The committed result contains:

- playback and line identities;
- native audio-component identity when controlled playback is used;
- actual start time;
- selected locale;
- positional or non-positional mode;
- subtitle and mouth-animation correlations;
- expected completion policy; and
- retained-handle ownership.

A fire-and-forget source is allowed only for bounded non-critical lines whose
completion is not required.

## Conversation playback

A conversation evaluates one line at a time from canonical order. The next line
starts only when the accepted sequence policy receives one terminal presentation
result for the current line.

The conversation state records:

- current ordinal;
- bound speaker for each role;
- line and playback revisions;
- optional inter-line delay;
- positional-source snapshot for the next speaker;
- subtitle and mouth state;
- interruption and cancellation policy; and
- terminal conversation result.

A missing required line, speaker, locale, or source fails the conversation
before
advancing. It never skips to an arbitrary later line.

## Positional dialogue

Positional dialogue follows
<!-- markdownlint-disable-next-line MD013 -->
[Spatial audio listener and positional-source runtime](spatial-audio-listener-and-positional-source-runtime.md).
The request names the exact speaker Actor, component, socket, world, and
revision,
or carries a bounded world-space snapshot when attachment is not required.

A positional line may fall back to a declared non-positional presentation when
the speaker unloads and the owning policy permits it. The fallback is recorded
and cannot change participant identity.

## Non-positional dialogue

Mission-critical, tutorial, system, accessibility, or cinematic lines may use a
non-positional mix according to policy. Non-positional playback still carries
speaker, local-player, subtitle, event, queue, and result identity.

A non-positional source does not lose correlation merely because it has no world
transform.

## Listener and local-player policy

Each line declares one of:

- owner-local-player listener;
- shared local-player mix;
- primary-listener mix;
- world positional mix;
- cinematic listener;
- frontend listener; or
- non-positional output.

Split-screen behavior is explicit. A line cannot silently bind every positional
speaker to player zero.

## Subtitle projection

Subtitle projection consumes the accepted playback result and contains:

- line, locale, text, speaker, and playback identities;
- local-player or shared-view ownership;
- timing and segmentation;
- accessibility style;
- interruption and replacement behavior;
- forced-subtitle and content-filter policy; and
- terminal visibility result.

Subtitle completion cannot complete dialogue playback or gameplay. A player may
hide optional subtitles without changing line selection or audio identity.

## Mouth and facial presentation

Mouth, facial, gesture, and look-at adapters consume immutable line-start,
line-progress, line-stop, and line-complete observations. They validate speaker
and character revisions before applying presentation.

Stopping all dialogue publishes idempotent stop observations for every active
speaker. It does not send duplicate gameplay events or leave a replacement
character mouth state active.

## Ducking and mix interaction

Dialogue policy may apply situational ducking to music, ambience, vehicle audio,
cinematic sources, or effects through native Sound Class, Sound Mix, submix, and
modulation facilities.

Ducking begins and ends through a reference-counted semantic lease. Queue empty,
preemption, cancellation, failure, and teardown release the exact lease. One
callback cannot restore the mix while another dialogue entry remains active.

## Cinematic and presentation interaction

Dialogue declares coexistence with cinematic, NIS-equivalent, frontend, mission,
and ambient presentation scopes. A mode may:

- allow both;
- duck one scope;
- pause one scope;
- preempt according to priority;
- reject the new request; or
- defer until the scope releases.

No implementation globally mutes unrelated presentation players by raw pointer.

## Mission, tutorial, villain, and ambient categories

Mission-critical and tutorial lines require explicit owning transactions and
failure recovery. Villain or antagonist lines use typed participant and event
bindings rather than a special raw queue. Ambient lines use bounded probability,
repeat suppression, positional readiness, and population significance.

The category changes priority and policy only. It does not become a second event
identity.

## Pause, focus, and suspension

Pause behavior is declared per line and conversation. Supported outcomes are:

- continue;
- pause and resume;
- duck;
- stop with terminal result;
- defer before start; or
- cancel and report to the owner.

Platform focus and suspension follow the platform audio contract. Resume
revalidates queue, line, speaker, listener, locale, world, and feature
revisions.

## Cancellation and stop-all

Cancellation is idempotent. It removes queued entries, stops or fades active
playback, releases subtitles and mouth presentation, releases ducking and asset
handles, invalidates callbacks, and publishes one terminal result per entry.

Stop-all is scoped by world, mode, local player, mission, conversation, feature,
or shutdown owner. A process-global emergency stop exists only for platform or
application teardown and returns explicit evidence.

## Streaming and locale changes

Dialogue bundles are scoped by active world, mission, characters, locale,
conversation, and immediate event demand. Required active lines remain pinned.

A locale change:

1. freezes new affected admissions;
1. lets uninterruptible accepted lines complete or follows declared stop policy;
1. resolves new localized audio and subtitles;
1. invalidates stale locale callbacks;
1. commits the new locale revision; and
1. resumes eligible requests.

It never changes canonical line, event, participant, or conversation identity.

## Networking

The authoritative gameplay owner replicates semantic events and accepted domain
results. Dialogue selection may be server-authoritative, locally deterministic,
or hybrid according to the definition.

Replication includes canonical identities, required participant context,
selection seed or result, and timing when synchronized presentation is required.
It never replicates raw audio components, source paths, linked queue nodes, or
callback pointers.

Local-only ambient and accessibility lines are explicitly marked. Late network
results are rejected by request, line, usage, world, and feature revision.

## Feature and mod overlays

A validated feature may add namespaced dialogue lines, conversations, selection
groups, event bindings, subtitles, localized assets, and policy rows. It cannot
replace a base line in place, intercept unrelated semantic events, mutate usage
history outside its namespace, or leave queued entries after feature removal.

Feature removal cancels owned entries, stops owned playback, releases assets,
subtitles, mouth and ducking leases, unregisters definitions, and invalidates
stale callbacks atomically.

## Diagnostics and coverage

Development diagnostics expose:

- request, line, conversation, selection-group, event, usage, locale, world,
  feature, and participant revisions;
- eligibility fields and rejection reasons;
- deterministic seed, weights, probability, and selected member;
- queue ordering, priority, deadlines, interruption, and deduplication;
- active playback, subtitle, mouth, positional, listener, ducking, and asset
  state;
- native concurrency and virtualization outcome;
- stale callback and duplicate-result counts;
- per-line development coverage;
- per-speaker or archetype sound-event coverage with mapped, missing, rejected,
  duplicate, fallback, and optional states;
- source-event alias and priority-mapping revision; and
- last failure, fallback, cancellation, or teardown evidence.

Coverage is observational. It cannot mark runtime usage, force a line, alter
selection probability, or change queue order.

## Failure behavior

The subsystem fails closed on:

- unknown or duplicate line, conversation, group, or event-binding identity;
- runtime filename or directory parsing;
- ambiguous participant, role, order, level, mission, locale, or positional
  field;
- incomplete required conversation;
- missing required localized audio or subtitle;
- stale event, request, participant, usage, locale, world, or feature revision;
- global random selection or callback-order arbitration;
- invalid probability, priority, lifetime, interruption, or concurrency policy;
- unknown or ambiguous sound-event alias, placeholder event, malformed priority
  token, or unresolved speaker, archetype, vehicle, location, or presentation
  owner;
- queue entry with raw mutable pointers;
- required positional source with no valid attachment or fallback;
- duplicate active line or terminal result outside declared policy;
- audio, subtitle, or mouth callback attempting to mutate gameplay;
- ducking or asset lease retained after completion;
- queue capacity with no typed resolution; and
- feature or world teardown with remaining entries.

Failure leaves domain state unchanged and reports typed evidence to the owner.

## Validation

Cook and content validation proves:

- every line, conversation, group, and event binding has stable identity;
- every conversation has complete canonical order;
- every participant role and matching field is registered;
- every locale has required audio and subtitle coverage or explicit fallback;
- every probability uses stable declared seed inputs;
- every priority class has bounded queue and interruption policy;
- every historical sound-event token maps through the closed versioned priority
  table or is rejected;
- every retained source-event alias resolves uniquely to one canonical event,
  speaker or archetype owner, and event binding;
- every mission-dialogue summary line either reconciles with one accepted
  conversation member or records an explicit conflict, omission, or rejection;
- exact duplicate dialogue summaries collapse and changed revisions cannot
  silently replace accepted localization, audio, participant, or condition data;
- every line has positional, listener, concurrency, ducking, lifetime, and
  completion policy;
- every mouth and subtitle binding correlates to the exact line revision;
- every feature namespace has complete teardown;
- all required bundles cook for each target; and
- no runtime filename parser or source dialogue table is packaged.

## Tests

Required automated tests include:

- import conversion of line, speaker, role, event, order, level, mission, and
  conversation metadata;
- mission-dialogue summary reconciliation with conversation sheets and accepted
  mission definitions;
- raw prose, performance direction, replacement note, audio-alias, heading, and
  source-order exclusion;
- exact duplicate collapse and changed-revision line comparison;
- sound-event alias normalization, placeholder rejection, speaker and archetype
  ownership, closed priority-token mapping, malformed priority rejection, source
  and reuse provenance, and coverage-matrix generation;
- ambiguous and unknown source-field rejection;
- complete and incomplete conversation assembly;
- participant and role matching;
- world, mission, vehicle, interior, and local-player filters;
- locale selection and fallback;
- deterministic selection with identical seeds;
- selection-group exhaustion and reset;
- repeat suppression and usage revisions;
- optional probability acceptance and rejection;
- queue ordering and stable tie-breaking;
- duplicate suppression under repeated collision or traffic events;
- immediate preemption without double advancement;
- expiry before preparation, before start, and while paused;
- positional and non-positional playback;
- speaker unload and declared fallback;
- subtitle and mouth correlation;
- ducking reference counting;
- pause, resume, stop-current, stop-all, focus loss, and suspension;
- locale change during queued and active dialogue;
- feature removal and stale callback rejection;
- native concurrency rejection and recovery;
- headless domain execution without audio; and
- fixed-input replay producing identical selections and queue results.

## Invariants

- Shipping runtime never parses dialogue identity from filenames.
- Matching and variation are deterministic and revisioned.
- Selection success, queue admission, playback start, and playback completion
  are
  separate results.
- Queue entries own immutable copied state, not raw selectable-dialogue
  pointers.
- Audio, subtitle, mouth, and debug callbacks cannot commit gameplay.
- Required conversations never skip unresolved lines silently.
- Positional dialogue follows explicit speaker and listener policy.
- Every accepted queue entry reaches one terminal result exactly once.
- Ducking, subtitles, mouth presentation, assets, and callbacks release with the
  owning playback revision.
- Feature, locale, world, participant, and application transitions reject stale
  results.
