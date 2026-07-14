# Eleven-phase remake delivery roadmap

- Status: Accepted
- Decision date: 2026-07-12
- Scope: Primary project delivery sequence

## Context

The project needs one durable dependency order from lawful local input to a
playable native reimplementation. That order must prevent downstream runtime or
packaging work from being treated as complete before its evidence and conversion
preconditions are satisfied, while excluding unrelated online products.

## Decision

The repository adopts the following eleven-phase dependency sequence. The term
`roadmap` denotes ordering only; it does not promise dates, release milestones,
or completion status.

1. decode required source evidence;
1. generate the minor-unit manifest;
1. classify deterministic packages;
1. generate first-principles binary FBX artifacts;
1. establish native Unreal MCP terminal control;
1. convert normalized evidence into native Unreal assets;
1. implement the complete native runtime;
1. verify the Low, Medium, High, Epic, and Ultra graphics presets;
1. add local drop-in mods and AI skills;
1. package the complete local path to validated native platform builds; and
1. optimize, verify, document, and close the bounded implementation sequence.

The roadmap excludes a hosted platform, marketplace, social layer, server
browser, general launcher, connected sandbox, and multiplayer product.

## Consequences

- A later phase cannot claim completion while an earlier dependency remains
  unverified or failed.
- Phase labels describe dependency order, not publication, release, or schedule
  commitments.
- Completion is finite and does not imply perpetual feature development.
- Scope additions require a new decision.

## Rejected alternatives

- Runtime-first development without sound conversion evidence.
- Combining the remake with an online platform or general editor.
