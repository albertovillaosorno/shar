# Repository knowledge model

- Status: Active
- Last reviewed: 2026-07-13

## Governing decision

<!-- markdownlint-disable-next-line MD013 -->
- [Decision and technical knowledge boundaries](../../adr/governance/documentation-and-knowledge-boundaries.md)

## Purpose

This specification explains how an agent distinguishes authority, implementation
knowledge, operational guidance, and supporting evidence.

## Repository model

The public overview summarizes purpose and status. Decision records own durable
choices. Technical specifications explain repository-owned behavior. Skills
provide executable task guidance. Bibliography, research, and legal records
provide supporting evidence and cannot override repository authority.

## Invariants

Every normative choice has one owning decision. Every implementation explanation
has one owning specification. Missing evidence remains unknown. Supporting
evidence never silently becomes policy.

## Failure behavior

Conflicting authority, duplicated normative rules, missing ownership, or
inferred facts invalidate the documentation change.

## Verification

Reviewers map each normative statement to its owning decision and each
implementation claim to current repository behavior.
