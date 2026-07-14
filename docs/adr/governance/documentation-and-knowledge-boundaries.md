# Decision and technical knowledge boundaries

- Status: Accepted
- Decision date: 2026-07-12
- Scope: Repository documentation authority

## Context

The repository previously mixed decisions, implementation explanations,
workstation preferences, external format research, and legal evidence.

## Decision

ADRs record repository-impacting decisions only. They do not record user
workstation choices, commands, implementation tutorials, or concrete repository
paths.

Technical specifications explain only how repository-owned code works. They do
not make decisions, include concrete repository paths, or explain proprietary
external formats. External evidence belongs in bibliography, research, or legal
records.

Every active technical specification names at least one current governing ADR.
That link provides decision traceability without restating or extending the
decision inside implementation documentation.

Bibliography records use `Evidence recorded` when the available evidence,
repository relationship, and known limitations have been documented. That status
closes the record as a bounded evidence account; it does not convert partly
verified or unknown propositions into verified facts. The separate
evidence-status field remains authoritative for proposition-level confidence.

The public overview is a summary. The ADR catalog is decision authority and the
technical catalog is implementation-knowledge authority. A document that no
longer represents a decision is moved to the technical catalog or removed, and
all repository references are updated to the current owning record.

## Consequences

- Every durable choice has one owning ADR.
- Technical prose cannot silently create policy.
- Technical behavior remains traceable to its current owning decision.
- Workstation and editor preferences are not architecture.
- Missing facts remain unknown instead of being inferred.
- A complete bibliography record can retain explicit uncertainty without
  remaining in an open research state.

## Rejected alternatives

- Treating every technical note as an ADR.
- Duplicating one contract across several documentation surfaces.
