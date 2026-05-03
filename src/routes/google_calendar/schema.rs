use rmcp::schemars;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CalendarGetRequest {
    #[schemars(description = "Calendar ID shared with the service account.")]
    pub calendar_id: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct EventListRequest {
    #[schemars(description = "Calendar ID shared with the service account.")]
    pub calendar_id: String,

    #[schemars(
        description = "Lower bound in RFC3339 format, for example 2026-05-01T00:00:00+09:00."
    )]
    pub time_min: Option<String>,

    #[schemars(
        description = "Upper bound in RFC3339 format, for example 2026-05-08T00:00:00+09:00."
    )]
    pub time_max: Option<String>,

    #[schemars(description = "IANA time zone used in the response, for example Asia/Tokyo.")]
    pub time_zone: Option<String>,

    #[schemars(description = "Free-text query.")]
    pub query: Option<String>,

    #[schemars(
        description = "Maximum number of events to return. Google Calendar allows up to 2500."
    )]
    pub max_results: Option<i32>,

    #[schemars(description = "Whether recurring events should be expanded into instances.")]
    pub single_events: Option<bool>,

    #[schemars(description = "Whether cancelled events should be included.")]
    pub show_deleted: Option<bool>,

    #[schemars(description = "Result page token.")]
    pub page_token: Option<String>,

    #[schemars(description = "Sort order. Valid values include startTime and updated.")]
    pub order_by: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct EventGetRequest {
    #[schemars(description = "Calendar ID shared with the service account.")]
    pub calendar_id: String,

    #[schemars(description = "Google Calendar event ID.")]
    pub event_id: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct EventCreateRequest {
    #[schemars(description = "Calendar ID shared with the service account.")]
    pub calendar_id: String,

    #[schemars(description = "Event fields to create.")]
    pub event: EventInput,

    #[schemars(description = "Event start. Use either date_time or date.")]
    pub start: EventDateTimeInput,

    #[schemars(description = "Event end. Use either date_time or date.")]
    pub end: EventDateTimeInput,

    #[schemars(
        description = "Guest notification behavior. Valid values include all, externalOnly, none."
    )]
    pub send_updates: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct EventUpdateRequest {
    #[schemars(description = "Calendar ID shared with the service account.")]
    pub calendar_id: String,

    #[schemars(description = "Google Calendar event ID.")]
    pub event_id: String,

    #[schemars(description = "Event fields to patch.")]
    pub event: EventInput,

    #[schemars(description = "Replacement event start. Use either date_time or date.")]
    pub start: Option<EventDateTimeInput>,

    #[schemars(description = "Replacement event end. Use either date_time or date.")]
    pub end: Option<EventDateTimeInput>,

    #[schemars(
        description = "Guest notification behavior. Valid values include all, externalOnly, none."
    )]
    pub send_updates: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct EventDeleteRequest {
    #[schemars(description = "Calendar ID shared with the service account.")]
    pub calendar_id: String,

    #[schemars(description = "Google Calendar event ID.")]
    pub event_id: String,

    #[schemars(
        description = "Guest notification behavior. Valid values include all, externalOnly, none."
    )]
    pub send_updates: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct FreeBusyQueryRequest {
    #[schemars(description = "Calendar IDs shared with the service account.")]
    pub calendar_ids: Vec<String>,

    #[schemars(description = "Query start in RFC3339 format.")]
    pub time_min: String,

    #[schemars(description = "Query end in RFC3339 format.")]
    pub time_max: String,

    #[schemars(description = "IANA time zone used in the response, for example Asia/Tokyo.")]
    pub time_zone: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct EventInput {
    pub summary: Option<String>,
    pub description: Option<String>,
    pub location: Option<String>,
    pub status: Option<String>,
    pub transparency: Option<String>,
    pub visibility: Option<String>,
    pub color_id: Option<String>,
    pub recurrence: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct EventDateTimeInput {
    #[schemars(description = "RFC3339 date-time, for example 2026-05-01T09:00:00+09:00.")]
    pub date_time: Option<String>,

    #[schemars(description = "All-day date in yyyy-mm-dd format.")]
    pub date: Option<String>,

    #[schemars(description = "IANA time zone, for example Asia/Tokyo.")]
    pub time_zone: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct EventDeleteResponse {
    pub calendar_id: String,
    pub event_id: String,
    pub deleted: bool,
}
