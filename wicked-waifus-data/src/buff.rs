use serde::Deserialize;

#[derive(Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct BuffData {
    pub id: i64,
    pub ge_desc: String,
    pub duration_policy: i32, // TODO: Enum ??
    #[cfg(feature = "strict_json_fields")]
    pub duration_calculation_policy: Vec<i32>,
    #[cfg(feature = "strict_json_fields")]
    pub duration_magnitude: Vec<i32>,
    #[cfg(feature = "strict_json_fields")]
    pub duration_magnitude2: Vec<i32>,
    #[cfg(feature = "strict_json_fields")]
    pub b_duration_affected_by_bullet_time: bool,
    #[cfg(feature = "strict_json_fields")]
    pub formation_policy: i32, // TODO: Enum ??
    #[cfg(feature = "strict_json_fields")]
    pub probability: i32,
    #[cfg(feature = "strict_json_fields")]
    pub period: i32,
    #[cfg(feature = "strict_json_fields")]
    pub b_execute_periodic_effect_on_application: bool,
    #[cfg(feature = "strict_json_fields")]
    pub periodic_inhibition_policy: i32, // TODO: Enum ??
    #[serde(rename = "GameAttributeID")]
    pub game_attribute_id: i32,
    #[cfg(feature = "strict_json_fields")]
    pub calculation_policy: Vec<i32>,
    #[cfg(feature = "strict_json_fields")]
    pub modifier_magnitude: Vec<i32>,
    #[cfg(feature = "strict_json_fields")]
    pub modifier_magnitude2: Vec<i32>,
    #[cfg(feature = "strict_json_fields")]
    pub stacking_type: i32, // TODO: Enum??
    #[cfg(feature = "strict_json_fields")]
    pub default_stack_count: i32,
    #[cfg(feature = "strict_json_fields")]
    pub stack_append_count: i32,
    #[cfg(feature = "strict_json_fields")]
    pub stack_limit_count: i32,
    #[cfg(feature = "strict_json_fields")]
    pub stack_duration_refresh_policy: i32,
    #[cfg(feature = "strict_json_fields")]
    pub stack_period_reset_policy: i32,
    #[cfg(feature = "strict_json_fields")]
    pub stack_expiration_remove_number: i32,
    #[cfg(feature = "strict_json_fields")]
    pub b_deny_overflow_application: bool,
    #[cfg(feature = "strict_json_fields")]
    pub b_clear_stack_on_overflow: bool,
    #[cfg(feature = "strict_json_fields")]
    pub b_require_modifier_success_to_trigger_cues: bool,
    #[cfg(feature = "strict_json_fields")]
    pub b_suppress_stacking_cues: bool,
    pub gameplay_cue_ids: Vec<i64>,
    #[cfg(feature = "strict_json_fields")]
    pub granted_tags: Vec<String>,
    #[cfg(feature = "strict_json_fields")]
    pub application_source_tag_requirements: Vec<String>,
    #[cfg(feature = "strict_json_fields")]
    pub application_source_tag_ignores: Vec<String>,
    #[cfg(feature = "strict_json_fields")]
    pub application_tag_requirements: Vec<String>,
    #[cfg(feature = "strict_json_fields")]
    pub application_tag_ignores: Vec<String>,
    pub ongoing_tag_requirements: Vec<String>,
    pub ongoing_tag_ignores: Vec<String>,
    pub removal_tag_requirements: Vec<String>,
    pub removal_tag_ignores: Vec<String>,
    #[cfg(feature = "strict_json_fields")]
    pub remove_buff_with_tags: Vec<String>,
    #[cfg(feature = "strict_json_fields")]
    pub granted_application_immunity_tags: Vec<String>,
    #[cfg(feature = "strict_json_fields")]
    pub granted_application_immunity_tag_ignores: Vec<String>,
    #[cfg(feature = "strict_json_fields")]
    pub premature_expiration_effects: Vec<String>,
    #[cfg(feature = "strict_json_fields")]
    pub routine_expiration_effects: Vec<String>,
    #[cfg(feature = "strict_json_fields")]
    pub overflow_effects: Vec<String>,
    #[cfg(feature = "strict_json_fields")]
    pub related_extra_effect_buff_id: Vec<String>,
    #[serde(rename = "ExtraEffectID")]
    pub extra_effect_id: i32,
    pub extra_effect_parameters: Vec<String>,
    pub extra_effect_parameters_grow1: Vec<i32>,
    pub extra_effect_parameters_grow2: Vec<i32>,
    #[cfg(feature = "strict_json_fields")]
    pub extra_effect_requirements: Vec<String>,
    #[cfg(feature = "strict_json_fields")]
    pub extra_effect_req_para: Vec<String>,
    #[cfg(feature = "strict_json_fields")]
    pub extra_effect_req_setting: i32, // TODO: Enum??
    #[cfg(feature = "strict_json_fields")]
    pub extra_effect_cd: Vec<i32>,
    #[cfg(feature = "strict_json_fields")]
    pub extra_effect_remove_stack_num: i32,
    #[cfg(feature = "strict_json_fields")]
    pub extra_effect_probability: Vec<i32>,
    #[cfg(feature = "strict_json_fields")]
    pub buff_action: Vec<String>,
    #[cfg(feature = "strict_json_fields")]
    pub tag_logic: Vec<String>,
    pub dead_remove: bool,
}