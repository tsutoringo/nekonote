use google_calendar3 as calendar3;
use rmcp::{
    handler::server::{tool::ToolRouter, wrapper::Parameters},
    model::CallToolResult,
    tool, tool_router,
};

use super::{
    GoogleCalendarMcp,
    convert::{build_event, build_event_datetime, parse_rfc3339, validate_max_results},
    error::{google_api_error, json_result},
    schema::{
        CalendarGetRequest, EventCreateRequest, EventDeleteRequest, EventDeleteResponse,
        EventGetRequest, EventListRequest, EventUpdateRequest, FreeBusyQueryRequest,
    },
};

pub(super) fn build_tool_router() -> ToolRouter<GoogleCalendarMcp> {
    GoogleCalendarMcp::tool_router()
}

#[tool_router]
impl GoogleCalendarMcp {
    #[tool(description = "Get metadata for a Google Calendar shared with the service account")]
    async fn calendar_get(
        &self,
        Parameters(input): Parameters<CalendarGetRequest>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        let google_calendar = self.google_calendar()?;
        let (_, calendar) = google_calendar
            .hub
            .calendars()
            .get(&input.calendar_id)
            .doit()
            .await
            .map_err(google_api_error)?;

        json_result(&calendar)
    }

    #[tool(description = "List events in a Google Calendar shared with the service account")]
    async fn event_list(
        &self,
        Parameters(input): Parameters<EventListRequest>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        let google_calendar = self.google_calendar()?;

        let mut call = google_calendar
            .hub
            .events()
            .list(&input.calendar_id)
            .add_scope(calendar3::api::Scope::EventReadonly);
        if let Some(time_min) = input.time_min.as_deref() {
            call = call.time_min(parse_rfc3339(time_min, "time_min")?);
        }
        if let Some(time_max) = input.time_max.as_deref() {
            call = call.time_max(parse_rfc3339(time_max, "time_max")?);
        }
        if let Some(time_zone) = input.time_zone.as_deref() {
            call = call.time_zone(time_zone);
        }
        if let Some(query) = input.query.as_deref() {
            call = call.q(query);
        }
        if let Some(page_token) = input.page_token.as_deref() {
            call = call.page_token(page_token);
        }
        if let Some(order_by) = input.order_by.as_deref() {
            call = call.order_by(order_by);
        }
        if let Some(max_results) = input.max_results {
            call = call.max_results(validate_max_results(max_results, 2500)?);
        }
        if let Some(single_events) = input.single_events {
            call = call.single_events(single_events);
        }
        if let Some(show_deleted) = input.show_deleted {
            call = call.show_deleted(show_deleted);
        }

        let (_, events) = call.doit().await.map_err(google_api_error)?;
        json_result(&events)
    }

    #[tool(description = "Get one event from a Google Calendar shared with the service account")]
    async fn event_get(
        &self,
        Parameters(input): Parameters<EventGetRequest>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        let google_calendar = self.google_calendar()?;

        let (_, event) = google_calendar
            .hub
            .events()
            .get(&input.calendar_id, &input.event_id)
            .add_scope(calendar3::api::Scope::EventReadonly)
            .doit()
            .await
            .map_err(google_api_error)?;

        json_result(&event)
    }

    #[tool(description = "Create an event on a Google Calendar shared with the service account")]
    async fn event_create(
        &self,
        Parameters(input): Parameters<EventCreateRequest>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        let google_calendar = self.google_calendar()?;
        let mut event = build_event(input.event);
        event.start = Some(build_event_datetime(input.start, "start")?);
        event.end = Some(build_event_datetime(input.end, "end")?);

        let mut call = google_calendar
            .hub
            .events()
            .insert(event, &input.calendar_id);
        if let Some(send_updates) = input.send_updates.as_deref() {
            call = call.send_updates(send_updates);
        }

        let (_, event) = call.doit().await.map_err(google_api_error)?;
        json_result(&event)
    }

    #[tool(description = "Patch an event on a Google Calendar shared with the service account")]
    async fn event_update(
        &self,
        Parameters(input): Parameters<EventUpdateRequest>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        let google_calendar = self.google_calendar()?;
        let mut event = build_event(input.event);
        if let Some(start) = input.start {
            event.start = Some(build_event_datetime(start, "start")?);
        }
        if let Some(end) = input.end {
            event.end = Some(build_event_datetime(end, "end")?);
        }

        let mut call =
            google_calendar
                .hub
                .events()
                .patch(event, &input.calendar_id, &input.event_id);
        if let Some(send_updates) = input.send_updates.as_deref() {
            call = call.send_updates(send_updates);
        }

        let (_, event) = call.doit().await.map_err(google_api_error)?;
        json_result(&event)
    }

    #[tool(description = "Delete an event from a Google Calendar shared with the service account")]
    async fn event_delete(
        &self,
        Parameters(input): Parameters<EventDeleteRequest>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        let google_calendar = self.google_calendar()?;

        let mut call = google_calendar
            .hub
            .events()
            .delete(&input.calendar_id, &input.event_id);
        if let Some(send_updates) = input.send_updates.as_deref() {
            call = call.send_updates(send_updates);
        }

        call.doit().await.map_err(google_api_error)?;

        json_result(&EventDeleteResponse {
            calendar_id: input.calendar_id,
            event_id: input.event_id,
            deleted: true,
        })
    }

    #[tool(description = "Query busy blocks for Google Calendars shared with the service account")]
    async fn freebusy_query(
        &self,
        Parameters(input): Parameters<FreeBusyQueryRequest>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        let google_calendar = self.google_calendar()?;
        if input.calendar_ids.is_empty() {
            return Err(rmcp::ErrorData::invalid_params(
                "calendar_ids must include at least one calendar ID",
                None,
            ));
        }

        let request = calendar3::api::FreeBusyRequest {
            items: Some(
                input
                    .calendar_ids
                    .into_iter()
                    .map(|id| calendar3::api::FreeBusyRequestItem { id: Some(id) })
                    .collect(),
            ),
            time_min: Some(parse_rfc3339(&input.time_min, "time_min")?),
            time_max: Some(parse_rfc3339(&input.time_max, "time_max")?),
            time_zone: input.time_zone,
            ..Default::default()
        };

        let (_, response) = google_calendar
            .hub
            .freebusy()
            .query(request)
            .doit()
            .await
            .map_err(google_api_error)?;

        json_result(&response)
    }
}
