# Music state and transition runtime

- Status: Active
- Last reviewed: 2026-07-14

## Governing decisions

<!-- markdownlint-disable-next-line MD013 -->
- [Event-driven music and ambience](../../adr/unreal/runtime/event-driven-music-and-ambience.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Platform-native audio cooking and streaming](../../adr/audio/platform-native-audio-cooking-and-streaming.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Canonical seven-level campaign and world variants](../../adr/unreal/runtime/canonical-seven-level-campaign-and-world-variants.md)
- [Runtime parity boundary](../../adr/unreal/runtime/remake-parity-boundary.md)

## Purpose

This specification defines canonical music identity, level profiles, semantic
events, state priority, mission and race bindings, Quartz clocks, quantized
transitions, MetaSound presentation, pause and lifecycle behavior, fallback,
failure, and verification.

## Ownership

`USharMusicSubsystem` is a game-instance subsystem. It owns one logical score
state across frontend, loading, gameplay, interiors, missions, races, movies,
pause, credits, and level travel.

It consumes typed observations from frontend, campaign, mission, race, vehicle,
interaction, notoriety, cinematic, platform-audio, and lifecycle ports. It emits
presentation commands and diagnostics. It never advances gameplay, awards
progression, or infers mission success from audio playback.

The target audio adapter owns native cooking, stream cache, voices, output
routes,
and device focus. The music subsystem owns semantic state and transition intent.

## Definition assets

The root gameplay catalog references:

<!-- markdownlint-disable MD013 -->

| Asset | Primary asset type | Purpose |
| :--- | :--- | :--- |
| Music catalog | `SharMusicCatalog` | Composition, state, profile, cue, and transition definitions. |
| Level music profile | `SharMusicProfile` | Level-specific default, driving, interior, and special-event bindings. |
| Music composition | `SharMusicComposition` | Canonical normalized composition and stem set. |
| Music graph | `SharMusicGraph` | MetaSound or native graph used for layers and parameters. |
| Music state table | `FSharMusicStateRow` | Semantic state, priority, persistence, and required composition. |
| Music binding table | `FSharMusicBindingRow` | Context and event to state transition. |
| Music transition table | `FSharMusicTransitionRow` | Quantization, fades, interruption, and fallback. |

<!-- markdownlint-enable MD013 -->

## Canonical composition identity

`USharMusicCompositionDefinition` contains:

| Field | Contract |
| :--- | :--- |
| `CompositionId` | Stable identity independent of display title. |
| `DisplayName` | Optional localizable or review-facing name. |
| `NormalizedAudioIds` | Canonical full mix, stems, layers, and stingers. |
| `TempoMap` | Exact tempo, meter, and change points. |
| `LoopRegions` | Sample-aligned loop identities and boundaries. |
| `SyncMarkers` | Sample-aligned bar, beat, section, and event markers. |
| `GraphAsset` | Soft MetaSound or repository-owned native graph reference. |
| `RequiredLayers` | Layers required for parity. |
| `OptionalLayers` | Presentation layers with deterministic fallback. |
| `TargetPolicyId` | Platform cook, streaming, cache, and concurrency policy. |
| `RevisionToken` | Deterministic source and definition revision. |

Display names, descriptive soundtrack titles, filenames, and Unreal object paths
are never composition identity.

## Music state

`FSharMusicStateRow` contains:

<!-- markdownlint-disable MD013 -->

| Field | Contract |
| :--- | :--- |
| `StateId` | Stable semantic state identity. |
| `StateClass` | Frontend, level, interior, mission, race, alert, cinematic, pause, loading, or credits. |
| `Priority` | Explicit signed priority. |
| `CompositionId` | Required canonical composition. |
| `EntrySectionId` | Declared entry section or marker. |
| `LoopRegionId` | Declared loop region. |
| `LayerParameters` | Deterministic stem and graph parameters. |
| `Persistence` | Restart, resume, retain-under-overlay, or stop. |
| `InterruptionPolicy` | Events allowed to replace or overlay the state. |
| `FallbackStateId` | Exact fallback when optional content is unavailable. |
| `Required` | Whether activation fails without this state. |

<!-- markdownlint-enable MD013 -->

The active state stack is ordered by priority and activation sequence. Equal
priority is valid only when the binding declares overlay composition; otherwise
it is rejected as ambiguous.

## Semantic event vocabulary

The fixed event vocabulary includes:

- frontend enter and exit;
- loading start and complete;
- level intro, level active, and level complete;
- free drive start, enter vehicle, and leave vehicle;
- interior enter and exit by canonical location identity;
- mission gated, start, normal, drama, warning, win, lose, retry, and complete;
- race start, normal, warning, win, lose, and leave vehicle;
- notoriety warning, pursuit start, pursuit resolved, and arrest;
- wasp attack and world bonus events;
- pause, unpause, movie start, movie end, focus loss, and focus restore;
- calendar or level-specific scary presentation;
- credits start, skip, and complete; and
- explicit package-overlay state registration and removal.

Every event contains a stable event identity, context identities, simulation or
presentation timestamp, and source revision. Frame callbacks and audio-component
completion are not semantic events.

## Level motif profiles

The verified base profiles define:

<!-- markdownlint-disable MD013 -->

| Level context | Canonical profile | Presentation direction |
| :--- | :--- | :--- |
| Level 1 | `music_profile_homer` | Orchestral comedy with a prominent low-brass motif. |
| Levels 2 and 6 | `music_profile_bart` | Rock-focused profile with overdriven guitar. |
| Level 3 | `music_profile_lisa` | Jazz-focused profile with prominent saxophone. |
| Level 4 | `music_profile_marge` | Piano, string, and woodwind profile; driving reuses the declared suburban composition. |
| Level 5 | `music_profile_apu` | South Asian-inspired profile with declared plucked and reed instrumentation. |
| Level 7 | `music_profile_halloween` | Horror profile with organ and theremin presentation. |

<!-- markdownlint-enable MD013 -->

These directions constrain composition and mix review. They are not procedural
instrument-generation rules and do not replace normalized audio evidence.

## Verified composition bindings

The following canonical cue identities have verified mission or race contexts.
A cue may bind the same composition to several states without duplicating audio.

<!-- markdownlint-disable MD013 -->

| Cue identity | Verified contexts |
| :--- | :--- |
| `cue_morning_chores` | `s_m_r_t`, `this_old_shanty`, `dial_b_for_blood`, Level 1 wager race, Level 2 wager race. |
| `cue_homer_a_doh_go` | `petty_theft_homer`, `for_a_few_donuts_more`, `alien_autotopsy_part_2`. |
| `cue_plowing_through` | `office_spaced`, `the_old_pirate_and_the_sea`, `flaming_tires`, Level 7 wager race. |
| `cue_husky` | `blind_big_brother`, `beached_love`, Level 4 wager race. |
| `cue_paranoid` | `flowers_by_irene`, `slithery_sleuthing`. |
| `cue_hitting_the_streets` | `bonestorm_storm`, `weapons_of_mass_delinquency`, `clueless`. |
| `cue_saving_springfield` | `the_fat_and_furious`, `return_of_the_nearly_dead`, `and_baby_makes_8`, `duff_for_me_duff_for_you`, `alien_autotopsy_part_3`. |
| `cue_lightning_fast_wit` | `detention_deficit_disorder`, `getting_down_with_the_clown`. |
| `cue_comic_book_theme` | `vox_nerduli`, `nerd_race_queen`. |
| `cue_fresh_skid_marks` | `bart_n_frink`, second `from_outer_space` state, `full_metal_jackass`. |
| `cue_cletus_theme` | `better_than_beef`, `bonfire_of_the_manatees`, `redneck_roundup`. |
| `cue_heavy_drinker` | `monkey_see_monkey_doh`, `the_cola_wars`. |
| `cue_frink_theme` | `cell_outs`, `lab_coat_caper`, `pocket_protector`. |
| `cue_otto_theme` | `operation_hellfish`, `going_to_the_lu`. |
| `cue_lisa_drive` | Level 3 free drive and `fishy_deals`. |
| `cue_busy_body_housewife` | `ketchup_logic`, first `from_outer_space` state. |
| `cue_wolves_stole_my_pills` | `wolves_stole_my_pills`. |
| `cue_large_vehicle` | `eight_is_too_much`, `kinky_frinky`, Level 5 wager race. |
| `cue_hindu_that_i_do` | `incriminating_caffeine`, `kwik_cash`. |
| `cue_stop_what_you_are_doing` | second `better_than_beef` state, `this_little_piggy`, `curious_curator`. |
| `cue_community_service` | final `weapons_of_mass_delinquency` state, `never_trust_a_snake`, `set_to_kill`. |
| `cue_halls_balls` | `milking_the_pigs`, first `theres_something_about_monty` state, Level 6 wager race. |
| `cue_kang_and_kodos` | `kang_and_kodos_strike_back`. |
| `cue_evergreen_terror` | Level 7 free drive and `rigor_motors`. |
| `cue_alien_probe` | `long_black_probes`, `alien_autotopsy_part_1`. |
| `cue_town_hero` | second `theres_something_about_monty` state. |

<!-- markdownlint-enable MD013 -->

A mission with multiple cue bindings uses explicit step or semantic-state
ranges.
The binding table never assumes the entire mission uses the first cue.

## Standard state bindings

Each level profile declares bindings for:

- level intro;
- on-foot free drive;
- in-vehicle free drive;
- store or purchase interaction;
- each supported interior;
- three street races and the wager race;
- mission normal, drama, warning, win, and lose states;
- notoriety warning and pursuit;
- wasp attack;
- level complete; and
- frontend, loading, movie, pause, and credits overlays when active.

An unsupported optional event resolves to the profile's declared default
state. A missing required binding fails profile validation.

## Quartz clock contract

Each active profile owns one Quartz clock with declared sample rate, tempo map,
meter, and transport revision. Transitions use one of:

- immediate sample boundary;
- next beat;
- next bar;
- next named section;
- declared crossfade window; or
- stop and restart after a blocking cinematic or load.

A transition request is revision-bound. If the tempo map or active state changes
before execution, the request is recalculated or rejected according to policy.
It cannot execute against stale timing.

Pause may suspend clock advancement or retain a low-priority overlay according
to
the active state. Unpause resumes at the accepted quantized position. Frame rate
does not determine musical timing.

## MetaSound graph contract

A music graph exposes only declared parameters such as:

- state and section identity;
- stem enable and gain;
- intensity permille;
- transition trigger;
- stinger identity;
- fade duration;
- pause or focus state; and
- target-specific mix policy.

Parameter names and ranges are generated from the music definition. Gameplay
code
never sets arbitrary graph parameters or object-path references.

A graph read-back adapter verifies the active composition, section, transport
position, required layers, and parameter revision. Presentation differences by
target cannot change semantic state.

## Mission and race integration

Mission definitions bind semantic mission steps or state ranges to cue
identities.
The mission runtime emits start, normal, drama, warning, win, lose, retry, and
complete events. The music subsystem does not inspect mission actor classes or
stage numbers.

Race definitions bind their own start, normal, warning, win, lose, and
leave-vehicle states. A race state cannot inherit mission music merely because a
race is represented by a mission definition.

A ten-second warning or equivalent urgency cue is emitted by the timer domain at
the declared threshold. The music subsystem does not poll a widget countdown.

## Interior and world integration

Interior portals emit location enter and exit events only after the
transactional
interior state commits. The profile selects the exact interior state or declared
fallback. Failed portal activation cannot switch music.

World Partition streaming may load audio bundles early, but the active state
changes only from semantic events. Streaming out an inactive zone cannot stop
the
current required score.

## Movies, loading, pause, and focus

A movie transition declares whether score stops, pauses and resumes, ducks
beneath
cinematic audio, or hands off at a named sync marker.

Loading owns a blocking state with an explicit return destination. A failed load
restores the prior stable profile or frontend state.

Platform focus loss follows the active role policy: pause, duck, stop, or
retain. Focus restoration resumes from the declared quantized boundary and never
replays a mission or progression event.

## Mod overlays

Validated data overlays may add compositions, states, bindings, and transitions
through declared extensibility points. Cooked asset overlays may provide
target-native music graphs and audio assets for those identities.

An overlay cannot replace immutable base cue identity, alter a base mission's
semantic timing without an authorized target, or register a native graph before
its package active-set transaction commits.

Removing an active music overlay transitions to its declared fallback before the
overlay is deactivated.

## Failure behavior

Music activation or transition fails closed on:

- unknown composition, state, cue, binding, profile, event, or transition;
- duplicate or ambiguous state priority;
- invalid tempo, meter, loop, marker, quantization, or fade data;
- missing required normalized audio, graph, layer, or target policy;
- stale profile, context, or transport revision;
- a graph parameter outside its declared schema;
- target-cook duration, loop, or marker drift;
- inability to satisfy an uninterruptible active state;
- read-back mismatch; or
- a binding that depends on a display title, filename, object path, or frame
  rate.

An optional-layer failure uses the exact fallback. A required-state failure
blocks
the owning level, mission, race, or frontend activation before misleading
playback begins.

## Verification

Automated evidence includes:

- every level profile resolving required default, driving, mission, race,
  interior, pause, loading, movie, and completion states;
- all verified cue-to-context bindings;
- missions with multiple cue ranges;
- deterministic priority and overlay resolution;
- next-beat, next-bar, section, immediate, and crossfade transitions;
- pause, unpause, movie, loading, focus loss, and focus restore;
- timer warning and retry without duplicate stingers;
- interior commit and rollback behavior;
- notoriety warning, pursuit, resolution, and arrest;
- missing optional layers and missing required states;
- target-cook duration, marker, loop, and graph read-back;
- Low through Ultra semantic equivalence;
- Android lifecycle and audio-focus behavior;
- mod overlay activation and removal; and
- repeated event traces producing equivalent state and transition traces.
