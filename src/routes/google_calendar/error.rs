use google_calendar3 as calendar3;
use rmcp::model::{CallToolResult, Content};
use serde::Serialize;

pub(super) fn json_result<T: Serialize>(value: &T) -> Result<CallToolResult, rmcp::ErrorData> {
    let text = serde_json::to_string_pretty(value).map_err(|err| {
        rmcp::ErrorData::internal_error(format!("failed to serialize response: {err}"), None)
    })?;

    Ok(CallToolResult::success(vec![Content::text(text)]))
}

pub(super) fn google_api_error(err: calendar3::Error) -> rmcp::ErrorData {
    rmcp::ErrorData::internal_error(format!("Google Calendar API request failed: {err}"), None)
}
