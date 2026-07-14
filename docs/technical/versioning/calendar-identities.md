# Calendar identities

- Status: Active
- Last reviewed: 2026-07-13

## Governing decision

- [Calendar versioning, Conventional Commits, and no releases](../../adr/governance/versioning-commits-and-publication.md)

## Purpose

This specification explains how repository-owned components construct calendar
identities without a release process.

## Repository model

The identity uses `YY.M.V` with no leading zeroes. `YY` is the two-digit
calendar year, `M` is the calendar month, and `V` is the zero-based accepted
compatibility-snapshot sequence within that month. The current identity is
`26.7.0`; examples include `26.12.2` and `27.1.0`.

## Invariants

A calendar identity states the year, month, and accepted snapshot sequence. It
does not claim backward compatibility, stability, support duration, or release
status. Conventional Commit types never infer or increment the identity.

## Failure behavior

Malformed years or months, leading zeroes, negative sequences, non-increasing
monthly sequences, identity collisions, or attempts to derive identity from a
commit type are rejected.

## Verification

Tests cover component validation, monthly sequencing, collision rejection,
stable rendering, and separation from commit classification.
