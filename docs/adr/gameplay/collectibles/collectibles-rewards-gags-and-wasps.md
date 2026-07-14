# Collector cards, coins, rewards, gags, and wasps

- Status: Accepted
- Decision date: 2026-07-12
- Scope: Gameplay collectible parity

## Context

Collector cards, coins, rewards, gags, and wasps expose distinct collection,
event, persistence, progression, currency, unlock, and completion behavior.
Treating them as decorative content or as one generic pickup family would omit
observable gameplay contracts and make parity impossible to prove.

## Decision

Collector cards, coins, rewards, gags, and wasps remain separate first-class
deterministic gameplay domains. Shared pickup presentation is permitted, but
identity, event meaning, state transitions, persistence, spending and unlock
semantics, completion rules, and parity tests remain family-specific.

## Consequences

- Every family has stable identities, state transitions, persistence, completion
  rules, and parity evidence.
- Card collection and gallery completion remain distinct from coin balance and
  spending, reward unlocks, gag completion, and wasp destruction.
- Save, reward, and progression behavior can reference explicit domain state
  instead of inferring completion from presentation.
- Missing support for any required family remains a visible parity gap.

## Rejected alternatives

- Collapsing cards, coins, rewards, gags, and wasps into one generic pickup
  model.
- Inferring completion from visual presence or editor placement alone.
- Omitting less common collectible behavior from parity scope.
