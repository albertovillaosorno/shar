# Deterministic validation

- Status: Active
- Last reviewed: 2026-07-13

## Governing decision

<!-- markdownlint-disable-next-line MD013 -->
- [Strict validation and
  linting](../../adr/engineering/quality/strict-validation-and-linting.md)

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

## Local Jig installation

SHAR uses a source-linked Jig development installation. Machine-local
`.dependencies/jig/source` resolves to the sibling Jig source checkout; the
ignored `.dependencies/jig/bin/` launchers and `.dependencies/jig/runtime/`
artifacts are refreshed from that source. Tracked repository policy lives only
under `.jig/` and must not vendor or copy Jig source into SHAR.

Shared validation-tool paths in `.jig/jig.toml` intentionally resolve through
`.dependencies/jig/source/` so Jig's reviewed local toolchain remains one
authority. Repository-specific pytest and Ruff executables live in SHAR's own
ignored `.dependencies/python/` virtual environment. Materialize or repair that
environment with `python tools/validation/python_dependencies.py --replace`;
the script itself uses the exact CPython 3.14.6 source-linked through Jig by
default and never installs global packages.

After installing or updating the local Jig checkout, refresh owned integrations
with `jig integrations refresh --root .`. Final repository evidence is produced
with `jig validate --fail-fast --root .`. The commit hook is a Jig-owned local
projection and must not gain a repository-authored fallback validator.

## CI posture

No CI service is canonical today. Local Jig validation is sufficient for the
current single-maintainer development flow, and adding a second execution surface
would create maintenance cost without new acceptance evidence. CI remains a
future option if public contribution or release automation creates a concrete
need; if added, it must invoke the same tracked Jig policy rather than define a
parallel validation contract.

## Verification

Validator self-tests, focused gate tests, no-cache runs, cache invalidation
tests, and full repository runs prove the contract. Architecture tests also pin
shared tool paths to the source-linked Jig boundary.
