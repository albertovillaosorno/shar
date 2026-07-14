# C++-primary and Blueprint-compatible Unreal project

- Status: Accepted
- Decision date: 2026-07-12
- Scope: Native runtime authority

## Context

C++ and Blueprint can both become de facto runtime authorities when ownership is
not explicit. Duplicating invariants across code, graphs, and editor state would
create conflicting behavior and make performance-critical systems harder to
test.

## Decision

C++ owns runtime behavior, invariants, and performance-critical systems.
Blueprints remain compatible for inspection, composition, and bounded content
authoring but do not replace authoritative C++ or validated data.

## Consequences

- Runtime behavior, invariants, and performance-critical systems have one C++
  authority.
- Blueprints remain available for inspection, composition, and bounded content
  authoring without becoming a second gameplay authority.
- Validated data and C++ contracts can be tested independently of Blueprint
  presentation and editor convenience.

## Rejected alternatives

- Making Blueprint graphs the authoritative runtime implementation.
- Prohibiting all Blueprint composition and inspection.
- Maintaining the same invariant independently in C++, Blueprint, and ad hoc
  editor state.
