use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use jsonwebtoken::EncodingKey;
use octocrab::{
    Octocrab,
    models::{AppId, InstallationId},
};

use crate::{
    config::GitHubConfig,
    error::{NekonoteError, Result},
};

#[derive(Clone)]
pub(crate) struct GitHubState {
    pub octocrab: Octocrab,
    pub mcp_endpoint: String,
}

impl GitHubState {
    pub(crate) fn new(config: &GitHubConfig) -> Result<GitHubState> {
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
