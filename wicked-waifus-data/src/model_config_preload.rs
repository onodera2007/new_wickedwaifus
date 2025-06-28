use serde::Deserialize;

#[derive(Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct ModelConfigPreloadData {
    pub id: i32,
    pub actor_class_path: String,
    #[cfg(feature = "strict_json_fields")]
    pub actor_class: Vec<String>,
    #[cfg(feature = "strict_json_fields")]
    pub animations: Vec<String>,
    #[cfg(feature = "strict_json_fields")]
    pub effects: Vec<String>,
    #[cfg(feature = "strict_json_fields")]
    pub audios: Vec<String>,
    #[cfg(feature = "strict_json_fields")]
    pub meshes: Vec<String>,
    #[cfg(feature = "strict_json_fields")]
    pub materials: Vec<String>,
    #[cfg(feature = "strict_json_fields")]
    pub animation_blueprints: Vec<String>,
    #[cfg(feature = "strict_json_fields")]
    pub others: Vec<String>,
}