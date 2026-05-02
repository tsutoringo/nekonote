use std::sync::Arc;

use rmcp::{
    ServerHandler,
    handler::server::tool::ToolRouter,
    model::{Implementation, ServerCapabilities, ServerInfo},
    tool_handler,
    transport::{
        StreamableHttpServerConfig, StreamableHttpService,
        streamable_http_server::session::local::LocalSessionManager,
    },
};

use crate::state::{AppState, GoogleCalendarState};

mod convert;
mod error;
mod schema;
mod tools;

pub fn routes(state: Arc<AppState>) -> axum::Router<Arc<AppState>> {
    let google_calendar = state.providers.google_calendar.clone();
    let mcp = StreamableHttpService::new(
        move || Ok(GoogleCalendarMcp::new(google_calendar.clone())),
        LocalSessionManager::default().into(),
        StreamableHttpServerConfig::default().with_allowed_hosts([
            "localhost",
            "127.0.0.1",
            "[::1]",
            "nekonote",
            "nekonote.moltis",
            "nekonote.moltis.svc",
            "nekonote.moltis.svc.cluster.local",
        ]),
    );

    axum::Router::new().nest_service("/mcp", mcp)
}

#[derive(Clone)]
pub struct GoogleCalendarMcp {
    google_calendar: Option<GoogleCalendarState>,
    #[allow(dead_code)]
    tool_router: ToolRouter<Self>,
}

impl GoogleCalendarMcp {
    fn new(google_calendar: Option<GoogleCalendarState>) -> Self {
        Self {
            google_calendar,
            tool_router: tools::build_tool_router(),
        }
    }

    fn google_calendar(&self) -> Result<&GoogleCalendarState, rmcp::ErrorData> {
        self.google_calendar.as_ref().ok_or_else(|| {
            rmcp::ErrorData::invalid_request("Google Calendar provider is not configured", None)
        })
    }
}

#[tool_handler(router = self.tool_router)]
impl ServerHandler for GoogleCalendarMcp {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build())
            .with_server_info(Implementation::from_build_env())
            .with_instructions(
                "Manage Google Calendar calendars shared with the configured service account."
                    .to_string(),
            )
    }
}
