# Animation clip timing

- Status: Active
- Last reviewed: 2026-07-13

## Governing decision

- [Hexagonal scene export](../../../adr/pipeline/fbx/hexagonal-scene-export.md)

## Purpose

This specification explains how repository-owned animation conversion preserves
authored duration and key ordering.

## Repository model

Input adapters provide the declared source rate, logical frame identities, clip
bounds, and key values. The canonical clip model keeps those values distinct.
The binary writer converts them into its serialization time unit without
changing logical playback speed.

## Invariants

- Clip bounds are finite and ordered.
- Key times are monotonic within each channel.
- Representation-unit conversion preserves authored duration.
- No importer or review application supplies an implicit frame-rate default.

## Failure behavior

- Missing, zero, negative, or contradictory rates reject the clip.
- Keys outside the validated clip range reject the clip.
- Non-finite or non-monotonic timing rejects the channel.

## Verification

- Domain tests compare source duration with canonical duration.
- Writer tests read back serialized key times and clip bounds.
- Regression fixtures cover fractional rates and multi-channel ordering.
