# Blueprint-to-C++ authority taxonomy

- Status: Accepted
- Decision date: 2026-07-12
- Scope: Runtime authoring authority

## Context

C++ and Blueprint can both express runtime behavior, which creates an ownership
risk when the same rule exists in both. The project needs a clear authority
split for gameplay logic, composition, inspection, and content authoring.

## Decision

Gameplay rules, invariants, and performance-critical behavior belong in C++.
Blueprints remain for bounded composition, inspection, and content authoring and
are retired when they duplicate authoritative logic.

## Consequences

- Gameplay rules, invariants, and performance-critical behavior have one native
  code authority.
- Blueprints remain available for bounded composition, inspection, and content
  authoring without redefining gameplay contracts.
- Blueprint logic that duplicates authoritative native behavior is retired.

## Rejected alternatives

- Implementing all gameplay authority in Blueprints.
- Prohibiting Blueprints from every bounded authoring and composition role.
- Maintaining the same gameplay rule independently in native code and a
  Blueprint graph.
