# Physical material and impact-response runtime

- Status: Active
- Last reviewed: 2026-07-14

## Governing decisions

<!-- markdownlint-disable-next-line MD013 -->
- [Data-driven Unreal gameplay content catalog](../../adr/unreal/runtime/data-driven-gameplay-content-catalog.md)
- [Runtime parity boundary](../../adr/unreal/runtime/remake-parity-boundary.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Runtime parity test boundary](../../adr/unreal/runtime/runtime-parity-test-boundary.md)

## Purpose

This specification defines native physical-material attributes and impact
presentation for collision-enabled world objects. It replaces a process-wide
attribute table, numeric row lookup, and raw string payloads with validated
physical-material profiles and typed response identities.

## Ownership

<!-- markdownlint-disable MD013 -->

| Authority | Responsibility |
| :--- | :--- |
| Gameplay catalog | Stable surface, collision class, physical profile, and response identities. |
| Native physical material | Friction, restitution, density, and surface-type projection. |
| Collision component | Shape, volume, mass override, channel, and per-instance state. |
| Impact-response subsystem | Sound, Niagara, decal, animation, damage, and telemetry requests. |
| Domain services | Damage, destruction, rewards, mission state, and persistence. |

<!-- markdownlint-enable MD013 -->

An impact response may present a sound, particle, decal, or animation. It cannot
become the authority for damage, destruction, rewards, or save data.

## Runtime topology

The runtime module owns these C++ types:

<!-- markdownlint-disable MD013 -->

| Type | Responsibility |
| :--- | :--- |
| `USharPhysicalMaterialProfile` | Immutable physical and presentation attributes for one canonical surface. |
| `USharImpactResponseDefinition` | Typed response policy for one impact class and surface combination. |
| `USharPhysicalMaterialCatalogSubsystem` | Validated identity lookup and revision checks. |
| `USharImpactResponseSubsystem` | Bounded impact classification, request construction, deduplication, and publication. |
| `FSharImpactObservation` | Immutable participants, surfaces, impulse, speed, normal, point, and context. |
| `FSharImpactResult` | Closed response identities, domain request identities, and rejection reasons. |

<!-- markdownlint-enable MD013 -->

The catalog is generated and immutable during an active world revision. Runtime
registration order does not affect identity or response selection.

## Physical profile contract

Every `USharPhysicalMaterialProfile` contains:

<!-- markdownlint-disable MD013 -->

| Field | Contract |
| :--- | :--- |
| `PhysicalMaterialId` | Globally unique canonical identity. |
| `SurfaceType` | Native surface projection used by collision queries. |
| `Friction` | Finite non-negative coefficient within the approved policy range. |
| `Restitution` | Finite coefficient clamped by the approved physics policy. |
| `Density` | Optional positive density used when mass is volume-derived. |
| `MassOverridePolicy` | None, explicit, or definition-driven override. |
| `ImpactResponseSetId` | Typed sound, effect, decal, and animation response set. |
| `DamageResponseId` | Optional typed domain-damage policy. |
| `GameplayTags` | Surface, material, breakability, vehicle, character, and world tags. |
| `DefinitionRevision` | Immutable revision used to reject stale instances. |

<!-- markdownlint-enable MD013 -->

A profile cannot carry free-form sound, particle, or animation names. Those
fields resolve through validated response identities.

## Collision-class binding

A collision-enabled asset declares:

- one collision-class identity;
- one physical-material profile identity;
- shape and volume evidence;
- collision channels and responses;
- optional mass override;
- breakability and damage tags; and
- native package and definition revisions.

The importer verifies that the assigned physical material exists and that its
surface type, friction, restitution, density, and response set match the plan.
A numeric source ordinal is provenance only and cannot remain the native runtime
lookup key.

## Mass and density

Mass is resolved in this order:

1. validated explicit per-instance override;
1. definition-owned fixed mass;
1. density multiplied by verified collision volume; or
1. native component calculation when the approved policy delegates it.

Zero, negative, non-finite, or implausible mass fails validation. Missing volume
cannot silently produce a density-derived mass. Presentation scale cannot alter
mass without a corresponding collision and import revision.

## Friction and restitution

Friction and restitution are native physical-material properties. Per-contact
combine modes are explicit. A surface profile cannot change these properties
from
an impact callback.

The runtime verifies:

- coefficients are finite and within policy;
- combine modes are compatible with target physics settings;
- moving and static components use the expected material;
- cooked targets preserve the logical physical profile; and
- replay under fixed physics input produces equivalent contact outcomes within
  declared tolerance.

## Impact observation

`FSharImpactObservation` contains:

- source and target stable identities;
- source and target physical-material identities;
- source and target collision classes;
- contact point and normal;
- relative velocity and normal speed;
- impulse magnitude;
- source and target mass snapshots;
- gameplay and mission tags;
- simulation timestamp and fixed-step ordinal; and
- definition revisions.

The observation is immutable after collision classification. Presentation
systems
cannot replace its surfaces or impulse based on listener order.

## Response selection

Response selection uses a validated matrix keyed by:

- source collision class;
- target collision class;
- source surface;
- target surface;
- impact severity band;
- damage or break state;
- world and mission tags; and
- platform presentation policy.

The result may contain:

- one sound-event identity;
- one Niagara-system identity;
- one decal identity;
- one optional animation or montage identity;
- one camera-interest or shake request;
- one typed domain-damage request; and
- one telemetry classification.

Missing optional presentation output produces no request. It does not substitute
an arbitrary default asset.

## Severity bands

Severity is derived from normalized physical evidence such as impulse, relative
normal speed, participating mass, and damage policy. Thresholds are definition
owned and use simulation units.

At minimum, the closed bands are:

- `none`;
- `light`;
- `medium`;
- `heavy`; and
- `destructive`.

Crossing a presentation threshold does not prove gameplay damage. Damage remains
a typed application-port result.

## Sound

Impact sound requests include event identity, emitter identity, contact
transform,
severity, surface identities, concurrency group, and deduplication key.

Repeated physics contacts are rate-limited by contact pair and response policy.
The limiter cannot suppress a required domain event or exactly-once destruction
result.

## Effects and decals

Niagara and decal requests are presentation-only. The response definition
declares
spawn transform, orientation policy, scale curve, lifetime, pooling, material,
and platform budget.

Effects do not perform collision traces that redefine the impact surface. A
decal
may run a bounded placement trace only to confirm the original contact geometry.

## Animation reactions

An optional reaction animation is requested through the typed action-sequence
runtime. The request declares target identity, action identity, resource claims,
priority, interruptibility, and required postcondition.

A collision callback cannot play an animation directly, mutate an animation
controller, or infer domain completion from playback.

## Breakable objects

Breakable presentation consumes a typed domain destruction result. The impact
subsystem may request break evaluation but cannot destroy or reward the object
itself.

On committed destruction, presentation may spawn fragments, sound, effects,
decals, and camera interest according to the response definition. Pooling and
cleanup cannot replay the domain result.

## Vehicles and characters

Vehicle impacts may select tire, body, glass, suspension, or destructible-part
responses according to the collision class. Character impacts additionally obey
ragdoll, reaction, mission, and accessibility policy.

A surface response cannot force vehicle or character state when the owning
movement or gameplay service rejects the request.

## Import integration

Native import converts normalized physical attributes into:

- physical-material assets;
- surface-type assignments;
- collision-component bindings;
- mass and density settings;
- response-definition assets; and
- provenance rows.

Import read-back verifies each generated value and dependency. Missing response
assets, invalid coefficients, unresolved surface types, or numeric-row-only
identity quarantines the affected package.

## Determinism

Equivalent impact observations and catalog revisions produce the same response
identities and severity classification. Random variation, when permitted for
presentation, uses a seed derived from response, participant, contact, and
simulation-step identities.

Random selection never changes damage, destruction, reward, or mission results.

## Failure behavior

The subsystem rejects an impact response when:

- either required physical-material identity is missing or stale;
- collision-class binding is unresolved;
- coefficients or mass evidence are invalid;
- response selection is ambiguous;
- a required response asset is unavailable;
- a request exceeds its platform budget without a declared fallback; or
- the observation is duplicated after its terminal result.

A rejected presentation response leaves physics and domain authority intact. A
required domain request failure is returned to the owning gameplay service.

## Verification

Automated verification proves:

- every physical profile has stable identity and valid coefficients;
- numeric source ordinals never become native runtime identity;
- density and collision volume resolve the expected mass;
- surface and collision-class combinations select one deterministic response;
- optional missing sound, effect, decal, or animation output remains absent;
- collision reactions use typed action requests;
- repeated contacts obey deduplication without losing terminal domain results;
- import read-back matches physical-material and response definitions; and
- fixed-step replay produces equivalent severity and response identities.
