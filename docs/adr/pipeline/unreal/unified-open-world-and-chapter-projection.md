# Unified open world and chapter projection

- Status: Accepted
- Decision date: 2026-07-15
- Scope: Physical world ownership and chapter-driven projection

## Context

The product has one connected sandbox and seven narrative chapters. Shared
terrain, roads, districts, structures, interiors, landmarks, shortcuts, and
coordinates cannot change identity when the story advances.

Chapter progression still changes missions, collectibles, population,
interactions, hazards, bosses, unlocked routes, and presentation. Those changes
must project over one world without recreating isolated level states or a hidden
test state.

## Decision

The native runtime has exactly one persistent geographic World Partition world.
It owns canonical terrain families, roads, districts, structures, exterior
components, linked interiors, landmarks, spatial components, transforms,
connectors, and placement identities.

The player-facing world projection is composed from:

- persistent base geography;
- cumulative chapter-unlock state;
- current `mission` or `non_mission` state;
- the one active mission projection when present;
- discovered-map and terrain-gate state;
- persistent collectible, structure, boss-area, and shortcut state;
- current world-clock, weather, and Chapter 7 atmosphere; and
- validated mod overlays.

A chapter is not a map or world variant. It contributes unlock definitions and
mission availability. Completing a chapter adds state cumulatively and never
silently removes earlier terrain, collectibles, interiors, routes, or side
activities.

Mission-specific actors, routes, pickups, hazards, and scripted changes exist
only while their mission projection is active. Persistent mission results commit
to world state through explicit transactions.

Every geographic location has one stable identity and coordinate record.
Historic level or map labels remain import aliases only.

Every structure is separate from terrain and declares an interior capability:
none, linked, streamed, mission-only, or future slot. Windows and doors are
separate components. A breakable window may create an entry route only when the
structure has a valid interior and navigation contract.

Terrain families are connected by original or appropriately licensed bridges,
roads, tunnels, paths, zip lines, and transitions. Availability is controlled by
chapter and discovery gates rather than duplicate geography.

The 24-minute sunrise, day, sunset, and night cycle is active in the base world.
Chapter 7 layers persistent irradiated cloud, humidity, haze, hazard, and
horror over that clock without creating another world.

There is no `level_11_test` projection. Editor and automation tests use fixtures
that are excluded from the gameplay catalog, package manifests, saves,
achievements, and player-facing state.

## Consequences

- One geographic correction applies everywhere that location appears.
- Chapter progression adds unlocks instead of swapping the complete world state.
- Mission projection is temporary and explicit.
- Map discovery, terrain gates, interiors, bosses, and shortcuts use stable
  world identities.
- Burns' mansion, the museum, and the stadium can become permanent world
  expansions after accepted unlock transactions.
- Mods target semantic world, structure, interior, connector, and chapter fields
  rather than source map files.
- Dynamic time and Chapter 7 weather are ordinary product behavior.

## Rejected alternatives

- Seven unrelated maps or complete level-state swaps.
- One world with every mission projection active simultaneously.
- A campaign-visible test state.
- Structures inferred from terrain meshes or interior availability inferred from
  decorative windows.
- Geography identity derived from map filename, loaded actor, or active layer.
- Shortcuts that bypass mission, chapter, boss, or fairness gates.
