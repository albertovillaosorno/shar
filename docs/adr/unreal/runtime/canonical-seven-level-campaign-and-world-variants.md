# Canonical seven-level campaign and world variants

- Status: Accepted
- Decision date: 2026-07-14
- Scope: Campaign identity, level composition, and world ownership

## Context

The game has seven ordered campaign levels that reuse one persistent geographic
world. Each level selects a protagonist, calendar day, mission sequence,
collectibles, rewards, traffic, interactions, audio, presentation variant, and
fixed time-of-day profile. Treating every level as an unrelated map or geographic
authority would duplicate geometry and make cross-level identity, coordinates,
progression, streaming, imports, mods, and tests disagree.

Unreal provides World Partition for automatic cell streaming, Runtime Data
Layers for world variants and gameplay transitions, and primary assets for
explicit load and audit control. These mechanisms must serve the campaign
domain rather than replace it.

The campaign also needs one deterministic completion model. Completion cannot be
inferred from loaded actors, menu text, map packages, or a collection of
unrelated save flags.

## Decision

The runtime has one non-Blueprint `USharCampaignDefinition` primary data asset
and exactly seven non-Blueprint `USharLevelDefinition` primary data assets.
Campaign and level identities are stable domain identifiers resolved through the
gameplay catalog.

The seven levels are ordered and immutable within the base campaign. Each level
definition owns its protagonist, calendar day, predecessor and successor,
Runtime Data Layer set, mission sequence, bonus mission, street-race set,
wager-race policy, collectible sets, progression requirements, audio profile,
starting state, fixed time-of-day profile, and completion transition.

There is exactly one persistent World Partition geographic world. It owns the
canonical terrain, roads, districts, buildings, linked interiors, landmarks,
coordinates, and placement identities for the complete map.

A level variant is composed by activating its generated Runtime Data Layer set
and definitions inside that world. Shared geography remains canonical to the
world. Level-specific missions, traffic, collectibles, gags, characters,
interior availability, damage state, props, lighting, audio, and presentation
remain in level-owned layers or definitions.

A non-campaign `level_11_test` state uses the same world for development and
validation. It may exercise dynamic day-night cycling and experimental mission
composition, but it has no predecessor, successor, campaign completion weight,
progression rewards, or save identity.

The campaign service is a `UGameInstanceSubsystem`. It resolves campaign and
level definitions, validates dependency closure, coordinates world travel,
requests Asset Manager bundles, and exposes read-only campaign state. World
subsystems own active gameplay; the campaign service does not keep mutable actor
pointers.

Level completion uses eight equally weighted categories. The domain preserves
exact rational progress and projects a one-decimal percentage to the user
interface. Overall game completion is the mean of the seven level percentages,
weighted to ninety-nine percent, plus one percent for the all-card movie reward.

## Consequences

- One campaign identity owns sequence and completion semantics.
- One level identity owns every level-specific gameplay and presentation
  requirement.
- One geographic world eliminates duplicated location and coordinate authority
  while preserving seven distinct campaign states.
- World Partition streams spatial cells; Runtime Data Layers select the active
  campaign or test state; neither mechanism owns progression.
- Campaign levels retain fixed time-of-day profiles while the test state may
  exercise the native dynamic day-night cycle.
- Level transitions fail before travel when the destination definition, world,
  layer set, protagonist, or required bundle is incomplete.
- Re-entering a level restores accepted progression without replaying rewards or
  reconstructing identity from world actors.
- Wager races remain repeatable economy challenges and do not contribute to
  level completion.
- Starting and secret vehicles remain playable catalog entries but do not count
  among the five progression vehicles required for level completion.
- The tutorial is a campaign onboarding step and does not replace one of the
  seven Level 1 story missions.
- Blueprint may author presentation and consume reflected state but cannot own
  campaign ordering, completion weights, or level identity.

## Rejected alternatives

- Three persistent base worlds with separate geographic identities.
- Seven unrelated monolithic maps with duplicated shared geometry.
- One world containing every level variant active simultaneously.
- Treating `level_11_test` as an eighth campaign level.
- A level identity inferred from a map filename or loaded Data Layers.
- Level progress calculated from widget state or actor enumeration.
- Equal weighting per individual collectible instead of per progress category.
- Counting wager races, starting vehicles, or secret vehicles toward level
  completion.
- Hand-authored campaign order in Blueprint or menu widgets.
