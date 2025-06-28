use serde::Deserialize;
use serde_repr::Deserialize_repr;

#[derive(Deserialize_repr, PartialEq, Debug, Copy, Clone)]
#[repr(i32)]
pub enum GachaViewTypeInfoId {
    NoviceConvene = 1,
    FeaturedResonatorConvene = 2,
    FeaturedWeaponConvene = 3,
    StandardResonatorConvene = 4,
    StandardWeaponConvene = 5,
    BeginnersChoiceConvene = 6,
    AnniversaryResonatorConvene = 7,
    AnniversaryWeaponConvene = 8,
    NewVoyageResonatorConvene = 9,
    NewVoyageWeaponConvene = 10,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct GachaViewInfoData {
    pub id: i32,
    pub r#type: GachaViewTypeInfoId,
    pub summary_title: String,
    pub summary_describe: String,
    pub theme_color: String,
    #[cfg(feature = "strict_json_fields")]
    pub effect_location: RawVectorData,
    #[cfg(feature = "strict_json_fields")]
    pub effect_path: String,
    #[cfg(feature = "strict_json_fields")]
    pub text_texture: String,
    #[cfg(feature = "strict_json_fields")]
    pub content_texture_path: String,
    #[cfg(feature = "strict_json_fields")]
    pub content_texture_bg_path: String,
    #[cfg(feature = "strict_json_fields")]
    pub under_bg_texture_path: String,
    #[cfg(feature = "strict_json_fields")]
    pub tag_not_selected_sprite_path: String,
    #[cfg(feature = "strict_json_fields")]
    pub tag_selected_sprite_path: String,
    #[cfg(feature = "strict_json_fields")]
    pub weapon_prefab_path: String,
    #[cfg(feature = "strict_json_fields")]
    pub spine_prefab_resource: String,
    pub up_list: Vec<i32>,
    pub show_id_list: Vec<i32>,
    pub preview_id_list: Vec<i32>,
}