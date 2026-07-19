# Character animation clip catalog and vehicle-handoff choreography runtime

- Status: Active
- Last reviewed: 2026-07-17

## Governing decisions and specifications

<!-- markdownlint-disable-next-line MD013 -->
- [Hexagonal Unreal runtime](../../adr/unreal/architecture/hexagonal-runtime-and-no-technical-debt.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Staged mesh import and world assembly](../../adr/unreal/import-adapters/staged-mesh-import-and-world-assembly.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Runtime parity boundary](../../adr/unreal/runtime/remake-parity-boundary.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Historical source-document evidence classification and publication boundary](historical-source-document-evidence-classification-and-publication-boundary.md)
- [Animation clip timing](../fbx/animation/clip-timing.md)
- [Animation rig model](../fbx/animation/rig-model.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Native import, material rebuild, and world assembly](native-import-material-and-world-assembly.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Playable avatar, character controller, and footprint runtime](playable-avatar-character-controller-and-footprint-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Native vehicle physics, control, damage, and presentation runtime](native-vehicle-physics-control-damage-and-presentation-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Native asset load request and streaming runtime](native-asset-load-request-and-streaming-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Typed event and observation routing runtime](typed-event-and-observation-routing-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Vehicle audio and avatar-sound runtime](vehicle-audio-and-avatar-sound-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Transient VFX and breakable-presentation runtime](transient-vfx-and-breakable-presentation-runtime.md)

## Purpose

This specification defines the canonical shared character-animation library,
source scene normalization, reference-pose handling, clip variants, runtime
playback, locomotion synchronization, dialogue gestures, reactions, action
clips, and phased vehicle enter and exit choreography used by the native Unreal
runtime.

It replaces source-era animation filenames, backup-directory precedence,
hand-authored batch commands, animation-choice text files, implicit rig aliases,
raw animation-curve inventories, hard-coded clip arrays, direct animation-player
calls, frame-number polling, vehicle-door timing guesses, and
animation-completion callbacks that mutate gameplay state.

Historical scene, pose, batch, and configuration files are non-public evidence.
They may establish normalized clip facts after review, but they do not become
public repository content, packaged runtime files, executable scripts, or target
asset identities.

## Native Unreal foundation

The implementation uses native Unreal facilities where applicable:

- Skeleton and Skeletal Mesh assets;
- Animation Sequences tied to an accepted Skeleton;
- Animation Blueprints and state machines;
- Blend Spaces for continuous locomotion policy when appropriate;
- Sync Groups and Sync Markers for compatible cyclic animation;
- Animation Montages, Slots, Slot Groups, and Sections for interruptible or
  phased actions;
- Animation Notifies and Notify States for typed timing observations;
- animation curves for approved presentation parameters;
- root-motion extraction and Character Movement integration according to policy;
- Motion Warping only when a validated interaction target requires bounded
  alignment;
- Pose Assets, additive animation, or reference-pose data when a pose has an
  accepted runtime purpose;
- Asset Manager bundles and retained streamable handles;
- native network montage and root-motion facilities where multiplayer policy
  accepts them; and
- Sequencer only for authored cinematic ownership, not ordinary gameplay
  locomotion or interaction authority.

A custom animation player, clip scheduler, skeleton evaluator, blend system,
notify dispatcher, or root-motion solver requires a separate accepted decision.

## Ownership

<!-- markdownlint-disable MD013 -->

| Authority | Responsibility |
| :--- | :--- |
| Character definition | Declares compatible Skeleton, animation library, movement profile, interaction policy, and fallbacks. |
| Shared animation library | Owns stable clip, pose, choreography, phase, marker, curve, variant, and compatibility identities. |
| Import pipeline | Converts reviewed evidence into deterministic normalized animation payloads and native assets. |
| Character animation adapter | Projects accepted movement, action, dialogue, damage, interaction, and lifecycle state into native animation inputs. |
| Character Movement | Owns authoritative ordinary movement integration and accepted root-motion consumption. |
| Vehicle-handoff application service | Owns access validation, seat reservation, door and occupant transactions, control transfer, cancellation, and terminal results. |
| Vehicle runtime | Owns vehicle, seat, door, collision, physics, controller, and occupant revisions. |
| Animation Blueprint | Owns native pose evaluation, state-machine transitions, blends, slots, montages, sync groups, and output pose. |
| Typed-event boundary | Publishes immutable accepted animation markers and terminal observations. |
| Audio and VFX services | Consume accepted markers and state observations for presentation. |
| Diagnostics | Observe immutable catalog, playback, transition, marker, root-motion, handoff, loading, and failure snapshots. |

<!-- markdownlint-enable MD013 -->

Animation playback may visualize or time one accepted transaction. It cannot
independently grant access, reserve a seat, attach an occupant, open a door,
transfer possession, apply damage, complete a mission, or persist state.

## Runtime identities

The boundary uses stable typed identities for:

- character definition;
- rig and Skeleton revision;
- shared animation-library revision;
- clip definition and clip asset;
- pose definition;
- choreography definition;
- choreography phase;
- variant family and variant selection;
- movement profile and gait;
- action and reaction;
- dialogue gesture;
- vehicle, seat, door, side, height class, and occupant role;
- montage, slot, section, sync group, and sync marker;
- notify and curve schema;
- playback request and playback lease;
- handoff transaction;
- local player, controller, world, feature, and application mode; and
- terminal result and diagnostic correlation.

Source filenames, directory names, backup labels, numeric clip positions, curve
array positions, node order, batch-line order, configuration-line order, and raw
joint names are not runtime identity.

## Shared rig-family animation library

`USharCharacterAnimationLibraryDefinition` is one Primary Asset per compatible
rig family. It declares:

- library identity and revision;
- compatible rig, Skeleton, mesh, and Animation Blueprint families;
- character or costume eligibility predicates only for clips that require them;
- required locomotion, idle, turn, jump, fall, recovery, reaction, action,
  dialogue, and vehicle-handoff clip roles;
- optional clip and pose roles;
- variant axes and compatibility predicates;
- root-motion and in-place policy;
- sync groups, markers, and gait policy;
- montage, slot, and section schemas;
- notify and curve schemas;
- transition and interruption policy;
- movement, collision, camera, audio, VFX, and accessibility bindings;
- loading bundles and retention policy;
- quality and platform policy;
- fallback definitions; and
- validation tolerances.

The library maps semantic roles to native assets. Every compatible clip is
stored once below
`/Game/SHAR/Art/Characters/Animations/<rig_family>/`; no shipping clip is copied
into a character folder. Runtime never constructs an animation asset path by
concatenating a character prefix, role suffix, direction, height, side, or seat
label.

## Clip definition

`FSharCharacterAnimationClipDefinition` contains:

- stable clip identity and revision;
- semantic role and tags;
- native Animation Sequence soft reference;
- compatible Skeleton and rig revisions;
- exact frame rate, sample count, duration, and accepted interval;
- looping, finite, hold, or pose-reference classification;
- root-motion policy;
- expected animated-track profile;
- required and optional curves;
- notify and marker schema;
- sync group and role;
- locomotion speed or angular range when applicable;
- entry, exit, action, reaction, or dialogue applicability;
- additive, full-body, upper-body, or masked-body policy;
- interruption and blend policy;
- variant family and selection predicates;
- quality and platform eligibility;
- fallback; and
- conversion and native read-back evidence.

A clip definition is invalid when its Skeleton, timing, track set, root policy,
role, markers, or variant predicates are ambiguous.

## Evidence intake

Historical animation evidence is classified by
<!-- markdownlint-disable-next-line MD013 -->
[Historical source-document evidence classification and publication boundary](historical-source-document-evidence-classification-and-publication-boundary.md).

For each scene, pose, batch, or configuration item, review records privately:

- opaque evidence identity and digest;
- media type and encoding;
- declared source time unit;
- clip or pose classification;
- animated-track names and channel types;
- clip metadata and interval evidence;
- presence or absence of an embedded hierarchy;
- compatibility with the accepted canonical rig;
- source duplication or variant relationships;
- batch or selection semantics when relevant;
- conversion warnings; and
- terminal review result.

Public output contains only independently written normalized contracts,
deterministic recipes, native asset identities, aggregate validation evidence,
and approved tests. Source scene text, raw curves, comments, private locations,
editor metadata, obsolete tool commands, and historical configuration text do
not enter Git or packaged content.

## Animation-only scenes

An animation-only scene contains animated channels and clip metadata but no
accepted embedded joint hierarchy. It must target one previously verified rig
and Skeleton revision.

Import resolves every animated track through the canonical rig alias table. It
rejects:

- unknown required tracks;
- duplicate normalized tracks;
- one source track mapping to multiple joints;
- unexpected helper transforms becoming Skeleton roots;
- missing required movement or orientation tracks;
- incompatible Skeleton revisions;
- unsupported channel types; and
- track sets that differ without a declared reduced-track variant.

Animation-only evidence cannot define rest pose, parentage, bind state, skin
weights, or a new Skeleton.

## Track-profile compatibility

Each catalog family declares one or more accepted track profiles. A profile
contains:

- required translation, rotation, scale, and custom channels;
- optional channels;
- intentionally omitted channels;
- expected root and motion tracks;
- curve aliases;
- tolerance for constant-channel removal; and
- compatibility result.

A reduced-track scene is not accepted merely because an importer can open it. It
requires a declared purpose such as a null-reduction experiment, verified
constant-channel removal, additive subset, or intentionally masked animation.

The importer compares the normalized track set with the profile before creating
or replacing a native asset. Silent missing-track acceptance is forbidden.

## Backup, duplicate, and revision evidence

A backup directory or older scene is evidence of chronology, not automatic
canonical precedence. Duplicate and revision candidates are compared using:

- semantic role;
- Skeleton and track compatibility;
- timing and sample range;
- root displacement;
- marker and curve evidence;
- visual parity review;
- conversion success;
- native read-back;
- downstream catalog references; and
- explicit operator-approved canonical selection.

The target catalog records one canonical clip revision and optional retained
comparison evidence. It does not package both copies under source-derived names
unless they are verified distinct variants with separate semantic purposes.

## Pose-reference evidence

A pose scene is classified as one of:

- rig reference pose;
- animation starting or ending pose;
- additive base pose;
- blend-space sample pose;
- interaction alignment pose;
- diagnostic comparison pose; or
- non-runtime production evidence.

A pose does not automatically become an Animation Sequence. When runtime use is
accepted, it becomes a Pose Asset, one-frame Animation Sequence, additive base,
or deterministic test fixture according to purpose.

Pose evidence cannot redefine the Skeleton rest pose without a separate accepted
rig revision and complete mesh, skin, animation, and retargeting validation.

## Batch conversion evidence

Historical batch commands establish only that a source-era conversion operation
or tool family existed. They do not become executable project scripts.

A repository-owned conversion recipe replaces them and declares:

- supported input class;
- canonical rig and catalog revision;
- exact frame-rate and range policy;
- track normalization;
- coordinate and unit conversion;
- root-motion policy;
- curve and marker extraction;
- output format and destination identity;
- deterministic environment and tool versions;
- validation and read-back;
- rollback; and
- diagnostics.

Relative command paths, shell behavior, environment assumptions, source tool
names, flags, and output locations remain non-public provenance unless an
independently authored target tool explicitly owns equivalent semantics.

## Animation-choice and rig configuration evidence

Historical animation-choice or choreography configuration may establish:

- rig and Skeleton roles;
- animation role membership;
- locomotion groups;
- idle selection;
- blend timing intent;
- priority intent;
- acceleration and turning thresholds;
- inverse-kinematics or foot-plant intent;
- root, orientation, balance, and motion roles; and
- source-era selection relationships.

The target converts only verified semantics into typed catalog fields, movement
profiles, sync groups, marker schemas, and Animation Blueprint policy.

Configuration line order, syntax, source paths, joint indexes, raw thresholds,
comments, and source tool assumptions do not become runtime authority.

## Clip taxonomy

The closed initial taxonomy includes:

- locomotion idle, walk, run, dash, jump, and transition clips;
- idle gestures and weakened or contextual idles;
- directional turn clips;
- jump, fall, flail, landing, and recovery clips;
- collision and run-into reactions;
- break, kick, dodge, and other action clips;
- dialogue and upper-body gesture clips;
- vehicle entry and exit choreography;
- cinematic-only clips;
- pose references; and
- diagnostic or rejected evidence.

New taxonomy values require a catalog schema revision. A filename token cannot
create a new gameplay action at runtime.

## Locomotion clips

Locomotion roles declare:

- gait identity;
- nominal movement speed or speed range;
- direction and angular range;
- looping policy;
- stride and foot-contact markers;
- sync group and leader or follower policy;
- root-motion or in-place policy;
- acceleration and deceleration transitions;
- start, stop, pivot, turn, dash, jump, and land relationships;
- slope, ground, airborne, and movement-mode eligibility;
- blend-space coordinates when used; and
- fallback.

Character Movement owns authoritative velocity, movement mode, floor, and
collision. The Animation Blueprint consumes immutable movement observations and
selects or blends compatible clips.

Animation playback does not integrate a second position, override collision, or
infer grounded state from a visual foot pose alone.

## Locomotion synchronization

Compatible cyclic walk and run clips use declared Sync Groups and markers. The
catalog defines:

- group identity;
- marker names and ordering;
- eligible clips;
- leader selection;
- notify suppression or forwarding policy;
- transition tolerances; and
- behavior when markers are missing.

A clip with missing required foot markers falls back or fails validation. It
does
not enter a synchronization group with guessed marker positions.

## Idle and contextual gestures

Idle definitions declare:

- base locomotion idle;
- contextual, weakened, impatient, dialogue, or personality role;
- full-body or layered-body policy;
- minimum and maximum eligible hold time;
- deterministic or weighted selection policy;
- cooldown and repetition policy;
- movement, dialogue, mission, camera, and interaction eligibility;
- interruption blend; and
- accessibility fallback.

Random idle selection uses a declared deterministic stream when gameplay capture
or replay requires reproducibility. An idle gesture cannot block required input,
movement, mission, or vehicle handoff.

## Directional turns

Turn clips declare:

- clockwise or counterclockwise direction;
- nominal angular displacement;
- accepted angular range;
- in-place or root-motion policy;
- start and end facing tolerances;
- foot and pivot markers;
- movement-mode and speed eligibility;
- mirroring policy; and
- fallback.

Separate authored directions remain separate variants unless a validated mirror
recipe proves Skeleton, asymmetry, prop, facial, and timing parity. Runtime does
not infer direction from filename case or negate root rotation blindly.

## Jump, fall, flail, landing, and recovery

Airborne and recovery roles declare:

- takeoff, ascent, apex, fall, flail, impact, landing, and recovery phases;
- movement-mode eligibility;
- root-motion policy;
- vertical-velocity and support observations;
- impact-severity and surface policy;
- landing and recovery markers;
- transition windows;
- interruption and ragdoll handoff; and
- fallback.

Character Movement and physics own airborne state, velocity, contact, and
impact.
Animation markers may request presentation, but they cannot declare a landing or
apply impact damage without accepted physical evidence.

## Collision and run-into reactions

Reaction definitions contain:

- cause and severity taxonomy;
- participant and surface eligibility;
- direction and body-region policy;
- movement cancellation or continuation rules;
- root-motion and displacement limits;
- camera, audio, VFX, haptic, and gameplay observation bindings;
- recovery phase; and
- deduplication identity.

A visual reaction cannot apply a second impulse or damage result. It consumes
the
accepted collision or damage transaction.

## Break, kick, dodge, and dash actions

Action clips declare:

- action identity and input or AI request;
- eligibility and cooldown;
- full-body or layered-body policy;
- movement, rotation, and root-motion ownership;
- active interaction or hit window;
- collision and query profile;
- marker schema;
- cancellation and chaining rules;
- camera, audio, VFX, and haptic presentation; and
- terminal result.

Animation completion alone cannot confirm a hit, break a prop, spend a resource,
or commit a mission result.

## Dialogue gestures

Dialogue gestures are presentation overlays correlated with one accepted
dialogue
line or conversation turn. A definition declares:

- gesture identity and semantic intent;
- full-body, upper-body, face, or additive policy;
- compatible locomotion and idle states;
- slot and Slot Group;
- start, emphasis, and release markers;
- line-duration scaling limits;
- interruption and speaker-change behavior;
- camera and subtitle correlation; and
- fallback.

Dialogue playback remains owned by the dialogue runtime. Gesture selection
cannot
change line identity, probability, subtitle text, queue priority, or
conversation
state.

## Vehicle-handoff variant axes

Vehicle entry and exit clips may vary by:

- entry or exit direction;
- driver, passenger, or other occupant role;
- seat and side;
- low, standard, or high vehicle access class;
- door present, absent, already open, or animation-disabled policy;
- open-door, traverse, seat, leave-seat, and close-door phase;
- composed all-phase sequence versus separate phases;
- character body size and movement profile;
- vehicle definition and hardpoint compatibility;
- root-motion, in-place, or motion-warping alignment policy; and
- fallback.

Every axis is typed catalog data. A token embedded in a historical filename is
provenance only until normalized and validated.

## Vehicle-handoff choreography definition

`USharVehicleHandoffChoreographyDefinition` declares:

- choreography identity and revision;
- enter or exit direction;
- compatible character, vehicle, seat, side, and height predicates;
- ordered phase definitions;
- native Montage and section identities;
- alignment transform and Motion Warping target policy;
- door and hardpoint bindings;
- occupant attachment and detachment markers;
- controller, input, camera, collision, and visibility policy;
- root-motion ownership;
- interruption and cancellation policy;
- audio, VFX, and haptic bindings;
- network authority policy;
- timeout and fallback; and
- terminal result schema.

A composed all-phase asset may implement the choreography when it exposes the
same typed phase markers and interruption semantics. A composed asset does not
remove the application transaction or make the montage authoritative.

## Handoff phases

The closed initial phases are:

1. `requested`;
1. `validating`;
1. `reserving_seat`;
1. `approaching_alignment`;
1. `opening_door`;
1. `traversing_threshold`;
1. `attaching_or_detaching_occupant`;
1. `seating_or_clearing_seat`;
1. `closing_door`;
1. `transferring_control`;
1. `restoring_collision_and_input`;
1. `completed`;
1. `cancelled`; and
1. `failed`.

The exact phase subset depends on the definition. Skipped optional phases are
explicit terminal phase results, not missing callbacks.

## Entry transaction

Vehicle entry proceeds as one revisioned transaction:

1. validate character, controller, vehicle, seat, access, world, mission, and
   feature revisions;
1. reserve the seat and required door or side;
1. validate asset and choreography readiness;
1. validate approach path, floor, collision clearance, and alignment target;
1. suppress incompatible input while retaining cancellation policy;
1. play the accepted approach or entry montage section;
1. consume correlated door and traversal markers;
1. attach the character to the accepted seat only after the application service
   accepts the attachment phase;
1. transfer possession, camera, input, collision, visibility, and occupant state
   atomically;
1. finish or close the door according to policy;
1. publish one completed terminal result; and
1. release temporary leases.

No marker may attach the character to a stale vehicle or seat revision.

## Exit transaction

Vehicle exit proceeds as one revisioned transaction:

1. validate occupant, vehicle, seat, controller, world, mission, and exit
   policy;
1. reserve an accepted exit side and destination transform;
1. validate floor, obstruction, traffic, physics, and streaming clearance;
1. select the compatible height, side, role, and door choreography;
1. suppress vehicle input according to policy;
1. open or project the door phase when required;
1. detach the occupant only after the application service accepts the phase;
1. place the character at the validated world transform;
1. restore character collision, movement, possession, input, and camera;
1. close or leave the door according to policy;
1. publish one completed terminal result; and
1. release seat, door, alignment, and temporary asset leases.

Failure to find a safe exit does not place the character inside collision or
silently complete the transaction.

## Door and vehicle adapter

The animation boundary receives immutable vehicle-handoff observations
containing:

- vehicle, seat, door, side, hardpoint, and occupant identities;
- transforms and revisions;
- vehicle height and access class;
- door presence, state, collision, and animation capability;
- seat occupancy and reservation state;
- vehicle movement and physics eligibility;
- world and streaming readiness;
- mission and interaction restrictions; and
- fallback candidates.

Animation cannot query mutable native vehicle internals or retain vehicle, door,
seat, mesh, or movement-component pointers across frames.

## Root motion and Motion Warping

Each clip declares one root policy:

- in-place with Character Movement ownership;
- authored root motion consumed by accepted movement policy;
- montage root motion for one bounded action;
- Motion Warping toward one validated target; or
- presentation-only root channels ignored by runtime.

Motion Warping may align a character to a validated seat, door, interaction, or
impact target within declared translation and rotation limits. It cannot repair
an invalid seat, blocked path, wrong vehicle height, incompatible side, stale
world transform, or missing choreography asset.

Root motion never bypasses collision, network authority, movement-mode, or
world-boundary validation.

## Montages, Slots, and Sections

Finite actions and phased interactions use native Montages when they require
sections, branching, interruption, layered playback, or network-aware root
motion.

The catalog declares:

- Montage identity;
- Slot Group and Slot;
- ordered Sections;
- compatible clip assets;
- section transition graph;
- blend in and out;
- root-motion policy;
- notify and marker schema;
- interruption points;
- fallback section; and
- terminal observation policy.

A Section name is stable catalog data, not an arbitrary source clip label. Two
sequences in the same Slot cannot overlap accidentally.

## Animation Blueprint state

The Animation Blueprint consumes immutable observations including:

- movement mode, speed, direction, acceleration, and angular velocity;
- floor, slope, airborne, falling, landing, and support evidence;
- action, reaction, dialogue, damage, and interaction state;
- vehicle-handoff transaction and phase;
- character, controller, world, feature, and asset revisions;
- quality and accessibility policy; and
- accepted montage playback state.

The Animation Blueprint produces a native pose and read-only diagnostic state.
It
does not own gameplay inventory, mission, seat, vehicle, damage, or persistence.

## Markers and Notifies

Markers are typed observations such as:

- left or right foot contact;
- takeoff, apex, landing, impact, and recovery;
- action active-window open and close;
- dialogue emphasis;
- hand reaches door or hardpoint;
- door may open or close;
- threshold traversal begins or ends;
- occupant may attach or detach;
- seat pose reached;
- control transfer presentation ready; and
- montage section completed.

Every marker carries clip, playback, montage, section, character, transaction,
world, and expected owner revisions. Stale, duplicate, out-of-order, or
incompatible markers are rejected.

A marker proposes a phase observation. The owning application service decides
whether the transaction may advance.

## Animation curves

Approved curves have stable schemas and declared consumers. Eligible uses
include:

- material or facial presentation;
- morph-target weights;
- foot or hand plant confidence;
- door or prop presentation;
- camera and audio presentation parameters;
- additive layer weights; and
- diagnostics.

Raw source curve names and indexes are normalized. Unknown curves are rejected,
quarantined, or ignored according to an explicit import rule. Curves cannot
mutate unrelated gameplay state.

## Blending and interruption

Every transition declares:

- source and destination roles;
- blend duration and profile;
- synchronization policy;
- root-motion continuity;
- interruption window;
- priority and arbitration;
- movement and collision policy;
- marker cancellation policy; and
- fallback.

Vehicle handoff, damage, death, ragdoll, world unload, controller removal,
application suspension, feature removal, and asset invalidation may interrupt
playback according to declared policy.

Cancellation invalidates pending markers before restoring or transferring state.
A late montage completion cannot complete a cancelled handoff.

## Movement and input integration

Enhanced Input and Character Movement own input interpretation and movement.
Animation consumes accepted semantic commands and movement observations.

During a bounded action or handoff, the application service declares which input
is:

- retained;
- buffered;
- suppressed;
- converted to cancellation;
- redirected to the vehicle; or
- restored at completion.

Animation cannot poll physical buttons, controller indexes, or platform-specific
input labels.

## Camera integration

Camera behavior consumes the same character and vehicle-handoff transaction
revision. Entry and exit definitions may request:

- retain current camera;
- blend to a declared vehicle or character rig;
- apply a bounded framing offset;
- delay ownership transfer until one accepted phase; or
- use an accessibility fallback.

Camera completion cannot attach an occupant or transfer control. A failed camera
blend does not roll back an otherwise accepted gameplay state unless policy
explicitly makes presentation readiness a transaction prerequisite.

## Audio, VFX, and haptics

Audio, VFX, and haptics consume accepted markers for:

- footsteps and skids;
- jumps, falls, impacts, and recovery;
- breaks, kicks, dodges, and dialogue gestures;
- door handles, hinges, closure, and occupant movement;
- seat attachment and control transfer; and
- cancellation or failure feedback.

Presentation services deduplicate by character, clip, marker, transaction, and
simulation-step identity. Presentation completion cannot advance gameplay.

## Asset loading and retention

Animation catalog bundles declare required and optional assets for:

- base locomotion;
- actions and reactions;
- dialogue gestures;
- vehicle entry and exit families;
- pose or additive references;
- montages and Animation Blueprints;
- curves, notifies, and marker schemas; and
- feature overlays.

A character may enter a mode only when its required animation scope is ready.
Optional presentation uses a declared fallback. Late loading completion carries
catalog, character, world, feature, and request revisions and cannot replace a
newer asset scope.

## Local multiplayer

Each local player has isolated controller, character, camera, input, montage,
transaction, and presentation state. One player's entry, exit, dialogue gesture,
reaction, or loading result cannot suppress or complete another player's state.

Shared asset retention may deduplicate immutable native assets, but playback and
transaction leases remain per character and per local player.

## Networking and multiplayer mods

The base single-player runtime keeps one authority path suitable for an optional
server-authoritative adapter. A multiplayer implementation must replicate or
reconstruct:

- character and vehicle identities;
- accepted movement and root-motion policy;
- montage and section state when required;
- handoff transaction and phase;
- seat, door, occupant, and control authority;
- marker correlation needed for presentation; and
- cancellation and terminal results.

Clients cannot advance seat attachment, possession, or mission state from a
local
animation marker alone.

## Accessibility and quality

Quality may reduce secondary additive layers, facial detail, curve frequency,
optional dialogue gestures, distant animation evaluation, cosmetic door detail,
or presentation effects.

Quality cannot change:

- clip identity or Skeleton compatibility;
- movement, root-motion, collision, or handoff authority;
- required phase and marker semantics;
- seat, side, role, or vehicle-height compatibility;
- transaction ordering;
- local-player isolation; or
- gameplay results.

Accessibility policy may replace rapid motion, camera movement, strong haptics,
or nonessential gesture detail while preserving semantic timing and state.

## Game Feature and mod overlays

A validated feature may add namespaced rig-family animation libraries, clips,
poses, variants, montages, sections, marker schemas, dialogue gestures, and
vehicle-handoff choreographies.

An overlay cannot:

- replace a protected base Skeleton or rig in place;
- mutate a base catalog revision;
- override native movement or physics globally;
- claim another feature's clip identities;
- inject raw scene files or executable batch commands;
- retain native animation assets after feature removal; or
- leave active playback, marker, handoff, seat, door, or asset leases.

Feature removal cancels owned playback and transactions, rejects stale markers,
releases retained handles, unregisters namespaced definitions, restores scoped
base state, and proves zero owned resources.

## Diagnostics

Development diagnostics expose immutable snapshots of:

- character, rig, Skeleton, catalog, clip, pose, montage, and asset revisions;
- semantic role and selected variant predicates;
- timing, duration, sample, looping, and root-motion policy;
- normalized track profile and compatibility result;
- Animation Blueprint state, sync group, marker, Slot, and Section;
- movement, action, reaction, dialogue, and vehicle-handoff observations;
- handoff transaction, phase, seat, door, side, role, and height class;
- loading and retained handles;
- recent accepted and rejected markers;
- cancellation and terminal results;
- network and local-player ownership; and
- fallback or degraded state.

Diagnostics do not expose source scene text, raw animation curves, private
paths,
historical commands, configuration prose, or mutable native object pointers.

## Failure behavior

The boundary fails closed when:

- the canonical rig or Skeleton is incompatible;
- required tracks are missing or ambiguous;
- timing, duration, interval, or sample evidence is contradictory;
- a reduced-track variant lacks a declared profile;
- root-motion policy is missing;
- markers, curves, Slots, Sections, or sync groups are invalid;
- a pose is misclassified as a runtime clip;
- backup or duplicate evidence has no canonical selection;
- a historical batch or configuration file is treated as executable authority;
- required animation assets are unavailable;
- no compatible vehicle-handoff variant exists;
- seat, door, side, height, path, floor, collision, world, or authority
  validation
  fails;
- a marker is stale, duplicate, or out of order;
- an interrupted transaction receives a late completion;
- feature teardown leaves owned resources; or
- generated output contains non-public evidence metadata.

Failure leaves the previous accepted catalog and runtime state unchanged or
performs the exact declared rollback transaction.

## Validation

Validation proves:

- every required catalog role resolves exactly once or has an accepted fallback;
- every native Animation Sequence targets the declared Skeleton;
- exact timing, sample count, and duration survive conversion and read-back;
- normalized tracks match an accepted profile;
- root-motion and in-place policy match movement behavior;
- locomotion sync markers are ordered and complete;
- turn direction and angular intent are correct;
- jump, fall, impact, landing, and recovery markers correlate with physical
  observations;
- dialogue gestures use compatible Slots and do not replace dialogue authority;
- action active windows and cancellation behave deterministically;
- every vehicle-handoff variant resolves through typed predicates;
- composed and phased handoffs produce equivalent terminal gameplay state;
- seat attachment, detachment, possession, input, camera, collision, and door
  state commit exactly once;
- cancellation rejects late markers and restores declared state;
- local-player and network ownership remain isolated;
- feature removal leaves zero owned resources; and
- public and packaged outputs contain no prohibited evidence.

## Tests

Required automated and review tests include:

- exact rational timing and interval read-back;
- animation-only scene binding to the accepted rig;
- unknown, duplicate, and missing-track rejection;
- full and reduced-track profile fixtures;
- duplicate and backup canonical-selection tests;
- pose classification and additive-base tests;
- locomotion loop and Sync Group tests;
- walk-to-run marker synchronization;
- clockwise and counterclockwise turn tests;
- impact, landing, and recovery correlation tests;
- dialogue Slot and locomotion-layer tests;
- action active-window and interruption tests;
- batch and configuration evidence non-execution tests;
- low, standard, and high vehicle-access selection tests;
- driver, passenger, side, seat, door, and no-door selection tests;
- separate-phase and composed-handoff parity tests;
- safe entry and exit clearance tests;
- cancellation at every handoff phase;
- stale marker, world, vehicle, seat, door, and asset-revision tests;
- split-screen isolation tests;
- multiplayer authority-adapter tests;
- feature removal and zero-resource tests;
- deterministic conversion snapshots; and
- confidentiality scans of generated output.

## Invariants

- Semantic identities, not source filenames or array positions, select
  animation.
- Animation-only evidence never defines a new rig or Skeleton.
- Every clip has exact timing, a compatible track profile, and an explicit root
  policy.
- A pose is not runtime animation unless its purpose is accepted explicitly.
- Historical batch and configuration files are evidence, never shipping
  executables or runtime data.
- Character Movement owns ordinary movement integration.
- Native Animation Blueprints own pose evaluation and blending.
- Gameplay services own actions, damage, dialogue, seats, doors, possession, and
  terminal results.
- Animation markers are revisioned observations, not commands with hidden
  authority.
- Vehicle entry and exit are transactions, not animation-completion side
  effects.
- Composed and phased choreographies produce the same accepted gameplay state.
- Quality changes presentation cost, not movement, handoff, or gameplay
  semantics.
- Local players and network authorities remain isolated.
- Stale playback, marker, loading, and transaction results cannot mutate current
  state.
- Public and packaged outputs contain no non-public evidence content.
