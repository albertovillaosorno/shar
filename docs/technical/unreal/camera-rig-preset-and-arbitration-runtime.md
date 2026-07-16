# Camera rig, preset, and arbitration runtime

- Status: Active
- Last reviewed: 2026-07-16

## Governing decisions and specifications

<!-- markdownlint-disable-next-line MD013 -->
- [HUD, radar, camera, and navigation parity](../../adr/unreal/ui/hud-radar-camera-and-navigation.md)
- [Runtime parity boundary](../../adr/unreal/runtime/remake-parity-boundary.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Runtime parity test boundary](../../adr/unreal/runtime/runtime-parity-test-boundary.md)
- [Camera system runtime](camera-system-runtime.md)
- [Presentation playback runtime](presentation-playback-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Spatial audio listener and positional-source runtime](spatial-audio-listener-and-positional-source-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Spatial visibility, bounds, and culling runtime](spatial-visibility-bounds-and-culling-runtime.md)

## Purpose

This specification defines the native camera rig catalog, target snapshots,
per-player arbitration, authored preset conversion, specialized camera modes,
shake modifiers, collision input, and development visualization. It complements
the semantic camera-system contract without creating a second authority for
camera selection or final view calculation.

The design replaces fixed camera arrays, enum-position lookup, global event
listeners, mutable source-data chunks, platform-specific hardcoded settings, and
raw previous-camera pointers with validated identities and typed requests.

## Ownership

<!-- markdownlint-disable MD013 -->

| Authority | Responsibility |
| :--- | :--- |
| Camera catalog | Stable mode, rig, preset, target, transition, modifier, and availability identities. |
| Per-player camera subsystem | Request arbitration, active stack, target binding, and lifecycle. |
| Player camera manager | Final view, native modifiers, projection, and engine integration. |
| Camera rig evaluator | Pure desired-view calculation for one validated mode and snapshot. |
| Collision adapter | Bounded scene queries and correction evidence. |
| Input adapter | Semantic camera actions for the owning local player. |
| Gameplay services | Typed mission, vehicle, character, conversation, race, and locator observations. |
| Development tools | Read-only frustum, rail, hull, collision, and candidate visualization. |

<!-- markdownlint-enable MD013 -->

A camera rig cannot write mission, progression, vehicle, character, locator,
input-device, or save state. It can only consume an immutable observation and
return a desired view plus optional presentation requests.

## Runtime topology

The runtime module owns these C++ types:

<!-- markdownlint-disable MD013 -->

| Type | Responsibility |
| :--- | :--- |
| `USharCameraRigDefinition` | Immutable execution kind, availability, target, preset, collision, input, and modifier policy. |
| `USharCameraPresetDefinition` | Versioned offsets, rods, lags, FOV, planes, limits, and transition settings. |
| `USharRailCameraDefinition` | Spline, interval, projection, radius, tracking, and reset policy. |
| `USharAuthoredCameraShotDefinition` | Static, surveillance, snapshot, relative-animation, or overview shot data. |
| `USharCameraModifierDefinition` | Shake and other bounded modifier parameters. |
| `USharCameraCatalogSubsystem` | Definition lookup, revision checks, alias migration, and asset validation. |
| `USharCameraSubsystem` | One per local player; owns request arbitration and active handles. |
| `ASharPlayerCameraManager` | Applies the final validated view and native modifiers. |
| `FSharCameraTargetSnapshot` | Immutable target transform, motion, state, bounds, and sockets. |
| `FSharDesiredCameraView` | Desired location, rotation, look target, FOV, aspect, planes, and modifier requests. |
| `FSharCameraEvaluationResult` | Closed success or failure result with correction evidence. |

<!-- markdownlint-enable MD013 -->

The camera catalog is shared immutable data. Active requests, input, collision,
and target snapshots are local-player state.

## Stable identities

Every rig and preset uses canonical identities. Registration order and enum
ordinal are not identity authority.

Required identities include:

- camera rig;
- mode kind;
- preset revision;
- owning local player;
- target set;
- input policy;
- transition policy;
- collision policy;
- modifier set;
- authored shot or rail when applicable; and
- availability class.

Aliases from imported evidence may resolve to one canonical identity. Duplicate
canonical identities, aliases with multiple targets, missing execution kinds, or
incompatible revisions fail catalog activation.

## Availability classes

Every rig declares one availability class:

<!-- markdownlint-disable MD013 -->

| Availability | Contract |
| :--- | :--- |
| `shipping_default` | Available through ordinary player or gameplay policy. |
| `shipping_contextual` | Available only from a typed mission, conversation, race, interaction, or world request. |
| `cheat_optional` | Player-visible only when an explicit cheat or accessibility policy enables it. |
| `development_only` | Editor, automation, or authorized development builds only. |
| `diagnostic_only` | Read-only verification and visualization; never a gameplay view. |
| `excluded` | Preserved as evidence but unavailable at runtime. |

<!-- markdownlint-enable MD013 -->

Orbit-debug, tracker-debug, free-debug, snapshot inspection, and frustum drawing
cannot appear in ordinary shipping mode rotation unless an accepted public
policy explicitly promotes them.

## Target snapshot contract

Camera targets implement a narrow native provider that produces one
`FSharCameraTargetSnapshot` per evaluation step. The snapshot contains:

- stable target identity and target kind;
- world transform and up vector;
- linear and angular velocity;
- collision bounds;
- grounded, airborne, unstable, quick-turn, reverse, and destroyed state;
- first-person socket or eye transform when supported;
- terrain or floor observation when required;
- optional named sockets and framing points; and
- snapshot revision and world frame identity.

The camera evaluator never calls arbitrary gameplay methods while calculating a
view. It receives the complete immutable snapshot before evaluation.

Missing optional fields are represented explicitly. A mode that requires a
first-person socket, vehicle reverse state, or terrain observation rejects a
snapshot that does not provide it.

## Per-player authority

Each local player owns one `USharCameraSubsystem` and one player camera manager.
The subsystem maintains:

- the validated request set;
- one active request identity;
- queued or superseded request state;
- target bindings;
- the current transition;
- active modifier handles;
- collision correction evidence;
- input mapping context; and
- the last verified safe view.

Split-screen or local multiplayer never shares mutable camera requests, input,
collision buffers, target arrays, or previous-view state. Shared catalog assets
remain immutable.

A process-wide camera singleton is forbidden. Manager convenience access cannot
become hidden ownership.

## Rig registration

Rig definitions are discovered from validated assets and indexed by canonical
identity. Registration fails when:

- the execution kind has no native evaluator;
- the preset is missing or incompatible;
- the target cardinality is invalid;
- the availability class conflicts with the build policy;
- a required input action is absent;
- collision channels or query shapes are unresolved;
- a referenced rail, shot, animation, sequence, or modifier is invalid; or
- the definition revision differs from the approved catalog.

Fixed maximum camera arrays and type arithmetic are not used. Runtime memory
budgets are enforced independently from identity lookup.

## Preset conversion

Imported follow, walker, tracker, bumper, chase, rail, and other camera data are
normalized into `USharCameraPresetDefinition` assets during Phase 6. Source data
chunks remain provenance and cannot be loaded as mutable runtime structs.

A preset records:

- source artifact and conversion revision;
- rotation, elevation, magnitude, rod, and target offsets;
- horizontal and vertical position lag;
- horizontal and vertical target lag;
- collision lag and safety margin;
- minimum, maximum, and default FOV;
- FOV lag and speed mapping;
- near and far clipping planes;
- aspect or aspect-policy identity;
- quick-turn and unstable-state delays;
- reverse hysteresis and stable duration;
- input sensitivity and limits; and
- target, collision, transition, and modifier policy identities.

Angles and FOV use declared units. Durations use seconds. Imported platform
variants must normalize into one semantic preset or remain explicit
presentation variants selected by target policy.

## Preset inheritance

Preset inheritance is resolved during generation, not by mutable runtime parent
pointers. The generated asset contains the complete effective value set and the
provenance chain used to derive it.

Near and far follow modes may share a base preset while overriding magnitude,
FOV, or lag. A missing override never falls back to an unrelated preset selected
by array position.

## Base rig lifecycle

Every camera rig follows this lifecycle:

1. resolve definition and preset revisions;
1. validate availability, player, targets, and context;
1. acquire the request and required modifier handles;
1. capture a target snapshot and prior safe view;
1. evaluate the desired view;
1. apply collision and projection validation;
1. transition or cut according to policy;
1. publish active-view evidence;
1. reevaluate until completion or cancellation; and
1. release input, modifier, target, and transition state.

Completion, failure, cancellation, target destruction, world teardown, and local
player removal all execute the same cleanup contract.

## Desired view evaluation

A rig evaluator is deterministic for equivalent definition, snapshot, input,
collision observations, and elapsed simulation time. It returns:

- desired position and rotation;
- desired look target or forward direction;
- up vector policy;
- FOV and aspect policy;
- near and far planes;
- post-process or accessibility preset identity;
- collision query request;
- modifier requests; and
- verification observations.

Non-finite transforms, invalid planes, inverted projection ranges, unresolved
look targets, or unsupported target kinds return a typed failure before the view
is applied.

After the camera and view commit, the subsystem publishes one immutable listener
candidate containing local-player, view, camera, world, position, orientation,
velocity, controlled-participant, and interior revisions to
<!-- markdownlint-disable-next-line MD013 -->
[Spatial audio listener and positional-source runtime](spatial-audio-listener-and-positional-source-runtime.md).

The camera runtime does not write directly to the platform audio device, clamp a
listener to an arbitrary participant, select the split-screen mix, or retain an
audio listener after view teardown.

## Input mapping

Camera input uses semantic Enhanced Input actions such as:

- look horizontal and vertical;
- zoom;
- look back;
- look left or right;
- camera-cycle forward or backward;
- first-person toggle;
- skip authored camera;
- debug elevation and distance; and
- authorized development free-camera movement.

Keyboard, mouse, gamepad, wheel, and touch adapters map to the same semantic
actions. A rig never reads a global controller index or changes input mapping
outside its move-only input-context handle.

Dead zones, sensitivity, inversion, acceleration, and device presentation are
settings. They cannot change mode eligibility or gameplay state.

## Shared collision input

Before final evaluation, the camera subsystem gathers bounded collision
observations for the current player and rig policy. The result contains blocking
hits, ground correction, target exclusion, query revision, and world frame.

Collision correction is applied by one shared adapter. Rigs may choose different
query shapes, radii, margins, lags, or fallback policies, but they cannot
maintain
private unbounded intersection arrays.

When no valid corrected position exists, the subsystem cuts or blends to the
last safe view or another eligible rig. It never accepts penetration or a
non-finite correction.

## FOV volumes and contextual projection

World-authored FOV regions publish typed contextual observations. A region
contains stable identity, priority, blend policy, target filters, FOV bounds,
and streaming revision.

The camera subsystem arbitrates FOV regions with mode presets and accessibility
settings. Enter and exit order cannot leave a stale override. Unloading a region
releases its request through the same cancellation path.

## Follow rigs

Near and far follow rigs share the base follow evaluator. Their complete preset
contains rotation, elevation, magnitude, target offset, camera lag, target lag,
collision lag, unstable delay, quick-turn delay, quick-turn modifier, aspect,
and FOV.

The evaluator tracks up to the definition's validated target cardinality. An
active target is selected by typed gameplay policy, not by mutable array index.
Target switching records the previous and next identities and either cuts or
blends according to the transition policy.

Unstable, stabilizing, quick-turn, look-back, sideways-look, collision, and
line-of-sight-corrected states are explicit transient states. Each state has an
entry condition, maximum duration, cancellation path, and verified terminal
condition.

## On-foot orbit rigs

The ordinary walker rig and pointer-oriented orbit rig consume character
snapshots and the same semantic look and zoom actions.

Walker presets include:

- minimum and maximum magnitude;
- elevation and rotation;
- rotation increment;
- target offset;
- position and target lag;
- jump target lag;
- landing transition duration;
- upward framing angle;
- collision lag; and
- minimum, maximum, and lagged FOV.

The walker rig supports normal, one-hit, and multi-hit collision correction
through the shared collision adapter. Jump and landing framing observe Character
Movement state and cannot create a second movement simulation.

The pointer-oriented rig supports bounded pitch, yaw, zoom, target-facing
compensation, collision correction, and physics read-back. It is not a separate
platform gameplay implementation; mouse, stick, and touch inputs select the same
semantic orbit policy.

## Cheat and development orbit rigs

Kull-style and tracker-style orbit rigs are classified as cheat or development
modes unless a public product policy promotes them. They orbit a target using
validated rotation, elevation, magnitude, FOV, and input limits.

Tracker-style distance-to-FOV mapping is monotonic, clamped, and uses a
validated
preset. Development controls cannot read an unrelated player's controller or
persist modified preset values.

## Rail rigs

A rail definition contains:

- ordered spline control points and stable rail identity;
- open or closed interval policy;
- distance or projection behavior;
- minimum and maximum target radius;
- maximum parametric step;
- starting parameter;
- optional reverse sensing;
- target and axis-play offsets;
- position and target lag;
- maximum FOV and FOV lag;
- target-speed modifier policy;
- reset and streaming policy; and
- development visualization flags.

Rail evaluation searches a bounded candidate neighborhood around the previous
parameter. Candidates are classified as exact, approximate, or fallback and are
ordered by solution quality, world distance, parameter distance, segment, and
parameter value.

Closed rails wrap only through validated interval policy. Open rails clamp.
Missing splines, invalid control points, non-finite derivatives, or no valid
candidate return a typed failure and safe fallback.

## Static and surveillance rigs

A static rig owns an authored world position and either tracks a target or looks
at an authored world point. Target-relative offsets are transformed from one
immutable target snapshot. Position, target lag, FOV, and transition behavior
come from the definition.

A surveillance rig is a constrained static rig with one fixed position and one
target. It does not infer camera placement from the target or own surveillance
gameplay state.

Streaming out the authored anchor cancels the request or activates its declared
fallback. It cannot leave a dangling world pointer.

## Snapshot inspection rig

Snapshot inspection is development-only. A definition references an explicit
ordered set of camera-shot identities. Selection uses semantic development input
and stable shot identity.

The runtime does not synchronously load an arbitrary package, enumerate a global
inventory, or keep a fixed array of discovered cameras. Missing shots are
reported individually and cannot shift remaining identities.

## Relative animated rig

A relative animated rig binds an authored camera animation to a stable target or
anchor transform. The animation produces a local camera transform that is
composed with one validated offset matrix and current target snapshot.

The presentation playback subsystem owns asset readiness, animation lifecycle,
skip, cancellation, owner correlation, and teardown. It submits one typed camera
request carrying the presentation and owner revisions. The camera subsystem
retains arbitration, view calculation, blend, preemption, and restoration
authority.

Pending mode switches, letterbox ownership, sequence completion, skip policy,
and restoration follow the ordinary authored-camera request contract. Animation
controllers and presentation callbacks cannot change gameplay state, select the
next camera, or restore a stale previous-camera pointer.

## Reverse rig

A dedicated reverse rig may be selected when a vehicle snapshot satisfies the
validated reverse policy. It defines rear position and target offsets,
collision,
FOV, shake, transition, and minimum stable duration.

The rig cannot switch from numerical velocity noise. If reverse state becomes
invalid during an uninterruptible transition, it reaches the next permitted
safe point and returns control to arbitration.

## Super-sprint overview rig

The super-sprint overview is an authored contextual rig. Each bonus race or
world variant references a camera-shot definition containing position, target,
up vector, FOV, aspect policy, and near and far planes.

Platform-specific hardcoded arrays and special-case clipping hacks are
forbidden.
Target packaging may choose validated presentation variants, but every variant
must preserve the same semantic framing and race visibility contract.

## Tracker and multi-target rigs

A tracker rig maintains target-centered orbit and may adapt FOV from validated
distance bounds. A true multi-target rig receives an ordered target set and
computes a bounded framing volume, target centroid, and required safe margin.

Target disappearance, split-screen player removal, or cardinality changes create
new target-set revisions. A rig cannot silently keep a stale target pointer.

## Wreckless locator rig

Wreckless presentation is a contextual or cheat rig driven by typed
camera-locator
observations. A locator observation contains locator identity, player identity,
world transform, activation reason, expiry, and streaming revision.

The rig chooses an eligible locator through deterministic priority and distance
policy, then frames the current target with bounded distance-to-FOV mapping.
When
no locator is eligible, it uses the declared target-relative fallback.

Raw global locator events and a mutable last-position sentinel are forbidden.
Unloading or invalidating a locator cancels its observation.

## Shake modifier

Camera shake is a native camera modifier with one typed definition. A periodic
shake definition contains:

- duration;
- frequency or speed;
- amplitude;
- direction;
- world-relative or camera-relative space;
- looping permission;
- blend-in and blend-out;
- maximum scale;
- optional amplitude growth; and
- accessibility attenuation policy.

Modifiers use simulation time and return a bounded offset or rotation. They do
not mutate the underlying rig position, target, or preset.

A modifier handle is released on completion, cancellation, mode supersession,
world teardown, or accessibility-policy change. Looping modifiers require an
explicit owner and cannot survive that owner's request.

## Frustum and rail visualization

Frustum, rail, hull, cylinder, collision, candidate, and target diagnostics are
read-only development views. They consume the same validated snapshots and
results used by runtime evaluation.

The camera subsystem owns accepted view and projection revisions. Renderer-owned
primitive bounds, per-view frustum rejection, occlusion, and converted
convex-volume diagnostics follow
<!-- markdownlint-disable-next-line MD013 -->
[Spatial visibility, bounds, and culling runtime](spatial-visibility-bounds-and-culling-runtime.md).
A camera diagnostic cannot publish a second draw list or mutate world
visibility.

Diagnostic geometry:

- is unavailable in shipping acceptance builds;
- cannot register as a gameplay camera;
- cannot modify a preset or target;
- cannot affect candidate ordering or collision results; and
- is excluded from save, replay, progression, and mod authority.

## Constants and defaults

Default FOV, clipping planes, aspect policy, transition rates, dead zones, and
other constants live in validated definitions or repository-owned native
constants with explicit units. Preprocessor platform branches cannot silently
select different gameplay framing.

A default is used only when the definition schema permits omission. Missing
required values fail asset validation rather than inheriting arbitrary engine or
historical global state.

## Streaming and teardown

World Partition and Runtime Data Layer changes may invalidate targets, rails,
shots, locators, FOV regions, or collision observations. Invalidation creates a
typed cancellation or fallback request.

Teardown order is:

1. stop accepting new requests for the local player;
1. cancel active and queued mode handles;
1. cancel camera modifiers and transitions;
1. release input contexts;
1. release targets and authored world references;
1. clear collision observations;
1. restore or discard the last safe view according to world state; and
1. destroy the player camera manager and subsystem state.

No static player count, global current camera, or shared mutable preset survives
world teardown.

## Determinism

Equivalent catalog revisions, target snapshots, semantic input, collision
observations, and simulation-time deltas produce equivalent desired views within
declared floating-point tolerance.

Determinism tests cover:

- request priority and tie-breaking;
- target-set ordering;
- preset conversion;
- rail candidate ordering;
- reverse hysteresis;
- quick-turn state;
- shake phase and completion;
- collision correction;
- FOV-region entry and exit;
- authored-shot selection; and
- safe fallback after invalidation.

## Failure behavior

The subsystem rejects or safely recovers from:

- missing or stale definitions;
- incompatible presets;
- invalid target cardinality;
- unavailable modes;
- missing input actions;
- non-finite snapshots or views;
- invalid spline or authored-shot data;
- stale locators or world anchors;
- collision penetration without a safe correction;
- modifier overflow or invalid duration;
- target destruction or streaming invalidation; and
- duplicate completion or cleanup.

A failed optional camera request never blocks gameplay completion. A required
authored camera failure follows its mission or interaction presentation policy
without mutating domain state.

## Verification

Automated tests prove:

- every rig and preset identity resolves exactly once;
- source data converts into stable complete presets;
- each local player has isolated request, input, collision, and modifier state;
- every mode validates its required target fields;
- rail and authored-shot evaluation is deterministic;
- development-only modes cannot enter shipping rotation;
- collision never leaves the final camera in blocking geometry;
- shake and transitions release every handle;
- world streaming cannot leave stale targets or locators;
- final views preserve supported aspect ratios and safe-area policy; and
- cancellation restores an eligible safe camera without changing gameplay state.
