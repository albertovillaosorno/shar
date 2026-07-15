# Unreal configuration and asset validation

- Status: Active
- Last reviewed: 2026-07-13

## Governing decision

<!-- markdownlint-disable-next-line MD013 -->
- [C++-primary and Blueprint-compatible Unreal project](../../adr/unreal/project/cpp-primary-blueprint-compatible-project.md)

## Purpose

This specification explains how repository-owned validation inspects supported
project configuration and generated asset evidence.

## Repository model

Typed parsers read repository-owned configuration and public generated evidence
needed for validation. They normalize values into repository domains and leave
proprietary engine internals and native asset authority to Unreal.

## Invariants

- Only supported repository-owned configuration is parsed directly.
- Parsed identities and values are deterministic.
- Native engine state remains authoritative for engine-owned objects.

## Failure behavior

- Malformed configuration, unsupported schema, ambiguous identity, stale
  evidence, and attempts to parse proprietary internals fail closed.

## Verification

- Parser tests cover valid, malformed, missing, and unknown values.
- Planning tests consume normalized configuration only.
- Native read-back verifies engine-owned results after application.
