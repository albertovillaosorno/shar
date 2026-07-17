# Typed StateTree action sequences

- Status: Accepted
- Decision date: 2026-07-14
- Scope: Reusable character, vehicle, state-prop, animation, artificial-
  intelligence, and interaction action execution

## Context

Gameplay repeatedly needs bounded ordered actions: wait, publish a typed event,
move a character, orient or position it, snap to ground, play or hold animation,
change locomotion, open a vehicle door, jump, dodge, react, recover, or commit
an
interaction result. These actions need shared cancellation, resource ownership,
timeouts, verification, and recovery.

A bespoke sequencer with an inheritance hierarchy and raw pointers would create
another scheduler beside StateTree, Character Movement, animation montages,
vehicle state, and interaction transactions. Positional string registries also
make action identity depend on array order rather than validated data.

## Decision

SHAR uses one bounded library of native C++ StateTree tasks, evaluators, and
conditions for reusable gameplay actions. Generated definitions select a
canonical StateTree template and bind typed parameters. They cannot inject code,
raw object paths, unregistered event names, or arbitrary Blueprint logic.

Each sequential phase is a StateTree state. Ordering uses explicit transitions;
parallel tasks are permitted only when their declared resources do not conflict.
`USharActionResourceArbiter` grants exclusive or shared leases for movement,
root motion, animation slots, character control, vehicle doors, camera requests,
input suppression, and interaction reservations.

Every task returns a closed result: success, rejected, failed, timed out,
cancelled, or compensated. A task must release every resource lease and restore
presentation state on completion, failure, cancellation, actor destruction,
world teardown, or StateTree replacement.

The action catalog is keyed by stable canonical identity and a closed execution
kind. Duplicate identities, missing executors, incompatible resource claims,
invalid timeouts, and unresolved assets fail validation before runtime. Array
position and case-insensitive string hashing are not identity authority.

Character movement uses Character Movement and authored navigation or
interaction slots. Animation uses montages, sections, slots, root motion, and
notifies. Vehicle actions delegate to the vehicle application port. State-prop
tasks may request typed transitions or await correlated state and marker
observations, but cannot treat animation completion as durable state authority.
Artificial-intelligence and non-player-character tasks publish movement, path,
reaction, and interaction intent through current character and world revisions.
Domain effects and save changes remain outside the task and commit only through
typed application ports.

Queued media, Level Sequences, type-on text, dialogue-facing animation, camera
playback, and visual transition graphs execute through the presentation playback
subsystem. A StateTree task may request that playback and await one correlated
terminal result, but the presentation subsystem is an adapter rather than a
second gameplay scheduler.

A visual transition may satisfy a presentation barrier. It cannot switch
application mode, resume simulation, change input authority, publish gameplay
events, or commit domain state except through the owning typed application port
and accepted action-sequence transition.

## Consequences

- Mission, interaction, vehicle, and ambient systems share one action vocabulary
  without sharing progression storage.
- StateTree remains the sole hierarchical scheduler for gameplay sequences.
- Resource leases prevent two tasks from driving the same movement, animation,
  vehicle door, or interaction reservation concurrently.
- Animation completion is proven by montage state or a required notify, not by a
  guessed duration.
- A one-shot interaction becomes unavailable only after verified completion, not
  when input is first pressed.
- Automatic doors use occupancy and explicit open, closing, closed, and blocked
  states; a raw entrant counter cannot become negative or strand the door.
- Cancellation and world streaming cannot leave input, locomotion, root motion,
  sounds, doors, or interaction slots locked.
- Blueprint may configure reflected parameters and presentation assets but
  cannot
  add unregistered action kinds or domain mutations.

## Rejected alternatives

- Recreating a custom task scheduler beside StateTree.
- One unique Blueprint graph or StateTree schema for each interaction.
- Action identity derived from array order or free-form strings.
- Fixed-duration animation completion without montage or notify verification.
- Direct domain mutation from animation, collision, or vehicle-door callbacks.
- Tasks that acquire movement, animation, or interaction ownership without a
  cancellation-safe lease.
