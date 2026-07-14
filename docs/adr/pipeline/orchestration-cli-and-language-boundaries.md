# Orchestration, command-line, and language boundaries

- Status: Accepted
- Decision date: 2026-07-12
- Scope: Pipeline ownership and implementation languages

## Context

The project combines deterministic conversion, engine integration, native
runtime behavior, and operator-facing commands.

## Decision

Rust owns deterministic orchestration, parsing, manifests, transforms, and
validation. C++ owns the native Unreal runtime. Python is limited to
repository-owned protocol clients and integration tooling and does not become an
asset-authority layer.

Command-line entry points delegate to application services and share only stable
argument, stream, and filesystem mechanisms. JSON is a review and interchange
representation, not final runtime authority when a native asset exists.
Blueprints remain compatible for inspection and bounded authoring while C++ and
validated data remain authoritative.

## Consequences

- Language boundaries follow ownership rather than convenience.
- Process and stream behavior remain testable through ports.
- Manual editor assembly is not a production pipeline step.

## Rejected alternatives

- Python as the canonical asset conversion engine.
- Blueprint authority for core runtime behavior.
- Duplicated command and filesystem policy in every capability.
