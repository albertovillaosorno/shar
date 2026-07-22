# Orchestration, command-line, and language boundaries

- Status: Accepted
- Decision date: 2026-07-12
- Scope: Pipeline ownership, implementation languages, and process coordination

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

Every normal pipeline command participates in a cooperative Rust run registry.
The default execution mode is exclusive: a create-new local lease blocks a
second pipeline command while any non-stale run is active. A blocked command
reports the active run identifier, process identifier, command, optional label,
lifecycle
state, current stage, item progress, elapsed time, and estimated remaining time
when the stage exposes enough measured work. Unknown progress or ETA remains
explicitly `unknown`; the registry does not invent timing evidence.

`pipeline active` and its `pipeline --active` alias inspect the registry without
acquiring a run lease. `pipeline cancel <run-id>` requests cooperative
cancellation, and `pipeline cancel all` requests it for every active run. The
running process observes cancellation at safe stage or work-item boundaries so
one atomic archive, package, or output transaction is never interrupted midway.
The request may therefore allow the current atomic unit to finish before the
process exits.

Operators may acknowledge intentional parallel work with `--allow-concurrent`.
This is a scoped concurrency mode rather than a global mutex bypass: every
process still receives its own run identifier, heartbeat, state record, and
cancellation route. Unless the caller explicitly selects a log path, concurrent
runs use independent logs under `logs/pipeline/runs/<run-id>.jsonl`. The
optional `--run-label` value supplies a portable display identity without
replacing the stable run identifier.

Registry state is derived and ignored under `temp/pipeline/runtime/`. Active
processes refresh a heartbeat once per second. Records and abandoned exclusive
leases without a live heartbeat are eligible for deterministic cleanup after two
minutes, preventing a crashed process from permanently blocking later commands.
No registry file is repository authority or a substitute for output validation.

## Consequences

- Language boundaries follow ownership rather than convenience.
- Process and stream behavior remain testable through ports.
- Default command execution is mutually exclusive and fail-closed.
- Intentional concurrency remains observable, individually labelled, and
  independently cancellable.
- Cancellation is cooperative and preserves atomic artifact boundaries.
- ETA is reported only from measured completed and total work.
- Manual editor assembly is not a production pipeline step.

## Rejected alternatives

- Python as the canonical asset conversion engine.
- Blueprint authority for core runtime behavior.
- Duplicated command and filesystem policy in every capability.
- An opaque operating-system mutex with no inspectable progress or recovery.
- A global `--ignore-mutex` switch that disables ownership and cancellation
  tracking for concurrent processes.
- Force-killing a process in the middle of an atomic output transaction.
