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
//   - Reviewed objective-scoped participant, target, route, timing, and fee
//     evidence.
// - Must-Not:
//   - Guess destination, dialogue, FMV, collectible, or presentation semantics.
// - Allows:
//   - Compile documented objective command fields into deterministic evidence.
// - Split-When:
//   - One directive family gains an independent runtime lifecycle.
// - Merge-When:
//   - Mission definition compilation owns this exact semantic boundary.
// - Summary:
//   - Mission objective directive semantic compiler.
// - Description:
//   - Types selected reviewed commands after lossless mission scope projection.
// - Usage:
//   - Runs after mission objective and scope preflight succeeds.
// - Defaults:
//   - Malformed identities, numbers, and reviewed command shapes fail closed.
//

//! Typed compilation of selected objective-scoped mission directives.

use super::super::{MissionScopeObjective, MissionScopeReport};

/// One NPC placed for an objective.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionObjectiveNpcReference {
    source_ordinal: usize,
    npc_id: String,
    locator_id: String,
    unused_argument: Option<String>,
}

impl MissionObjectiveNpcReference {
    /// Return the source statement ordinal.
    #[must_use]
    pub const fn source_ordinal(&self) -> usize {
        self.source_ordinal
    }

    /// Return the exact NPC identity.
    #[must_use]
    pub fn npc_id(&self) -> &str {
        &self.npc_id
    }

    /// Return the exact source locator identity.
    #[must_use]
    pub fn locator_id(&self) -> &str {
        &self.locator_id
    }

    /// Return the optional source argument documented as unused.
    #[must_use]
    pub fn unused_argument(&self) -> Option<&str> {
        self.unused_argument.as_deref()
    }
}

/// Exact conversation-camera token observed in objective-scoped source.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MissionConversationCameraToken {
    /// Exact `pc_far` source token.
    PlayerFar,
    /// Exact `pc_near` source token.
    PlayerNear,
    /// Exact `npc_far` source token.
    NpcFar,
    /// Exact `npc_near` source token.
    NpcNear,
}

/// Exact dialogue-animation character selector observed in source comments.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MissionAmbientAnimationCharacter {
    /// Source selector `0`, documented by the source corpus as player
    /// character.
    Player,
    /// Source selector `1`, documented by the source corpus as NPC.
    Npc,
}

/// One reviewed objective-scoped directive with documented field roles.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MissionObjectiveDirective {
    /// Add one NPC participant at an exact locator.
    Npc(MissionObjectiveNpcReference),
    /// Add one ordered walking waypoint to an objective NPC.
    NpcWaypoint {
        /// Source statement ordinal.
        source_ordinal: usize,
        /// Exact NPC identity.
        npc_id: String,
        /// Exact waypoint locator identity.
        locator_id: String,
    },
    /// Add one NPC as the driver of a mission vehicle.
    Driver {
        /// Source statement ordinal.
        source_ordinal: usize,
        /// Exact NPC identity.
        npc_id: String,
        /// Exact mission vehicle identity.
        vehicle_id: String,
    },
    /// Remove a previously assigned driver by NPC identity.
    RemoveDriver {
        /// Source statement ordinal.
        source_ordinal: usize,
        /// Exact NPC identity.
        npc_id: String,
    },
    /// Remove an objective NPC by identity.
    RemoveNpc {
        /// Source statement ordinal.
        source_ordinal: usize,
        /// Exact NPC identity.
        npc_id: String,
    },
    /// Select the mission vehicle targeted by this objective.
    TargetVehicle {
        /// Source statement ordinal.
        source_ordinal: usize,
        /// Exact vehicle identity, including the special `current` token.
        vehicle_id: String,
    },
    /// Configure which NPC satisfies a `talkto` objective.
    TalkTarget {
        /// Source statement ordinal.
        source_ordinal: usize,
        /// Exact NPC identity.
        npc_id: String,
        /// Optional explicitly authored icon kind, 0 through 2.
        icon: Option<u8>,
        /// Optional exact finite source decimal for vertical icon offset.
        icon_y_offset: Option<String>,
        /// Optional exact positive source decimal for trigger radius.
        trigger_radius: Option<String>,
    },
    /// Queue one ambient animation for the dialogue NPC.
    AmbientNpcAnimation {
        /// Source statement ordinal.
        source_ordinal: usize,
        /// Exact animation identity defined by the character `.cho` data.
        animation_id: String,
    },
    /// Queue one ambient animation for the playable dialogue character.
    AmbientPcAnimation {
        /// Source statement ordinal.
        source_ordinal: usize,
        /// Exact animation identity defined by the character `.cho` data.
        animation_id: String,
    },
    /// Preserve the exact dialogue animation randomization selector pair.
    AmbientAnimationRandomize {
        /// Source statement ordinal.
        source_ordinal: usize,
        /// Reviewed source character selector.
        character: MissionAmbientAnimationCharacter,
        /// Reviewed source randomization flag.
        randomized: bool,
    },
    /// Select the Type-3 locator used to choose the dialogue camera side.
    CameraBestSide {
        /// Source statement ordinal.
        source_ordinal: usize,
        /// Exact Type-3 locator identity from source.
        locator_id: String,
    },
    /// Select one indexed conversation-camera token.
    ConversationCamera {
        /// Source statement ordinal.
        source_ordinal: usize,
        /// Exact source camera slot; current reviewed corpus uses 0 through 6.
        source_slot: u8,
        /// Exact reviewed source camera token.
        camera: MissionConversationCameraToken,
    },
    /// Bind a dialogue conversation to its two authored participants.
    DialogueInfo {
        /// Source statement ordinal.
        source_ordinal: usize,
        /// Exact playable-character source identity.
        player_character_id: String,
        /// Exact other-character source identity.
        npc_character_id: String,
        /// Exact conversation source identity.
        dialogue_id: String,
        /// Exact observed fourth legacy value; current corpus is always zero.
        legacy_zero: String,
    },
    /// Preserve the three authored dialogue locators and optional legacy flag.
    DialoguePositions {
        /// Source statement ordinal.
        source_ordinal: usize,
        /// Three exact locator identities in source order.
        locator_ids: [String; 3],
        /// Optional fourth source flag; current reviewed corpus uses only `1`.
        legacy_flag: Option<String>,
    },
    /// Preserve the positive distance value authored for a `losetail`
    /// objective.
    ObjectDistance {
        /// Source statement ordinal.
        source_ordinal: usize,
        /// Exact positive integer source value; units remain unresolved.
        source_value: u32,
    },
    /// Preserve the positive par-time value authored for a race objective.
    ParTime {
        /// Source statement ordinal.
        source_ordinal: usize,
        /// Exact positive integer source value; runtime units remain
        /// unresolved.
        source_value: u32,
    },
    /// Select the exact pickup target identity for a `pickupitem` objective.
    PickupTarget {
        /// Source statement ordinal.
        source_ordinal: usize,
        /// Exact source target identity.
        target_id: String,
    },
    /// Preserve an authored `TurnGotoDialogOff` marker without inventing
    /// policy.
    TurnGotoDialogOff {
        /// Source statement ordinal.
        source_ordinal: usize,
    },
    /// Preserve an authored `MustActionTrigger` marker without inventing
    /// policy.
    MustActionTrigger {
        /// Source statement ordinal.
        source_ordinal: usize,
    },
    /// Preserve an authored `AllowRockOut` marker without inventing policy.
    AllowRockOut {
        /// Source statement ordinal.
        source_ordinal: usize,
    },
    /// Select one objective destination and optional presentation marker.
    Destination {
        /// Source statement ordinal.
        source_ordinal: usize,
        /// Exact source destination identity.
        destination_id: String,
        /// Optional exact source marker/drawable identity.
        marker_id: Option<String>,
    },
    /// Select an exact P3D presentation bitmap source.
    PresentationBitmap {
        /// Source statement ordinal.
        source_ordinal: usize,
        /// Exact repository-relative `.p3d` source path.
        p3d_path: String,
    },
    /// Select an exact mission FMV source while preserving legacy options.
    FmvInfo {
        /// Source statement ordinal.
        source_ordinal: usize,
        /// Exact relative `.rmv` source path.
        rmv_path: String,
        /// Optional second source argument whose runtime semantics remain
        /// opaque.
        legacy_argument: Option<String>,
    },
    /// Configure a timer objective duration.
    Duration {
        /// Source statement ordinal.
        source_ordinal: usize,
        /// Exact positive finite source decimal in seconds.
        source_seconds: String,
    },
    /// Configure the positive lap count carried by a race objective.
    RaceLaps {
        /// Source statement ordinal.
        source_ordinal: usize,
        /// Exact positive source lap count.
        laps: u32,
    },
    /// Configure the positive entry fee carried by a coin objective.
    CoinFee {
        /// Source statement ordinal.
        source_ordinal: usize,
        /// Exact nonzero source coin fee.
        coins: u32,
    },
    /// Add one source collectible while preserving legacy extension fields.
    Collectible {
        /// Source statement ordinal.
        source_ordinal: usize,
        /// Exact Type-0 locator identity used for the collectible.
        locator_id: String,
        /// Explicit composite drawable identity when authored.
        drawable_id: Option<String>,
        /// Additional observed source fields whose semantics remain
        /// unresolved.
        legacy_arguments: Vec<String>,
    },
    /// Select an explicitly authored collectible effect identity.
    CollectibleEffect {
        /// Source statement ordinal.
        source_ordinal: usize,
        /// Exact source effect identity.
        effect_id: String,
    },
    /// Bind one collectible index to one stage-waypoint index.
    BindCollectibleToWaypoint {
        /// Source statement ordinal.
        source_ordinal: usize,
        /// Zero-based collectible index.
        collectible_index: u32,
        /// Zero-based stage-waypoint index.
        waypoint_index: u32,
    },
}

/// Typed selected directives for one projected objective.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionObjectiveSemanticBinding {
    source_ordinal: usize,
    source_alias: String,
    canonical_kind: Option<&'static str>,
    unavailable_code: Option<&'static str>,
    directives: Vec<MissionObjectiveDirective>,
}

impl MissionObjectiveSemanticBinding {
    /// Return the source `AddObjective` ordinal.
    #[must_use]
    pub const fn source_ordinal(&self) -> usize {
        self.source_ordinal
    }

    /// Return the exact source objective alias.
    #[must_use]
    pub fn source_alias(&self) -> &str {
        &self.source_alias
    }

    /// Return the canonical runtime objective kind when mapping is complete.
    #[must_use]
    pub const fn canonical_kind(&self) -> Option<&'static str> {
        self.canonical_kind
    }

    /// Return the explicit unavailable identity when no runtime kind exists.
    #[must_use]
    pub const fn unavailable_code(&self) -> Option<&'static str> {
        self.unavailable_code
    }

    /// Return selected typed directives in source order.
    #[must_use]
    pub fn directives(&self) -> &[MissionObjectiveDirective] {
        &self.directives
    }
}

/// One objective waypoint bound to its prior NPC declaration.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionObjectiveNpcWaypointBinding {
    source_ordinal: usize,
    declaration_source_ordinal: usize,
    npc_id: String,
    npc_locator_id: String,
    waypoint_locator_id: String,
}

impl MissionObjectiveNpcWaypointBinding {
    /// Return the waypoint source ordinal.
    #[must_use]
    pub const fn source_ordinal(&self) -> usize {
        self.source_ordinal
    }

    /// Return the matched `AddNPC` source ordinal.
    #[must_use]
    pub const fn declaration_source_ordinal(&self) -> usize {
        self.declaration_source_ordinal
    }

    /// Return the exact authored NPC identity.
    #[must_use]
    pub fn npc_id(&self) -> &str {
        &self.npc_id
    }

    /// Return the NPC placement locator identity.
    #[must_use]
    pub fn npc_locator_id(&self) -> &str {
        &self.npc_locator_id
    }

    /// Return the exact walking waypoint locator identity.
    #[must_use]
    pub fn waypoint_locator_id(&self) -> &str {
        &self.waypoint_locator_id
    }
}

/// All objective NPC waypoints resolved to prior declarations.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct MissionObjectiveNpcWaypointReport {
    waypoints: Vec<MissionObjectiveNpcWaypointBinding>,
}

impl MissionObjectiveNpcWaypointReport {
    /// Return waypoint bindings in authored objective order.
    #[must_use]
    pub fn waypoints(&self) -> &[MissionObjectiveNpcWaypointBinding] {
        &self.waypoints
    }
}

/// Typed selected objective directives for all projected mission stages.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionObjectiveSemanticReport {
    objectives: Vec<MissionObjectiveSemanticBinding>,
}

impl MissionObjectiveSemanticReport {
    /// Return typed objective bindings in mission/source order.
    #[must_use]
    pub fn objectives(&self) -> &[MissionObjectiveSemanticBinding] {
        &self.objectives
    }

    #[cfg(test)]
    pub(crate) fn from_route_entries_for_tests(
        entries: Vec<(usize, String, Vec<MissionObjectiveDirective>)>,
    ) -> Self {
        Self {
            objectives: entries
                .into_iter()
                .map(|(source_ordinal, source_alias, directives)| {
                    let schema = super::objective_alias_schema(&source_alias);
                    MissionObjectiveSemanticBinding {
                        source_ordinal,
                        source_alias,
                        canonical_kind:
                            schema.and_then(|item| item.canonical_kind),
                        unavailable_code:
                            schema.and_then(|item| item.unavailable_code),
                        directives,
                    }
                })
                .collect(),
        }
    }
}

/// Bind each objective NPC waypoint to one unique prior `AddNPC`.
///
/// # Errors
///
/// Fails when a waypoint has no unique prior declaration with the same NPC id.
pub fn preflight_mission_objective_npc_waypoints(
    report: &MissionObjectiveSemanticReport,
) -> Result<MissionObjectiveNpcWaypointReport, String> {
    let mut waypoints = Vec::new();
    for objective in report.objectives() {
        let declarations = objective
            .directives()
            .iter()
            .filter_map(|directive| match directive {
                MissionObjectiveDirective::Npc(reference) => Some(reference),
                _ => None,
            })
            .collect::<Vec<_>>();
        for directive in objective.directives() {
            let MissionObjectiveDirective::NpcWaypoint {
                source_ordinal,
                npc_id,
                locator_id,
            } = directive
            else {
                continue;
            };
            let matching = declarations
                .iter()
                .filter(|declaration| {
                    declaration.npc_id() == npc_id
                        && declaration.source_ordinal() < *source_ordinal
                })
                .copied()
                .collect::<Vec<_>>();
            let [declaration] = matching.as_slice() else {
                return Err(
                    concat!(
                        "mission objective NPC waypoint has no unique ",
                        "prior declaration"
                    )
                    .to_owned(),
                );
            };
            waypoints.push(MissionObjectiveNpcWaypointBinding {
                source_ordinal: *source_ordinal,
                declaration_source_ordinal: declaration.source_ordinal(),
                npc_id: npc_id.clone(),
                npc_locator_id: declaration.locator_id().to_owned(),
                waypoint_locator_id: locator_id.clone(),
            });
        }
    }
    Ok(MissionObjectiveNpcWaypointReport { waypoints })
}

/// Compile selected reviewed objective-scoped command semantics.
///
/// # Errors
/// Returns an error when a selected value falls outside its reviewed contract.
pub fn preflight_mission_objective_semantics(
    scopes: &MissionScopeReport,
) -> Result<MissionObjectiveSemanticReport, String> {
    let mut objectives = Vec::new();
    for mission in scopes.missions() {
        for stage in mission.stages() {
            objectives.push(compile_objective(stage.objective())?);
        }
    }
    Ok(MissionObjectiveSemanticReport { objectives })
}

fn compile_objective(
    objective: &MissionScopeObjective,
) -> Result<MissionObjectiveSemanticBinding, String> {
    let source_alias = objective.binding().source_alias();
    let mut directives = Vec::new();
    for command in objective.commands() {
        match compile_directive(
            source_alias,
            command.ordinal(),
            command.command(),
            command.arguments(),
        )? {
            Some(directive) => directives.push(directive),
            None if command_has_external_semantic_owner(command.command()) => {
            },
            None => {
                return Err(
                    "reviewed objective command lacks typed semantic owner"
                        .to_owned(),
                );
            },
        }
    }
    Ok(MissionObjectiveSemanticBinding {
        source_ordinal: objective.binding().ordinal(),
        source_alias: source_alias.to_owned(),
        canonical_kind: objective.binding().canonical_kind(),
        unavailable_code: objective.binding().unavailable_code(),
        directives,
    })
}

fn compile_directive(
    source_alias: &str,
    source_ordinal: usize,
    command: &str,
    arguments: &[String],
) -> Result<Option<MissionObjectiveDirective>, String> {
    let directive = match command {
        "addnpc" => Some(MissionObjectiveDirective::Npc(compile_npc(
            source_ordinal,
            arguments,
        )?)),
        "addobjectivenpcwaypoint" => {
            let [npc, locator] = arguments else {
                return Err(
                    "mission objective NPC waypoint shape drifted".to_owned()
                );
            };
            validate_identity(npc, "mission objective waypoint NPC")?;
            validate_identity(locator, "mission objective waypoint locator")?;
            Some(MissionObjectiveDirective::NpcWaypoint {
                source_ordinal,
                npc_id: npc.clone(),
                locator_id: locator.clone(),
            })
        },
        "adddriver" => {
            let [npc, vehicle] = arguments else {
                return Err("mission objective driver shape drifted".to_owned());
            };
            validate_identity(npc, "mission objective driver NPC")?;
            validate_identity(vehicle, "mission objective driver vehicle")?;
            Some(MissionObjectiveDirective::Driver {
                source_ordinal,
                npc_id: npc.clone(),
                vehicle_id: vehicle.clone(),
            })
        },
        "removedriver" => Some(MissionObjectiveDirective::RemoveDriver {
            source_ordinal,
            npc_id: required_identity(
                arguments,
                "mission objective removed driver",
            )?,
        }),
        "removenpc" => Some(MissionObjectiveDirective::RemoveNpc {
            source_ordinal,
            npc_id: required_identity(
                arguments,
                "mission objective removed NPC",
            )?,
        }),
        "setobjtargetvehicle" => {
            Some(MissionObjectiveDirective::TargetVehicle {
                source_ordinal,
                vehicle_id: required_identity(
                    arguments,
                    "mission objective target vehicle",
                )?,
            })
        },
        "settalktotarget" => {
            Some(compile_talk_target(source_ordinal, arguments)?)
        },
        "addambientnpcanimation" => {
            Some(MissionObjectiveDirective::AmbientNpcAnimation {
                source_ordinal,
                animation_id: required_identity(
                    arguments,
                    "mission NPC ambient animation",
                )?,
            })
        },
        "addambientpcanimation" => {
            Some(MissionObjectiveDirective::AmbientPcAnimation {
                source_ordinal,
                animation_id: required_identity(
                    arguments,
                    "mission player ambient animation",
                )?,
            })
        },
        "ambientanimationrandomize" => Some(
            compile_ambient_animation_randomize(source_ordinal, arguments)?,
        ),
        "setcambestside" => Some(MissionObjectiveDirective::CameraBestSide {
            source_ordinal,
            locator_id: required_identity(
                arguments,
                "mission dialogue camera best-side locator",
            )?,
        }),
        "setconversationcam" => {
            Some(compile_conversation_camera(source_ordinal, arguments)?)
        },
        "setdialogueinfo" => {
            Some(compile_dialogue_info(source_ordinal, arguments)?)
        },
        "setdialoguepositions" => {
            Some(compile_dialogue_positions(source_ordinal, arguments)?)
        },
        "setobjdistance" => Some(MissionObjectiveDirective::ObjectDistance {
            source_ordinal,
            source_value: required_positive_u32(
                arguments,
                "mission objective source distance",
            )?,
        }),
        "setpartime" => Some(MissionObjectiveDirective::ParTime {
            source_ordinal,
            source_value: required_positive_u32(
                arguments,
                "mission objective par time",
            )?,
        }),
        "setpickuptarget" => Some(MissionObjectiveDirective::PickupTarget {
            source_ordinal,
            target_id: required_identity(
                arguments,
                "mission objective pickup target",
            )?,
        }),
        "turngotodialogoff" => {
            require_no_arguments(
                arguments,
                "mission TurnGotoDialogOff marker",
            )?;
            Some(MissionObjectiveDirective::TurnGotoDialogOff {
                source_ordinal,
            })
        },
        "mustactiontrigger" => {
            require_no_arguments(
                arguments,
                "mission MustActionTrigger marker",
            )?;
            Some(MissionObjectiveDirective::MustActionTrigger {
                source_ordinal,
            })
        },
        "allowrockout" => {
            require_no_arguments(arguments, "mission AllowRockOut marker")?;
            Some(MissionObjectiveDirective::AllowRockOut { source_ordinal })
        },
        "setdestination" => {
            Some(compile_destination(source_ordinal, arguments)?)
        },
        "setpresentationbitmap" => {
            Some(MissionObjectiveDirective::PresentationBitmap {
                source_ordinal,
                p3d_path: required_relative_asset_path(
                    arguments,
                    ".p3d",
                    "mission presentation bitmap",
                )?,
            })
        },
        "setfmvinfo" => Some(compile_fmv_info(source_ordinal, arguments)?),
        "setdurationtime" => Some(MissionObjectiveDirective::Duration {
            source_ordinal,
            source_seconds: required_positive_decimal(
                arguments,
                "mission objective duration",
            )?,
        }),
        "setracelaps" => Some(MissionObjectiveDirective::RaceLaps {
            source_ordinal,
            laps: required_positive_u32(arguments, "mission race laps")?,
        }),
        "setcoinfee" => Some(MissionObjectiveDirective::CoinFee {
            source_ordinal,
            coins: required_positive_u32(arguments, "mission coin fee")?,
        }),
        "addcollectible" => {
            Some(compile_collectible(source_ordinal, arguments)?)
        },
        "setcollectibleeffect" => {
            Some(MissionObjectiveDirective::CollectibleEffect {
                source_ordinal,
                effect_id: required_identity(
                    arguments,
                    "mission collectible effect",
                )?,
            })
        },
        "bindcollectibleto" => {
            let [collectible, waypoint] = arguments else {
                return Err(
                    "mission collectible binding shape drifted".to_owned()
                );
            };
            Some(MissionObjectiveDirective::BindCollectibleToWaypoint {
                source_ordinal,
                collectible_index: parse_u32(
                    collectible,
                    "mission collectible index",
                )?,
                waypoint_index: parse_u32(waypoint, "mission waypoint index")?,
            })
        },
        _ => None,
    };
    if directive.is_some()
        && !command_is_allowed_for_alias(source_alias, command)
    {
        return Err(
            "mission typed objective directive crossed alias scope".to_owned()
        );
    }
    Ok(directive)
}

fn compile_ambient_animation_randomize(
    source_ordinal: usize,
    arguments: &[String],
) -> Result<MissionObjectiveDirective, String> {
    let [character, randomize] = arguments else {
        return Err(
            "mission ambient-animation selector shape drifted".to_owned()
        );
    };
    let character = match character.as_str() {
        "0" => MissionAmbientAnimationCharacter::Player,
        "1" => MissionAmbientAnimationCharacter::Npc,
        _ => {
            return Err(
                "mission ambient-animation character selector is not reviewed"
                    .to_owned(),
            );
        },
    };
    let randomized = match randomize.as_str() {
        "0" => false,
        "1" => true,
        _ => {
            return Err(
                "mission ambient-animation randomization flag is not reviewed"
                    .to_owned(),
            );
        },
    };
    Ok(MissionObjectiveDirective::AmbientAnimationRandomize {
        source_ordinal,
        character,
        randomized,
    })
}

fn compile_conversation_camera(
    source_ordinal: usize,
    arguments: &[String],
) -> Result<MissionObjectiveDirective, String> {
    let [slot, camera] = arguments else {
        return Err("mission conversation-camera shape drifted".to_owned());
    };
    if slot.is_empty() || !slot.chars().all(|ch| ch.is_ascii_digit()) {
        return Err("mission conversation-camera slot is malformed".to_owned());
    }
    let source_slot = slot.parse::<u8>().map_err(|_error| {
        "mission conversation-camera slot is malformed".to_owned()
    })?;
    if source_slot > 6 {
        return Err(
            "mission conversation-camera slot is not reviewed".to_owned()
        );
    }
    let camera = match camera.as_str() {
        "pc_far" => MissionConversationCameraToken::PlayerFar,
        "pc_near" => MissionConversationCameraToken::PlayerNear,
        "npc_far" => MissionConversationCameraToken::NpcFar,
        "npc_near" => MissionConversationCameraToken::NpcNear,
        _ => {
            return Err(
                "mission conversation-camera token is not reviewed".to_owned()
            );
        },
    };
    Ok(MissionObjectiveDirective::ConversationCamera {
        source_ordinal,
        source_slot,
        camera,
    })
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

fn compile_dialogue_info(
    source_ordinal: usize,
    arguments: &[String],
) -> Result<MissionObjectiveDirective, String> {
    let [player, npc, dialogue, legacy_zero] = arguments else {
        return Err("mission dialogue-info shape drifted".to_owned());
    };
    validate_identity(player, "mission dialogue player")?;
    validate_identity(npc, "mission dialogue NPC")?;
    validate_identity(dialogue, "mission dialogue identity")?;
    if legacy_zero != "0" {
        return Err("mission dialogue legacy value is not reviewed".to_owned());
    }
    Ok(MissionObjectiveDirective::DialogueInfo {
        source_ordinal,
        player_character_id: player.clone(),
        npc_character_id: npc.clone(),
        dialogue_id: dialogue.clone(),
        legacy_zero: legacy_zero.clone(),
    })
}

fn compile_dialogue_positions(
    source_ordinal: usize,
    arguments: &[String],
) -> Result<MissionObjectiveDirective, String> {
    let (a, b, c, legacy_flag) = match arguments {
        [a, b, c] => (a, b, c, None),
        [a, b, c, flag] if flag == "1" => (a, b, c, Some(flag.clone())),
        [_, _, _, _] => {
            return Err(
                "mission dialogue-position legacy flag is not reviewed"
                    .to_owned(),
            );
        },
        _ => return Err("mission dialogue-position shape drifted".to_owned()),
    };
    for locator in [a, b, c] {
        validate_identity(locator, "mission dialogue locator")?;
    }
    Ok(MissionObjectiveDirective::DialoguePositions {
        source_ordinal,
        locator_ids: [a.clone(), b.clone(), c.clone()],
        legacy_flag,
    })
}

fn compile_destination(
    source_ordinal: usize,
    arguments: &[String],
) -> Result<MissionObjectiveDirective, String> {
    let (destination, marker_id) = match arguments {
        [destination] => (destination, None),
        [destination, marker] => {
            validate_identity(marker, "mission destination marker")?;
            (destination, Some(marker.clone()))
        },
        _ => return Err("mission destination shape drifted".to_owned()),
    };
    validate_identity(destination, "mission destination")?;
    Ok(MissionObjectiveDirective::Destination {
        source_ordinal,
        destination_id: destination.clone(),
        marker_id,
    })
}

fn compile_fmv_info(
    source_ordinal: usize,
    arguments: &[String],
) -> Result<MissionObjectiveDirective, String> {
    let (rmv_path, legacy_argument) = match arguments {
        [path] => (path, None),
        [path, legacy] => {
            validate_legacy_text(legacy, "mission FMV legacy argument")?;
            (path, Some(legacy.clone()))
        },
        _ => return Err("mission FMV info shape drifted".to_owned()),
    };
    validate_relative_asset_path(rmv_path, ".rmv", "mission FMV")?;
    Ok(MissionObjectiveDirective::FmvInfo {
        source_ordinal,
        rmv_path: rmv_path.clone(),
        legacy_argument,
    })
}

fn compile_collectible(
    source_ordinal: usize,
    arguments: &[String],
) -> Result<MissionObjectiveDirective, String> {
    let (locator, drawable, legacy_arguments) = match arguments {
        [locator] => (locator, None, Vec::new()),
        [locator, drawable] => (locator, Some(drawable), Vec::new()),
        [locator, drawable, legacy] => {
            validate_legacy_text(
                legacy,
                "mission collectible legacy argument",
            )?;
            (locator, Some(drawable), vec![legacy.clone()])
        },
        [locator, drawable, legacy_a, legacy_b] => {
            validate_legacy_text(
                legacy_a,
                "mission collectible first legacy argument",
            )?;
            validate_legacy_text(
                legacy_b,
                "mission collectible second legacy argument",
            )?;
            (locator, Some(drawable), vec![
                legacy_a.clone(),
                legacy_b.clone(),
            ])
        },
        _ => return Err("mission collectible shape drifted".to_owned()),
    };
    validate_identity(locator, "mission collectible locator")?;
    if let Some(drawable) = drawable {
        validate_identity(drawable, "mission collectible drawable")?;
    }
    Ok(MissionObjectiveDirective::Collectible {
        source_ordinal,
        locator_id: locator.clone(),
        drawable_id: drawable.cloned(),
        legacy_arguments,
    })
}

fn compile_npc(
    source_ordinal: usize,
    arguments: &[String],
) -> Result<MissionObjectiveNpcReference, String> {
    let (npc, locator, unused_argument) = match arguments {
        [npc, locator] => (npc, locator, None),
        [npc, locator, unused] => {
            validate_legacy_text(
                unused,
                "mission objective NPC unused argument",
            )?;
            (npc, locator, Some(unused.clone()))
        },
        _ => return Err("mission objective NPC shape drifted".to_owned()),
    };
    validate_identity(npc, "mission objective NPC")?;
    validate_identity(locator, "mission objective NPC locator")?;
    Ok(MissionObjectiveNpcReference {
        source_ordinal,
        npc_id: npc.clone(),
        locator_id: locator.clone(),
        unused_argument,
    })
}

fn compile_talk_target(
    source_ordinal: usize,
    arguments: &[String],
) -> Result<MissionObjectiveDirective, String> {
    let (npc, icon, icon_y_offset, trigger_radius) = match arguments {
        [npc] => (npc, None, None, None),
        [npc, icon, offset] => (
            npc,
            Some(parse_icon(icon)?),
            Some(validate_finite_decimal(
                offset,
                "mission talk-target icon Y offset",
            )?),
            None,
        ),
        [npc, icon, offset, radius] => (
            npc,
            Some(parse_icon(icon)?),
            Some(validate_finite_decimal(
                offset,
                "mission talk-target icon Y offset",
            )?),
            Some(validate_positive_decimal(
                radius,
                "mission talk-target trigger radius",
            )?),
        ),
        _ => return Err("mission talk-target shape drifted".to_owned()),
    };
    validate_identity(npc, "mission talk-target NPC")?;
    Ok(MissionObjectiveDirective::TalkTarget {
        source_ordinal,
        npc_id: npc.clone(),
        icon,
        icon_y_offset,
        trigger_radius,
    })
}

fn parse_icon(value: &str) -> Result<u8, String> {
    let icon = value
        .parse::<u8>()
        .map_err(|_error| "mission talk-target icon is malformed".to_owned())?;
    if icon <= 2 {
        Ok(icon)
    } else {
        Err("mission talk-target icon is not reviewed".to_owned())
    }
}

fn required_identity(
    arguments: &[String],
    label: &str,
) -> Result<String, String> {
    let [value] = arguments else {
        return Err(format!("{label} shape drifted"));
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

fn validate_legacy_text(value: &str, label: &str) -> Result<(), String> {
    if value.is_empty() || value.chars().any(char::is_control) {
        Err(format!("{label} is malformed"))
    } else {
        Ok(())
    }
}

fn required_relative_asset_path(
    arguments: &[String],
    extension: &str,
    label: &str,
) -> Result<String, String> {
    let [value] = arguments else {
        return Err(format!("{label} shape drifted"));
    };
    validate_relative_asset_path(value, extension, label)?;
    Ok(value.clone())
}

fn validate_relative_asset_path(
    value: &str,
    extension: &str,
    label: &str,
) -> Result<(), String> {
    let normalized = value.replace(char::from(92), "/");
    let valid = !normalized.is_empty()
        && normalized.to_ascii_lowercase().ends_with(extension)
        && !normalized.starts_with('/')
        && !normalized.split('/').any(|segment| {
            segment.is_empty() || segment == "." || segment == ".."
        })
        && !normalized.contains(':')
        && !normalized.chars().any(char::is_control);
    if valid {
        Ok(())
    } else {
        Err(format!("{label} asset path is malformed"))
    }
}

fn required_positive_decimal(
    arguments: &[String],
    label: &str,
) -> Result<String, String> {
    let [value] = arguments else {
        return Err(format!("{label} shape drifted"));
    };
    validate_positive_decimal(value, label)
}

fn is_plain_decimal(value: &str, allow_negative: bool) -> bool {
    let body = match value.strip_prefix('-') {
        Some(rest) if allow_negative => rest,
        Some(_rest) => return false,
        None => value,
    };
    let mut parts = body.split('.');
    let Some(integer) = parts.next() else {
        return false;
    };
    if integer.is_empty() || !integer.chars().all(|ch| ch.is_ascii_digit()) {
        return false;
    }
    parts.next().is_none_or(|fraction| {
        !fraction.is_empty()
            && fraction.chars().all(|ch| ch.is_ascii_digit())
            && parts.next().is_none()
    })
}

fn validate_positive_decimal(
    value: &str,
    label: &str,
) -> Result<String, String> {
    if !is_plain_decimal(value, false) {
        return Err(format!("{label} source decimal is malformed"));
    }
    if !value
        .bytes()
        .any(|byte| byte.is_ascii_digit() && byte != b'0')
    {
        return Err(format!("{label} must be positive"));
    }
    Ok(value.to_owned())
}

fn validate_finite_decimal(value: &str, label: &str) -> Result<String, String> {
    if !is_plain_decimal(value, true) {
        return Err(format!("{label} source decimal is malformed"));
    }
    Ok(value.to_owned())
}

fn required_positive_u32(
    arguments: &[String],
    label: &str,
) -> Result<u32, String> {
    let [value] = arguments else {
        return Err(format!("{label} shape drifted"));
    };
    let parsed = parse_u32(value, label)?;
    if parsed == 0 {
        Err(format!("{label} must be positive"))
    } else {
        Ok(parsed)
    }
}

fn parse_u32(value: &str, label: &str) -> Result<u32, String> {
    if value.is_empty() || !value.chars().all(|ch| ch.is_ascii_digit()) {
        return Err(format!("{label} is malformed"));
    }
    value
        .parse::<u32>()
        .map_err(|_error| format!("{label} is malformed"))
}

fn command_has_external_semantic_owner(command: &str) -> bool {
    super::super::mission_stage::objective_command_has_stage_semantics(command)
        || matches!(command, "addcondition" | "closecondition")
}

fn command_is_allowed_for_alias(source_alias: &str, command: &str) -> bool {
    super::modifier::objective_modifier_schema(source_alias, command).is_some()
}

#[cfg(test)]
#[path = "../../../../../../tests/migration/pipeline/unit/domain/package/mission_objective/directive_tests.rs"]
mod tests;
