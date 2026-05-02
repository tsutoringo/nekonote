use std::sync::Arc;

use google_calendar3 as calendar3;

use crate::{
    config::{GoogleCalendarAuthConfig, GoogleCalendarConfig},
    error::{NekonoteError, Result},
};

type GoogleCalendarConnector = calendar3::hyper_rustls::HttpsConnector<
    calendar3::hyper_util::client::legacy::connect::HttpConnector,
>;
type GoogleCalendarClient =
    calendar3::hyper_util::client::legacy::Client<GoogleCalendarConnector, calendar3::common::Body>;
type GoogleCalendarHub = calendar3::CalendarHub<GoogleCalendarConnector>;
type GoogleCalendarAuthenticator =
    calendar3::yup_oauth2::authenticator::Authenticator<GoogleCalendarConnector>;

#[derive(Clone)]
pub(crate) struct GoogleCalendarState {
    pub hub: Arc<GoogleCalendarHub>,
}

impl GoogleCalendarState {
    pub(crate) async fn new(config: &GoogleCalendarConfig) -> Result<GoogleCalendarState> {
        let auth = build_google_calendar_authenticator(&config.auth).await?;
        let client = build_google_calendar_client()?;

        Ok(GoogleCalendarState {
            hub: Arc::new(calendar3::CalendarHub::new(client, auth)),
        })
    }
}

async fn build_google_calendar_authenticator(
    config: &GoogleCalendarAuthConfig,
) -> Result<GoogleCalendarAuthenticator> {
    match config {
        GoogleCalendarAuthConfig::ServiceAccount { key_path } => {
            build_service_account_authenticator(key_path).await
        }
    }
}

async fn build_service_account_authenticator(
    key_path: &str,
) -> Result<GoogleCalendarAuthenticator> {
    let service_account_key = calendar3::yup_oauth2::read_service_account_key(key_path)
        .await
        .map_err(NekonoteError::GoogleCalendarServiceAccountKeyReadError)?;

    calendar3::yup_oauth2::ServiceAccountAuthenticator::with_client(
        service_account_key,
        google_calendar_auth_client_builder()?,
    )
    .build()
    .await
    .map_err(NekonoteError::GoogleCalendarAuthBuildError)
}

fn google_calendar_auth_client_builder()
-> Result<calendar3::yup_oauth2::client::CustomHyperClientBuilder<GoogleCalendarConnector>> {
    let client = calendar3::hyper_util::client::legacy::Client::builder(
        calendar3::hyper_util::rt::TokioExecutor::new(),
    )
    .build::<_, String>(build_google_calendar_connector()?);

    Ok(calendar3::yup_oauth2::client::CustomHyperClientBuilder::from(client))
}

fn build_google_calendar_client() -> Result<GoogleCalendarClient> {
    Ok(calendar3::hyper_util::client::legacy::Client::builder(
        calendar3::hyper_util::rt::TokioExecutor::new(),
    )
    .build(build_google_calendar_connector()?))
}

fn build_google_calendar_connector() -> Result<GoogleCalendarConnector> {
    calendar3::hyper_rustls::HttpsConnectorBuilder::new()
        .with_native_roots()
        .map_err(NekonoteError::GoogleCalendarNativeRootsError)
        .map(|builder| builder.https_only().enable_http2().build())
}
