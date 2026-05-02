use reqwest::Client;

use crate::{
    config::ProviderConfig,
    error::{NekonoteError, Result},
    util::OptionFutureTransposeExt,
};

mod github;
mod google_calendar;

pub(crate) use github::GitHubState;
pub(crate) use google_calendar::GoogleCalendarState;

pub(crate) struct AppState {
    pub providers: ProviderState,
    pub client: Client,
}

impl AppState {
    pub(crate) async fn new(config: &crate::config::Config) -> Result<AppState> {
        let providers = ProviderState::new(&config.provider).await?;
        let client = Client::builder()
            .user_agent(concat!("nekonote/", env!("CARGO_PKG_VERSION")))
            .build()
            .map_err(NekonoteError::ReqwestClientBuildError)?;

        Ok(AppState { providers, client })
    }
}

#[derive(Clone)]
pub(crate) struct ProviderState {
    pub github: Option<GitHubState>,
    pub google_calendar: Option<GoogleCalendarState>,
}

impl ProviderState {
    async fn new(config: &ProviderConfig) -> Result<ProviderState> {
        Ok(ProviderState {
            github: config.github.as_ref().map(GitHubState::new).transpose()?,
            google_calendar: config
                .google_calendar
                .as_ref()
                .map(|config| GoogleCalendarState::new(config))
                .transpose()
                .await
                .transpose()?,
        })
    }
}
