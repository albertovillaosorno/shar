# Game manifest as a completeness ledger

- Status: Accepted
- Decision date: 2026-07-12
- Scope: User-supplied installation validation

## Context

The pipeline needs to establish that required local input families are present
without publishing confidential original names.

## Decision

A deterministic, public-safe manifest acts as a minimum completeness ledger. It
uses obfuscated location identities, normalized type identities, and minimum
counts. A positive minimum is required base-installation evidence. A zero
minimum records an optional coordinate, including edition- or language-specific
content, without requiring that content to be present.

The ledger does not
enumerate original file names or distribute content.

Validation fails when required evidence is absent, malformed, duplicated
ambiguously, or inconsistent with the ledger. Optional coordinates do not fail
validation when absent.

## Consequences

- English base-installation completeness is checked before expensive conversion.
- Optional language and edition coordinates remain visible without becoming
  mandatory.
- Public history does not disclose unnecessary source names.
- The ledger proves minimum shape, not ownership or legal entitlement.

## Rejected alternatives

- Publishing a full source-file inventory.
- Accepting any installation shape and failing later.
- Local absolute paths as identity.
