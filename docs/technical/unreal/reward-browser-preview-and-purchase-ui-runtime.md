# Reward browser, preview, and purchase UI runtime

- Status: Active
- Last reviewed: 2026-07-15

## Governing decisions and specifications

<!-- markdownlint-disable-next-line MD013 -->
- [Transactional phone-booth vehicle retrieval](../../adr/unreal/runtime/transactional-phone-booth-vehicle-retrieval.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Common UI front end and progress projection](../../adr/unreal/ui/common-ui-frontend-and-progress-projection.md)
- [UI parity boundary](../../adr/unreal/ui/ui-parity-boundary.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Common UI navigation, menu, and modal runtime](common-ui-navigation-menu-and-modal-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Vehicle retrieval and phone-booth runtime](vehicle-retrieval-and-phone-booth-runtime.md)
- [Vehicle access and roster runtime](vehicle-access-and-roster-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Progression, collectibles, cheats, and credits](progression-collectibles-and-cheats.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Native asset load request and streaming runtime](native-asset-load-request-and-streaming-runtime.md)

## Purpose

This specification defines the native Unreal user-interface runtime for reward
browsing, outfit and vehicle previews, seller inventories, prices,
affordability, purchase confirmation, owned-vehicle selection, damaged-vehicle
presentation, repair handoff, and verified transaction completion.

It preserves the observable browsing and preview experience while replacing
fixed preview arrays, filename-derived models, widget-owned currency, direct
ownership mutation, render-inventory sections, synchronous draw barriers,
platform-specific carousel behavior, and asynchronous callbacks that can update
a superseded selection.

## Native Unreal composition

The runtime uses:

- Common Activatable Widgets for reward stores and vehicle browsers;
- Common UI action data for previous, next, buy, select, repair, confirm,
  cancel, details, and view-toggle actions;
- a game-instance reward-browser subsystem for presentation flow and request
  ownership;
- C++ UMG viewmodels for immutable browser, entry, price, stat, and transaction
  projections;
- Asset Manager primary assets and named bundles for thumbnails, meshes,
  materials, animation, audio, cameras, lights, and preview stages;
- retained streamable handles for accepted browser and preview leases;
- an isolated preview presentation service for three-dimensional scenes; and
- typed application ports for retrieval, merchandise, economy, progression,
  equipment, world delivery, and save operations.

A widget may request a domain command and render its result. It cannot grant
ownership, debit currency, repair a vehicle, equip an outfit, spawn a gameplay
vehicle, or claim that a save completed.

## Ownership

<!-- markdownlint-disable MD013 -->

| Service | Authority |
| :--- | :--- |
| `USharRewardBrowserSubsystem` | Browser sessions, entry ordering, selection, preview leases, focus, and accepted presentation revision. |
| `USharRewardBrowserViewModelSubsystem` | Immutable browser, selected-entry, price, stat, and transaction-result viewmodels. |
| Merchandise application service | Seller inventory, offer eligibility, price, purchase transaction, ownership grant, and purchase result. |
| Vehicle-retrieval application service | Owned-vehicle projection, damage, repair eligibility, selection, delivery, and active retrieval result. |
| Economy service | Currency balance, reservation, debit, refund, and ledger revision. |
| Progression service | Chapter reach, rewards, unlocks, completion overrides, and accepted save state. |
| Equipment service | Current outfit, outfit compatibility, equip transaction, and character-presentation revision. |
| Preview presentation service | Isolated preview scene, camera, lighting, animation, turntable, and asset lease. |
| Asset-load service | Required browser, entry, preview, and transaction presentation bundles. |
| Common UI kernel | Screen activation, focus, semantic actions, modals, transitions, and restoration. |

<!-- markdownlint-enable MD013 -->

The browser subsystem owns presentation only. Domain application services remain
the sole authorities for eligibility, price, ownership, repair, equipment,
world delivery, currency, progression, and persistence.

## Runtime identities

Every accepted operation carries:

- `FSharRewardBrowserSessionId` for one opened browser;
- `FSharRewardBrowserRevision` for its accepted projection;
- `FSharSellerId` for a store, purchase center, or retrieval provider;
- `FSharRewardOfferId` for one seller-specific offer;
- `FSharRewardId` for canonical outfit, vehicle, or other reward identity;
- `FSharRewardPreviewRequestId` for one selected preview;
- `FSharRewardTransactionRequestId` for one purchase, repair, equip, or
  retrieval handoff;
- `FSharPreviewSceneLeaseId` for one isolated presentation scene; and
- exact catalog, progression, economy, equipment, vehicle, seller, feature, and
  save revisions.

A callback without the expected browser, selection, request, and source
revisions is stale. Stale asset completion, preview setup, purchase result,
repair result, delivery result, or save result cannot mutate the current screen.

## Browser states

One browser session is in exactly one state:

- `created`;
- `validating`;
- `loading_catalog`;
- `browsing`;
- `loading_preview`;
- `preview_ready`;
- `confirming`;
- `transaction_pending`;
- `delivery_pending`;
- `recovering`;
- `cancelled`;
- `failed`; or
- `completed`.

Only one selection owns the preview scene. Only one domain transaction may be
pending for the browser. Every request has one terminal result.

## Reward browser entry

`FSharRewardBrowserEntry` contains:

<!-- markdownlint-disable MD013 -->

| Field | Contract |
| :--- | :--- |
| `RewardId` | Canonical outfit, vehicle, or reward identity. |
| `OfferId` | Seller-specific offer identity when purchasable. |
| `RewardKind` | Outfit, vehicle, retrieval entry, or registered feature kind. |
| `DisplayName` | Localized reward name identity. |
| `Description` | Optional localized description identity. |
| `OwnershipState` | Locked, available, owned, equipped, or completion override. |
| `UnlockReason` | Typed prerequisite or effective override. |
| `Price` | Non-negative integer currency amount when purchasable. |
| `Affordability` | Affordable, insufficient balance, not for sale, or stale. |
| `ThumbnailAsset` | Optional soft thumbnail reference. |
| `PreviewAssetId` | Registered isolated preview definition. |
| `StatProfileId` | Optional vehicle or reward statistic projection. |
| `DamageState` | Healthy, damaged, destroyed, or not applicable. |
| `RepairCost` | Optional declared repair amount. |
| `Selectable` | Derived selection eligibility. |
| `UnavailableReason` | Typed reason when selection or purchase is blocked. |
| `CatalogOrdinal` | Stable data-defined ordering value. |
| `FeatureOwnerId` | Base game or validated feature package. |
| `ProjectionRevision` | Exact source revision set. |

<!-- markdownlint-enable MD013 -->

Display text, filenames, widget indexes, array positions, and preview mesh names
are never domain identities.

## Browser membership

Membership comes from seller, progression, ownership, completion, mission, and
platform policies. The browser never scans directories or infers entries from
loaded assets.

Ordering is deterministic by seller-defined group, catalog ordinal, reward kind,
and canonical identity. Carousel wrap is a presentation policy over that order.
A fixed maximum preview array is forbidden.

Locked entries may remain visible when product policy intends acquisition
progress to be understood. Hidden content remains absent until its visibility
predicate is satisfied. Visibility never grants ownership or selection.

## Seller profiles

The base seller profiles include:

- interior outfit seller;
- standard vehicle seller;
- special vehicle seller;
- phone-booth owned-vehicle retrieval; and
- validated feature-owned sellers.

A seller profile declares accepted reward kinds, inventory query, purchase or
selection command, preview stage, price presentation, confirmation policy,
post-transaction behavior, and required bundles.

Seller identity is data. A numeric switch or platform-specific screen subclass
cannot define store behavior.

## Outfit browser

The outfit browser projects the current character, current outfit, default
outfit, owned outfits, visible purchasable outfits, compatibility, prices, and
unlock reasons.

The currently equipped outfit remains represented through selected state or a
separate current-item projection. It is never silently removed in a way that
makes ownership ambiguous.

Selecting an outfit loads its preview definition only. Buying and equipping are
separate commands unless an accepted offer explicitly defines an atomic
purchase-and-equip transaction.

An outfit preview cannot alter the gameplay character, skeleton, materials,
collision, mission eligibility, or save state.

## Vehicle purchase browser

The vehicle purchase browser projects the current seller inventory and may show:

- locked, available, or owned state;
- price and affordability;
- localized name and description;
- speed, acceleration, handling, toughness, or other registered statistics;
- preview mesh and presentation profile;
- acquisition role and seller identity; and
- typed unavailable or duplicate-ownership reason.

Vehicle statistics are read-only catalog projections. Bars, stars,
abbreviations, and localized labels are presentation profiles and cannot change
tuning.

A purchased vehicle becomes owned only after the merchandise transaction,
economy debit, ownership grant, save write, and verification complete.

## Phone-booth browser

The phone-booth browser consumes the immutable projection defined by the vehicle
retrieval runtime. It may show owned and locked vehicles, accepted health,
damage state, destroyed-state repair cost, current selection, and delivery
availability.

The browser does not duplicate retrieval policy. Selection hands the exact
vehicle, booth, projection, mission, currency, and world revisions to the
vehicle-retrieval service.

Repair confirmation, debit, health restoration, delivery reservation, spawn or
reuse, active-slot replacement, and save commit remain one retrieval
transaction. The browser renders the terminal result and never performs a
partial repair or spawn.

## Completion and cheat projections

Completion and cheat state may alter effective visibility, selection, or price
through progression and catalog services. The browser consumes that accepted
effective projection.

A widget never marks entries unlocked merely because a cheat action was entered.
An override does not silently convert into ordinary purchase or ownership unless
the governing domain policy explicitly says so.

## Selection transaction

Changing selection performs:

1. validate browser, seller, entry, and source revisions;
1. reserve a new preview request identity;
1. cancel the previous preview request and release its selection-only lease;
1. request the selected thumbnail and preview bundles;
1. publish a loading selection viewmodel;
1. create or update the isolated preview scene;
1. verify mesh, materials, animation, camera, lights, and stage policy;
1. publish the ready preview revision; and
1. enable only actions valid for that exact revision.

Rapid selection changes may have several external loads in flight, but only the
latest accepted request may publish. Duplicate selection is idempotent.

Back remains available while an optional preview loads. Confirm, buy, repair,
or select is disabled until required preview and domain data are ready. A
transaction already pending blocks conflicting input.

## Preview definition

`FSharRewardPreviewDefinition` contains:

- preview identity and reward kind;
- soft mesh, material, animation, and thumbnail references;
- stage, pedestal, background, camera, and lighting profile identities;
- framing bounds and scale policy;
- idle or turntable motion policy;
- damage or alternate-view variants;
- statistic presentation profile;
- required asset bundles;
- accessibility and reduced-motion profile; and
- fallback behavior.

Definitions reject direct local paths, filename conventions, missing required
assets, unsafe world actors, gameplay-only components, and cameras without
validated framing policy.

## Isolated preview scene

The preview presentation service owns an isolated non-gameplay scene or preview
world. Preview actors:

- have no gameplay authority, possession, artificial intelligence, damage,
  collision effects, save identity, or world registration;
- use presentation-only meshes, materials, animation, camera, lighting, and
  stage assets;
- cannot trigger gameplay events or interact with the active world;
- are destroyed when their lease ends; and
- expose bounded diagnostics through canonical identities.

Preview setup never requires a synchronous render flush. Loading, scene
construction, and teardown are request-owned asynchronous operations.

## Camera, lighting, and framing

Each preview profile declares a camera rig, target bounds, framing margin,
orientation, turntable axis, lighting rig, background, and safe fallback.

Outfits and vehicles use different framing policies. Damage variants may select
a different mesh or material state but retain the same canonical vehicle
identity.

The browser cannot derive scale from hardcoded per-item correction constants.
Exceptional framing data belongs to the preview definition and is validated
against allowed bounds.

## Preview interaction

The base preview actions may include:

- previous and next entry;
- rotate or turntable;
- toggle normal and damaged vehicle presentation;
- toggle summary and detailed statistics;
- confirm purchase, repair, retrieval, or equip; and
- cancel or return.

Actions use semantic Common UI identities. Pointer hotspots, physical buttons,
and platform branches do not define behavior.

Reduced-motion mode disables automatic turntable motion while preserving manual
inspection and all required information.

## Price and affordability

Price is a non-negative integer domain value. Zero is permitted only for an
explicit free offer. A negative, missing, or overflowed price invalidates the
offer.

`FSharRewardPriceViewModel` contains price, accepted balance, affordability,
optional completion or discount policy, currency identity, and source revisions.

The buy action is enabled only when the exact offer is purchasable and the
accepted balance covers the price. Disabled actions expose localized reasons and
accessible state. Hiding a button is not the affordability authority.

## Purchase confirmation

A purchase request contains:

- browser, seller, offer, reward, and player identities;
- exact catalog, progression, economy, ownership, equipment, and save revisions;
- accepted price and currency identity;
- requested post-purchase equip or delivery behavior; and
- idempotency identity.

The confirmation modal renders the accepted price and effect. Confirm and cancel
are semantic actions. Duplicate confirm is idempotent, and closing the modal
does not imply purchase success.

## Purchase transaction

A merchandise purchase performs:

1. validate seller, offer, reward, progression, ownership, and feature state;
1. verify exact price and accepted balance;
1. reserve the currency debit;
1. validate required post-purchase assets and compatibility;
1. stage the ownership grant;
1. stage optional outfit equip or vehicle delivery policy;
1. write and verify the complete save revision;
1. commit the currency ledger and ownership grant atomically;
1. publish one terminal purchase result; and
1. rebuild the browser projection from accepted state.

Any failure before commit releases reservations and preserves balance,
ownership, equipment, world, and save state. A failure after durable commit must
recover by reading the committed result, not by charging again.

## Outfit purchase and equip

Outfit ownership and current equipment are distinct state. The accepted offer
may define one of:

- purchase only;
- purchase and equip atomically;
- equip already owned outfit; or
- preview only.

Equipping validates character compatibility, mission restrictions, required
presentation assets, and save revision. Previewing or purchasing cannot silently
replace the current outfit when the offer does not declare equip behavior.

## Vehicle purchase and delivery

Vehicle purchase grants persistent ownership according to the merchandise
contract. Immediate world delivery is optional and separately declared.

When delivery is requested, the purchase service hands the committed ownership
result to the vehicle-delivery or retrieval service. Purchase success may remain
valid even if optional immediate delivery fails; the terminal result must
separate ownership commit from delivery outcome.

The purchase screen never spawns a gameplay vehicle directly.

## Repair handoff

Destroyed owned vehicles may expose repair through the phone-booth projection.
The browser displays the accepted repair cost and requests confirmation.

Repair is not a merchandise purchase. It remains part of the vehicle-retrieval
transaction so health restoration, currency debit, delivery, active-slot
replacement, and save verification cannot diverge.

## Transaction result

`FSharRewardTransactionResultViewModel` contains:

- request and reward identities;
- purchased, equipped, retrieved, repaired, delivered, cancelled, or failed
  outcome;
- committed currency, ownership, equipment, vehicle, world, and save revisions;
- optional balance, ownership, or delivery change summary;
- localized confirmation or failure identity;
- retry availability and reason; and
- destination or browser-refresh policy.

Presentation acknowledges a terminal application result. It never synthesizes a
success from animation, audio, asset visibility, or changed button state.

## Loading and cancellation

Browser catalog, thumbnail, and preview loads have separate leases. Cancelling a
selection releases selection-only assets while retaining the browser catalog and
shared stage assets.

Closing the browser cancels pending presentation requests. A pending domain
transaction follows its application cancellation policy; the UI remains in a
recovering state until the terminal result is known.

The runtime never abandons an uncertain debit, ownership grant, repair, or save
operation and never allows a second request while the first has unknown state.

## Localization and accessibility

Names, descriptions, prices, statistics, ownership state, unavailable reasons,
confirmation effects, and results use localized identities and typed arguments.

Every browser declares:

- deterministic focus order and selection announcement;
- color-independent lock, ownership, affordability, and damage state;
- text scaling and localization expansion;
- narrated price and transaction effect;
- reduced-motion preview behavior;
- touch-safe previous, next, confirm, and back actions;
- safe-area and split-screen policy; and
- a non-three-dimensional fallback when preview rendering is unavailable.

## Feature and mod overlays

A validated feature package may add sellers, offers, reward kinds, preview
profiles, statistics, or transaction adapters through namespaced definitions.

Feature removal cancels owned browser and preview requests, removes owned
entries, releases owned assets, and restores a valid base selection. A pending
committed domain transaction is completed or recovered before feature teardown.

A feature cannot override base prices, ownership, or retrieval policy merely by
replacing a widget.

## Concurrency

Presentation state mutates on the game thread. Asset loads and domain
transactions may complete asynchronously, but only the owning request and
accepted source revision may publish.

Economy, ownership, equipment, repair, delivery, and save mutations are
serialized through application services. Two browser sessions cannot spend the
same accepted balance or commit the same offer twice.

## Diagnostics

The runtime records bounded structured diagnostics for:

- browser, seller, offer, reward, selection, preview, and transaction
  identities;
- accepted source revisions;
- ordered browser membership and selection;
- asset bundle and preview-scene lease ownership;
- stale, cancelled, or rejected loads;
- affordability and eligibility reasons;
- purchase, equip, retrieval, repair, delivery, and save results; and
- recovery from uncertain external completion.

Diagnostics use canonical identities and typed reasons, never raw local asset
paths or machine-specific locations.

## Failure behavior

- Invalid seller or catalog data blocks the browser before activation.
- An empty valid inventory presents an explicit empty state.
- Missing optional preview art uses a declared accessible fallback.
- Missing required preview or transaction content disables the affected entry
  with a typed reason.
- Invalid price blocks purchase without changing balance.
- Stale selection completion cannot replace the current preview.
- Failed purchase preserves balance and ownership unless a durable commit is
  proven.
- Failed optional delivery preserves committed ownership and reports delivery
  separately.
- Failed repair preserves balance, health, active vehicle, and world state.
- Closing during an uncertain transaction enters recovery rather than assuming
  cancellation.
- Feature removal restores a valid base browser or closes safely.

## Validation

Validation proves:

- every seller and offer has one canonical identity;
- browser membership contains no duplicate offers or lost rewards;
- ordering is deterministic and independent of widget construction;
- prices are non-negative and currency-compatible;
- every entry resolves ownership, unlock, selection, and unavailable state;
- required thumbnails, preview definitions, bundles, cameras, lights, and stages
  resolve;
- preview definitions contain no gameplay authority;
- transaction commands resolve to registered application ports;
- every transaction has idempotency, rollback, save, and verification policy;
- accessibility profiles expose lock, affordability, damage, and result state;
  and
- feature definitions are namespaced and removable.

## Tests

Automated tests cover:

- empty, one-entry, normal, and large inventories;
- deterministic order and carousel wrap;
- locked, available, owned, equipped, and completion-override entries;
- outfit, vehicle, retrieval, and feature reward kinds;
- rapid selection changes and stale preview completion;
- missing optional and required preview assets;
- normal, damaged, destroyed, and repaired vehicle presentation;
- zero, affordable, exactly affordable, unaffordable, invalid, and stale prices;
- duplicate purchase confirmation and idempotent recovery;
- purchase-only, purchase-and-equip, and equip-owned-outfit behavior;
- vehicle purchase with successful and failed optional delivery;
- repair success, insufficient balance, delivery failure, and rollback;
- closing during catalog, preview, and transaction loading;
- two sessions competing for the same balance or offer;
- feature install, removal, and stale callbacks; and
- deterministic diagnostics and repeated browser generation.

## Invariants

- Browser presentation never owns currency, ownership, equipment, repair,
  delivery, or save state.
- Canonical reward and offer identities never come from filenames or widget
  indexes.
- Exactly one selection owns the preview scene.
- Exactly one terminal result exists per domain transaction request.
- A stale load or result never mutates the accepted browser.
- Preview actors never enter the gameplay world or trigger gameplay behavior.
- Purchase, repair, equip, and retrieval remain distinct typed commands.
- Failed or uncertain transactions never permit an unverified second debit.
