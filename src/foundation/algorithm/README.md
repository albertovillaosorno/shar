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

Direct-file source inputs may optionally use an authenticated
`offset-mask-set-v1` projection. Authoring supplies compact canonical source
bytes plus one or more positional masks. The plan stores only those masks and
never stores a source hash for that projected record. Replay applies each mask
to the caller's actual file, derives the source key from the selected bytes, and
writes nothing unless one complete projected candidate authenticates every
protected target.

Distinct known layouts may use distinct alternatives as long
as they select the same common-byte count. Raw, non-projected source records
retain their existing exact SHA-256 binding for backward compatibility.

## Navigation

- `composition`
- `domain`
- `composition/adapter-inbound/settings.json`
