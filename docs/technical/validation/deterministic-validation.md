# Deterministic validation

- Status: Active
- Last reviewed: 2026-07-13

## Governing decision

<!-- markdownlint-disable-next-line MD013 -->
- [Strict validation and linting](../../adr/engineering/quality/strict-validation-and-linting.md)

## Purpose

This specification explains how the canonical validator orders repository gates,
identifies reusable evidence, invalidates stale results, and reports one final
repository outcome.

## Repository model

One validator coordinates formatting, static analysis, compilation, tests,
architecture checks, documentation checks, provenance checks, and
confidentiality checks. Direct tool execution may assist diagnosis but is not
final evidence.

## Invariants

Diagnostics have stable ordering in deterministic mode. Successful cache entries
include relevant input, configuration, policy, environment, and toolchain
identity. Failed, interrupted, stale, or partial runs are never cached as
success.

## Failure behavior

Unavailable required tools, changed authority, stale cache identity, missing
decision targets, invalid documentation boundaries, private-data leakage, or any
underlying gate failure produce a non-success result.

## Verification

Validator self-tests, focused gate tests, no-cache runs, cache invalidation
tests, and full repository runs prove the contract.
