use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum LevelPlayType {
    Challenge,
    SilentArea,
    Dungeon,
    MonsterTreasure,
    Quest,
    Riddle,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct LevelPlayOpenCondition {
    pub conditions: Option<Vec<Condition>>,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct LevelPlayActive {
    pub active_type: i32,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct LevelPlayRewardConfigResetTypeMidNight {
    pub count: i32,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "Type")]
pub enum LevelPlayRewardConfigResetType {
    MidNight(LevelPlayRewardConfigResetTypeMidNight),
}

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct LevelPlayRewardConfigInteract {
    pub reward_id: i32,
    pub reward_entity_id: i64,
    pub reward_complete_actions: Vec<Action>,
    pub first_complete_actions: Option<Vec<Action>>,
    pub reset: Option<LevelPlayRewardConfigResetType>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "Type")]
pub enum LevelPlayRewardConfig {
    None,
    Interact(LevelPlayRewardConfigInteract),
}

#[derive(Debug, Deserialize)]
pub enum FixedDateTime {
    Daily,
    Weekly
}

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct LevelPlayRefreshConfigFixedDateTime {
    pub update_type: FixedDateTime,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct LevelPlayRefreshConfigCompleted {
    pub min_refresh_cd: i32,
    pub max_refresh_cd: i32,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "Type")]
pub enum LevelPlayRefreshConfig {
    None,
    FixedDateTime(LevelPlayRefreshConfigFixedDateTime),
    Completed(LevelPlayRefreshConfigCompleted),
}

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct LevelPlayTrack {
    pub track_radius: i32,
    pub track_priority: i32,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct LevelPlayMark {
    pub mark_id: i32,
    pub map_bg_path: String,
}

#[derive(Debug, Deserialize)]
pub enum OnlineType {
    Multiplayer,
    Local,
    Hang,
}

#[derive(Debug, Deserialize)]
pub enum ObjType {
    LevelPlay,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct LevelPlayDataDetail { // Json file contains Data in name, so it has to be DataData
    pub id: i32,
    pub key: String,
    #[cfg(feature = "strict_json_fields")]
    pub internal_dest: String,
    pub level_id: i32,
    #[cfg(feature = "strict_json_fields")]
    pub tid_name: String,
    pub r#type: LevelPlayType,
    pub instance_id: i32,
    pub level_play_entity_id: i64,
    pub level_additive_id: i32,
    pub enter_radius: i32,
    pub leave_radius: i32,
    pub delay_refresh: bool,
    pub delay_destroy: bool,
    pub level_play_open_condition: LevelPlayOpenCondition,
    pub level_play_active: LevelPlayActive,
    pub level_play_reward_config: LevelPlayRewardConfig,
    pub level_play_refresh_config: LevelPlayRefreshConfig,
    pub level_play_track: LevelPlayTrack,
    pub level_play_mark: Option<LevelPlayMark>,
    pub enter_in_range_actions: Option<Vec<Action>>,
    pub pack_id: i32,
    pub online_type: OnlineType,
    pub obj_type: ObjType,
    #[cfg(feature = "strict_json_fields")]
    pub children: Option<Vec<String>>,
    #[cfg(feature = "strict_json_fields")]
    pub reference: Vec<String>,
    #[cfg(feature = "strict_json_fields")]
    pub weak_reference: Option<Vec<String>>,
    pub exploratory_degree: Option<i32>,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct LevelPlayDataData { // Json file contains Data in name, so it has to be DataData
    pub level_play_id: i32,
    pub data: LevelPlayDataDetail,
}

