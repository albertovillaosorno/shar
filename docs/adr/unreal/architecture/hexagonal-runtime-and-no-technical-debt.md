# Minimal hexagonal native runtime

- Status: Accepted
- Decision date: 2026-07-12
- Scope: Native runtime architecture

## Context

Gameplay domains must remain independent from engine adapters, editor state, and
infrastructure concerns. Temporary cross-layer shortcuts would become permanent
runtime authority and make parity behavior difficult to test in isolation.

## Decision

The Unreal runtime follows minimal hexagonal dependency direction and rejects
temporary architecture debt, duplicated policy, hidden editor state, and adapter
concerns inside gameplay domains.

## Consequences

- Gameplay policy and invariants remain independent of engine, editor, protocol,
  and storage adapters.
- Adapter or plugin changes cannot redefine domain behavior.
- Temporary dependency inversions, duplicated policy, and hidden editor state
  are treated as architecture failures rather than deferred cleanup.

## Rejected alternatives

- Placing gameplay policy inside engine or protocol adapters.
- Maintaining duplicate authoritative logic across native code and visual
  scripting.
- Accepting temporary architecture debt as a production shortcut.
