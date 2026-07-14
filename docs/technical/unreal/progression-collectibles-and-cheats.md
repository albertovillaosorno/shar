# Progression, collectibles, cheats, and credits

- Status: Active
- Last reviewed: 2026-07-14

## Governing decisions

- [Canonical seven-level campaign and world variants](../../adr/unreal/runtime/canonical-seven-level-campaign-and-world-variants.md)
- [Common UI front end and progress projection](../../adr/unreal/ui/common-ui-frontend-and-progress-projection.md)
- [Collector cards, coins, rewards, gags, and wasps](../../adr/gameplay/collectibles/collectibles-rewards-gags-and-wasps.md)
- [Data-driven Unreal gameplay content catalog](../../adr/unreal/runtime/data-driven-gameplay-content-catalog.md)
- [State-driven missions, interactions, interiors, and notoriety](../../adr/unreal/runtime/state-driven-missions-interactions-and-notoriety.md)
- [Runtime parity boundary](../../adr/unreal/runtime/remake-parity-boundary.md)
- [Shared runtime tagging, modding, and platform compatibility](../../adr/unreal/runtime/shared-runtime-tagging-modding-and-platform-compatibility.md)
- [Portable save storage and lifecycle](../../adr/unreal/runtime/portable-save-storage-and-lifecycle.md)

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

Definitions and tables live under the catalog's existing data roots. Secondary
icons, meshes, sounds, sequences, and menu presentation remain soft references
in the catalog's art, audio, media, and user-interface roots.

## Identity contract

Canonical identifiers are stable lowercase `snake_case` values.

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

A definition identity and a placement identity are never interchangeable. One
collectible definition may have one placement, while one destructible source
definition may have many separately consumed placements.

## Currency ledger

The coin balance is a signed 64-bit domain value constrained to a non-negative
accepted state. Every change is an immutable `FSharCurrencyTransaction` with:

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

The accepted balance is the previous accepted balance plus the ordered
transaction batch. Validation occurs before any presentation pickup, sound, or
counter animation is shown as successful.

Coins transfer unchanged across level transitions. A level reload does not
create a new balance and cannot replay accepted one-time source transactions.

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

## Gag completion

A gag is a level-scoped interaction and progression record. Its presentation may
reuse a prop, animation, sound, or gag concept across world variants, but its
completion key is the ordered pair of `LevelId` and `GagPlacementId`.

`FSharGagProgressRow` contains:

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
the completion or reward. The progression service validates the level,
placement, current completion state, and reward policy before committing one
transaction.

A repeated gag concept in reused world geometry receives a distinct placement
and completion key only when it is declared for that level variant. Reusing the
same actor identity across variants is invalid. An interior gag follows the same
ledger and remains level-scoped even when the interior geometry is shared.

Most gag definitions may grant currency, but the reward is explicit rather than
implied. A gag with no declared reward still records completion when it counts
toward level progress. A special reward amount is represented by its own reward
policy and does not change the gag identity.

After completion, a replayable gag may repeat presentation without replaying the
currency transaction, completion key, statistics event, or level-progress
increment. Streaming, mission restart, level reload, save reload, or entering a
second portal to the same interior cannot duplicate the accepted result.

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

Presentation may animate multiple coin objects, but the ledger remains the
single balance authority.

## Purchases

Vehicle and costume purchases use one atomic transaction:

1. resolve the canonical offer and seller context;
1. verify level, progression, availability, and current ownership;
1. verify the accepted balance covers the exact price;
1. stage the debit and reward unlock together;
1. validate that the granted definition resolves through the catalog;
1. commit the debit and unlock in one save revision; and
1. publish presentation only after commit succeeds.

Failure leaves both balance and ownership unchanged. Repeating a completed
purchase returns the existing ownership result and never charges twice.

The current authored vehicle-and-costume purchase census requires 14,800 coins
for complete purchase progression. This total is verification evidence, not a
hardcoded economy rule. A generated catalog revision must explain any changed
total through changed offers.

## Collector-card definitions

Collector cards are not generic currency pickups. `FSharCollectibleRow` records:

| Field | Contract |
| :--- | :--- |
| `CollectibleId` | Stable card identity. |
| `SetId` | Owning level deck. |
| `SetOrdinal` | Dense ordinal within the deck. |
| `LevelId` | Canonical level identity. |
| `DisplayName` | Localizable gallery name. |
| `Image` | Soft gallery image reference. |
| `QuoteEvents` | Optional ordered collection-response references. |
| `CompletionWeight` | Explicit scrapbook contribution. |

There are seven collector-card definitions in each of seven level sets, for 49
cards total. Set ordinals are unique and dense from one through seven.

## Verified collector-card deck membership

Each ordered display name below binds to the stable ordinal identity
`collector_card_level_<level>_<ordinal>`. Localized display text and quote events
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

Completing all seven cards in a level performs one atomic transition:

- mark the level card set complete;
- grant the matching bonus-race map reward;
- update scrapbook completion; and
- publish the card-set completion event.

Completing one level set cannot grant another level's map. Removing or replacing
presentation assets does not revoke accepted card state.

Collecting all 49 cards enables the movie-ticket reward transaction. Granting
the ticket contributes to complete progression. Entering the associated movie
presentation is a separate state transition; viewing the movie is not inferred
merely from ticket ownership.

## Level and game progress projection

The exact eight-category level formula, seven-level aggregation, counted vehicle
roles, level denominators, and one-percent movie contribution are owned by
[campaign level composition and progress](campaign-level-composition-and-progress.md).
The complete 42-vehicle ownership census and the distinction between persistent,
traffic, secret, mission, completion, and development access follow
[Vehicle access and roster runtime](vehicle-access-and-roster-runtime.md).

Progression state provides accepted identity sets and transaction results. The
campaign service calculates exact rational progress. Scrapbook, save-slot, and
pause-menu widgets consume the same immutable projection and never recalculate
it from visible entries. Exact scrapbook modes and gallery membership follow
[Frontend shell and menu runtime](frontend-shell-and-menu-runtime.md).

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

| Field | Contract |
| :--- | :--- |
| `CheatId` | Stable logical effect identity. |
| `InputTokens` | Exactly four logical input tokens. |
| `Prerequisite` | None, loaded profile, completed story, or developer build. |
| `ActivationMode` | Enable-only, toggle, or immediate command. |
| `Lifetime` | Session, current level, current mission, or persistent transaction. |
| `EffectKind` | Controlled effect taxonomy. |
| `EffectParameters` | Typed parameters owned by the receiving subsystem. |
| `FeedbackEvent` | Success, unavailable, disabled, or invalid-sequence feedback. |

Input recognition is controller-scoped. A controller must explicitly activate
cheat entry before its four-token sequence is accepted. Duplicate sequences and
prefix ambiguity are invalid catalog data.

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

| Sequence | Entry | Return contract |
| :--- | :--- | :--- |
| `front_end_credits` | Explicit front-end selection | Return to the invoking front-end state. |
| `post_ending_credits` | Final mission and ending transition | Continue to the declared post-ending front-end state. |

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
- A collected card belongs to exactly one set and one level.
- One completed card set grants exactly one matching bonus map.
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
slice. Gags, wasp behavior, wager races, every purchase offer, every credits row,
and every environmental reference extend these contracts when their coverage
entries are reviewed; they do not introduce parallel state models.
