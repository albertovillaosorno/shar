// Copyright:
//   - Copyright © 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT
//
// Boundary-Contract:
// - Owns:
//   - Deterministic mission, stage, objective, and condition scope projection.
// - Must-Not:
//   - Infer legacy parameter meaning, transitions, rewards, or Unreal assets.
// - Allows:
//   - Bind already-reviewed aliases and scoped commands to source structure.
// - Split-When:
//   - Typed parameter or transition compilation gains its own schema lifecycle.
// - Merge-When:
//   - Final mission compilation owns this exact lossless scope projection.
// - Summary:
//   - Mission scope graph compiler.
// - Description:
//   - Projects validated v3 mission evidence into deterministic source scopes.
// - Usage:
//   - Runs after structural, objective, and condition semantic preflight.
// - Defaults:
//   - Missing root objectives, scope drift, or unconsumed evidence fails
//     closed.
//

//! Lossless mission scope graph projection before typed gameplay compilation.

use std::collections::BTreeMap;

use super::mission_condition::{
    MissionConditionBinding, MissionConditionCommandBinding,
    MissionConditionParameterBinding, preflight_mission_condition_commands,
    preflight_mission_condition_parameters, preflight_mission_conditions,
};
use super::mission_objective::{
    MissionObjectiveBinding, MissionObjectiveCommandBinding,
    MissionObjectiveParameterBinding, preflight_mission_objective_commands,
    preflight_mission_objective_parameters, preflight_mission_objectives,
};
use super::mission_script::{MissionCommandInvocation, MissionScriptEvidence};

const L2_ORPHAN_CONDITION_CLOSE: &str =
    "legacy-l2-m6sdi-ignore-orphan-condition-close-v1";
const L7_KEEPBARREL_CLOSE: &str =
    "legacy-l7-m7i-close-keepbarrel-before-stage-complete-v1";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum DirectScope {
    Unscoped,
    Mission,
    Stage,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct DirectCommandSchema {
    scope: DirectScope,
    command: &'static str,
    argument_counts: &'static [usize],
}

const DIRECT_COMMANDS: &[DirectCommandSchema] = &[
    direct(DirectScope::Unscoped, "initlevelplayervehicle", &[3, 4]),
    direct(DirectScope::Mission, "addcollectiblestateprop", &[3]),
    direct(DirectScope::Mission, "initlevelplayervehicle", &[3]),
    direct(DirectScope::Mission, "placeplayercar", &[2]),
    direct(DirectScope::Mission, "setanimatedcameraname", &[1]),
    direct(DirectScope::Mission, "setanimcammulticontname", &[1]),
    direct(DirectScope::Mission, "setdynaloaddata", &[1, 2]),
    direct(DirectScope::Mission, "setforcedcar", &[0]),
    direct(DirectScope::Mission, "setinitialwalk", &[1]),
    direct(DirectScope::Mission, "setmissionresetplayerincar", &[1]),
    direct(DirectScope::Mission, "setmissionresetplayeroutcar", &[2]),
    direct(DirectScope::Mission, "setmissionstartcameraname", &[1]),
    direct(DirectScope::Mission, "setmissionstartmulticontname", &[1]),
    direct(DirectScope::Mission, "setnumvalidfailurehints", &[1]),
    direct(DirectScope::Mission, "setpresentationbitmap", &[1]),
    direct(DirectScope::Mission, "showhud", &[1]),
    direct(DirectScope::Mission, "streetracepropsload", &[1]),
    direct(DirectScope::Mission, "streetracepropsunload", &[1]),
    direct(DirectScope::Mission, "usepedgroup", &[1]),
    direct(DirectScope::Stage, "activatevehicle", &[3]),
    direct(DirectScope::Stage, "addcollectiblestateprop", &[3]),
    direct(DirectScope::Stage, "addsafezone", &[2]),
    direct(DirectScope::Stage, "addstagecharacter", &[3, 4]),
    direct(DirectScope::Stage, "addstagemusicchange", &[0]),
    direct(DirectScope::Stage, "addstagetime", &[1]),
    direct(DirectScope::Stage, "addstagevehicle", &[4, 5]),
    direct(DirectScope::Stage, "addstagewaypoint", &[1]),
    direct(DirectScope::Stage, "addtocountdownsequence", &[2]),
    direct(DirectScope::Stage, "allowmissionabort", &[1]),
    direct(DirectScope::Stage, "disablehitandrun", &[0]),
    direct(DirectScope::Stage, "gotopsscreenwhendone", &[0]),
    direct(DirectScope::Stage, "notrafficforstage", &[0]),
    direct(DirectScope::Stage, "placeplayercar", &[2]),
    direct(DirectScope::Stage, "putmfplayerincar", &[0]),
    direct(DirectScope::Stage, "reset_to_here", &[0]),
    direct(DirectScope::Stage, "setcharactertohide", &[1]),
    direct(DirectScope::Stage, "setcompletiondialog", &[1, 2]),
    direct(DirectScope::Stage, "setdemolooptime", &[1]),
    direct(DirectScope::Stage, "setfadeout", &[1]),
    direct(DirectScope::Stage, "sethudicon", &[1]),
    direct(DirectScope::Stage, "setiriswipe", &[1]),
    direct(DirectScope::Stage, "setmaxtraffic", &[1]),
    direct(DirectScope::Stage, "setmusicstate", &[2]),
    direct(DirectScope::Stage, "setpresentationbitmap", &[1]),
    direct(DirectScope::Stage, "setraceenteryfee", &[1]),
    direct(DirectScope::Stage, "setstageairacecatchupparams", &[5]),
    direct(DirectScope::Stage, "setstageaitargetcatchupparams", &[3]),
    direct(DirectScope::Stage, "setstagemessageindex", &[1]),
    direct(DirectScope::Stage, "setstagemusicalwayson", &[0]),
    direct(DirectScope::Stage, "setstagetime", &[1]),
    direct(DirectScope::Stage, "setswapdefaultcarlocator", &[1]),
    direct(DirectScope::Stage, "setswapforcedcarlocator", &[1]),
    direct(DirectScope::Stage, "setswapplayerlocator", &[1]),
    direct(DirectScope::Stage, "setvehicleaiparams", &[3]),
    direct(DirectScope::Stage, "showstagecomplete", &[0]),
    direct(DirectScope::Stage, "stagestartmusicevent", &[1]),
    direct(DirectScope::Stage, "startcountdown", &[1, 2]),
    direct(DirectScope::Stage, "swapindefaultcar", &[0]),
    direct(DirectScope::Stage, "useelapsedtime", &[0]),
];

const fn direct(
    scope: DirectScope,
    command: &'static str,
    argument_counts: &'static [usize],
) -> DirectCommandSchema {
    DirectCommandSchema {
        scope,
        command,
        argument_counts,
    }
}

/// One exact normalized command retained at mission or stage scope.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionScopeCommand {
    source_ordinal: usize,
    name: String,
    args_raw: String,
    semantic_role: String,
    arguments: Vec<String>,
}

impl MissionScopeCommand {
    /// Return the source statement ordinal.
    #[must_use]
    pub const fn source_ordinal(&self) -> usize {
        self.source_ordinal
    }

    /// Return the normalized lowercase command identity.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the exact normalized argument payload inside the call.
    #[must_use]
    pub fn args_raw(&self) -> &str {
        &self.args_raw
    }

    /// Return the extraction-level semantic role classification.
    #[must_use]
    pub fn semantic_role(&self) -> &str {
        &self.semantic_role
    }

    /// Return normalized argument values in source order.
    #[must_use]
    pub fn arguments(&self) -> &[String] {
        &self.arguments
    }
}

/// Source scope that owns one reviewed condition.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MissionConditionScope {
    /// Condition was declared directly in the owning stage.
    Stage,
    /// Condition was declared while the stage root objective was open.
    Objective,
}

/// One reviewed condition attached to its exact source scope.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionScopeCondition {
    binding: MissionConditionBinding,
    parameters: MissionConditionParameterBinding,
    scope: MissionConditionScope,
    owner_objective_source_ordinal: Option<usize>,
    commands: Vec<MissionConditionCommandBinding>,
}

impl MissionScopeCondition {
    /// Return the reviewed condition binding.
    #[must_use]
    pub const fn binding(&self) -> &MissionConditionBinding {
        &self.binding
    }

    /// Return typed parameters carried directly by `AddCondition`.
    #[must_use]
    pub const fn parameters(&self) -> &MissionConditionParameterBinding {
        &self.parameters
    }

    /// Return the exact source scope where the condition was declared.
    #[must_use]
    pub const fn scope(&self) -> MissionConditionScope {
        self.scope
    }

    /// Return the root `AddObjective` ordinal for objective-scoped conditions.
    #[must_use]
    pub const fn owner_objective_source_ordinal(&self) -> Option<usize> {
        self.owner_objective_source_ordinal
    }

    /// Return reviewed commands owned by this condition.
    #[must_use]
    pub fn commands(&self) -> &[MissionConditionCommandBinding] {
        &self.commands
    }
}

/// One reviewed root objective attached to a stage.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionScopeObjective {
    binding: MissionObjectiveBinding,
    parameters: MissionObjectiveParameterBinding,
    commands: Vec<MissionObjectiveCommandBinding>,
}

impl MissionScopeObjective {
    /// Return the reviewed objective binding.
    #[must_use]
    pub const fn binding(&self) -> &MissionObjectiveBinding {
        &self.binding
    }

    /// Return typed parameters carried directly by `AddObjective`.
    #[must_use]
    pub const fn parameters(&self) -> &MissionObjectiveParameterBinding {
        &self.parameters
    }

    /// Return reviewed commands owned by this objective.
    #[must_use]
    pub fn commands(&self) -> &[MissionObjectiveCommandBinding] {
        &self.commands
    }
}

/// One source stage with exactly one root objective.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionScopeStage {
    source_ordinal: usize,
    sequence_ordinal: usize,
    legacy_parameters: Vec<String>,
    commands: Vec<MissionScopeCommand>,
    objective: MissionScopeObjective,
    conditions: Vec<MissionScopeCondition>,
}

impl MissionScopeStage {
    /// Return the source statement ordinal that opened this scope.
    #[must_use]
    pub const fn source_ordinal(&self) -> usize {
        self.source_ordinal
    }

    /// Return dense stage declaration order without interpreting source args.
    #[must_use]
    pub const fn sequence_ordinal(&self) -> usize {
        self.sequence_ordinal
    }

    /// Return exact normalized `AddStage` arguments for later compilation.
    #[must_use]
    pub fn legacy_parameters(&self) -> &[String] {
        &self.legacy_parameters
    }

    /// Return direct stage-scope commands in source order.
    #[must_use]
    pub fn commands(&self) -> &[MissionScopeCommand] {
        &self.commands
    }

    /// Return the unique root objective declared by this stage.
    #[must_use]
    pub const fn objective(&self) -> &MissionScopeObjective {
        &self.objective
    }

    /// Return conditions declared by this stage in source order.
    #[must_use]
    pub fn conditions(&self) -> &[MissionScopeCondition] {
        &self.conditions
    }
}

/// One source mission block projected without interpreting gameplay parameters.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionScopeGraph {
    source_ordinal: usize,
    source_mission_id: String,
    commands: Vec<MissionScopeCommand>,
    stages: Vec<MissionScopeStage>,
}

impl MissionScopeGraph {
    /// Return the source statement ordinal that opened this mission.
    #[must_use]
    pub const fn source_ordinal(&self) -> usize {
        self.source_ordinal
    }

    /// Return the exact normalized source mission identity.
    #[must_use]
    pub fn source_mission_id(&self) -> &str {
        &self.source_mission_id
    }

    /// Return direct mission-scope commands in source order.
    #[must_use]
    pub fn commands(&self) -> &[MissionScopeCommand] {
        &self.commands
    }

    /// Return source stages in dense declaration order.
    #[must_use]
    pub fn stages(&self) -> &[MissionScopeStage] {
        &self.stages
    }

    /// Return whether every root objective already has a canonical mapping.
    #[must_use]
    pub fn has_only_mapped_objectives(&self) -> bool {
        self.stages
            .iter()
            .all(|stage| stage.objective.binding.is_mapped())
    }
}

/// Complete scope projection for one normalized mission-script document.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionScopeReport {
    unscoped_commands: Vec<MissionScopeCommand>,
    missions: Vec<MissionScopeGraph>,
}

impl MissionScopeReport {
    /// Return commands outside any selected mission in source order.
    #[must_use]
    pub fn unscoped_commands(&self) -> &[MissionScopeCommand] {
        &self.unscoped_commands
    }

    /// Return projected mission blocks in source order.
    #[must_use]
    pub fn missions(&self) -> &[MissionScopeGraph] {
        &self.missions
    }

    /// Return whether every projected mission has mapped root objectives.
    #[must_use]
    pub fn has_only_mapped_objectives(&self) -> bool {
        self.missions
            .iter()
            .all(MissionScopeGraph::has_only_mapped_objectives)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct StageBuilder {
    source_ordinal: usize,
    sequence_ordinal: usize,
    legacy_parameters: Vec<String>,
    commands: Vec<MissionScopeCommand>,
    objective: Option<MissionScopeObjective>,
    conditions: Vec<MissionScopeCondition>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct MissionBuilder {
    source_ordinal: usize,
    source_mission_id: String,
    commands: Vec<MissionScopeCommand>,
    stages: Vec<StageBuilder>,
}

/// Project reviewed mission-script evidence into exact source scopes.
///
/// Non-mission utility scripts and inert zero-byte placeholders produce an
/// empty report. The projection never interprets legacy parameter meaning.
///
/// # Errors
///
/// Returns an error when reviewed evidence cannot be consumed exactly or a
/// stage does not own exactly one root objective.
pub fn compile_mission_scope_graphs(
    evidence: &MissionScriptEvidence,
) -> Result<MissionScopeReport, String> {
    let objectives = preflight_mission_objectives(evidence)?;
    let objective_parameters =
        preflight_mission_objective_parameters(evidence)?;
    let objective_commands = preflight_mission_objective_commands(evidence)?;
    let conditions = preflight_mission_conditions(evidence)?;
    let condition_parameters =
        preflight_mission_condition_parameters(evidence)?;
    let condition_commands = preflight_mission_condition_commands(evidence)?;

    let mut objective_by_ordinal = collect_objectives(objectives.objectives())?;
    let mut objective_parameter_by_ordinal =
        collect_objective_parameters(objective_parameters.objectives())?;
    let mut objective_command_by_ordinal =
        collect_objective_commands(objective_commands.commands())?;
    let mut condition_by_ordinal = collect_conditions(conditions.conditions())?;
    let mut condition_parameter_by_ordinal =
        collect_condition_parameters(condition_parameters.conditions())?;
    let mut condition_command_by_ordinal =
        collect_condition_commands(condition_commands.commands())?;

    let mut unscoped_commands = Vec::<MissionScopeCommand>::new();
    let mut missions = Vec::<MissionBuilder>::new();
    let mut current_mission = None::<usize>;
    let mut current_stage = None::<usize>;
    let mut objective_open = false;
    let mut current_condition = None::<usize>;

    for invocation in evidence.invocations() {
        if closes_condition_before_invocation(evidence, invocation.ordinal()) {
            current_condition = None;
        }
        let objective_command =
            objective_command_by_ordinal.remove(&invocation.ordinal());
        let condition_command =
            condition_command_by_ordinal.remove(&invocation.ordinal());
        if !is_context_command(invocation.name())
            && objective_command.is_none()
            && condition_command.is_none()
        {
            attach_direct_command(
                &mut unscoped_commands,
                &mut missions,
                current_mission,
                current_stage,
                objective_open,
                current_condition,
                invocation,
            )?;
        }
        attach_scoped_commands(
            &mut missions,
            current_mission,
            current_stage,
            current_condition,
            objective_command,
            condition_command,
        )?;

        match invocation.name() {
            "selectmission" => {
                if current_mission.is_some() {
                    return Err("mission scope projection reopened a mission"
                        .to_owned());
                }
                let source_mission_id = invocation
                    .arguments()
                    .first()
                    .ok_or_else(|| {
                        "mission scope projection lost mission identity"
                            .to_owned()
                    })?
                    .clone();
                if source_mission_id.is_empty() {
                    return Err(
                                                // jig-ignore-next-line: literal
                                                "mission scope projection found an empty mission identity"
                            .to_owned(),
                    );
                }
                missions.push(MissionBuilder {
                    source_ordinal: invocation.ordinal(),
                    source_mission_id,
                    commands: Vec::new(),
                    stages: Vec::new(),
                });
                current_mission = Some(missions.len().saturating_sub(1));
            },
            "closemission" => {
                if current_stage.is_some()
                    || objective_open
                    || current_condition.is_some()
                {
                    return Err(
                        "mission scope projection closed an active child scope"
                            .to_owned(),
                    );
                }
                current_mission = None;
            },
            "addstage" => {
                let mission_index = current_mission.ok_or_else(|| {
                    "mission scope projection opened a stage outside a mission"
                        .to_owned()
                })?;
                let mission =
                    missions.get_mut(mission_index).ok_or_else(|| {
                        "mission scope projection lost the active mission"
                            .to_owned()
                    })?;
                let sequence_ordinal = mission.stages.len();
                mission.stages.push(StageBuilder {
                    source_ordinal: invocation.ordinal(),
                    sequence_ordinal,
                    legacy_parameters: invocation.arguments().to_vec(),
                    commands: Vec::new(),
                    objective: None,
                    conditions: Vec::new(),
                });
                current_stage = Some(sequence_ordinal);
            },
            "closestage" => {
                if objective_open || current_condition.is_some() {
                    return Err(
                        "mission scope projection closed an active stage child"
                            .to_owned(),
                    );
                }
                let stage = active_stage(
                    &mut missions,
                    current_mission,
                    current_stage,
                )?;
                if stage.objective.is_none() {
                    return Err(
                        "mission stage requires exactly one root objective"
                            .to_owned(),
                    );
                }
                current_stage = None;
            },
            "addobjective" => {
                let binding = objective_by_ordinal
                    .remove(&invocation.ordinal())
                    .ok_or_else(|| {
                        "mission scope projection lost objective evidence"
                            .to_owned()
                    })?;
                let parameters = objective_parameter_by_ordinal
                    .remove(&invocation.ordinal())
                    .ok_or_else(|| {
                        "mission scope projection lost objective parameters"
                            .to_owned()
                    })?;
                if parameters.source_alias() != binding.source_alias() {
                    return Err(
                        "mission objective parameter owner drifted".to_owned()
                    );
                }
                let stage = active_stage(
                    &mut missions,
                    current_mission,
                    current_stage,
                )?;
                if stage.objective.is_some() {
                    return Err(
                        "mission stage requires exactly one root objective"
                            .to_owned(),
                    );
                }
                stage.objective = Some(MissionScopeObjective {
                    binding,
                    parameters,
                    commands: Vec::new(),
                });
                objective_open = true;
            },
            "closeobjective" => {
                objective_open = false;
            },
            "addcondition" => {
                let binding = condition_by_ordinal
                    .remove(&invocation.ordinal())
                    .ok_or_else(|| {
                        "mission scope projection lost condition evidence"
                            .to_owned()
                    })?;
                let parameters = condition_parameter_by_ordinal
                    .remove(&invocation.ordinal())
                    .ok_or_else(|| {
                        "mission scope projection lost condition parameters"
                            .to_owned()
                    })?;
                if parameters.source_alias() != binding.source_alias() {
                    return Err(
                        "mission condition parameter owner drifted".to_owned()
                    );
                }
                let scope = if objective_open {
                    MissionConditionScope::Objective
                } else {
                    MissionConditionScope::Stage
                };
                let stage = active_stage(
                    &mut missions,
                    current_mission,
                    current_stage,
                )?;
                let owner_objective_source_ordinal = match scope {
                    MissionConditionScope::Stage => None,
                    MissionConditionScope::Objective => Some(
                        stage
                            .objective
                            .as_ref()
                            .ok_or_else(|| {
                                "objective-scoped condition lost root objective"
                                    .to_owned()
                            })?
                            .binding()
                            .ordinal(),
                    ),
                };
                stage.conditions.push(MissionScopeCondition {
                    binding,
                    parameters,
                    scope,
                    owner_objective_source_ordinal,
                    commands: Vec::new(),
                });
                current_condition =
                    Some(stage.conditions.len().saturating_sub(1));
            },
            "closecondition" => {
                if current_condition.is_none()
                    && !ignores_orphan_condition_close(
                        evidence,
                        invocation.ordinal(),
                    )
                {
                    return Err(
                        "mission scope projection closed a missing condition"
                            .to_owned(),
                    );
                }
                current_condition = None;
            },
            _ => {},
        }
    }

    if current_mission.is_some()
        || current_stage.is_some()
        || objective_open
        || current_condition.is_some()
    {
        return Err(
            "mission scope projection ended with an open context".to_owned()
        );
    }
    if !objective_by_ordinal.is_empty()
        || !objective_parameter_by_ordinal.is_empty()
        || !objective_command_by_ordinal.is_empty()
        || !condition_by_ordinal.is_empty()
        || !condition_parameter_by_ordinal.is_empty()
        || !condition_command_by_ordinal.is_empty()
    {
        return Err(
            "mission scope projection left reviewed evidence unconsumed"
                .to_owned(),
        );
    }

    let missions = missions
        .into_iter()
        .map(finalize_mission)
        .collect::<Result<Vec<_>, _>>()?;
    Ok(MissionScopeReport {
        unscoped_commands,
        missions,
    })
}

fn is_context_command(name: &str) -> bool {
    matches!(
        name,
        "selectmission"
            | "closemission"
            | "addstage"
            | "closestage"
            | "addobjective"
            | "closeobjective"
            | "addcondition"
            | "closecondition"
    )
}

fn command_from_invocation(
    invocation: &MissionCommandInvocation,
) -> MissionScopeCommand {
    MissionScopeCommand {
        source_ordinal: invocation.ordinal(),
        name: invocation.name().to_owned(),
        args_raw: invocation.args_raw().to_owned(),
        semantic_role: invocation.semantic_role().to_owned(),
        arguments: invocation.arguments().to_vec(),
    }
}

fn validate_direct_command(
    scope: DirectScope,
    invocation: &MissionCommandInvocation,
) -> Result<(), String> {
    let Some(schema) = DIRECT_COMMANDS.iter().find(|schema| {
        schema.scope == scope && schema.command == invocation.name()
    }) else {
        return Err(
            "mission direct command is not registered for its scope".to_owned()
        );
    };
    if !schema
        .argument_counts
        .contains(&invocation.arguments().len())
    {
        return Err("mission direct command arity is not registered".to_owned());
    }
    Ok(())
}

fn attach_direct_command(
    unscoped_commands: &mut Vec<MissionScopeCommand>,
    missions: &mut [MissionBuilder],
    current_mission: Option<usize>,
    current_stage: Option<usize>,
    objective_open: bool,
    current_condition: Option<usize>,
    invocation: &MissionCommandInvocation,
) -> Result<(), String> {
    if objective_open || current_condition.is_some() {
        return Err(
            "mission nested command escaped reviewed scoped registries"
                .to_owned(),
        );
    }
    let command = command_from_invocation(invocation);
    if current_stage.is_some() {
        validate_direct_command(DirectScope::Stage, invocation)?;
        active_stage(missions, current_mission, current_stage)?
            .commands
            .push(command);
        return Ok(());
    }
    if let Some(mission_index) = current_mission {
        validate_direct_command(DirectScope::Mission, invocation)?;
        let mission = missions.get_mut(mission_index).ok_or_else(|| {
            "mission scope projection lost the active mission".to_owned()
        })?;
        mission.commands.push(command);
        return Ok(());
    }
    if DIRECT_COMMANDS
        .iter()
        .any(|schema| schema.command == invocation.name())
    {
        validate_direct_command(DirectScope::Unscoped, invocation)?;
    }
    unscoped_commands.push(command);
    Ok(())
}

fn active_stage(
    missions: &mut [MissionBuilder],
    current_mission: Option<usize>,
    current_stage: Option<usize>,
) -> Result<&mut StageBuilder, String> {
    let mission_index = current_mission.ok_or_else(|| {
        "mission scope projection has no active mission".to_owned()
    })?;
    let stage_index = current_stage.ok_or_else(|| {
        "mission scope projection has no active stage".to_owned()
    })?;
    missions
        .get_mut(mission_index)
        .and_then(|mission| mission.stages.get_mut(stage_index))
        .ok_or_else(|| {
            "mission scope projection lost the active stage".to_owned()
        })
}

fn attach_scoped_commands(
    missions: &mut [MissionBuilder],
    current_mission: Option<usize>,
    current_stage: Option<usize>,
    current_condition: Option<usize>,
    objective_command: Option<MissionObjectiveCommandBinding>,
    condition_command: Option<MissionConditionCommandBinding>,
) -> Result<(), String> {
    if let Some(command) = condition_command {
        let stage = active_stage(missions, current_mission, current_stage)?;
        let condition_index = current_condition.ok_or_else(|| {
            "mission condition command escaped its condition scope".to_owned()
        })?;
        let condition =
            stage.conditions.get_mut(condition_index).ok_or_else(|| {
                "mission scope projection lost the active condition".to_owned()
            })?;
        if condition.binding.source_alias() != command.condition_alias() {
            return Err("mission condition command owner drifted".to_owned());
        }
        condition.commands.push(command);
        return Ok(());
    }
    if let Some(command) = objective_command {
        let stage = active_stage(missions, current_mission, current_stage)?;
        let objective = stage.objective.as_mut().ok_or_else(|| {
            "mission objective command escaped its objective scope".to_owned()
        })?;
        if objective.binding.source_alias() != command.objective_alias() {
            return Err("mission objective command owner drifted".to_owned());
        }
        objective.commands.push(command);
    }
    Ok(())
}

fn finalize_mission(
    builder: MissionBuilder,
) -> Result<MissionScopeGraph, String> {
    let stages = builder
        .stages
        .into_iter()
        .map(|stage| {
            let objective = stage.objective.ok_or_else(|| {
                "mission stage requires exactly one root objective".to_owned()
            })?;
            Ok(MissionScopeStage {
                source_ordinal: stage.source_ordinal,
                sequence_ordinal: stage.sequence_ordinal,
                legacy_parameters: stage.legacy_parameters,
                commands: stage.commands,
                objective,
                conditions: stage.conditions,
            })
        })
        .collect::<Result<Vec<_>, String>>()?;
    if stages.is_empty() {
        return Err("mission scope projection found a mission without stages"
            .to_owned());
    }
    Ok(MissionScopeGraph {
        source_ordinal: builder.source_ordinal,
        source_mission_id: builder.source_mission_id,
        commands: builder.commands,
        stages,
    })
}

fn collect_objectives(
    bindings: &[MissionObjectiveBinding],
) -> Result<BTreeMap<usize, MissionObjectiveBinding>, String> {
    collect_by_ordinal(bindings, MissionObjectiveBinding::ordinal, "objective")
}

fn collect_objective_parameters(
    bindings: &[MissionObjectiveParameterBinding],
) -> Result<BTreeMap<usize, MissionObjectiveParameterBinding>, String> {
    collect_by_ordinal(
        bindings,
        MissionObjectiveParameterBinding::ordinal,
        "objective parameter",
    )
}

fn collect_condition_parameters(
    bindings: &[MissionConditionParameterBinding],
) -> Result<BTreeMap<usize, MissionConditionParameterBinding>, String> {
    collect_by_ordinal(
        bindings,
        MissionConditionParameterBinding::ordinal,
        "condition parameter",
    )
}

fn collect_conditions(
    bindings: &[MissionConditionBinding],
) -> Result<BTreeMap<usize, MissionConditionBinding>, String> {
    collect_by_ordinal(bindings, MissionConditionBinding::ordinal, "condition")
}

fn collect_objective_commands(
    bindings: &[MissionObjectiveCommandBinding],
) -> Result<BTreeMap<usize, MissionObjectiveCommandBinding>, String> {
    collect_by_ordinal(
        bindings,
        MissionObjectiveCommandBinding::ordinal,
        "objective command",
    )
}

fn collect_condition_commands(
    bindings: &[MissionConditionCommandBinding],
) -> Result<BTreeMap<usize, MissionConditionCommandBinding>, String> {
    collect_by_ordinal(
        bindings,
        MissionConditionCommandBinding::ordinal,
        "condition command",
    )
}

fn collect_by_ordinal<T: Clone>(
    bindings: &[T],
    ordinal: impl Fn(&T) -> usize,
    label: &str,
) -> Result<BTreeMap<usize, T>, String> {
    let mut indexed = BTreeMap::new();
    for binding in bindings {
        if indexed.insert(ordinal(binding), binding.clone()).is_some() {
            return Err(format!(
                "mission scope projection duplicated {label} ordinal"
            ));
        }
    }
    Ok(indexed)
}

fn has_adaptation(
    evidence: &MissionScriptEvidence,
    ordinal: usize,
    code: &str,
) -> bool {
    evidence.adaptations().iter().any(|adaptation| {
        adaptation.ordinal() == ordinal && adaptation.code() == code
    })
}

fn ignores_orphan_condition_close(
    evidence: &MissionScriptEvidence,
    ordinal: usize,
) -> bool {
    has_adaptation(evidence, ordinal, L2_ORPHAN_CONDITION_CLOSE)
}

fn closes_condition_before_invocation(
    evidence: &MissionScriptEvidence,
    ordinal: usize,
) -> bool {
    has_adaptation(evidence, ordinal, L7_KEEPBARREL_CLOSE)
}

#[cfg(test)]
// jig-ignore-next-line: exact test module path is indivisible
#[path = "../../../../../tests/migration/pipeline/unit/domain/package/mission_scope/tests.rs"]
mod tests;
