use chrono::{DateTime, NaiveDate, Utc};
use google_calendar3 as calendar3;

use super::schema::{EventDateTimeInput, EventInput};

pub(super) fn build_event(input: EventInput) -> calendar3::api::Event {
    calendar3::api::Event {
        summary: input.summary,
        description: input.description,
        location: input.location,
        status: input.status,
        transparency: input.transparency,
        visibility: input.visibility,
        color_id: input.color_id,
        recurrence: input.recurrence,
        ..Default::default()
    }
}

pub(super) fn build_event_datetime(
    input: EventDateTimeInput,
    field: &'static str,
) -> Result<calendar3::api::EventDateTime, rmcp::ErrorData> {
    match (input.date_time, input.date) {
        (Some(date_time), None) => Ok(calendar3::api::EventDateTime {
            date_time: Some(parse_rfc3339(&date_time, field)?),
            time_zone: input.time_zone,
            ..Default::default()
        }),
        (None, Some(date)) => Ok(calendar3::api::EventDateTime {
            date: Some(parse_date(&date, field)?),
            time_zone: input.time_zone,
            ..Default::default()
        }),
        (None, None) => Err(rmcp::ErrorData::invalid_params(
            format!("{field} must include either date_time or date"),
            None,
        )),
        (Some(_), Some(_)) => Err(rmcp::ErrorData::invalid_params(
            format!("{field} must not include both date_time and date"),
            None,
        )),
    }
}

pub(super) fn parse_rfc3339(
    value: &str,
    field: &'static str,
) -> Result<DateTime<Utc>, rmcp::ErrorData> {
    DateTime::parse_from_rfc3339(value)
        .map(|date_time| date_time.with_timezone(&Utc))
        .map_err(|err| {
            rmcp::ErrorData::invalid_params(
                format!("{field} must be an RFC3339 date-time: {err}"),
                None,
            )
        })
}

fn parse_date(value: &str, field: &'static str) -> Result<NaiveDate, rmcp::ErrorData> {
    NaiveDate::parse_from_str(value, "%Y-%m-%d").map_err(|err| {
        rmcp::ErrorData::invalid_params(
            format!("{field} date must use yyyy-mm-dd format: {err}"),
            None,
        )
    })
}

pub(super) fn validate_max_results(value: i32, max: i32) -> Result<i32, rmcp::ErrorData> {
    if (1..=max).contains(&value) {
        Ok(value)
    } else {
        Err(rmcp::ErrorData::invalid_params(
            format!("max_results must be between 1 and {max}"),
            None,
        ))
    }
}
