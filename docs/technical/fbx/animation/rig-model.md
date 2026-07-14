# Animation rig model

- Status: Active
- Last reviewed: 2026-07-13

## Governing decision

- [Hexagonal scene export](../../../adr/pipeline/fbx/hexagonal-scene-export.md)

## Purpose

This specification explains the joint identity and transform model shared by
skeletons and animation.

## Repository model

A canonical rig owns joint identity, parentage, rest transforms, and
animation-channel binding. Source adapters resolve references before application
logic accepts a clip. Coordinate conversion is applied consistently to rest and
animated transforms.

## Invariants

- Every non-root joint has exactly one known parent.
- The hierarchy is acyclic and deterministically ordered.
- Animation channels bind only to known canonical joints.
- Rest transforms and animated transforms remain separate values.

## Failure behavior

- Unknown parents, duplicate identities, hierarchy cycles, and orphan channels
  fail closed.
- Missing rest evidence is never synthesized from animation keys.

## Verification

- Hierarchy tests cover roots, deep parent chains, and cycle rejection.
- Animation tests verify stable joint binding after input reordering.
- Transform tests compare rest and animated coordinate conversion.
