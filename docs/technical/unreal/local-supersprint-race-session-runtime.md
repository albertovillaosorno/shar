# Local supersprint race session runtime

- Status: Active
- Last reviewed: 2026-07-16

## Governing decisions and specifications

<!-- markdownlint-disable-next-line MD013 -->
- [Hexagonal Unreal runtime](../../adr/unreal/architecture/hexagonal-runtime-and-no-technical-debt.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Runtime parity boundary](../../adr/unreal/runtime/remake-parity-boundary.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Runtime parity test boundary](../../adr/unreal/runtime/runtime-parity-test-boundary.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Graphics quality presets and platform support](../../adr/unreal/runtime/graphics-quality-presets-and-platform-support.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Local split-screen minigame session UI runtime](local-split-screen-minigame-session-ui-runtime.md)
- [Race route and opponent runtime](race-route-and-opponent-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Road-network geometry and traffic runtime](road-network-geometry-and-traffic-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Vehicle AI and route runtime](vehicle-ai-and-route-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Application lifecycle and mode runtime](application-lifecycle-and-mode-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Typed event and observation routing runtime](typed-event-and-observation-routing-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Native asset load request and streaming runtime](native-asset-load-request-and-streaming-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Music state and transition runtime](music-state-and-transition-runtime.md)

## Purpose

This specification defines the gameplay runtime for the local supersprint mode:
session authority, participant and controller ownership, character and vehicle
selection results, track and direction projection, loading, starting grid,
countdown, race states, checkpoints, laps, position calculation, artificial-
intelligence participants, turbo policy, camera and HUD observations,
disconnect,
pause, did-not-finish handling, winner results, high scores, cleanup, and
replay.

It complements the frontend and lobby behavior in
<!-- markdownlint-disable-next-line MD013 -->
[Local split-screen minigame session UI runtime](local-split-screen-minigame-session-ui-runtime.md)
and the generic race semantics in
[Race route and opponent runtime](race-route-and-opponent-runtime.md).

It replaces one process-global manager that combines input mapping, loading,
world setup, vehicle creation, artificial intelligence, cameras, traps, HUD
text,
race timing, result formatting, controller enumeration, and cleanup in one
mutable object.

## Product boundary

Supersprint is a local competitive race mode with up to four participant slots.
It is not a campaign and does not own portable campaign progression. A networked
or server-hosted version may be added by a mod through the validated multiplayer
adapter, but the base contract remains deterministic local multiplayer.

Participant count, lap range, turbo allowance, did-not-finish timing, eligible
characters, eligible vehicles, tracks, direction, and scoring are data-driven.
Source constants and static arrays remain provenance only.

## Native Unreal composition

The implementation uses native Unreal facilities where applicable:

- `UGameInstanceSubsystem` or world subsystem for session coordination;
- `ULocalPlayer`, Player Controller, Pawn, and Enhanced Input ownership;
- Game Mode and Game State for accepted race state;
- Player State for participant race data;
- native local-player and split-screen viewport support;
- Asset Manager bundles and retained handles;
- Level or World Partition streaming;
- Chaos vehicle or accepted native vehicle movement components;
- spline, road, checkpoint, and route assets;
- AI Controller, StateTree, navigation, or route-following components;
- Gameplay Cameras or project camera adapters;
- Common UI and per-player HUD presentation;
- Quartz or the music runtime for synchronized countdown and music cues; and
- SaveGame or device configuration for local high-score records.

No custom global frame loop, raw controller table, direct renderer text object,
or fixed vehicle pointer array becomes authority.

## Ownership

<!-- markdownlint-disable MD013 -->

| Authority | Responsibility |
| :--- | :--- |
| Supersprint session service | Owns session identity, participant slots, race state, transition transactions, results, and cleanup. |
| Lobby and UI runtime | Owns joining, selection presentation, readiness, options, result screens, and user commands. |
| Race route runtime | Owns route, checkpoint, lap, progress, position, finish, and reset semantics. |
| Local-player and input services | Own controller assignment, mapping contexts, reconnect, and per-player input. |
| Vehicle runtime | Owns vehicle movement, damage policy, turbo execution, and physical state. |
| AI runtime | Owns route-following decisions for accepted artificial-intelligence participants. |
| Camera runtime | Owns native cameras, split-screen views, transitions, and accepted focus targets. |
| Audio runtime | Owns countdown, music, effects, and ducking presentation. |
| Persistence service | Owns validated local high-score records and migration. |
| Developer diagnostics | Observe immutable session, participant, route, vehicle, controller, camera, and result state. |

<!-- markdownlint-enable MD013 -->

HUD text, camera completion, vehicle animation, music, or sound effects cannot
commit race state or results.

## Runtime identities

Stable identities include:

- `FSharSupersprintDefinitionId`;
- `FSharSupersprintDefinitionRevision`;
- `FSharSupersprintSessionId`;
- `FSharSupersprintSessionRevision`;
- `FSharSupersprintTrackId`;
- `FSharSupersprintTrackRevision`;
- `FSharSupersprintParticipantId`;
- `FSharSupersprintSlotId`;
- `FSharLocalPlayerId`;
- `FSharControllerAssignmentRevision`;
- `FSharCharacterDefinitionId`;
- `FSharVehicleDefinitionId`;
- `FSharRaceRouteId`;
- `FSharRaceRevision`;
- `FSharCheckpointId`;
- `FSharLapRevision`;
- `FSharRaceResultId`;
- `FSharHighScoreRevision`;
- `FSharCameraLeaseId`;
- `FSharInputLeaseId`;
- `FSharWorldRevision`; and
- `FSharFeatureRevision`.

Source player indexes, vehicle-array positions, character-array positions,
controller ordinals, waypoint-array indexes, locator names, pointer equality,
and rendered color positions are not identity.

## Supersprint definition

`USharSupersprintDefinition` declares:

- canonical identity and revision;
- supported participant range;
- character eligibility policy;
- vehicle eligibility policy;
- default and allowed lap counts;
- direction options;
- track definitions;
- countdown definition;
- turbo policy;
- artificial-intelligence fill policy;
- starting-grid policy;
- checkpoint and finish policy;
- did-not-finish policy;
- result and scoring policy;
- high-score categories;
- camera, HUD, music, audio, and accessibility policy;
- loading bundles;
- platform and quality constraints;
- teardown and replay policy; and
- feature-overlay namespace.

Definitions use canonical catalog identities. Display names and localized text
are
presentation metadata, not selection identity.

## Track definition

Each track declares:

- canonical track and world identities;
- required streaming cells or level assets;
- route and checkpoint definitions;
- forward and optional reverse direction;
- starting-grid transforms;
- artificial-intelligence waypoint or route policy;
- camera definitions;
- trap and hazard definitions;
- minimap or position-icon presentation;
- music and ambience context;
- supported participant count;
- collision and reset policy;
- high-score scope; and
- validation digest.

A checkpoint locator must resolve to the accepted race route and legal world
geometry. Runtime does not search arbitrary road data by raw locator name and
silently accept an off-route point.

## Participant record

`FSharSupersprintParticipantState` contains:

- participant and slot identities;
- local-player identity or artificial-intelligence owner;
- controller-assignment revision;
- selected character and vehicle identities;
- readiness state;
- race eligibility;
- accepted Pawn and vehicle instance identities;
- current checkpoint and lap revisions;
- normalized route progress;
- race and lap timers;
- best lap;
- finish position;
- did-not-finish status;
- turbo inventory and usage;
- disconnect and reconnect state;
- result identity; and
- diagnostic flags.

Raw vehicle, character, AI, controller, HUD, or camera pointers are runtime
implementation details and never serialized or used as durable identity.

## Session states

The closed session states are:

- `inactive`;
- `lobby`;
- `loading`;
- `preparing_world`;
- `intro`;
- `countdown`;
- `racing`;
- `finish_window`;
- `results`;
- `paused`;
- `replay_preparing`;
- `tearing_down`;
- `completed`; and
- `failed`.

A source-era sequence such as idle, countdown, racing, did-not-finish timeout,
winner circle, and paused is converted into these typed states with explicit
entry, exit, rollback, and failure behavior.

## Session transition transaction

Every transition contains:

- session and expected revision;
- source and target state;
- cause identity;
- requesting owner;
- required readiness barrier;
- participant and controller snapshot;
- world, feature, route, camera, input, and audio revisions;
- cancellation token; and
- terminal result.

Transition order is deterministic. A late loading callback, camera blend,
countdown sound, controller event, or vehicle callback cannot activate a
superseded session state.

## Joining and selection handoff

The lobby runtime owns join, character selection, vehicle selection, lap count,
direction, and readiness presentation. Race gameplay accepts one immutable start
request containing the committed selections and option revision.

Gameplay does not re-read mutable UI widgets or static selection arrays. If an
asset becomes unavailable between readiness and load, the start transaction
fails or applies a declared deterministic fallback before world activation.

## Controller ownership

Each human participant has one local-player and controller-assignment revision.
Enhanced Input mapping contexts are installed per local player and scoped to the
session state.

Controller disconnect freezes or applies the configured AI takeover, pause,
reconnect grace, or forfeiture policy. Reconnect may restore control only when
the
slot, local player, device, vehicle, session, and assignment revisions still
match.

Input cannot be routed by first connected device, array position, platform-
specific button name, or process-global mappable state.

## Loading transaction

Loading requires:

- session and track definitions;
- route and checkpoint assets;
- selected character and vehicle bundles;
- artificial-intelligence bundles;
- world or streaming cells;
- camera and HUD definitions;
- audio and music bundles;
- traps and hazards;
- local-player mapping contexts; and
- fallback assets.

The transaction retains handles by semantic scope and publishes a typed
readiness
result. Callback order cannot choose the active track or participant mapping.

## World preparation

World preparation:

1. verifies the accepted world revision;
1. activates the race route and checkpoints;
1. creates or obtains participant vehicles;
1. assigns human or artificial-intelligence controllers;
1. applies starting-grid transforms;
1. places accepted characters into vehicles for presentation;
1. initializes cameras and per-player HUD;
1. activates traps and hazards;
1. resets timers, laps, checkpoints, turbo, and results; and
1. publishes one ready barrier.

Any required failure compensates all created vehicles, controllers, cameras,
input leases, route state, and assets before returning to the lobby.

## Starting grid

Starting positions are stable slot definitions. The grid validates clearance,
road support, orientation, route direction, camera visibility, and participant
count.

Vehicle placement uses native teleport or spawn transactions with physics reset,
not direct transform writes to stale objects. Artificial-intelligence and human
participants receive equivalent legal starting state.

## Intro and camera

An optional intro sequence may establish the track and participants before the
countdown. It owns a bounded camera and input lease.

Camera sequence completion is presentation evidence. The session advances only
when the intro barrier has either completed or been skipped under accepted
policy
and all required participants remain ready.

## Countdown

The countdown definition contains ordered semantic stages, durations, localized
presentation, audio cues, camera behavior, input lock, vehicle control policy,
and the exact race-start barrier.

Countdown timing uses monotonic or sample-aligned time as declared. Frame count,
rendered text, or sound completion cannot start the race. The final stage
commits
one race-start revision and releases control simultaneously for eligible
participants.

## Race clock

Race and lap clocks use one accepted monotonic session time source. Pause,
suspension, focus loss, loading interruption, and results state declare whether
clocks freeze.

Wall-clock time, render frames, and per-player update order cannot alter race
results. Timers use bounded integer or fixed precision representation suitable
for deterministic comparison and persistence.

## Checkpoints and laps

Checkpoint and lap behavior follows
[Race route and opponent runtime](race-route-and-opponent-runtime.md).
Supersprint adds per-participant local-player ownership and result projection.

A checkpoint observation contains participant, vehicle, route, checkpoint, lap,
world, and race revisions. Duplicate, out-of-order, stale, wrong-direction, or
wrong-route observations are rejected.

A lap completes only after the declared ordered checkpoint sequence. Reverse
tracks use a validated reversed route definition rather than decrementing a raw
checkpoint index without route evidence.

## Position calculation

Position order is deterministic from:

1. completed race status;
1. completed lap count;
1. accepted checkpoint progress;
1. normalized distance to the next legal checkpoint;
1. finish time where applicable; and
1. stable participant identity as the final tie-breaker.

Euclidean distance, update order, pointer order, or HUD icon position cannot
become race authority.

## Human and artificial-intelligence participants

Artificial-intelligence slots use the same participant, checkpoint, lap,
position, turbo, finish, and result schemas as human slots. AI control follows
[Vehicle AI and route runtime](vehicle-ai-and-route-runtime.md).

Difficulty, route speed, turbo usage, recovery, and hazard response are data-
driven. AI cannot bypass checkpoints, spawn with extra accepted laps, or commit
results through presentation state.

## Turbo policy

Turbo inventory is a typed race resource. A definition declares initial count,
maximum count, cooldown, eligibility, AI policy, vehicle capability, and effect.

An accepted turbo request consumes exactly one unit and applies one vehicle
input
or ability transaction. Audio, VFX, camera shake, or speed observation cannot
consume or refund turbo independently.

## Traps and hazards

Track traps and hazards are authored world entities with stable identities,
activation policy, collision and damage contracts, reset behavior, and feature
ownership.

The session activates definitions; it does not retain raw animated-collision
objects or mutate arbitrary level objects. Trap presentation cannot alter
checkpoint or lap state.

## Pause

Pause is a session transition with local-player and application policy. It
freezes
race clocks and vehicle input according to definition, applies audio and camera
mixes, and preserves participant and controller ownership.

In local competitive play, any participant may request pause only when the
configured policy allows it. Pause UI completion does not mutate race results.

## Disconnect and reconnect

A disconnect observation contains local player, device, slot, session, and
assignment revisions. Policy may:

- pause all local play;
- allow a grace window;
- assign an artificial-intelligence controller;
- mark the participant did not finish; or
- end the session when no valid humans remain.

Reconnect never assigns a device to a different slot merely because an array
entry is free.

## Finish and did-not-finish window

The first valid finish may open a bounded finish window for remaining racers.
The definition declares duration, freeze behavior for finished vehicles,
artificial-intelligence behavior, warnings, and terminal did-not-finish rules.

A participant becomes did not finish only through the accepted timeout or
forfeiture transaction. Rendered countdown text, a missing vehicle pointer, or a
controller callback cannot set the result.

## Winner and results

A race result contains:

- result and race identities;
- ordered participant results;
- character and vehicle identities;
- finish or did-not-finish status;
- race time;
- best lap;
- accepted position;
- tie-break evidence;
- cheat or assist classifications where required;
- high-score eligibility; and
- deterministic digest.

Winner-circle cameras, text, icons, audio, and animations consume this immutable
result. Presentation cannot reorder participants or change eligibility.

## High scores

High-score categories use stable track, direction, lap, participant class, and
ruleset identities. Names and display formatting are presentation metadata.

A score write validates the complete result, ruleset, cheat policy, existing
revision, capacity, ordering, and migration. Fixed three-character buffers,
static ten-entry arrays, source vehicle indexes, and pointer-owned tables are
not
portable storage formats.

## Replay and restart

Replay creates a new session and race revision while retaining only allowed
lobby
selections and settings. It reloads or reuses assets through explicit ownership,
reconstructs vehicles, resets route state, and installs new input and camera
leases.

No checkpoint, lap, timer, turbo, did-not-finish, callback, AI, camera, or
result
state leaks from the previous race.

## Cleanup

Cleanup freezes new requests and then:

- cancels loading and callbacks;
- disables participant input;
- releases camera and HUD leases;
- stops or releases music and audio scopes;
- unregisters route, checkpoints, traps, and icons;
- destroys or returns owned vehicles and AI controllers;
- removes temporary characters and presentation attachments;
- releases retained asset handles;
- restores application and controller state; and
- invalidates the session revision.

Cleanup is idempotent and safe after partial preparation.

## Networking and mod boundary

The base implementation is local-only. A network mod must add a separate server-
authoritative adapter for participant admission, input, race clocks, checkpoint
observations, prediction, correction, results, and reconnect.

Portable mods may add tracks, characters, vehicles, rulesets, UI, camera, music,
and AI definitions through validated namespaces. They cannot replace
local-player
ownership, native input, route authority, or result validation globally.

## Accessibility and quality

Accessibility may provide remapping, hold alternatives, captions, color-safe
participant markers, readable countdowns, camera comfort, and additional result
cues.

Quality may reduce optional crowd, particles, reflections, camera effects,
distant audio, or HUD flourish. It cannot change physics, checkpoint
reachability,
route direction, clocks, AI rules, participant count, local-player isolation,
results, or required feedback.

## Diagnostics

Read-only diagnostics expose:

- session state and revision;
- track, direction, lap, and ruleset;
- participant slots and controller assignments;
- selected character and vehicle identities;
- loading and world readiness;
- vehicle and AI ownership;
- checkpoint, lap, progress, and position evidence;
- clocks, turbo, finish window, and results;
- camera, HUD, input, audio, and asset leases;
- disconnect and reconnect state; and
- stale callback and cleanup findings.

Diagnostics cannot join a player, assign a controller, change a selection,
advance
a checkpoint, consume turbo, finish a race, or write a high score in shipping
runtime.

## Failure behavior

Closed failures include:

- `definition_missing`;
- `track_invalid`;
- `participant_invalid`;
- `controller_conflict`;
- `selection_invalid`;
- `asset_not_ready`;
- `world_not_ready`;
- `route_invalid`;
- `grid_invalid`;
- `vehicle_spawn_failed`;
- `ai_setup_failed`;
- `camera_failed`;
- `input_failed`;
- `checkpoint_stale`;
- `timer_stale`;
- `result_conflict`;
- `high_score_conflict`;
- `cancelled`;
- `superseded`; and
- `internal_failure`.

A required failure returns safely to the lobby or application-defined recovery
state after compensation. It never begins a partial race with mismatched
players,
vehicles, checkpoints, cameras, or inputs.

## Validation

Validation proves:

- participant, slot, track, route, checkpoint, grid, character, vehicle, camera,
  input, result, and score identities are stable;
- supported participant and lap ranges are explicit;
- every direction has a legal route and checkpoint order;
- grid positions are valid and non-overlapping;
- all eligible selections resolve required bundles;
- controller mappings are per local player;
- artificial-intelligence participants use legal route and result semantics;
- countdown and race clocks are deterministic;
- did-not-finish and tie-break policy is total and deterministic;
- cleanup covers every owned resource;
- replay creates new revisions; and
- source arrays, raw names, pointer identity, callback order, and platform
  button
  names are not runtime authority.

## Tests

Required tests cover:

- one through four human participants;
- mixed human and artificial-intelligence participants;
- every allowed lap count and direction;
- valid and invalid character and vehicle selections;
- asynchronous loading in every callback order;
- partial preparation rollback;
- countdown, pause, resume, focus loss, and suspension;
- checkpoint order, duplicate rejection, reverse tracks, laps, and positions;
- turbo consumption and AI policy;
- first finish, finish window, all-finish, and did-not-finish results;
- controller disconnect and reconnect in every state;
- camera and HUD isolation across local players;
- high-score eligibility, ordering, capacity, migration, and conflict;
- replay without leaked state;
- feature removal during loading and racing;
- deterministic result replay; and
- headless semantic race execution without cameras, HUD, or audio.

## Invariants

- Participant identity is never an array position.
- Controller assignment is always scoped to one local player and revision.
- Countdown presentation never starts the race by itself.
- Checkpoint and lap authority never comes from distance or HUD state alone.
- Artificial-intelligence participants obey the same race contract as humans.
- Camera, audio, VFX, and text cannot commit results.
- Replay never reuses accepted checkpoint, timer, turbo, or result revisions.
- Cleanup leaves no owned vehicle, AI, input, camera, HUD, route, audio, or
  asset
  lease.
- The base session remains local-only while exposing a validated adapter
  boundary
  for multiplayer mods.
