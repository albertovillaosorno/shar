# Runtime parity boundary

- Status: Accepted
- Decision date: 2026-07-13
- Scope: Faithful runtime behavior across supported targets

## Context

A faithful reimplementation must define parity through observable behavior
rather than source-code similarity. Without that boundary, proprietary
implementation details can be mistaken for requirements and missing gameplay
contracts can be ignored.

Multiple operating systems, processor architectures, graphics presets, and input
adapters must not create multiple interpretations of the game.

## Decision

The independently authored runtime preserves observable mission, world, vehicle,
character, collectible, UI, camera, input, save, and progression contracts
without importing proprietary runtime code.

Those contracts are platform-neutral. Windows, Linux, macOS, Android, x64,
ARM64, Low, Medium, High, Epic, Ultra, keyboard and mouse, gamepad, and touch
adapters may change native presentation or integration only. They do not change
simulation, timing, mission meaning, progression, save identity, package
identity, or mod semantics.

## Consequences

- Parity acceptance is based on observable mission, world, vehicle, character,
  collectible, UI, camera, input, save, and progression contracts.
- Every claimed platform, architecture, graphics preset, and input adapter maps
  to the same domain contracts.
- Native implementation structure may differ from the original when those
  observable contracts remain satisfied.
- Platform-specific lifecycle, storage, rendering, and input behavior remains in
  adapters and cannot redefine gameplay.
- A lower graphics preset may reduce visual cost only within its documented
  quality contract; it cannot remove gameplay-relevant information.
- Proprietary runtime code is not an input to the independently authored
  runtime.

## Rejected alternatives

- Measuring parity by source-code or internal architecture similarity.
- Importing proprietary runtime code to accelerate implementation.
- Treating platform, architecture, preset, or input differences as permission to
  fork gameplay behavior.
- Declaring parity from visual resemblance without gameplay, input, save, and
  progression evidence.
