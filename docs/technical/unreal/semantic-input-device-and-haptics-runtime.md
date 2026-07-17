# Semantic input, device, and haptics runtime

- Status: Active
- Last reviewed: 2026-07-16

## Governing decisions and specifications

- [UI parity boundary](../../adr/unreal/ui/ui-parity-boundary.md)
<!-- markdownlint-disable-next-line MD013 -->
- [HUD, radar, camera, and navigation parity](../../adr/unreal/ui/hud-radar-camera-and-navigation.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Common UI front end and progress projection](../../adr/unreal/ui/common-ui-frontend-and-progress-projection.md)
- [Runtime parity boundary](../../adr/unreal/runtime/remake-parity-boundary.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Portable save storage and lifecycle](../../adr/unreal/runtime/portable-save-storage-and-lifecycle.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Application lifecycle and mode runtime](application-lifecycle-and-mode-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Device configuration and save-slot runtime](device-configuration-and-save-slot-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Frontend screen flow and settings runtime](frontend-screen-flow-and-settings-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Native vehicle physics, control, damage, and presentation runtime](native-vehicle-physics-control-damage-and-presentation-runtime.md)

## Purpose

This specification defines semantic actions, physical-device discovery,
local-player assignment, mapping contexts, rebinding, pointer presentation,
controller hotplug, rumble, haptics, and steering-wheel force feedback.

It replaces platform-specific button enums, device-location strings, fixed
controller arrays, shared active-state bit masks, raw mappable callbacks,
custom physical-to-logical arrays, process-wide button timestamps, and
platform-specific rumble tables compiled into gameplay code.

All supported devices produce one semantic action model. Device capability and
presentation may differ; mission, vehicle, camera, menu, and progression
behavior
do not.

## Ownership

<!-- markdownlint-disable MD013 -->

| Authority | Responsibility |
| :--- | :--- |
| Enhanced Input | Action values, triggers, modifiers, mapping contexts, and devices. |
| Common UI | Menu actions, active input method, glyphs, focus, and pointer use. |
| Local-player input subsystem | Player-specific leases, profiles, rebinding, and observations. |
| Platform input adapter | Device discovery, capabilities, hotplug, and text entry. |
| Haptics subsystem | Pattern selection, blending, output, accessibility, and teardown. |
| Gameplay systems | Consume semantic actions and publish non-authoritative cues. |
| Device configuration | Requested bindings, sensitivity, inversion, and haptics preferences. |

<!-- markdownlint-enable MD013 -->

The input runtime does not own mission state, vehicle state, camera state,
front-end navigation state, or durable progression.

## Runtime topology

The input module owns these C++ types:

- `USharInputActionCatalog`, immutable semantic action definitions;
- `USharInputProfileDefinition`, default mappings and capability rules;
- `USharLocalPlayerInputSubsystem`, one authority per local player;
- `FSharInputContextLease`, move-only mapping-context ownership;
- `FSharInputDeviceId`, stable runtime device identity;
- `FSharInputDeviceObservation`, immutable device and capability snapshot;
- `FSharPlayerDeviceAssignment`, local-player and device association;
- `USharHapticPatternDefinition`, immutable rumble or haptic pattern;
- `USharWheelForceFeedbackDefinition`, spring, damper, and force policy;
- `USharLocalPlayerHapticsSubsystem`, one output authority per local player;
- `FSharHapticHandle`, move-only active-effect ownership; and
- `FSharInputBindingTransaction`, validated rebinding operation.

Physical device objects and platform handles remain behind engine and platform
adapters. Gameplay code consumes semantic actions and stable observations.

## Semantic action catalog

Every action definition contains:

<!-- markdownlint-disable MD013 -->

| Field | Contract |
| :--- | :--- |
| `ActionId` | Stable namespaced semantic identity. |
| `ValueType` | Boolean, scalar, axis pair, or bounded vector. |
| `Owner` | Front end, gameplay, vehicle, camera, pause, or development. |
| `TriggerPolicy` | Started, ongoing, triggered, completed, and cancelled rules. |
| `ModifierPolicy` | Dead zone, scale, inversion, curve, and smoothing rules. |
| `ConflictGroup` | Actions that cannot share one binding in the same profile. |
| `Availability` | Platform, device, mode, and package capability predicate. |
| `Presentation` | Localizable label, category, and glyph role. |

<!-- markdownlint-enable MD013 -->

Examples include move, look, accelerate, reverse, steer, brake, handbrake,
interact, attack, jump, sprint, horn, vehicle reset, camera controls, pause,
menu navigation, submit, cancel, pointer click, and accessibility actions.

A platform button name, direct-input offset, physical array index, or source
enum
ordinal is never the semantic identity.

## Mapping contexts and leases

Mapping contexts are activated through explicit leases. Initial context families
include:

- boot and legal presentation;
- front-end navigation;
- gameplay on foot;
- gameplay in vehicle;
- camera variant or first-person controls;
- pause menu;
- super-sprint selection and race;
- demonstration session;
- cinematic or animated-camera restrictions;
- developer console; and
- touch presentation.

Each lease declares local player, context identity, priority, owning mode or
feature, expected revision, and release reason. A context becomes active only
when its owner commits.

World teardown, application-mode exit, local-player removal, feature
deactivation, focus loss where required, and cancellation release affected
leases. A released context cannot continue receiving hidden input.

Priority resolves overlapping mappings. An integer active-state bit mask does
not select behavior, and animated-camera restrictions do not trap the player in
one special global state.

## Action value and timing

Enhanced Input supplies action values and trigger transitions. The local-player
subsystem records a bounded observation containing:

- semantic action identity;
- value and value type;
- trigger transition;
- local-player and device identity;
- input profile and mapping revision;
- engine-frame and monotonic time observation; and
- active context identity.

One process-wide static button clock is not used. Each action transition follows
engine input timing and local-player scope.

Digital press, release, hold, repeat, tap, chord, and sequence semantics are
declared by triggers. Analog values are finite and normalized before gameplay
consumption.

## Analog normalization

Analog definitions may declare:

- inner and outer dead zones;
- axial or radial dead-zone policy;
- sign and inversion;
- sensitivity scale;
- response curve;
- saturation;
- smoothing or filtering window;
- snap or digital threshold; and
- device-specific calibration.

Normalization occurs once in the input adapter or Enhanced Input modifier chain.
Gameplay systems do not reinterpret one physical axis independently.

POV hats and directional pads map to semantic digital or axis-pair actions. A
POV angle is not converted through platform-specific code inside gameplay.

## Device discovery

The platform adapter reports connected devices through engine device identities
and capabilities. An observation includes:

- stable runtime device identity;
- device class;
- platform user and local-player association when known;
- connection state;
- available buttons, axes, pointer, text, haptic, and force-feedback features;
- glyph and input-method family;
- calibration state;
- battery or transport data when safely available; and
- observation revision.

Device classes include keyboard, mouse, gamepad, steering wheel, touch, and
platform text-entry surfaces.

Runtime discovery does not construct device identity from strings such as port,
slot, channel, joystick number, or USB number. Those values may appear only in
adapter diagnostics.

## Hotplug and reconnection

Connection changes produce typed device observations. The assignment service:

1. validates the platform user and device;
1. preserves an existing compatible assignment when possible;
1. pauses or limits affected gameplay according to product policy;
1. requests user reassignment when identity is ambiguous;
1. restores active mapping leases after a verified reconnect; and
1. records a terminal result.

A disconnected device cannot leave stale callbacks, active haptics, or a hidden
controller pointer. Reconnection does not infer player ownership from physical
enumeration order.

## Local-player assignment

A player-device assignment contains local-player identity, platform user,
primary device, optional companion devices, assignment revision, and accepted
mode or join transaction.

Keyboard and mouse may form one companion-device set. A gamepad or wheel may be
primary for one local player. Touch belongs to the device-local player unless a
platform adapter proves another mapping.

Split-screen assignment is transactional. Every participant must have a unique
local-player identity and a compatible device policy before race or gameplay
mode commits.

Compile-time player counts and direct controller-index arrays do not define the
current product limit. The product and platform policies declare supported local
players and test them explicitly.

## Physical-device adapters

Keyboard, mouse, gamepad, wheel, and touch adapters translate engine keys and
axes into semantic bindings. An adapter may normalize platform naming and
capabilities but cannot create new gameplay actions.

Generated default profiles replace source tables that map physical offsets to
virtual-button numbers. Profile generation validates:

- every physical binding resolves through an engine key;
- every action exists in the action catalog;
- value types are compatible;
- required actions have at least one supported binding;
- no forbidden conflict exists;
- aliases resolve uniquely; and
- platform overrides preserve semantic parity.

## Keyboard

Keyboard mappings use engine key identities. Banned or reserved combinations are
policy data, not hardcoded scan-code tests. Platform shortcuts, text input, and
accessibility keys remain available according to platform requirements.

Key repeat is consumed only by actions whose trigger policy permits it. Text
entry uses native platform or Slate text input rather than gameplay key events.

## Mouse and pointer

Mouse motion provides a relative gameplay look path and a pointer path for
Common
UI. The active mode and input-method policy choose presentation; one raw mouse
object does not switch the entire process between front-end and gameplay modes.

Pointer behavior uses Slate and Common UI hit testing, capture, focus, safe
area,
and viewport transforms. Custom hotspot enums and manual pixel-coordinate tests
do not define widget interaction.

The cursor asset is a cooked UI presentation asset selected by role. It is not a
manually rendered world drawable. Cursor visibility and capture restore through
an explicit lease on close, focus loss, mode change, or viewport destruction.

Slider dragging, button activation, scrolling, and directional navigation emit
semantic Common UI actions. A pointer click cannot call front-end controller
methods directly.

## Gamepad

Gamepad input uses engine keys for sticks, triggers, directional pads, face
buttons, shoulders, stick clicks, and supported auxiliary controls.

Axis and trigger ranges are normalized by the profile. Device glyphs are
selected
through the active Common Input method and platform mapping. A gameplay action
is
not renamed because one platform labels a face button differently.

## Steering wheel

A steering-wheel adapter declares:

- steering axis and calibrated range;
- accelerator, brake, clutch, and optional combined-pedal policy;
- buttons and directional controls;
- wheel rotation capability;
- rumble, haptic, spring, damper, and constant-force capabilities;
- independent-axis support; and
- device-specific calibration profile.

Wheel controls map to the same accelerate, brake, steer, handbrake, reverse,
turbo, horn, reset, menu, and camera actions used by other devices. Semantic
vehicle-command projection, controller leases, native movement, and accepted
read-back follow
<!-- markdownlint-disable-next-line MD013 -->
[Native vehicle physics, control, damage, and presentation runtime](native-vehicle-physics-control-damage-and-presentation-runtime.md).
Wheel-only presentation or tuning does not change vehicle physics authority.

## Device sessions and action dispatch

Every connected physical device is represented by one revisioned device session.
The session owns the engine device handle, capability snapshot, calibration,
active profile, local-player assignment, mapping leases, and output handles.

Connection callbacks update only the device session. They do not call gameplay
objects, mutate global button arrays, or infer player identity from a port
index. Semantic actions are emitted only after the session validates:

- the device and profile revisions;
- local-player ownership;
- active mapping context;
- physical key and value type;
- analog normalization and finite range;
- trigger and modifier policy; and
- focus or capture requirements.

A disconnect first emits a typed device observation, releases active output,
cancels held-action state, and then enters the assignment recovery policy. A
reconnect creates a new observation revision and cannot revive stale callbacks.

Character, vehicle, front-end, camera, and development inputs are action-catalog
families, not separate arrays of virtual button numbers. Family membership
controls availability and conflict validation; it never changes action identity.

## Development input injection

Automated input injection is available only to editor, automation, and
explicitly authorized diagnostic builds. A request identifies:

- automation session and deterministic seed;
- local player and active device profile;
- semantic action identity;
- typed value and trigger transition;
- duration or release condition;
- expected world and mapping revisions; and
- maximum action count and rate.

Injected actions traverse the same semantic validation and gameplay consumption
path as physical input. They cannot write physical button arrays, bypass focus,
select arbitrary integer actions, or continue after session cancellation.

Random-action fuzzing uses a bounded catalog subset and records every selected
action and seed. It is excluded from shipping packages and never becomes a
command-line gameplay option.

## Wheel output channels

Wheel output is resolved by capability rather than a platform force-type enum.
The adapter may expose centering spring, damper, constant force, periodic
vibration, collision impulse, and ordinary rumble channels.

Each update contains finite magnitude, direction, frequency or pulse interval,
duration, priority, and definition revision. Unsupported channels use a declared
fallback or return `unavailable`; they are never cast to the nearest hardware
effect number.

## Rebinding

A binding transaction includes local profile, action identity, device class,
proposed engine key or axis, modifiers, expected configuration revision, and
conflict policy.

The transaction:

1. verifies action and device compatibility;
1. rejects reserved or unavailable inputs;
1. detects same-context conflicts;
1. applies swap, replace, clear, or reject policy;
1. verifies required-action coverage;
1. stages the configuration revision;
1. read-backs the active mapping; and
1. commits or rolls back atomically.

Binding display text and glyphs are projections. Device-local configuration
stores
engine key identities, modifiers, and schema revision, not platform scan codes.

## Frontend binding capture

The Common UI binding editor opens one typed capture request through the local
player input subsystem. It declares the action, binding slot, eligible device
classes and value types, reserved-input policy, timeout, cancellation, conflict
policy, and expected configuration revision.

The input subsystem returns an engine-key identity, device class, axis or
direction metadata, modifiers, and capability revision. The frontend displays
localized labels and glyphs from that accepted result. It never converts virtual
key integers, button offsets, platform strings, or manual pointer hotspots into
binding authority.

A disconnect, focus loss, back action, timeout, feature removal, or failed
conflict resolution cancels the capture and restores the last accepted draft.
Commit persists the complete validated profile through the device-configuration
transaction defined by the
<!-- markdownlint-disable-next-line MD013 -->
[frontend screen flow and settings runtime](frontend-screen-flow-and-settings-runtime.md).

## Haptic cue model

Gameplay and presentation systems request a canonical haptic cue after an
accepted cause. A request contains:

- pattern identity and revision;
- local-player identity;
- cause and transaction identity;
- intensity scale;
- duration scale or explicit bounded duration;
- optional direction or contact data;
- priority and blend group;
- accessibility classification; and
- world or presentation scope.

The haptics subsystem resolves the active device capabilities and produces an
output pattern. The cue is non-authoritative and cannot change damage,
collision,
mission, camera, or progression state.

## Haptic pattern definition

A pattern definition may contain:

- low- and high-frequency motor envelopes;
- trigger or adaptive-control channels when available;
- amplitude and frequency curves;
- pulse spacing and count;
- attack, sustain, and release;
- maximum duration;
- blend and interruption policy;
- fallback pattern; and
- platform capability overrides.

Patterns use simulation time or presentation time according to declared scope.
Frame-modulo pulse code and assumed motor indices are not portable authority.

## Collision and surface haptics

Collision cues receive normalized physical evidence from the impact-response
adapter. The haptics subsystem may map impulse, relative speed, surface,
vehicle,
and player role into a bounded intensity.

Repeated contacts use cooldown, aggregation, and priority policy. One physics
contact cannot start duplicate effects through multiple callback paths.

Road, ground, damage, explosion, collection, UI, and notification cues use
separate definitions. A stronger simultaneous cue may blend with or supersede a
weaker cue according to its group.

## Haptic preferences and accessibility

Device-local configuration owns:

- master haptics enabled state;
- optional intensity scale;
- separate gameplay and UI haptic preferences when supported;
- reduced-intensity accessibility profile; and
- wheel force-feedback preference.

Disabling haptics stops active output safely but does not suppress the
underlying
gameplay event. Preferences are device-local and do not enter portable
progression.

## Wheel force feedback

Wheel force feedback uses typed definitions for:

- centering spring;
- steering damper;
- constant directional force;
- collision impulse;
- road texture or vibration; and
- bounded pulse feedback.

A definition may declare center offset, dead band, positive and negative
coefficients, saturation, magnitude, direction, duration, attack, release, and
update rate.

Vehicle simulation supplies normalized steering, speed, slip, suspension,
collision, and surface observations. The wheel adapter converts those values to
supported engine force-feedback channels. It cannot feed force output back into
vehicle physics.

Unsupported spring, damper, or constant-force capabilities return an unavailable
observation and use the declared rumble fallback when one exists. No guessed
hardware counter or output point is used.

## Effect lifecycle and blending

Every active haptic or force-feedback effect has a move-only handle. Effects end
on completion, explicit stop, device disconnect, local-player removal, mode
exit,
world teardown, suspension, or subsystem destruction.

Blending is deterministic by priority, group, start sequence, and definition
policy. Motor gain, force magnitude, and duration are clamped to finite
supported
ranges. A later weak effect cannot accidentally clear a stronger unrelated
channel.

Late updates from released effects are ignored and recorded. Reconnection never
resumes an old effect unless its owner explicitly requests a new cue.

## Platform reset and system actions

Platform-level reset, dashboard, guide, or system-menu actions remain in the
platform application adapter. Gameplay input does not implement them as hidden
button combinations.

When a platform requires a reset chord or reserved action, the adapter reports a
typed platform request after the required hold and policy checks. It is excluded
from ordinary rebinding and gameplay action catalogs.

## Persistence

Device-local configuration stores:

- requested bindings and modifiers;
- sensitivity and inversion;
- calibration references;
- active glyph or input-method preferences where user-selected;
- haptics enabled state and intensity; and
- force-feedback preferences.

Transient button values, held durations, device pointers, current assignments,
active effects, mode leases, and pointer positions are not persisted.

## Mods and game features

A validated feature may add namespaced actions, mapping contexts, and haptic
patterns when its manifest declares owner, availability, conflicts, teardown,
package policy, and tests.

A feature cannot replace required base actions, capture all devices globally,
broaden local-player scope, bypass accessibility settings, or leave mappings and
haptics active after deactivation.

## Diagnostics

Development observations include connected devices, capabilities, local-player
assignments, active input method, mapping contexts, action transitions, binding
conflicts, active haptic handles, output channels, clamps, drops, and teardown.

Diagnostics use stable identities and redact platform user data. Raw device
locations, private paths, and arbitrary hardware descriptors are not public
runtime output.

## Failure behavior

The input runtime fails closed on:

- unknown action, profile, key, context, or device identity;
- incompatible value type or device capability;
- duplicate or ambiguous player assignment;
- reserved or conflicting binding without accepted resolution;
- non-finite analog values;
- stale mapping or configuration revision;
- released context or haptic handle use;
- device disconnect during output;
- unsupported force-feedback capability;
- invalid platform callback scope; and
- unauthorized shipping or mod input registration.

Gameplay receives neutral input when required input evidence is invalid. A
device
or presentation failure cannot synthesize an action or mutate gameplay state.

## Validation

Catalog and cook validation prove:

- required actions exist and have compatible default bindings;
- every mapping context has an owner and release path;
- platform overrides preserve semantic parity;
- device-local settings match the configuration schema;
- cursor and glyph presentation assets resolve;
- haptic patterns have finite bounded curves and duration;
- wheel force definitions declare capability fallbacks;
- game-feature teardown releases every lease and effect; and
- development-only actions are absent from unauthorized packages.

## Tests

Required tests include:

- keyboard, mouse, gamepad, wheel, and touch action parity;
- action trigger and analog modifier behavior;
- dead zone, inversion, sensitivity, and curve normalization;
- context priority and lease teardown;
- front-end, gameplay, pause, camera, demo, and race mappings;
- local-player and split-screen device isolation;
- hotplug, reconnect, reassignment, and disconnect pause;
- binding conflict, swap, reset, migration, and rollback;
- pointer focus, capture, cursor, slider, and viewport teardown;
- haptics enable, scale, blend, priority, and cancellation;
- collision cue normalization and duplicate suppression;
- low- and high-frequency capability mapping;
- wheel spring, damper, constant force, and fallback;
- suspension and application-mode cleanup;
- mod registration and teardown; and
- shipping exclusion for development actions.

## Invariants

- Gameplay consumes semantic actions, never platform button ordinals.
- Every mapping context has explicit local-player ownership and teardown.
- Device enumeration order never assigns player identity.
- Input timing is local and engine-derived, not one process-wide button clock.
- Rebinding is typed, validated, revisioned, and atomic.
- Pointer presentation uses Common UI and Slate hit testing.
- Haptics and force feedback never become gameplay authority.
- Accessibility preferences apply to every output path.
- Disconnected devices retain no active callbacks or effects.
- Platform-specific capability differences do not change gameplay semantics.
