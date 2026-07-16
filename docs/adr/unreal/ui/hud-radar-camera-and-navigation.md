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
needed to complete the game. A connected gamepad may replace touch presentation,
but it does not select different gameplay behavior.

`USharInGameUiSubsystem`, a `UGameInstanceSubsystem`, owns shared in-game screen
routing, overlay definitions, pause policy, and fade, iris, or letterbox leases.
`USharPlayerHudSubsystem`, a `ULocalPlayerSubsystem`, owns each local player's
HUD projection, focus, input method, safe area, and split-screen presentation.

C++ UMG viewmodels publish immutable mission, timer, race, vehicle, notoriety,
currency, action, radar, and tutorial state through field notifications. Common
UI widgets own layout, activation, focus, animation, styles, and semantic action
presentation only. Gameplay services remain the authorities for every projected
value and command result.

HUD overlays and blocking transitions are registered data with stable
identities, source revisions, accessibility profiles, retained asset leases,
timeout, and cancellation behavior. A stale gameplay observation or animation
callback cannot mutate the accepted HUD or complete a replacement transition.

Short-lived card, currency, countdown, notoriety, mission, item, and target
feedback uses bounded per-player or explicitly shared cue channels. The cue
scheduler consumes typed accepted observations, applies deterministic priority,
coalescing, exclusion, and accessibility policy, and returns one terminal
result.
A visual countdown cannot start gameplay or grant input authority.

Numeric formatting, text scrolling, sliders, color modulation, transforms, and
radar icon projection are pure reusable presentation primitives. They evaluate
from immutable source state and cannot become mission, progression, economy,
input, race, or navigation authority.

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
