# HUD feedback cue and presentation-primitives runtime

- Status: Active
- Last reviewed: 2026-07-15

## Governing decisions and specifications

<!-- markdownlint-disable-next-line MD013 -->
- [HUD, radar, camera, and navigation parity](../../adr/unreal/ui/hud-radar-camera-and-navigation.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Common UI front end and progress projection](../../adr/unreal/ui/common-ui-frontend-and-progress-projection.md)
- [UI parity boundary](../../adr/unreal/ui/ui-parity-boundary.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Typed event and observation routing runtime](typed-event-and-observation-routing-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [In-game HUD, pause, and transition runtime](in-game-hud-pause-and-transition-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Progression, collectibles, cheats, and credits](progression-collectibles-and-cheats.md)
- [Race route and opponent runtime](race-route-and-opponent-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Semantic input, device, and haptics runtime](semantic-input-device-and-haptics-runtime.md)

## Purpose

This specification defines the native Unreal runtime for short-lived HUD
feedback
cues and the reusable presentation primitives they consume. It covers
collectible
and currency feedback, countdowns, notoriety messages, dropped-item warnings,
mission-objective and stage-progress cues, destroyed-target feedback, numeric
presentation, scrolling text, normalized sliders, radar icon projection, color
modulation, and reusable motion effects.

It replaces screen-owned event listeners, process-global singleton handlers,
mutable integer substate arrays, direct gameplay queries from animation code,
fixed-resolution coordinates, platform-specific timing branches, unbounded cue
overlap, and callbacks that can mutate a HUD after its player or source revision
has changed.

## Native Unreal composition

The runtime uses:

- `USharPlayerHudFeedbackSubsystem`, a `ULocalPlayerSubsystem`, for per-player
  feedback scheduling, accepted cue ownership, and local presentation revision;
- `USharSharedHudFeedbackSubsystem`, a `UGameInstanceSubsystem`, for explicitly
  shared viewport cues and cross-player arbitration;
- the typed observation router for immutable gameplay observations;
- C++ UMG viewmodels derived from `UMVVMViewModelBase` with `FieldNotify`
  values;
- Common Activatable Widgets and ordinary UMG widgets for cue surfaces;
- registered UMG animations or native timeline evaluators for presentation;
- Asset Manager primary assets and named bundles for cue art, audio, haptics,
  fonts, styles, and radar icons; and
- retained streamable handles for every accepted cue and HUD-map presentation
  lease.

Widgets render immutable projections and publish semantic dismissal or action
commands. They do not award collectibles, increment currency, complete a
mission,
change notoriety, unlock content, disable gameplay input, or mutate navigation
state.

## Ownership

<!-- markdownlint-disable MD013 -->

| Service | Authority |
| :--- | :--- |
| `USharPlayerHudFeedbackSubsystem` | Per-player cue queue, priority, coalescing, cancellation, and accepted presentation revision. |
| `USharSharedHudFeedbackSubsystem` | Shared countdowns, viewport-wide banners, and cross-player mutual exclusion. |
| `USharHudFeedbackViewModelSubsystem` | Immutable cue, numeric, slider, message, and radar projections. |
| Typed observation router | Delivery of accepted gameplay observations with source identity and revision. |
| Progression service | Cards, currency, gags, flying hazards, unlocks, and accepted completion state. |
| Mission service | Objective identity, stage progress, countdown policy, and mission terminal state. |
| Notoriety service | Warning, pursuit, arrest, fine, and accepted notoriety state. |
| Navigation service | Radar focal point, route, icon membership, visibility, and projection revision. |
| Input service | Semantic input leases and countdown-owned gameplay-input gating. |
| Presentation primitive library | Pure evaluation of color, opacity, transform, clipping, and normalized values. |

<!-- markdownlint-enable MD013 -->

No cue, widget animation, numeric formatter, slider, or radar icon may become a
second source of gameplay state.

## Runtime identities

Every accepted cue carries:

- `FSharHudCueRequestId` for one scheduling request;
- `FSharHudCueInstanceId` for one accepted running instance;
- `FSharHudCueDefinitionId` for registered presentation policy;
- `FSharHudCueChannelId` for arbitration and layout;
- `FSharPlayerHudId` or an explicit shared-viewport identity;
- `FSharObservationId` for the source gameplay observation;
- `FSharPresentationLeaseId` for loaded cue assets;
- exact source, local-player, HUD, catalog, and feature revisions; and
- optional mission, progression, navigation, or notoriety identities.

A late animation, timer, asset, audio, or haptic callback must match the
accepted
cue instance and every required revision. Stale callbacks are recorded and
ignored.

## Cue definition

`FSharHudCueDefinition` contains:

<!-- markdownlint-disable MD013 -->

| Field | Contract |
| :--- | :--- |
| `CueId` | Stable registered cue identity. |
| `ChannelId` | Layout and arbitration channel. |
| `Scope` | One local player, selected local players, or shared viewport. |
| `Priority` | Deterministic scheduling priority. |
| `ExclusionGroupId` | Optional mutual-exclusion group. |
| `CoalescingPolicyId` | Replace, accumulate, merge, ignore duplicate, or queue. |
| `MaximumQueued` | Validated bounded pending count. |
| `LifetimePolicyId` | Enter, hold, exit, timeout, and cancellation behavior. |
| `PresentationProfileId` | Widget, style, animation, layout, and safe-area policy. |
| `AccessibilityProfileId` | Narration, reduced motion, contrast, timing, and haptic alternatives. |
| `AudioPolicyId` | Optional cue, ducking, and concurrency behavior. |
| `HapticPolicyId` | Optional semantic haptic pattern. |
| `InputGatePolicyId` | Optional semantic gameplay-input lease. |
| `RequiredBundles` | Art, font, audio, haptic, animation, and icon bundles. |
| `FeatureOwnerId` | Base game or validated feature package. |

<!-- markdownlint-enable MD013 -->

Definitions are validated before registration. A widget asset cannot silently
supply missing priority, gameplay-input, coalescing, or completion policy.

## Cue states

One cue instance is in exactly one state:

- `created`;
- `queued`;
- `preparing`;
- `entering`;
- `holding`;
- `exiting`;
- `completed`;
- `cancelled`;
- `timed_out`; or
- `failed`.

Each state transition is monotonic. Re-entering a completed instance, completing
an instance twice, or returning from cancellation to presentation is invalid.

## Observation acceptance

A feedback request is built only from an accepted typed observation. The adapter
validates:

1. observation kind and payload schema;
1. local-player or shared scope;
1. source gameplay identity and revision;
1. cue definition and feature ownership;
1. current HUD and application mode;
1. visibility and suppression policy;
1. required asset readiness; and
1. duplicate or coalescing behavior.

The cue scheduler never infers completion by polling gameplay managers or
reading
mutable actors from a widget tick.

## Scheduler and arbitration

Each channel has one deterministic queue. Ordering is:

1. higher priority;
1. earlier accepted observation sequence;
1. stable cue-definition identity; and
1. stable request identity.

An exclusion group may allow only one entering or holding member. Replacement
policy declares whether the current cue exits, cancels immediately, or completes
before the replacement starts.

Queue capacity is bounded. Overflow uses the definition's explicit behavior:
reject newest, drop oldest noncritical cue, coalesce, or replace. Critical cues
cannot be silently discarded.

## Coalescing

Coalescing occurs only for requests with compatible player, channel, cue,
source, and feature revisions.

Supported policies include:

- accumulate a numeric delta and restart or extend the hold;
- replace content while preserving one running presentation lease;
- merge several unlocks into one ordered summary;
- ignore an idempotent duplicate observation; and
- retain each request as a separately narrated queue entry.

A currency cue may combine rapid accepted changes, while card identity,
mission-objective identity, arrest, and terminal completion remain distinct.

## Cue lifecycle

Starting a cue:

1. reserves its channel or exclusion group;
1. acquires required presentation bundles;
1. constructs the immutable viewmodel;
1. activates the cue widget in the owning HUD layer;
1. establishes optional audio, haptic, and input leases; and
1. begins the registered enter transition.

Completion releases only the owning widget, assets, audio, haptics, and input
lease. Cancellation publishes one typed terminal result and restores the next
valid cue or stable HUD state.

## Per-player and shared presentation

Per-player observations are presented only in the owning local player's safe
area and action context. Shared cues use a dedicated viewport layer and cannot
borrow a player's focus, narration queue, or input lease.

Split-screen layout may use compact presentation profiles, but it preserves the
same cue meaning, ordering, and accessibility content. A shared countdown may
render in each player viewport or once in the shared layer according to data.

## Card-collected feedback

The card cue projects:

- canonical card identity;
- localized title and optional quote identity;
- card artwork or validated fallback;
- accepted chapter collected count and total;
- chapter-set completion;
- complete-deck completion;
- ordered unlock results; and
- progression revision.

Card, chapter-set, and complete-deck states are distinct cue phases. Unlock text
is derived from committed progression results; the cue does not grant a movie,
track, reward, or achievement.

The base presentation may enter with a card-focused motion, hold the card and
count, then transition to completion and unlock content. Reduced-motion profiles
replace ballistic or large-scale motion with fades, restrained scale, and
narration-equivalent timing.

## Currency-collected feedback

Currency feedback uses the committed balance and optional accepted delta. Rapid
observations may coalesce into one numeric cue, but the displayed total always
comes from the latest accepted economy revision.

The cue has explicit enter, update, hold, and exit timing. Repeated updates do
not
allocate unbounded widgets or reset audio and narration without policy.

A currency visual cannot claim success for a transaction whose economy or save
result is unresolved.

## Countdown and input gating

A countdown definition contains ordered sequence units rather than hardcoded
text or frame names. A unit may declare localized text, number, sound, haptic,
duration, scale, and flash behavior.

The countdown may acquire a semantic gameplay-input gate. Input is enabled only
when the accepted sequence reaches its release unit and the owning gameplay
session confirms readiness. Stopping, cancelling, timing out, or replacing the
countdown releases its own gate exactly once.

A visual reaching zero cannot independently start a race or mission. It
publishes
one presentation-complete observation to the owning session transaction.

## Notoriety and arrest feedback

Notoriety feedback distinguishes warning, active pursuit, arrest, and terminal
fine results. It consumes the accepted notoriety state and never increments or
clears the meter.

Warning and arrest cues use separate priorities and exclusion behavior. Color,
text, shape, audio, and optional haptics communicate critical state; color alone
is insufficient.

## Dropped-item feedback

A dropped-item cue identifies the mission item, owning objective, local player,
and accepted drop observation. It may blink or pulse according to the registered
profile, then exits or is replaced by a restored-item observation.

The cue does not respawn, teleport, reserve, or reattach the item.

## Mission-objective feedback

Mission-objective feedback contains:

- mission and stage identities;
- objective presentation identity;
- localized objective text;
- registered icon identity;
- optional action prompt;
- source revision; and
- replacement or dismissal policy.

A missing optional icon uses a declared fallback. A missing required objective
identity fails validation instead of searching assets by filename or integer
index.

Objective replacement is atomic. A late asset load or exit animation from the
prior objective cannot hide the accepted replacement.

## Mission-progress feedback

Stage-complete and mission-progress cues are presentation of accepted mission
observations. They may flash, hold, or sequence with a mission-completion
banner,
but do not advance a stage or commit progression.

The mission service determines whether several rapid progress observations may
coalesce. Terminal mission results have higher priority than intermediate stage
feedback.

## Destroyed-target feedback

Flying-hazard and gag feedback projects accepted destroyed counts, totals,
chapter completion, and optional unlocks. The definition distinguishes hazard,
gag, mission target, and feature-owned target categories.

The cue cannot infer completion from actor destruction or actor unloading. It
uses the committed progression or mission result.

## Presentation primitives

Presentation primitives are pure deterministic functions. Their inputs are
validated elapsed time, duration, start and end values, curve identity, layout
scale, and accessibility policy. They return presentation values and never
mutate
gameplay.

The runtime provides registered primitives for:

- color and opacity interpolation;
- flash and blink cadence;
- pulse and restrained emphasis;
- pendulum and oscillation;
- horizontal and vertical slide;
- spiral and rotation;
- flip and scale transition;
- authored ballistic-looking motion; and
- clipping or fill progression.

A primitive with zero or negative duration, nonfinite input, invalid range, or
unsupported transform fails validation and returns the declared safe terminal
value.

## Color modulation

Color modulation interpolates between registered colors using a named curve and
phase. It preserves premultiplied-alpha and color-space policy selected by the
UI
style system.

Critical state is never encoded by color alone. High-contrast and
color-vision-accessible variants are part of the presentation profile.

## Blink, flash, and pulse

Blink and flash definitions declare minimum visible and hidden intervals,
maximum frequency, total duration, and reduced-motion replacement. Frequencies
that violate accessibility policy are rejected or clamped by validation.

Pulse changes a declared property around a stable base value. It cannot
accumulate scale or transform error across frames; each value is evaluated from
the original presentation state.

## Motion effects

Slide, pendulum, spiral, flip, and ballistic-looking effects evaluate from
immutable start state. They do not reuse world physics, collision, gravity, or
frame-rate-dependent integration.

Reduced-motion policy may replace movement with opacity, restrained scale, or an
instant state change while retaining equivalent information and minimum reading
time.

## Numeric presentation

`FSharNumericPresentation` contains value, optional minimum and maximum,
formatting policy, alignment, minimum digits, sign policy, localized grouping,
and presentation style.

Numbers use locale-aware formatting where language requires it. A stylized digit
atlas may render the resulting logical characters, but sprite names or digit
positions never become numeric authority.

Overflow, negative values, and unavailable values use explicit policies. Fixed
three-digit arrays and unchecked format buffers are forbidden.

## Scrolling text

Scrolling text consumes immutable localized text and a measured layout region.
It declares direction, speed, initial delay, pause points, repeat behavior,
clipping, narration policy, and cancellation behavior.

Layout measurement occurs after font, scale, locale, and safe-area resolution.
The system does not clip text by mutating the source string or assuming one font
has fixed character widths.

A stopped or replaced scroll restores the stable presentation state and cannot
continue updating a replacement widget.

## Normalized sliders

A slider projection contains normalized value, semantic minimum and maximum,
step, orientation, fill origin, accessibility label, and source revision.

Values are validated and clamped before presentation. Horizontal and vertical
fill variants are style policy, not different setting semantics. A two-way
settings control publishes a typed edit command; a gameplay meter remains
one-way to the widget.

Polygon or image clipping is an implementation detail. Widgets cannot write a
setting merely because a pointer or touch drag changed the visual fill.

## HUD-map icon registry

Radar icons use registered definitions and stable instance identities. An icon
projection contains:

- icon type and presentation profile;
- source entity or placement identity;
- owning local player;
- world position and heading;
- visibility modes;
- priority and decluttering policy;
- focal-point eligibility;
- distance and off-map behavior; and
- navigation revision.

Supported categories include players, owned vehicles, artificial-intelligence
vehicles, targets, checkpoints, waypoints, collectibles, missions, bonus
missions, phone booths, purchase locations, races, and feature-owned categories.

Fixed icon arrays are replaced by bounded registries and object pools.
Exhaustion
returns typed diagnostics and a declared decluttering result rather than
silently
reusing another active icon.

## Radar projection and camera

The radar presentation adapter consumes the navigation service's accepted focal
point, route, icon set, and road projection. It computes view-space icon
positions, headings, scale, fade, edge clamping, and cone presentation without
changing world navigation.

Camera height uses validated minimum, maximum, smoothing, and fixed-height
policy. Every value is evaluated from viewport shape, route extent, speed,
platform presentation, and accessibility data rather than compile-time platform
branches.

Nearest-road or route projection failure hides or degrades only the affected
optional presentation and records a typed reason. It cannot assert, move the
player, or manufacture a gameplay route.

## Asset readiness and leases

Cue widgets, icon atlases, fonts, styles, audio, haptics, and animation data use
soft references in registered primary assets. A cue enters only after its
required bundle is ready.

The owning cue retains its streamable handle until terminal cleanup. Replacement
cancels or releases the prior request only after ownership transfers safely.
Optional art failure uses the cue's declared fallback; required definition
failure rejects the cue without disturbing the stable HUD.

## Localization and accessibility

All player-facing text uses localized text identities and typed arguments.
Formatting occurs after locale selection and supports text expansion,
bidirectional layout where enabled, and locale-specific number formatting.

Each cue declares:

- narration content and interruption behavior;
- minimum reading and hold time;
- reduced-motion replacement;
- color-independent meaning;
- safe-area and split-screen layout;
- optional audio and haptic alternatives; and
- whether semantic dismissal is permitted.

A rapid visual-only cue still exposes equivalent persistent or narrated
information when required by accessibility policy.

## Feature and mod overlays

A validated feature package may register namespaced cue definitions,
presentation primitives, radar icon types, styles, and accessibility profiles.
It cannot replace base identities without an explicit overlay rule.

Feature removal cancels owned cues, releases owned assets, unregisters owned
icon
instances, and restores the next valid base presentation. It cannot leave a cue,
viewmodel, or radar icon referring to removed content.

## Concurrency

Gameplay observations may arrive while assets or animations complete. The
scheduler serializes cue acceptance per player and channel. Shared cues use one
separate shared sequence.

Only the owning cue instance may accept asynchronous completion. Numeric
updates,
coalescing, and radar snapshots use exact source revisions and cannot regress an
accepted projection.

## Diagnostics

The runtime records bounded structured diagnostics for:

- player, cue request, cue instance, channel, and observation identities;
- accepted source and HUD revisions;
- queue order, priority, exclusion, and coalescing decisions;
- asset and presentation-lease ownership;
- enter, hold, exit, timeout, and cancellation state;
- input, audio, haptic, and narration leases;
- numeric, slider, and radar projection validation; and
- stale, rejected, or overflowed requests.

Diagnostics use canonical identities and typed reasons, never raw local asset
paths or machine-specific locations.

## Failure behavior

- Unknown cue or channel identity rejects the request.
- Invalid player, source revision, or application mode rejects the request.
- Missing required assets preserve the stable HUD and return a typed failure.
- Missing optional art, icon, sound, or haptic data uses declared fallback.
- Queue overflow follows the registered bounded policy.
- Invalid numeric, slider, time, transform, or color input fails validation.
- Input-gate failure prevents a countdown from claiming gameplay readiness.
- Radar projection failure affects presentation only and never changes routing.
- Stale callbacks cannot hide, complete, or release a replacement cue.
- Feature removal cancels only feature-owned instances and assets.

## Validation

Validation proves:

- unique cue, channel, primitive, icon, and presentation-profile identities;
- bounded queue and icon-pool capacities;
- complete exclusion, coalescing, lifetime, and accessibility policy;
- valid durations, curves, ranges, colors, transforms, and normalized values;
- valid source observation schemas and player scopes;
- required bundles and fallback assets exist;
- countdown input gates have terminal release behavior;
- radar icon types have visibility and decluttering policy; and
- feature overlays cannot leave unresolved base references.

## Tests

Automated tests cover:

- deterministic channel ordering and tie breaking;
- duplicate suppression, replacement, accumulation, and queue overflow;
- card, currency, countdown, notoriety, dropped-item, objective, progress, and
  destroyed-target cue lifecycles;
- exactly-once completion and input-gate release;
- stale asset and animation callback rejection;
- per-player isolation and shared-viewport arbitration;
- reduced-motion, narration, contrast, and minimum-reading behavior;
- numeric formatting, slider clamping, and scrolling-text replacement;
- radar registration, focal-point replacement, decluttering, and projection;
- feature registration and teardown; and
- deterministic results at different frame rates.

## Invariants

- Gameplay observations are immutable inputs, not widget-owned state.
- Every running cue has one owner, one channel, and one terminal result.
- Cue queues and radar registries are bounded.
- Input, audio, haptic, asset, and widget leases are released exactly once.
- A visual countdown cannot independently start gameplay.
- Presentation primitives are pure and frame-rate independent.
- Numeric, slider, text, and radar widgets never become gameplay authority.
- Accessibility alternatives preserve cue meaning.
- Stale callbacks cannot mutate the accepted HUD.
- Feature removal restores a valid base presentation.
