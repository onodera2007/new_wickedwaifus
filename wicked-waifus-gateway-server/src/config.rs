use wicked_waifus_commons::config_util::TomlConfig;
use serde::Deserialize;
use wicked_waifus_database::DatabaseSettings;
use wicked_waifus_network::config::ServiceEndPoint;
use wicked_waifus_protokey::ProtoKeySettings;

#[derive(Deserialize)]
pub struct ServerConfig {
    pub service_id: u32,
    pub network: NetworkSettings,
    pub database: DatabaseSettings,
    pub protokey: ProtoKeySettings,
    pub service_end_point: ServiceEndPoint,
    pub game_server_end_point: ServiceEndPoint,
}

#[derive(Deserialize)]
pub struct NetworkSettings {
    pub kcp_port: u16,
    pub kcp_crc: Option<bool>
}

impl TomlConfig for ServerConfig {
    const DEFAULT_TOML: &str = include_str!("../gateway.default.toml");
}
