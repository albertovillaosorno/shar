# Algorithm Function

## Purpose

Authors and replays generic deterministic source-bound reconstruction algorithms.

## Ownership

Owns mechanism-only source admission, protected target serialization, and the
`algorithm` CLI. Product-specific source identities and release thresholds stay
outside this boundary.

## Prohibitions

Does not own format- or product-specific reconstruction policy.
It never writes into caller source inputs and never emits plaintext protected
target bytes into a public algorithm document.

## Navigation

- `composition`
- `domain`
- `settings.json`
