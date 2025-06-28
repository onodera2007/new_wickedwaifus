use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct StateMachineTransition {
    pub from: i32,
    pub to: i32,
    pub transition_prediction_type: i32,
    pub weight: i32,
    #[cfg(feature = "strict_json_fields")]
    pub conditions: Vec<serde_json::Value>, // TODO: Implement conditions
}

#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct StateMachineNodeCommon {
    pub uuid: i32,
    #[cfg(feature = "strict_json_fields")]
    pub is_anim_state_machine: Option<bool>,
    #[cfg(feature = "strict_json_fields")]
    pub is_conduit_node: Option<bool>,
    #[cfg(feature = "strict_json_fields")]
    pub is_any_state: Option<bool>,
    #[cfg(feature = "strict_json_fields")]
    pub name: String,
    #[cfg(feature = "strict_json_fields")]
    pub take_control_type: i32,
    #[cfg(feature = "strict_json_fields")]
    pub transition_rule: i32,
    pub children: Option<Vec<i32>>,
    pub transitions: Option<Vec<StateMachineTransition>>,
    #[cfg(feature = "strict_json_fields")]
    pub on_enter_actions: Option<Vec<serde_json::Value>>,  // TODO: Implement actions
    #[cfg(feature = "strict_json_fields")]
    pub on_exit_actions: Option<Vec<serde_json::Value>>,  // TODO: Implement actions
    #[cfg(feature = "strict_json_fields")]
    pub bind_states: Option<Vec<serde_json::Value>>,  // TODO: Implement bindStates
    #[cfg(feature = "strict_json_fields")]
    pub task: Option<serde_json::Value>,  // TODO: Implement bindStates
}

#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct StateMachineNodeReferenced {
    pub reference_uuid: i32,
    #[serde(flatten)]
    pub common: StateMachineNodeCommon,
}

#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct StateMachineNodeOverrideCommon {
    pub override_common_uuid: i32,
    #[serde(flatten)]
    pub common: StateMachineNodeCommon,
}

#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct StateMachineNodeCustom {
    #[serde(flatten)]
    pub common: StateMachineNodeCommon,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum StateMachineNode {
    Reference(StateMachineNodeReferenced),
    Override(StateMachineNodeOverrideCommon),
    Custom(StateMachineNodeCustom),
}

#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct StateMachineJson {
    pub version: u32,
    pub state_machines: Vec<i32>,
    pub nodes: Vec<StateMachineNode>,
}

#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct AiStateMachineConfigData {
    pub id: String,
    pub state_machine_json: StateMachineJson,
}