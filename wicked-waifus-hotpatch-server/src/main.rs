use std::fs;
use std::sync::LazyLock;

use anyhow::Result;

use wicked_waifus_commons::config_util::{self, TomlConfig};
use serde::Deserialize;
use wicked_waifus_http::{
    config::{AesSettings, NetworkSettings},
    Application, Path,
};

#[derive(Deserialize)]
pub struct ServerConfig {
    pub network: NetworkSettings,
    pub encryption: AesSettings,
}

impl TomlConfig for ServerConfig {
    const DEFAULT_TOML: &str = include_str!("../hotpatch.default.toml");
}

#[tokio::main]
async fn main() -> Result<()> {
    static CONFIG: LazyLock<ServerConfig> =
        LazyLock::new(|| config_util::load_or_create("hotpatch.toml"));

    ::wicked_waifus_commons::splash::print_splash();
    ::wicked_waifus_commons::logging::init_axum(::tracing::Level::DEBUG);

    Application::new()
        .get("/{env}/client/{hash}/{platform}/config.json", get_config)
        .with_encryption(&CONFIG.encryption)
        .with_logger()
        .serve(&CONFIG.network)
        .await?;

    Ok(())
}

#[tracing::instrument]
async fn get_config(Path((env, _hash, platform)): Path<(String, String, String)>) -> String {
    tracing::debug!("hotpatch config requested");

    let local_path = format!("assets/hotpatch/{env}/{platform}/config.json");
    fs::read_to_string(&local_path).unwrap_or_else(|_| {
        tracing::warn!("requested config was not found");
        String::from("{}")
    })
}
