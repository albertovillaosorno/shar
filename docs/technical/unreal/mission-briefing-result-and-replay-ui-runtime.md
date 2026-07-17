# Mission briefing, result, and replay UI runtime

- Status: Active
- Last reviewed: 2026-07-15

## Governing decisions and specifications

<!-- markdownlint-disable-next-line MD013 -->
- [State-driven missions, interactions, interiors, and notoriety](../../adr/unreal/runtime/state-driven-missions-interactions-and-notoriety.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Common UI front end and progress projection](../../adr/unreal/ui/common-ui-frontend-and-progress-projection.md)
- [UI parity boundary](../../adr/unreal/ui/ui-parity-boundary.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Common UI navigation, menu, and modal runtime](common-ui-navigation-menu-and-modal-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [In-game HUD, pause, and transition runtime](in-game-hud-pause-and-transition-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Mission definition, stage, and objective runtime](mission-definition-stage-and-objective-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Mission, interaction, interior, and notoriety runtime](mission-interaction-and-notoriety-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Progression, collectibles, cheats, and credits](progression-collectibles-and-cheats.md)

## Purpose

This specification defines the native Unreal user-interface runtime for mission
briefings, mission loading, wager presentation, mission completion, mission
failure, retry and skip choices, chapter completion, chapter statistics, and
mission replay selection.

It preserves observable mission presentation while replacing screen-owned
mission state, fixed mission arrays, filename-derived artwork, random failure
hints, widget-driven reloads, platform-specific text branches, and transition
callbacks that can complete a superseded mission request.

## Native Unreal composition

The runtime uses:

- Common Activatable Widgets for briefing, loading, result, statistics, and
  replay screens;
- Common UI action data for confirm, cancel, retry, abort, skip, chapter change,
  and mission selection;
- a game-instance mission-presentation subsystem for flow and request ownership;
- C++ UMG viewmodels for immutable briefing, result, and replay projections;
- Asset Manager primary assets and named bundles for mission art, icons, audio,
  cameras, and transition presentation;
- retained streamable handles for accepted mission-presentation leases; and
- the shared fade, iris, letterbox, navigation, and modal transactions.

A widget never starts, completes, fails, restarts, skips, or saves a mission.
Those operations remain typed commands to the mission and application services.

## Ownership

<!-- markdownlint-disable MD013 -->

| Service | Authority |
| :--- | :--- |
| `USharMissionPresentationSubsystem` | Presentation flow, mission screen requests, accepted presentation revision, and screen restoration. |
| `USharMissionPresentationViewModelSubsystem` | Immutable briefing, result, statistics, and replay viewmodels. |
| Mission application service | Mission definition, load, start, stage, completion, failure, retry, abort, skip eligibility, and terminal result. |
| Progression service | Attempt counts, completion, skipped state, unlocks, chapter reach, best results, and save revision. |
| Economy service | Wager entry fee, balance, payout, and linked currency transactions. |
| Asset-load service | Required mission, presentation, audio, and world bundle readiness. |
| Application lifecycle service | Loading, gameplay, pause, frontend, and chapter-transition modes. |
| Common UI kernel | Screen activation, focus, actions, history, modals, and transition leases. |

<!-- markdownlint-enable MD013 -->

Mission art, text, camera motion, button labels, and result animations are
projections. They cannot become mission or progression authority.

## Runtime identities

The runtime uses:

- `FSharMissionPresentationRequestId` for one briefing or result flow;
- `FSharMissionSessionId` for the mission execution being presented;
- `FSharMissionDefinitionId` for canonical mission identity;
- `FSharMissionPresentationProfileId` for visual and audio presentation;
- `FSharMissionLoadPlanId` for correlated asset and world readiness;
- `FSharMissionResultId` for one terminal mission result;
- `FSharMissionFailureCategoryId` for typed failure classification;
- `FSharMissionReplayRequestId` for one replay selection; and
- exact mission, progression, economy, catalog, world, and application
  revisions.

A callback lacking the accepted request, session, and revision is stale and
cannot navigate, reveal input, publish a result, or release another request's
assets.

## Mission presentation definition

`FSharMissionPresentationDefinition` contains:

<!-- markdownlint-disable MD013 -->

| Field | Contract |
| :--- | :--- |
| `MissionId` | Canonical mission identity. |
| `PresentationProfileId` | Normal, race, bonus, wager, tutorial, or registered feature profile. |
| `TitleTextId` | Localized title identity. |
| `BriefingTextId` | Localized briefing identity. |
| `HintSetId` | Optional localized hint set. |
| `ArtworkId` | Optional mission artwork or animated presentation identity. |
| `FlagId` | Optional chapter or mission marker presentation. |
| `CameraPresentationId` | Optional camera sequence or static camera policy. |
| `AudioPolicyId` | Briefing mix, cue, voice, and restoration policy. |
| `TransitionPolicyId` | Enter, readiness barrier, exit, and reduced-motion behavior. |
| `InputPolicyId` | Confirm, cancel, skip, and hold requirements. |
| `RequiredBundles` | Presentation, mission, audio, and world dependencies. |
| `FailurePresentationMapId` | Typed failure categories and localized reason data. |
| `ReplayPolicyId` | Visibility, eligibility, best-result fields, and load behavior. |
| `FeatureOwnerId` | Base game or validated feature package. |

<!-- markdownlint-enable MD013 -->

Definitions reject duplicate mission identities, unresolved text, missing
required bundles, unsupported actions, invalid transition graphs, and result
mappings that omit a possible failure category.

Historical mission-title, hint, and icon tables are import evidence only. Intake
resolves one canonical mission or failure category, one selected localized title
or reason, one bounded hint set, and one registered icon or cue identity with
accessible text and fallback policy.

Proposal columns remain alternatives until an explicit selection is recorded.
Source row order, color keys, item descriptions, filenames, blank cells, and
spreadsheet headings are not runtime identity. Duplicate tables collapse by
content digest and semantic identity; changed revisions are compared field by
field. Missing required text or icon mappings fail publication rather than
selecting the first non-empty source cell.

## Presentation states

One request is in exactly one state:

- `created`;
- `validating`;
- `loading`;
- `briefing_intro`;
- `briefing_ready`;
- `starting_mission`;
- `result_intro`;
- `result_ready`;
- `replay_selecting`;
- `leaving`;
- `failed`;
- `cancelled`; or
- `completed`.

The request has one terminal result. Loading completion, animation completion,
world readiness, and user confirmation are separate observations.

## Briefing request

A briefing request contains:

- mission, chapter, session, and presentation identities;
- source application and progression revisions;
- mission type and optional wager definition;
- required load plan;
- accepted player and vehicle bindings;
- checkpoint or replay policy;
- input and cancellation policy; and
- destination on success or failure.

The request validates the complete mission definition before activating a
screen. Missing required content fails before the current stable screen is
replaced.

## Briefing variants

The base presentation profiles are:

- `normal`, for campaign missions;
- `race`, for races and route competitions;
- `bonus`, for optional missions;
- `wager`, for missions with an entry fee and payout policy;
- `tutorial`, for the introductory mission flow; and
- `special`, for an explicitly registered mission-specific presentation that
  changes presentation only.

Profiles may vary color, art, title treatment, audio mix, camera, layout, and
transition timing. They cannot change mission objectives, reward, failure,
progression, or save behavior.

## Mission artwork and camera presentation

Mission artwork is referenced by canonical presentation identity and soft asset
reference. It may be static, animated, or absent. The system never constructs a
filename from chapter and mission numbers.

Artwork loading uses an Asset Manager bundle and a retained handle. A failed
optional artwork load uses the profile's fallback. A failed required artwork or
camera load blocks the request with a typed content error.

Camera presentation is a scoped lease. It may play an authored camera sequence
or hold a static composition during briefing. Exiting the screen restores the
prior camera policy even after cancellation or feature removal.

## Loading transaction

Mission loading follows this sequence:

1. validate request identity, mission definition, and source revisions;
1. reserve the briefing layer and transition lease;
1. request mission, world, audio, and presentation bundles;
1. begin the application loading mode;
1. build the briefing viewmodel from accepted mission data;
1. activate the briefing screen without enabling confirmation;
1. wait for all required load-plan members and world readiness;
1. publish the accepted ready revision;
1. reveal the configured confirmation action; and
1. start gameplay only after the player confirms or the profile authorizes
   automatic continuation.

A loading label or animation does not prove readiness. Progress is derived from
load-plan members with explicit weights, not memory use or file count.

## Load completion and confirmation

The ready viewmodel contains:

- mission title and briefing text;
- mission type and chapter presentation;
- artwork and camera readiness;
- load progress and terminal readiness;
- optional wager information;
- confirm and cancel availability;
- semantic glyph context; and
- accessibility timing state.

Confirmation is ignored until the accepted ready revision is visible and focus
is established. Duplicate confirmation is idempotent.

A cancellable briefing may return to mission selection or the prior stable
screen. Cancellation releases only the request-owned assets and never cancels an
unrelated world or feature load.

## Wager presentation

A wager viewmodel contains entry fee, current balance, payout policy, vehicle or
participant odds presentation, and eligibility reason.

The screen does not debit currency. Entry fee is staged and committed by the
mission start transaction. Insufficient currency, stale balance, invalid odds,
or unavailable participant state rejects start and leaves the accepted balance
unchanged.

Odds labels are derived from typed difficulty bands or exact policy data. A
color or layout cannot define wager difficulty.

## Audio behavior

Briefing start requests a scoped audio-mix lease. The lease may attenuate world
sound, play a briefing cue, and preserve voice or music according to profile.
Mission start, cancellation, and failure restore the prior accepted mix exactly
once.

No screen directly stops or resumes unrelated audio channels.

## Mission start handoff

After confirmation, the subsystem:

1. revalidates mission, load, player, vehicle, economy, and world revisions;
1. commits any wager entry transaction;
1. obtains the mission start result from the mission service;
1. closes the briefing through the declared transition;
1. transfers required asset leases to mission execution;
1. restores the gameplay camera and audio policies; and
1. publishes one completed presentation result.

Failure restores the stable briefing or prior screen with typed recovery. It
cannot enter gameplay with a partially committed wager or incomplete world.

## Mission result envelope

`FSharMissionResultPresentation` contains:

- terminal result identity;
- mission and session identities;
- completed, failed, aborted, skipped, or cancelled outcome;
- failure category and reason when applicable;
- accepted attempt count;
- retry, abort, and skip eligibility;
- progression and economy commit revisions;
- earned reward and best-result projection;
- next destination policy; and
- presentation profile and required bundles.

The screen renders only a terminal result already accepted by the mission
service. It cannot infer success from an objective widget or failure from a
timer reaching zero.

## Failure categories

The base failure taxonomy includes:

- vehicle destroyed or damage limit exceeded;
- player defeated;
- time expired;
- player remained outside the required vehicle;
- follow-distance constraint failed;
- objective or player left allowed bounds;
- race condition failed;
- protected target was abducted or not preserved;
- required position was not maintained;
- notoriety arrest occurred; and
- required collectibles were not obtained.

A mission may declare additional namespaced categories. Every possible failure
condition must map to one localized reason and zero or more validated hints.
Unknown categories fail closed into a generic recoverable presentation while
preserving the typed diagnostic; they never index arbitrary text.

## Failure hints

Hint sets are ordered data associated with a failure category. The base content
may provide up to eight hints per category, but the schema accepts any bounded
validated count.

Hint selection is deterministic. The selector hashes mission identity,
failure category, accepted attempt count, and hint-set revision, then selects
one member. Repeated generation and replay therefore produce stable results
while still varying hints across attempts. Process-global randomness is
forbidden.

An empty hint set hides the hint region without hiding the failure reason.

## Retry, abort, and skip choices

Retry restarts from the mission's declared restart policy or checkpoint. Abort
returns to the declared sandbox or frontend destination. Both are typed mission
commands and may require confirmation.

The base campaign skip policy becomes eligible after seven accepted failed
attempts for a skippable mission. Terminal missions in the final campaign
sequence remain non-skippable. Mission definitions may be stricter, but a mod
cannot make a base non-skippable terminal mission skippable without an explicit
validated override.

Skip is a progression transaction. The result screen may expose the action, but
only the mission and progression services can commit skipped state, choose the
next mission, and save the new revision.

## Mission success presentation

Success presentation may show mission-specific art, title, reward, best-result,
or special character presentation through a registered profile. Special
presentation overrides art and layout only.

Confirmation or back accepts the declared continuation policy after progression
commit is verified. A success animation cannot grant the reward or advance the
campaign.

## Chapter completion

Chapter completion is a separate terminal projection. It appears only after the
progression service accepts the chapter-complete revision.

The projection may offer continuation, chapter statistics, mission replay, or
return to sandbox according to campaign policy. It cannot mark the next chapter
reached merely because the player closed the screen.

## Chapter statistics

`FSharChapterStatisticsViewModel` contains:

- chapter identity and progression revision;
- completed and total campaign missions;
- bonus mission completion;
- completed and total street races;
- collected and total cards;
- unlocked and total outfits;
- unlocked and total vehicles;
- destroyed and total flying hazards;
- viewed and total gags; and
- exact chapter-completion value plus localized display text.

Counts come from progression services and catalog totals. Widgets do not derive
totals from visible rows. Percentage formatting is presentation only and cannot
round a chapter into completion.

## Mission replay catalog

The replay screen consumes an immutable catalog of chapters and regular mission
entries. Each entry contains:

- chapter and mission identities;
- localized mission number and title;
- locked, available, attempted, failed, completed, or skipped state;
- best time and optional player initials or profile label;
- required replay load plan;
- replay restrictions and unavailable reason; and
- catalog and progression revisions.

Ordering comes from the campaign catalog. The seven base chapters and their
regular mission ordering are data, not fixed widget arrays. The introductory
tutorial remains a distinct mission identity and does not shift public mission
identity.

## Replay availability and status

Replay availability is based on reached chapter, highest accepted mission,
completion, explicit replay policy, and effective development or cheat override.
A widget cannot unlock a mission by making its row visible.

The base status presentation distinguishes:

- not attempted;
- attempted and failed;
- completed; and
- skipped.

Color and icons are presentation profiles; accessible text exposes the same
state.

## Replay transaction

Selecting a replay performs:

1. validate screen, catalog, progression, and application revisions;
1. validate replay eligibility and active mission policy;
1. reserve a replay request identity;
1. stop or transfer presentation-only dialogue through the dialogue service;
1. request the mission load plan;
1. leave the current gameplay session through the lifecycle service;
1. activate the selected mission session only after readiness; and
1. publish one replay result or restore the prior stable screen.

Replay progression policy is explicit. A replay cannot accidentally overwrite
campaign position, current mission, rewards, or best results.

## Transition choreography

Briefing and result screens use registered transition graphs composed from fade,
iris, letterbox, artwork, title, and content motions. Every node declares start,
completion, timeout, cancellation, and reduced-motion fallback.

The graph is data-driven and dynamically sized. A fixed transition array or
mutable pending counter is forbidden. The graph completes only when all required
nodes and readiness barriers complete.

## Localization and accessibility

Titles, reasons, hints, statistics, wager text, result actions, and status
labels use localized text identities and typed arguments. Layout supports text
expansion, bidirectional text where enabled, and locale-specific capitalization
policy.

Every screen declares focus order, narration, color-independent status,
reduced-motion behavior, minimum result-reading duration, touch targets, and
safe-area constraints.

## Feature and mod overlays

A validated feature package may add missions, failure categories, hint sets,
presentation profiles, or replay entries through namespaced definitions.

Feature removal cancels owned requests, releases owned assets, removes owned
catalog entries, and restores a valid base screen. It cannot leave a replay row
or result action referring to removed content.

## Concurrency

Mission, progression, economy, and application mutations are serialized through
their application services. Asset loading and presentation transitions may
complete asynchronously, but only the owning request may accept them.

A replacement mission request cancels the prior presentation request before
activation. Results from the prior session remain diagnostic evidence and cannot
navigate the replacement flow.

## Diagnostics

The subsystem records bounded structured diagnostics for:

- presentation request, mission session, result, and replay identities;
- accepted source revisions;
- presentation profile and load-plan membership;
- asset readiness and retained leases;
- transition-node state and timeout;
- failure category and selected hint identity;
- retry, abort, and skip eligibility reasons;
- progression, wager, and replay transaction results; and
- stale or rejected callbacks.

Diagnostics never contain raw local asset paths or machine-specific locations.

## Failure behavior

- Missing required mission definition or text blocks presentation before
  replacing the stable screen.
- Missing optional artwork uses the profile fallback.
- Required load failure returns a typed error and preserves the prior mission
  and economy state.
- Invalid failure category uses a recoverable generic presentation and records a
  validation defect.
- Empty hint sets hide hints without hiding the failure reason.
- Failed retry, abort, skip, or replay commands leave the accepted result screen
  active with a typed reason.
- Failed wager commit cannot enter gameplay.
- Transition timeout restores the prior stable screen or a declared recovery
  screen.
- Feature removal cannot leave unresolved actions or retained assets.

## Validation

Validation proves:

- every mission has one presentation definition or declared fallback;
- every possible mission failure condition maps to a reason;
- historical title proposals have one explicit selected result or remain
  unpublished;
- hint sets contain valid localized identities and bounded membership;
- every required mission icon resolves one registered cue identity, accessible
  text, and fallback policy;
- source proposal columns, color keys, descriptions, row order, and blank cells
  create no presentation identity;
- retry, abort, skip, and replay policies resolve to mission commands;
- base skip thresholds and terminal exclusions are represented correctly;
- chapter and mission ordering contain no duplicate identities;
- statistics fields resolve to progression queries and catalog totals;
- required assets, actions, viewmodels, and transition nodes resolve;
- replay load plans are complete; and
- feature-owned definitions are namespaced and removable.

## Tests

Automated tests cover:

- normal, race, bonus, wager, tutorial, and special briefing profiles;
- historical title-proposal selection, no-selection rejection, duplicate-table
  collapse, and changed-revision comparison;
- mission-hint import, deterministic bounded membership, empty-set behavior, and
  failure-category ownership;
- mission-icon semantic mapping, accessibility text, required/optional fallback,
  and source color-key rejection;
- empty, immediate, delayed, cancelled, failed, and stale load plans;
- confirmation before and after accepted readiness;
- duplicate confirmation and cancellation;
- wager eligibility, insufficient balance, stale balance, and rollback;
- every base failure category;
- zero, one, eight, and maximum configured hints;
- deterministic hint selection across repeated runs;
- attempt counts below, at, and above seven;
- terminal non-skippable missions;
- retry, abort, skip, and replay command failures;
- chapter completion and exact statistic totals;
- locked, attempted, failed, completed, and skipped replay entries;
- tutorial mission identity without mission-number shifting;
- transition cancellation, timeout, and reduced-motion fallback;
- feature install, removal, and stale completion; and
- deterministic diagnostics and repeated generation.

## Invariants

- A mission screen never owns mission or progression state.
- Loading readiness and visual animation completion are distinct.
- Exactly one terminal presentation result exists per request.
- Failure hints are deterministic and data-driven.
- Skip is exposed only when the accepted mission policy permits it.
- Replay never changes campaign progression unless its explicit policy says so.
- Artwork and widget order never define mission identity.
- Stale callbacks never navigate or mutate the accepted flow.
