# Normalized language interchange

- Status: Active
- Last reviewed: 2026-07-13

## Governing decision

- [Voice and language mod packages](../../adr/modding/voice-language-modding-suite.md)

## Purpose

This specification explains how decoded localization evidence becomes
deterministic repository-owned interchange data.

## Repository model

The localization adapter emits typed records containing locale identity, entry
identity, normalized text or media reference, ordering, and provenance. The
interchange form supports review and native planning but is not final runtime
authority.

## Invariants

- Locale and entry identities are explicit and stable.
- Ordering is deterministic for equivalent evidence.
- Unsupported records remain visible rather than becoming empty success.

## Failure behavior

- Duplicate identities, malformed text, contradictory locale evidence, and
  missing provenance reject the affected record set.

## Verification

- Parser tests cover ordering, duplicate rejection, and text normalization.
- Round-trip tests preserve supported values and provenance.
- Native planning tests consume the typed interchange contract.
