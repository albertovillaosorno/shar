// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT
//
// Boundary-Contract:
// - Owns:
//   - Reviewed stage header, timing, checkpoint, message, vehicle, and waypoint
//     evidence.
// - Must-Not:
//   - Interpret unrelated stage commands or resolve world objects by guesswork.
// - Allows:
//   - Compile documented stage forms into deterministic typed evidence.
// - Split-When:
//   - One stage directive family gains an independent runtime lifecycle.
// - Merge-When:
//   - Mission definition compilation owns this exact stage evidence boundary.
// - Summary:
//   - Mission stage semantic compiler.
// - Description:
//   - Types reviewed AddStage arguments and selected direct stage commands.
// - Usage:
//   - Runs after lossless mission scope projection succeeds.
// - Defaults:
//   - Unknown stage forms and malformed reviewed directive values fail closed.
//

//! Typed compilation of reviewed mission stage evidence.

use super::{MissionScopeReport, MissionScopeStage};

/// Reviewed kind carried by one `AddStage` declaration.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MissionStageKind {
    /// Standard stage with preserved opaque legacy flags and final marker.
    Standard {
        /// Exact reviewed numeric flags from the legacy `AddStage` call.
        legacy_flags: Option<u8>,
        /// Whether the source explicitly marks this as the final stage.
        final_stage: bool,
    },
    /// Locked stage requiring one exact vehicle identity.
    LockedVehicle {
        /// Exact required source vehicle identity.
        vehicle_id: String,
    },
    /// Locked stage requiring one exact costume identity.
    LockedCostume {
        /// Exact required source costume identity.
        costume_id: String,
    },
}

/// Text namespace selected by the stage message command.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MissionStageMessageKind {
    /// Minimum-width-two `MISSION_OBJECTIVE_00..299` namespace.
    Objective,
    /// `INGAME_MESSAGE_00..19` namespace for locked stages.
    Locked,
}

/// One reviewed vehicle declaration attached directly to a stage.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionStageVehicleReference {
    source_ordinal: usize,
    vehicle_id: String,
    locator_id: String,
    behaviour: String,
    con_file: String,
    driver_id: Option<String>,
}

impl MissionStageVehicleReference {
    /// Return source statement ordinal.
    #[must_use]
    pub const fn source_ordinal(&self) -> usize {
        self.source_ordinal
    }

    /// Return exact vehicle identity.
    #[must_use]
    pub fn vehicle_id(&self) -> &str {
        &self.vehicle_id
    }

    /// Return exact `CarStart` locator identity.
    #[must_use]
    pub fn locator_id(&self) -> &str {
        &self.locator_id
    }

    /// Return exact source vehicle behaviour token.
    #[must_use]
    pub fn behaviour(&self) -> &str {
        &self.behaviour
    }

    /// Return exact source CON path.
    #[must_use]
    pub fn con_file(&self) -> &str {
        &self.con_file
    }

    /// Return optional exact driver identity.
    #[must_use]
    pub fn driver_id(&self) -> Option<&str> {
        self.driver_id.as_deref()
    }
}

/// Selected typed direct stage directives.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MissionStageDirective {
    /// Replace the stage timer with this many seconds.
    SetTimeSeconds {
        /// Source statement ordinal.
        source_ordinal: usize,
        /// Exact positive timer value in seconds.
        seconds: u32,
    },
    /// Add this many seconds to the current stage timer.
    AddTimeSeconds {
        /// Source statement ordinal.
        source_ordinal: usize,
        /// Exact source value in seconds.
        source_seconds: u32,
        /// Effective legacy addition; source zero adds one second.
        effective_seconds: u32,
    },
    /// Restart/select checkpoint begins at this stage.
    ResetCheckpoint {
        /// Source statement ordinal.
        source_ordinal: usize,
    },
    /// Stage message index in its stage-dependent namespace.
    MessageIndex {
        /// Source statement ordinal.
        source_ordinal: usize,
        /// Stage-dependent message namespace.
        kind: MissionStageMessageKind,
        /// Exact source message index.
        index: u16,
        /// Optional exact argument documented by the legacy API as unused.
        unused_argument: Option<String>,
    },
    /// Vehicle declaration with documented field roles.
    Vehicle(MissionStageVehicleReference),
    /// Activate an existing mission vehicle with reviewed field roles.
    ActivateVehicle {
        /// Source statement ordinal.
        source_ordinal: usize,
        /// Exact mission vehicle identity.
        vehicle_id: String,
        /// Exact `CarStart` locator identity, including `NULL` when authored.
        locator_id: String,
        /// Exact reviewed vehicle behaviour token.
        behaviour: String,
    },
    /// Select the exact HUD icon sprite for the stage.
    HudIcon {
        /// Source statement ordinal.
        source_ordinal: usize,
        /// Exact source sprite identity.
        sprite_id: String,
    },
    /// Preserve the required but documented-unused fade-out argument.
    FadeOutLegacyArgument {
        /// Source statement ordinal.
        source_ordinal: usize,
        /// Exact source argument retained as compatibility evidence.
        source_value: String,
    },
    /// Preserve the required but documented-unused iris-wipe argument.
    IrisWipeLegacyArgument {
        /// Source statement ordinal.
        source_ordinal: usize,
        /// Exact source argument retained as compatibility evidence.
        source_value: String,
    },
    /// Set the maximum number of traffic cars active for the stage.
    MaxTrafficCars {
        /// Source statement ordinal.
        source_ordinal: usize,
        /// Exact reviewed car limit; the current source corpus uses 1 through
        /// 5.
        cars: u8,
    },
    /// Preserve one vehicle AI tuning tuple without assigning gameplay units.
    VehicleAiTuning {
        /// Source statement ordinal.
        source_ordinal: usize,
        /// Exact source vehicle identity.
        vehicle_id: String,
        /// First exact signed integer source value.
        source_first: i32,
        /// Second exact signed integer source value.
        source_second: i32,
    },
    /// Preserve one target-catchup tuning tuple without assigning gameplay
    /// units.
    TargetCatchupTuning {
        /// Source statement ordinal.
        source_ordinal: usize,
        /// Exact source vehicle identity.
        vehicle_id: String,
        /// First exact signed integer source value.
        source_first: i32,
        /// Second exact signed integer source value.
        source_second: i32,
    },
    /// Preserve one safe-zone locator and positive integer source value.
    SafeZone {
        /// Source statement ordinal.
        source_ordinal: usize,
        /// Exact source locator identity.
        locator_id: String,
        /// Exact positive integer source value; units remain unresolved.
        source_value: u32,
    },
    /// Preserve an authored stay-in-black transition marker.
    StayInBlack {
        /// Source statement ordinal.
        source_ordinal: usize,
    },
    /// Preserve an authored game-over transition marker.
    GameOver {
        /// Source statement ordinal.
        source_ordinal: usize,
    },
    /// Preserve an authored level-over transition marker.
    LevelOver {
        /// Source statement ordinal.
        source_ordinal: usize,
    },
    /// Preserve an authored stage-complete presentation marker.
    ShowStageComplete {
        /// Source statement ordinal.
        source_ordinal: usize,
    },
    /// Preserve an authored hit-and-run disable marker.
    DisableHitAndRun {
        /// Source statement ordinal.
        source_ordinal: usize,
    },
    /// Bind a state-prop collectible identity to a locator and source state.
    CollectibleStateProp {
        /// Source statement ordinal.
        source_ordinal: usize,
        /// Exact source state-prop identity.
        prop_id: String,
        /// Exact source locator identity.
        locator_id: String,
        /// Exact nonnegative source state value.
        source_state: u32,
    },
    /// Place a stage character with optional on-foot locator and vehicle
    /// context.
    StageCharacter {
        /// Source statement ordinal.
        source_ordinal: usize,
        /// Exact source character identity.
        character_id: String,
        /// Optional exact character locator identity.
        character_locator_id: Option<String>,
        /// Exact source vehicle identity, including the `current` token.
        vehicle_id: String,
        /// Exact source vehicle locator identity.
        vehicle_locator_id: String,
    },
    /// Preserve an authored stage music-change marker.
    StageMusicChange {
        /// Source statement ordinal.
        source_ordinal: usize,
    },
    /// Preserve one countdown display token and authored duration.
    CountdownSequenceEntry {
        /// Source statement ordinal.
        source_ordinal: usize,
        /// Exact nonempty source countdown token.
        token: String,
        /// Exact positive authored duration in milliseconds.
        duration_milliseconds: u32,
    },
    /// Preserve the reviewed mission-abort boolean source value.
    MissionAbortAllowed {
        /// Source statement ordinal.
        source_ordinal: usize,
        /// Exact reviewed source boolean.
        allowed: bool,
    },
    /// Preserve the exact legacy `GoToPsScreenWhenDone` marker.
    GotoPsScreenWhenDone {
        /// Source statement ordinal.
        source_ordinal: usize,
    },
    /// Preserve an authored no-traffic stage marker.
    NoTrafficForStage {
        /// Source statement ordinal.
        source_ordinal: usize,
    },
    /// Place the player car at an exact source locator.
    PlacePlayerCar {
        /// Source statement ordinal.
        source_ordinal: usize,
        /// Exact source vehicle identity, including the `current` token.
        vehicle_id: String,
        /// Exact source car locator identity.
        locator_id: String,
    },
    /// Preserve the exact legacy `PutMFPlayerInCar` marker.
    PutMfPlayerInCar {
        /// Source statement ordinal.
        source_ordinal: usize,
    },
    /// Select one character identity to hide for the stage.
    CharacterToHide {
        /// Source statement ordinal.
        source_ordinal: usize,
        /// Exact source character identity.
        character_id: String,
    },
    /// Select a completion dialogue and optional source character identity.
    CompletionDialog {
        /// Source statement ordinal.
        source_ordinal: usize,
        /// Exact source dialogue identity.
        dialogue_id: String,
        /// Optional exact source character identity.
        character_id: Option<String>,
    },
    /// Preserve the exact nonnegative demo-loop time source value.
    DemoLoopTime {
        /// Source statement ordinal.
        source_ordinal: usize,
        /// Exact source integer.
        source_value: u64,
    },
    /// Select one documented music state/value pair.
    MusicState {
        /// Source statement ordinal.
        source_ordinal: usize,
        /// Exact source music-state identity.
        state_name: String,
        /// Exact source music-state value identity.
        state_value: String,
    },
    /// Select an exact stage-level P3D presentation bitmap source.
    StagePresentationBitmap {
        /// Source statement ordinal.
        source_ordinal: usize,
        /// Exact repository-relative `.p3d` source path.
        p3d_path: String,
    },
    /// Preserve the exact positive stage race-entry fee.
    RaceEntryFee {
        /// Source statement ordinal.
        source_ordinal: usize,
        /// Exact source integer fee.
        source_value: u32,
    },
    /// Preserve one five-field race AI catch-up source tuple.
    RaceCatchupTuning {
        /// Source statement ordinal.
        source_ordinal: usize,
        /// Exact source vehicle identity.
        vehicle_id: String,
        /// Exact first integer source value.
        source_value: u32,
        /// Exact three source decimal lexemes in authored order.
        source_factors: [String; 3],
    },
    /// Preserve an authored always-on stage-music marker.
    StageMusicAlwaysOn {
        /// Source statement ordinal.
        source_ordinal: usize,
    },
    /// Select the default-car locator used by a source swap sequence.
    SwapDefaultCarLocator {
        /// Source statement ordinal.
        source_ordinal: usize,
        /// Exact source locator identity.
        locator_id: String,
    },
    /// Select the forced-car locator used by a source swap sequence.
    SwapForcedCarLocator {
        /// Source statement ordinal.
        source_ordinal: usize,
        /// Exact source locator identity.
        locator_id: String,
    },
    /// Select the player locator used by a source swap sequence.
    SwapPlayerLocator {
        /// Source statement ordinal.
        source_ordinal: usize,
        /// Exact source locator identity.
        locator_id: String,
    },
    /// Select an exact stage-start music event identity.
    StageStartMusicEvent {
        /// Source statement ordinal.
        source_ordinal: usize,
        /// Exact source event identity.
        event_id: String,
    },
    /// Start an exact countdown sequence with an optional character identity.
    StartCountdown {
        /// Source statement ordinal.
        source_ordinal: usize,
        /// Exact source sequence identity.
        sequence_id: String,
        /// Optional exact source character identity.
        character_id: Option<String>,
    },
    /// Preserve an authored swap-in-default-car marker.
    SwapInDefaultCar {
        /// Source statement ordinal.
        source_ordinal: usize,
    },
    /// Preserve an authored use-elapsed-time marker.
    UseElapsedTime {
        /// Source statement ordinal.
        source_ordinal: usize,
    },
    /// Type-0 event locator used as a stage waypoint.
    Waypoint {
        /// Source statement ordinal.
        source_ordinal: usize,
        /// Exact source locator identity.
        locator_id: String,
    },
}

/// Typed stage semantics for one projected source stage.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionStageSemanticBinding {
    source_ordinal: usize,
    sequence_ordinal: usize,
    kind: MissionStageKind,
    directives: Vec<MissionStageDirective>,
}

impl MissionStageSemanticBinding {
    /// Return source `AddStage` ordinal.
    #[must_use]
    pub const fn source_ordinal(&self) -> usize {
        self.source_ordinal
    }

    /// Return dense source stage order.
    #[must_use]
    pub const fn sequence_ordinal(&self) -> usize {
        self.sequence_ordinal
    }

    /// Return reviewed stage header semantics.
    #[must_use]
    pub const fn kind(&self) -> &MissionStageKind {
        &self.kind
    }

    /// Return selected typed stage directives in source order.
    #[must_use]
    pub fn directives(&self) -> &[MissionStageDirective] {
        &self.directives
    }
}

/// Typed stage semantics for all projected missions.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionStageSemanticReport {
    stages: Vec<MissionStageSemanticBinding>,
}

impl MissionStageSemanticReport {
    /// Return typed stages in mission/source order.
    #[must_use]
    pub fn stages(&self) -> &[MissionStageSemanticBinding] {
        &self.stages
    }

    #[cfg(test)]
    pub(crate) fn from_message_entries_for_tests(
        entries: Vec<(
            usize,
            usize,
            MissionStageKind,
            Vec<MissionStageDirective>,
        )>,
    ) -> Self {
        Self {
            stages: entries
                .into_iter()
                .map(
                    |(source_ordinal, sequence_ordinal, kind, directives)| {
                        MissionStageSemanticBinding {
                            source_ordinal,
                            sequence_ordinal,
                            kind,
                            directives,
                        }
                    },
                )
                .collect(),
        }
    }

    #[cfg(test)]
    pub(crate) fn from_topology_entries_for_tests(
        entries: Vec<(usize, usize, bool, Vec<MissionStageDirective>)>,
    ) -> Self {
        Self {
            stages: entries
                .into_iter()
                .map(
                    |(
                        source_ordinal,
                        sequence_ordinal,
                        final_stage,
                        directives,
                    )| MissionStageSemanticBinding {
                        source_ordinal,
                        sequence_ordinal,
                        kind: MissionStageKind::Standard {
                            legacy_flags: Some(0),
                            final_stage,
                        },
                        directives,
                    },
                )
                .collect(),
        }
    }
}

/// Compile reviewed stage header and direct-command semantics.
///
/// # Errors
///
/// Returns an error when a stage header, timer, message, vehicle, or waypoint
/// value is outside the reviewed contract.
pub fn preflight_mission_stage_semantics(
    scopes: &MissionScopeReport,
) -> Result<MissionStageSemanticReport, String> {
    let mut stages = Vec::new();
    for mission in scopes.missions() {
        for stage in mission.stages() {
            stages.push(compile_stage(stage)?);
        }
    }
    Ok(MissionStageSemanticReport { stages })
}

pub(super) fn objective_command_has_stage_semantics(name: &str) -> bool {
    matches!(
        name,
        "activatevehicle"
            | "addsafezone"
            | "addstagecharacter"
            | "addstagevehicle"
            | "disablehitandrun"
            | "setgameover"
            | "setlevelover"
            | "setstageaitargetcatchupparams"
            | "setstagemessageindex"
            | "setvehicleaiparams"
            | "stayinblack"
    )
}

fn compile_stage(
    stage: &MissionScopeStage,
) -> Result<MissionStageSemanticBinding, String> {
    let kind = compile_stage_kind(stage.legacy_parameters())?;
    let mut directives = Vec::<(usize, MissionStageDirective)>::new();
    for command in stage.commands() {
        let directive = match compile_stage_directive(
            &kind,
            command.source_ordinal(),
            command.name(),
            command.arguments(),
        )? {
            Some(directive) => directive,
            None => compile_direct_stage_only_directive(
                command.source_ordinal(),
                command.name(),
                command.arguments(),
            )?
            .ok_or_else(|| {
                "reviewed direct stage command lacks typed semantics".to_owned()
            })?,
        };
        directives.push((command.source_ordinal(), directive));
    }
    for command in stage.objective().commands() {
        let directive = compile_stage_directive(
            &kind,
            command.ordinal(),
            command.command(),
            command.arguments(),
        )?;
        if objective_command_has_stage_semantics(command.command()) {
            let directive = directive.ok_or_else(|| {
                "objective command lost delegated stage semantics".to_owned()
            })?;
            directives.push((command.ordinal(), directive));
        } else if directive.is_some() {
            return Err(format!(
                "objective command unexpectedly acquired stage semantics: {}",
                command.command()
            ));
        }
    }
    directives.sort_by_key(|(ordinal, _directive)| *ordinal);
    Ok(MissionStageSemanticBinding {
        source_ordinal: stage.source_ordinal(),
        sequence_ordinal: stage.sequence_ordinal(),
        kind,
        directives: directives
            .into_iter()
            .map(|(_ordinal, directive)| directive)
            .collect(),
    })
}

fn compile_direct_stage_only_directive(
    source_ordinal: usize,
    name: &str,
    arguments: &[String],
) -> Result<Option<MissionStageDirective>, String> {
    let directive = match name {
        "addcollectiblestateprop" => {
            Some(compile_collectible_state_prop(source_ordinal, arguments)?)
        },
        "addstagemusicchange" => {
            require_no_arguments(arguments, "stage music-change marker")?;
            Some(MissionStageDirective::StageMusicChange { source_ordinal })
        },
        "addtocountdownsequence" => {
            Some(compile_countdown_entry(source_ordinal, arguments)?)
        },
        "allowmissionabort" => {
            let [value] = arguments else {
                return Err(
                    "stage mission-abort source shape drifted".to_owned()
                );
            };
            if value != "false" {
                return Err("stage mission-abort source value is not reviewed"
                    .to_owned());
            }
            Some(MissionStageDirective::MissionAbortAllowed {
                source_ordinal,
                allowed: false,
            })
        },
        "gotopsscreenwhendone" => {
            require_no_arguments(
                arguments,
                "stage GoToPsScreenWhenDone marker",
            )?;
            Some(MissionStageDirective::GotoPsScreenWhenDone { source_ordinal })
        },
        "notrafficforstage" => {
            require_no_arguments(arguments, "stage no-traffic marker")?;
            Some(MissionStageDirective::NoTrafficForStage { source_ordinal })
        },
        "placeplayercar" => {
            Some(compile_place_player_car(source_ordinal, arguments)?)
        },
        "putmfplayerincar" => {
            require_no_arguments(arguments, "stage PutMFPlayerInCar marker")?;
            Some(MissionStageDirective::PutMfPlayerInCar { source_ordinal })
        },
        "setcharactertohide" => Some(MissionStageDirective::CharacterToHide {
            source_ordinal,
            character_id: required_identity(
                arguments,
                "stage hidden character",
            )?,
        }),
        "setcompletiondialog" => {
            Some(compile_completion_dialog(source_ordinal, arguments)?)
        },
        "setdemolooptime" => Some(MissionStageDirective::DemoLoopTime {
            source_ordinal,
            source_value: required_source_u64(
                arguments,
                "stage demo-loop time",
            )?,
        }),
        "setmusicstate" => {
            Some(compile_music_state(source_ordinal, arguments)?)
        },
        "setpresentationbitmap" => {
            Some(MissionStageDirective::StagePresentationBitmap {
                source_ordinal,
                p3d_path: required_p3d_path(
                    arguments,
                    "stage presentation bitmap",
                )?,
            })
        },
        "setraceenteryfee" => Some(MissionStageDirective::RaceEntryFee {
            source_ordinal,
            source_value: required_positive_source_u32(
                arguments,
                "stage race-entry fee",
            )?,
        }),
        "setstageairacecatchupparams" => {
            Some(compile_race_catchup(source_ordinal, arguments)?)
        },
        "setstagemusicalwayson" => {
            require_no_arguments(arguments, "stage music-always-on marker")?;
            Some(MissionStageDirective::StageMusicAlwaysOn { source_ordinal })
        },
        "setswapdefaultcarlocator" => {
            Some(MissionStageDirective::SwapDefaultCarLocator {
                source_ordinal,
                locator_id: required_identity(
                    arguments,
                    "stage default-car swap locator",
                )?,
            })
        },
        "setswapforcedcarlocator" => {
            Some(MissionStageDirective::SwapForcedCarLocator {
                source_ordinal,
                locator_id: required_identity(
                    arguments,
                    "stage forced-car swap locator",
                )?,
            })
        },
        "setswapplayerlocator" => {
            Some(MissionStageDirective::SwapPlayerLocator {
                source_ordinal,
                locator_id: required_identity(
                    arguments,
                    "stage player swap locator",
                )?,
            })
        },
        "stagestartmusicevent" => {
            Some(MissionStageDirective::StageStartMusicEvent {
                source_ordinal,
                event_id: required_identity(
                    arguments,
                    "stage-start music event",
                )?,
            })
        },
        "startcountdown" => {
            Some(compile_start_countdown(source_ordinal, arguments)?)
        },
        "swapindefaultcar" => {
            require_no_arguments(arguments, "stage default-car swap marker")?;
            Some(MissionStageDirective::SwapInDefaultCar { source_ordinal })
        },
        "useelapsedtime" => {
            require_no_arguments(arguments, "stage elapsed-time marker")?;
            Some(MissionStageDirective::UseElapsedTime { source_ordinal })
        },
        _ => None,
    };
    Ok(directive)
}

fn compile_stage_directive(
    kind: &MissionStageKind,
    source_ordinal: usize,
    name: &str,
    arguments: &[String],
) -> Result<Option<MissionStageDirective>, String> {
    let directive = match name {
        "setstagetime" => Some(MissionStageDirective::SetTimeSeconds {
            source_ordinal,
            seconds: parse_positive_seconds(arguments, "stage timer")?,
        }),
        "addstagetime" => {
            let source_seconds =
                parse_nonnegative_seconds(arguments, "stage added time")?;
            Some(MissionStageDirective::AddTimeSeconds {
                source_ordinal,
                source_seconds,
                effective_seconds: source_seconds.max(1),
            })
        },
        "reset_to_here" => {
            Some(MissionStageDirective::ResetCheckpoint { source_ordinal })
        },
        "setstagemessageindex" => {
            Some(compile_message(kind, source_ordinal, arguments)?)
        },
        "addstagevehicle" => Some(MissionStageDirective::Vehicle(
            compile_vehicle(source_ordinal, arguments)?,
        )),
        "activatevehicle" => {
            let [vehicle, locator, behaviour] = arguments else {
                return Err("stage vehicle activation shape drifted".to_owned());
            };
            validate_identity(vehicle, "activated stage vehicle")?;
            validate_identity(locator, "stage vehicle activation locator")?;
            validate_token(behaviour, "stage vehicle activation behaviour")?;
            Some(MissionStageDirective::ActivateVehicle {
                source_ordinal,
                vehicle_id: vehicle.clone(),
                locator_id: locator.clone(),
                behaviour: behaviour.clone(),
            })
        },
        "sethudicon" => Some(MissionStageDirective::HudIcon {
            source_ordinal,
            sprite_id: required_identity(arguments, "stage HUD icon sprite")?,
        }),
        "setfadeout" => Some(MissionStageDirective::FadeOutLegacyArgument {
            source_ordinal,
            source_value: required_legacy_argument(
                arguments,
                "stage fade-out unused argument",
            )?,
        }),
        "setiriswipe" => Some(MissionStageDirective::IrisWipeLegacyArgument {
            source_ordinal,
            source_value: required_legacy_argument(
                arguments,
                "stage iris-wipe unused argument",
            )?,
        }),
        "setmaxtraffic" => Some(MissionStageDirective::MaxTrafficCars {
            source_ordinal,
            cars: parse_max_traffic(arguments)?,
        }),
        "setvehicleaiparams" => {
            Some(compile_ai_tuning(source_ordinal, arguments, false)?)
        },
        "setstageaitargetcatchupparams" => {
            Some(compile_ai_tuning(source_ordinal, arguments, true)?)
        },
        "addsafezone" => Some(compile_safe_zone(source_ordinal, arguments)?),
        "addstagecharacter" => {
            Some(compile_stage_character(source_ordinal, arguments)?)
        },
        "stayinblack" => {
            require_no_arguments(arguments, "stage stay-in-black marker")?;
            Some(MissionStageDirective::StayInBlack { source_ordinal })
        },
        "setgameover" => {
            require_no_arguments(arguments, "stage game-over marker")?;
            Some(MissionStageDirective::GameOver { source_ordinal })
        },
        "setlevelover" => {
            require_no_arguments(arguments, "stage level-over marker")?;
            Some(MissionStageDirective::LevelOver { source_ordinal })
        },
        "showstagecomplete" => {
            require_no_arguments(arguments, "stage-complete marker")?;
            Some(MissionStageDirective::ShowStageComplete { source_ordinal })
        },
        "disablehitandrun" => {
            require_no_arguments(
                arguments,
                "stage hit-and-run disable marker",
            )?;
            Some(MissionStageDirective::DisableHitAndRun { source_ordinal })
        },
        "addstagewaypoint" => Some(MissionStageDirective::Waypoint {
            source_ordinal,
            locator_id: required_identity(arguments, "stage waypoint locator")?,
        }),
        _ => None,
    };
    Ok(directive)
}

fn compile_stage_kind(
    parameters: &[String],
) -> Result<MissionStageKind, String> {
    match parameters {
        [] => Ok(MissionStageKind::Standard {
            legacy_flags: None,
            final_stage: false,
        }),
        [final_token] if final_token == "final" => {
            Ok(MissionStageKind::Standard {
                legacy_flags: None,
                final_stage: true,
            })
        },
        [flags] => Ok(MissionStageKind::Standard {
            legacy_flags: Some(parse_legacy_stage_flags(flags)?),
            final_stage: false,
        }),
        [flags, final_token] if final_token == "final" => {
            Ok(MissionStageKind::Standard {
                legacy_flags: Some(parse_legacy_stage_flags(flags)?),
                final_stage: true,
            })
        },
        [locked, kind, identity] if locked == "locked" && kind == "car" => {
            validate_identity(identity, "locked-stage vehicle")?;
            Ok(MissionStageKind::LockedVehicle {
                vehicle_id: identity.clone(),
            })
        },
        [locked, kind, identity] if locked == "locked" && kind == "skin" => {
            validate_identity(identity, "locked-stage costume")?;
            Ok(MissionStageKind::LockedCostume {
                costume_id: identity.clone(),
            })
        },
        _ => Err("mission stage header parameters are not reviewed".to_owned()),
    }
}

fn parse_legacy_stage_flags(value: &str) -> Result<u8, String> {
    let flags = value.parse::<u8>().map_err(|_error| {
        "mission legacy stage flags are not numeric".to_owned()
    })?;
    if matches!(
        flags,
        0 | 1
            | 2
            | 3
            | 4
            | 5
            | 7
            | 8
            | 9
            | 10
            | 15
            | 16
            | 18
            | 22
            | 25
            | 36
            | 37
    ) {
        Ok(flags)
    } else {
        Err("mission legacy stage flags are not reviewed".to_owned())
    }
}

fn compile_message(
    kind: &MissionStageKind,
    source_ordinal: usize,
    arguments: &[String],
) -> Result<MissionStageDirective, String> {
    let (raw, unused_argument) = match arguments {
        [raw] => (raw, None),
        [raw, unused] => (raw, Some(unused.clone())),
        _ => return Err("stage message index shape is not reviewed".to_owned()),
    };
    let index = raw
        .parse::<u16>()
        .map_err(|_error| "stage message index is not numeric".to_owned())?;
    let message_kind = match kind {
        MissionStageKind::LockedVehicle { .. }
        | MissionStageKind::LockedCostume { .. } => {
            if index > 19 {
                return Err(
                    "locked-stage message index is out of range".to_owned()
                );
            }
            MissionStageMessageKind::Locked
        },
        MissionStageKind::Standard { .. } => {
            if index > 299 {
                return Err(
                    "stage objective message index is out of range".to_owned()
                );
            }
            MissionStageMessageKind::Objective
        },
    };
    Ok(MissionStageDirective::MessageIndex {
        source_ordinal,
        kind: message_kind,
        index,
        unused_argument,
    })
}

fn compile_vehicle(
    source_ordinal: usize,
    arguments: &[String],
) -> Result<MissionStageVehicleReference, String> {
    let (vehicle_id, locator_id, behaviour, con_file, driver_id) =
        match arguments {
            [vehicle, locator, behaviour, con] => {
                (vehicle, locator, behaviour, con, None)
            },
            [vehicle, locator, behaviour, con, driver] => {
                (vehicle, locator, behaviour, con, Some(driver.clone()))
            },
            _ => {
                return Err(
                    "stage vehicle parameter shape is not reviewed".to_owned()
                );
            },
        };
    validate_identity(vehicle_id, "stage vehicle")?;
    validate_identity(locator_id, "stage vehicle locator")?;
    validate_token(behaviour, "stage vehicle behaviour")?;
    validate_con_path(con_file)?;
    if let Some(driver) = &driver_id {
        validate_identity(driver, "stage vehicle driver")?;
    }
    Ok(MissionStageVehicleReference {
        source_ordinal,
        vehicle_id: vehicle_id.clone(),
        locator_id: locator_id.clone(),
        behaviour: behaviour.clone(),
        con_file: con_file.clone(),
        driver_id,
    })
}

fn compile_collectible_state_prop(
    source_ordinal: usize,
    arguments: &[String],
) -> Result<MissionStageDirective, String> {
    let [prop, locator, state] = arguments else {
        return Err("stage collectible-state-prop shape drifted".to_owned());
    };
    validate_identity(prop, "stage collectible state prop")?;
    validate_identity(locator, "stage collectible state-prop locator")?;
    Ok(MissionStageDirective::CollectibleStateProp {
        source_ordinal,
        prop_id: prop.clone(),
        locator_id: locator.clone(),
        source_state: parse_source_u32(
            state,
            "stage collectible state-prop source state",
        )?,
    })
}

fn compile_countdown_entry(
    source_ordinal: usize,
    arguments: &[String],
) -> Result<MissionStageDirective, String> {
    let [token, duration] = arguments else {
        return Err("stage countdown-sequence shape drifted".to_owned());
    };
    validate_token(token, "stage countdown-sequence token")?;
    let duration_milliseconds =
        parse_source_u32(duration, "stage countdown duration")?;
    if duration_milliseconds == 0 {
        return Err("stage countdown duration must be positive".to_owned());
    }
    Ok(MissionStageDirective::CountdownSequenceEntry {
        source_ordinal,
        token: token.clone(),
        duration_milliseconds,
    })
}

fn compile_place_player_car(
    source_ordinal: usize,
    arguments: &[String],
) -> Result<MissionStageDirective, String> {
    let [vehicle, locator] = arguments else {
        return Err("stage player-car placement shape drifted".to_owned());
    };
    validate_identity(vehicle, "stage player-car vehicle")?;
    validate_identity(locator, "stage player-car locator")?;
    Ok(MissionStageDirective::PlacePlayerCar {
        source_ordinal,
        vehicle_id: vehicle.clone(),
        locator_id: locator.clone(),
    })
}

fn compile_completion_dialog(
    source_ordinal: usize,
    arguments: &[String],
) -> Result<MissionStageDirective, String> {
    let (dialogue, character_id) = match arguments {
        [dialogue] => (dialogue, None),
        [dialogue, character] => (dialogue, Some(character.clone())),
        _ => return Err("stage completion-dialog shape drifted".to_owned()),
    };
    validate_identity(dialogue, "stage completion dialogue")?;
    if let Some(character) = &character_id {
        validate_identity(character, "stage completion-dialog character")?;
    }
    Ok(MissionStageDirective::CompletionDialog {
        source_ordinal,
        dialogue_id: dialogue.clone(),
        character_id,
    })
}

fn compile_music_state(
    source_ordinal: usize,
    arguments: &[String],
) -> Result<MissionStageDirective, String> {
    let [state_name, state_value] = arguments else {
        return Err("stage music-state shape drifted".to_owned());
    };
    validate_identity(state_name, "stage music-state name")?;
    validate_identity(state_value, "stage music-state value")?;
    Ok(MissionStageDirective::MusicState {
        source_ordinal,
        state_name: state_name.clone(),
        state_value: state_value.clone(),
    })
}

fn compile_race_catchup(
    source_ordinal: usize,
    arguments: &[String],
) -> Result<MissionStageDirective, String> {
    let [vehicle, value, first, second, third] = arguments else {
        return Err("stage race catch-up shape drifted".to_owned());
    };
    validate_identity(vehicle, "stage race catch-up vehicle")?;
    let source_value = parse_source_u32(value, "stage race catch-up integer")?;
    if source_value == 0 {
        return Err("stage race catch-up integer must be positive".to_owned());
    }
    for factor in [first, second, third] {
        validate_source_decimal(factor, "stage race catch-up decimal")?;
    }
    Ok(MissionStageDirective::RaceCatchupTuning {
        source_ordinal,
        vehicle_id: vehicle.clone(),
        source_value,
        source_factors: [first.clone(), second.clone(), third.clone()],
    })
}

fn compile_start_countdown(
    source_ordinal: usize,
    arguments: &[String],
) -> Result<MissionStageDirective, String> {
    let (sequence, character_id) = match arguments {
        [sequence] => (sequence, None),
        [sequence, character] => (sequence, Some(character.clone())),
        _ => return Err("stage start-countdown shape drifted".to_owned()),
    };
    validate_identity(sequence, "stage countdown sequence")?;
    if let Some(character) = &character_id {
        validate_identity(character, "stage countdown character")?;
    }
    Ok(MissionStageDirective::StartCountdown {
        source_ordinal,
        sequence_id: sequence.clone(),
        character_id,
    })
}

fn required_source_u64(
    arguments: &[String],
    label: &str,
) -> Result<u64, String> {
    let [value] = arguments else {
        return Err(format!("{label} shape drifted"));
    };
    if value.is_empty() || !value.chars().all(|ch| ch.is_ascii_digit()) {
        return Err(format!("{label} is malformed"));
    }
    value
        .parse::<u64>()
        .map_err(|_error| format!("{label} is out of range"))
}

fn required_positive_source_u32(
    arguments: &[String],
    label: &str,
) -> Result<u32, String> {
    let [value] = arguments else {
        return Err(format!("{label} shape drifted"));
    };
    let parsed = parse_source_u32(value, label)?;
    if parsed == 0 {
        Err(format!("{label} must be positive"))
    } else {
        Ok(parsed)
    }
}

fn required_p3d_path(
    arguments: &[String],
    label: &str,
) -> Result<String, String> {
    let [value] = arguments else {
        return Err(format!("{label} shape drifted"));
    };
    validate_p3d_path(value, label)?;
    Ok(value.clone())
}

fn validate_p3d_path(value: &str, label: &str) -> Result<(), String> {
    if value.is_empty()
        || !value.to_ascii_lowercase().ends_with(".p3d")
        || value.contains("..")
        || value.starts_with('/')
        || value.starts_with(char::from(92))
        || value.chars().any(char::is_control)
    {
        Err(format!("{label} P3D path is malformed"))
    } else {
        Ok(())
    }
}

fn validate_source_decimal(value: &str, label: &str) -> Result<(), String> {
    let mut pieces = value.split('.');
    let whole = pieces.next().unwrap_or_default();
    if whole.is_empty() || !whole.chars().all(|ch| ch.is_ascii_digit()) {
        return Err(format!("{label} is malformed"));
    }
    if let Some(fraction) = pieces.next()
        && (fraction.is_empty()
            || !fraction.chars().all(|ch| ch.is_ascii_digit()))
    {
        return Err(format!("{label} is malformed"));
    }
    if pieces.next().is_some() {
        return Err(format!("{label} is malformed"));
    }
    Ok(())
}

fn compile_stage_character(
    source_ordinal: usize,
    arguments: &[String],
) -> Result<MissionStageDirective, String> {
    let (character, character_locator_id, vehicle, vehicle_locator) =
        match arguments {
            [character, vehicle, vehicle_locator] => {
                (character, None, vehicle, vehicle_locator)
            },
            [character, character_locator, vehicle, vehicle_locator] => (
                character,
                Some(character_locator.clone()),
                vehicle,
                vehicle_locator,
            ),
            _ => return Err("stage character source shape drifted".to_owned()),
        };
    validate_identity(character, "stage character")?;
    if let Some(locator) = &character_locator_id {
        validate_identity(locator, "stage character locator")?;
    }
    validate_identity(vehicle, "stage character vehicle")?;
    validate_identity(vehicle_locator, "stage character vehicle locator")?;
    Ok(MissionStageDirective::StageCharacter {
        source_ordinal,
        character_id: character.clone(),
        character_locator_id,
        vehicle_id: vehicle.clone(),
        vehicle_locator_id: vehicle_locator.clone(),
    })
}

fn compile_ai_tuning(
    source_ordinal: usize,
    arguments: &[String],
    target_catchup: bool,
) -> Result<MissionStageDirective, String> {
    let [vehicle, first, second] = arguments else {
        return Err("stage AI tuning shape drifted".to_owned());
    };
    validate_identity(vehicle, "stage AI tuning vehicle")?;
    let source_first = parse_source_i32(first, "stage AI first source value")?;
    let source_second =
        parse_source_i32(second, "stage AI second source value")?;
    if target_catchup {
        Ok(MissionStageDirective::TargetCatchupTuning {
            source_ordinal,
            vehicle_id: vehicle.clone(),
            source_first,
            source_second,
        })
    } else {
        Ok(MissionStageDirective::VehicleAiTuning {
            source_ordinal,
            vehicle_id: vehicle.clone(),
            source_first,
            source_second,
        })
    }
}

fn compile_safe_zone(
    source_ordinal: usize,
    arguments: &[String],
) -> Result<MissionStageDirective, String> {
    let [locator, value] = arguments else {
        return Err("stage safe-zone shape drifted".to_owned());
    };
    validate_identity(locator, "stage safe-zone locator")?;
    let source_value = parse_source_u32(value, "stage safe-zone source value")?;
    if source_value == 0 {
        return Err("stage safe-zone source value must be positive".to_owned());
    }
    Ok(MissionStageDirective::SafeZone {
        source_ordinal,
        locator_id: locator.clone(),
        source_value,
    })
}

fn parse_source_i32(value: &str, label: &str) -> Result<i32, String> {
    let body = value.strip_prefix('-').unwrap_or(value);
    if body.is_empty() || !body.chars().all(|ch| ch.is_ascii_digit()) {
        return Err(format!("{label} is malformed"));
    }
    value
        .parse::<i32>()
        .map_err(|_error| format!("{label} is out of range"))
}

fn parse_source_u32(value: &str, label: &str) -> Result<u32, String> {
    if value.is_empty() || !value.chars().all(|ch| ch.is_ascii_digit()) {
        return Err(format!("{label} is malformed"));
    }
    value
        .parse::<u32>()
        .map_err(|_error| format!("{label} is out of range"))
}

fn require_no_arguments(
    arguments: &[String],
    label: &str,
) -> Result<(), String> {
    if arguments.is_empty() {
        Ok(())
    } else {
        Err(format!("{label} unexpectedly carries arguments"))
    }
}

fn parse_max_traffic(arguments: &[String]) -> Result<u8, String> {
    let [raw] = arguments else {
        return Err("stage max-traffic shape is not reviewed".to_owned());
    };
    if raw.is_empty() || !raw.chars().all(|ch| ch.is_ascii_digit()) {
        return Err("stage max-traffic value is not numeric".to_owned());
    }
    let cars = raw.parse::<u8>().map_err(|_error| {
        "stage max-traffic value is not numeric".to_owned()
    })?;
    if matches!(cars, 1..=5) {
        Ok(cars)
    } else {
        Err("stage max-traffic value is not reviewed".to_owned())
    }
}

fn required_legacy_argument(
    arguments: &[String],
    label: &str,
) -> Result<String, String> {
    let [value] = arguments else {
        return Err(format!("{label} shape drifted"));
    };
    if value.is_empty() || value.chars().any(char::is_control) {
        Err(format!("{label} is malformed"))
    } else {
        Ok(value.clone())
    }
}

fn parse_nonnegative_seconds(
    arguments: &[String],
    label: &str,
) -> Result<u32, String> {
    let [raw] = arguments else {
        return Err(format!("{label} shape is not reviewed"));
    };
    match raw.parse::<u32>() {
        Ok(seconds) => Ok(seconds),
        Err(_error) => {
            let mut error = label.to_owned();
            error.push_str(" is not numeric");
            Err(error)
        },
    }
}

fn parse_positive_seconds(
    arguments: &[String],
    label: &str,
) -> Result<u32, String> {
    let [raw] = arguments else {
        return Err(format!("{label} shape is not reviewed"));
    };
    let seconds = match raw.parse::<u32>() {
        Ok(seconds) => seconds,
        Err(_error) => {
            let mut error = label.to_owned();
            error.push_str(" is not numeric");
            return Err(error);
        },
    };
    if seconds == 0 {
        Err(format!("{label} must be positive"))
    } else {
        Ok(seconds)
    }
}

fn required_identity(
    arguments: &[String],
    label: &str,
) -> Result<String, String> {
    let [value] = arguments else {
        return Err(format!("{label} shape is not reviewed"));
    };
    validate_identity(value, label)?;
    Ok(value.clone())
}

fn validate_identity(value: &str, label: &str) -> Result<(), String> {
    let valid = !value.is_empty()
        && value.bytes().all(|byte| {
            byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-')
        });
    if valid {
        Ok(())
    } else {
        Err(format!("{label} identity is malformed"))
    }
}

fn validate_token(value: &str, label: &str) -> Result<(), String> {
    if value.is_empty() || value.chars().any(char::is_control) {
        Err(format!("{label} is malformed"))
    } else {
        Ok(())
    }
}

fn validate_con_path(value: &str) -> Result<(), String> {
    if value.is_empty()
        || value.chars().any(char::is_control)
        || !value.to_ascii_lowercase().ends_with(".con")
        || value.contains("..")
        || value.starts_with('/')
        || value.starts_with(char::from(92))
    {
        Err("stage vehicle CON path is malformed".to_owned())
    } else {
        Ok(())
    }
}

#[cfg(test)]
#[path = "../../../../../tests/migration/pipeline/unit/domain/package/mission_stage/tests.rs"]
mod tests;
