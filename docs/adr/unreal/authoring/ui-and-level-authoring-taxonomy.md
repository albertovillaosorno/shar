# UI and level authoring taxonomy

- Status: Accepted
- Decision date: 2026-07-12
- Scope: Native authoring ownership

## Context

UI and level work can be expressed as imported data, native code, Blueprint
composition, or editor automation. Without a taxonomy, manual assembly becomes
an undocumented fifth authority and ownership drifts between mechanisms.

## Decision

UI and level changes are classified as deterministic data import, native code
construction, bounded Blueprint composition, or explicit editor automation. No
class of work relies on undocumented manual assembly.

## Consequences

- Every UI or level change has one declared authoring class and ownership
  boundary.
- Deterministic import, native code, bounded Blueprint composition, and editor
  automation are selected by responsibility rather than convenience.
- Production state can be reconstructed without undocumented manual assembly.

## Rejected alternatives

- Using one authoring mechanism for every UI and level responsibility.
- Treating manual editor assembly as an unrecorded fifth category.
- Moving authoritative gameplay policy into content-authoring graphs.
