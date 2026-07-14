# Vehicle access and roster runtime

- Status: Active
- Last reviewed: 2026-07-14

## Governing decisions

- [Data-driven Unreal gameplay content catalog](../../adr/unreal/runtime/data-driven-gameplay-content-catalog.md)
- [Gameplay census, presentation, and development-content boundary](gameplay-census-presentation-and-development-boundary.md)
- [Transactional phone-booth vehicle retrieval](../../adr/unreal/runtime/transactional-phone-booth-vehicle-retrieval.md)
- [Runtime parity boundary](../../adr/unreal/runtime/remake-parity-boundary.md)
- [Driving, traffic, and vehicle behavior parity](../../adr/gameplay/vehicles/driving-traffic-and-vehicle-ai.md)

## Purpose

This specification defines vehicle access classes, the complete base-campaign
ownership roster, level traffic membership, secret placements, completion
overrides, driver presentation, inaccessible development content, validation,
and verification.

It complements the transaction rules in
[Vehicle retrieval and phone-booth runtime](vehicle-retrieval-and-phone-booth-runtime.md).
That specification owns browser, repair, loading, delivery, and rollback. This
one owns which vehicle definitions may enter each access class.

## Access classes

Every vehicle definition declares one or more explicit access rows.

| Access class | Meaning |
| :--- | :--- |
| `starting` | Persistent ownership is granted when the level becomes available. |
| `bonus_reward` | One accepted bonus-mission reward grants ownership. |
| `street_race_reward` | Completing the level street-race set grants ownership. |
| `purchase` | One accepted offer transaction grants ownership. |
| `traffic` | The level may spawn disposable drivable traffic instances. |
| `secret_world` | One authored world placement grants temporary level access. |
| `mission_forced` | A mission supplies the vehicle without granting ownership. |
| `mission_required` | Mission activation requires existing ownership. |
| `completion_override` | Full completion plus the declared cheat permits retrieval. |
| `development_only` | Content remains unavailable to normal shipping gameplay. |

Access rows are additive but do not imply one another. Entering traffic, using a
secret placement, driving a mission vehicle, or selecting a completion override
never becomes ordinary persistent ownership.

## Persistent ownership roster

The base campaign has exactly 42 persistent vehicles: six per level. Each level
contains one starting vehicle, one bonus reward, one street-race reward, and
three purchases.

### Level 1

| Vehicle identity | Acquisition |
| :--- | :--- |
| `family_sedan` | Starting vehicle. |
| `cletus_pickup_truck` | Bonus-mission reward. |
| `electaurus` | Street-race reward. |
| `plow_king` | Purchase from Barney for 150 coins. |
| `surveillance_van` | Purchase from Gil for 100 coins. |
| `duff_truck` | Purchase from Gil for 125 coins. |

### Level 2

| Vehicle identity | Acquisition |
| :--- | :--- |
| `honor_roller` | Starting vehicle. |
| `wwii_vehicle` | Bonus-mission reward. |
| `sedan_level_02` | Street-race reward. |
| `mr_plow` | Purchase from Homer for 200 coins. |
| `limo` | Purchase from Gil for 150 coins. |
| `fire_truck` | Purchase from Gil for 250 coins. |

### Level 3

| Vehicle identity | Acquisition |
| :--- | :--- |
| `malibu_stacy_car` | Starting vehicle. |
| `sedan_level_03` | Bonus-mission reward. |
| `book_burning_van` | Street-race reward. |
| `school_bus` | Purchase from Otto for 300 coins. |
| `nerd_car` | Purchase from Gil for 250 coins. |
| `donut_truck` | Purchase from Gil for 250 coins. |

`skinners_sedan` is a display alias for `sedan_level_03`. It does not resolve to
`sedan_level_02`, `sedan_a`, `sedan_b`, or `chase_sedan`.

### Level 4

| Vehicle identity | Acquisition |
| :--- | :--- |
| `canyonero` | Starting vehicle. |
| `kremlin` | Bonus-mission reward. |
| `clown_car` | Street-race reward. |
| `tractor` | Purchase from Willie for 400 coins. |
| `curator` | Purchase from Gil for 300 coins. |
| `krustys_limo` | Purchase from Gil for 350 coins. |

### Level 5

| Vehicle identity | Acquisition |
| :--- | :--- |
| `longhorn` | Starting vehicle. |
| `hover_car` | Bonus-mission reward. |
| `el_carro_loco` | Street-race reward. |
| `car_built_for_homer` | Purchase from Homer for 500 coins. |
| `cola_truck` | Purchase from Gil for 350 coins. |
| `police_car` | Purchase from Gil for 425 coins. |

`professor_frinks_hover_car` and `frinks_hover_car` are aliases for
`hover_car`.

### Level 6

| Vehicle identity | Acquisition |
| :--- | :--- |
| `ferrini_red` | Starting vehicle. |
| `bandit` | Bonus-mission reward. |
| `stutz_bearcat_36` | Street-race reward. |
| `globex_super_villain_car` | Purchase from Kearney for 600 coins. |
| `armored_truck` | Purchase from Gil for 400 coins. |
| `chase_sedan` | Purchase from Gil for 500 coins. |

### Level 7

| Vehicle identity | Acquisition |
| :--- | :--- |
| `seventies_sports_car` | Starting vehicle. |
| `mr_burns_limo` | Bonus-mission reward. |
| `open_wheel_race_car` | Street-race reward. |
| `zombie_car` | Purchase from the declared zombie vendor for 500 coins. |
| `hearse` | Purchase from Gil for 750 coins. |
| `hover_bike` | Purchase from Gil for 1,000 coins. |

## Ownership invariants

- The roster contains exactly six rows for each of seven levels.
- Every row has exactly one ordinary acquisition class.
- Starting ownership is level-gated and requires no currency transaction.
- Bonus and street-race rewards commit exactly once.
- Purchases use the declared offer, price, seller, and economy revision.
- A mission placement may reference an owned vehicle without replaying its
  acquisition.
- An NPC driver is presentation and does not grant, revoke, or duplicate
  ownership.
- A display alias cannot collapse two distinct sedan or limousine definitions.
- Save state stores canonical vehicle identities, never roster indices.

## Secret vehicle roster

Each base level has exactly one secret world vehicle.

| Level | Vehicle identity | Placement family |
| :--- | :--- | :--- |
| 1 | `speed_rocket` | Rich-district Gold House frontage. |
| 2 | `monorail_car` | Broken monorail station rail. |
| 3 | `knight_boat` | C-Spanker shipping-container placement. |
| 4 | `quad_bike` | Trailer-park placement between two trailers. |
| 5 | `obliteratatron_big_wheel_truck` | Stadium elevated platform. |
| 6 | `planet_hype_50s_car` | Planet Hype roof opening. |
| 7 | `rc_buggy` | Krusty Burger roof near the tanker site. |

Aliases include `atv` for `quad_bike`, `knightboat` for `knight_boat`, and
`monster_truck` plus `obliteration_big_wheel_truck` for
`obliteratatron_big_wheel_truck`.

Secret access is level-scoped and temporary. Leaving the vehicle, replacing the
world, restarting the level, or loading a save does not create ordinary
ownership. The completion override may expose an eligible secret definition
through retrieval without changing that rule.

## Traffic rosters

Each level declares exactly four ordinary traffic models.

| Level | Canonical traffic roster |
| :--- | :--- |
| 1 | `minivan`, `glass_truck`, `mini_school_bus`, `pickup_road_vehicle` |
| 2 | `taxi`, `sedan_b`, `sports_car_a`, `pizza_van` |
| 3 | `compact_car`, `pickup_road_vehicle`, `sports_car_a`, `fish_delivery_truck` |
| 4 | `compact_car`, `suv`, `garbage_truck`, `nuclear_waste_truck` |
| 5 | `sports_car_b`, `suv`, `ambulance`, `vote_quimby_truck` |
| 6 | `compact_car`, `pickup_road_vehicle`, `burns_armored_truck`, `itchy_and_scratchy_movie_truck` |
| 7 | `coffin_cart`, `hallo_hearse`, `ghost_ship`, `witch_car` |

A traffic row declares level membership, relative spawn weight, driver
archetype, traffic tuning, horn or ambient-audio profile, color-variant policy,
and completion-override eligibility.

Traffic instances are disposable world state. Hijacking one permits immediate
use but does not add it to the persistent 42-vehicle roster. A traffic vehicle
used as a race opponent or mission target retains the same canonical definition
and receives a placement-specific controller and tuning profile.

## Completion override

The all-vehicles completion override requires both:

1. accepted full-game completion; and
1. activation of the declared unlock-vehicles cheat.

The override exposes only definitions whose policy allows it. It may include
traffic, secret, mission, or otherwise non-owned vehicles, but it remains a
separate access state. It does not:

- grant ordinary acquisition records;
- increase level vehicle-completion counts;
- replay a purchase or reward;
- erase damage or repair requirements unless the vehicle has no persistent
  health contract;
- create a vendor or seller transaction; or
- make development-only content available.

## Driver presentation

A vehicle may have level- or mission-scoped driver bindings. Examples include
Homer, Bart, Lisa, Marge, Apu, Cletus, Grampa, Skinner and Agnes, Comic Book Guy,
Professor Frink, Snake, Smithers, Otto, Chief Wiggum, and the zombie driver.

Driver rows declare the vehicle, character placement, allowed contexts, dialogue
profile, seat binding, and lifecycle. A driver binding never becomes vehicle
identity. A forced mission driver may differ from the driver used when the same
owned vehicle is retrieved in free play.

## Sedan identity boundary

The sedan family contains distinct canonical definitions:

- `sedan_level_02`, the Level 2 street-race reward;
- `sedan_level_03`, Skinner's bonus-mission reward;
- `sedan_a`, an unavailable development traffic definition;
- `sedan_b`, Level 2 traffic;
- `chase_sedan`, a Level 6 purchase with mission placements; and
- other separately cataloged sedan vehicles.

A disambiguation page contributes aliases and collision tests only. It never
creates a `sedan` primary asset or chooses a target by display name.

## Development-only vehicles

`red_brick_car` and `sedan_a` are development-only in the base campaign. Their
presence in source or generated assets does not make them traffic, secret,
owned, purchasable, or completion-override vehicles.

Development-only definitions may exist in repository-owned diagnostic catalogs
when needed for migration verification. Shipping gameplay, save state,
progression, retrieval, traffic, races, missions, and user-facing package claims
must not expose them without a separate accepted content decision.

Prerelease screenshots, prototypes, unused variants, and abandoned placements are
negative compatibility evidence. They do not override final roster or access
records.

## Query model

`FSharVehicleAccessProjection` contains:

| Field | Contract |
| :--- | :--- |
| `VehicleId` | Canonical definition identity. |
| `AccessClasses` | Explicit applicable access rows. |
| `OwningLevelId` | Ordinary acquisition level when present. |
| `AcquisitionId` | Starting, reward, race-set, or offer identity. |
| `Price` | Exact purchase price when applicable. |
| `TrafficLevels` | Ordered native traffic memberships. |
| `SecretPlacementId` | Optional temporary world placement. |
| `CompletionOverrideEligible` | Explicit completion-cheat policy. |
| `DevelopmentAvailability` | Shipping, diagnostic, or unavailable. |
| `DriverBindings` | Context-scoped presentation rows. |
| `RevisionToken` | Catalog, campaign, progression, and mod revision. |

Equivalent catalog and progression state must produce an equivalent projection
independent of object paths, directory enumeration, frame rate, or platform.

## Mod overlays

Validated overlays may add vehicle definitions and access rows through declared
extension points. They cannot alter immutable base ownership counts, repurpose a
base canonical identity, or make development-only base content available by path
collision.

A mod-owned campaign owns its own roster counts. A base-campaign overlay must
declare every changed acquisition, traffic, secret, or completion-override row
and pass the same uniqueness and closure checks.

## Failure behavior

Vehicle access compilation fails closed on:

- a duplicate canonical vehicle or acquisition identity;
- an ambiguous alias or unqualified sedan reference;
- a base level with other than six persistent ownership rows;
- a base level with other than four traffic rows;
- a base level with other than one secret vehicle;
- a purchase with missing seller, price, or economy policy;
- a reward with missing mission or race-set ownership;
- traffic, secret, forced, or completion access represented as ordinary
  ownership;
- a development-only vehicle exposed to shipping gameplay;
- a missing vehicle, driver, placement, tuning, or presentation dependency; or
- a projection whose read-back differs from the deterministic plan.

## Verification

Automated evidence includes:

- exactly 42 persistent vehicles and six per level;
- exact acquisition class, seller, and price for every purchase;
- exactly seven secret vehicles and one per level;
- exact four-vehicle traffic roster for every level;
- sedan and limousine alias collision rejection;
- traffic hijacking without ownership mutation;
- secret access without ownership mutation;
- forced and required mission access without ownership mutation;
- bonus and street-race rewards committing exactly once;
- completion override without progression-count mutation;
- driver presentation across free-play and mission contexts;
- unavailable development vehicles remaining absent from shipping queries;
- save reload preserving canonical ownership and health; and
- equivalent projections across supported platforms and graphics presets.
