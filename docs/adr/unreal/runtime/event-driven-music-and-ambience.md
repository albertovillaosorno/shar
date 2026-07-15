# Event-driven music and ambience

- Status: Accepted
- Decision date: 2026-07-14
- Scope: Level, mission, race, interior, and frontend score state

## Context

The score changes with level identity, protagonist motif, mission phase, race
state, interiors, pause, loading, cinematics, notoriety, completion, and special
world events. A playlist or widget-owned sound selection cannot preserve those
transitions, loop boundaries, resume positions, or deterministic event timing.

Track display names are not reliable runtime identity. Several musical
compositions are reused by different missions, races, and level states, while
one
mission may transition between more than one composition or layer.

## Decision

The runtime uses one repository-owned `USharMusicSubsystem` and generated music
state definitions. Gameplay systems emit typed semantic events. The subsystem
resolves them against the active level audio profile, mission or race identity,
state priority, and current music revision.

Quartz owns sample-aligned clocks, quantized transitions, and resume boundaries.
MetaSound or repository-owned native audio graphs own stems, layers,
transitions,
and parameterized presentation. Canonical composition, state, cue, loop, and
transition identities remain independent of Unreal object paths and descriptive
track titles.

Every state transition declares its trigger, source state, destination state,
quantization rule, fade or crossfade, allowed interruption, loop behavior,
resume behavior, and fallback. Audio completion alone never advances gameplay.

## Consequences

- Level driving, mission, race, interior, frontend, cinematic, pause, loading,
  notoriety, and completion states share one typed event matrix.
- A reused composition has one audio identity and multiple explicit state
  bindings.
- Mission start, drama, warning, win, lose, vehicle exit, and retry are separate
  semantic transitions.
- Race start, win, lose, and vehicle exit do not reuse mission state implicitly.
- Pause, movie, focus loss, and lifecycle events preserve or reset position only
  according to the active state policy.
- Presentation names and fan titles never become runtime identity.
- Missing optional layers fall back deterministically; missing required music
  fails the owning level or mission activation before play.
- Graphics presets cannot change music state, timing, or content membership.
- Android and desktop use target-native cooked audio while preserving the same
  logical event and transition graph.

## Rejected alternatives

- One looping track per map with ad hoc Blueprint switches.
- Selecting music from filenames, folders, or display titles.
- Advancing missions because an audio component stopped.
- Maintaining separate race, mission, mobile, and desktop music logic.
- Unquantized transitions whose timing depends on frame rate.
- Restarting every composition after pause, interior travel, or short focus
  loss.
