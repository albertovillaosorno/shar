# Unified geographic world and level-state projection

- Status: Accepted
- Decision date: 2026-07-14
- Scope: Physical world ownership and campaign-state composition

## Context

The seven campaign levels preserve distinct protagonists, missions, progression,
collectibles, traffic, interiors, props, lighting, audio, and presentation, but
those distinctions do not require separate physical copies of shared geography.
Maintaining several base-world authorities would make a location's coordinates,
building identity, road topology, mission anchors, and mod edits depend on the
selected campaign level.

The project also needs a bounded environment for testing dynamic world behavior
without adding another campaign level or changing parity completion.

## Decision

The native runtime has exactly one persistent geographic World Partition world.
It owns the canonical terrain, roads, districts, buildings, exteriors, linked
interiors, landmarks, spatial components, transforms, geographic taxonomy, and
placement identities for the complete map.

The seven canonical campaign levels remain separate immutable gameplay and
progression identities. Each level projects a deterministic state over the one
geographic world through data layers, catalog definitions, and state records.
A level state selects its protagonist, mission sequence, traffic, collectibles,
gags, characters, props, interior availability, damage state, audio,
presentation, and fixed time-of-day profile. Only the state required by the
active level is enabled; one world does not mean every level variant is active
simultaneously.

Every geographic location has one stable identity and coordinate record across
all level states. Level availability and state-specific behavior are attributes
of that location, not alternate copies of its geography. Missions and mods may
therefore target semantic locations, routes, districts, buildings, interiors,
or coordinates through one map-like geographic catalog.

A non-campaign development state named `level_11_test` uses the same geographic
world. It does not participate in campaign order, progression, completion,
unlocks, saves, or parity counts. It is the initial integration environment for
dynamic day-night cycling, lighting transitions, streaming, component placement,
mission experiments, and deterministic asset validation. Campaign levels use
their declared fixed time until a later decision changes that behavior.

## Consequences

- One geographic identity owns every physical location and placement.
- Seven campaign identities remain explicit without duplicating the physical
  map.
- A geographic or component correction is verified against every level state
  that exposes the affected area.
- Semantic mission editing and mods can operate on stable map locations rather
  than level-specific mesh or actor copies.
- Fixed campaign lighting remains reproducible while dynamic day-night behavior
  can be developed independently in the test state.
- World Partition streams the one physical world; data layers and definitions
  select campaign or test state. Neither streaming nor editor actor presence
  owns progression identity.

## Rejected alternatives

- Three persistent base-world families with separate geographic authorities.
- Seven unrelated maps with duplicated shared geography.
- One world with every campaign variant active simultaneously.
- Treating the development test state as an eighth campaign or parity level.
- Deriving location identity from a map filename, loaded actor, or active data
  layer.
