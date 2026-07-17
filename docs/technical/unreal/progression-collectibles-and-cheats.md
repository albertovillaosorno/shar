# Progression, collectibles, cheats, and credits

- Status: Active
- Last reviewed: 2026-07-16

## Governing decisions

<!-- markdownlint-disable-next-line MD013 -->
- [Open sandbox chapters and world progression](../../adr/gameplay/open-sandbox-chapters-and-world-progression.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Common UI front end and progress projection](../../adr/unreal/ui/common-ui-frontend-and-progress-projection.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Collector cards, coins, rewards, gags, and wasps](../../adr/gameplay/collectibles/collectibles-rewards-gags-and-wasps.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Native flying-hazard actors and StateTree execution](../../adr/unreal/runtime/native-flying-hazard-actors-and-state-trees.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Flying-hazard and projectile runtime](flying-hazard-and-projectile-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Mission world-entity and respawn runtime](mission-world-entity-and-respawn-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Data-driven Unreal gameplay content catalog](../../adr/unreal/runtime/data-driven-gameplay-content-catalog.md)
<!-- markdownlint-disable-next-line MD013 -->
- [State-driven missions, interactions, interiors, and notoriety](../../adr/unreal/runtime/state-driven-missions-interactions-and-notoriety.md)
- [Runtime parity boundary](../../adr/unreal/runtime/remake-parity-boundary.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Shared runtime tagging, modding, and platform compatibility](../../adr/unreal/runtime/shared-runtime-tagging-modding-and-platform-compatibility.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Portable save storage and lifecycle](../../adr/unreal/runtime/portable-save-storage-and-lifecycle.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Frontend screen flow and settings runtime](frontend-screen-flow-and-settings-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [HUD feedback cue and presentation-primitives runtime](hud-feedback-cue-and-presentation-primitives-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Playable avatar, character controller, and footprint runtime](playable-avatar-character-controller-and-footprint-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Transient VFX and breakable-presentation runtime](transient-vfx-and-breakable-presentation-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Native audio device, resource, player, and tuning adapter runtime](native-audio-device-resource-player-and-tuning-adapter-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Mission briefing, result, and replay UI runtime](mission-briefing-result-and-replay-ui-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Reward browser, preview, and purchase UI runtime](reward-browser-preview-and-purchase-ui-runtime.md)

## Purpose

This specification defines the native Unreal contracts for currency,
collectibles, reward progression, cheat input, credits, and calendar-driven
presentation. These systems share catalog and save infrastructure, but they do
not share identity, persistence, completion, or mutation semantics.

The implementation preserves observable gameplay while removing accidental
coupling among world pickups, purchasing, gallery completion, debug effects,
front-end presentation, and save storage.

## Repository model

`USharProgressionSubsystem` owns deterministic progression transactions. It
consumes canonical definitions from the gameplay catalog and reads or writes
portable state only through the save port. It does not discover world actors,
inspect asset paths, or infer completion from presentation.

`USharCheatSubsystem` owns logical cheat-input recognition and effect lifetime.
It publishes typed effect requests to the owning gameplay systems. It does not
reach into arbitrary actors or serialize an opaque global bit mask.

`USharCreditsSubsystem` owns credits-sequence selection, playback state, audio
cues, skip policy, and return destination. It does not decide whether the final
mission completed or write progression directly.

The root gameplay catalog references the following generated assets:

<!-- markdownlint-disable MD013 -->

| Asset | Primary asset type | Purpose |
| :--- | :--- | :--- |
| Progression catalog | `SharProgressionCatalog` | Currency, collectible sets, rewards, and completion rules. |
| Meta catalog | `SharMetaCatalog` | Cheat definitions, credits sequences, and calendar themes. |
| Currency source table | `FSharCurrencySourceRow` | World, reward, penalty, repair, purchase, and cheat transactions. |
| Collectible table | `FSharCollectibleRow` | Card and other collectible identities. |
| Collectible placement table | `FSharCollectiblePlacementRow` | Level-scoped placement and consumption state. |
| Collectible-set table | `FSharCollectibleSetRow` | Deck membership and completion rewards. |
| Cheat table | `FSharCheatDefinitionRow` | Logical sequence, prerequisites, lifetime, and effect. |
| Credits table | `FSharCreditsSequenceRow` | Ordered rows, cues, playback mode, and return state. |
| Calendar-theme table | `FSharCalendarThemeRow` | Date predicate and presentation-only overrides. |

<!-- markdownlint-enable MD013 -->

Definitions and tables live under the catalog's existing data roots. Secondary
icons, meshes, sounds, sequences, and menu presentation remain soft references
in the catalog's art, audio, media, and user-interface roots.

## Identity contract

Canonical identifiers are stable lowercase `snake_case` values.

<!-- markdownlint-disable MD013 -->

| Domain | Identity examples | Rule |
| :--- | :--- | :--- |
| Currency | `coins` | One global spendable balance. |
| Card set | `collector_cards_level_01` | One set per gameplay level. |
| Card | `collector_card_level_01_01` | One stable identity per card and level ordinal. |
| Destructible source | `cola_crate`, `vending_machine` | Definition identity, not placement identity. |
| Placement | deterministic opaque placement identity | One consumed-state key per authored instance. |
| Reward | `bonus_map_level_01`, `movie_ticket` | Explicit unlock identity. |
| Cheat | `unlock_cards`, `extra_coins` | Logical effect identity, never a button sequence. |
| Credits | `front_end_credits`, `post_ending_credits` | Distinct playback and return contracts. |
| Calendar theme | `christmas_menu`, `halloween_menu` | Presentation-only date rule. |

<!-- markdownlint-enable MD013 -->

A definition identity and a placement identity are never interchangeable. One
collectible definition may have one placement, while one destructible source
definition may have many separately consumed placements.

## Currency ledger

The coin balance is a signed 64-bit domain value constrained to a non-negative
accepted state. Every change is an immutable `FSharCurrencyTransaction` with:

<!-- markdownlint-disable MD013 -->

| Field | Contract |
| :--- | :--- |
| `TransactionId` | Deterministic unique identity within the save revision. |
| `CurrencyId` | Canonical currency identity. |
| `Reason` | Controlled source or sink kind. |
| `Amount` | Positive grant or negative debit. |
| `SourceDefinitionId` | Optional reward, purchase, penalty, or cheat identity. |
| `PlacementId` | Required for one-time world sources. |
| `LevelId` | Level context without changing the global balance. |
| `MissionId` | Optional mission context. |
| `TimestampOrdinal` | Deterministic transaction order, not wall-clock authority. |
| `SavePolicy` | Persistent, transient-drop, or session-only. |

<!-- markdownlint-enable MD013 -->

The accepted balance is the previous accepted balance plus the ordered
transaction batch. Validation occurs before any presentation pickup, sound, or
counter animation is shown as successful.

Coins persist across chapter unlocks, mission transitions, world reloads, and
character switches. None creates a new balance or replays accepted one-time
source transactions.

## Currency sources

`FSharCurrencySourceRow` declares an exact source kind and amount.

Supported source kinds are:

- authored world pickup;
- destructible prop;
- vehicle or world-object destruction;
- wasp destruction;
- gag completion;
- mission or race reward;
- wager result;
- recoverable dropped balance;
- cheat grant; and
- migration correction.

The current verified destructible values are:

| Definition | Accepted total payout |
| :--- | ---: |
| `cola_crate` | 30 coins |
| `vending_machine` | 15 coins |

A destructible may emit staged pickups while taking damage only when its
definition declares the stages. The sum of every stage and final-destruction
payout must equal the accepted total exactly. Reloading, mission selection, or
streaming cannot reset damage merely to duplicate an already accepted payout.

Preset world trails and gag-generated one-time sources remain consumed in the
save after collection. Transient pickups emitted by damage may despawn, but
their source transaction cannot be accepted twice.

## World coin pickup runtime

A visible coin is one presentation instance for a canonical currency-source
batch.
`FSharWorldCoinInstance` contains:

- coin instance and revision;
- source batch and transaction identities;
- placement identity when authored;
- accepted amount;
- spawn, collection, decay, and HUD-flight policy;
- world, mission, feature, and persistence revisions;
- native Actor or component weak identity; and
- terminal state.

The closed states are `inactive`, `spawning`, `available`, `collecting`,
`flying_to_hud`, `collected`, `decaying`, `expired`, and `cancelled`. Pool
slots,
active-array positions, drawable pointers, and HUD coordinates are presentation
details rather than identity.

Spawn requests may describe an authored pickup, staged destructible payout,
vehicle-destruction payout, recoverable loss, cheat presentation, or other
registered currency source. The accepted source batch fixes total value before
individual coins are presented. Randomized initial velocity, bounce, spin,
pitch,
and sparkle parameters use a stable declared seed and cannot alter total payout.

Collection validates the current avatar or vehicle, pickup radius, world and
coin
revisions, source eligibility, duplicate-suppression key, and currency
transaction.
The currency ledger commits exactly once before collection presentation begins.
Audio, sparkle, pop-up, HUD flight, or object deactivation cannot grant
currency.

Coins use native collision or overlap observations, movement or projectile
components where appropriate, and bounded pooling. Physics drag, gravity,
bounce,
ground hover, lifetime, decay, vehicle collection multiplier, visibility
culling,
and animation are authored policies with units and validation.

A collected coin may animate toward one local player's HUD using a presentation
lease. Split-screen projection, viewport coordinates, camera changes, pause,
quality, and HUD teardown cannot change the accepted balance. If HUD
presentation
is unavailable, the ledger result remains valid and a terminal presentation
finding is recorded.

World unload, mission replacement, feature removal, source cancellation, or pool
teardown cancels only presentation that has not committed a ledger result. A
recoverable dropped-balance source retains its batch and expiration contract.
Returning a pooled instance resets every transform, collision, movement,
material,
audio, sparkle, HUD, callback, and ownership field before reuse.

Coin collection sounds follow
<!-- markdownlint-disable-next-line MD013 -->
[Native audio device, resource, player, and tuning adapter runtime](native-audio-device-resource-player-and-tuning-adapter-runtime.md).
Sparkles and collection effects follow
<!-- markdownlint-disable-next-line MD013 -->
[Transient VFX and breakable-presentation runtime](transient-vfx-and-breakable-presentation-runtime.md).

## Gag completion

A gag is a level-scoped interaction and progression record. Its presentation may
reuse a prop, animation, sound, or gag concept across world variants, but its
completion key is the ordered pair of `LevelId` and `GagPlacementId`.

`FSharGagProgressRow` contains:

<!-- markdownlint-disable MD013 -->

| Field | Contract |
| :--- | :--- |
| `LevelId` | Exact level variant that owns the completion. |
| `GagPlacementId` | Stable interaction placement identity. |
| `GagDefinitionId` | Reusable gag concept and presentation identity. |
| `LocationId` | Canonical exterior or interior location. |
| `RewardPolicyId` | Optional exactly-once reward transaction. |
| `CompletionKey` | Durable level-scoped save identity. |
| `CountsForLevelCompletion` | Whether the placement contributes to the level gag total. |
| `ReplayPolicy` | Whether presentation may replay after completion. |

<!-- markdownlint-enable MD013 -->

The verified level totals are:

| Level | Required gag completions |
| :--- | ---: |
| Level 1 | 15 |
| Level 2 | 11 |
| Level 3 | 11 |
| Level 4 | 15 |
| Level 5 | 6 |
| Level 6 | 11 |
| Level 7 | 15 |
| **Total** | **84** |

Activation is accepted through the interaction runtime. A Smart Object slot,
world actor, animation, sound, visible cue, or user-interface prompt never owns
the completion or reward. The progression service validates the chapter,
placement, activation state, current completion state, and reward policy before
committing one transaction.

A repeated gag concept in reused world geometry receives a distinct placement
and completion key only when declared for a chapter or persistent world
projection.
Reusing one placement identity for incompatible projections is invalid. An
interior gag follows the same ledger and remains placement-scoped even when the
interior geometry is shared.

Most gag definitions may grant currency, but the reward is explicit rather than
implied. A gag with no declared reward still records completion when it counts
toward chapter progress. A special reward amount is represented by its own
reward policy and does not change the gag identity.

After completion, a replayable gag may repeat presentation without replaying the
currency transaction, completion key, statistics event, or chapter-progress
increment. Streaming, mission restart, world reload, chapter unlock, save
reload, or entering a second portal to the same interior cannot duplicate the
accepted
result.

Unused, unreachable, malformed, or unverified gag concepts remain inactive and
do not count toward the total. No separate gag quote collection is created for
this verified slice.

Validation rejects duplicate completion keys, duplicate level placement
identities, totals that do not match the declared level rows, a reward without a
registered policy, a counted placement with no reachable interaction, and a gag
that grants progression directly from presentation code.

## Currency loss and recovery

Currency loss is represented by explicit transactions, never direct counter
mutation.

<!-- markdownlint-disable MD013 -->

- A police capture applies a 50-coin fine, clamped so the balance cannot become
  negative.
- Repairing a destroyed selected vehicle through the retrieval interface costs
  10 coins when the repair condition applies. Eligibility, insufficient-currency
  rejection, health persistence, delivery, and rollback follow
  [Vehicle retrieval and phone-booth runtime](vehicle-retrieval-and-phone-booth-runtime.md).
- A wasp hit or destruction of the currently driven vehicle may emit a
  recoverable transient drop according to its definition.
- Recoverable dropped coins retain a source batch identity and expiration. A
  recovered drop reverses only the corresponding pending loss.
- An expired drop finalizes the loss exactly once.

<!-- markdownlint-enable MD013 -->

Presentation may animate multiple coin objects, but the ledger remains the
single balance authority.

## Purchases and recurring sinks

Vehicle, costume, instant-repair, paid-sleep, taxi, wager, and convenience
purchases use one atomic transaction:

1. resolve the canonical offer and context;
1. verify chapter, progression, availability, and current ownership or need;
1. verify the accepted balance covers the exact price;
1. stage the debit and resulting service or unlock together;
1. validate every granted definition and postcondition;
1. commit the debit and result in one save revision; and
1. publish presentation only after commit succeeds.

Failure leaves both balance and result unchanged. Repeating a permanent purchase
returns the existing ownership result and never charges twice. Repeatable
services such as repairs, motels, taxi fees, and wagers require a new service
transaction.

Owned costumes are permanently menu-equippable. The player does not return to a
shop merely to wear an owned costume.

A bounded renewable coin-source set resets eligibility when a world session
begins. One-time mission, chapter, collectible, boss, achievement, purchase, and
discovery transactions never reset. Economy simulation proves that renewable
income and recurring sinks preserve a recoverable story path without enabling
instant unlimited purchases.

The historic 14,800-coin purchase census is source evidence only. The open-world
economy uses versioned mathematical curves for chapter income, permanent
costumes, vehicles, repair, sleep, taxi work, wagers, and reasonable failure
recovery. Every catalog revision explains changed totals and passes solvency and
grind-bound tests.

## Reward grants and merchandise offers

A reward definition and a merchandise offer are different records. A reward
describes what can be granted and under which progression predicate. An offer
quotes one reward or service in a seller context with a price and availability
revision.

`FSharRewardGrantTransaction` contains:

- reward definition identity;
- source mission, race, collectible, purchase, achievement, or system identity;
- granted vehicle, costume, currency, media, or feature identities;
- participant and save-slot identity;
- repeatability and ownership policy;
- source and catalog revisions;
- exactly-once transaction key; and
- resulting progression revision.

A reward grant validates every target identity before commit. Granting a vehicle
or costume creates ownership of the existing canonical definition; it does not
create another asset or alias identity.

`FSharMerchandiseOffer` contains:

- offer and reward identities;
- seller role and seller placement identity;
- chapter, mission, progression, and feature predicates;
- exact currency price and currency kind;
- stock and repeatability policy;
- preview and presentation profile;
- offer, catalog, and save revisions; and
- replacement or supersession policy.

Seller roles are catalog data rather than a fixed enum in runtime code. A shop,
mission giver, contextual interaction, frontend menu, or mod feature can expose
an offer only through a validated seller binding.

Purchase uses the atomic transaction above. A stale offer, changed price,
already-owned permanent reward, insufficient balance, missing seller, or invalid
grant rejects before debit. A repeated accepted request returns the existing
transaction result.

Reward synchronization derives projections from the accepted save ledger. It
does not copy mutable flags between a reward manager and a separate character
sheet or rely on an update queue to converge state.

## Chapter collectible activation

Chapter 1 persistent collectible sets activate at new game. Completing each
chapter activates its successor's sets. Activated sets remain active
permanently, so completing all story missions without collecting optional
content leaves all
seven chapter sets available together.

Activation and collection are independent. Mission-objective pickups remain
scoped to the active mission lease and do not enter persistent chapter
activation.

## Collector-card definitions

Collector cards are not generic currency pickups. `FSharCollectibleRow` records:

| Field | Contract |
| :--- | :--- |
| `CollectibleId` | Stable card identity. |
| `SetId` | Owning chapter deck. |
| `SetOrdinal` | Dense ordinal within the deck. |
| `ChapterId` | Canonical chapter identity. |
| `SourceLevelAlias` | Optional historic conversion alias only. |
| `DisplayName` | Localizable gallery name. |
| `Image` | Soft gallery image reference. |
| `QuoteEvents` | Optional ordered collection-response references. |
| `CompletionWeight` | Explicit scrapbook contribution. |

### Card subtype and quote metadata

The shared card catalog uses a closed subtype:

<!-- markdownlint-disable MD013 -->

| Subtype | Contract |
| :--- | :--- |
| `collector` | Counted world collectible that belongs to one seven-card level set. |
| `bonus` | Non-collector card metadata used only by an explicitly owning presentation or reward definition. |

<!-- markdownlint-enable MD013 -->

A bonus-card row does not become a world collectible, add scrapbook completion,
or unlock a set reward merely because it shares the card schema. Its owner must
reference it through a typed catalog identity.

Every card row also records a stable level identity, level-local ordinal,
localizable display-name key, image identity, and an ordered bounded list of
quote-event identities. Imported evidence may contain up to three quote slots;
empty slots are discarded during normalization rather than preserved as runtime
sentinels.

Quote identities resolve through the dialogue or sound catalog. Array position,
character enum ordinal, display text, and hashed source names are provenance
only
and cannot become native runtime authority. An unresolved quote fails catalog
validation without invalidating the card's already accepted progression state.

There are seven collector-card definitions in each of seven level sets, for 49
cards total. Set ordinals are unique and dense from one through seven.

## Verified collector-card deck membership

Each ordered display name below binds to the stable ordinal identity
`collector_card_level_<level>_<ordinal>` . Localized display text and quote
events
may change without changing that identity.

### Level 1

1. `collector_card_level_01_01` — Home Made Football;
1. `collector_card_level_01_02` — Crab Juice;
1. `collector_card_level_01_03` — Insanity Pepper;
1. `collector_card_level_01_04` — Spinemelter 2000;
1. `collector_card_level_01_05` — Parchment;
1. `collector_card_level_01_06` — Carbon Rod; and
1. `collector_card_level_01_07` — Mr. Sparkle Box.

### Level 2

1. `collector_card_level_02_01` — Head of Jebediah;
1. `collector_card_level_02_02` — AM Radio Toy;
1. `collector_card_level_02_03` — Bonestorm Game;
1. `collector_card_level_02_04` — Big Butt Skinner;
1. `collector_card_level_02_05` — Mr. Honeybunny;
1. `collector_card_level_02_06` — Drivers License; and
1. `collector_card_level_02_07` — Pregnancy Test.

### Level 3

1. `collector_card_level_03_01` — Angel Skeleton;
1. `collector_card_level_03_02` — Bart's Soul;
1. `collector_card_level_03_03` — Lisa Lionheart;
1. `collector_card_level_03_04` — Lisa's Valentine;
1. `collector_card_level_03_05` — Lisa's Machine;
1. `collector_card_level_03_06` — Evil Braces; and
1. `collector_card_level_03_07` — Soy Pop.

### Level 4

1. `collector_card_level_04_01` — Mr. Plow Jacket;
1. `collector_card_level_04_02` — Burns Portrait;
1. `collector_card_level_04_03` — Love Letter;
1. `collector_card_level_04_04` — "Homer" Bowling Ball;
1. `collector_card_level_04_05` — Red Blazer;
1. `collector_card_level_04_06` — Boudoir Album; and
1. `collector_card_level_04_07` — Pepper Spray.

### Level 5

1. `collector_card_level_05_01` — Apu's T-Shirt;
1. `collector_card_level_05_02` — Pin Pals Shirt;
1. `collector_card_level_05_03` — Prop 24 Sign;
1. `collector_card_level_05_04` — Baby Feeder;
1. `collector_card_level_05_05` — Ganesh Costume;
1. `collector_card_level_05_06` — Chutney Squishee; and
1. `collector_card_level_05_07` — Hot Dog.

### Level 6

1. `collector_card_level_06_01` — Radioactive Man #1;
1. `collector_card_level_06_02` — "BORT" License Plate;
1. `collector_card_level_06_03` — Bart T-Shirt;
1. `collector_card_level_06_04` — Australia Boot;
1. `collector_card_level_06_05` — Itchy and Scratchy Cel;
1. `collector_card_level_06_06` — Gabbo Doll; and
1. `collector_card_level_06_07` — Bart's Flying Hamster Science Project.

### Level 7

1. `collector_card_level_07_01` — Soul Donut;
1. `collector_card_level_07_02` — Evil Krusty Doll;
1. `collector_card_level_07_03` — Human Cookbook;
1. `collector_card_level_07_04` — Time Travel Toaster;
1. `collector_card_level_07_05` — Hell Toupee;
1. `collector_card_level_07_06` — Monkey's Paw; and
1. `collector_card_level_07_07` — "Smarch" Calendar.

A card placement references one card identity, one world-layer composition, and
one deterministic placement identity. Collecting the placement is idempotent:
re-entering, streaming, save reload, or overlapping collision events cannot add
the same card twice.

## Card gallery and set completion

The gallery reads collected identities from progression state and joins them to
catalog definitions. It does not store a second mutable copy of collection
state.

Completing all seven cards in a chapter performs one atomic transition:

- mark the chapter card set complete;
- grant the matching bonus-race map reward where retained;
- unlock the chapter's one bounded passive ability;
- update scrapbook and achievement projection; and
- publish the card-set completion event.

The base game has seven chapter-set passive abilities, not 49 card abilities.
Completing one chapter set cannot grant another chapter's reward or passive.
Removing or replacing presentation assets does not revoke accepted card or
ability state.

Collecting all 49 cards enables the movie-ticket reward transaction. Granting
the ticket contributes to complete progression. Entering the associated movie
presentation is a separate state transition; viewing the movie is not inferred
merely from ticket ownership.

### Gallery repository and save projection

`USharCardCatalogSubsystem` owns immutable card definitions and resolves by
canonical card identity or one validated source alias. Runtime lookup does not
allocate card subclasses, index a fixed pointer array by image number, or search
hashed display names linearly.

Portable progression stores the set of collected canonical card identities and
accepted set-completion transactions. A compact device representation may use a
versioned generated bitset, but bit position belongs to that exact save-schema
revision and is converted back to identities before domain use. Catalog reorder,
new cards, removed presentation, or mod overlays cannot reinterpret an existing
bit.

The gallery view is derived from:

- active catalog revision;
- collected card identity set;
- completed card-set transaction set;
- localizable card metadata;
- presentation availability; and
- current cheat overlay when one is enabled.

Adding a collected card validates subtype, chapter set, set ordinal, placement
transaction, and existing progression state before one idempotent commit. Deck
completion is calculated from the required identity set, not a mutable count.
Removing cards is a development-only test operation and cannot occur through an
ordinary player or cheat overlay.

The card-unlock cheat changes gallery visibility and eligibility projection
only. It does not insert card identities into portable progression, complete
chapter sets, unlock passive abilities, grant bonus maps, update achievements,
or grant the movie ticket.

## In-game card browser

The pause-time card browser consumes the same immutable gallery projection as
the
frontend. Opening it validates pause ownership, local-player focus, progression
revision, card-catalog revision, and presentation readiness before activating a
blocking Common UI layer.

The browser preserves canonical deck order, collected state, localized metadata,
full-view eligibility, and declared fallback art. It does not maintain a second
pause-specific card collection or infer identity from a widget slot.

Back, previous, next, inspect, and close are semantic UI actions. Closing
restores
the exact pause screen and focus owner that opened the browser. A late
thumbnail,
high-resolution art, or transition callback cannot change the accepted card or
close a replacement screen.

Card-collected, chapter-set-complete, complete-deck, and unlock feedback follows
<!-- markdownlint-disable-next-line MD013 -->
[HUD feedback cue and presentation-primitives runtime](hud-feedback-cue-and-presentation-primitives-runtime.md).
Those cues project committed progression and cannot grant collection or unlocks.

## Frontend card and mission galleries

Gallery view models are immutable projections of accepted catalog, progression,
and replay-eligibility revisions.

The card gallery exposes canonical deck order, collected state, title, episode,
description, quote, image presentation, full-view eligibility, focus, and
accessibility alternatives. Thumbnail or high-resolution asset availability
cannot grant collection or change deck order. Missing presentation uses the
declared fallback while preserving identity and progress.

The mission gallery exposes canonical mission and chapter identities, display
order, completion, replay policy, required content, image and title
presentation, checkpoint policy, and disabled reason. Runtime code never derives
mission identity from a screenshot filename or menu ordinal.

Selecting an eligible mission submits a replay transition containing the exact
save, progression, mission, participant, checkpoint, content, and isolation
revisions. Widget exit, image load, or transition animation cannot start replay.
Detailed screen and asset lifetime follows the
<!-- markdownlint-disable-next-line MD013 -->
[frontend screen flow and settings runtime](frontend-screen-flow-and-settings-runtime.md).

## Chapter and game progress projection

Chapter, story, optional-content, achievement, and 100-percent completion rules
are owned by the
[open sandbox chapter runtime](open-sandbox-chapter-runtime.md) and the
[open sandbox campaign design](../gameplay/open-sandbox-campaign-design.md).
Historic level denominators remain conversion evidence only and cannot create a
player-facing level state.
The complete 42-vehicle ownership census and the distinction between persistent,
traffic, secret, mission, completion, and development access follow
[Vehicle access and roster runtime](vehicle-access-and-roster-runtime.md).

Progression state provides accepted identity sets and transaction results. The
campaign service calculates exact rational progress. Scrapbook, save-slot, and
pause-menu widgets consume the same immutable projection and never recalculate
it from visible entries. Exact scrapbook modes and gallery membership follow
[Frontend shell and menu runtime](frontend-shell-and-menu-runtime.md).

## Character-sheet decomposition

The native runtime does not retain one process-global character sheet that owns
all campaign, mission, race, collectible, vehicle, costume, media, tutorial, and
percentage state.

Portable state is decomposed by domain and stable identity:

<!-- markdownlint-disable MD013 -->

| Domain | Authoritative state |
| :--- | :--- |
| Campaign | Current chapter boundary, completed story missions, and accepted chapter transitions. |
| Mission | Active mission checkpoint and completed mission transaction identities. |
| Side activities | Completed bonus mission, street race, wager, taxi, and boss identities. |
| Collectibles | Collected card, wasp, gag, and other placement transaction identities. |
| Economy | Ordered currency ledger and permanent purchase identities. |
| Characters and costumes | Unlocked, currently eligible, purchased, and equipped identities. |
| Vehicles | Ownership, access, active vehicle, damage, repair, and retrieval state. |
| Media and tutorials | Explicit viewed, unlocked, or acknowledged identities. |
| Achievements | Accepted counters and completion identities under the achievement schema. |
| World | Discovery, interiors, connectors, expansions, persistent placements, and clock state. |

<!-- markdownlint-enable MD013 -->

A projection service joins immutable snapshots for the scrapbook, pause menu,
save-slot summary, achievement screen, mission browser, and diagnostics. The
projection is not a second mutable copy of domain state.

### Mission progression

Mission state distinguishes:

- available mission identity;
- accepted active mission and checkpoint;
- completed mission transaction;
- chapter-final transition transaction;
- replay eligibility;
- accepted attempt count and declared skip eligibility or skip transaction;
- optional-objective terminal result and reward-claim revision; and
- best time, no-death, damage, position, score, or other registered evidence
  where declared.

The current and highest historic mission ordinals are derived compatibility
projections only. They cannot skip an unavailable mission, infer a chapter
transition, or replace the exact completed identity set.

Story, bonus, street-race, taxi, wager, and boss results remain separate typed
families. Completing one family cannot increment another because they shared an
array slot or mission number.

### Counts and completion

Counts are derived from accepted identity sets and the active catalog revision.
The runtime does not increment independent mutable totals for cards, gags,
wasps, vehicles, costumes, missions, races, or rewards and then trust those
totals as
authority.

A counted projection records:

- required canonical identity set;
- accepted completed identity set;
- excluded, optional, mod-owned, and unavailable identities;
- catalog and save revisions;
- exact rational numerator and denominator; and
- localized display rounding policy.

Chapter and game completion use the current open-sandbox completion contract.
Historic per-level denominators and one-sheet percentage formulas remain
conversion evidence only.

Adding, removing, or overriding catalog content cannot reinterpret an accepted
save silently. Migration maps old ordinals or bit positions to canonical
identities before projection.

### Tutorial and media state

Tutorial acknowledgement, cinematic or movie eligibility, viewing state, and
credits access use separate stable identities and explicit predicates.

A single boolean cannot stand for every tutorial or media item. Unlocking a
movie, owning a ticket, viewing the movie, and receiving any completion credit
are separate accepted states.

Presentation failure or skipping playback cannot fabricate progression unless
the product definition explicitly grants credit at another accepted step.

Navigation assistance, radar visibility, camera preferences, input preferences,
and similar player options belong to the device or platform-user configuration
profile. Progression may query those settings when projecting guidance, but it
cannot store them as campaign completion or reinterpret them during save
migration.

### Vehicle damage and character presentation

Vehicle damage state belongs to the vehicle and save runtimes. Null, undamaged,
damaged, disabled, and destroyed or husk-like presentation states are typed
vehicle conditions, not fields owned by campaign progression.

Costume ownership and equipped presentation belong to character/costume state.
The progression projection may count ownership but cannot change the active
complete-model presentation, animation, or semantic FBX preparation.

### Save and migration

Each domain contributes one versioned save section with canonical identities and
accepted transaction revisions. Save composition validates every section before
commit and publishes one terminal result.

A legacy compact sheet may be imported only through an explicit converter that:

1. validates source schema and bounds;
1. maps mission, race, card, gag, wasp, vehicle, costume, tutorial, and media
   ordinals to canonical identities;
1. maps mission attempts, skips, best results, optional-objective outcomes,
   purchases, vehicle health, persistent world-object bits, and registered
   global state bits into their owning domain schemas;
1. separates active checkpoint state from completed state;
1. derives chapter and game projections from accepted identities;
1. records unsupported or ambiguous entries as findings;
1. validates cross-domain invariants; and
1. commits one migrated save revision atomically.

The converter cannot guess missing identities from counts, display names, array
positions, or the highest mission number.

### Queries

Queries return immutable projections with catalog, save, and domain revisions.
Initial query families include:

- mission and chapter availability;
- mission attempts, skip state, optional-objective result, and best evidence;
- story completion;
- side-activity completion;
- card-set and collectible completion;
- vehicle and costume ownership;
- tutorial and media state;
- exact chapter and game progress;
- 100-percent eligibility; and
- achievement reachability.

A query cannot mutate progression, load presentation, or repair missing data.
Cached projections invalidate by revision and never become durable authority.

### Character-sheet failure behavior

Projection or migration fails closed on:

- unknown or duplicate canonical identity;
- count and identity-set disagreement;
- impossible mission or chapter ordering;
- active checkpoint for a completed or unavailable mission;
- cross-domain transaction mismatch;
- stale catalog or save revision;
- unmapped legacy ordinal or bit position;
- ambiguous tutorial, media, vehicle, costume, or collectible record; or
- a percentage that cannot be reproduced from exact identity sets.

Failure preserves the prior accepted save. It does not clamp counts, advance the
highest mission, or silently mark content complete.

## Achievement projection

Achievements are required but pending implementation. The schema records stable
identity, exact predicate, counters, platform mapping, replay route, no-missable
reachability, mod compatibility, presentation, and accepted progress revision.

Base achievement families include chapter completion, cards, wasps, costumes,
coin-total milestones, side missions, taxi milestones, 100 percent completion,
shortcuts, per-mission no-death records, major world expansions, purchases, and
cumulative humorous actions. Every base condition remains obtainable through
free roam, mission replay, side-activity replay, or post-game play.

A mission no-death achievement is tracked per mission and can be retried.
Current coin achievements observe an accepted balance threshold and do not
consume
currency. Mods declare base-compatible, base-incompatible, or namespaced custom
achievement policy.

## Objective integration

The mission objective taxonomy distinguishes progression mutation from world
presentation.

A `collect` step declares:

- ordered or unordered target identities;
- required and optional counts;
- whether missed moving targets may fail the step;
- placement-consumption policy;
- timer and recovery policy; and
- the exact transition after the required count is accepted.

A `destroy` step declares:

- one or more target identities;
- accepted damage source rules;
- completion at validated destruction, not disappearance;
- optional destination or timer failure conditions;
- target respawn policy after mission restart; and
- whether dropped mission items become a following collect step.

World streaming, target despawn, actor replacement, or client timeout cannot
silently complete either objective.

## Cheat input model

A cheat sequence contains exactly four logical input tokens. Logical tokens are
mapped to physical buttons or keys through the active platform input profile.
The cheat definition never stores a platform-specific key code as its identity.

`FSharCheatDefinitionRow` contains:

<!-- markdownlint-disable MD013 -->

| Field | Contract |
| :--- | :--- |
| `CheatId` | Stable logical effect identity. |
| `InputTokens` | Exactly four logical input tokens. |
| `Prerequisite` | None, loaded profile, completed story, or developer build. |
| `ActivationMode` | Enable-only, toggle, or immediate command. |
| `Lifetime` | Session, current chapter boundary, current mission, or persistent transaction. |
| `EffectKind` | Controlled effect taxonomy. |
| `EffectParameters` | Typed parameters owned by the receiving subsystem. |
| `FeedbackEvent` | Success, unavailable, disabled, or invalid-sequence feedback. |

<!-- markdownlint-enable MD013 -->

Input recognition is controller-scoped. A controller must explicitly activate
cheat entry before its four-token sequence is accepted. Duplicate sequences and
prefix ambiguity are invalid catalog data.

### Recognition and activation

Each local player owns one transient recognizer with `inactive`, `armed`,
`collecting`, `accepted`, and `rejected` states. The active input profile maps
physical controls to four semantic cheat tokens plus one explicit activation
chord or action. Platform button labels are presentation only.

Arming clears the previous sequence and records the local-player, controller,
input-profile, and catalog revisions. Releasing the activation chord, changing
controller ownership, changing application mode, opening an incompatible modal
UI, suspending the application, or reaching a timeout clears the transient
sequence.

Exactly four accepted token-down transitions are evaluated. Key repeat, analog
noise, button-up events, duplicate device delivery, and another local player's
input cannot advance the recognizer. Recognition returns one of `matched`,
`unknown_sequence`, `unavailable`, `prerequisite_failed`, or `input_cancelled`.

Sequence lookup is a generated map from the four-token tuple to one canonical
cheat identity. It does not convert the tuple into an array index, depend on
cheat
enum order, or broadcast through a fixed callback list.

A matched definition is sent to `USharCheatEffectSubsystem`, which validates the
prerequisite and delegates the typed effect request to its owning application
port. Subscribers receive immutable result observations after the effect reaches
its declared postcondition. Listener order cannot change activation.

The success or failure presentation is local-player scoped and cannot reveal
unavailable developer-only cheats in an ordinary player build.

## Cheat state and effects

Session cheat state is a typed set of enabled cheat identities. It resets when
the gameplay session resets. Runtime systems subscribe to typed effect-change
events rather than polling an unstructured global flag.

The verified effect taxonomy includes:

- display registered cheats;
- unlock cards, costumes, missions, movies, vehicles, or all unlock families;
- remove the vehicle top-speed limit;
- increase acceleration;
- jump the vehicle from the horn action;
- apply full damage or one-hit traffic destruction;
- add mission time;
- show avatar position or a speedometer;
- toggle the controlled character presentation;
- grant 100 coins through an explicit persistent currency transaction;
- unlock alternate cameras;
- enter the bounded demonstration flow;
- enable credits dialogue;
- enable the redbrick vehicle;
- make the current vehicle invincible;
- show the diagnostic scene tree; and
- enable the trippy rendering presentation.

Unlock effects form a session overlay over persisted progression. They do not
rewrite card, costume, mission, movie, or vehicle ownership records. The
`unlock_all` effect enables every individual unlock overlay through the same
validation and notification path.

Final player builds require completed story progression before card, costume,
or vehicle unlock overlays become available. Developer builds may define a
separate prerequisite profile, but that profile cannot ship accidentally as the
player default.

Immediate commands such as the 100-coin grant or demonstration transition are
not ordinary toggles. Their definition specifies whether each successful input
may execute again. Every persistent result is an explicit transaction with a
cheat-origin marker so tests and save diagnostics can distinguish it from normal
progression.

## Credits sequences

Credits are deterministic presentation sequences with two entry modes:

<!-- markdownlint-disable MD013 -->

| Sequence | Entry | Return contract |
| :--- | :--- | :--- |
| `front_end_credits` | Explicit front-end selection | Return to the invoking front-end state. |
| `post_ending_credits` | Final mission and ending transition | Continue to the declared post-ending front-end state. |

<!-- markdownlint-enable MD013 -->

`FSharCreditsSequenceRow` contains ordered text identity, role category,
presentation style, scroll timing, optional dialogue cue, optional music cue,
optional sound-mix override, skippability, and minimum display policy.

Final progression commits before post-ending credits begin. Skipping,
interrupting, or failing to load credits presentation cannot roll back mission
completion or grant it a second time.

The credits-dialogue cheat changes only the dialogue-cue selection for the
credits presentation. It does not select a different progression path.

## Calendar presentation and Easter eggs

Calendar themes are presentation-only definitions selected from a normalized
local calendar date. They may override front-end meshes, materials, lighting,
props, and audio through soft references.

The verified rules include:

- `christmas_menu` on December 25;
- `halloween_menu` on October 31; and
- `thanksgiving_menu` through an explicit locale-independent calendar predicate
  stored in the generated definition.

A calendar theme cannot change gameplay definitions, save progression, mission
availability, currency, physics, collision, or catalog identity. An unavailable
theme falls back to the ordinary menu without failing profile load.

## Environmental-reference rows

Intentional environmental references use `FSharEnvironmentalReferenceRow` with:

<!-- markdownlint-disable MD013 -->

| Field | Contract |
| :--- | :--- |
| `ReferenceId` | Stable presentation identity. |
| `LevelIds` | Exact level contexts in which it is observable. |
| `LocationId` | Canonical owning location. |
| `PlacementId` | Deterministic world placement or ambience zone. |
| `ReferenceKind` | Prop, texture, architecture variant, ambience, pedestrian, gag, or collectible metadata. |
| `PresentationAssets` | Soft references loaded through the owning location bundle. |
| `InteractionPolicy` | None, inspect, activate, destroy, collect, or ambience trigger. |
| `ProgressionEffect` | Explicit reward identity or none. |
| `LegacyStatus` | Intentional, accidental out-of-bounds, or unused. |

<!-- markdownlint-enable MD013 -->

The verified intentional presentation set includes:

- three portrait-parody placements associated with the tunnel environment;
- one national-flag placement near the ship bow;
- Halloween-world child-voice ambience in the park and schoolyard zones;
- a lighthouse machine prop, a ship-crate gag, and a lighthouse silhouette cue;
- a recurring elderly pedestrian presentation in the declared level contexts;
- Halloween-world variants for the shortcut house, school, and animated store
  mascot; and
- episode-reference metadata on every collector-card definition.

These rows may load art or audio and may expose a declared interaction, but they
cannot alter progression unless `ProgressionEffect` names an ordinary reward
transaction.

Verified accidental out-of-bounds or unused records include a floating appliance
and adjacent face prop, two unused environment themes, and decorative rooftop
cars outside ordinary play. They remain editor-review metadata with their
historical level context. Production world composition does not restore them as
reachable gameplay, spawn them into active cells, grant their incidental coin
behavior, or treat boundary escape as a supported route.

A descriptive source list is not an executable registry. Every retained runtime
reference must resolve through a location, placement, asset bundle, and explicit
interaction policy.

## Save integration

Persistent destructible, removable, consumable, and variant placement state
follows
<!-- markdownlint-disable-next-line MD013 -->
the [persistent world-object state
runtime](persistent-world-object-state-runtime.md).

Portable progression stores:

- accepted coin balance and transaction revision;
- consumed persistent currency-source placement identities;
- collected card identities and completed set identities;
- granted reward, vehicle, costume, bonus-map, and movie-ticket identities;
- viewed movie identities when viewing has separate significance;
- purchase identities and prices at the accepted catalog revision; and
- cheat-origin persistent transactions.

Session cheat toggles, active input sequences, credits playback position,
calendar theme, transient coin drops, presentation animation state, and physical
asset paths are not portable progression.

Migration resolves every identity through explicit catalog redirects. Missing
collectible, reward, purchase, or currency definitions block migration rather
than deleting state.

## Invariants

- Coin balance never becomes negative.
- One accepted transaction identity changes the ledger at most once.
- A persistent world source pays at most once per save.
- A destructible's staged payouts sum to its declared total.
- A completed purchase debits and grants atomically.
- There are exactly seven collector-card sets with seven cards each in the
  verified base catalog.
- A collected card belongs to exactly one set and one chapter.
- One completed card set grants its declared passive and any retained bonus map.
- All 49 cards are required for the movie-ticket reward.
- Cheat sequences contain exactly four logical tokens.
- Session cheat overlays do not replace persisted ownership.
- Credits playback cannot mutate progression.
- Calendar themes are presentation-only.
- Platform and graphics-preset differences cannot rekey progression identities.

## Failure behavior

The progression layer fails closed on:

- duplicate transaction, collectible, set, placement, reward, purchase, or cheat
  identities;
- invalid negative balances or arithmetic overflow;
- one-time source replay;
- payout stages that exceed or fail to reach the declared total;
- purchase debit without a resolvable grant;
- duplicate card ordinals, incomplete level decks, or a card in multiple sets;
- a set reward that targets the wrong level;
- a four-token cheat sequence with an invalid token, duplicate mapping, or
  unsatisfied prerequisite;
- an effect handled by no owning subsystem;
- credits rows with nondeterministic order or invalid return state;
- a calendar rule that changes non-presentation state; or
- read-back state that differs from the staged transaction batch.

Failure leaves the last accepted save revision and runtime catalog unchanged.

## Verification

Pure domain tests verify:

- transaction ordering, overflow, clamping, and duplicate rejection;
- persistent and transient source lifetime;
- 30-coin crate and 15-coin vending-machine totals;
- 50-coin police fines and 10-coin repair charges;
- atomic purchase success and rollback;
- seven-by-seven card membership and 49-card total;
- idempotent card collection;
- one matching bonus-map reward per completed deck;
- movie-ticket eligibility only after all 49 cards;
- collect and destroy objective completion versus despawn or streaming;
- four-token cheat recognition and duplicate-sequence rejection;
- story-completion prerequisites for player-build unlock effects;
- `unlock_all` expansion through individual effect events;
- one 100-coin transaction per accepted immediate command;
- credits entry, skip, interruption, and return contracts; and
- presentation-only calendar themes.

Editor and runtime integration tests verify:

- every generated row resolves its canonical catalog references;
- placement identities survive World Partition streaming and save reload;
- destructible presentation never duplicates accepted currency;
- card gallery state is derived from progression rather than copied;
- purchase screens display the price committed by the transaction;
- platform input mappings produce the same logical cheat identities;
- session cheat state resets at the declared boundary;
- post-ending credits begin only after the ending transition commits;
- front-end credits return to the invoking state; and
- cooked targets preserve identical progression and meta definitions.

## Known limits

This specification establishes the complete architecture and the verified
currency, collector-card, destructible-source, cheat, credits, and calendar
slice. Gags, wasp behavior, wager races, every purchase offer, every credits
row,
and every environmental reference extend these contracts when their coverage
entries are reviewed; they do not introduce parallel state models.
