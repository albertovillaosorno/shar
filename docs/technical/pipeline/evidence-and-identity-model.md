# Evidence and identity model

- Status: Active
- Last reviewed: 2026-07-13

## Governing decision

- [Extraction provenance and manifest linkage](../../adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md)

## Purpose

This specification explains how source evidence, minor units, packages,
transformations, and generated artifacts receive stable identity.

## Repository model

Source evidence receives public-safe identity from validated content and
normalized classification. Minor units group the smallest coherent evidence for
one capability. Packages group minor units with one conversion and runtime
responsibility. Transformation identity records the policy and tool behavior
that produced an artifact.

## Invariants

Identity never depends on a local storage route. Provenance records source
identities, package identity, transformation identity, capability claims, and
verification results. Confidential original names remain absent when
unnecessary.

## Failure behavior

Unknown origin, duplicate semantic identity, inconsistent capability claims, or
missing transformation evidence invalidates the artifact.

## Verification

Tests compare logical identity across reordered or relocated equivalent input
and reject collisions, omissions, and stale provenance.
