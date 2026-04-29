mod config;
mod error;
mod routes;

use std::sync::Arc;

use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use jsonwebtoken::EncodingKey;
use octocrab::{
    Octocrab,
    models::{AppId, InstallationId},
};
use reqwest::Client;
use tokio::net::TcpListener;

use crate::{
    config::{GitHubConfig, ProviderConfig},
    error::{NekonoteError, Result},
};

struct AppState {
    pub providers: ProviderState,
    pub client: Client,
}

impl AppState {
    fn new(config: &config::Config) -> Result<AppState> {
        let providers = ProviderState::new(&config.provider)?;
        let client = Client::builder()
            .user_agent(concat!("nekonote/", env!("CARGO_PKG_VERSION")))
            .build()
            .map_err(NekonoteError::ReqwestClientBuildError)?;

        Ok(AppState { providers, client })
    }
}

#[derive(Clone)]
struct ProviderState {
    pub github: Option<GitHubState>,
}

impl ProviderState {
    fn new(config: &ProviderConfig) -> Result<ProviderState> {
        Ok(ProviderState {
            github: config.github.as_ref().map(GitHubState::new).transpose()?,
        })
    }
}

#[derive(Clone)]
struct GitHubState {
    octocrab: Octocrab,
    mcp_endpoint: String,
}

impl GitHubState {
    fn new(config: &GitHubConfig) -> Result<GitHubState> {
        let app_key = BASE64
            .decode(&config.app_key)
            .map_err(NekonoteError::DecodeGitHubAppKeyError)?;
        let app_key =
            EncodingKey::from_rsa_pem(&app_key).map_err(NekonoteError::EncodeGitHubAppKeyError)?;

        let app_octocrab = Octocrab::builder()
            .app(AppId(config.app_id), app_key)
            .build()
            .map_err(NekonoteError::OctocrabBuildError)?;

        let octocrab = app_octocrab
            .installation(InstallationId(config.installation_id))
            .map_err(NekonoteError::OctocrabBuildError)?;

        Ok(GitHubState {
            octocrab,
            mcp_endpoint: config.mcp_endpoint.clone(),
        })
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = config::Config::load()?;
    let state = Arc::new(AppState::new(&config)?);

    let app = routes::routes().with_state(state);
    let listener = TcpListener::bind(config.server.addr).await?;

    println!("Server running on http://{}", config.server.addr);
    axum::serve(listener, app).await?;

    Ok(())
}
