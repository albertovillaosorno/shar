# Faithful seven-chapter open-world scope

- Status: Accepted
- Decision date: 2026-07-15
- Scope: Canonical campaign and world-delivery boundary

## Context

Parity and delivery need a finite canonical scope. The product preserves
seven ordered story chapters and their original mission sequences, but the
player lives in one connected sandbox rather than seven isolated levels.

A separate test level would create a false campaign identity and duplicate
world, lighting, streaming, progression, and save assumptions.
Development validation belongs in ordinary fixtures and automation maps that
never enter
the product catalog.

## Decision

The canonical base-game scope is:

- one connected persistent geographic world;
- seven ordered narrative chapters;
- exactly two player-facing gameplay states, `mission` and `non_mission`;
- cumulative chapter unlocks and collectible activation;
- one mandatory 24-minute sunrise, day, sunset, and night cycle; and
- no Level 11, test level, or campaign-visible development state.

Historic level identifiers remain source aliases for conversion and evidence
only. Player-facing progression, menus, achievements, saves, and transitions use
chapter identities.

Additional campaigns, worlds, or challenge spaces are optional mod or future
scope. They cannot substitute for incomplete base chapters, missions, connected
geography, sandbox systems, or progression.

## Consequences

- Delivery evidence covers all seven chapters in the connected sandbox.
- Dynamic time, weather, map discovery, terrain gates, and open-world
  persistence are base-game requirements, not test-only experiments.
- Editor and automation fixtures are excluded from campaign manifests and saves.
- Chapter 7 may alter weather, visibility, hazards, and ambience without
  becoming another world or disabling its underlying clock.
- New connectors, interiors, boss areas, and side activities expand the same
  canonical world.

## Rejected alternatives

- Seven isolated player-facing maps.
- A hidden or visible `level_11_test` state.
- Fixed campaign lighting with dynamic time reserved for development.
- Additional worlds counted as substitutes for missing base-game scope.
- Player-facing progression inferred from source map or level filenames.
