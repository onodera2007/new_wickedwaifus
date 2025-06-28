use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct ModelIdType {
    pub model_id: i32
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct PrefabStateList {
    #[cfg(feature = "strict_json_fields")]
    pub scene_interaction_state: i32,
    #[cfg(feature = "strict_json_fields")]
    pub level_tag: i64,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct LevelPrefabType {
    #[cfg(feature = "strict_json_fields")]
    pub blueprint_path: String,
    #[cfg(feature = "strict_json_fields")]
    pub prefab_path: String,
    #[cfg(feature = "strict_json_fields")]
    pub prefab_state_list: Vec<PrefabStateList>,
    #[cfg(feature = "strict_json_fields")]
    pub effect_state_list: Vec<String>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct MeshType {
    #[cfg(feature = "strict_json_fields")]
    pub mesh: String,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct DaType {
    #[cfg(feature = "strict_json_fields")]
    pub da: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "Type")]
pub enum NpcModel {
    Mesh(MeshType),
    Da(DaType),
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct NpcType {
    #[cfg(feature = "strict_json_fields")]
    pub blueprint_path: String,
    #[cfg(feature = "strict_json_fields")]
    pub npc_model: NpcModel,
    #[cfg(feature = "strict_json_fields")]
    pub abp: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "Type")]
pub enum ModelType {
    ModelId(ModelIdType),
    LevelPrefab(LevelPrefabType),
    Npc(NpcType),
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct ModelComponent {
    pub half_height: Option<i32>,
    pub track_height: Option<i32>,
    pub disabled: Option<bool>,
    pub model_type: Option<ModelType>,
}