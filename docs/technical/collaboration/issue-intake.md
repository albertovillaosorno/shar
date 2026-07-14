# Issue intake

- Status: Active
- Last reviewed: 2026-07-13

## Governing decision

- [Issue-only collaboration](../../adr/governance/issue-only-collaboration.md)

## Purpose

This specification explains the supported public intake for defects, evidence,
and requests.

## Repository model

An issue records observed behavior, expected behavior, reproducible evidence,
relevant environment facts, and confidentiality constraints. Authorized
maintainers or agents reproduce the report, identify the owning contract,
implement a bounded change, and record validation evidence.

## Invariants

An issue is evidence or a request, not write authorization. Pull requests are
not reviewed or merged. Discussion and closure remain on the issue while
downstream forks remain permitted by the license.

## Failure behavior

Reports lacking reproducible evidence, violating confidentiality, requesting
unsupported scope, or conflicting with repository authority are rejected or
closed with a reason.

## Verification

Governance review verifies that public guidance offers issue intake only and
that repository automation does not require pull requests.
