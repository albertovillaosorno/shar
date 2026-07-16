# Vehicle retrieval and phone-booth runtime

- Status: Active
- Last reviewed: 2026-07-14

## Governing decisions

<!-- markdownlint-disable-next-line MD013 -->
- [Transactional phone-booth vehicle retrieval](../../adr/unreal/runtime/transactional-phone-booth-vehicle-retrieval.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Data-driven Unreal gameplay content catalog](../../adr/unreal/runtime/data-driven-gameplay-content-catalog.md)
<!-- markdownlint-disable-next-line MD013 -->
- [State-driven missions, interactions, interiors, and notoriety](../../adr/unreal/runtime/state-driven-missions-interactions-and-notoriety.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Common UI front end and progress projection](../../adr/unreal/ui/common-ui-frontend-and-progress-projection.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Reward browser, preview, and purchase UI runtime](reward-browser-preview-and-purchase-ui-runtime.md)

## Purpose

This specification defines booth interaction, vehicle browsing, ownership,
locked presentation, health persistence, repair, driver presentation, safe world
delivery, forced-mission restrictions, completion overrides, rollback, and
verification. The complete persistent, traffic, secret, mission, completion, and
development-only membership rules follow
[Vehicle access and roster runtime](vehicle-access-and-roster-runtime.md).

## Ownership

`USharVehicleRetrievalSubsystem` is a game-instance subsystem. It owns retrieval
queries and transactions across level travel. It consumes read-only ports for:

- gameplay catalog and aliases;
- campaign reach, chapter boundary, terrain discovery, and sandbox availability;
- accepted vehicle ownership and completion overrides;
- vehicle health and active retrieval slot;
- currency transactions;
- mission replacement policy;
- Asset Manager bundle loading;
- world delivery reservation and spawning; and
- save revision publication.

`USharVehicleRetrievalWorldSubsystem` owns the active world-side delivery and
read-back adapter. It never decides ownership or repair price.

A phone booth is a Smart Object with one exclusive interaction slot and one
canonical placement identity. Common UI presents the immutable query result.

## Retrieval state

`FSharVehicleRetrievalEntry` contains:

<!-- markdownlint-disable MD013 -->

| Field | Contract |
| :--- | :--- |
| `VehicleId` | Canonical vehicle identity. |
| `DisplayName` | Localizable presentation. |
| `PreviewAssets` | Soft icon, mesh, and material references. |
| `OwnershipState` | Locked, owned, completion override, or unavailable. |
| `DiscoveryState` | Hidden or visible in the reached campaign range. |
| `AcquisitionKind` | Starting, purchase, reward, traffic, secret, mission, or override. |
| `HealthPermille` | Accepted health from zero through one thousand. |
| `DamageState` | Healthy, damaged, destroyed, or unavailable. |
| `RepairCost` | Explicit charge when destroyed. |
| `DriverPresentationId` | Optional level-scoped driver binding. |
| `Selectable` | Derived from ownership, mission, repair, and content readiness. |
| `UnavailableReason` | Typed reason when not selectable. |
| `ProjectionRevision` | Catalog, progression, mission, currency, and world revision. |

<!-- markdownlint-enable MD013 -->

Health is stored as an integer domain value. Presentation may render a whole
percentage but never writes the value.

## Browser membership

The normal browser projects vehicle definitions associated with campaign levels
up to the highest accepted reached level. It includes owned and locked entries
so
acquisition progress is visible. Ordering is deterministic by campaign level,
acquisition role, offer ordinal, and canonical vehicle identity.

The browser does not include:

- an arbitrary traffic vehicle merely because the player entered it;
- secret vehicles without an ownership or completion-override rule;
- mission-only or forced placements;
- inaccessible development content; or
- content whose required definition is unavailable.

The completion override is a separate browser mode. It is available only after
complete game progression and activation of the declared unlock-vehicles cheat.
It exposes only vehicle definitions whose override policy explicitly permits
retrieval. It never grants purchase, reward, or ordinary ownership state.

## Interaction availability

The booth interaction is unavailable while:

- phone-booth interactions are disabled by the active mission stage;
- a mission-failure or recovery transition owns input;
- another user owns the Smart Object slot;
- the frontend or pause stack is already entering a blocking transition;
- the owning world or booth placement is not ready;
- required catalogs or save state are invalid; or
- a prior retrieval transaction is unresolved.

Unavailable interaction produces typed busy feedback and no menu or state
mutation.

## Opening transaction

Opening the browser performs:

1. reserve the booth Smart Object slot;
1. verify the chapter boundary, gameplay state, mission policy, player, terrain,
   and booth placement;
1. snapshot health for the current owned active vehicle when present;
1. persist that health in the candidate gameplay snapshot;
1. query catalog, ownership, completion, currency, and mission policy;
1. validate every projected vehicle definition and preview reference;
1. build the immutable ordered browser projection;
1. activate the Common UI modal layer with semantic input actions; and
1. retain the reservation until selection, cancellation, or failure completes.

A forced mission vehicle is not written into owned health state. Opening a booth
cannot repair, replace, or grant that vehicle.

## Selection and repair

Selecting an entry validates the projection revision and the exact vehicle
state.
A locked, unavailable, stale, or mission-forbidden entry returns typed feedback.

A destroyed owned vehicle requires an explicit repair confirmation. The base
repair policy charges 10 coins and restores health to one thousand permille.
Repair is one currency transaction linked to the vehicle and retrieval request.
If the current balance is below the charge, selection fails without a debit,
health change, or world mutation.

A damaged but non-destroyed vehicle retains its accepted health and does not
incur
the destroyed-vehicle repair charge.

## Delivery plan

`FSharVehicleDeliveryPlan` contains:

| Field | Contract |
| :--- | :--- |
| `RequestId` | Unique idempotency identity. |
| `VehicleId` | Canonical selected vehicle. |
| `BoothPlacementId` | Owning interaction placement. |
| `CurrentVehicleInstanceId` | Optional instance being replaced or reused. |
| `DeliveryTransformId` | Stable candidate transform identity. |
| `DriverPresentationId` | Optional driver actor or presentation binding. |
| `RequiredBundles` | Definition, gameplay, presentation, and audio bundles. |
| `HealthPermille` | Accepted post-repair or retained health. |
| `RepairTransactionId` | Optional staged economy transaction. |
| `MissionPolicyRevision` | Exact replacement policy used by the plan. |
| `WorldRevision` | Exact world and streaming state. |

The world adapter evaluates declared booth delivery transforms in deterministic
order. A transform is valid only when its World Partition cell and required Data
Layers are active and a swept vehicle volume is clear of blocking geometry,
characters, vehicles, mission targets, Smart Objects, and unsafe navigation.

The service never invents a fallback transform from the current camera or player
facing. If no declared transform is safe, retrieval fails without mutation.

## Commit transaction

A valid selection performs:

1. revalidate projection, mission, currency, and world revisions;
1. stage the repair debit and health update when required;
1. load the selected primary-asset bundles asynchronously;
1. reserve the selected delivery transform;
1. reuse the active instance when it is the same canonical vehicle and remains
   valid;
1. otherwise spawn one candidate vehicle instance outside the active slot;
1. apply tuning, health, damage presentation, and driver binding;
1. register the candidate with vehicle, traffic, audio, save, and mission ports;
1. verify canonical identity, transform, health, collision, and world ownership;
1. atomically commit repair, health, and active retrieval-slot revisions;
1. retire the superseded owned instance only after commit; and
1. close Common UI, release the Smart Object slot, and publish success feedback.

A repeated request with the same request identity returns the accepted result
and
never charges or spawns twice.

## Driver presentation

Some retrieved vehicles arrive with an owner or associated driver in declared
level contexts. The driver is a character placement binding with its own
canonical identity and actor lifecycle.

Driver presence:

- does not change vehicle ownership;
- does not create another vehicle definition;
- may select a level-scoped dialogue event;
- is omitted when the declared character or presentation bundle is unavailable;
  and
- is released according to the character and ambient-population contracts after
  the retrieval sequence ends.

Re-selecting the same active vehicle may emit a bounded driver response without
replacing the vehicle or replaying any acquisition reward.

## Mission integration

Each mission stage declares one retrieval policy:

- `allowed`;
- `disabled`;
- `allowed_without_replacement`;
- `owned_vehicle_only`; or
- `declared_vehicle_only`.

A forced-vehicle stage normally uses `disabled` . A required-vehicle gate may
allow
the booth so the player can retrieve the exact owned required vehicle. A mission
cannot use the booth to convert a forced or target vehicle into ownership.

Mission restart restores the mission policy and active mission vehicle
separately
from the player's persistent retrieval slot.

## Clothing boundary

Costume offers and current clothing are owned by the character-purchase and
presentation services. Purchase locations may expose a clothing browser. The
phone-booth vehicle browser does not list, purchase, equip, preload, or save
costumes.

## Save contract

Portable save state records:

- owned vehicle identities and acquisition transactions;
- accepted health for each owned vehicle;
- the active retrieval vehicle identity;
- completion-override and cheat state through their own contracts; and
- pending or accepted repair transaction identity when required by recovery.

It never stores booth object paths, spawn actor names, preview indices, driver
actor pointers, or physical asset paths.

## Failure behavior

Retrieval fails closed on:

- a stale projection or changed mission, currency, catalog, or world revision;
- locked, unavailable, unknown, or ambiguous vehicle identity;
- insufficient currency for a required repair;
- invalid health or repair policy;
- missing bundles or driver dependencies;
- no safe declared delivery transform;
- spawn, registration, collision, or read-back mismatch;
- duplicate canonical active instances;
- save commit failure; or
- cancellation during a stage that cannot produce a complete candidate.

Failure destroys or unregisters only the candidate instance, releases staged
bundles and reservations, reverses the staged debit, restores the prior active
slot, and releases the booth interaction.

## Verification

Automated evidence includes:

- deterministic browser membership and ordering for every campaign reach;
- locked entries remaining unselectable;
- traffic, secret, target, and forced access not granting ownership;
- completion override eligibility and cheat gating;
- current owned-vehicle health persistence on browser entry;
- healthy, damaged, destroyed, insufficient-currency, and repaired selection;
- exactly-once 10-coin repair debit;
- same-vehicle reuse without duplicate spawn;
- driver and no-driver level contexts;
- blocked, occupied, streamed-out, and stale delivery transforms;
- forced and required mission policies;
- interruption at every load, spawn, registration, and commit stage;
- save reload preserving ownership, health, and active selection; and
- keyboard, gamepad, and touch Common UI parity.
