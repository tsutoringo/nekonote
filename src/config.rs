use std::net::SocketAddr;

use serde::Deserialize;

use crate::error::NekonoteError;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub provider: ProviderConfig,
}

impl Config {
    pub fn load() -> Result<Self, NekonoteError> {
        let config = config::Config::builder()
            .add_source(config::File::with_name("config").required(false))
            .add_source(config::Environment::with_prefix("NEKONOTE").separator("__"))
            .set_default(
                "provider.github.mcp_endpoint",
                "https://api.githubcopilot.com/mcp/",
            )?
            .build()?;

        config.try_deserialize().map_err(NekonoteError::from)
    }
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub addr: SocketAddr,
}

#[derive(Debug, Deserialize)]
pub struct ProviderConfig {
    pub github: Option<GitHubConfig>,
}

#[derive(Debug, Deserialize)]
pub struct GitHubConfig {
    pub app_id: u64,
    pub app_key: String,
    pub installation_id: u64,

    pub mcp_endpoint: String,
}
