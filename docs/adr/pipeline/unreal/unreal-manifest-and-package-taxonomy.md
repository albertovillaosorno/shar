# Unreal manifest and package taxonomy

- Status: Accepted
- Decision date: 2026-07-12
- Scope: Native asset planning

## Context

Normalized packages need stable native targets without exposing confidential
source organization or embedding editor state in conversion policy.

## Decision

A public-safe manifest maps opaque package identities and capabilities to native
asset plans, dependencies, import behavior, and provenance. Target identity is
derived from package semantics and deterministic policy, never local paths or
manual editor placement.

The manifest describes intended native assets; it does not control the editor or
carry proprietary payloads.

## Consequences

- Native planning is reviewed before editor mutation.
- Repeated planning produces stable logical targets.
- Editor automation consumes plans rather than inventing taxonomy.

## Rejected alternatives

- Native identity derived from source directories.
- Mutable editor object references as package authority.
- Combined planning and editor control.
