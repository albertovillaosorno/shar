# Shared rig-family animation libraries

- Status: Accepted
- Decision date: 2026-07-18
- Scope: Character animation assets, catalogs, runtime lookup, import, and mods

## Context

Character models and animation data are separate concerns. Multiple compatible
characters can consume the same locomotion, idle, traversal, interaction,
reaction, dialogue, and vehicle-handoff clips. Duplicating those clips under
every character would multiply storage, cook time, validation, patch size,
memory pressure, and mod conflicts while making corrections inconsistent.

The migration evidence also distinguishes model identity from animation-bank
identity and includes universal banks that are not owned by one visible
character. The native architecture should preserve that useful separation
without carrying forward historical naming or loading mechanisms.

## Decision

All character Animation Sequences, Montages, Blend Spaces, Pose Assets, Control
Rig support assets, and animation metadata live under one shared
character-animation root. They are grouped by compatible rig family and semantic
clip group, never by character folder.

```text
/Game/SHAR/Art/Characters/Animations/<rig_family>/<clip_group>/
/Game/SHAR/Data/Characters/AnimationLibraries/
```

Each rig family has one `USharCharacterAnimationLibraryDefinition` Primary
Asset. It maps semantic clip identities to native soft references, compatibility
rules, root-motion policy, markers, curves, variants, and fallbacks. Character
and presentation definitions reference the library by Primary Asset identity.
They do not own copied sequences.

The first compatible humanoid family uses one global library assembled from the
best validated common clips. A clip's source provenance or historical
association does not make it character-owned. Character-specific performances
remain in the same central root and are marked with explicit eligibility
predicates or variant tags. They are still imported once.

Runtime selects animation by semantic role, rig-family compatibility, gameplay
state, and declared variant policy. It never builds asset paths from character
names and never searches character folders.

Retargeting is an offline deterministic pipeline operation. When another rig
family cannot consume a source clip directly, the pipeline creates one validated
native retargeted result for that target family. Runtime retargeting is not the
default content path.

## Consequences

- Common clips are imported, cooked, patched, loaded, and validated once.
- Fixes to timing, markers, root motion, curves, or compression apply
  consistently.
- Characters can replace models, materials, abilities, or presentations without
  duplicating the animation corpus.
- Mods can add namespaced libraries, clips, or eligibility overlays without
  copying the base library.
- A genuinely incompatible creature or topology receives a new rig family and
  library rather than forcing unsafe compatibility.
- The shared root may contain many assets, but packages remain one asset each
  and directories remain partitioned by rig family and semantic group.

## Rejected alternatives

- One copied animation directory per character.
- Treating a common clip as owned by the character whose source package supplied
  it.
- One unstructured flat folder containing every animation for every topology.
- Runtime filename construction or directory scanning.
- Automatic runtime retargeting of arbitrary mod or imported skeletons.
