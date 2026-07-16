# Presentation playback runtime

- Status: Active
- Last reviewed: 2026-07-15

## Governing decisions and specifications

<!-- markdownlint-disable-next-line MD013 -->
- [Typed StateTree action sequences](../../adr/unreal/runtime/typed-state-tree-action-sequences.md)
<!-- markdownlint-disable-next-line MD013 -->
- [HUD, radar, camera, and navigation parity](../../adr/unreal/ui/hud-radar-camera-and-navigation.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Native cinematic package strategy](../../adr/rmv/unreal-native-cinematic-package.md)
- [Local cinematic overrides](../../adr/rmv/local-movie-overrides.md)
- [Typed action-sequence runtime](typed-action-sequence-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Camera rig, preset, and arbitration runtime](camera-rig-preset-and-arbitration-runtime.md)
- [Platform cinematic media packaging](platform-cinematic-media-packaging.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Frontend screen flow and settings runtime](frontend-screen-flow-and-settings-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Frontend media, gallery, and audio runtime](frontend-media-gallery-and-audio-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Common UI navigation, menu, and modal runtime](common-ui-navigation-menu-and-modal-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Typed event and observation routing runtime](typed-event-and-observation-routing-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Spatial visibility, bounds, and culling runtime](spatial-visibility-bounds-and-culling-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Native asset load request and streaming runtime](native-asset-load-request-and-streaming-runtime.md)

## Purpose

This specification defines the shared lifecycle for animation, camera, cosmetic
character, sequence, and cinematic presentation playback. It replaces mutable
player singletons, uncorrelated asynchronous callbacks, package-section
identity,
global freeze and thaw calls, render-driven completion, and platform-specific
runtime behavior hidden behind aggregate translation units.

Presentation playback observes gameplay state and emits typed presentation
results. It does not own mission completion, dialogue progression, rewards,
character identity, camera authority, or save state.

## Native Unreal composition

The runtime uses:

- `USharPresentationPlaybackSubsystem`, a world-scoped subsystem, as the
  request,
  queue, playback-revision, cancellation, result, and teardown authority;
- the Asset Manager and retained streamable handles for presentation bundles;
- Level Sequence and its player for authored composite cinematic timelines;
- animation montages, animation sequences, Animation Blueprints, and Control Rig
  for character presentation;
- the camera subsystem for cuts, blends, targets, and restoration;
- Media Framework adapters for packaged cinematic media;
- Common UI and UMG for overlays, type-on text, and registered transitions;
- the localization manager, `FText`, String Tables, and culture fallback for
  player-facing text and localized media selection; and
- typed observations and terminal results for application, dialogue, mission,
  interaction, and frontend owners.

Level Sequence, animation, media, widget, camera, and render objects are bounded
adapters. None can become a second application or gameplay scheduler.

## Ownership

<!-- markdownlint-disable MD013 -->

| Service | Authority |
| :--- | :--- |
| Presentation playback subsystem | Owns request validation, lifecycle, cancellation, result publication, and teardown. |
| Asset Manager adapter | Resolves required assets and reports correlated load results. |
| Action-sequence runtime | Owns authored action order and gameplay-facing task results. |
| Camera subsystem | Owns camera requests, arbitration, view calculation, and restoration. |
| Character presentation service | Owns cosmetic animation layers such as blink and facial idles. |
| Media adapter | Owns platform playback handles and normalized media results. |
| Mission and interaction services | Consume validated presentation results without delegating authority. |

<!-- markdownlint-enable MD013 -->

A sequence player, animation instance, camera actor, media player, render layer,
or platform decoder is an adapter. None is the canonical request identity.

## Runtime topology

The runtime uses:

- `FSharPresentationRequestId`, a unique request identity;
- `FSharPresentationRevision`, one accepted playback revision;
- `FSharPresentationDefinition`, immutable playback policy;
- `FSharPresentationAssetSet`, required animation, camera, audio, and media
  data;
- `FSharPresentationLeaseId`, one scoped exclusivity or suppression lease;
- `FSharPresentationResult`, one normalized terminal result;
- `USharPresentationPlaybackSubsystem`, the world-scoped authority;
- repository-owned animation, camera, character, media, and render adapters; and
- typed observations consumed by mission, interaction, frontend, and action
  sequence services.

Every callback carries request, definition, world, owner, and asset revisions.
A callback from an older load, animation, camera, or media handle cannot
complete
or clean up a replacement request.

## Definition contract

`FSharPresentationDefinition` contains:

<!-- markdownlint-disable MD013 -->

| Field | Contract |
| :--- | :--- |
| `PresentationId` | Canonical presentation identity. |
| `PresentationKind` | Animation, camera, cosmetic layer, sequence, media, or registered composite. |
| `AssetSetId` | Required primary assets and load bundles. |
| `OwnerPolicyId` | Mission, interaction, frontend, world, or ambient owner policy. |
| `PlaybackPolicyId` | Start, loop, completion, and replay behavior. |
| `ExclusivityPolicyId` | Scoped input, simulation, camera, HUD, and world-presentation leases. |
| `SkipPolicyId` | Not skippable, immediate, hold, vote, accessibility, or owner-controlled. |
| `TimePolicyId` | Simulation, sequence, audio, or media time source. |
| `CameraPolicyId` | Optional camera request and restoration policy. |
| `CharacterLayerPolicyId` | Optional facial, blink, or additive animation policy. |
| `FallbackPolicyId` | Platform, accessibility, and missing-presentation fallback. |
| `ResultPolicyId` | Terminal results accepted by the owner. |
| `TeardownPolicyId` | Stop, restore, release, transfer, and asset residency behavior. |

<!-- markdownlint-enable MD013 -->

Definitions reject missing assets, unsupported presentation kinds, contradictory
skip and result policies, unbounded exclusivity, and teardown without a complete
release path.

## Playback states

One playback revision has exactly one state:

- `idle`;
- `loading`;
- `ready`;
- `starting`;
- `playing`;
- `paused`;
- `stopping`;
- `completed`;
- `skipped`;
- `cancelled`;
- `failed`; or
- `released`.

`completed`, `skipped`, `cancelled`, and `failed` are terminal results for the
owner. `released` means every adapter, lease, and transient asset handle has
been
released or transferred.

A request cannot render or update before `playing`. Loading completion creates
`ready`; it never starts playback without the owner revision still being valid.

## Request transaction

A playback request contains:

- canonical presentation identity;
- owner and owner revision;
- participant and target identities;
- world and gameplay-state revisions;
- requested start and end policies;
- skip and fallback policy;
- priority and arbitration data; and
- optional completion deadline.

Acceptance follows this sequence:

1. resolve and validate the definition;
1. validate owner, world, participant, and target revisions;
1. acquire required asset handles;
1. prepare exclusivity and camera requests without applying them;
1. construct animation, camera, character, or media adapters;
1. verify all required adapters;
1. commit leases and the active playback revision;
1. start every required adapter at the same accepted boundary; and
1. publish a playing snapshot.

Partial preparation rolls back assets, adapters, camera requests, input leases,
HUD suppression, and world-presentation effects.

## Queue and arbitration

Queued playback uses `FSharPresentationQueueEntryId`, a stable channel identity,
priority, insertion sequence, owner revision, and cancellation token. The queue
is bounded by registered channel policy rather than a compile-time event array.

A channel declares:

- maximum pending and active requests;
- priority and deterministic tie-break order;
- first-in, first-out behavior within equal priority;
- duplicate, replace, merge, and reject rules;
- exclusivity compatibility;
- starvation and deadline policy; and
- teardown behavior for queued and active requests.

Acceptance of a queue entry does not mean playback has started. One correlated
lifecycle publishes `queued`, `loading`, `ready`, `started`, and exactly one
terminal result. Begin, load-complete, marker, stop, and end observations carry
the queue entry, request, owner, world, and presentation revisions.

Clearing a queue is a typed cancellation transaction. It cancels each owned
pending or active entry, compensates committed leases, releases retained assets,
and records terminal results. It cannot reset pooled memory and silently discard
owner obligations.

A callback may start only the accepted head entry for its channel. Late load or
end callbacks cannot advance a replacement entry, release its leases, or invoke
an older raw callback object.

## Asynchronous loading

Asset loading is revision-correlated. A load request records:

- presentation and request identities;
- owner and world revisions;
- required and optional bundle identities;
- cancellation token;
- timeout and fallback policy; and
- resulting adapter construction plan.

A callback may publish only `ready`, `failed`, or optional degradation. It
cannot
start playback, mutate owner state, or release another request's assets.

Content already resident follows the same readiness barrier as newly loaded
content. Residency does not bypass validation or create a different start path.

## Culture and localized presentation

Presentation culture comes from the accepted Unreal localization environment,
including user preference, platform culture, product-supported culture, and the
engine culture-fallback chain. Hardware-language enums, platform compile-time
branches, filename suffixes, and media audio indices are adapter evidence rather
than portable identity.

A localized presentation request records:

- requested and resolved culture names;
- localization-target and String Table revisions;
- text, subtitle, voice, and media-variant identities;
- fallback chain and terminal fallback reason;
- number, date, line-break, and bidirectional-text policy; and
- accessibility alternatives.

Player-facing text remains `FText` until the final adapter boundary. A missing
translation follows the declared fallback chain and records a finding; it cannot
select an unrelated asset or change gameplay behavior.

Localized media selection validates packaged track or variant availability
before
playback commit. Culture changes supersede older readiness work and cannot swap
text, subtitles, or audio under an accepted playback revision unless the
definition explicitly supports a correlated live update.

## Type-on text presentation

Type-on text is a non-authoritative presentation adapter over immutable
localized
`FText`. It reveals Unicode grapheme clusters using locale-aware boundaries; it
does not insert terminators into a mutable string buffer or expose partial raw
storage to widgets.

`FSharTypeOnTextPolicy` declares:

- reveal rate and time source;
- punctuation and authored pause markers;
- minimum and maximum display duration;
- pause, resume, reveal-all, dismiss, and replacement behavior;
- narration synchronization;
- right-to-left and bidirectional layout support;
- reduced-motion and instant-text alternatives; and
- completion result mapping.

Pausing preserves the exact reveal cursor and accepted text revision. Resume,
reveal-all, timeout, user dismissal, replacement, and owner cancellation each
produce one correlated result. A stale timer cannot reveal or hide replacement
text.

## Exclusivity and scoped leases

Exclusive presentation never freezes arbitrary render layers or gameplay systems
by direct global mutation. The request acquires scoped leases for the exact
policies it needs, which may include:

- participant input suppression;
- mission clock suspension;
- AI or ambient presentation pause;
- HUD and prompt suppression;
- camera priority;
- world-presentation focus;
- dialogue skip routing; and
- audio focus.

Each lease has an owner, priority, world revision, restoration snapshot, and
release path. Nested compatible presentations compose through arbitration.
Incompatible requests wait, preempt according to policy, or fail with a typed
result.

Stopping, cancellation, owner replacement, world teardown, and feature removal
release the exact leases. A later presentation cannot be thawed or restored by a
stale earlier request.

## Animation playback adapter

An animation playback adapter binds:

- animation and target identities;
- target representation revision;
- montage, sequence, or animation asset;
- start section and normalized start time;
- loop and completion policy;
- root-motion and transform policy;
- additive or full-body layer policy;
- playback rate and time source;
- visibility policy; and
- terminal event mapping.

The adapter reports loaded, started, marker, loop, completed, interrupted,
cancelled, and failed observations. Animation notifies are presentation evidence
unless a registered action-sequence task explicitly validates them as a bounded
result.

Rendering and simulation are separate. Visibility loss, representation LOD, or
an off-screen target does not complete playback. A full-body animation that
loses
its target follows cancellation or recovery policy rather than continuing on a
stale pointer.

## Camera playback adapter

A camera playback adapter submits a typed request to the camera subsystem. It
contains:

- rig or camera definition identity;
- target snapshot identities;
- requested blend, cut, and restoration policy;
- priority and owning presentation revision;
- animation or sequence synchronization data; and
- fallback camera policy.

The camera subsystem owns view calculation and arbitration. The presentation
adapter receives accepted, active, preempted, completed, cancelled, and failed
results.

Loading a camera asset, resolving a camera name, or completing an animation does
not grant camera authority. Stopping playback releases the exact camera request
and restores the current valid camera policy, not a cached global pointer.

## Cosmetic character layers

Blinking, facial idles, breathing, and similar cosmetic layers are
non-authoritative character presentation. A cosmetic layer definition declares:

- eligible character and presentation profiles;
- animation or material channel;
- deterministic interval range and session seed;
- duration and blend policy;
- suppression tags;
- representation support;
- quality policy; and
- teardown behavior.

A blink scheduler derives intervals from the character presentation identity,
session seed, and accepted blink count. Frame rate and global random-call order
do
not change the resulting sequence.

Blinking pauses during incompatible facial animation, dialogue phoneme
ownership,
closed-eye states, representation swaps, and explicit presentation suppression.
It resumes with a new correlated schedule rather than replaying a stale timer.

Cosmetic layer failure cannot alter character state, dialogue, mission progress,
or interaction eligibility.

## Dialogue character presentation

Dialogue presentation consumes accepted speaker, listener, line, conversation,
character-representation, and audio revisions. It may request ambient idles,
speaking layers, listening layers, look-at presentation, facial curves, and
camera targets without owning dialogue progression.

A character presentation profile declares:

- eligible ambient and conversation animation identities;
- deterministic sequential or seeded selection policy;
- speaking, listening, interruption, and return-to-idle blends;
- facial, jaw, blink, and eye-look channels;
- Animation Blueprint and optional Control Rig bindings;
- representation and LOD support;
- missing-bone or missing-curve fallback; and
- cancellation and restoration behavior.

Ambient selection derives from conversation, participant, profile, and accepted
selection-count identities. Global random-call order and frame rate cannot
change
which animation is chosen.

Speaking begins and ends from typed dialogue or audio observations. Authored
facial curves, animation data, or validated audio-driven envelopes are
preferred.
A bounded procedural jaw fallback may be used only when declared by the profile;
it uses deterministic input, clamps to authored limits, and never writes a named
skeleton joint through an uncorrelated random tick.

Camera-per-line metadata submits ordinary camera requests. A dialogue line,
animation marker, mouth motion, or camera cut cannot advance the conversation or
mark the line accepted.

## Level Sequence and composite playback

Authored non-interactive scenes use registered Level Sequence definitions. A
sequence definition declares:

- sequence asset and playback range;
- display rate and tick resolution;
- camera, animation, audio, event, visibility, and effects tracks;
- deterministic frame or marker identities;
- intro, loop, outro, and completion policy;
- bound actor and spawnable resolution;
- skip and restoration behavior; and
- fallback for unsupported or missing optional tracks.

A legacy scene-graph or camera presentation converts to a validated sequence or
another registered composite definition. Source filenames, inventory sections,
and player-type enums do not survive as runtime authority.

Sequence evaluation is presentation evidence. Event tracks may publish typed
observations or satisfy declared presentation barriers, but they cannot call
mission, application, save, or progression mutation directly.

Simple animation playback uses the same request lifecycle. Intro, loop, and
outro ranges have explicit markers, and completion is derived from the accepted
player state rather than rendering or a guessed frame count.

## Registered transition composition

Fades, iris wipes, color changes, show and hide actions, transforms, spring-like
motion, scale pulses, spins, pauses, and event barriers are registered
presentation nodes. They compose as an immutable validated graph rather than raw
objects linked by pointers.

Each node declares:

- stable node identity and closed node kind;
- duration, time source, curve, and accessibility policy;
- input and output value schema;
- owned widget, overlay, camera, or visibility lease;
- predecessors and successors by checked identity;
- completion and cancellation result; and
- compensation and restoration behavior.

The graph rejects cycles unless the definition declares a bounded loop, missing
nodes, ambiguous joins, incompatible resource claims, and terminal paths without
cleanup. Parallel branches may run only when their leases are compatible.

A display node may request a screen route, input-state change, application-mode
transition, gameplay resume, or typed event through the owning application port.
Visual completion merely satisfies a presentation barrier; it cannot switch
context, resume simulation, change input authority, or mutate gameplay directly.

Reduced-motion policy may replace movement, spring, spin, pulse, or iris nodes
with an equivalent fade or instant semantic transition while preserving focus,
result, and owner-visible timing guarantees.

## Sequence and action integration

The typed action-sequence runtime may request presentation playback and wait for
a
registered result. The action task declares which results satisfy, skip, cancel,
or fail the task.

A sequence cannot inspect a mutable player state or render flag to infer
completion. It consumes one terminal `FSharPresentationResult` matching the
request and owner revisions.

Presentation playback may contain several animation, camera, audio, and media
adapters. The composite definition declares whether they start together, follow
a
barrier, run in parallel, or use an ordered handoff. Hidden callback order is
not
a sequencing mechanism.

## Skip and cancellation

A skip request contains participant, input action, request, owner, and policy
revisions. The playback subsystem validates eligibility and returns one of:

- `skip_not_allowed`;
- `skip_pending_hold`;
- `skip_pending_vote`;
- `skip_accepted`;
- `already_terminal`; or
- a typed invalid-context result.

Accepted skip executes the definition's compensation plan, which may advance
adapters to an authored end state, stop media, restore camera and leases, and
publish `skipped` to the owner.

Cancellation is distinct from skip. Owner replacement, mission abort, feature
removal, world teardown, target destruction, or load failure may cancel playback
without satisfying the owner's objective.

## Frontend cinematic input and recovery

Frontend and boot media requests declare minimum unskippable time, semantic skip
and confirm actions, pause policy, controller-loss behavior, audio focus,
fallback, and accepted owner results.

Raw decoder buttons, platform controller maps, and device indices never decide
skip or ownership. A skip before eligibility returns `skip_not_allowed`. An
accepted skip publishes one terminal result and executes the same camera, audio,
input, display, and asset restoration contract as normal completion.

Losing the assigned navigation device may pause playback and open one correlated
reassignment modal. Reconnection resumes only after the input subsystem accepts
a compatible local-player assignment. Duplicate disconnects cannot stack prompts
or revive stale callbacks.

The screen and recovery flow follows the
<!-- markdownlint-disable-next-line MD013 -->
[frontend screen flow and settings runtime](frontend-screen-flow-and-settings-runtime.md).

## Completion result

`FSharPresentationResult` contains:

<!-- markdownlint-disable MD013 -->

| Field | Contract |
| :--- | :--- |
| `RequestId` | Accepted request identity. |
| `PresentationRevision` | Exact playback revision. |
| `OwnerId` | Owning mission, interaction, sequence, frontend, or world identity. |
| `OwnerRevision` | Exact owner revision. |
| `ResultKind` | Completed, skipped, cancelled, failed, or unavailable fallback. |
| `CompletedAdapters` | Deterministic set of required adapter results. |
| `DegradedAdapters` | Optional presentation degradation. |
| `StartTick` | Accepted playback start. |
| `EndTick` | Accepted terminal boundary. |
| `FindingIds` | Typed load, playback, fallback, or teardown evidence. |

<!-- markdownlint-enable MD013 -->

The owner accepts a result once. Replayed callbacks, duplicate stop requests, or
render-loop observations cannot publish another terminal result.

## Platform media adapters

Platform-specific media adapters normalize decoder and input behavior behind the
same request and result schema. They may differ in codec, surface, buffering,
platform SDK, and packaging details, but they cannot differ in owner authority,
skip semantics, result kinds, or teardown guarantees.

Platform aggregate translation units contain no independent gameplay contract.
The media definition and packaging specification select the supported adapter at
build and runtime boundaries.

A missing required platform variant fails readiness. An optional accessibility
or presentation fallback must be declared and produce the same owner-visible
result policy.

## Overlay and render adapter

A presentation overlay is a scoped render and UI adapter owned by the accepted
playback revision. It may project a fade color, letterbox, texture, media
surface,
or diagnostic frame through native Unreal rendering and UMG.

The adapter does not own a process-global render layer, freeze or thaw the
world,
remove another subsystem's drawables, or infer playback completion from whether
it rendered. HUD suppression, world-presentation focus, camera priority, and
simulation pause are independent leases with independent restoration evidence.

Overlay construction, visibility, and destruction carry request, world, view,
and presentation revisions. A stale fade or render callback cannot reinitialize
the HUD, resume gameplay, restore an old camera, or remove a replacement
overlay.

Cinematic and overlay views follow
<!-- markdownlint-disable-next-line MD013 -->
[Spatial visibility, bounds, and culling runtime](spatial-visibility-bounds-and-culling-runtime.md).
Temporary camera or visibility changes do not become durable world or streaming
state.

## Update and render boundaries

Playback time uses the definition's declared time source. Update consumes
bounded
delta time and may advance adapter state. Render projects the last accepted
snapshot and cannot mutate lifecycle state.

Paused gameplay, paused media, world suspension, loading, and camera preemption
are separate states. The definition declares whether each pauses presentation
time, continues it, or cancels the request.

Variable rendering rates cannot change terminal results, skip eligibility,
cosmetic schedules, or restoration state.

## Streaming and feature lifecycle

Streaming may remove targets or assets while preserving owner state. The
playback
policy declares whether to pause, cancel, substitute a fallback, or retain the
required assets until terminal result.

Feature removal and mod deactivation:

- reject new requests owned by the feature;
- cancel or migrate active requests according to policy;
- release animation, camera, media, input, HUD, audio, and world leases;
- reject late callbacks by revision; and
- verify zero owned adapter and asset handles.

A target representation swap may transfer compatible cosmetic or animation
layers only through an explicit correlated handoff.

## Mod overlays

A validated package may add namespaced presentation definitions, animation and
camera bindings, cosmetic layers, media variants, and fallbacks. It must declare
platform support, dependencies, conflicts, resource limits, and teardown.

An overlay cannot replace another package's active request in place, weaken skip
or restoration policy, gain gameplay authority, or require unsupported platform
behavior without a declared fallback or packaging rejection.

## Diagnostics

Development diagnostics expose immutable snapshots of:

- presentation, request, owner, world, and asset revisions;
- lifecycle state and terminal result;
- required and optional adapters;
- active asset handles;
- input, camera, HUD, audio, and world leases;
- playback time and time source;
- queue channel, position, priority, and arbitration result;
- requested and resolved culture and localized variant;
- type-on text reveal cursor and accessibility policy;
- Level Sequence range, marker, and bound-object revisions;
- transition-graph node and owned leases;
- skip and fallback eligibility;
- cosmetic and dialogue-animation schedule state;
- target representation binding; and
- last load, playback, cancellation, restoration, or teardown finding.

Diagnostics may request a validated stop or skip in a test world. They cannot
publish an owner result or restore arbitrary global state.

## Failure behavior

The runtime fails closed on:

- missing or duplicate presentation identity;
- unsupported presentation kind or platform variant;
- stale owner, world, target, asset, or request revision;
- load completion without a matching request;
- playback start before readiness;
- unbounded queue, ambiguous priority, or starvation without policy;
- duplicate terminal result;
- invalid culture, text revision, media variant, or fallback chain;
- invalid Unicode reveal boundary or mutable text-buffer presentation;
- malformed Level Sequence binding, range, marker, or terminal path;
- cyclic or incomplete transition graph;
- dialogue animation or procedural facial motion without correlated ownership;
- invalid skip or cancellation context;
- lost target without a recovery or fallback policy;
- exclusivity without scoped leases;
- camera restoration using stale authority;
- render-driven lifecycle mutation;
- feature removal with unreleased handles; or
- presentation attempting gameplay or save mutation.

Failure returns typed evidence and restores the last accepted camera, input,
world, and owner state. It never guesses completion or silently leaves a frozen
system.

## Validation

Definition validation proves:

- every presentation and asset identity resolves;
- every kind has a registered adapter;
- required platform variants or fallbacks exist;
- skip, completion, cancellation, and fallback policies are compatible;
- every exclusivity effect uses a scoped lease;
- every adapter has a stop and release path;
- every owner result is revision-correlated;
- queue capacity, priority, replacement, and cancellation are bounded;
- culture, localized variant, and fallback chains resolve deterministically;
- type-on text uses immutable `FText` and Unicode reveal boundaries;
- every Level Sequence binding, marker, range, and terminal path resolves;
- every transition graph is bounded, acyclic or explicitly loop-bounded, and
  cancellation-safe;
- cosmetic and dialogue-animation schedules are deterministic; and
- overlays cannot gain authoritative gameplay behavior.

## Tests

Required automated tests include:

- resident and asynchronous load readiness equivalence;
- late load callback rejection;
- bounded queue ordering, replacement, coalescing, starvation, and clear;
- start, pause, resume, stop, complete, skip, cancel, fail, and release;
- nested compatible exclusivity leases;
- incompatible request wait, preemption, and rejection;
- camera request acceptance, preemption, completion, and restoration;
- culture fallback and localized media-variant selection;
- type-on grapheme reveal for combining marks, emoji, and bidirectional text;
- type-on pause, resume, reveal-all, replacement, and reduced-motion behavior;
- Level Sequence binding, marker, intro, loop, outro, skip, and cancellation;
- transition graph joins, parallel leases, bounded loops, and compensation;
- animation target loss and representation transfer;
- deterministic blink and ambient-dialogue schedules across frame rates;
- blink suppression during dialogue and facial animation;
- authored facial curves and bounded procedural fallback;
- skip hold, vote, denial, acceptance, and duplicate input;
- media platform result normalization;
- required media variant rejection and declared fallback;
- owner replacement and mission abort cancellation;
- world streaming and feature-removal teardown; and
- identical terminal results across supported quality presets.

## Invariants

- Every active playback has one canonical request and owner revision.
- Queue order and arbitration are bounded, deterministic, and
  revision-correlated.
- Loading completion never starts playback without owner revalidation.
- Culture and localized presentation resolve through one accepted fallback
  chain.
- Type-on text never mutates localized source storage.
- Level Sequence and transition graphs remain presentation adapters.
- Rendering never mutates lifecycle state.
- Exclusive presentation uses scoped leases with exact restoration.
- Camera authority remains in the camera subsystem.
- Cosmetic and dialogue animation cannot change gameplay or progression.
- One playback revision publishes at most one terminal owner result.
- Skip and cancellation are distinct typed outcomes.
- Platform adapters share one result and teardown contract.
- Every terminal request releases or transfers all owned handles.
