pub type Result<T> = std::result::Result<T, NekonoteError>;

#[derive(Debug, thiserror::Error)]
pub enum NekonoteError {
    #[error("Failed to read config file: {0}")]
    ConfigReadError(#[from] std::io::Error),
    #[error("Failed to parse config file: {0}")]
    ConfigParseError(#[from] config::ConfigError),

    #[error("Failed to build octocrab.")]
    OctocrabBuildError(#[source] octocrab::Error),
    #[error("Failed to build reqwest client.")]
    ReqwestClientBuildError(#[source] reqwest::Error),
    #[error("Failed to decode GitHub app_key.")]
    DecodeGitHubAppKeyError(#[source] base64::DecodeError),
    #[error("Failed to encode GitHub app_key.")]
    EncodeGitHubAppKeyError(#[source] jsonwebtoken::errors::Error),
}
