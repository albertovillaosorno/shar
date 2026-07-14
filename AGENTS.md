# SHAR Agent Guide

## Audience

This guide is primarily for AI agents helping users create, validate, preview,
and install lawful local mods. It also gives repository operators a minimal
reading order. It is not a contributor handbook and does not replace technical
specifications.

## Reading order

1. Read the public overview for purpose, legal boundaries, and current status.
1. Read the ADR index for repository decisions.
1. Read the technical index only when implementation understanding is required.
1. Read the relevant user-facing mod skill for the requested game change.
1. Read Unreal skills only when an AI agent or technical operator must control
   Unreal through the native MCP server.
1. Use bibliography, research, and legal records as supporting evidence, never
   as repository authority.

## Mod-user posture

Translate the user's requested game change into supported package capabilities,
required lawful inputs, deterministic data changes, validation evidence, and a
reviewable preview. Ask for missing licensed assets. Never invent ownership,
source availability, engine state, tool names, parameters, or compatibility.

User-facing mod skills must remain understandable to non-programmers. Unreal
skills are different: they are technical operating instructions for AI agents
and repository operators, not ordinary end-user modding documentation.

## Unreal control

Connect to Unreal through the official native inbound MCP server by using the
repository-owned terminal translator. The translator is an MCP client, not an
MCP server. Discover the live tool catalog before acting, preserve native names
and schemas, serialize mutations, use loopback only, and obtain explicit
approval before destructive or project-wide changes.

The outbound MCP client toolset solves the opposite connection direction and is
not part of normal terminal-to-Unreal control.

## Repository knowledge

Architecture decision records contain repository-impacting decisions only.
Technical specifications explain only repository-owned implementation. Neither
surface may contain concrete repository paths. Technical specifications must not
explain proprietary external formats. Bibliography, research, and legal records
preserve external evidence.

Every production ADR reference in source, tests, skills, and documentation must
resolve to a current decision record. When a document is reclassified as
technical knowledge, move it to the technical catalog and update all references;
do not keep a false ADR only to preserve an old path.

## Architecture

The repository uses minimal hexagonal architecture. Add a port or adapter only
for a real external, substitution, or isolation boundary. Keep domain policy out
of process, storage, serialization, protocol, and engine adapters. Do not create
layers as ceremony.

## Model conversion

The canonical FBX artifact is generated from first principles by the
repository-owned binary writer. Do not use Blender or Maya for generation,
conversion, staging, repair, validation, or acceptance. Legacy helpers that
invoke those applications are not supported evidence.

## Validation

Use the canonical repository validator for final evidence. Do not replace it
with direct formatter, compiler, linter, or test commands. In particular, never
run direct Rust formatting commands; the canonical validator owns formatting.
Report uncertainty and failures exactly, and never convert missing evidence into
invented success.

## Versioning and collaboration

Repository-owned version identities use Calendar Versioning, not Semantic
Versioning. Commit history uses Conventional Commits, but commit types never
calculate a calendar identity.

The repository maintains no changelog or release surface. Public collaboration
uses issues only. Pull requests are not part of the workflow.

## Safety and provenance

Never publish original game content, extracted payloads, proprietary engine
source, private local paths, credentials, or third-party replacement media.
Tracked evidence must be synthetic, repository-owned, or otherwise lawfully
redistributable.
