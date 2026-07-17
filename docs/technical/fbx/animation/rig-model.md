# Animation rig model

- Status: Active
- Last reviewed: 2026-07-16

## Governing decisions and specifications

- [Hexagonal scene export](../../../adr/pipeline/fbx/hexagonal-scene-export.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Historical source-document evidence classification and publication boundary](../../unreal/historical-source-document-evidence-classification-and-publication-boundary.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Native import, material rebuild, and world assembly](../../unreal/native-import-material-and-world-assembly.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Playable avatar, character controller, and footprint runtime](../../unreal/playable-avatar-character-controller-and-footprint-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Character animation clip catalog and vehicle-handoff choreography runtime](../../unreal/character-animation-clip-catalog-and-vehicle-handoff-choreography-runtime.md)
- [Animation clip timing](clip-timing.md)

## Purpose

This specification defines the canonical joint, hierarchy, rest-transform,
animated-track, root, motion, and native Skeleton compatibility model shared by
repository-owned character rigs and animation conversion.

A source scene may contain a full rig, animation curves only, helper transforms,
controllers, constraints, clip nodes, or obsolete editor metadata. Only
validated canonical joints, approved curves, and explicit conversion policy
become target animation data.

## Canonical rig identity

A rig definition records:

- canonical character and rig identities;
- rig revision;
- root joint identity;
- optional motion-root identity;
- deterministically ordered joint identities;
- one parent identity per non-root joint;
- rest translation, rotation, scale, and orientation;
- segment and semantic-region roles;
- deformation and non-deformation classification;
- required and optional animation tracks;
- approved custom curves;
- coordinate-system and unit policy;
- retargeting policy;
- compatible native Skeleton identity;
- compatible mesh and Physics Asset identities; and
- validation tolerances.

Source node order, Maya path, namespace, array index, pointer, or display name
is
not canonical joint identity.

## Repository model

A canonical rig owns joint identity, parentage, rest transforms, orientation,
semantic roles, and animation-channel binding. Source adapters resolve
references
before application logic accepts a clip.

Coordinate conversion is applied consistently to rest and animated transforms.
The same versioned recipe owns axis conversion, handedness, unit scale, rotation
order normalization, joint orientation, root policy, and motion extraction.

## Full-rig and animation-only evidence

Full-rig evidence may establish hierarchy, rest state, bind state, skinning
relationships, and animation tracks.

Animation-only evidence may establish animated track names, key data, curves,
clip metadata, and timing, but it cannot create a new canonical rig by itself.
It must target one previously validated rig and native Skeleton revision.

When a source scene contains no joint nodes, track compatibility is tested
against the declared rig. Every required track must resolve exactly once.

## Joint identity normalization

Normalization uses an explicit alias table and semantic rules. It may remove a
known source namespace or map a verified historical alias to one canonical
identity.

Normalization rejects:

- two source names mapping to one joint without a declared merge policy;
- one source name matching multiple joints;
- case-only collisions on targets with case-insensitive tooling;
- unknown namespace removal;
- suffix or prefix guessing without a registered rule;
- generated display names as authority; and
- joint identity derived from source array order.

## Hierarchy

The canonical hierarchy is acyclic and deterministically ordered. Every non-root
joint has exactly one known parent.

Validation proves:

- one canonical root;
- no hierarchy cycles;
- no orphan required joints;
- stable parentage across import and reimport;
- parent-before-child ordering;
- consistent semantic regions; and
- no hidden helper transform inserted as an unexpected skeletal root.

A project transform used for export organization is not automatically a native
Skeleton joint.

## Root and motion ownership

Root ownership is explicit. The rig declares whether one joint owns skeletal
root state and whether a separate motion root exists.

The conversion recipe defines:

- source root identity;
- target root identity;
- optional motion track;
- root translation and rotation policy;
- scale policy;
- root-motion extraction policy;
- in-place conversion policy;
- accumulated displacement expectations; and
- retargeting behavior.

A transform-only wrapper, first node, clip node, or namespace root cannot become
native root authority by accident.

## Rest and bind state

Rest transforms, bind transforms, inverse bind matrices, and animated transforms
remain distinct values.

Missing rest evidence is never synthesized from an arbitrary animation frame.
When an accepted external canonical rig supplies rest and bind state, the
animation-only scene contributes no replacement.

Validation compares native Skeleton and Skeletal Mesh read-back against the
accepted rig within declared tolerances.

## Animated tracks

Each animated channel binds to one canonical joint or approved curve identity.
Track metadata records:

- source track name;
- canonical target identity;
- channel kind;
- translation, rotation, scale, or custom-curve role;
- key count and timing range;
- interpolation and tangent policy;
- required or optional classification;
- normalization rule; and
- conversion result.

Unknown, duplicate, orphan, or ambiguous tracks fail closed unless a versioned
allowlist explicitly marks an optional source helper as ignored.

## Track profiles and reduced variants

A character animation catalog may declare multiple compatible track profiles:

- full-body canonical animation;
- verified constant-channel reduction;
- additive or masked subset;
- pose reference;
- diagnostic comparison; or
- another explicitly accepted profile.

Each profile lists required, optional, intentionally omitted, and forbidden
tracks plus expected root, motion, orientation, foot-plant, and approved curve
roles. A scene with fewer channels is not accepted merely because it can be
imported. Its reduced set must match one declared profile and preserve the
target
pose and motion within tolerance.

Track-profile selection follows
<!-- markdownlint-disable-next-line MD013 -->
[Character animation clip catalog and vehicle-handoff choreography runtime](../../unreal/character-animation-clip-catalog-and-vehicle-handoff-choreography-runtime.md).
Filename tokens, scene size, source node count, or first-successful import
cannot
select a profile.

## Scale animation

Scale animation is rejected by default unless the character and clip contract
explicitly permit it. Permitted scale tracks must be finite, positive where
required, compatible with the target Skeleton, and verified after import.

Scale cannot compensate for a wrong unit conversion, malformed hierarchy, or
incorrect rest pose.

## Coordinate conversion

Coordinate conversion applies to:

- joint rest transforms;
- animated translation and rotation;
- root and motion displacement;
- orientation bases;
- sockets and attachment transforms when part of the rig contract; and
- any approved vector-valued custom curves.

Rest and animation use the same handedness and unit recipe. A correction applied
only to animation or only to the mesh is invalid unless an accepted explicit
adapter owns the difference.

## Native Unreal Skeleton binding

Animation import targets the exact native Skeleton identity declared by the rig.
Validation proves:

- required track names resolve;
- hierarchy and root expectations match;
- no unexpected Skeleton is created;
- the Animation Sequence references the intended Skeleton;
- track count and normalized names read back correctly;
- root motion and curves use the declared policy;
- mesh preview uses the intended Skeletal Mesh; and
- reimport preserves asset identity and dependencies.

An importer warning about missing expected tracks is terminal unless the public
clip definition marks those tracks optional.

## Retargeting

Retargeting is not implicit import repair. A retarget operation requires:

- source and target rig identities;
- exact mapping profile;
- reference-pose compatibility;
- scale and root policy;
- chain or joint mapping;
- expected semantic regions;
- pose and motion tolerances;
- deterministic output; and
- separate validation.

A source clip compatible with the canonical character rig is imported directly
against that rig rather than retargeted merely because the source file is old.

## Animation controllers and helpers

Source controllers, constraints, character sets, clip nodes, helper transforms,
editor layers, and selection sets are evidence or authoring machinery. Their
baked result may contribute to normalized animation when the conversion recipe
proves it.

They do not become runtime controller objects, Skeleton joints, gameplay states,
or public documentation.

## Determinism

The same source evidence, canonical rig, conversion recipe, toolchain, and
target
profile produces an equivalent joint map, normalized transforms, animation
tracks, native Skeleton binding, and read-back result.

Filesystem order, namespace allocation, editor selection, loaded scene history,
locale, or pointer order cannot change joint mapping.

## Invariants

- Every non-root joint has exactly one known parent.
- The hierarchy is acyclic and deterministically ordered.
- Exactly one canonical skeletal root exists.
- Motion ownership is explicit.
- Animation channels bind only to known canonical joints or approved curves.
- Rest transforms and animated transforms remain separate values.
- Rest and animation share one coordinate conversion policy.
- Animation-only scenes cannot create a canonical rig.
- Required tracks resolve exactly once.
- Unexpected helper transforms cannot become native roots.
- The native Animation Sequence binds to the intended Skeleton.
- Source scene text, namespaces, private paths, and editor metadata never ship.

## Failure behavior

The rig or clip fails closed when:

- parent identity is unknown;
- identities collide after normalization;
- hierarchy contains a cycle;
- required joint is orphaned;
- root ownership is ambiguous;
- expected track is missing;
- one track maps to multiple targets;
- an unknown track is not explicitly optional;
- scale animation violates policy;
- coordinate conversion is inconsistent;
- native import creates or selects the wrong Skeleton;
- root motion differs beyond tolerance;
- reimport changes joint mapping unexpectedly; or
- generated output contains private evidence metadata.

Failure leaves the previous accepted rig and animations unchanged.

## Verification

- Hierarchy tests cover roots, deep parent chains, stable ordering, and cycle
  rejection.
- Identity tests cover namespaces, aliases, case collisions, duplicate mapping,
  and unknown tracks.
- Animation tests verify stable joint binding after input reordering.
- Transform tests compare rest and animated coordinate conversion.
- Root tests verify skeletal root, motion root, in-place, and extracted motion.
- Import tests verify native Skeleton identity, tracks, curves, dependencies,
  and
  reimport stability.
- Animation-only fixtures prove that curve scenes bind to an existing rig
  without
  creating a new one.
- Privacy tests reject source scene text, paths, comments, and editor metadata.
