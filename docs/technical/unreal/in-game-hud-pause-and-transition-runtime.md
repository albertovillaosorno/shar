# In-game HUD, pause, and transition runtime

- Status: Active
- Last reviewed: 2026-07-17

## Governing decisions and specifications

<!-- markdownlint-disable-next-line MD013 -->
- [HUD, radar, camera, and navigation parity](../../adr/unreal/ui/hud-radar-camera-and-navigation.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Common UI front end and progress projection](../../adr/unreal/ui/common-ui-frontend-and-progress-projection.md)
- [UI parity boundary](../../adr/unreal/ui/ui-parity-boundary.md)
<!-- markdownlint-disable-next-line MD013 -->
- [State-driven missions, interactions, interiors, and notoriety](../../adr/unreal/runtime/state-driven-missions-interactions-and-notoriety.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Common UI navigation, menu, and modal runtime](common-ui-navigation-menu-and-modal-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [HUD feedback cue and presentation-primitives runtime](hud-feedback-cue-and-presentation-primitives-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Local split-screen minigame session UI runtime](local-split-screen-minigame-session-ui-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Mission, interaction, interior, and notoriety runtime](mission-interaction-and-notoriety-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Semantic input, device, and haptics runtime](semantic-input-device-and-haptics-runtime.md)
- [Camera system runtime](camera-system-runtime.md)

## Purpose

This specification defines the native Unreal runtime for the in-game user
interface, per-player HUD, expanded map, cinematic barriers, screen fades,
letterboxing, split-screen presentation, pause flows, mission and sandbox pause
menus, pause-time settings, controller-loss handling, tutorial presentation, and
post-media credits handoff.

It replaces global integer messages, fixed widget arrays, screen-owned gameplay
state, direct polling of controller buttons, platform-specific pause subclasses,
mutable transition counters, and callbacks that can update a screen after its
flow has been replaced.

## Native Unreal composition

The runtime uses one fixed composition of native facilities:

- `UCommonGameViewportClient` routes Common UI input through the viewport;
- Common Activatable Widgets own activation, deactivation, focus, and action
  presentation;
- Common UI action data owns abstract click, back, pause, map, skip, confirm,
  cancel, and screen-specific user-interface actions;
- `UGameInstanceSubsystem` services own shared in-game flow and transitions;
- `ULocalPlayerSubsystem` services own per-player HUD, focus, input method, and
  split-screen state;
- C++ UMG viewmodels derive from `UMVVMViewModelBase` and publish `FieldNotify`
  values to widgets;
- Asset Manager primary assets and bundles own HUD themes, icons, maps,
  transitions, tutorials, and pause presentation dependencies; and
- retained streamable handles keep each accepted presentation lease resident
  until it is replaced or released.

Gameplay Enhanced Input mappings remain separate from Common UI action data.
Widgets consume viewmodels and publish typed commands; they do not read gameplay
actors, mission objects, save files, or physical controller keys directly.

## Ownership

<!-- markdownlint-disable MD013 -->

| Service | Lifetime | Authority |
| :--- | :--- | :--- |
| `USharInGameUiSubsystem` | Game instance | In-game screen routing, overlay catalog, transition leases, pause policy, and accepted flow revision. |
| `USharPlayerHudSubsystem` | Local player | Per-player HUD projection, HUD visibility policy, focus, and local presentation revision. |
| `USharInGameViewModelSubsystem` | Game instance | Immutable shared and per-player viewmodel construction from accepted domain snapshots. |
| `USharUiNavigationSubsystem` | Game instance | Common UI layers, navigation transactions, history, modals, and asset leases. |
| Application lifecycle service | Game instance | Gameplay, pause, loading, cinematic, frontend, and shutdown mode transitions. |
| Mission service | Application | Mission stage, timers, objective state, failure, completion, restart, abort, and skip eligibility. |
| Progression service | Application | Currency, collectibles, cards, level progress, tutorials, and unlock state. |
| Notoriety service | World | Accepted notoriety value, warning state, pursuit state, and arrest result. |
| Camera and navigation services | World and local player | Camera policy, route guidance, radar markers, expanded-map data, and safe map bounds. |
| Device-configuration service | Application | Settings drafts, previews, commit, rollback, and device-local persistence. |

<!-- markdownlint-enable MD013 -->

No widget, animation, viewmodel, or HUD event adapter may become the authority
for mission, progression, currency, notoriety, vehicle health, camera mode, save
state, or application mode.

## Runtime identities

Every operation carries explicit identity:

- `FSharInGameUiFlowId` and `FSharInGameUiFlowRevision` identify the accepted
  in-game screen flow;
- `FSharPlayerHudId` identifies one local player's HUD;
- `FSharPlayerHudRevision` changes whenever that HUD projection is replaced;
- `FSharHudOverlayId` identifies one registered overlay definition;
- `FSharHudObservationId` correlates one accepted gameplay observation;
- `FSharUiTransitionLeaseId` correlates one fade, iris, or letterbox barrier;
- `FSharPauseSessionId` identifies one pause transaction;
- `FSharTutorialPresentationId` identifies one tutorial projection; and
- source revisions identify the mission, progression, currency, navigation,
  settings, local-player, and world snapshots used by the viewmodels.

An input or callback without the expected owner and revision is stale. Stale
asset loads, transition completions, timer observations, controller events, map
results, settings previews, or mission results are recorded and ignored.

## In-game flow states

One accepted in-game user-interface flow is in exactly one state:

- `initializing`;
- `active`;
- `overlay_blocked`;
- `cinematic`;
- `transitioning`;
- `pause_requested`;
- `paused`;
- `recovering`;
- `leaving_gameplay`; or
- `terminated`.

The flow may own multiple non-blocking HUD overlays, but only one blocking
transition, cinematic barrier, pause transaction, or recovery modal at a time.
A visual animation never changes the application mode by itself.

## Per-player HUD viewmodel

`USharPlayerHudViewModel` is a manually assigned C++ UMG viewmodel per local
player. It exposes immutable or one-way-to-widget fields for:

- current mission and objective presentation;
- timer and par-time presentation;
- collectible progress;
- race position and route-validity state;
- lap progress;
- vehicle damage and proximity values;
- notoriety value and pursuit state;
- currency balance and recent currency delta;
- action prompt and contextual label;
- message queue head;
- radar visibility and marker snapshot;
- transition opacity and input barrier;
- tutorial presentation; and
- accessibility and safe-area profile.

The subsystem updates the viewmodel only after accepting a complete source
snapshot. C++ setters use field notifications only when a value actually
changes. Widgets never bind directly to gameplay actors or poll values every
frame when a source observation is available.

## HUD overlay catalog

`FSharHudOverlayDefinition` contains:

<!-- markdownlint-disable MD013 -->

| Field | Contract |
| :--- | :--- |
| `OverlayId` | Canonical overlay identity. |
| `WidgetClass` | Cooked Common UI or UMG widget class. |
| `LayerId` | Per-player HUD, shared cinematic, or registered feature layer. |
| `ViewModelSchemaId` | Accepted projection schema. |
| `Priority` | Stable stacking priority. |
| `MutualExclusionGroup` | Optional group that permits only one visible member. |
| `CompactPolicyId` | Layout policy when several mission overlays coexist. |
| `LifetimePolicyId` | Persistent, timed, observation-owned, or command-owned. |
| `InputPolicyId` | Pass-through, action-only, or blocking. |
| `RequiredBundles` | Presentation dependencies retained while visible. |
| `AccessibilityProfileId` | Narration, contrast, motion, timing, and text policy. |
| `FeatureOwnerId` | Base game or validated feature package. |

<!-- markdownlint-enable MD013 -->

The base catalog includes timer, temporary timer, par time, collectible count,
race position, lap counter, damage meter, proximity meter, notoriety meter,
mission result banner, action prompt, message panel, radar, currency, tutorial,
and transition cover overlays.

## Overlay composition

The subsystem resolves visible overlays from accepted definitions and source
snapshots. It does not rely on widget child indexes.

Mission overlays use stable priority and compact-layout rules. When several are
visible, the layout service preserves all required information, scales only
within declared readability bounds, and moves lower-priority overlays into
reserved slots. It never silently drops the third overlay or depends on creation
order.

An overlay request validates:

- overlay identity and feature owner;
- local-player and HUD revisions;
- required source snapshot;
- mutual-exclusion and blocking policy;
- required assets and styles;
- safe-area and accessibility constraints; and
- lifetime ownership.

A rejected request leaves the accepted HUD unchanged and returns a typed reason.

## Observation and update model

Gameplay services publish typed observations. The HUD subsystem coalesces them
by source identity and revision, builds one immutable projection, and updates
viewmodels on the game thread.

High-frequency values such as vehicle damage, proximity, timer, and notoriety
may update at a bounded presentation rate. The final source value is never lost.
Low-frequency events such as card collection, lap change, mission completion,
and tutorial activation are queued exactly once.

A late observation from a prior mission, vehicle, world, or local-player
assignment cannot affect the current HUD.

## Timer and par time

The timer projection contains:

- source timer identity;
- remaining monotonic duration;
- formatted minute-and-second text;
- warning interval;
- warning cadence curve;
- warning color or accessible alternative;
- audio cue policy;
- visibility threshold; and
- source revision.

The display uses `minutes:seconds` with two second digits. A timer whose
accepted remaining value is at or below the configured visibility threshold is
hidden.
Warning cadence accelerates only through data. Color is never the sole warning;
shape, text, sound, or haptic alternatives remain available.

A condition-owned timer may temporarily replace the mission timer through the
mutual-exclusion catalog. Par time is a separate read-only value and cannot
change mission timing.

## Race, lap, and collectible projections

Race position contains current rank, participant count, ordinal presentation,
and route-validity state. When the route service reports that the player has
remained off route beyond the declared grace period, rank is presented as
unknown until a valid route observation returns.

Lap presentation contains the current lap, total laps, and a one-shot lap-change
observation. A lap animation or sound never increments the lap.

Collectible presentation contains collected count, required count, item type,
and objective revision. A visual pulse acknowledges an accepted increment but
cannot grant or remove a collectible.

## Damage and proximity meters

Damage and proximity values use normalized integer domain values. The viewmodel
provides current value, threshold bands, direction, color token, and accessible
label.

Damage warning behavior begins only at a declared threshold. Proximity may use a
positive or negative semantic profile, such as stay close or stay away. Widgets
never infer the semantic meaning from a color.

## Notoriety meter

The notoriety projection contains accepted normalized value, warning threshold,
pursuit state, arrest state, decay state, and presentation segment count.

A radial or segmented widget may reproduce the intended appearance, but its
segments are derived presentation. The notoriety subsystem remains the only
state authority. Warning blinking, sound, and color are presentation policies,
and they cannot start or end pursuit.

## Messages and action prompts

HUD messages use a bounded typed queue. Each entry contains:

- message identity and localized arguments;
- priority and replacement group;
- display duration and transition policy;
- source and owner revisions;
- accessibility timing override; and
- optional acknowledgement action.

Overflow follows a declared drop or replacement policy and produces a
diagnostic. It never asserts or corrupts adjacent state.

The action prompt contains one semantic action, localized label, glyph context,
availability reason, and interaction revision. Controller, keyboard, mouse, and
touch glyphs are projections of the same action.

## Mission completion and failure banners

Mission result banners are projections of accepted mission results. The base
profiles distinguish standard completion, bonus completion, wager result, and
failure. Result text, art, sound, and duration are data-driven.

A HUD banner cannot commit progression, debit a wager, choose the next mission,
or reopen gameplay. It acknowledges only a result already committed by the
mission service.

## Currency presentation

Currency presentation consumes the accepted currency ledger revision. It shows
balance and optional recent delta, then coordinates the numeric widget with the
world-space or screen-space coin icon.

The HUD never increments or decrements currency. A lost-coin or collected-coin
animation is driven by an accepted ledger entry and cannot be replayed as a new
transaction.

## Radar and expanded map

The radar viewmodel contains player transform, camera heading, route polyline,
marker snapshots, marker priorities, visibility policy, and navigation revision.
Each marker uses canonical gameplay identity and a registered presentation
profile.

The expanded map is a blocking Common UI screen that consumes the same
navigation snapshot. It may expose pan, zoom, recenter, filter, and route
actions. Camera height and target are presentation state constrained by
declared map bounds; they do not mutate world cameras or navigation identity.

Radar movement, resizing, and safe-area placement use responsive layout data.
A development-only movable-radar tool remains outside shipping behavior.

The base mission radar profile declares a fixed world-range policy rather than
speed-driven auto-zoom. Its exact validated range is data, not a compile-time
constant. Free-roam may use an `auto` visibility policy that reveals only the
next eligible story-mission start marker when the player enters its declared
range. Mission profiles may add artificial-intelligence vehicles, pursuit
vehicles, checkpoints, objectives, collectibles, and key locations.

Off-range moving threats and mission vehicles may use edge-clamped directional
icons. Off-range collectibles and ordinary locations are hidden unless their
registered profile explicitly permits another behavior. Pursuit icons have a
distinct pulse, color, or animation profile without changing pursuit authority.
Radar settings expose registered `auto` and `off` policies, while route guidance
has its own independent setting. A decorative sweep, glow, pulse, or fade is
presentation only and cannot discover, select, or complete an objective.

## Fade transaction

A fade request contains owner, source and target opacity, duration, easing,
input policy, completion event, and cancellation policy.

Only one fade lease may own the shared cover. Duplicate equivalent requests are
idempotent. Conflicting requests are rejected or replace the current lease only
according to explicit policy.

Completion is accepted only after the target opacity is reached and the request
revision still matches. Cancellation restores the declared terminal opacity and
publishes one terminal result. Pause input is blocked while a blocking fade owns
the cover.

## Iris and letterbox barriers

Iris and letterbox presentation use registered transition definitions rather
than screen-specific frame ranges.

An iris lease has `closing`, `closed`, `opening`, and `open` states. A closed
iris may serve as an asset or world readiness barrier. The flow cannot resume
merely because an animation reached its midpoint; the owning transaction must
also report readiness.

A letterbox lease declares top and bottom bars, screen blanking policy, accept,
cancel, and skip actions, optional button suppression, and whether the next
transition begins open or closed. Cinematic skip is available only when the
presentation request allows it. Reduced-motion profiles replace moving bars and
iris motion with an equivalent accessible transition.

## Split-screen and local bonus HUD

Each joined local player owns one HUD subsystem, viewmodel, safe area, radar,
vehicle telemetry projection, and semantic action context. Shared overlays use a
separate viewport layer and never borrow one player's focus.

Local bonus-mode HUD definitions may expose speed, route markers, opponents,
waypoints, finish markers, collectibles, and reset actions. Membership comes
from the active local-session definition, not from hardcoded widget names.

The built-in local split-screen minigame uses the complete session, lobby,
controller, loading, pause, summary, and teardown contract in
<!-- markdownlint-disable-next-line MD013 -->
[Local split-screen minigame session UI runtime](local-split-screen-minigame-session-ui-runtime.md).
Local split-screen identities remain separate from future network-session or
server identities.

A manual reset request is a typed gameplay command. The HUD cannot move a
vehicle or respawn a player directly.

## Tutorial presentation

`USharTutorialSubsystem`, a game-instance subsystem, owns tutorial definitions,
profile-scoped completion, eligibility projections, the bounded pending queue,
and the currently accepted tutorial request. World-scoped gameplay observations
are normalized by the typed observation router before eligibility evaluation.

A tutorial definition contains:

- canonical tutorial identity and definition revision;
- typed observation predicates that may propose it;
- participant, world, chapter, mission, interaction, and vehicle filters;
- prerequisite and mutual-exclusion tutorial identities;
- repeat, once-per-profile, once-per-save, and session policy;
- localized text, optional art, narration, and prompt actions;
- blocking, timing, priority, coalescing, and replacement policy;
- dismissal and permanent-disable eligibility; and
- accepted completion and persistence behavior.

Raw event numbers, enum ordinals, listener removal, mutable queue slots, and bit
positions are not tutorial identity or eligibility authority.

An observation proposes a tutorial; it does not immediately display or mark it
seen. The subsystem validates the exact observation, participant, world,
progression, settings, catalog, and tutorial-state revisions. It then rejects,
coalesces, replaces, or enqueues the proposal according to deterministic
priority
and stable identity order.

The queue is bounded. Only one blocking tutorial may own a local player's focus
at a time. Shared tutorials use an explicit viewport scope and cannot borrow one
player's action context. A terminal tutorial result triggers eligibility
re-evaluation before the next request starts.

Prerequisites are data-driven. For example, a traffic-vehicle tutorial may
require the accepted player-vehicle entry tutorial, and race, bonus-mission, and
wager tutorials remain separate identities even when proposed by one observation
family. Completing or suppressing one definition cannot silently mark another
seen.

Tutorial presentation contains tutorial identity, localized text, optional art,
blocking policy, accepted actions, narration, timing, and source revision.

A blocking tutorial uses a Common UI modal or declared letterbox layer and owns
one semantic focus lease. Its projection includes the exact tutorial revision,
input method, prompt glyph data, minimum reading time, and whether dismissal or
permanent tutorial disablement is currently eligible.

Confirm, dismiss, and disable-tutorial actions are distinct typed commands.
Disabling future tutorials updates device or profile configuration through a
verified settings transaction; it does not retroactively complete gameplay or
silently dismiss another player's tutorial. Separately disabling observation
proposals for a test or special mode is transient session policy and cannot
rewrite persistent completion.

Portable state stores canonical accepted tutorial identities with schema and
source revisions. A generated bitset may be a versioned storage representation,
but enum position never becomes domain identity. Loading validates unknown,
removed, aliased, or feature-owned tutorials before publishing eligibility.

A tutorial cannot mark itself completed. Completion is accepted only after the
tutorial service commits the exact terminal result. Closing, cancelling, or
superseding the tutorial releases focus, narration, input, and asset leases
exactly once. A late dialog, animation, observation, save, or settings callback
cannot close, complete, or mark seen a replacement tutorial.

## Pause request transaction

A pause request validates:

- application and in-game flow revisions;
- local-player ownership;
- mission and cinematic policy;
- transition and loading barriers;
- conversation or interior-transition ownership;
- platform suspension state; and
- existing pause or recovery sessions.

If accepted, the lifecycle service suspends pause-sensitive simulation, creates
`FSharPauseSessionId`, captures the prior HUD and focus state, activates the
pause layer, and verifies semantic actions. Audio, haptics, camera, and timers
follow their declared pause policies.

A rejected request leaves gameplay active and returns a typed reason.

## Mission pause menu

The mission pause projection may expose:

- continue;
- expanded map;
- restart mission;
- abort mission;
- options; and
- quit to frontend or system where supported.

Availability comes from the active mission and platform policies. Restart and
abort require confirmation when the policy declares destructive consequences.
The widget publishes a command; the mission service performs cancellation,
checkpoint restoration, or reload.

## Sandbox pause menu

The sandbox pause projection may expose:

- continue;
- mission replay selection;
- level progress;
- collected cards;
- options;
- save game;
- quit to frontend; and
- exit application where the platform permits it.

Save and mission replay remain independent transactions. A pause animation or
menu selection cannot claim that a save completed or that a replay loaded.

## Pause options and settings

Pause options project registered settings categories. Base categories include
controller and input, audio, display, camera, gameplay settings, radar,
vibration, tutorials, and accessibility where applicable.

Opening a category creates or reuses one correlated settings edit session. The
pause widgets edit a draft and may request scoped previews. Back cancels the
draft or returns to the prior category according to policy. Commit validates and
persists the complete draft atomically.

Camera choice, jump-camera behavior, inversion, navigation intersection, radar,
haptics, and tutorial visibility are typed fields with capability and mission
predicates. Cheats or development capabilities may expose additional values
through explicit feature data; they do not alter the shipping schema silently.

Controller diagrams and display illustrations are presentation assets. They do
not define bindings, supported modes, or device capability.

## Controller loss and recovery

Disconnect observations identify the physical device, local player, assignment
revision, and pause session. The affected player receives a blocking recovery
modal while other local players retain their own state according to session
policy.

Reconnect validates assignment before dismissing recovery. A different device
may be assigned through the input subsystem; widget focus alone cannot claim the
controller.

## Post-media credits handoff

A post-media credits screen is a presentation continuation owned by the accepted
credits sequence. It may show additional artwork or character presentation,
allow semantic skip or back where policy permits, and then return to the
declared frontend or gameplay destination.

Media completion, credits completion, and navigation completion are distinct
observations. A late media callback cannot close a replacement credits screen.

## Accessibility and responsive layout

Every HUD and pause definition declares:

- safe-area anchors and split-screen behavior;
- supported aspect ratios and display densities;
- text scaling and localization expansion;
- color-independent state cues;
- narration and focus order;
- reduced-motion alternatives;
- minimum prompt and tutorial duration; and
- touch hit-target requirements.

Low, Medium, High, Epic, and Ultra use the same gameplay and user-interface
state. A graphics preset may change rendering cost but cannot remove required
information or actions.

## Feature and mod overlays

A validated feature package may add overlays, pause entries, tutorials, map
markers, or presentation profiles through namespaced definitions. It cannot
replace a base identity without an explicit override contract.

Feature removal cancels owned leases, removes owned definitions, restores a
valid base screen, and leaves no stale viewmodel or input action.

## Concurrency

State mutation is serialized on the game thread. Asset loads, navigation
queries, and save or settings operations may complete asynchronously, but their
results are accepted only through request identities and source revisions.

Per-player HUD updates do not share mutable widget state. Shared transitions and
modals use one explicit owner and deterministic arbitration.

## Diagnostics

The runtime records bounded structured diagnostics for:

- flow, HUD, local-player, overlay, pause, and transition identities;
- accepted source revisions;
- visible overlay set and layout profile;
- stale or rejected observations;
- asset lease ownership and loading state;
- focus and semantic action ownership;
- pause rejection reasons;
- transition duration and cancellation; and
- settings preview and commit results.

Diagnostics contain canonical identities and typed reasons, never raw local
asset paths or machine-specific locations.

## Failure behavior

- Missing required HUD definitions block activation with a typed content error.
- Missing optional art uses a declared accessible fallback.
- Queue overflow follows explicit replacement policy and emits a diagnostic.
- Invalid normalized values are rejected before viewmodel publication.
- Conflicting blocking transitions fail without stealing the active lease.
- Failed pause activation restores gameplay, HUD visibility, focus, audio, and
  timer policy.
- Failed settings preview restores the last accepted configuration.
- Controller recovery cannot dismiss against a stale assignment revision.
- Feature removal restores a valid base overlay and pause catalog.

## Validation

Validation proves:

- every registered overlay and pause entry has one canonical identity;
- required widgets, actions, styles, bundles, and viewmodels resolve;
- source fields and viewmodel fields have compatible types and ranges;
- all blocking transitions have timeout and cancellation behavior;
- every local-player HUD has independent focus and state;
- map markers resolve to registered presentation profiles;
- radar fixed-range, auto-visibility, off-range, pursuit, and independent route-
  guidance setting policies are complete;
- pause commands resolve to typed application commands;
- settings fields resolve to the device-configuration schema;
- responsive and accessibility profiles are complete; and
- feature overlays are namespaced and removable.

## Tests

Automated tests cover:

- empty, minimal, normal, and maximum overlay sets;
- simultaneous mission overlays with deterministic compact layout;
- timer boundaries, warning cadence, and condition-timer replacement;
- off-route race position and route recovery;
- lap, collectible, currency, and mission-result exactly-once observations;
- normalized meter boundaries and invalid values;
- message queue overflow and replacement policy;
- fixed-range radar, free-roam auto visibility, off-range threat clamping,
  collectible hiding, pursuit pulse, radar off, and independent route guidance;
- duplicate, conflicting, cancelled, and stale fade requests;
- iris readiness barriers and reduced-motion fallbacks;
- letterbox accept, cancel, skip, and suppressed-action policy;
- one, two, and maximum supported local-player HUDs;
- pause acceptance and every declared rejection reason;
- restart, abort, save, replay, quit, and settings command handoff;
- controller disconnect, reassignment, and stale reconnect;
- feature install, activation, removal, and stale callback rejection; and
- deterministic diagnostics and repeated activation.

## Invariants

- Each local player owns exactly one accepted HUD revision.
- Widgets and viewmodels never own gameplay state.
- Common UI action identities are independent of physical keys.
- A stale observation never mutates accepted presentation.
- Only one blocking transition or pause transaction owns shared input.
- HUD overlays never complete mission, currency, save, or settings operations.
- Pause restores the exact prior gameplay and HUD policy on resume.
- Required information remains accessible on every supported platform and
  graphics preset.
