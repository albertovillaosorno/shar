# UI parity boundary

- Status: Accepted
- Decision date: 2026-07-13
- Scope: Native user-interface behavior

## Context

User-interface parity includes layout, interaction, state, timing, localization,
accessibility, safe areas, display density, and input adaptation—not only visual
resemblance. Copied widgets or manual editor assembly cannot provide an
independently authored, testable UI contract.

Desktop and Android builds must expose the same game state through different
native presentation and input adapters. Graphics quality must not create
separate UI behavior.

## Decision

Native UI preserves observable layout, interaction, state, timing, localization,
and accessibility contracts through deterministic data and code rather than
copied source-engine widgets.

UI actions use one semantic input model. Keyboard and mouse, gamepad, and Android
touch adapters map to that model without changing gameplay meaning. Layout
responds to supported aspect ratios, resolutions, display densities, window
modes, safe areas, and display cutouts.

## Consequences

- Layout, interaction, state, timing, localization, accessibility, and input
  semantics are explicit observable UI contracts.
- Native widgets and code may differ internally from the source implementation
  while preserving those contracts.
- Low, Medium, High, Epic, and Ultra share one UI state and interaction model.
- Android touch UI exposes every action required to complete the game and may be
  replaced by a connected gamepad without changing action semantics.
- Presentation adapters consume the same deterministic UI state instead of
  duplicating menus or gameplay rules by platform.
- Verification covers meaningful states, input devices, safe areas, densities,
  aspect ratios, and presentation shapes, not only one static capture.

## Rejected alternatives

- Copying proprietary source-engine widgets or UI implementation.
- Declaring parity from screenshots without interaction, input, and state
  evidence.
- Repairing each resolution, platform, input device, or locale through
  undocumented manual layout edits.
- Maintaining separate Android menu logic or gameplay actions.
- Allowing a graphics preset to hide, remove, or redefine required UI behavior.
