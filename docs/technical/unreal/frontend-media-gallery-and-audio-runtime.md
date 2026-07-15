# Frontend media, gallery, and audio runtime

- Status: Active
- Last reviewed: 2026-07-15

## Governing decisions and specifications

<!-- markdownlint-disable-next-line MD013 -->
- [Common UI front end and progress projection](../../adr/unreal/ui/common-ui-frontend-and-progress-projection.md)
- [UI parity boundary](../../adr/unreal/ui/ui-parity-boundary.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Common UI navigation, menu, and modal runtime](common-ui-navigation-menu-and-modal-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Frontend screen flow and settings runtime](frontend-screen-flow-and-settings-runtime.md)
- [Frontend shell and menu runtime](frontend-shell-and-menu-runtime.md)
- [Presentation playback runtime](presentation-playback-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Progression, collectibles, cheats, and credits](progression-collectibles-and-cheats.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Device configuration and save-slot runtime](device-configuration-and-save-slot-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Platform audio cooking and streaming](platform-audio-cooking-and-streaming.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Platform cinematic media packaging](platform-cinematic-media-packaging.md)

## Purpose

This specification defines the frontend projections for options routing,
cinematic playback, unlocked-movie browsing, scrapbook navigation, chapter and
category progress, aggregate statistics, character-outfit previews, vehicle
previews, audio configuration, splash and start handling, idle demonstrations,
and credits.

These screens are projections over media, progression, catalog, asset-loading,
audio, configuration, input, and application-lifecycle services. They do not
own unlocks, save data, media queues, settings persistence, controller identity,
or application-mode transitions.

## Ownership

<!-- markdownlint-disable MD013 -->

| Service | Authority |
| :--- | :--- |
| Frontend flow subsystem | Screen entry, navigation intent, history, and accepted frontend revision. |
| Progression application service | Chapter completion, category counts, unlock state, and completion percentages. |
| Content catalog | Canonical movie, outfit, vehicle, mission, card, and chapter identities. |
| Presentation playback service | Media requests, loading, playback, skip, audio policy, completion, and teardown. |
| Preview presentation service | Thumbnail and three-dimensional preview loading, scene leases, camera policy, and release. |
| Device-configuration service | Audio settings draft, validation, preview, commit, and rollback. |
| Audio runtime | Accepted category gains, output mode, preview samples, and movie-audio suspension leases. |
| Application lifecycle service | Start acceptance, idle-demo entry, loading, gameplay, frontend restoration, and exit. |
| Common UI kernel | Widget activation, focus, actions, menus, modals, transitions, and asset leases. |

<!-- markdownlint-enable MD013 -->

Widgets consume immutable snapshots and publish typed commands. Visual
animations, cursor motion, thumbnail loading, and model rotation cannot change
progression or configuration authority.

## Frontend projection identities

The runtime uses stable identities for:

- options category;
- media item and media playback request;
- scrapbook section and chapter;
- progress category and statistic;
- gallery item and preview request;
- audio setting and edit session;
- splash session and idle-demonstration request;
- credits sequence and credits line; and
- the feature package that owns an optional projection.

Display order, localized labels, widget child indexes, filenames, and physical
input buttons are not identities.

## Options hub projection

The options hub projects registered categories. The base catalog may expose:

- controller and input;
- audio;
- display and graphics;
- language and accessibility;
- unlocked movies;
- credits; and
- authorized development diagnostics.

Each category declares availability predicates, destination screen, required
asset bundles, settings domain, accessibility metadata, and feature owner.
Platform capability changes alter availability through evaluated data, not
compile-time menu variants.

Opening a settings category creates or resumes one correlated settings edit
session. Opening media or credits creates a read-only presentation request. The
options widget cannot write settings or start media directly.

## Media library

`FSharFrontendMediaItemSnapshot` contains:

- canonical media identity;
- localized title and description identities;
- chapter, campaign, bonus, or system classification;
- unlocked, visible, and playable state with typed reason;
- thumbnail and media asset identities;
- localized-audio availability;
- skippable policy;
- music-suspension policy;
- completion-routing policy; and
- progression, catalog, and feature revisions.

The media library is sorted by a stable catalog order. Locked entries remain
visible or hidden only according to product policy. A cheat or development
command may alter the progression service's effective unlock projection, but a
screen never treats visible artwork as proof of unlock.

## Media playback request

A frontend media playback request contains:

- request, owner, screen, local-player, and media identities;
- expected frontend, catalog, progression, locale, and settings revisions;
- source screen and completion destination;
- skippable, pause, focus-loss, and suspension policy;
- localized audio and subtitle policy;
- music and ambience lease policy;
- required media and fallback bundles; and
- timeout and recovery policy.

The presentation playback service validates the request against the catalog and
platform decoder capability. Unknown or unavailable media cannot be replaced by
an adjacent catalog item.

## Media playback transaction

Playback follows this sequence:

1. validate media identity, unlock state, locale, platform support, and owner;
1. reserve the presentation layer and acquire required asset bundles;
1. acquire audio, input, camera, and screen-visibility leases;
1. open the media source and verify tracks before visible playback;
1. activate the playback projection and accepted skip policy;
1. publish started, progress, and terminal observations with request revision;
1. stop and close media on completion, skip, cancellation, or failure;
1. restore audio, input, camera, and frontend presentation; and
1. route exactly once to the accepted completion destination.

Back does not implicitly leave a media screen. It requests skip only when the
media definition and current playback state allow it. Repeated skip input is
idempotent.

## Localized media tracks

The locale service resolves preferred spoken language, subtitle language, and
fallback order. The media adapter reports actual available tracks before
playback.

A localized request selects the highest-priority supported track. A media item
that is intentionally non-localized uses its declared canonical track. Missing
required audio or subtitle tracks produce a typed finding; they cannot silently
select an unrelated track index.

Track selection is independent of widget language and platform build flags.

## Audio and presentation leases during media

Media playback may request temporary suspension of frontend music, ambience,
input, camera, and selected rendering layers. Each suspension is a scoped lease
owned by the media request.

The prior accepted state is restored when the final overlapping lease ends.
Failure to open media releases every acquired lease. A stale completion cannot
resume audio or reveal a screen owned by a newer request.

## Media completion routing

Completion policy may:

- return to the source library;
- return to the prior valid screen;
- continue a boot or introduction task graph;
- request a new-game or demonstration transition;
- show credits; or
- terminate an optional sequence.

The application lifecycle service owns mode changes. Media completion is an
observation, not permission for the widget to enter gameplay or terminate the
frontend.

## Scrapbook entry

The scrapbook is a read-only progression portal. Its entry snapshot contains:

- total completion value and display precision;
- completion-marker policy;
- available sections;
- current chapter selection;
- catalog and progression revisions; and
- any typed degradation or migration finding.

A one-hundred-percent marker is shown only when the progression service reports
the accepted completion threshold. Floating-point display rounding cannot award
or remove that marker.

The base scrapbook exposes a content browser and an aggregate statistics view.
Feature packages may add namespaced sections without changing base identities.

## Chapter selector

The chapter selector uses canonical chapter identities and a deterministic
catalog order. Page-left and page-right actions select the previous or next
eligible chapter. Pointer and touch input map to the same semantic commands.

The snapshot declares whether chapter wrapping is allowed, which chapters are
visible, and why a chapter is unavailable. A chapter change invalidates pending
category and preview requests from the prior chapter through the chapter
revision.

Localized trigger glyphs and labels are presentation data, not alternate
navigation logic.

## Scrapbook categories

The base content browser may project:

- story and bonus missions;
- character outfits;
- vehicles;
- collector cards; and
- unlocked cinematics.

Each category snapshot contains current and total counts, completion value,
visibility, entry eligibility, disabled reason, destination, and required asset
bundles.

Counts come from progression and catalog authorities. A placeholder menu cell,
hidden row, or unavailable gallery asset never changes the denominator. Empty
categories remain a valid explicit state.

## Mission projection

Mission rows contain canonical mission identity, chapter, role, completion,
replay eligibility, best accepted result where applicable, destination, and
failure reason.

Starting replay uses the mission replay transaction defined by the mission
runtime. The scrapbook cannot start a mission from a filename or visible row
index.

## Aggregate statistics

`FSharFrontendStatisticsSnapshot` contains exact values and display formatting
for registered statistics such as:

- story missions completed;
- bonus missions completed;
- races completed;
- cards collected;
- outfits unlocked;
- vehicles unlocked;
- hazards cleared;
- gags completed;
- cinematics unlocked; and
- total completion.

Each statistic declares value type, numerator, denominator when meaningful,
formatting, localization, accessibility text, and source revision. Percentages
are calculated by the progression service and formatted by localization policy.
The widget cannot derive total completion by averaging displayed rows.

## Gallery definition

Outfit and vehicle galleries use one generic gallery contract.
`FSharFrontendGalleryDefinition` declares:

- gallery identity and item family;
- chapter or global filter;
- deterministic item order;
- grid and paging policy;
- thumbnail bundle;
- three-dimensional preview bundle;
- preview scene and camera policy;
- rotation, zoom, and idle-animation policy;
- unavailable-item presentation;
- selection and back behavior; and
- feature owner.

Outfits and vehicles keep distinct catalog schemas and preview adapters while
sharing navigation and loading behavior.

## Gallery item snapshot

A gallery item snapshot contains:

- canonical item identity;
- localized display identity;
- unlock, visibility, and preview eligibility;
- thumbnail soft reference;
- preview asset soft references;
- associated character or vehicle identity;
- optional statistics or descriptive metadata;
- disabled or degraded reason; and
- catalog, progression, chapter, and feature revisions.

Missing thumbnail and missing three-dimensional preview are distinct findings.
A validated fallback thumbnail may be used without pretending that the preview
asset exists.

## Gallery state machine

One gallery screen has these states:

- `browsing`;
- `thumbnail_loading`;
- `preview_loading`;
- `entering_preview`;
- `preview_active`;
- `leaving_preview`;
- `recovering`; or
- `blocked`.

Selection remains identified by item identity across asynchronous loads and
layout changes. Entering preview suspends grid commands, reserves the preview
scene, and verifies the expected item revision before committing.

Back from preview returns to the same item when still valid. Back from browsing
uses normal navigation history.

## Thumbnail loading

Visible thumbnails are requested through a bounded Asset Manager bundle. The
screen may prefetch a small deterministic window around the current page.

Requests are deduplicated by asset identity. Completion attaches only when the
gallery, page, item, and request revisions still match. Leaving the gallery or
changing chapter releases unshared handles.

The widget never constructs asset paths from display names.

## Three-dimensional preview

A preview request contains item identity, expected revisions, preview scene,
mesh or actor definition, material and animation requirements, camera preset,
lighting profile, floor and background policy, and timeout.

The preview service loads into an isolated presentation world or registered
preview scene. It cannot spawn or mutate gameplay actors. Outfit previews use a
compatible character presentation definition. Vehicle previews use the
canonical vehicle presentation definition and presentation-only state.

Preview assets are visible only after all required components validate. Partial
models, mismatched skeletons, missing required materials, or stale requests fail
without replacing the accepted selection.

## Preview interaction

Rotate, zoom, inspect, and back are semantic actions. Motion is frame-rate
independent and constrained by the gallery policy. Pointer drag, gamepad axis,
and touch gesture adapters produce the same normalized request.

Reduced-motion mode disables automatic spin or substitutes a low-motion policy.
Accessibility navigation remains available without requiring three-dimensional
interaction.

## Audio settings model

The audio category edits one `FSharSettingsEditSession`. The base schema may
contain:

- music gain;
- sound-effects gain;
- vehicle or engine gain;
- dialogue gain;
- output-mode selection;
- dynamic-range profile;
- subtitle and caption audio-related preferences; and
- platform-supported accessibility values.

Every setting declares stable identity, type, range or finite domain, step,
default, platform capability, preview policy, persistence scope, and localized
formatter.

## Audio adjustment and preview

Slider movement updates the settings draft and may apply a scoped preview gain.
The end of an adjustment may request one bounded representative preview sample
for the affected category. Repeated movement does not create overlapping sample
storms.

Output mode is selected from capabilities reported by the platform audio
adapter. Mono, stereo, surround, headphones, or future modes are data values,
not compile-time screen variants.

Leaving without commit restores accepted audio state. Commit validates the
complete draft, persists device-local configuration atomically, activates one
new revision, and releases preview leases.

## Splash and start session

The splash screen is one application-lifecycle task, not an autonomous mode
controller. Its snapshot contains:

- legal and boot-task completion;
- accepted locale and configuration readiness;
- press-to-start availability;
- current input method and glyph context;
- primary-player claim policy;
- loading or recovery presentation;
- idle-demonstration policy; and
- frontend and lifecycle revisions.

A start command claims or validates the initiating local player and physical
device, then submits one frontend-entry request. Duplicate start commands are
idempotent while the request is pending.

## Idle demonstration

The idle policy declares timeout, activity-reset observations, eligible media or
runtime demonstrations, selection order, platform predicates, and restoration
behavior.

Input, focus, accessibility activity, modal activation, application suspension,
or an incomplete required boot task resets or blocks the timer. Selection is
deterministic for the accepted policy revision.

A media demonstration uses the media transaction. A runtime demonstration uses
a bounded application-lifecycle session with isolated progression. Completion,
input, or failure restores the accepted splash or frontend screen through a
typed result.

## Credits sequence

Credits are data-driven sequences of localized, styled entries. Each entry
contains stable identity, role or section, text identity, style, spacing,
optional asset reference, and accessibility narration order.

The credits controller owns monotonic scroll progress, viewport-safe layout,
line-entry observations where needed for presentation cues, skip policy, pause
policy, and terminal routing.

Skipping is a semantic command accepted only when the sequence policy allows
it. Credits completion cannot mutate progression unless a separate application
command explicitly owns that effect.

## Localization and layout

Movie titles, scrapbook categories, statistics, item names, audio labels, splash
instructions, loading text, and credits use stable localization identities with
typed arguments.

Layout adapts to locale length, text direction, display density, safe areas,
cutouts, and accessibility scale. Manual line splitting, fixed byte counts,
platform suffixes, and art-authored text selection do not define behavior.

## Accessibility

Every projection provides:

- deterministic focus order;
- narrated identity, value, state, and disabled reason;
- captions or subtitles where required;
- configurable text and timing policy;
- touch-safe targets;
- reduced-motion gallery and transition behavior;
- non-color-only selection and unlock state; and
- an equivalent path that does not require a three-dimensional preview.

Media skip, gallery controls, sliders, and credits remain operable through the
semantic action model.

## Feature and mod overlays

Validated feature packages may add namespaced media, scrapbook sections,
statistics, gallery items, audio profiles, demonstrations, or credits entries.
They must declare catalog identity, localization, asset bundles, predicates,
ordering anchors, fallback, and removal behavior.

Feature removal cancels owned loads and playback, removes owned rows, restores a
valid selection, and releases preview or audio leases. It cannot change base
progress denominators without a declared progression extension contract.

## Diagnostics

Structured diagnostics include:

- frontend, screen, media, gallery, item, preview, settings, demo, credits, and
  feature identities;
- expected and observed revisions;
- unlock and eligibility result;
- requested and resolved locale tracks;
- asset bundle, decoder, and preview-scene result;
- audio preview and commit result;
- stale-completion reason;
- terminal routing result; and
- fallback or recovery action.

Logs exclude private local routes, media payloads, save contents, proprietary
asset data, credentials, and unbounded user text.

## Failure behavior

Unknown catalog identities, stale revisions, unavailable required assets,
unsupported media tracks, invalid settings values, and conflicting preview
leases fail closed.

A failed optional thumbnail uses only a validated fallback. A failed preview
returns to browsing. A failed media request restores frontend audio, input, and
visibility. A failed settings commit restores the accepted configuration. A
failed idle demonstration returns to the accepted splash or frontend state.

No failure may award an unlock, alter completion, persist a partial setting,
leave audio suspended, retain an orphan preview actor, or route to an unrelated
screen.

## Validation

Validation proves:

- every options category resolves to a registered destination and capability
  predicate;
- every media row resolves to catalog, thumbnail, media, locale-track, and
  completion policy;
- scrapbook category counts reconcile with catalog and progression authorities;
- statistic numerators, denominators, and formatting schemas are valid;
- every gallery row resolves to stable item identity and cooked bundles;
- preview definitions are compatible with their character or vehicle schema;
- audio ranges, steps, defaults, output modes, and preview samples validate;
- splash, idle-demo, and credits routes have terminal restoration behavior;
- required localization identities resolve for supported locales; and
- every asynchronous completion verifies request and revision identity.

## Tests

Automated tests cover:

- empty, one-item, normal, and maximum supported media libraries and galleries;
- locked, hidden, unlocked, missing-thumbnail, and missing-preview states;
- localized media-track selection and declared fallback;
- skip before open, during playback, after completion, and repeated skip;
- audio, input, camera, and visibility lease restoration on every media result;
- scrapbook chapter paging and category counts;
- one-hundred-percent marker threshold and display rounding;
- statistics reconciliation and localization formatting;
- thumbnail prefetch, deduplication, cancellation, and stale completion;
- preview enter, rotate, zoom, back, timeout, incompatibility, and removal;
- audio slider preview, adjustment completion, cancel, commit, and rollback;
- output-mode capability changes during an edit session;
- start-command deduplication and primary-player claim;
- idle timer reset, media demonstration, runtime demonstration, and recovery;
- credits scroll, reduced motion, skip, and terminal routing;
- feature registration and removal during playback or preview; and
- keyboard, mouse, gamepad, touch, accessibility, and locale variants.

## Invariants

- Frontend projections never own progression, saves, settings, or application
  modes.
- One media request reaches one terminal result and releases every owned lease.
- Locked artwork or a visible widget never proves an unlock.
- Scrapbook counts come from catalog and progression authorities.
- Gallery selection is a stable item identity, never a child index.
- Required preview assets validate before presentation becomes active.
- Audio settings persist only through an atomic settings commit.
- Start and idle-demo commands are correlated lifecycle requests.
- Credits are data-driven and cannot infer progression effects.
- Stale asset, playback, preview, settings, or lifecycle completion never
  mutates accepted frontend state.
