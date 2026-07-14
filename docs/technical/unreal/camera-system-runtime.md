# Camera system runtime

- Status: Active
- Last reviewed: 2026-07-14

## Governing decisions

- [HUD, radar, camera, and navigation parity](../../adr/unreal/ui/hud-radar-camera-and-navigation.md)
- [Runtime parity boundary](../../adr/unreal/runtime/remake-parity-boundary.md)
- [State-driven missions, interactions, interiors, and notoriety](../../adr/unreal/runtime/state-driven-missions-interactions-and-notoriety.md)
- [Runtime parity test boundary](../../adr/unreal/runtime/runtime-parity-test-boundary.md)

## Purpose

This specification defines the native player-camera system, mode arbitration,
vehicle and character tracking, animated shots, conversations, first-person and
bumper views, camera collision, field of view, input, shake, cuts, blending,
letterbox presentation, skipping, and recovery. It replaces direct mode switching,
mode-owned global input, and restoration of raw prior-camera pointers with typed
camera requests and deterministic policy.

## Ownership

| Authority | Responsibility |
| :--- | :--- |
| Camera catalog | Stable mode, preset, transition, shot, collision, and input identities. |
| Camera subsystem | Request arbitration, active stack, transitions, cancellation, and recovery. |
| Player camera manager | Final view calculation and native camera-modifier execution. |
| Gameplay services | Mission, conversation, vehicle, damage, and interaction observations. |
| UI presentation | Letterbox, skip prompt, subtitle-safe region, and accessibility projection. |
| Input service | Camera action values and active mapping context. |

A camera mode never owns mission progression, conversation state, vehicle state,
or input-device identity. It consumes immutable observations and returns a view.

## Runtime topology

The runtime module owns these C++ types:

| Type | Responsibility |
| :--- | :--- |
| `USharCameraModeDefinition` | Immutable mode, target, collision, FOV, input, and transition policy. |
| `USharCameraPresetDefinition` | Data for rods, offsets, lag, limits, framing, and platform variants. |
| `USharConversationShotDefinition` | Typed two-character or authored conversation framing. |
| `USharAnimatedCameraDefinition` | Authored camera and sequence bindings with skip and completion policy. |
| `USharCameraSubsystem` | Request arbitration, handles, active stack, restoration, and typed results. |
| `ASharPlayerCameraManager` | Native final view target and modifier integration. |
| `USharCameraCollisionComponent` | Bounded obstruction queries and safe camera placement. |
| `FSharCameraRequest` | Priority, mode, targets, preset, transition, reason, and lifetime. |
| `FSharCameraHandle` | Move-only cancellation and ownership handle. |
| `FSharCameraObservation` | Immutable target transform, velocity, state, input, and world snapshot. |

Gameplay systems request camera interest. Only the camera subsystem selects and
activates a mode.

## Definition contract

Every camera-mode definition contains:

| Field | Contract |
| :--- | :--- |
| `CameraModeId` | Globally unique canonical identity. |
| `ModeKind` | Follow, chase, bumper, burnout, first person, conversation, animated, debug, or registered extension. |
| `RequiredTargetKinds` | Character, vehicle, camera actor, sequence, or multiple targets. |
| `PresetId` | Validated offset, lag, FOV, input, collision, and framing data. |
| `PriorityClass` | Default, gameplay override, conversation, cinematic, safety, or debug. |
| `TransitionPolicyId` | Cut, blend, match, defer, or authored transition. |
| `CollisionPolicyId` | Query shape, channels, margin, recovery, and target exclusion. |
| `InputPolicyId` | Allowed look, zoom, reverse, quick-turn, skip, and debug actions. |
| `CancellationPolicy` | Immediate, blend out, safe point, or uninterruptible authored section. |
| `VerificationPolicy` | Required active mode, target, framing, and completion observations. |
| `DefinitionRevision` | Immutable revision used to reject stale requests. |

Presets use explicit units and validated bounds. Display names cannot select
runtime behavior.

## Request arbitration

A request contains requester identity, reason, priority class, mode identity,
target identities, preset identity, transition policy, lifetime, and cancellation
policy.

Requests are ordered by:

1. safety and invalid-view recovery;
1. required cinematic or mission presentation;
1. conversation or interaction ownership;
1. explicit player-selected camera mode;
1. contextual gameplay interest;
1. default follow mode; and
1. debug mode when development policy permits it.

Equal-priority requests use stable requester and request identities as tie-breakers.
Listener order and frame arrival order cannot decide the active camera.

The active stack stores request identities and typed restoration policy, not raw
previous-camera pointers. Releasing a request reevaluates the remaining stack.

## Camera result

Each request reaches one result:

| Result | Meaning |
| :--- | :--- |
| `active` | The requested mode owns the final view. |
| `queued` | A higher-priority request is active and deferral is permitted. |
| `superseded` | Another request replaced it according to policy. |
| `rejected` | Preconditions failed before activation. |
| `cancelled` | The owner released it and cleanup completed. |
| `completed` | An authored mode reached its verified terminal condition. |
| `failed` | Activation began but no valid view or cleanup result was reached. |

Every non-active result contains a typed reason.

## Shared view calculation

Each mode produces a desired location, rotation, target, FOV, post-process preset,
and optional modifier requests. The player camera manager applies them in one
native update path.

The shared path verifies:

- every target is valid for the current world;
- transforms are finite;
- FOV and lag values are within preset bounds;
- collision correction does not penetrate blocking geometry;
- transition state is valid; and
- the final view preserves the required target or falls back safely.

Presentation interpolation cannot alter gameplay state.

## Follow camera

Follow mode tracks a character or vehicle using an authored rod, target offset,
position lag, target lag, FOV range, speed mapping, and collision policy.

It supports:

- forward and reverse framing;
- player look input;
- target-relative or world-relative offsets;
- unstable or damage presentation;
- bounded quick turn;
- camera cuts after invalid displacement or reset;
- optional shake requests; and
- physics-aware target read-back.

Reverse selection uses typed vehicle direction, velocity, input, and preset
policy. It cannot flip repeatedly from numerical noise; hysteresis and minimum
stable duration are required.

A quick turn is a bounded transition state with explicit entry, interpolation,
completion, and cancellation. It cannot leave a residual yaw offset after the
request ends.

## Chase camera

Chase mode is a vehicle-follow specialization with a validated rod, position lag,
target lag, minimum and maximum FOV, maximum-speed reference, and FOV lag.

Speed-to-FOV mapping is monotonic and clamped. A reset, teleport, invalid target,
or extreme displacement triggers a declared cut or recovery blend rather than a
single-frame sweep through the world.

## Bumper camera

Bumper mode uses separate forward and reverse position and target offsets plus a
validated FOV. The target vehicle supplies one stable transform snapshot.

Collision correction may offset the camera away from body geometry but cannot
move it to an arbitrary external view. Reverse selection follows the same stable
policy as follow mode.

## First-person camera

First-person mode binds to a character or vehicle socket and consumes native look
input. It declares:

- socket or eye identity;
- yaw and pitch limits;
- smoothing and sensitivity;
- collision offset and near-clip policy;
- body, vehicle, and attachment visibility policy;
- entry and exit orientation restoration; and
- accessibility overrides.

Entry captures a typed orientation snapshot. Exit restores according to policy
without overwriting legitimate target rotation that occurred while active.

## Burnout camera

Burnout mode is a temporary vehicle presentation request. Its preset declares
target, orbit or offset behavior, duration, FOV, input suppression, and terminal
condition.

It never becomes the authority for burnout detection or vehicle control. The
owning vehicle service supplies the active observation and releases the request
when the effect ends.

## Conversation camera

Conversation mode consumes two stable character identities and a shot definition.
A shot declares:

- speaker and listener role;
- camera position and target offsets;
- FOV;
- side, distance, and height policy;
- child or adult framing variant;
- obstruction and safe-region policy;
- character-position locking permission; and
- transition and cut behavior.

Shot selection is deterministic from conversation identity, line or beat ordinal,
speaker role, and shot policy. Free-form character names or array indices cannot
select a shot.

Position locking, when permitted, uses the typed action-sequence runtime and
resource leases. The camera cannot directly move or freeze a character.

Conversation shutdown releases every character, input, letterbox, and camera
handle even when dialogue is cancelled or a participant unloads.

## Animated camera

Animated mode binds a validated camera actor or camera animation to one authored
sequence identity. It supports mission-start and general cinematic requests.

The definition declares:

- camera and sequence identities;
- expected duration and completion notify;
- initial and terminal view target;
- next-camera or restoration policy;
- transition flags;
- letterbox policy;
- skip permission and safe skip points;
- input suppression; and
- cancellation cleanup.

Transport completion or sequence start is not cinematic completion. Completion
requires the declared notify, sequence state, or final camera observation.

A pending switch is represented by a typed request transition. It cannot store a
raw future mode and activate it after the owner has been destroyed.

## Skipping

Skip input is accepted only when the active definition permits it and the current
sequence is at a safe skip point. A skip transaction:

1. validates the request and active owner;
1. advances or terminates the sequence through its native control port;
1. applies required mission or presentation completion policy;
1. verifies the terminal camera and letterbox state; and
1. publishes one skip result.

Repeated input cannot commit completion twice.

## Letterbox and UI

Letterbox presentation is a UI request owned by the active camera or cinematic
handle. It declares transition duration, safe-area behavior, subtitle policy,
and suppression rules.

The camera subsystem does not draw bars directly. Releasing or failing the camera
request releases the letterbox handle. A definition may suppress one transition
only through explicit typed policy.

## Camera collision

Camera collision uses bounded sweeps from the target or authored pivot to the
desired camera position. The policy declares shape, channels, ignored target
components, margin, minimum distance, recovery speed, and obstruction behavior.

The result may shorten the rod, offset along the hit normal, choose an authored
alternate shoulder, or cut to a safe fallback. It cannot pass through blocking
geometry because a prior frame was unobstructed.

Collision queries use simulation-time target transforms. Rendering interpolation
may smooth the accepted result but cannot replace collision authority.

## FOV

FOV values are finite and clamped to platform and accessibility policy. Dynamic
FOV may use speed, state, boost, or authored sequence progress. Every mapping is
versioned and deterministic.

A mode transition blends FOV according to its transition policy. Releasing a mode
restores the selected lower-priority preset, not a cached numeric FOV from an
unrelated world revision.

## Input

Enhanced Input owns camera actions such as look, zoom, reverse view, quick turn,
mode selection, and skip. The camera subsystem receives typed action values from
the active local player.

A camera mode cannot register hidden global controls. Input mapping contexts are
acquired and released through handles, and cancellation restores the previous
mapping stack.

## Shake and modifiers

Shake, damage instability, speed effects, and other modifiers are independent
requests. The player camera manager applies them after the base mode view.

Each modifier declares priority, amplitude limits, duration, blend, accessibility
scaling, and cancellation. Disabling shake cannot change base camera position,
mode selection, or gameplay state.

## Debug camera

Debug camera is development-only. It uses an explicit debug authorization and
separate input mapping context. It cannot satisfy shipping gameplay, mission,
cinematic, or accessibility acceptance.

Entering or leaving debug camera preserves the underlying active request stack.
Debug state is excluded from save data and deterministic gameplay replay.

## Streaming and lifecycle

Target destruction, world teardown, streaming, possession change, or local-player
removal invalidates affected requests. The subsystem then:

1. cancels mode-local input, animation, UI, and modifier handles;
1. removes invalid targets;
1. reevaluates the request stack;
1. selects a valid lower-priority mode or safety fallback; and
1. publishes the typed terminal result.

A pooled or recycled actor cannot inherit a camera request from its previous
identity.

## Failure behavior

The camera subsystem fails closed when:

- the mode or preset revision is missing or stale;
- required targets do not resolve;
- a camera actor, sequence, socket, or shot identity is missing;
- transforms, FOV, lag, or collision settings are invalid;
- no collision-safe view can be found;
- input or UI handles cannot be restored;
- an animated or conversation camera cannot prove completion; or
- native read-back contradicts the selected mode.

Failure selects a validated safety camera when possible. It never leaves the
player with an invalid view, permanent input suppression, or stranded letterbox.

## Verification

Automated verification proves:

- request arbitration and tie-breaking are deterministic;
- releasing a request restores the correct remaining mode;
- follow, chase, bumper, first-person, burnout, conversation, and animated modes
  reach their declared views;
- reverse and quick-turn transitions are stable and cancellation-safe;
- camera collision never penetrates declared blocking geometry;
- FOV and lag remain inside preset bounds;
- conversation shots select the correct speaker and framing variant;
- animated completion and skip are exactly once;
- letterbox, input, shake, and modifiers always release on terminal paths;
- debug camera cannot influence shipping acceptance; and
- fixed-step replay produces equivalent mode, target, transition, and view-policy
  results.
