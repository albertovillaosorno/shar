# Native audio device, resource, player, and tuning adapter runtime

- Status: Active
- Last reviewed: 2026-07-16

## Governing decisions and specifications

<!-- markdownlint-disable-next-line MD013 -->
- [Native gameplay audio, dialogue, and listener boundary](../../adr/unreal/runtime/native-gameplay-audio-dialogue-and-listener-boundary.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Gameplay audio source, residency, mix, and environment runtime](gameplay-audio-source-residency-mix-and-environment-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Platform audio cooking and streaming](platform-audio-cooking-and-streaming.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Spatial audio listener and positional-source runtime](spatial-audio-listener-and-positional-source-runtime.md)
- [Music state and transition runtime](music-state-and-transition-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Presentation playback runtime](presentation-playback-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Application lifecycle and mode runtime](application-lifecycle-and-mode-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Native asset load request and streaming runtime](native-asset-load-request-and-streaming-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Device configuration and save-slot runtime](device-configuration-and-save-slot-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Developer command and diagnostic runtime](developer-command-and-diagnostic-runtime.md)

## Purpose

This specification defines the project adapter around Unreal's native audio
renderer. It covers audio-device readiness, output-device and channel-layout
policy, source assets, resource handles, compressed-audio readiness, Audio
Components, playback leases, source parameters, fades, pause and continuation,
concurrency admission, stream-cache pressure, Sound Class and modulation
control,
localized asset selection, callbacks, diagnostics, failure, and teardown.

It replaces a complete custom sound renderer built from process-global managers,
clip and stream player arrays, first-free player capture, manual resource
capture,
custom file instances, linked-list per-frame polling, manually partitioned sound
memory, swap regions, raw resource keys, source namespaces, script-created sound
objects, custom faders, tuning wiring graphs, callback pointers with untyped
user
data, and platform-specific renderer initialization.

The adapter is deliberately narrow. Unreal's Audio Mixer remains authority over
decoding, source voices, resampling, spatialization, mixing, submix processing,
stream caching, effects, audio-thread execution, device output, and platform
backends. Repository code owns semantic identities, validated configuration,
request correlation, ownership, policy, and typed results only.

## Native Unreal foundation

The implementation uses native facilities where applicable:

- `USoundWave`, Sound Cue, MetaSound Source, Dialogue Wave, and validated source
  assets;
- `UAudioComponent` and native component delegates;
- Sound Attenuation and Sound Concurrency assets;
- Sound Classes, Sound Mixes, Audio Modulation, Control Buses, and Control Bus
  Mixes;
- Sound Submixes, effect presets, source effects, buses, sends, and modulation;
- the Audio Mixer and platform audio device;
- the native audio stream cache and asynchronous I/O;
- Asset Manager primary assets, bundles, soft references, and retained handles;
- Audio Volumes or an accepted room and portal adapter;
- platform project settings, audio quality levels, output-device notifications,
  application focus, suspension, and resume; and
- Unreal Insights, audio statistics, logs, and development-only diagnostics.

A custom renderer may be introduced only by a separate accepted decision proving
that the native Audio Mixer cannot satisfy a required behavior. A compatibility
wrapper around source-era interfaces is not sufficient justification.

## Ownership

<!-- markdownlint-disable MD013 -->

| Authority | Responsibility |
| :--- | :--- |
| Gameplay and application services | Own semantic events, mode state, settings intent, world state, and durable results. |
| Gameplay-audio source service | Owns canonical source requests, source definitions, source leases, and terminal presentation results. |
| Audio residency service | Owns scope bundles, retained handles, readiness, references, and release. |
| Audio-device adapter | Observes device readiness, output changes, suspension, focus, and platform capability without replacing Unreal's renderer. |
| Audio-parameter adapter | Applies validated component parameters, Sound Class policy, Sound Mix or modulation state, and bounded fades. |
| Unreal Audio Mixer | Owns source voices, decoding, rendering, DSP, stream cache, effects, submixes, and hardware output. |
| Platform audio policy | Owns target cooking, channel-layout support, quality, memory, device, and focus behavior. |
| Settings service | Owns validated player-facing volume, output, dynamic-range, accessibility, and language preferences. |
| Developer diagnostics | Observe immutable renderer, source, residency, parameter, and device snapshots. |

<!-- markdownlint-enable MD013 -->

The adapter cannot infer gameplay success from audibility, a native playback
delegate, a fade delegate, stream-cache readiness, or output-device state.

## Runtime identities

Stable identities include:

- `FSharAudioAdapterRevision`;
- `FSharAudioDeviceId`;
- `FSharAudioDeviceRevision`;
- `FSharAudioOutputPolicyId`;
- `FSharAudioOutputRevision`;
- `FSharAudioSourceDefinitionId`;
- `FSharAudioSourceDefinitionRevision`;
- `FSharAudioAssetId`;
- `FSharAudioAssetRevision`;
- `FSharAudioResourceHandleId`;
- `FSharAudioResidencyScopeId`;
- `FSharAudioResidencyRevision`;
- `FSharAudioPlaybackId`;
- `FSharAudioPlaybackRevision`;
- `FSharAudioLeaseId`;
- `FSharAudioParameterSchemaId`;
- `FSharAudioParameterRevision`;
- `FSharAudioMixPolicyId`;
- `FSharAudioMixRevision`;
- `FSharAudioFadeId`;
- `FSharAudioFadeRevision`;
- `FSharAudioCallbackCorrelationId`;
- `FSharAudioSettingsRevision`;
- `FSharWorldCompositionRevision`;
- `FSharApplicationModeRevision`;
- `FSharLocalPlayerId`;
- `FSharFeatureRevision`; and
- `FSharAudioAdapterResultId`.

Object addresses, linked-list positions, fixed player slots, namespace
membership,
resource hashes, script load counts, array ordinals, file handles, and callback
addresses are not identity.

## Adapter topology

The runtime uses bounded services rather than one process-global renderer
object:

- a game-instance audio-device coordinator;
- a game-instance settings and mix coordinator;
- world-owned gameplay-audio source services;
- local-player listener projections;
- scope-owned residency services;
- presentation-owned music, dialogue, cinematic, and frontend adapters;
- feature-owned definitions and leases; and
- development-only diagnostic projections.

Each service exposes typed application ports. No gameplay module receives direct
access to an Audio Mixer device, source voice, decoder, stream-cache object,
native
submix instance, or platform backend.

## Audio-device readiness

`FSharAudioDeviceSnapshot` contains:

- device and adapter identities and revisions;
- initialized, suspended, recovering, unavailable, or headless state;
- output device and channel-layout capability;
- sample-rate and block-size observations when exposed safely;
- active quality policy;
- focus and background state;
- current world and application-mode revisions;
- supported spatialization, reverb, modulation, and plugin policy; and
- diagnostics correlation.

Application startup may proceed without an audio device only when the target and
mode explicitly support headless or silent operation. An interactive client that
requires audio reports a typed degraded or unavailable result rather than
pretending initialization succeeded.

Device readiness is not proven by constructing a project singleton, allocating a
silent buffer, registering a script loader, creating a listener pointer, or
observing one rendered frame.

## Initialization transaction

Interactive audio initialization follows a revisioned transaction:

1. resolve platform and project audio policy;
1. validate required native plugins and assets;
1. observe native Audio Mixer and output-device readiness;
1. establish master Sound Class, submix, modulation, and quality policy;
1. prepare required boot and frontend residency scopes;
1. apply validated player settings;
1. publish one current audio-device snapshot; and
1. release superseded preparation state.

Every step is cancellable. A stale initialization completion cannot reactivate
audio after shutdown, travel, device loss, application suspension, or a newer
recovery attempt.

The target does not register custom file formats, script factories, global
object
creation callbacks, fixed namespace arrays, platform memory heaps, or a manually
owned listener as part of normal audio startup.

## Termination transaction

Termination:

- rejects new source, fade, residency, and parameter requests;
- cancels preparation and pending callbacks;
- stops or transfers active sources according to policy;
- releases feature, world, local-player, frontend, cinematic, music, dialogue,
  and
  settings leases;
- clears project-owned Sound Mix, modulation, submix-send, and environment
  state;
- releases retained Asset Manager handles;
- unregisters project delegates;
- publishes zero owned playback and residency state; and
- allows Unreal to terminate or replace the native device.

Termination is idempotent. A callback that arrives after the adapter revision
has
ended is ignored and diagnosed.

## Output-device and channel-layout policy

Output policy uses native platform capability and validated user settings. The
closed logical layouts include:

- mono accessibility fallback;
- stereo;
- surround layouts supported by the target and selected device;
- headphones or binaural presentation when an accepted spatialization policy is
  active; and
- platform-default automatic selection.

The project does not hard-code bit flags for stereo or surround, directly wire
speaker channels, or assume that one historical console layout maps to every
current device.

An unsupported requested layout returns a typed fallback or rejection. Device
replacement, HDMI or headset changes, operating-system route changes, and mobile
focus changes create new device and output revisions.

## Output-device change and recovery

A device change transaction:

1. freezes new affected commits;
1. captures semantic playback, music, dialogue, mix, and listener state;
1. observes the replacement native device;
1. reapplies compatible project settings and mix policy;
1. restores, restarts, seeks, or cancels sources according to definition;
1. publishes a replacement device snapshot; and
1. releases stale native component and callback state.

No raw source voice or platform handle is transferred. Required uninterruptible
presentation declares its recovery guarantee; optional sources may end with a
typed device-change result.

## Source-asset definition

Each playable source resolves through a canonical definition containing:

- source and asset identities and revisions;
- validated native source class;
- role and Sound Class identity;
- loading, streaming, cache, and residency policy;
- looping and finite-duration policy;
- attenuation, concurrency, virtualization, and priority policy;
- parameter schema and default values;
- pitch, gain, filter, modulation, and randomization bounds;
- submix, bus, send, source-effect, and reverb policy;
- locale, platform, and quality variants;
- fallback source;
- required or optional status; and
- feature ownership and teardown.

A definition may contain several authored variants. Runtime variant selection is
deterministic from declared context and seed. A filename list or resource-key
array cannot silently choose the next playable file.

## Clip and stream policy

The source distinction between a fully resident clip and a streamed asset
becomes
native loading policy, not a separate fixed player class.

The policy may select:

- retain-on-load;
- prime-on-load;
- load-on-demand;
- stream caching;
- procedural or MetaSound generation;
- platform-specific compression and chunking; or
- an accepted specialized streaming adapter.

The choice belongs to cooking and runtime residency policy. Gameplay code does
not
request a clip-player slot or stream-player slot.

## Audio resource handles

A project resource handle represents validated semantic readiness, not ownership
of decoder memory. It records:

- handle, asset, definition, bundle, scope, owner, and feature identities;
- expected asset, definition, scope, world, mode, locale, quality, and feature
  revisions;
- required chunks or native readiness guarantee;
- retained Asset Manager handle when applicable;
- reference and release state;
- deadline, cancellation, and fallback; and
- terminal result.

The handle cannot expose native file data sources, compressed buffers, source
voices, decoder objects, platform memory addresses, or stream-cache slots.

## Resource readiness

Loading a `USoundWave` object does not by itself prove that every required audio
chunk is immediately playable. Definitions therefore declare the readiness level
required before commit:

- object resolved;
- first chunk primed;
- required start region primed;
- finite source fully resident;
- streaming source prepared within latency policy;
- procedural graph prepared; or
- best-effort optional readiness.

Readiness is measured through supported native APIs and accepted evidence. The
project does not maintain a parallel custom sound-memory allocator or manually
swap resource objects through fixed regions.

## Residency and stream-cache integration

Residency follows
<!-- markdownlint-disable-next-line MD013 -->
[Gameplay audio source, residency, mix, and environment runtime](gameplay-audio-source-residency-mix-and-environment-runtime.md).
The adapter translates a semantic residency policy into native Sound Wave
loading
behavior, Asset Manager retention, stream-cache priming, quality, and release.

Stream-cache pressure may produce prime delay, degraded optional readiness,
virtualization, admission rejection, or a typed required-content failure. It
never
silently reuses another resource's memory slot or invalidates an active source
without a result.

Cache trimming is a platform and pressure-policy operation. It cannot evict
protected active, queued, dialogue, music, cinematic, frontend, or accessibility
content.

## Playback lease

A playback lease contains:

- request, playback, lease, definition, asset, owner, and causation identities;
- expected definition, asset, device, world, mode, local-player, feature, mix,
  parameter, and residency revisions;
- native Audio Component weak reference after commit;
- source start, seek, loop, and time policy;
- positional or non-positional projection;
- parameters and modulation state;
- pause, virtualization, and stop state;
- callback correlation;
- cancellation and deadline; and
- exactly one terminal result.

The lease owns project correlation. The native component owns native playback.
A component pointer or component pool index cannot replace lease identity.

## Playback lifecycle

The closed lifecycle is:

1. `requested`;
1. `validating`;
1. `resolving`;
1. `loading`;
1. `priming`;
1. `prepared`;
1. `admitting`;
1. `starting`;
1. `playing`;
1. `virtualized`;
1. `paused`;
1. `fading`;
1. `stopping`;
1. `completed`;
1. `cancelled`;
1. `released`; or
1. `failed`.

Every accepted request reaches one terminal result exactly once. Native
delegates
may contribute evidence to a transition, but callback order is not the state
machine.

## Player admission

Source admission uses Sound Concurrency, native voice management, source
priority,
project significance, residency, stream, decoder, and quality policy.

Possible results include:

- admitted and audible;
- admitted and initially virtualized;
- queued by project policy;
- rejected by concurrency;
- rejected by voice or decoder budget;
- rejected by stream or residency pressure;
- superseded;
- cancelled; and
- failed because required content is unavailable.

The target never scans a fixed clip or stream array for the first uncaptured
player. Array order, allocation order, and linked-list order cannot select which
source wins.

## Native component ownership

The project creates or acquires an Audio Component through an accepted component
policy. The component is configured only after request validation and is bound
to
one lease revision.

Component pooling is optional and measurement-driven. A reused component must
reset:

- source asset;
- owner and attachment;
- auto-activate and auto-destroy behavior;
- location, rotation, and velocity projection;
- volume, pitch, filters, parameters, sends, and modulation;
- attenuation and concurrency settings;
- virtualization and priority;
- subtitle and dialogue correlation;
- delegates; and
- feature, world, and lease ownership.

Incomplete reset fails validation. A stale native delegate cannot act on a
replacement lease that reused the same component.

## Source parameters

All runtime parameters are declared by a schema. Supported kinds include:

- normalized or decibel gain;
- pitch ratio or semitone offset;
- low-pass and high-pass frequency;
- integer and enum switches;
- boolean gates;
- continuous graph parameters;
- source, surface, vehicle, environment, or participant identities;
- submix and send values within policy; and
- modulation destination values.

Values are finite, bounded, unit-labelled, and validated before commit. Gameplay
code cannot set arbitrary MetaSound, Sound Cue, component, or effect parameter
names.

## Pitch and variation

Pitch variation uses declared ranges and deterministic seed policy. Dynamic
pitch
updates carry source, parameter, and time revisions and are smoothed according
to
the definition.

A historical threshold used to avoid frequent player updates is implementation
evidence only. The target relies on native parameter behavior and measured
update policy, not a fixed global pitch-change constant.

## Gain and volume

Logical gain is composed from:

- source-definition gain;
- authored variation;
- instance parameter;
- role and Sound Class policy;
- user setting;
- application-mode mix;
- ducking;
- local-player and listener policy;
- environmental sends;
- accessibility policy; and
- native master output.

Every layer has a stable owner and revision. A project source does not multiply
several hidden trim fields stored in a custom player object.

## Fades

A fade request contains fade identity, owner, target role or source, start
value,
target value, duration, curve, time source, pause behavior, supersession,
cancellation, and callback correlation.

Fades use native Audio Component fades, Sound Mix or modulation transitions,
submix or effect transitions, or another accepted native facility. A manually
serviced fader array is not runtime authority.

A fade completion can satisfy one presentation barrier, release a lease, or
begin
a declared next audio transition. It cannot pause gameplay, complete a mission,
close a menu, or apply a setting without the owning application transaction.

## Pause, continue, and stop

Pause policy is role-specific. Application pause, cinematic pause, world
suspension, device loss, focus loss, frontend modal state, and user mute are
separate causes.

A source definition declares whether each cause:

- pauses source time;
- continues unchanged;
- ducks;
- virtualizes;
- fades and stops;
- cancels; or
- transfers to a replacement owner.

Continue revalidates the lease, owner, device, world, mode, and asset revisions.
A global iterate-all-players pause or continue operation is prohibited except as
an engine-level emergency owned by platform policy.

Stop is idempotent and correlated. It invalidates late readiness, fade, and
playback callbacks and leads to one terminal result.

## Callback correlation

Native readiness, playback-percent, finished, virtualization, device, and fade
delegates are converted into typed observations containing:

- correlation, request, playback, lease, and callback identities;
- native component weak identity;
- expected device, asset, owner, world, mode, and feature revisions;
- callback kind and timestamp; and
- accepted or stale result.

Untyped `void*` user data, raw callback interfaces, process-global callback
objects, and one callback field reused by several requests are prohibited.

Callbacks execute no domain mutation. They enqueue or publish an observation to
the owning service.

## Sound Class hierarchy

The canonical hierarchy follows semantic roles such as master, music, ambience,
dialogue, vehicle, sound effects, user interface, cinematic, accessibility,
frontend stinger, and namespaced feature children.

Parentage is authored and validated. A source cannot become a child of another
role because a script wiring function happened to run first.

Loading behavior, default submix, reverb participation, UI behavior, music
flags,
modulation, and quality policy are explicit assets and project settings.

## Sound Mix and Audio Modulation

Dynamic role control prefers one accepted project policy:

- Sound Mix and Sound Class adjusters;
- Audio Modulation with Control Buses and Control Bus Mixes; or
- a documented hybrid during migration.

The selected policy owns master, music, ambience, dialogue, vehicle, effects,
frontend, cinematic, accessibility, ducking, and temporary presentation
controls.

A custom tuner graph, hand-wired group paths, mutable slave-group tests, or
direct
iteration across every player is not target architecture.

## Ducking

Situational ducking is a scoped lease. Causes may include:

- pause or modal presentation;
- mission briefing;
- dialogue;
- letterbox or cinematic playback;
- store or frontend overlays;
- credits;
- local-player focus;
- accessibility narration; and
- explicit music-only presentation.

Each lease declares affected roles, target gain, attack, hold, release,
priority,
stacking, supersession, and cancellation. Combining leases is deterministic and
unit-tested. An enum ordinal or fixed matrix row cannot select ducking behavior.

## Music and ambience control

Music and ambience semantic state remain owned by
[Music state and transition runtime](music-state-and-transition-runtime.md).
The adapter only applies validated native source, parameter, mix, modulation,
submix, and device operations.

There are no fake player objects whose only purpose is to receive music or
ambience trim updates. Semantic music state and role gain are independent.

## Frontend, gameplay, and pause effects

High-level effects consume typed events and publish canonical audio requests.
Application modes define which event bindings are active.

Frontend bindings may include navigation accept, back, scroll, cheat accepted,
cheat rejected, settings preview, and accessibility feedback.

Gameplay bindings may include collision, footstep, jump, object kick, switch,
collection, door, mission presentation, warning, hazard, breakage, and other
validated cues.

Pause bindings may include open, close, continue, cancel, and settings-preview
cues.

The runtime does not instantiate one event-listener subclass per source-era
mode,
keep a six-player local array, or let a killable boolean and first-free slot
decide
priority. Event bindings, concurrency, significance, and queue policy decide.

## Options preview stingers

Car, dialogue, music, and effects volume previews use canonical preview
definitions. A preview request contains settings revision, role, target gain,
source definition, owner, local player, and cancellation.

The preview is routed so the role being adjusted remains audible without
applying
the setting twice. Accepting or cancelling a settings screen is an application
transaction; stinger completion cannot save the setting.

Only the newest preview for the same setting and owner may remain current unless
the definition explicitly allows overlap.

## Collision and positional effects

Collision and positional effects follow
<!-- markdownlint-disable-next-line MD013 -->
[Gameplay audio source, residency, mix, and environment runtime](gameplay-audio-source-residency-mix-and-environment-runtime.md)
and
<!-- markdownlint-disable-next-line MD013 -->
[Spatial audio listener and positional-source runtime](spatial-audio-listener-and-positional-source-runtime.md).

Distance, impulse, severity, surface, participant, world, and listener evidence
is
validated before request creation. A hard-coded distance square or arbitrary
impulse threshold may be retained only when promoted into a named, authored,
unit-labelled policy with tests.

## Sound-content loading

Level, mission, character, vehicle, frontend, minigame, dialogue-language, and
feature audio use canonical residency definitions and Asset Manager bundles.

Source-era cluster strings, vehicle tables, empty placeholder slots, file
archive
names, character namespace positions, and formatted resource keys are import
provenance only.

Load and unload are reference-counted by semantic scope. A mission unload cannot
release a source still protected by world, vehicle, dialogue, cinematic, or
frontend ownership.

## Locale and dialogue assets

Locale selection follows the localization and audio fallback contracts. A locale
change creates a new settings and audio-content revision, prepares required
localized bundles, atomically commits compatible dialogue and presentation
state,
and releases superseded locale assets.

The adapter does not register locale-specific archive filenames or expose a
fixed
four-language renderer enum as authority.

## Settings persistence

Player-facing audio settings are validated and persisted through the device and
save-slot runtime. Settings include only supported project policy, such as:

- master, music, ambience, dialogue, vehicle, effects, and interface volume;
- output layout or automatic selection;
- dynamic-range profile;
- subtitles and accessibility audio policy;
- selected dialogue language when separately supported;
- spatialization or headphone preference when exposed; and
- platform-supported device preference.

Settings are versioned, bounded, migrated, and recoverable. A raw property
count, float-string table, or singleton config interface is not persistence
architecture.

## Global tuning definitions

`USharGlobalAudioTuningDefinition` contains stable, unit-labelled values for:

- role defaults for master, effects, vehicle, music, dialogue, ambience, and
  interface audio;
- collision, footstep, skid, peel-out, and collection source bindings;
- bounded pitch, trim, gain, filter, and variation ranges;
- surface and physical-material routing;
- ducking policies by semantic cause;
- settings-preview definitions;
- accessibility and dynamic-range policy;
- target quality overrides; and
- definition and migration revision.

Source setter methods, mutable script-created global objects, raw clip-name
strings, ordinal ducking matrices, and unlabelled scalar constants are import
provenance only. Runtime consumes immutable validated definitions and current
settings projections.

Coin pitch or other repeated-pickup variation is derived from the canonical
source definition, accepted collection sequence, and stable deterministic seed.
It cannot use one process-global mutable pitch value whose update order changes
audible results.

## Game callback adapter

A project callback adapter may translate a native ready or finished delegate
into
one typed game observation. It owns one correlation at a time and supports:

- request cancellation before readiness;
- owner cancellation before completion;
- exactly-once ready delivery;
- exactly-once terminal delivery;
- release after terminal delivery; and
- stale callback rejection after replacement or teardown.

A completion check verifies request, playback, lease, owner, device, world,
mode,
feature, and callback revisions. Cancelling the game callback cannot leave the
native delegate able to invoke released gameplay state.

## Mode and lifecycle integration

Boot, frontend, loading, gameplay, pause, store, cinematic, credits, minigame,
focus-loss, suspension, and recovery publish application-mode revisions.

Each mode requests:

- required residency scopes;
- allowed source families;
- music and ambience policy;
- Sound Class, Sound Mix, modulation, and ducking state;
- listener policy;
- output and focus behavior; and
- teardown order.

A mode commits only after required audio preparation succeeds. Audibility and
playback completion cannot activate the mode.

## Feature overlays

A validated Game Feature may add namespaced source definitions, bundles, Sound
Class children, Control Buses, mix stages, submix sends, effect presets, event
bindings, diagnostic labels, and quality variants.

It cannot replace the master submix, audio device, protected Sound Class
hierarchy,
base output policy, platform codec policy, or another feature's active lease.

Feature removal cancels owned requests, releases residency and components,
removes modulation and mix state, clears delegates, rejects late callbacks, and
proves zero owned native and project resources.

## Diagnostics

The adapter publishes read-only snapshots for:

- device, output, focus, suspension, and recovery;
- source requests, leases, native components, and terminal results;
- source asset and stream-cache readiness;
- residency bundles and retained handles;
- concurrency, voices, decoders, virtualization, and priority;
- Sound Classes, Sound Mixes, modulation, buses, submixes, sends, and effects;
- fade, pause, stop, and callback state;
- locale and quality variants;
- stale callbacks and superseded requests; and
- feature and world teardown.

Development views may use supported native audio statistics and profiling. They
cannot capture a player, force a resource active, alter a mix, retain cache
data,
rewire classes, or invoke platform renderer internals.

## Headless and server operation

Dedicated servers and commandlets execute semantic gameplay without creating an
audio device, Audio Component, source voice, decoder, listener, or submix graph.

Audio requests may be omitted, recorded as non-authoritative telemetry, or
replicated as semantic presentation events for clients. Server correctness never
depends on a native playback callback.

## Concurrency and threading

Project audio definitions and request state are immutable or synchronized
through
explicit ownership. Game-thread services submit supported operations to native
audio facilities and consume returned delegates through typed correlation.

Repository code does not read or mutate audio-render-thread objects directly,
spin on native state, poll every source in one project per-frame loop, or expose
platform audio memory across threads.

## Failure behavior

Closed failures include:

- audio device unavailable;
- output policy unsupported;
- required plugin unavailable;
- source definition missing or stale;
- asset missing, invalid, or wrong class;
- required chunk not ready by deadline;
- residency rejected or cancelled;
- concurrency or voice admission rejected;
- decoder or stream pressure;
- component acquisition failed;
- parameter schema mismatch;
- unsupported channel layout;
- device changed during preparation;
- callback stale or duplicated;
- mix, modulation, submix, or effect policy invalid;
- locale or platform variant unavailable;
- feature or world removed;
- teardown leak; and
- unexpected native failure.

Optional presentation uses only declared fallback. Required presentation fails
its
own preparation transaction. Neither path mutates gameplay or durable state.

## Validation

Validation proves:

- every source definition resolves to a supported native source and role;
- every parameter is declared, bounded, and unit-labelled;
- loading and residency policy is compatible with the cooked source;
- protected content cannot be evicted while owned;
- output layouts and device policy are supported per target;
- Sound Class hierarchy and default submix routing are acyclic and complete;
- Sound Mix or modulation ownership is deterministic;
- every ducking lease has bounded attack and release;
- every playback request has exactly one terminal result;
- native callbacks reject stale lease and device revisions;
- component reuse performs complete reset;
- concurrency and pressure outcomes are explicit;
- frontend, gameplay, pause, and preview bindings use canonical identities;
- locale changes cannot mix incompatible dialogue revisions;
- feature and world teardown leaves zero owned resources; and
- headless operation requires no audio-native object.

## Tests

Automated tests cover:

- device initialization, absence, recovery, and replacement;
- stereo, surround, automatic, and unsupported output policy;
- clip-like, streamed, retained, primed, and load-on-demand sources;
- Asset Manager and stream-cache readiness barriers;
- required and optional residency failure;
- source admission under voice, decoder, and stream pressure;
- one-shot, queued, looping, positional, and controlled playback;
- pause, continue, fade, stop, cancel, supersede, and release;
- stale readiness, fade, playback, and device callbacks;
- parameter, pitch, gain, filter, modulation, and send updates;
- Sound Class, Sound Mix, Control Bus, ducking, and submix policy;
- frontend, gameplay, pause, options-preview, collision, and collection cues;
- music and ambience role control without fake players;
- level, mission, vehicle, character, locale, and feature bundle ownership;
- component reuse reset;
- settings migration and invalid values;
- device change during loading and active playback;
- world travel and feature removal;
- deterministic diagnostics; and
- dedicated-server execution.

## Invariants

- Unreal's Audio Mixer is the only default runtime audio renderer.
- Project code never allocates fixed clip or stream player arrays.
- Project code never captures the first free player as semantic admission
  policy.
- Native Audio Components are correlated through revisioned playback leases.
- Raw callback pointers and untyped callback data are prohibited.
- Source identity is never a filename, hash, namespace position, or player slot.
- Audio residency never uses custom fixed memory swap regions as authority.
- Sound Class, mix, modulation, and submix policy is asset- and
  definition-driven.
- Player settings cannot be committed by preview playback completion.
- A native audio callback cannot mutate gameplay or persistence.
- Required active audio cannot be silently evicted or replaced.
- Optional source rejection is deterministic and observable.
- Device changes invalidate stale native state.
- World and feature teardown leave zero owned audio leases and handles.
- Headless correctness requires no audio device.
