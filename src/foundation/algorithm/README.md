# Algorithm Function

## Purpose

Authors and replays generic deterministic source-bound reconstruction
algorithms.

## Ownership

Owns mechanism-only source admission, protected target serialization, and the
`algorithm` CLI. Product-specific source identities and release thresholds stay
outside this boundary.

## Prohibitions

Does not own format- or product-specific reconstruction policy.
It never writes into caller source inputs and never emits plaintext protected
target bytes into a public algorithm document.

Canonical v1 documents serialize protected ciphertext as deterministic
64-hex-character chunks so tracked plans remain reviewable by line-oriented
repository gates. Replay also accepts the earlier monolithic ciphertext string
representation and normalizes both forms to the same authenticated bytes.

## Navigation

- `composition`
- `domain`
- `composition/adapter-inbound/settings.json`
