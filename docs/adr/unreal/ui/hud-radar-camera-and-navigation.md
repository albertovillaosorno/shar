# HUD, radar, camera, and navigation parity

- Status: Accepted
- Decision date: 2026-07-13
- Scope: Player-facing runtime systems

## Context

HUD, radar, camera, route guidance, and navigation jointly shape player-facing
state and spatial feedback. Treating them as incidental widgets would omit
scaling, timing, placement, input, safe-area, and gameplay behavior from parity
ownership.

The runtime now targets desktop and Android packages across five graphics
presets. Presentation and input adaptation therefore need one semantic contract
rather than platform-specific gameplay implementations.

## Decision

HUD, radar, camera, route guidance, and navigation are independently authored
native domains that preserve observable placement, scaling, state, timing, and
gameplay behavior across all supported graphics presets, presentation shapes,
platforms, and input adapters.

Keyboard and mouse, gamepad, and Android touch controls map to the same semantic
player actions. Android provides native touch interaction for every action
needed
to complete the game. A connected gamepad may replace touch presentation, but it
does not select different gameplay behavior.

## Consequences

- HUD, radar, camera, route guidance, and navigation remain separate native
  domains with explicit state and integration contracts.
- Placement and scaling are verified from Low through Ultra across supported
  aspect ratios, display densities, window modes, mobile safe areas, and display
  cutouts instead of one fixed viewport.
- Input adapters publish the same semantic actions and cannot create
  platform-specific mission, vehicle, camera, or navigation rules.
- Touch controls remain presentation and input concerns; they do not alter
  simulation, timing, progression, saves, or package identities.
- Gameplay-relevant indicators remain visible and readable at every supported
  quality preset and platform presentation.
- A change in one domain cannot silently redefine the state or behavior of the
  others.

## Rejected alternatives

- Combining all HUD, radar, camera, navigation, and input behavior in one
  monolithic controller.
- Hard-coding presentation for one resolution, graphics preset, aspect ratio, or
  input device.
- Treating Android as gamepad-only or maintaining separate mobile gameplay
  actions.
- Treating screenshot similarity as sufficient gameplay, input, and state
  evidence.
